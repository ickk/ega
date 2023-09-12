mod add;
mod conjugate;
mod dot;
mod geometric_product;
mod grade_select;
mod hodge_dual;
mod ideal_norm;
mod join;
mod meet;
mod mul;
mod neg;
mod norm;
mod normalise;
mod reverse;
mod sub;

pub use add::Add;
pub use conjugate::Conjugate;
pub use dot::Dot;
pub use geometric_product::GeometricProduct;
pub use grade_select::GradeSelect;
pub use hodge_dual::HodgeDual;
pub use ideal_norm::{IdealNorm, IdealNormSquared};
pub use join::Join;
pub use meet::Meet;
pub use mul::Mul;
pub use neg::Neg;
pub use norm::{Norm, NormSquared};
pub use normalise::Normalise;
pub use reverse::Reverse;
pub use sub::Sub;

use crate::values::Empty;

/// Grade Involution
pub trait Involution {
  type Output;

  /// Grade Involution
  fn involution(self) -> Self;
}

/// Scalar Product
pub trait ScalarProduct {
  type Output;
  /// Scalar Product, "star operator"
  ///
  /// not to be confused with the geometric product of a Scalar & Multivector
  fn scalar_product(self) -> Self::Output;
}

/// Left Contraction
pub trait LeftContraction {
  type Output;

  fn left_contraction(self) -> Self::Output;
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
