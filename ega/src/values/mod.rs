mod bivector;
mod empty;
mod multivector;
mod pseudoscalar;
mod scalar;
mod trivector;
mod vector;
mod zero;

pub use bivector::*;
pub use empty::*;
pub use multivector::*;
pub use pseudoscalar::*;
pub use scalar::*;
pub use trivector::*;
pub use vector::*;
pub use {zero::zero, zero::Zero};

// Some of the supertraits rely on std/libm (i.e. Norm)
#[cfg(any(feature = "std", feature = "libm"))]
pub use ega_value::EgaValue;
#[cfg(any(feature = "std", feature = "libm"))]
mod ega_value {
  use crate::*;

  pub trait EgaValue:
    Copy
    + Clone
    + core::fmt::Debug
    + PartialEq
    + Zero
    + Neg
    + GradeSelect
    + Reverse
    + Conjugate
    + HodgeDual
    + NormSquared
    + Norm
    + IdealNormSquared
    + IdealNorm
    + Add<Multivector>
    + Add<Empty>
    + Add<Scalar>
    + Add<Vector>
    + Add<Bivector>
    + Add<Trivector>
    + Add<Pseudoscalar>
    + Sub<Multivector>
    + Sub<Empty>
    + Sub<Scalar>
    + Sub<Vector>
    + Sub<Bivector>
    + Sub<Trivector>
    + Sub<Pseudoscalar>
    + Meet<Multivector>
    + Meet<Empty>
    + Meet<Scalar>
    + Meet<Vector>
    + Meet<Bivector>
    + Meet<Trivector>
    + Meet<Pseudoscalar>
    + Dot<Multivector>
    + Dot<Empty>
    + Dot<Scalar>
    + Dot<Vector>
    + Dot<Bivector>
    + Dot<Trivector>
    + Dot<Pseudoscalar>
    + Join<Multivector>
    + Join<Empty>
    + Join<Scalar>
    + Join<Vector>
    + Join<Bivector>
    + Join<Trivector>
    + Join<Pseudoscalar>
    + GeometricProduct<Multivector>
    + GeometricProduct<Empty>
    + GeometricProduct<Scalar>
    + GeometricProduct<Vector>
    + GeometricProduct<Bivector>
    + GeometricProduct<Trivector>
    + GeometricProduct<Pseudoscalar>
    + ScalarProduct<Multivector>
    + ScalarProduct<Empty>
    + ScalarProduct<Scalar>
    + ScalarProduct<Vector>
    + ScalarProduct<Bivector>
    + ScalarProduct<Trivector>
    + ScalarProduct<Pseudoscalar>
  {
  }

  impl EgaValue for Multivector {}
  impl EgaValue for Empty {}
  impl EgaValue for Scalar {}
  impl EgaValue for Vector {}
  impl EgaValue for Bivector {}
  impl EgaValue for Trivector {}
  impl EgaValue for Pseudoscalar {}

  mod private {
    pub trait Sealed {}
  }
}

/// Create accessor functions for elements of a field
///
/// usage: `accessors { <pub?> <field>[<type>]: <a1>[<i1>], <a2>[<i2>] }`
///
/// where you want to create accessors `self.a1()`, `self.a2()` that index into
/// `self.field[i1]` & `self.field[i2]`
macro_rules! accessors {
  (
    $visability:vis $field_name:ident[$element_type:ty]: $(
      $accessor_name:ident[$index:literal]
    ),+ $(,)?
  ) => {
    $(
      #[inline]
      $visability const fn $accessor_name(&self) -> $element_type {
        self.$field_name[$index]
      }
    )+
  }
}
pub(crate) use accessors;
