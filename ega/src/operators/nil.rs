use crate::values::*;

pub trait Nil {
  type Output;

  /// The zero value
  fn nil() -> Self::Output;
}

macro_rules! impl_nil {
  ($type:ty, $count:literal) => {
    impl Nil for $type {
      type Output = $type;

      #[inline]
      fn nil() -> Self::Output {
        Self::Output::from([0f32; $count])
      }
    }
  };
}

impl_nil! { Multivector, 16 }
impl_nil! { Empty, 0 }
impl_nil! { Scalar, 1 }
impl_nil! { Vector, 4 }
impl_nil! { Bivector, 6 }
impl_nil! { Trivector, 4 }
impl_nil! { Pseudoscalar, 1 }
