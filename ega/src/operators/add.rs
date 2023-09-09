use super::{return_lhs, return_rhs};
use crate::*;
pub use core::ops::Add;

macro_rules! impl_add {
  ($add_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl Add<$rhs> for $lhs {
      type Output = $output;
      #[inline]
      fn add(self, rhs: $rhs) -> Self::Output {
        $add_fn(self, rhs)
      }
    }
  };
}

impl_add! { multivector_add_multivector: Multivector, Multivector => Multivector }
impl_add! { multivector_add_scalar: Multivector, Scalar => Multivector }
impl_add! { multivector_add_vector: Multivector, Vector => Multivector }
impl_add! { multivector_add_bivector: Multivector, Bivector => Multivector }
impl_add! { multivector_add_trivector: Multivector, Trivector => Multivector }
impl_add! { multivector_add_pseudoscalar: Multivector, Pseudoscalar => Multivector }
impl_add! { return_lhs: Multivector, Empty => Multivector }

impl_add! { scalar_add_multivector: Scalar, Multivector => Multivector }
impl_add! { scalar_add_scalar: Scalar, Scalar => Scalar }
impl_add! { scalar_add_vector: Scalar, Vector => Multivector }
impl_add! { scalar_add_bivector: Scalar, Bivector => Multivector }
impl_add! { scalar_add_trivector: Scalar, Trivector => Multivector }
impl_add! { scalar_add_pseudoscalar: Scalar, Pseudoscalar => Multivector }
impl_add! { return_lhs: Scalar, Empty => Scalar }

impl_add! { vector_add_multivector: Vector, Multivector => Multivector }
impl_add! { vector_add_scalar: Vector, Scalar => Multivector }
impl_add! { vector_add_vector: Vector, Vector => Vector }
impl_add! { vector_add_bivector: Vector, Bivector => Multivector }
impl_add! { vector_add_trivector: Vector, Trivector => Multivector }
impl_add! { vector_add_pseudoscalar: Vector, Pseudoscalar => Multivector }
impl_add! { return_lhs: Vector, Empty => Vector }

impl_add! { bivector_add_multivector: Bivector, Multivector => Multivector }
impl_add! { bivector_add_scalar: Bivector, Scalar => Multivector }
impl_add! { bivector_add_vector: Bivector, Vector => Multivector }
impl_add! { bivector_add_bivector: Bivector, Bivector => Bivector }
impl_add! { bivector_add_trivector: Bivector, Trivector => Multivector }
impl_add! { bivector_add_pseudoscalar: Bivector, Pseudoscalar => Multivector }
impl_add! { return_lhs: Bivector, Empty => Bivector }

impl_add! { trivector_add_multivector: Trivector, Multivector => Multivector }
impl_add! { trivector_add_scalar: Trivector, Scalar => Multivector }
impl_add! { trivector_add_vector: Trivector, Vector => Multivector }
impl_add! { trivector_add_bivector: Trivector, Bivector => Multivector }
impl_add! { trivector_add_trivector: Trivector, Trivector => Trivector }
impl_add! { trivector_add_pseudoscalar: Trivector, Pseudoscalar => Multivector }
impl_add! { return_lhs: Trivector, Empty => Trivector }

impl_add! { pseudoscalar_add_multivector: Pseudoscalar, Multivector => Multivector }
impl_add! { pseudoscalar_add_scalar: Pseudoscalar, Scalar => Multivector }
impl_add! { pseudoscalar_add_vector: Pseudoscalar, Vector => Multivector }
impl_add! { pseudoscalar_add_bivector: Pseudoscalar, Bivector => Multivector }
impl_add! { pseudoscalar_add_trivector: Pseudoscalar, Trivector => Multivector }
impl_add! { pseudoscalar_add_pseudoscalar: Pseudoscalar, Pseudoscalar => Pseudoscalar }
impl_add! { return_lhs: Pseudoscalar, Empty => Pseudoscalar }

