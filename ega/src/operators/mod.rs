mod add;
mod dot;
mod geometric_product;
mod grade_select;
mod hodge_dual;
mod join;
mod meet;
mod mul;
mod neg;
mod reverse;
mod sub;
mod zero;
pub use add::Add;
pub use dot::Dot;
pub use geometric_product::GeometricProduct;
pub use grade_select::GradeSelect;
pub use hodge_dual::HodgeDual;
pub use join::Join;
pub use meet::Meet;
pub use mul::Mul;
pub use neg::Neg;
pub use reverse::Reverse;
pub use sub::Sub;
pub use {zero::zero, zero::Zero};

use crate::{values::Empty, Scalar};

/// The norm
pub trait Norm {
  /// The squared norm
  fn norm_squared(self) -> Scalar;

  /// The norm
  fn norm(self) -> Scalar;
}

/// Normalise
pub trait Normalise: Norm {
  /// The normalised value
  fn normalise(self) -> Self;
}

/// Exponentiation
pub trait Exponent<Rhs> {
  type Output;

  /// Exponentiation
  fn exp(self, rhs: Rhs) -> Self::Output;
}

#[inline]
fn return_empty<Lhs, Rhs>(_: Lhs, _: Rhs) -> Empty {
  Empty
}

#[inline]
fn return_lhs<Lhs, Rhs>(lhs: Lhs, _: Rhs) -> Lhs {
  lhs
}

#[inline]
fn return_rhs<Lhs, Rhs>(_: Lhs, rhs: Rhs) -> Rhs {
  rhs
}
