use crate::{
  io::{
    BinaryRead,
    StringOrBytes,
  },
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

#[derive(Getters)]
pub struct Frq {
  #[get = "pub"]
  header: FrqHeader,
  #[get = "pub"]
  samples: Vec<f64>,
  #[get = "pub"]
  amplitude_samples: Vec<f64>,
}

pub struct FrqHeader {
  pub chunk_id: StringOrBytes,
  pub sampling_interval: i32,
  pub key_frequency: f64,
  pub comment: StringOrBytes,
  pub len: i32,
}

pub struct FrqIter<'a> {
  samples: Iter<'a, f64>,
  amplitude_samples: Iter<'a, f64>,
}

impl Frq {
  pub fn open(path: impl AsRef<Path>) -> Frq {
    let mut frq_file = File::open(path).unwrap();
    let header = FrqHeader::read(&mut frq_file);

    // Prepare a buffered reader with capacity for header.len pairs of f64
    let mut reader = BufReader::with_capacity((2 * 16 * header.len) as usize, frq_file);
    Self::read_with_header(header, &mut reader)
  }

  fn read_with_header(header: FrqHeader, reader: &mut impl BinaryRead) -> Frq {
    let mut samples = Vec::new();
    let mut amplitude_samples = Vec::new();

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

  pub fn iter(&self) -> FrqIter<'_> {
    FrqIter {
      samples: self.samples.iter(),
      amplitude_samples: self.amplitude_samples.iter(),
    }
  }
}

impl FrqHeader {
  pub fn read(reader: &mut impl BinaryRead) -> FrqHeader {
    FrqHeader {
      chunk_id: reader.read_string_from_chunk(8),
      sampling_interval: reader.read_i32::<LE>().unwrap(),
      key_frequency: reader.read_f64::<LE>().unwrap(),
      comment: reader.read_string_from_chunk(16),
      len: reader.read_i32::<LE>().unwrap(),
    }
  }
}

impl FrqIter<'_> {
  pub fn as_slice(&self) -> &[f64] {
    self.samples.as_slice()
  }

  pub fn as_amplitude_slice(&self) -> &[f64] {
    self.amplitude_samples.as_slice()
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

impl Deref for Frq {
  type Target = [f64];

  fn deref(&self) -> &Self::Target {
    &self.samples
  }
}

impl Deref for FrqIter<'_> {
  type Target = [f64];

  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}
