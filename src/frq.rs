use std::{
  slice::{
    Iter,
  },
};

pub struct Frq {
  samples: Vec<f64>,
  amplitude_samples: Vec<f64>,
}

pub struct FrqIter<'a> {
  samples: Iter<'a, f64>,
  amplitude_samples: Iter<'a, f64>,
}

impl Frq {
  pub fn iter(&self) -> FrqIter<'_> {
    FrqIter {
      samples: self.samples.iter(),
      amplitude_samples: self.amplitude_samples.iter(),
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