impl_add! { return_rhs: Empty, Empty => Empty }
impl_add! { return_rhs: Empty, Scalar => Scalar }
impl_add! { return_rhs: Empty, Vector => Vector }
impl_add! { return_rhs: Empty, Bivector => Bivector }
impl_add! { return_rhs: Empty, Trivector => Trivector }
impl_add! { return_rhs: Empty, Pseudoscalar => Pseudoscalar }

// Multivector

#[inline]
fn multivector_add_multivector(
  mut lhs: Multivector,
  rhs: Multivector,
) -> Multivector {
  lhs.s += rhs.s;
  lhs.e0 += rhs.e0;
  lhs.e1 += rhs.e1;
  lhs.e2 += rhs.e2;
  lhs.e3 += rhs.e3;
  lhs.e01 += rhs.e01;
  lhs.e02 += rhs.e02;
  lhs.e03 += rhs.e03;
  lhs.e23 += rhs.e23;
  lhs.e31 += rhs.e31;
  lhs.e12 += rhs.e12;
  lhs.e032 += rhs.e032;
  lhs.e021 += rhs.e021;
  lhs.e013 += rhs.e013;
  lhs.e123 += rhs.e123;
  lhs.e0123 += rhs.e0123;

  lhs
}

#[inline]
fn multivector_add_scalar(mut lhs: Multivector, rhs: Scalar) -> Multivector {
  lhs.s += rhs.s;

  lhs
}

#[inline]
fn multivector_add_vector(mut lhs: Multivector, rhs: Vector) -> Multivector {
  lhs.e0 += rhs.e0;
  lhs.e1 += rhs.e1;
  lhs.e2 += rhs.e2;
  lhs.e3 += rhs.e3;

  lhs
}

#[inline]
fn multivector_add_bivector(
  mut lhs: Multivector,
  rhs: Bivector,
) -> Multivector {
  lhs.e01 += rhs.e01;
  lhs.e02 += rhs.e02;
  lhs.e03 += rhs.e03;
  lhs.e23 += rhs.e23;
  lhs.e31 += rhs.e31;
  lhs.e12 += rhs.e12;

  lhs
}

#[inline]
fn multivector_add_trivector(
  mut lhs: Multivector,
  rhs: Trivector,
) -> Multivector {
  lhs.e032 += rhs.e032;
  lhs.e021 += rhs.e021;
  lhs.e013 += rhs.e013;
  lhs.e123 += rhs.e123;

  lhs
}

#[inline]
fn multivector_add_pseudoscalar(
  mut lhs: Multivector,
  rhs: Pseudoscalar,
) -> Multivector {
  lhs.e0123 += rhs.e0123;

  lhs
}

// Scalar

#[inline]
fn scalar_add_multivector(lhs: Scalar, mut rhs: Multivector) -> Multivector {
  rhs.s += lhs.s;

  rhs
}

#[inline]
fn scalar_add_scalar(mut lhs: Scalar, rhs: Scalar) -> Scalar {
  lhs.s += rhs.s;

  lhs
}

#[inline]
fn scalar_add_vector(lhs: Scalar, rhs: Vector) -> Multivector {
  Multivector {
    s: lhs.s,
    e0: rhs.e0,
    e1: rhs.e1,
    e2: rhs.e2,
    e3: rhs.e3,
    ..zero()
  }
}

#[inline]
fn scalar_add_bivector(lhs: Scalar, rhs: Bivector) -> Multivector {
  Multivector {
    s: lhs.s,
    e01: rhs.e01,
    e02: rhs.e02,
    e03: rhs.e03,
    e23: rhs.e23,
    e31: rhs.e31,
    e12: rhs.e12,
    ..zero()
  }
}

#[inline]
fn scalar_add_trivector(lhs: Scalar, rhs: Trivector) -> Multivector {
  Multivector {
    s: lhs.s,
    e123: rhs.e123,
    e032: rhs.e032,
    e013: rhs.e013,
    e021: rhs.e021,
    ..zero()
  }
}

#[inline]
fn scalar_add_pseudoscalar(lhs: Scalar, rhs: Pseudoscalar) -> Multivector {
  Multivector {
    s: lhs.s,
    e0123: rhs.e0123,
    ..zero()
  }
}

