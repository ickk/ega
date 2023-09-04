pub mod grade_select;
pub mod meet;
pub mod neg;
pub mod nil;
pub use grade_select::*;
pub use meet::*;
pub use neg::*;
pub use nil::*;

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
