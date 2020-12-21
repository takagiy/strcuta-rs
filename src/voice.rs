use crate::{
  wav::{
    Wav,
    WavPart,
    WavIter,
  },
  frq::{
    Frq,
    FrqPart,
    FrqIter,
  },
  cut::{
    Cut,
  },
};
use std::{
  slice::{
    SliceIndex,
  },
};

#[derive(Debug)]
pub struct Voice {
  wav: Wav,
  frq: Frq,
}

#[derive(Debug)]
pub struct VoicePart<'a> {
  wav: WavPart<'a>,
  frq: FrqPart<'a>,
}

#[derive(Debug)]
pub struct VoiceIter<'a> {
  wav: WavIter<'a>,
  frq: FrqIter<'a>,
}

pub trait VoiceRef {
  fn wav(&self) -> &[i32];

  fn frq(&self) -> &[f64];
}

impl Voice {
  pub fn as_part(&self) -> VoicePart<'_> {
    VoicePart {
      wav: self.wav.as_part(),
      frq: self.frq.as_part(),
    }
  }

  pub fn iter(&self) -> VoiceIter<'_> {
    VoiceIter {
      wav: self.wav.iter(),
      frq: self.frq.iter(),
    }
  }
}

impl VoiceRef for Voice {
  fn wav(&self) -> &[i32] {
    &self.wav
  }

  fn frq(&self) -> &[f64] {
    &self.frq
  }
}

impl VoiceRef for VoicePart<'_> {
  fn wav(&self) -> &[i32] {
    self.wav.samples()
  }

  fn frq(&self) -> &[f64] {
    self.frq.samples()
  }
}

impl<'a> Iterator for VoiceIter<'a> {
  type Item = (&'a i32, &'a f64);

  fn next(&mut self) -> Option<Self::Item> {
    self.wav.next().and_then(|wav| {
      self.frq.next().map(|frq| (wav, frq.0))
    })
  }
}

impl<
  I: Clone +
     SliceIndex<[i32], Output = [i32]> +
     SliceIndex<[f64], Output = [f64]>
> Cut<I> for VoicePart<'_> {
  fn cut(&self, index: I) -> Self {
    VoicePart {
      wav: self.wav.cut(index.clone()),
      frq: self.frq.cut(index),
    }
  }
}
