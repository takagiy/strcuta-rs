use std::{
  slice::{
    Iter,
  },
  path::{
    Path,
  },
  ops::{
    Deref,
  }
};
use hound::{
  WavReader,
  WavSpec,
};
use getset::{
  Getters,
};

#[derive(Getters)]
pub struct Wav {
  #[get = "pub"]
  header: WavHeader,
  #[get = "pub"]
  samples: Vec<i32>,
}

pub type WavHeader = WavSpec;

#[derive(Getters)]
pub struct WavIter<'a> {
  #[get = "pub"]
  header: &'a WavHeader,
  samples: Iter<'a, i32>,
}

impl Wav {
  pub fn open(path: impl AsRef<Path>) -> Wav {
    let reader = WavReader::open(path).unwrap();
    Wav {
      header: reader.spec(),
      samples: reader.into_samples::<i32>().map(|smp| smp.unwrap()).collect(),
    }
  }

  pub fn iter(&self) -> WavIter<'_> {
    WavIter {
      header: &self.header,
      samples: self.samples.iter(),
    }
  }
}

impl WavIter<'_> {
  pub fn samples(&self) -> &[i32] {
    self.samples.as_slice()
  }
}

impl<'a> Iterator for WavIter<'a> {
  type Item = &'a i32;

  fn next(&mut self) -> Option<Self::Item> {
    self.samples.next()
  }
}

impl ExactSizeIterator for WavIter<'_> {
  fn len(&self) -> usize {
    self.samples.len()
  }
}

impl Deref for Wav {
  type Target = [i32];

  fn deref(&self) -> &Self::Target {
    &self.samples
  }
}

impl Deref for WavIter<'_> {
  type Target = [i32];

  fn deref(&self) -> &Self::Target {
    self.samples()
  }
}
