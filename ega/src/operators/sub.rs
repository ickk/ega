use super::{return_lhs, return_rhs};
use crate::*;
pub use core::ops::Sub;

macro_rules! impl_sub {
  ($sub_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl Sub<$rhs> for $lhs {
      type Output = $output;
      #[inline]
      fn sub(self, rhs: $rhs) -> Self::Output {
        $sub_fn(self, rhs)
      }
    }
  };
}

impl_sub! { multivector_sub_multivector: Multivector, Multivector => Multivector }
impl_sub! { multivector_sub_scalar: Multivector, Scalar => Multivector }
impl_sub! { multivector_sub_vector: Multivector, Vector => Multivector }
impl_sub! { multivector_sub_bivector: Multivector, Bivector => Multivector }
impl_sub! { multivector_sub_trivector: Multivector, Trivector => Multivector }
impl_sub! { multivector_sub_pseudoscalar: Multivector, Pseudoscalar => Multivector }
impl_sub! { return_lhs: Multivector, Empty => Multivector }

impl_sub! { scalar_sub_multivector: Scalar, Multivector => Multivector }
impl_sub! { scalar_sub_scalar: Scalar, Scalar => Scalar }
impl_sub! { scalar_sub_vector: Scalar, Vector => Multivector }
impl_sub! { scalar_sub_bivector: Scalar, Bivector => Multivector }
impl_sub! { scalar_sub_trivector: Scalar, Trivector => Multivector }
impl_sub! { scalar_sub_pseudoscalar: Scalar, Pseudoscalar => Multivector }
impl_sub! { return_lhs: Scalar, Empty => Scalar }

impl_sub! { vector_sub_multivector: Vector, Multivector => Multivector }
impl_sub! { vector_sub_scalar: Vector, Scalar => Multivector }
impl_sub! { vector_sub_vector: Vector, Vector => Vector }
impl_sub! { vector_sub_bivector: Vector, Bivector => Multivector }
impl_sub! { vector_sub_trivector: Vector, Trivector => Multivector }
impl_sub! { vector_sub_pseudoscalar: Vector, Pseudoscalar => Multivector }
impl_sub! { return_lhs: Vector, Empty => Vector }

impl_sub! { bivector_sub_multivector: Bivector, Multivector => Multivector }
impl_sub! { bivector_sub_scalar: Bivector, Scalar => Multivector }
impl_sub! { bivector_sub_vector: Bivector, Vector => Multivector }
impl_sub! { bivector_sub_bivector: Bivector, Bivector => Bivector }
impl_sub! { bivector_sub_trivector: Bivector, Trivector => Multivector }
impl_sub! { bivector_sub_pseudoscalar: Bivector, Pseudoscalar => Multivector }
impl_sub! { return_lhs: Bivector, Empty => Bivector }

impl_sub! { trivector_sub_multivector: Trivector, Multivector => Multivector }
impl_sub! { trivector_sub_scalar: Trivector, Scalar => Multivector }
impl_sub! { trivector_sub_vector: Trivector, Vector => Multivector }
impl_sub! { trivector_sub_bivector: Trivector, Bivector => Multivector }
impl_sub! { trivector_sub_trivector: Trivector, Trivector => Trivector }
impl_sub! { trivector_sub_pseudoscalar: Trivector, Pseudoscalar => Multivector }
impl_sub! { return_lhs: Trivector, Empty => Trivector }

impl_sub! { pseudoscalar_sub_multivector: Pseudoscalar, Multivector => Multivector }
impl_sub! { pseudoscalar_sub_scalar: Pseudoscalar, Scalar => Multivector }
impl_sub! { pseudoscalar_sub_vector: Pseudoscalar, Vector => Multivector }
impl_sub! { pseudoscalar_sub_bivector: Pseudoscalar, Bivector => Multivector }
impl_sub! { pseudoscalar_sub_trivector: Pseudoscalar, Trivector => Multivector }
impl_sub! { pseudoscalar_sub_pseudoscalar: Pseudoscalar, Pseudoscalar => Pseudoscalar }
impl_sub! { return_lhs: Pseudoscalar, Empty => Pseudoscalar }

impl_sub! { return_rhs: Empty, Empty => Empty }
impl_sub! { return_rhs: Empty, Scalar => Scalar }
impl_sub! { return_rhs: Empty, Vector => Vector }
impl_sub! { return_rhs: Empty, Bivector => Bivector }
impl_sub! { return_rhs: Empty, Trivector => Trivector }
impl_sub! { return_rhs: Empty, Pseudoscalar => Pseudoscalar }

