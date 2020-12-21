use crate::{
  io::{
    IO,
  },
  iter::{
    Splitted,
  },
  wav::{
    WavHeader,
  },
};
use std::{
    collections::{
      HashMap,
    },
    io::{
      BufRead,
    },
    path::{
      Path,
      PathBuf,
    },
    ops::{
      Range,
    },
};
use walkdir::{
  WalkDir,
  DirEntry,
};
use getset::{
  Getters,
};

#[derive(Getters, Debug)]
pub struct OtoIni {
  // Map from moras to OTOs
  #[get = "pub"]
  entries: HashMap<String, OtoEntry>,
}

#[derive(Getters, Clone, Debug)]
pub struct OtoEntry {
  #[get = "pub"]
  source_wav : PathBuf,
  #[get = "pub"]
  mora : String,
  #[get = "pub"]
  offset : f64,
  #[get = "pub"]
  consonent : f64,
  #[get = "pub"]
  duration: OtoDuration,
  #[get = "pub"]
  preutterance : f64,
  #[get = "pub"]
  overlap : f64,
}

pub use OtoDuration::*;

#[derive(Clone, Debug)]
pub enum OtoDuration {
  LastSilence(f64),
  Sound(f64),
}

impl OtoIni {
  pub fn new() -> OtoIni {
    OtoIni {
      entries: HashMap::new()
    }
  }

  pub fn open(path: impl AsRef<Path>) -> OtoIni {
    let ini_reader = IO::shift_jis_reader(path);

    let mut entries = HashMap::new();
    for line in ini_reader.lines() {
       let entry = OtoEntry::from_line(&line.unwrap());
       entries.insert(entry.mora.clone(), entry);
    }
    OtoIni {
      entries: entries
    }
  }

  pub fn explore(path: impl AsRef<Path>) -> OtoIni {
    let mut oto_ini = Self::new();
    for dir_entry in WalkDir::new(path).max_depth(2) {
      let dir_entry = dir_entry.unwrap();
      if Self::is_oto_ini(&dir_entry) {
        oto_ini.extend(Self::open(dir_entry.path()));
      }
    }
    oto_ini
  }

  pub fn extend(&mut self, other: Self) {
    self.entries.extend(other.entries);
  }

  fn is_oto_ini(dir_entry: &DirEntry) -> bool {
    dir_entry.file_name() == "oto.ini" && dir_entry.file_type().is_file()
  }
}

impl OtoEntry {
  fn from_line(line: &str) -> OtoEntry {
    let (source_wav, params) = line.split_once('=').unwrap();
    let params = &mut params.split(',');
    
    OtoEntry {
       source_wav: PathBuf::from(source_wav),
       mora: Splitted::next_str(params).to_string(),
       offset: Splitted::next_f64(params),
       consonent: Splitted::next_f64(params),
       duration: {
         let d = Splitted::next_f64(params);
         if d > 0. { LastSilence(d) } else { Sound(-d) }
       },
       preutterance: Splitted::next_f64(params),
       overlap: Splitted::next_f64(params),
    }
  }

  pub fn start(&self) -> f64 {
    self.offset
  }

  pub fn ovl_end(&self) -> f64 {
    self.offset + self.overlap
  }

  pub fn pre_end(&self) -> f64 {
    self.offset + self.preutterance
  }

  pub fn con_end(&self) -> f64 {
    self.offset + self.consonent
  }

  pub fn end(&self) -> Result<f64, f64> {
    match self.duration {
      Sound(duration) => Ok(self.offset + duration),
      LastSilence(negative_offset) => Err(negative_offset),
    }
  }

  pub fn definite_end(&self, wav_header: &WavHeader, wav_len: usize) -> f64 {
    match self.end() {
      Ok(end) => end,
      Err(negative_offset) => {
        let wav_duration = wav_len as f64 / wav_header.sample_rate as f64;
        wav_duration - negative_offset
      },
    }
  }

  pub fn ovl(&self) -> Range<f64> {
    self.start()..self.ovl_end()
  }

  pub fn pre(&self) -> Range<f64> {
    self.start()..self.pre_end()
  }

  pub fn con(&self) -> Range<f64> {
    self.start()..self.con_end()
  }

  pub fn vow(&self) -> Result<Range<f64>, f64> {
    self.end().map(|end| self.con_end()..end)
  }

  pub fn definite_vow(&self, wav_header: &WavHeader, wav_len: usize) -> Range<f64> {
    self.con_end()..self.definite_end(wav_header, wav_len)
  }

  pub fn voice(&self) -> Result<Range<f64>, f64> {
    self.end().map(|end| self.start()..end)
  }

  pub fn definite_voice(&self, wav_header: &WavHeader, wav_len: usize) -> Range<f64> {
    self.start()..self.definite_end(wav_header, wav_len)
  }
}
