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
  oto_ini::{
    OtoEntry
  },
  cut::{
    Cut,
  },
};
use std::{
  slice::{
    SliceIndex,
  },
  ops::{
    Range,
  },
};
use getset::{
  Getters,
};

#[derive(Getters, Debug)]
pub struct Voice {
  #[get = "pub"]
  oto: OtoEntry,
  wav: Wav,
  frq: Frq,
}

#[derive(Getters, Debug)]
pub struct VoicePart<'a> {
  #[get = "pub"]
  oto: &'a OtoEntry,
  wav: WavPart<'a>,
  frq: FrqPart<'a>,
}

#[derive(Debug)]
pub struct VoiceIter<'a> {
  wav: WavIter<'a>,
  frq: FrqIter<'a>,
}

pub trait VoiceRef {
  fn wav(&self) -> WavPart<'_>;

  fn frq(&self) -> FrqPart<'_>;
}

pub trait SampleRange {
  fn to_usize_range(&self, sample_rate: u32) -> Range<usize>;
}

impl Voice {
  pub fn new(oto: &OtoEntry) -> Voice {
    Voice {
      oto: oto.clone(),
      wav: Wav::open(oto.source_wav()),
      frq: Frq::open_by_wav_path(oto.source_wav()),
    }
  }

  pub fn wav(&self) -> WavPart<'_> {
    self.wav.as_part()
  }

  pub fn frq(&self) -> FrqPart<'_> {
    self.frq.as_part()
  }

  pub fn as_part(&self) -> VoicePart<'_> {
    VoicePart {
      oto: &self.oto,
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

  pub fn ovl(&self) -> VoicePart<'_> {
    self.as_part().cut(self.oto.ovl().to_usize_range(self.wav.header().sample_rate))
  }

  pub fn pre(&self) -> VoicePart<'_> {
    self.as_part().cut(self.oto.ovl().to_usize_range(self.wav.header().sample_rate))
  }

  pub fn con(&self) -> VoicePart<'_> {
    self.as_part().cut(self.oto.con().to_usize_range(self.wav.header().sample_rate))
  }

  pub fn vow(&self) -> VoicePart<'_> {
    let duration =
        self.oto.definite_vow(self.wav.header().sample_rate, self.wav.len())
            .to_usize_range(self.wav.header().sample_rate);
    self.as_part().cut(duration)
  }
}

impl VoicePart<'_> {
  pub fn wav(&self) -> WavPart<'_> {
    self.wav.clone()
  }

  pub fn frq(&self) -> FrqPart<'_> {
    self.frq.clone()
  }

  fn cut(
      &self,
      index: impl Clone +
             SliceIndex<[i32], Output = [i32]> +
             SliceIndex<[f64], Output = [f64]>
  ) -> Self {
    VoicePart {
      oto: &self.oto,
      wav: self.wav.cut(index.clone()),
      frq: self.frq.cut(index),
    }
  }
}

impl VoiceRef for Voice {
  fn wav(&self) -> WavPart<'_> {
    self.wav()
  }

  fn frq(&self) -> FrqPart<'_> {
    self.frq()
  }
}

impl VoiceRef for VoicePart<'_> {
  fn wav(&self) -> WavPart<'_> {
    self.wav()
  }

  fn frq(&self) -> FrqPart<'_> {
    self.frq()
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
    self.cut(index)
  }
}

impl SampleRange for Range<f64> {
  fn to_usize_range(&self, sample_rate: u32) -> Range<usize> {
    let Range { start, end } = self;
    Range {
      start: (start * sample_rate as f64) as usize,
      end: (end * sample_rate as f64) as usize,
    }
  }
}