// Multivector

#[inline]
fn multivector_sub_multivector(
  mut lhs: Multivector,
  rhs: Multivector,
) -> Multivector {
  lhs.s -= rhs.s;
  lhs.e0 -= rhs.e0;
  lhs.e1 -= rhs.e1;
  lhs.e2 -= rhs.e2;
  lhs.e3 -= rhs.e3;
  lhs.e01 -= rhs.e01;
  lhs.e02 -= rhs.e02;
  lhs.e03 -= rhs.e03;
  lhs.e23 -= rhs.e23;
  lhs.e31 -= rhs.e31;
  lhs.e12 -= rhs.e12;
  lhs.e032 -= rhs.e032;
  lhs.e021 -= rhs.e021;
  lhs.e013 -= rhs.e013;
  lhs.e123 -= rhs.e123;
  lhs.e0123 -= rhs.e0123;

  lhs
}

#[inline]
fn multivector_sub_scalar(mut lhs: Multivector, rhs: Scalar) -> Multivector {
  lhs.s -= rhs.s;

  lhs
}

#[inline]
fn multivector_sub_vector(mut lhs: Multivector, rhs: Vector) -> Multivector {
  lhs.e0 -= rhs.e0;
  lhs.e1 -= rhs.e1;
  lhs.e2 -= rhs.e2;
  lhs.e3 -= rhs.e3;

  lhs
}

#[inline]
fn multivector_sub_bivector(
  mut lhs: Multivector,
  rhs: Bivector,
) -> Multivector {
  lhs.e01 -= rhs.e01;
  lhs.e02 -= rhs.e02;
  lhs.e03 -= rhs.e03;
  lhs.e23 -= rhs.e23;
  lhs.e31 -= rhs.e31;
  lhs.e12 -= rhs.e12;

  lhs
}

#[inline]
fn multivector_sub_trivector(
  mut lhs: Multivector,
  rhs: Trivector,
) -> Multivector {
  lhs.e032 -= rhs.e032;
  lhs.e021 -= rhs.e021;
  lhs.e013 -= rhs.e013;
  lhs.e123 -= rhs.e123;

  lhs
}

#[inline]
fn multivector_sub_pseudoscalar(
  mut lhs: Multivector,
  rhs: Pseudoscalar,
) -> Multivector {
  lhs.e0123 -= rhs.e0123;

  lhs
}

// Scalar

#[inline]
fn scalar_sub_multivector(lhs: Scalar, mut rhs: Multivector) -> Multivector {
  rhs = -rhs;

  rhs.s += lhs.s;

  rhs
}

#[inline]
fn scalar_sub_scalar(mut lhs: Scalar, rhs: Scalar) -> Scalar {
  lhs.s -= rhs.s;

  lhs
}

#[inline]
fn scalar_sub_vector(lhs: Scalar, rhs: Vector) -> Multivector {
  Multivector {
    s: lhs.s,
    e0: -rhs.e0,
    e1: -rhs.e1,
    e2: -rhs.e2,
    e3: -rhs.e3,
    ..zero()
  }
}

#[inline]
fn scalar_sub_bivector(lhs: Scalar, rhs: Bivector) -> Multivector {
  Multivector {
    s: lhs.s,
    e01: -rhs.e01,
    e02: -rhs.e02,
    e03: -rhs.e03,
    e23: -rhs.e23,
    e31: -rhs.e31,
    e12: -rhs.e12,
    ..zero()
  }
}

#[inline]
fn scalar_sub_trivector(lhs: Scalar, rhs: Trivector) -> Multivector {
  Multivector {
    s: lhs.s,
    e123: -rhs.e123,
    e032: -rhs.e032,
    e013: -rhs.e013,
    e021: -rhs.e021,
    ..zero()
  }
}

#[inline]
fn scalar_sub_pseudoscalar(lhs: Scalar, rhs: Pseudoscalar) -> Multivector {
  Multivector {
    s: lhs.s,
    e0123: -rhs.e0123,
    ..zero()
  }
}

// Vector

