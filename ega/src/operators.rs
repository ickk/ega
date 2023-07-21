pub trait Meet<Rhs> {
  type Output;

  /// The outer product
  fn meet(&self, rhs: Rhs) -> Self::Output;

  #[inline]
  fn wedge(&self, rhs: Rhs) -> Self::Output {
    self.meet(rhs)
  }
}

pub trait Join<Rhs> {
  type Output;

  fn join(&self, rhs: Rhs) -> Self::Output;
}

pub trait Dot<Rhs> {
  type Output;

  /// The inner product
  fn dot(&self, rhs: Rhs) -> Self::Output;
}

// TODO: dual, reverse
