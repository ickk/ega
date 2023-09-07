use crate::values::*;

/// The zero value
pub trait Zero {
  type Output;

  /// The zero value
  fn zero() -> Self::Output;
}

macro_rules! impl_zero {
  ($type:ty, $count:literal) => {
    impl Zero for $type {
      type Output = $type;

      #[inline]
      fn zero() -> Self::Output {
        Self::Output::default()
      }
    }
  };
}

impl_zero! { Multivector, 16 }
impl_zero! { Empty, 0 }
impl_zero! { Scalar, 1 }
impl_zero! { Vector, 4 }
impl_zero! { Bivector, 6 }
impl_zero! { Trivector, 4 }
impl_zero! { Pseudoscalar, 1 }
