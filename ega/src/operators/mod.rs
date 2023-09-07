mod dot;
mod geometric_product;
mod grade_select;
mod join;
mod meet;
mod neg;
mod zero;
mod reverse;
pub use reverse::Reverse;
pub use dot::Dot;
pub use geometric_product::GeometricProduct;
pub use grade_select::GradeSelect;
pub use join::Join;
pub use meet::Meet;
pub use neg::Neg;
pub use zero::Zero;

/// The norm
pub trait Normalise {
  /// The norm
  fn norm(&self) -> Self;
}

/// The dual
pub trait Dual {
  type Output;

  /// The dual
  fn dual(&self) -> Self::Output;
}

/// Exponentiation
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
