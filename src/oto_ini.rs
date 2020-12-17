use crate::{
  io::{
    IO
  }
};
use std::{
    collections::{
      HashMap,
    },
    io::{
      BufReader,
      BufRead,
    },
    fs:: {
      File,
    },
    path::{
      Path,
      PathBuf,
    },
};
use walkdir::{
  WalkDir,
  DirEntry,
};

pub struct OtoIni {
  // Map from moras to OTOs
  entries: HashMap<String, OtoEntry>,
}

pub struct OtoEntry {
  source_wav : PathBuf,
  mora : String,
  offset : f64,
  consonent : f64,
  duration: OtoDuration,
  preutterance : f64,
  overlap : f64,
}

pub use OtoDuration::*;

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

  pub fn load(path: impl AsRef<Path>) -> OtoIni {
    //let oto_ini = File::open(path.as_ref()).unwrap();
    //let ini_reader = BufReader::new(oto_ini);
    let ini_reader = IO::shift_jis_reader(path.as_ref());

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
    for dir_entry in WalkDir::new(path).max_depth(3) {
      let dir_entry = dir_entry.unwrap();
      if Self::is_oto_ini(&dir_entry) {
        oto_ini.extend(Self::load(dir_entry.path()));
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
       mora: Self::next_str(params).to_string(),
       offset: Self::next_f64(params),
       consonent: Self::next_f64(params),
       duration: {
         let d = Self::next_f64(params);
         if d > 0. { LastSilence(d) } else { Sound(d) }
       },
       preutterance: Self::next_f64(params),
       overlap: Self::next_f64(params),
    }
  }

  fn next_str<'a>(iterator: &mut impl Iterator<Item=&'a str>) -> &'a str {
    iterator.next().unwrap().trim()
  }

  fn next_f64<'a>(iterator: &mut impl Iterator<Item=&'a str>) -> f64 {
    Self::next_str(iterator).parse().unwrap()
  }
}
