use crate::*;

pub use ::core::cmp::PartialEq;

macro_rules! impl_eq_when_zero {
  ($lhs:ty, $rhs:ty) => {
    impl PartialEq<$rhs> for $lhs {
      #[inline]
      fn eq(&self, rhs: &$rhs) -> bool {
        *self == zero::<$lhs>() && *rhs == zero::<$rhs>()
      }
    }
  };
}

impl_eq_when_zero! { Scalar, Vector }
impl_eq_when_zero! { Scalar, Bivector }
impl_eq_when_zero! { Scalar, Trivector }
impl_eq_when_zero! { Scalar, Pseudoscalar }

impl_eq_when_zero! { Vector, Scalar }
impl_eq_when_zero! { Vector, Bivector }
impl_eq_when_zero! { Vector, Trivector }
impl_eq_when_zero! { Vector, Pseudoscalar }

impl_eq_when_zero! { Bivector, Scalar }
impl_eq_when_zero! { Bivector, Vector }
impl_eq_when_zero! { Bivector, Trivector }
impl_eq_when_zero! { Bivector, Pseudoscalar }

impl_eq_when_zero! { Trivector, Scalar }
impl_eq_when_zero! { Trivector, Vector }
impl_eq_when_zero! { Trivector, Bivector }
impl_eq_when_zero! { Trivector, Pseudoscalar }

impl_eq_when_zero! { Pseudoscalar, Scalar }
impl_eq_when_zero! { Pseudoscalar, Vector }
impl_eq_when_zero! { Pseudoscalar, Bivector }
impl_eq_when_zero! { Pseudoscalar, Trivector }

macro_rules! impl_eq_from_rhs {
  ($lhs:ty, $rhs:ty) => {
    impl PartialEq<$rhs> for $lhs {
      #[inline]
      fn eq(&self, rhs: &$rhs) -> bool {
        let rhs = <$lhs>::from(*rhs);
        *self == rhs
      }
    }
  };
}

impl_eq_from_rhs! { Multivector, Scalar }
impl_eq_from_rhs! { Multivector, Vector }
impl_eq_from_rhs! { Multivector, Bivector }
impl_eq_from_rhs! { Multivector, Trivector }
impl_eq_from_rhs! { Multivector, Pseudoscalar }

macro_rules! impl_eq_from_lhs {
  ($lhs:ty, $rhs:ty) => {
    impl PartialEq<$rhs> for $lhs {
      #[inline]
      fn eq(&self, rhs: &$rhs) -> bool {
        let lhs = <$rhs>::from(*self);
        lhs == *rhs
      }
    }
  };
}

impl_eq_from_lhs! { Scalar, Multivector }
impl_eq_from_lhs! { Vector, Multivector }
impl_eq_from_lhs! { Bivector, Multivector }
impl_eq_from_lhs! { Trivector, Multivector }
impl_eq_from_lhs! { Pseudoscalar, Multivector }
