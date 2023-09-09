use crate::*;

pub fn zero<T: Zero>() -> T {
  <T as Zero>::zero()
}

/// The zero value
pub trait Zero {
  /// The zero value
  fn zero() -> Self;
}

macro_rules! impl_zero {
  ($type:ty, $count:literal) => {
    impl Zero for $type {
      #[inline]
      fn zero() -> Self {
        Self::default()
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
