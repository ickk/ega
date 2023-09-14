mod add;
mod conjugate;
mod div;
mod dot;
mod geometric_product;
mod grade_select;
mod hodge_dual;
mod ideal_norm;
mod inverse;
mod join;
mod meet;
mod mul;
mod neg;
mod norm;
mod normalise;
mod partial_eq;
mod reverse;
mod scalar_product;
mod sub;

pub use add::Add;
pub use conjugate::Conjugate;
pub use div::Div;
pub use dot::Dot;
pub use geometric_product::GeometricProduct;
pub use grade_select::GradeSelect;
pub use hodge_dual::HodgeDual;
pub use ideal_norm::{IdealNorm, IdealNormSquared};
pub use inverse::Inverse;
pub use join::Join;
pub use meet::Meet;
pub use mul::Mul;
pub use neg::Neg;
pub use norm::{Norm, NormSquared};
pub use normalise::Normalise;
pub use partial_eq::PartialEq;
pub use reverse::Reverse;
pub use scalar_product::ScalarProduct;
pub use sub::Sub;

use crate::{values::Empty, Zero};

/// Grade Involution
pub trait Involution {
  type Output;

  /// Grade Involution
  fn involution(self) -> Self;
}

/// Exponentiation
pub trait Exponent<Rhs> {
  type Output;

  /// Exponentiation
  fn exp(self, rhs: Rhs) -> Self::Output;
}

/// return `Empty`
#[inline]
fn return_empty<Lhs, Rhs>(_: Lhs, _: Rhs) -> Empty {
  Empty
}

/// return the left-hand-side
#[inline]
fn return_lhs<Lhs, Rhs>(lhs: Lhs, _: Rhs) -> Lhs {
  lhs
}

/// return the right-hand-side
#[inline]
fn return_rhs<Lhs, Rhs>(_: Lhs, rhs: Rhs) -> Rhs {
  rhs
}

/// negate and return the right-hand-side
#[inline]
fn return_neg_rhs<Lhs, Rhs: Neg<Output = Rhs>>(_: Lhs, rhs: Rhs) -> Rhs {
  -rhs
}

/// return zero
#[inline]
fn return_zero<Lhs, Rhs, Output: Zero>(_: Lhs, _: Rhs) -> Output {
  Output::zero()
}
