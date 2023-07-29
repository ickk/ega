pub mod meet;
pub use meet::*;

pub trait Join<Rhs> {
  type Output;

  /// The regressive product
  fn join(&self, rhs: Rhs) -> Self::Output;
}

pub trait Dot<Rhs> {
  type Output;

  /// The inner product
  fn dot(&self, rhs: Rhs) -> Self::Output;
}

pub trait GeometricProduct<Rhs> {
  type Output;

  fn mul(&self, rhs: Rhs) -> Self::Output;
}

pub trait Dual {
  type Output;

  fn dual(&self) -> Self::Output;
}

pub trait Reverse {
  type Output;

  fn reverse(&self) -> Self::Output;
}

pub trait GradeSelect {
  type Output;

  fn grade(&self) -> Self::Output;
}