#[inline]
fn vector_sub_multivector(lhs: Vector, mut rhs: Multivector) -> Multivector {
  rhs = -rhs;

  rhs.e0 += lhs.e0;
  rhs.e1 += lhs.e1;
  rhs.e2 += lhs.e2;
  rhs.e3 += lhs.e3;

  rhs
}

#[inline]
fn vector_sub_scalar(lhs: Vector, rhs: Scalar) -> Multivector {
  Multivector {
    s: -rhs.s,
    e0: lhs.e0,
    e1: lhs.e1,
    e2: lhs.e2,
    e3: lhs.e3,
    ..zero()
  }
}

#[inline]
fn vector_sub_vector(mut lhs: Vector, rhs: Vector) -> Vector {
  lhs.e0 -= rhs.e0;
  lhs.e1 -= rhs.e1;
  lhs.e2 -= rhs.e2;
  lhs.e3 -= rhs.e3;

  lhs
}

#[inline]
fn vector_sub_bivector(lhs: Vector, rhs: Bivector) -> Multivector {
  Multivector {
    e0: lhs.e0,
    e1: lhs.e1,
    e2: lhs.e2,
    e3: lhs.e3,
    e01: -rhs.e01,
    e02: -rhs.e02,
    e03: -rhs.e03,
    e23: -rhs.e23,
    e31: -rhs.e31,
    e12: -rhs.e12,
    ..zero()
  }
}

#[inline]
fn vector_sub_trivector(lhs: Vector, rhs: Trivector) -> Multivector {
  Multivector {
    e0: lhs.e0,
    e1: lhs.e1,
    e2: lhs.e2,
    e3: lhs.e3,
    e123: -rhs.e123,
    e032: -rhs.e032,
    e013: -rhs.e013,
    e021: -rhs.e021,
    ..zero()
  }
}

#[inline]
fn vector_sub_pseudoscalar(lhs: Vector, rhs: Pseudoscalar) -> Multivector {
  Multivector {
    e0: lhs.e0,
    e1: lhs.e1,
    e2: lhs.e2,
    e3: lhs.e3,
    e0123: -rhs.e0123,
    ..zero()
  }
}

// Bivector

#[inline]
fn bivector_sub_multivector(
  lhs: Bivector,
  mut rhs: Multivector,
) -> Multivector {
  rhs = -rhs;

  rhs.e01 += lhs.e01;
  rhs.e02 += lhs.e02;
  rhs.e03 += lhs.e03;
  rhs.e23 += lhs.e23;
  rhs.e31 += lhs.e31;
  rhs.e12 += lhs.e12;

  rhs
}

#[inline]
fn bivector_sub_scalar(lhs: Bivector, rhs: Scalar) -> Multivector {
  Multivector {
    s: -rhs.s,
    e01: lhs.e01,
    e02: lhs.e02,
    e03: lhs.e03,
    e23: lhs.e23,
    e31: lhs.e31,
    e12: lhs.e12,
    ..zero()
  }
}

#[inline]
fn bivector_sub_vector(lhs: Bivector, rhs: Vector) -> Multivector {
  Multivector {
    e0: -rhs.e0,
    e1: -rhs.e1,
    e2: -rhs.e2,
    e3: -rhs.e3,
    e01: lhs.e01,
    e02: lhs.e02,
    e03: lhs.e03,
    e23: lhs.e23,
    e31: lhs.e31,
    e12: lhs.e12,
    ..zero()
  }
}

#[inline]
fn bivector_sub_bivector(mut lhs: Bivector, rhs: Bivector) -> Bivector {
  lhs.e01 -= rhs.e01;
  lhs.e02 -= rhs.e02;
  lhs.e03 -= rhs.e03;
  lhs.e23 -= rhs.e23;
  lhs.e31 -= rhs.e31;
  lhs.e12 -= rhs.e12;

  lhs
}

#[inline]
fn bivector_sub_trivector(lhs: Bivector, rhs: Trivector) -> Multivector {
  Multivector {
    e01: lhs.e01,
    e02: lhs.e02,
    e03: lhs.e03,
    e23: lhs.e23,
    e31: lhs.e31,
    e12: lhs.e12,
    e123: -rhs.e123,
    e032: -rhs.e032,
    e013: -rhs.e013,
    e021: -rhs.e021,
    ..zero()
  }
}

