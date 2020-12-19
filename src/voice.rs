use crate::{
  wav::{
    Wav,
    WavIter,
  },
  frq::{
    Frq,
    FrqIter,
  },
};

pub struct Voice {
  wav: Wav,
  frq: Frq,
}

pub struct VoiceIter<'a> {
  wav: WavIter<'a>,
  frq: FrqIter<'a>,
}

pub trait VoiceRef {
  fn wav(&self) -> &[i32];

  fn frq(&self) -> &[f64];
}

impl Voice {
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

impl VoiceRef for VoiceIter<'_> {
  fn wav(&self) -> &[i32] {
    self.wav.as_slice()
  }

  fn frq(&self) -> &[f64] {
    self.frq.as_slice()
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
