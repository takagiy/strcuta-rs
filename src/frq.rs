use crate::{
  io::{
    BinaryRead,
    StringOrBytes,
  },
  cut::{
    Cut,
    SampleRange,
  }
};
use std::{
  slice::{
    Iter,
  },
  ops::{
    Deref,
  },
  io::{
    BufReader
  },
  fs::{
    File,
  },
  path::{
    Path,
  },
};
use byteorder::{
  LittleEndian as LE,
};
use getset::{
  Getters,
};

#[derive(Getters, Debug)]
pub struct Frq {
  #[get = "pub"]
  header: FrqHeader,
  #[get = "pub"]
  samples: Vec<f64>,
  #[get = "pub"]
  amplitude_samples: Vec<f64>,
}

#[derive(Debug)]
pub struct FrqHeader {
  pub chunk_id: StringOrBytes,
  pub sampling_interval: i32,
  pub key_frequency: f64,
  pub comment: StringOrBytes,
  pub len: i32,
  pub wav_sample_rate: u32,
}

#[derive(Getters, Clone, Debug)]
pub struct FrqPart<'a> {
  #[get = "pub"]
  header: &'a FrqHeader,
  samples: &'a [f64],
  amplitude_samples: &'a [f64],
}

#[derive(Debug)]
pub struct FrqIter<'a> {
  samples: Iter<'a, f64>,
  amplitude_samples: Iter<'a, f64>,
}

impl Frq {
  pub fn open(path: impl AsRef<Path>, wav_sample_rate: u32) -> Frq {
    let mut frq_file = File::open(path).unwrap();
    let header = FrqHeader::read(&mut frq_file, wav_sample_rate);

    // Prepare a buffered reader with capacity for header.len pairs of f64
    let mut reader = BufReader::with_capacity((2 * 8 * header.len) as usize, frq_file);
    Self::read_with_header(header, &mut reader)
  }

  pub fn open_by_wav_path(wav_path: impl AsRef<Path>, wav_sample_rate: u32) -> Frq {
    let wav_file_stem = wav_path.as_ref().file_stem().unwrap();
    let wav_extension = wav_path.as_ref().extension().unwrap();
    let frq_file_name = {
      let mut frq_file_name = wav_file_stem.to_os_string();
      frq_file_name.push("_");
      frq_file_name.push(wav_extension);
      frq_file_name.push(".frq");
      frq_file_name
    };
    let frq_path = wav_path.as_ref().with_file_name(frq_file_name);
    Self::open(frq_path, wav_sample_rate)
  }

  fn read_with_header(header: FrqHeader, reader: &mut impl BinaryRead) -> Frq {
    let mut samples = Vec::with_capacity(header.len as usize);
    let mut amplitude_samples = Vec::with_capacity(header.len as usize);

    for _ in 0..header.len {
      samples.push(reader.read_f64::<LE>().unwrap());
      amplitude_samples.push(reader.read_f64::<LE>().unwrap());
    }

    Frq {
      header: header,
      samples: samples,
      amplitude_samples: amplitude_samples,
    }
  }

  pub fn as_part(&self) -> FrqPart<'_> {
    FrqPart {
      header: &self.header,
      samples: &self.samples,
      amplitude_samples: &self.amplitude_samples
    }
  }

  pub fn iter(&self) -> FrqIter<'_> {
    FrqIter {
      samples: self.samples.iter(),
      amplitude_samples: self.amplitude_samples.iter(),
    }
  }
}

impl FrqHeader {
  pub fn read(reader: &mut impl BinaryRead, wav_sample_rate: u32) -> FrqHeader {
    FrqHeader {
      chunk_id: reader.read_string_from_chunk(8),
      sampling_interval: reader.read_i32::<LE>().unwrap(),
      key_frequency: reader.read_f64::<LE>().unwrap(),
      comment: reader.read_string_from_chunk(16),
      len: reader.read_i32::<LE>().unwrap(),
      wav_sample_rate: wav_sample_rate,
    }
  }

  pub fn sample_rate(&self) -> u32 {
    self.wav_sample_rate / self.sampling_interval as u32
  }
}

impl FrqPart<'_> {
  pub fn samples(&self) -> &[f64] {
    self.samples
  }

  pub fn amplitude_samples(&self) -> &[f64] {
    self.amplitude_samples
  }

  pub fn iter(&self) -> FrqIter<'_> {
    FrqIter {
      samples: self.samples.iter(),
      amplitude_samples: self.samples.iter(),
    }
  }

  pub fn cut(&self, index: impl SampleRange) -> Self {
    let index = index.to_usize_range(self.header.sample_rate());
    FrqPart {
      header: &self.header,
      samples: &self.samples[index.clone()],
      amplitude_samples: &self.amplitude_samples[index],
    }
  }
}

impl<'a> Iterator for FrqIter<'a> {
  type Item = (&'a f64, &'a f64);

  fn next(&mut self) -> Option<Self::Item> {
    self.samples.next().and_then(|frq| {
        self.amplitude_samples.next().map(|amp| (frq, amp))
    })
  }
}

impl ExactSizeIterator for FrqIter<'_> {
  fn len(&self) -> usize {
    self.samples.len()
  }
}

impl Deref for Frq {
  type Target = [f64];

  fn deref(&self) -> &Self::Target {
    &self.samples
  }
}

impl Deref for FrqPart<'_> {
  type Target = [f64];

  fn deref(&self) -> &Self::Target {
    self.samples
  }
}

impl Deref for FrqIter<'_> {
  type Target = [f64];

  fn deref(&self) -> &Self::Target {
    self.samples.as_slice()
  }
}

impl Cut for FrqPart<'_> {
  fn cut(&self, index: impl SampleRange) -> Self {
    self.cut(index)
  }
}
