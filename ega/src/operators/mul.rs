use crate::*;

pub use ::core::ops::Mul;

macro_rules! impl_mul {
  // we wrap `f32`s with `Scalar`
  ($lhs:ty, f32 => $output:ty) => {
    impl Mul<f32> for $lhs {
      type Output = $output;

      /// The `GeometricProduct`
      #[inline]
      fn mul(self, rhs: f32) -> Self::Output {
        self.geometric_product(Scalar { s: rhs })
      }
    }
  };
  (f32, $rhs:ty => $output:ty) => {
    impl Mul<$rhs> for f32 {
      type Output = $output;

      /// The `GeometricProduct`
      #[inline]
      fn mul(self, rhs: $rhs) -> Self::Output {
        Scalar { s: self }.geometric_product(rhs)
      }
    }
  };
  ($lhs:ty, $rhs:ty => $output:ty) => {
    impl Mul<$rhs> for $lhs {
      type Output = $output;

      /// The `GeometricProduct`
      #[inline]
      fn mul(self, rhs: $rhs) -> Self::Output {
        self.geometric_product(rhs)
      }
    }
  };
}

impl_mul! { f32, Multivector => Multivector }
impl_mul! { f32, Scalar => Scalar }
impl_mul! { f32, Vector => Vector }
impl_mul! { f32, Bivector => Bivector }
impl_mul! { f32, Trivector => Trivector }
impl_mul! { f32, Pseudoscalar => Pseudoscalar }
impl_mul! { f32, Empty => Empty }

impl_mul! { Multivector, f32 => Multivector }
impl_mul! { Scalar, f32 => Scalar }
impl_mul! { Vector, f32 => Vector }
impl_mul! { Bivector, f32 => Bivector }
impl_mul! { Trivector, f32 => Trivector }
impl_mul! { Pseudoscalar, f32 => Pseudoscalar }
impl_mul! { Empty, f32 => Empty }

impl_mul! { Multivector, Multivector => Multivector }
impl_mul! { Multivector, Scalar => Multivector }
impl_mul! { Multivector, Vector => Multivector }
impl_mul! { Multivector, Bivector => Multivector }
impl_mul! { Multivector, Trivector => Multivector }
impl_mul! { Multivector, Pseudoscalar => Multivector }
impl_mul! { Multivector, Empty => Empty }

impl_mul! { Scalar, Multivector => Multivector }
impl_mul! { Scalar, Scalar => Scalar }
impl_mul! { Scalar, Vector => Vector }
impl_mul! { Scalar, Bivector => Bivector }
impl_mul! { Scalar, Trivector => Trivector }
impl_mul! { Scalar, Pseudoscalar => Pseudoscalar }
impl_mul! { Scalar, Empty => Empty }

impl_mul! { Vector, Multivector => Multivector }
impl_mul! { Vector, Scalar => Vector }
impl_mul! { Vector, Vector => Multivector }
impl_mul! { Vector, Bivector => Multivector }
impl_mul! { Vector, Trivector => Multivector }
impl_mul! { Vector, Pseudoscalar => Trivector }
impl_mul! { Vector, Empty => Empty }

impl_mul! { Bivector, Multivector => Multivector }
impl_mul! { Bivector, Scalar => Bivector }
impl_mul! { Bivector, Vector => Multivector }
impl_mul! { Bivector, Bivector => Multivector }
impl_mul! { Bivector, Trivector => Multivector }
impl_mul! { Bivector, Pseudoscalar => Bivector }
impl_mul! { Bivector, Empty => Empty }

impl_mul! { Trivector, Multivector => Multivector }
impl_mul! { Trivector, Scalar => Trivector }
impl_mul! { Trivector, Vector => Multivector }
impl_mul! { Trivector, Bivector => Multivector }
impl_mul! { Trivector, Trivector => Multivector }
impl_mul! { Trivector, Pseudoscalar => Vector }
impl_mul! { Trivector, Empty => Empty }

impl_mul! { Pseudoscalar, Multivector => Multivector }
impl_mul! { Pseudoscalar, Scalar => Pseudoscalar }
impl_mul! { Pseudoscalar, Vector => Trivector }
impl_mul! { Pseudoscalar, Bivector => Bivector }
impl_mul! { Pseudoscalar, Trivector => Vector }
impl_mul! { Pseudoscalar, Pseudoscalar => Empty }
impl_mul! { Pseudoscalar, Empty => Empty }

impl_mul! { Empty, Multivector => Empty }
impl_mul! { Empty, Scalar => Empty }
impl_mul! { Empty, Vector => Empty }
impl_mul! { Empty, Bivector => Empty }
impl_mul! { Empty, Trivector => Empty }
impl_mul! { Empty, Pseudoscalar => Empty }
impl_mul! { Empty, Empty => Empty }
