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
};
use getset::{
  Getters,
};

#[derive(Getters)]
pub struct Wav {
  #[get = "pub"]
  samples: Vec<i32>
}

pub struct WavIter<'a> {
  samples: Iter<'a, i32>
}

impl Wav {
  pub fn open(path: impl AsRef<Path>) -> Wav {
    let reader = WavReader::open(path).unwrap();
    Wav {
      samples: reader.into_samples::<i32>().map(|smp| smp.unwrap()).collect()
    }
  }

  pub fn iter(&self) -> WavIter<'_> {
    WavIter {
      samples: self.samples.iter()
    }
  }
}

impl WavIter<'_> {
  pub fn as_slice(&self) -> &[i32] {
    self.samples.as_slice()
  }
}

impl<'a> Iterator for WavIter<'a> {
  type Item = &'a i32;

  fn next(&mut self) -> Option<Self::Item> {
    self.samples.next()
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
    self.as_slice()
  }
}
