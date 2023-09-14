use crate::*;

pub use core::ops::Div;

macro_rules! impl_div {
  ($lhs:ty, f32 => $output:ty) => {
    #[allow(clippy::suspicious_arithmetic_impl)]
    impl Div<f32> for $lhs {
      type Output = $output;

      #[inline]
      fn div(self, rhs: f32) -> Self::Output {
        self * Scalar { s: 1.0 / rhs }
      }
    }
  };
  ($lhs:ty, $rhs:ty => $output:ty) => {
    #[allow(clippy::suspicious_arithmetic_impl)]
    impl Div<$rhs> for $lhs {
      type Output = $output;

      #[inline]
      fn div(self, rhs: $rhs) -> Self::Output {
        self * rhs.inverse()
      }
    }
  };
}

impl_div! { Scalar, f32 => Scalar }
impl_div! { f32, Scalar => Scalar }
impl_div! { Scalar, Scalar => Scalar }
impl_div! { f32, Vector => Vector }
impl_div! { Scalar, Vector => Vector }
impl_div! { f32, Trivector => Trivector }
impl_div! { Scalar, Trivector => Trivector }

impl_div! { Vector, f32 => Vector }
impl_div! { Vector, Scalar => Vector }
impl_div! { Vector, Vector => Multivector }
impl_div! { Vector, Trivector => Multivector }

impl_div! { Bivector, f32 => Bivector }
impl_div! { Bivector, Scalar => Bivector }
impl_div! { Bivector, Vector => Multivector }
impl_div! { Bivector, Trivector => Multivector }

impl_div! { Trivector, f32 => Trivector }
impl_div! { Trivector, Scalar => Trivector }
impl_div! { Trivector, Vector => Multivector }
impl_div! { Trivector, Trivector => Multivector }
