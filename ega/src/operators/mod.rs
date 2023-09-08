mod dot;
mod dual;
mod geometric_product;
mod grade_select;
mod join;
mod meet;
mod neg;
mod reverse;
mod zero;
pub use dot::Dot;
pub use dual::Dual;
pub use geometric_product::GeometricProduct;
pub use grade_select::GradeSelect;
pub use join::Join;
pub use meet::Meet;
pub use neg::Neg;
pub use reverse::Reverse;
pub use zero::Zero;

use crate::{values::Empty, Scalar};

/// The norm
pub trait Norm {
  /// The squared norm
  fn norm_squared(&self) -> Scalar;

  /// The norm
  fn norm(&self) -> Scalar;
}

/// Normalise
pub trait Normalise: Norm {
  /// The normalised value
  fn normalise(&self) -> Self;
}

/// Exponentiation
pub trait Exponent<Rhs> {
  type Output;

  /// Exponentiation
  fn exp(&self, rhs: Rhs) -> Self::Output;
}

#[inline]
fn return_empty<Lhs, Rhs>(_: &Lhs, _: &Rhs) -> Empty {
  Empty
}
