use std::{
  slice::{
    SliceIndex,
  },
  ops::{
    Range,
    RangeInclusive,
    RangeFrom,
    RangeTo,
    RangeToInclusive,
    RangeFull,
  },
};

pub trait Cut {
  fn cut(&self, index: impl SampleRange) -> Self;
}

pub trait SampleRange: Clone {
  type UsizeRange: Clone + SliceIndex<[i32], Output = [i32]> + SliceIndex<[f64], Output = [f64]>;
  fn to_usize_range(&self, sample_rate: u32) -> Self::UsizeRange;
}

pub trait SamplePosition: Clone {
  fn to_usize_position(&self, sample_rate: u32) -> usize;
}

impl SamplePosition for f64 {
  fn to_usize_position(&self, sample_rate: u32) -> usize {
    (self / 1000. * sample_rate as f64) as usize
  }
}

impl SamplePosition for usize {
  fn to_usize_position(&self, _sample_rate: u32) -> usize {
    *self
  }
}

impl<T: SamplePosition> SampleRange for T {
  type UsizeRange = RangeInclusive<usize>;
  fn to_usize_range(&self, sample_rate: u32) -> Self::UsizeRange {
    self.to_usize_position(sample_rate)..=self.to_usize_position(sample_rate)
  }
}

impl<T: SamplePosition> SampleRange for Range<T> {
  type UsizeRange = Range<usize>;
  fn to_usize_range(&self, sample_rate: u32) -> Self::UsizeRange {
    self.start.to_usize_position(sample_rate)..self.end.to_usize_position(sample_rate)
  }
}

impl<T: SamplePosition> SampleRange for RangeInclusive<T> {
  type UsizeRange = RangeInclusive<usize>;
  fn to_usize_range(&self, sample_rate: u32) -> Self::UsizeRange {
    self.start().to_usize_position(sample_rate)..=self.end().to_usize_position(sample_rate)
  }
}

impl<T: SamplePosition> SampleRange for RangeFrom<T> {
  type UsizeRange = RangeFrom<usize>;
  fn to_usize_range(&self, sample_rate: u32) -> Self::UsizeRange {
    self.start.to_usize_position(sample_rate)..
  }
}

impl<T: SamplePosition> SampleRange for RangeTo<T> {
  type UsizeRange = RangeTo<usize>;
  fn to_usize_range(&self, sample_rate: u32) -> Self::UsizeRange {
    ..self.end.to_usize_position(sample_rate)
  }
}

impl<T: SamplePosition> SampleRange for RangeToInclusive<T> {
  type UsizeRange = RangeToInclusive<usize>;
  fn to_usize_range(&self, sample_rate: u32) -> Self::UsizeRange {
    ..=self.end.to_usize_position(sample_rate)
  }
}

impl SampleRange for RangeFull {
  type UsizeRange = RangeFull;
  fn to_usize_range(&self, _sample_rate: u32) -> Self::UsizeRange {
    ..
  }
}
