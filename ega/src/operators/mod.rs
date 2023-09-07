pub mod dot;
pub mod grade_select;
pub mod join;
pub mod meet;
pub mod neg;
pub mod zero;
pub use dot::*;
pub use grade_select::*;
pub use join::*;
pub use meet::*;
pub use neg::*;
pub use neg::*;
pub use zero::*;

pub trait GeometricProduct<Rhs> {
  type Output;

  /// The geometric product
  fn mul(&self, rhs: Rhs) -> Self::Output;
}

pub trait Normalise {
  /// The norm
  fn norm(&self) -> Self;
}

pub trait Dual {
  type Output;

  /// The dual
  fn dual(&self) -> Self::Output;
}

pub trait Reverse {
  type Output;

  /// The reverse
  fn reverse(&self) -> Self::Output;
}

pub trait Exponent<Rhs> {
  type Output;

  /// Exponentiation
  fn exp(&self, rhs: Rhs) -> Self::Output;
}

use crate::values::Empty;
#[inline]
fn return_empty<Lhs, Rhs>(_: &Lhs, _: &Rhs) -> Empty {
  Empty
}