// Vector

#[inline]
fn vector_add_multivector(lhs: Vector, mut rhs: Multivector) -> Multivector {
  rhs.e0 += lhs.e0;
  rhs.e1 += lhs.e1;
  rhs.e2 += lhs.e2;
  rhs.e3 += lhs.e3;

  rhs
}

#[inline]
fn vector_add_scalar(lhs: Vector, rhs: Scalar) -> Multivector {
  Multivector {
    s: rhs.s,
    e0: lhs.e0,
    e1: lhs.e1,
    e2: lhs.e2,
    e3: lhs.e3,
    ..zero()
  }
}

#[inline]
fn vector_add_vector(mut lhs: Vector, rhs: Vector) -> Vector {
  lhs.e0 += rhs.e0;
  lhs.e1 += rhs.e1;
  lhs.e2 += rhs.e2;
  lhs.e3 += rhs.e3;

  lhs
}

#[inline]
fn vector_add_bivector(lhs: Vector, rhs: Bivector) -> Multivector {
  Multivector {
    e0: lhs.e0,
    e1: lhs.e1,
    e2: lhs.e2,
    e3: lhs.e3,
    e01: rhs.e01,
    e02: rhs.e02,
    e03: rhs.e03,
    e23: rhs.e23,
    e31: rhs.e31,
    e12: rhs.e12,
    ..zero()
  }
}

#[inline]
fn vector_add_trivector(lhs: Vector, rhs: Trivector) -> Multivector {
  Multivector {
    e0: lhs.e0,
    e1: lhs.e1,
    e2: lhs.e2,
    e3: lhs.e3,
    e123: rhs.e123,
    e032: rhs.e032,
    e013: rhs.e013,
    e021: rhs.e021,
    ..zero()
  }
}

#[inline]
fn vector_add_pseudoscalar(lhs: Vector, rhs: Pseudoscalar) -> Multivector {
  Multivector {
    e0: lhs.e0,
    e1: lhs.e1,
    e2: lhs.e2,
    e3: lhs.e3,
    e0123: rhs.e0123,
    ..zero()
  }
}

// Bivector

#[inline]
fn bivector_add_multivector(
  lhs: Bivector,
  mut rhs: Multivector,
) -> Multivector {
  rhs.e01 += lhs.e01;
  rhs.e02 += lhs.e02;
  rhs.e03 += lhs.e03;
  rhs.e23 += lhs.e23;
  rhs.e31 += lhs.e31;
  rhs.e12 += lhs.e12;

  rhs
}

