use crate::*;

/// The `GeometricProduct` as the multiplication operator `*`.
pub use core::ops::Mul;

macro_rules! impl_mul {
  // we wrap `f32`s with `Scalar`
  ($lhs:ty, f32 => $output:ty) => {
    impl Mul<f32> for $lhs {
      type Output = $output;
      #[inline]
      fn mul(self, rhs: lhs) -> Self::Output {
        self.geometric_product(Scalar { s: rhs })
      }
    }
  };
  (f32, $rhs:ty => $output:ty) => {
    impl Mul<$rhs> for f32 {
      type Output = $output;
      #[inline]
      fn mul(self, rhs: $rhs) -> Self::Output {
        Scalar { s: self }.geometric_product(rhs)
      }
    }
  };
  ($lhs:ty, $rhs:ty => $output:ty) => {
    impl Mul<$rhs> for $lhs {
      type Output = $output;
      #[inline]
      fn mul(self, rhs: $rhs) -> Self::Output {
        self.geometric_product(rhs)
      }
    }
  };
}

// impl_mul! { f32, Multivector => Multivector }
// impl_mul! { f32, Scalar => Scalar }
// impl_mul! { f32, Vector => Multivector }
// impl_mul! { f32, Bivector => Multivector }
// impl_mul! { f32, Trivector => Multivector }
// impl_mul! { f32, Pseudoscalar => Multivector }
// impl_mul! { f32, Empty => Scalar }

// impl_mul! { Multivector, f32 => Multivector }
// impl_mul! { Scalar, f32 => Scalar }
// impl_mul! { Vector, f32 => Multivector }
// impl_mul! { Bivector, f32 => Multivector }
// impl_mul! { Trivector, f32 => Multivector }
// impl_mul! { Pseudoscalar, f32 => Multivector }
// impl_mul! { Empty, f32 => Scalar }

impl_mul! { Multivector, Multivector => Multivector }
// impl_mul! { Multivector, Scalar => Multivector }
// impl_mul! { Multivector, Vector => Multivector }
// impl_mul! { Multivector, Bivector => Multivector }
// impl_mul! { Multivector, Trivector => Multivector }
// impl_mul! { Multivector, Pseudoscalar => Multivector }
// impl_mul! { Multivector, Empty => Multivector }

// impl_mul! { Scalar, Multivector => Multivector }
// impl_mul! { Scalar, Scalar => Scalar }
// impl_mul! { Scalar, Vector => Multivector }
// impl_mul! { Scalar, Bivector => Multivector }
// impl_mul! { Scalar, Trivector => Multivector }
// impl_mul! { Scalar, Pseudoscalar => Multivector }
// impl_mul! { Scalar, Empty => Scalar }

// impl_mul! { Vector, Multivector => Multivector }
// impl_mul! { Vector, Scalar => Multivector }
// impl_mul! { Vector, Vector => Vector }
// impl_mul! { Vector, Bivector => Multivector }
// impl_mul! { Vector, Trivector => Multivector }
// impl_mul! { Vector, Pseudoscalar => Multivector }
// impl_mul! { Vector, Empty => Vector }

// impl_mul! { Bivector, Multivector => Multivector }
// impl_mul! { Bivector, Scalar => Multivector }
// impl_mul! { Bivector, Vector => Multivector }
// impl_mul! { Bivector, Bivector => Bivector }
// impl_mul! { Bivector, Trivector => Multivector }
// impl_mul! { Bivector, Pseudoscalar => Multivector }
// impl_mul! { Bivector, Empty => Bivector }

// impl_mul! { Trivector, Multivector => Multivector }
// impl_mul! { Trivector, Scalar => Multivector }
// impl_mul! { Trivector, Vector => Multivector }
// impl_mul! { Trivector, Bivector => Multivector }
// impl_mul! { Trivector, Trivector => Trivector }
// impl_mul! { Trivector, Pseudoscalar => Multivector }
// impl_mul! { Trivector, Empty => Trivector }

// impl_mul! { Pseudoscalar, Multivector => Multivector }
// impl_mul! { Pseudoscalar, Scalar => Multivector }
// impl_mul! { Pseudoscalar, Vector => Multivector }
// impl_mul! { Pseudoscalar, Bivector => Multivector }
// impl_mul! { Pseudoscalar, Trivector => Multivector }
// impl_mul! { Pseudoscalar, Pseudoscalar => Pseudoscalar }
// impl_mul! { Pseudoscalar, Empty => Pseudoscalar }

// impl_mul! { Empty, Empty => Empty }
// impl_mul! { Empty, Scalar => Scalar }
// impl_mul! { Empty, Vector => Vector }
// impl_mul! { Empty, Bivector => Bivector }
// impl_mul! { Empty, Trivector => Trivector }
// impl_mul! { Empty, Pseudoscalar => Pseudoscalar }
