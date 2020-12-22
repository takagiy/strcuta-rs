use crate::{
  cut::{
    Cut,
  },
};
use std::{
  slice::{
    Iter,
    SliceIndex,
  },
  path::{
    Path,
  },
  ops::{
    Deref,
  },
};
use hound::{
  WavReader,
  WavWriter,
  WavSpec,
};
use getset::{
  Getters,
};

#[derive(Getters, Debug)]
pub struct Wav {
  #[get = "pub"]
  header: WavHeader,
  #[get = "pub"]
  samples: Vec<i32>,
}

pub type WavHeader = WavSpec;

#[derive(Getters, Clone, Debug)]
pub struct WavPart<'a> {
  #[get = "pub"]
  header: &'a WavHeader,
  samples: &'a [i32],
}

#[derive(Getters, Debug)]
pub struct WavIter<'a> {
  samples: Iter<'a, i32>,
}

impl Wav {
  pub fn open(path: impl AsRef<Path>) -> Wav {
    println!("{:?}", path.as_ref());
    let reader = WavReader::open(path).unwrap();
    Wav {
      header: reader.spec(),
      samples: reader.into_samples::<i32>().map(|smp| smp.unwrap()).collect(),
    }
  }

  pub fn as_part(&self) -> WavPart<'_> {
    WavPart {
      header: &self.header,
      samples: &self.samples,
    }
  }

  pub fn iter(&self) -> WavIter<'_> {
    WavIter {
      samples: self.samples.iter()
    }
  }
}

impl WavPart<'_> {
  pub fn samples(&self) -> &[i32] {
    self.samples
  }

  pub fn iter(&self) -> WavIter<'_> {
    WavIter {
      samples: self.samples.iter()
    }
  }

  pub fn cut(&self, index: impl SliceIndex<[i32], Output = [i32]>) -> Self {
    WavPart {
      header: &self.header,
      samples: &self.samples[index],
    }
  }

  pub fn save(&self, path: impl AsRef<Path>) {
    let mut writer = WavWriter::create(path, self.header.clone()).unwrap();
    for &smp in self.samples {
      writer.write_sample(smp).unwrap();
    }
    writer.finalize().unwrap();
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

impl Deref for WavPart<'_> {
  type Target = [i32];

  fn deref(&self) -> &Self::Target {
    self.samples
  }
}

impl Deref for WavIter<'_> {
  type Target = [i32];

  fn deref(&self) -> &Self::Target {
    self.samples.as_slice()
  }
}

impl<I: SliceIndex<[i32], Output = [i32]>> Cut<I> for WavPart<'_> {
  fn cut(&self, index: I) -> Self {
    self.cut(index)
  }
}