#[inline]
fn bivector_add_scalar(lhs: Bivector, rhs: Scalar) -> Multivector {
  Multivector {
    s: rhs.s,
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
fn bivector_add_vector(lhs: Bivector, rhs: Vector) -> Multivector {
  Multivector {
    e0: rhs.e0,
    e1: rhs.e1,
    e2: rhs.e2,
    e3: rhs.e3,
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
fn bivector_add_bivector(mut lhs: Bivector, rhs: Bivector) -> Bivector {
  lhs.e01 += rhs.e01;
  lhs.e02 += rhs.e02;
  lhs.e03 += rhs.e03;
  lhs.e23 += rhs.e23;
  lhs.e31 += rhs.e31;
  lhs.e12 += rhs.e12;

  lhs
}

#[inline]
fn bivector_add_trivector(lhs: Bivector, rhs: Trivector) -> Multivector {
  Multivector {
    e01: lhs.e01,
    e02: lhs.e02,
    e03: lhs.e03,
    e23: lhs.e23,
    e31: lhs.e31,
    e12: lhs.e12,
    e123: rhs.e123,
    e032: rhs.e032,
    e013: rhs.e013,
    e021: rhs.e021,
    ..zero()
  }
}

#[inline]
fn bivector_add_pseudoscalar(lhs: Bivector, rhs: Pseudoscalar) -> Multivector {
  Multivector {
    e01: lhs.e01,
    e02: lhs.e02,
    e03: lhs.e03,
    e23: lhs.e23,
    e31: lhs.e31,
    e12: lhs.e12,
    e0123: rhs.e0123,
    ..zero()
  }
}

// Trivector

#[inline]
fn trivector_add_multivector(
  lhs: Trivector,
  mut rhs: Multivector,
) -> Multivector {
  rhs.e123 += lhs.e123;
  rhs.e032 += lhs.e032;
  rhs.e013 += lhs.e013;
  rhs.e021 += lhs.e021;

  rhs
}

#[inline]
fn trivector_add_scalar(lhs: Trivector, rhs: Scalar) -> Multivector {
  Multivector {
    s: rhs.s,
    e123: lhs.e123,
    e032: lhs.e032,
    e013: lhs.e013,
    e021: lhs.e021,
    ..zero()
  }
}

#[inline]
fn trivector_add_vector(lhs: Trivector, rhs: Vector) -> Multivector {
  Multivector {
    e0: rhs.e0,
    e1: rhs.e1,
    e2: rhs.e2,
    e3: rhs.e3,
    e123: lhs.e123,
    e032: lhs.e032,
    e013: lhs.e013,
    e021: lhs.e021,
    ..zero()
  }
}

#[inline]
fn trivector_add_bivector(lhs: Trivector, rhs: Bivector) -> Multivector {
  Multivector {
    e01: rhs.e01,
    e02: rhs.e02,
    e03: rhs.e03,
    e23: rhs.e23,
    e31: rhs.e31,
    e12: rhs.e12,
    e123: lhs.e123,
    e032: lhs.e032,
    e013: lhs.e013,
    e021: lhs.e021,
    ..zero()
  }
}

#[inline]
fn trivector_add_trivector(mut lhs: Trivector, rhs: Trivector) -> Trivector {
  lhs.e123 += rhs.e123;
  lhs.e032 += rhs.e032;
  lhs.e013 += rhs.e013;
  lhs.e021 += rhs.e021;

  lhs
}

#[inline]
fn trivector_add_pseudoscalar(
  lhs: Trivector,
  rhs: Pseudoscalar,
) -> Multivector {
  Multivector {
    e123: lhs.e123,
    e032: lhs.e032,
    e013: lhs.e013,
    e021: lhs.e021,
    e0123: rhs.e0123,
    ..zero()
  }
}

// Pseudoscalar

#[inline]
fn pseudoscalar_add_multivector(
  lhs: Pseudoscalar,
  mut rhs: Multivector,
) -> Multivector {
  rhs.e0123 += lhs.e0123;
  rhs
}

#[inline]
fn pseudoscalar_add_scalar(lhs: Pseudoscalar, rhs: Scalar) -> Multivector {
  Multivector {
    s: rhs.s,
    e0123: lhs.e0123,
    ..zero()
  }
}

#[inline]
fn pseudoscalar_add_vector(lhs: Pseudoscalar, rhs: Vector) -> Multivector {
  Multivector {
    e0: rhs.e0,
    e1: rhs.e1,
    e2: rhs.e2,
    e3: rhs.e3,
    e0123: lhs.e0123,
    ..zero()
  }
}

#[inline]
fn pseudoscalar_add_bivector(lhs: Pseudoscalar, rhs: Bivector) -> Multivector {
  Multivector {
    e01: rhs.e01,
    e02: rhs.e02,
    e03: rhs.e03,
    e23: rhs.e23,
    e31: rhs.e31,
    e12: rhs.e12,
    e0123: lhs.e0123,
    ..zero()
  }
}

#[inline]
fn pseudoscalar_add_trivector(
  lhs: Pseudoscalar,
  rhs: Trivector,
) -> Multivector {
  Multivector {
    e123: rhs.e123,
    e032: rhs.e032,
    e013: rhs.e013,
    e021: rhs.e021,
    e0123: lhs.e0123,
    ..zero()
  }
}

#[inline]
fn pseudoscalar_add_pseudoscalar(
  mut lhs: Pseudoscalar,
  rhs: Pseudoscalar,
) -> Pseudoscalar {
  lhs.e0123 += rhs.e0123;

  lhs
}
