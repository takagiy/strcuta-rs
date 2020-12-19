use std::{
  slice::{
    Iter,
  },
  path::{
    Path,
  },
};
use hound::{
  WavReader,
};

pub struct Wav {
  samples: Vec<i32>
}

pub type WavIter<'a> = Iter<'a, i32>;

impl Wav {
  pub fn open(path: impl AsRef<Path>) -> Wav {
    let reader = WavReader::open(path).unwrap();
    Wav {
      samples: reader.into_samples::<i32>().map(|smp| smp.unwrap()).collect()
    }
  }

  pub fn iter(&self) -> WavIter<'_> {
    self.samples.iter()
  }
}
