use std::{
  slice::{
    Iter,
  },
};

pub struct Wav {
  samples: Vec<i32>
}

pub type WavIter<'a> = Iter<'a, i32>;

impl Wav {
  pub fn iter(&self) -> WavIter<'_> {
    self.samples.iter()
  }
}