#[inline]
fn bivector_sub_pseudoscalar(lhs: Bivector, rhs: Pseudoscalar) -> Multivector {
  Multivector {
    e01: lhs.e01,
    e02: lhs.e02,
    e03: lhs.e03,
    e23: lhs.e23,
    e31: lhs.e31,
    e12: lhs.e12,
    e0123: -rhs.e0123,
    ..zero()
  }
}

// Trivector

#[inline]
fn trivector_sub_multivector(
  lhs: Trivector,
  mut rhs: Multivector,
) -> Multivector {
  rhs = -rhs;

  rhs.e123 += lhs.e123;
  rhs.e032 += lhs.e032;
  rhs.e013 += lhs.e013;
  rhs.e021 += lhs.e021;

  rhs
}

#[inline]
fn trivector_sub_scalar(lhs: Trivector, rhs: Scalar) -> Multivector {
  Multivector {
    s: -rhs.s,
    e123: lhs.e123,
    e032: lhs.e032,
    e013: lhs.e013,
    e021: lhs.e021,
    ..zero()
  }
}

#[inline]
fn trivector_sub_vector(lhs: Trivector, rhs: Vector) -> Multivector {
  Multivector {
    e0: -rhs.e0,
    e1: -rhs.e1,
    e2: -rhs.e2,
    e3: -rhs.e3,
    e123: lhs.e123,
    e032: lhs.e032,
    e013: lhs.e013,
    e021: lhs.e021,
    ..zero()
  }
}

#[inline]
fn trivector_sub_bivector(lhs: Trivector, rhs: Bivector) -> Multivector {
  Multivector {
    e01: -rhs.e01,
    e02: -rhs.e02,
    e03: -rhs.e03,
    e23: -rhs.e23,
    e31: -rhs.e31,
    e12: -rhs.e12,
    e123: lhs.e123,
    e032: lhs.e032,
    e013: lhs.e013,
    e021: lhs.e021,
    ..zero()
  }
}

#[inline]
fn trivector_sub_trivector(mut lhs: Trivector, rhs: Trivector) -> Trivector {
  lhs.e123 -= rhs.e123;
  lhs.e032 -= rhs.e032;
  lhs.e013 -= rhs.e013;
  lhs.e021 -= rhs.e021;

  lhs
}

#[inline]
fn trivector_sub_pseudoscalar(
  lhs: Trivector,
  rhs: Pseudoscalar,
) -> Multivector {
  Multivector {
    e123: lhs.e123,
    e032: lhs.e032,
    e013: lhs.e013,
    e021: lhs.e021,
    e0123: -rhs.e0123,
    ..zero()
  }
}

// Pseudoscalar

#[inline]
fn pseudoscalar_sub_multivector(
  lhs: Pseudoscalar,
  mut rhs: Multivector,
) -> Multivector {
  rhs = -rhs;

  rhs.e0123 += lhs.e0123;

  rhs
}

#[inline]
fn pseudoscalar_sub_scalar(lhs: Pseudoscalar, rhs: Scalar) -> Multivector {
  Multivector {
    s: -rhs.s,
    e0123: lhs.e0123,
    ..zero()
  }
}

#[inline]
fn pseudoscalar_sub_vector(lhs: Pseudoscalar, rhs: Vector) -> Multivector {
  Multivector {
    e0: -rhs.e0,
    e1: -rhs.e1,
    e2: -rhs.e2,
    e3: -rhs.e3,
    e0123: lhs.e0123,
    ..zero()
  }
}

#[inline]
fn pseudoscalar_sub_bivector(lhs: Pseudoscalar, rhs: Bivector) -> Multivector {
  Multivector {
    e01: -rhs.e01,
    e02: -rhs.e02,
    e03: -rhs.e03,
    e23: -rhs.e23,
    e31: -rhs.e31,
    e12: -rhs.e12,
    e0123: lhs.e0123,
    ..zero()
  }
}

#[inline]
fn pseudoscalar_sub_trivector(
  lhs: Pseudoscalar,
  rhs: Trivector,
) -> Multivector {
  Multivector {
    e123: -rhs.e123,
    e032: -rhs.e032,
    e013: -rhs.e013,
    e021: -rhs.e021,
    e0123: lhs.e0123,
    ..zero()
  }
}

#[inline]
fn pseudoscalar_sub_pseudoscalar(
  mut lhs: Pseudoscalar,
  rhs: Pseudoscalar,
) -> Pseudoscalar {
  lhs.e0123 -= rhs.e0123;

  lhs
}
