pub trait Cut<Idx> {
  fn cut(&self, index: Idx) -> Self;
}
