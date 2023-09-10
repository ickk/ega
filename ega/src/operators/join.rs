use super::return_empty;
use crate::*;

/// The regressive product
pub trait Join<Rhs> {
  type Output;

  /// The regressive product
  fn join(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_join {
  ($join_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl Join<$rhs> for $lhs {
      type Output = $output;
      #[inline]
      fn join(self, rhs: $rhs) -> Self::Output {
        $join_fn(self, rhs)
      }
    }
  };
}

impl_join! { multivector_join_multivector: Multivector, Multivector => Multivector }
impl_join! { multivector_join_scalar: Multivector, Scalar => Scalar }
impl_join! { multivector_join_vector: Multivector, Vector => Multivector }
impl_join! { multivector_join_bivector: Multivector, Bivector => Multivector }
impl_join! { multivector_join_trivector: Multivector, Trivector => Multivector }
impl_join! { multivector_join_pseudoscalar: Multivector, Pseudoscalar => Multivector }

impl_join! { scalar_join_multivector: Scalar, Multivector => Scalar }
impl_join! { return_empty: Scalar, Scalar => Empty }
impl_join! { return_empty: Scalar, Vector => Empty }
impl_join! { return_empty: Scalar, Bivector => Empty }
impl_join! { return_empty: Scalar, Trivector => Empty }
impl_join! { scalar_join_pseudoscalar: Scalar, Pseudoscalar => Scalar }

impl_join! { vector_join_multivector: Vector, Multivector => Multivector }
impl_join! { return_empty: Vector, Scalar => Empty }
impl_join! { return_empty: Vector, Vector => Empty }
impl_join! { return_empty: Vector, Bivector => Empty }
impl_join! { vector_join_trivector: Vector, Trivector => Scalar }
impl_join! { vector_join_pseudoscalar: Vector, Pseudoscalar => Vector }

impl_join! { bivector_join_multivector: Bivector, Multivector => Multivector }
impl_join! { return_empty: Bivector, Scalar => Empty }
impl_join! { return_empty: Bivector, Vector => Empty }
impl_join! { bivector_join_bivector: Bivector, Bivector => Scalar }
impl_join! { bivector_join_trivector: Bivector, Trivector => Vector }
impl_join! { bivector_join_pseudoscalar: Bivector, Pseudoscalar => Bivector }

impl_join! { trivector_join_multivector: Trivector, Multivector => Multivector }
impl_join! { return_empty: Trivector, Scalar => Empty }
impl_join! { trivector_join_vector: Trivector, Vector => Scalar }
impl_join! { trivector_join_bivector: Trivector, Bivector => Vector }
impl_join! { trivector_join_trivector: Trivector, Trivector => Bivector }
impl_join! { trivector_join_pseudoscalar: Trivector, Pseudoscalar => Trivector }

impl_join! { pseudoscalar_join_multivector: Pseudoscalar, Multivector => Multivector }
impl_join! { pseudoscalar_join_scalar: Pseudoscalar, Scalar => Scalar }
impl_join! { pseudoscalar_join_vector: Pseudoscalar, Vector => Vector }
impl_join! { pseudoscalar_join_bivector: Pseudoscalar, Bivector => Bivector }
impl_join! { pseudoscalar_join_trivector: Pseudoscalar, Trivector => Trivector }
impl_join! { pseudoscalar_join_pseudoscalar: Pseudoscalar, Pseudoscalar => Pseudoscalar }

impl_join! { return_empty: Empty, Empty => Empty }
impl_join! { return_empty: Multivector, Empty => Empty }
impl_join! { return_empty: Scalar, Empty => Empty }
impl_join! { return_empty: Vector, Empty => Empty }
impl_join! { return_empty: Bivector, Empty => Empty }
impl_join! { return_empty: Trivector, Empty => Empty }
impl_join! { return_empty: Pseudoscalar, Empty => Empty }
impl_join! { return_empty: Empty, Multivector => Empty }
impl_join! { return_empty: Empty, Scalar => Empty }
impl_join! { return_empty: Empty, Vector => Empty }
impl_join! { return_empty: Empty, Bivector => Empty }
impl_join! { return_empty: Empty, Trivector => Empty }
impl_join! { return_empty: Empty, Pseudoscalar => Empty }

// Multivector

#[rustfmt::skip]
#[inline]
fn multivector_join_multivector(
  lhs: Multivector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e0123*b.s + a.s*b.e0123
        + a.e123*b.e0 - a.e0*b.e123
        + a.e032*b.e1 - a.e1*b.e032
        + a.e013*b.e2 - a.e2*b.e013
        + a.e021*b.e3 - a.e3*b.e021
        + a.e01*b.e23 + a.e23*b.e01
        + a.e02*b.e31 + a.e31*b.e02
        + a.e03*b.e12 + a.e12*b.e03;
  let e0 = a.e0*b.e0123 + a.e0123*b.e0
         - a.e032*b.e01 - a.e01*b.e032
         - a.e013*b.e02 - a.e02*b.e013
         - a.e021*b.e03 - a.e03*b.e021;
  let e1 = a.e0123*b.e1 + a.e1*b.e0123
         + a.e021*b.e31 + a.e31*b.e021
         + a.e123*b.e01 + a.e01*b.e123
         - a.e013*b.e12 - a.e12*b.e013;
  let e2 = a.e0123*b.e2 + a.e2*b.e0123
         + a.e123*b.e02 + a.e02*b.e123
         + a.e032*b.e12 + a.e12*b.e032
         - a.e021*b.e23 - a.e23*b.e021;
  let e3 = a.e0123*b.e3 + a.e3*b.e0123
         + a.e123*b.e03 + a.e03*b.e123
         - a.e032*b.e31 - a.e31*b.e032
         + a.e013*b.e23 + a.e23*b.e013;
  let e01 = a.e0123*b.e01 + a.e01*b.e0123
          - a.e013*b.e021 + a.e021*b.e013;
  let e02 = a.e0123*b.e02 + a.e02*b.e0123
          + a.e032*b.e021 - a.e021*b.e032;
  let e03 = a.e0123*b.e03 + a.e03*b.e0123
          - a.e032*b.e013 + a.e013*b.e032;
  let e12 = a.e0123*b.e12 + a.e12*b.e0123
          - a.e123*b.e021 + a.e021*b.e123;
  let e31 = a.e0123*b.e31 + a.e31*b.e0123
          - a.e123*b.e013 + a.e013*b.e123;
  let e23 = a.e0123*b.e23 + a.e23*b.e0123
          - a.e123*b.e032 + a.e032*b.e123;
  let e021 = a.e0123*b.e021 + a.e021*b.e0123;
  let e013 = a.e0123*b.e013 + a.e013*b.e0123;
  let e032 = a.e0123*b.e032 + a.e032*b.e0123;
  let e123 = a.e0123*b.e123 + a.e123*b.e0123;
  let e0123 = a.e0123*b.e0123;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_join_scalar(
  lhs: Multivector,
  rhs: Scalar,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.e0123*b.s;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn multivector_join_vector(
  lhs: Multivector,
  rhs: Vector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e123*b.e0
        + a.e032*b.e1
        + a.e013*b.e2
        + a.e021*b.e3;
  let e0 = a.e0123*b.e0;
  let e1 = a.e0123*b.e1;
  let e2 = a.e0123*b.e2;
  let e3 = a.e0123*b.e3;

  Multivector {
    s,
    e0,    e1,    e2,    e3,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_join_bivector(
  lhs: Multivector,
  rhs: Bivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e01*b.e23 + a.e23*b.e01
        + a.e02*b.e31 + a.e31*b.e02
        + a.e03*b.e12 + a.e12*b.e03;
  let e0 = -a.e032*b.e01
         - a.e013*b.e02
         - a.e021*b.e03;
  let e1 = a.e021*b.e31
         + a.e123*b.e01
         - a.e013*b.e12;
  let e2 = a.e123*b.e02
         + a.e032*b.e12
         - a.e021*b.e23;
  let e3 = a.e123*b.e03
         - a.e032*b.e31
         + a.e013*b.e23;
  let e01 = a.e0123*b.e01;
  let e02 = a.e0123*b.e02;
  let e03 = a.e0123*b.e03;
  let e12 = a.e0123*b.e12;
  let e31 = a.e0123*b.e31;
  let e23 = a.e0123*b.e23;

  Multivector {
     e0,    e1,    e2,    e3,
      s,   e23,   e31,   e12,
    e01,   e02,   e03,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_join_trivector(
  lhs: Multivector,
  rhs: Trivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = -a.e0*b.e123
        - a.e1*b.e032
        - a.e2*b.e013
        - a.e3*b.e021;
  let e0 = -a.e01*b.e032
         - a.e02*b.e013
         - a.e03*b.e021;
  let e1 = a.e31*b.e021
         + a.e01*b.e123
         - a.e12*b.e013;
  let e2 = a.e02*b.e123
         + a.e12*b.e032
         - a.e23*b.e021;
  let e3 = a.e03*b.e123
         - a.e31*b.e032
         + a.e23*b.e013;
  let e01 = -a.e013*b.e021 + a.e021*b.e013;
  let e02 = a.e032*b.e021 - a.e021*b.e032;
  let e03 = -a.e032*b.e013 + a.e013*b.e032;
  let e12 = -a.e123*b.e021 + a.e021*b.e123;
  let e31 = -a.e123*b.e013 + a.e013*b.e123;
  let e23 = -a.e123*b.e032 + a.e032*b.e123;
  let e021 = a.e0123*b.e021;
  let e013 = a.e0123*b.e013;
  let e032 = a.e0123*b.e032;
  let e123 = a.e0123*b.e123;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03,
    e123,  e032,  e013,  e021,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_join_pseudoscalar(
  lhs: Multivector,
  rhs: Pseudoscalar,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.s*b.e0123;
  let e0 = a.e0*b.e0123;
  let e1 = a.e1*b.e0123;
  let e2 = a.e2*b.e0123;
  let e3 = a.e3*b.e0123;
  let e01 = a.e01*b.e0123;
  let e02 = a.e02*b.e0123;
  let e03 = a.e03*b.e0123;
  let e12 = a.e12*b.e0123;
  let e31 = a.e31*b.e0123;
  let e23 = a.e23*b.e0123;
  let e021 = a.e021*b.e0123;
  let e013 = a.e013*b.e0123;
  let e032 = a.e032*b.e0123;
  let e123 = a.e123*b.e0123;
  let e0123 = a.e0123*b.e0123;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

// Scalar

#[rustfmt::skip]
#[inline]
fn scalar_join_multivector(
  lhs: Scalar,
  rhs: Multivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.s*b.e0123;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_join_pseudoscalar(
  lhs: Scalar,
  rhs: Pseudoscalar,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.s*b.e0123;

  Scalar { s }
}

// Vector

#[rustfmt::skip]
#[inline]
fn vector_join_multivector(
  lhs: Vector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = -a.e0*b.e123
        - a.e1*b.e032
        - a.e2*b.e013
        - a.e3*b.e021;
  let e0 = a.e0*b.e0123;
  let e1 = a.e1*b.e0123;
  let e2 = a.e2*b.e0123;
  let e3 = a.e3*b.e0123;

  Multivector {
    e0,    e1,    e2,    e3,
    s,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_join_trivector(
  lhs: Vector,
  rhs: Trivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = -a.e0*b.e123
        - a.e1*b.e032
        - a.e2*b.e013
        - a.e3*b.e021;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn vector_join_pseudoscalar(
  lhs: Vector,
  rhs: Pseudoscalar,
) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e0*b.e0123;
  let e1 = a.e1*b.e0123;
  let e2 = a.e2*b.e0123;
  let e3 = a.e3*b.e0123;

  Vector { e0, e1, e2, e3 }
}

// Bivector

#[rustfmt::skip]
#[inline]
fn bivector_join_multivector(
  lhs: Bivector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e01*b.e23 + a.e23*b.e01
        + a.e02*b.e31 + a.e31*b.e02
        + a.e03*b.e12 + a.e12*b.e03;
  let e0 = -a.e01*b.e032
         - a.e02*b.e013
         - a.e03*b.e021;
  let e1 = a.e31*b.e021
         + a.e01*b.e123
         - a.e12*b.e013;
  let e2 = a.e02*b.e123
         + a.e12*b.e032
         - a.e23*b.e021;
  let e3 = a.e03*b.e123
         - a.e31*b.e032
         + a.e23*b.e013;
  let e01 = a.e01*b.e0123;
  let e02 = a.e02*b.e0123;
  let e03 = a.e03*b.e0123;
  let e12 = a.e12*b.e0123;
  let e31 = a.e31*b.e0123;
  let e23 = a.e23*b.e0123;

  Multivector {
     e0,    e1,    e2,    e3,
      s,   e23,   e31,   e12,
    e01,   e02,   e03,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_join_bivector(
  lhs: Bivector,
  rhs: Bivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.e01*b.e23 + a.e23*b.e01
        + a.e02*b.e31 + a.e31*b.e02
        + a.e03*b.e12 + a.e12*b.e03;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn bivector_join_trivector(
  lhs: Bivector,
  rhs: Trivector,
) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = -a.e01*b.e032
         - a.e02*b.e013
         - a.e03*b.e021;
  let e1 = a.e31*b.e021
         + a.e01*b.e123
         - a.e12*b.e013;
  let e2 = a.e02*b.e123
         + a.e12*b.e032
         - a.e23*b.e021;
  let e3 = a.e03*b.e123
         - a.e31*b.e032
         + a.e23*b.e013;

  Vector { e0, e1, e2, e3 }
}

#[rustfmt::skip]
#[inline]
fn bivector_join_pseudoscalar(
  lhs: Bivector,
  rhs: Pseudoscalar,
) -> Bivector {
  let (a, b) = (lhs, rhs);

  let e01 = a.e01*b.e0123;
  let e02 = a.e02*b.e0123;
  let e03 = a.e03*b.e0123;
  let e12 = a.e12*b.e0123;
  let e31 = a.e31*b.e0123;
  let e23 = a.e23*b.e0123;

  Bivector {
    e23,   e31,   e12,
    e01,   e02,   e03,
  }
}

// Trivector

#[rustfmt::skip]
#[inline]
fn trivector_join_multivector(
  lhs: Trivector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e123*b.e0
        + a.e032*b.e1
        + a.e013*b.e2
        + a.e021*b.e3;
  let e0 = -a.e032*b.e01
         - a.e013*b.e02
         - a.e021*b.e03;
  let e1 = a.e021*b.e31
         + a.e123*b.e01
         - a.e013*b.e12;
  let e2 = a.e123*b.e02
         + a.e032*b.e12
         - a.e021*b.e23;
  let e3 = a.e123*b.e03
         - a.e032*b.e31
         + a.e013*b.e23;
  let e01 = -a.e013*b.e021 + a.e021*b.e013;
  let e02 = a.e032*b.e021 - a.e021*b.e032;
  let e03 = -a.e032*b.e013 + a.e013*b.e032;
  let e12 = -a.e123*b.e021 + a.e021*b.e123;
  let e31 = -a.e123*b.e013 + a.e013*b.e123;
  let e23 = -a.e123*b.e032 + a.e032*b.e123;
  let e021 = a.e021*b.e0123;
  let e013 = a.e013*b.e0123;
  let e032 = a.e032*b.e0123;
  let e123 = a.e123*b.e0123;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03,
    e123,  e032,  e013,  e021,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_join_vector(
  lhs: Trivector,
  rhs: Vector,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.e123*b.e0
        + a.e032*b.e1
        + a.e013*b.e2
        + a.e021*b.e3;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn trivector_join_bivector(
  lhs: Trivector,
  rhs: Bivector,
) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = -a.e032*b.e01
         - a.e013*b.e02
         - a.e021*b.e03;
  let e1 = a.e021*b.e31
         + a.e123*b.e01
         - a.e013*b.e12;
  let e2 = a.e123*b.e02
         + a.e032*b.e12
         - a.e021*b.e23;
  let e3 = a.e123*b.e03
         - a.e032*b.e31
         + a.e013*b.e23;

  Vector { e0, e1, e2, e3 }
}

#[rustfmt::skip]
#[inline]
fn trivector_join_trivector(
  lhs: Trivector,
  rhs: Trivector,
) -> Bivector {
  let (a, b) = (lhs, rhs);

  let e01 = -a.e013*b.e021 + a.e021*b.e013;
  let e02 = a.e032*b.e021 - a.e021*b.e032;
  let e03 = -a.e032*b.e013 + a.e013*b.e032;
  let e12 = -a.e123*b.e021 + a.e021*b.e123;
  let e31 = -a.e123*b.e013 + a.e013*b.e123;
  let e23 = -a.e123*b.e032 + a.e032*b.e123;

  Bivector {
    e23,   e31,   e12,
    e01,   e02,   e03,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_join_pseudoscalar(
  lhs: Trivector,
  rhs: Pseudoscalar,
) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e021 = a.e021*b.e0123;
  let e013 = a.e013*b.e0123;
  let e032 = a.e032*b.e0123;
  let e123 = a.e123*b.e0123;

  Trivector { e123,  e032,  e013,  e021 }
}

// Pseudoscalar

#[rustfmt::skip]
#[inline]
fn pseudoscalar_join_multivector(
  lhs: Pseudoscalar,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e0123*b.s;
  let e0 = a.e0123*b.e0;
  let e1 = a.e0123*b.e1;
  let e2 = a.e0123*b.e2;
  let e3 = a.e0123*b.e3;
  let e01 = a.e0123*b.e01;
  let e02 = a.e0123*b.e02;
  let e03 = a.e0123*b.e03;
  let e12 = a.e0123*b.e12;
  let e31 = a.e0123*b.e31;
  let e23 = a.e0123*b.e23;
  let e021 = a.e0123*b.e021;
  let e013 = a.e0123*b.e013;
  let e032 = a.e0123*b.e032;
  let e123 = a.e0123*b.e123;
  let e0123 = a.e0123*b.e0123;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_join_scalar(
  lhs: Pseudoscalar,
  rhs: Scalar,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.e0123*b.s;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_join_vector(
  lhs: Pseudoscalar,
  rhs: Vector,
) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e0123*b.e0;
  let e1 = a.e0123*b.e1;
  let e2 = a.e0123*b.e2;
  let e3 = a.e0123*b.e3;

  Vector { e0, e1, e2, e3 }
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_join_bivector(
  lhs: Pseudoscalar,
  rhs: Bivector,
) -> Bivector {
  let (a, b) = (lhs, rhs);

  let e01 = a.e0123*b.e01;
  let e02 = a.e0123*b.e02;
  let e03 = a.e0123*b.e03;
  let e12 = a.e0123*b.e12;
  let e31 = a.e0123*b.e31;
  let e23 = a.e0123*b.e23;

  Bivector {
    e23,   e31,   e12,
    e01,   e02,   e03,
  }
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_join_trivector(
  lhs: Pseudoscalar,
  rhs: Trivector,
) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e021 = a.e0123*b.e021;
  let e013 = a.e0123*b.e013;
  let e032 = a.e0123*b.e032;
  let e123 = a.e0123*b.e123;

  Trivector { e123, e032, e013, e021 }
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_join_pseudoscalar(
  lhs: Pseudoscalar,
  rhs: Pseudoscalar,
) -> Pseudoscalar {
  let (a, b) = (lhs, rhs);

  let e0123 = a.e0123*b.e0123;

  Pseudoscalar { e0123 }
}

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod tests {
  use super::*;

  const MULTIVECTOR_A: Multivector = Multivector {
      e0:    2.,   e1:    3.,   e2:    5.,    e3:    7.,
       s:   11.,  e23:   13.,  e31:   17.,   e12:   19.,
     e01:   23.,  e02:   29.,  e03:   31., e0123:   37.,
    e123:   41., e032:   43., e013:   47.,  e021:   53.,
  };
  const MULTIVECTOR_B: Multivector = Multivector {
      e0:   59.,   e1:   61.,   e2:   67.,    e3:   71.,
       s:   73.,  e23:   79.,  e31:   83.,   e12:   89.,
     e01:   97.,  e02:  101.,  e03:  103., e0123:  107.,
    e123:  109., e032:  113., e013:  127.,  e021:  131.,
  };
  const MULTIVECTOR_C: Multivector = Multivector {
      e0:  -59.,   e1:   61.,   e2:  -67.,    e3:   71.,
       s:  -73.,  e23:   79.,  e31:  -83.,   e12:   89.,
     e01:  -97.,  e02:  101.,  e03: -103., e0123:  107.,
    e123: -109., e032:  113., e013: -127.,  e021:  131.,
  };
  const MULTIVECTOR_D: Multivector = Multivector {
      e0:  -59.,   e1:  -61.,   e2:  -67.,    e3:  -71.,
       s:  -73.,  e23:  -79.,  e31:  -83.,   e12:  -89.,
     e01:  -97.,  e02: -101.,  e03: -103., e0123: -107.,
    e123: -109., e032: -113., e013: -127.,  e021: -131.,
  };
  const SCALAR_A: Scalar = Scalar { s:  137. };
  const SCALAR_B: Scalar = Scalar { s:  139. };
  const SCALAR_C: Scalar = Scalar { s: -149. };
  const VECTOR_A: Vector = Vector {
    e0:  151., e1:  157., e2:  163., e3:  167.
  };
  const VECTOR_B: Vector = Vector {
    e0:  173., e1:  179., e2:  181., e3:  191.
  };
  const VECTOR_C: Vector = Vector {
    e0:  -193., e1: -197., e2:  -199., e3: -211.
  };
  const BIVECTOR_A: Bivector = Bivector {
    e23:  223., e31:  227., e12:  229.,
    e01:  233., e02:  239., e03:  241.,
  };
  const BIVECTOR_C: Bivector = Bivector {
    e23: -281., e31: -283., e12: -293.,
    e01: -307., e02: -311., e03: -313.,
  };
  const TRIVECTOR_A: Trivector = Trivector {
    e123:  317., e032:  331., e013:  337., e021:  347.
  };
  const TRIVECTOR_C: Trivector = Trivector {
    e123: -373., e032: -379., e013: -383., e021: -389.
  };
  const PSEUDOSCALAR_A: Pseudoscalar = Pseudoscalar { e0123:  397. };
  const PSEUDOSCALAR_C: Pseudoscalar = Pseudoscalar { e0123: -409. };

  mod multivector {
    use super::*;
    #[test]
    fn join_multivector_1() {
      let result = MULTIVECTOR_A.join(MULTIVECTOR_B);
      let expected = Multivector {
          e0: -22323.,   e1:  9092.,   e2: 10400.,    e3: 10852.,
           s:  25641.,  e23:  4368.,  e31:  4806.,   e12:  5732.,
         e01:   6624.,  e02:  6484.,  e03:  6978., e0123:  3959.,
        e123:   8420., e032:  8782., e013:  9728.,  e021: 10518.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_multivector_2() {
      let result = MULTIVECTOR_A.join(MULTIVECTOR_C);
      let expected = Multivector {
          e0:    -63.,   e1: -7848.,   e2:  -880.,    e3:  -516.,
           s:   -441.,  e23: -5006.,  e31: -1168.,   e12: -5822.,
         e01: -14016.,  e02:  6484.,  e03: 10278., e0123:  3959.,
        e123:    354., e032:  8782., e013:   330.,  e021: 10518.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_multivector_3() {
      let result = MULTIVECTOR_A.join(MULTIVECTOR_D);
      let expected = Multivector {
          e0:  22323.,   e1: -9092.,   e2: -10400.,    e3: -10852.,
           s: -25641.,  e23: -4368.,  e31:  -4806.,   e12:  -5732.,
         e01:  -6624.,  e02: -6484.,  e03:  -6978., e0123:  -3959.,
        e123:  -8420., e032: -8782., e013:  -9728.,  e021: -10518.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_scalar_1() {
      let result = MULTIVECTOR_A.join(SCALAR_A);
      let expected = Scalar { s: 5069. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_scalar_2() {
      let result = MULTIVECTOR_A.join(SCALAR_C);
      let expected = Scalar { s: -5513. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_vector_1() {
      let result = MULTIVECTOR_A.join(VECTOR_A);
      let expected = Multivector {
        s: 29454.,
        e0: 5587., e1: 5809., e2: 6031., e3: 6179.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_vector_2() {
      let result = MULTIVECTOR_A.join(VECTOR_C);
      let expected = Multivector {
        s: -36920.,
        e0: -7141., e1: -7289., e2: -7363., e3: -7807.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_bivector_1() {
      let result = MULTIVECTOR_A.join(BIVECTOR_A);
      let expected = Multivector {
        s: 30482.,
        e0: -34025., e1: 10821., e2: 7827., e3: 10601.,
        e01: 8621., e02: 8843., e03: 8917.,
        e12: 8473., e31: 8399., e23: 8251.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_bivector_2() {
      let result = MULTIVECTOR_A.join(BIVECTOR_C);
      let expected = Multivector {
        s: -38978.,
        e0: 44407., e1: -13815., e2: -10457., e3: -13871.,
        e01: -11359., e02: -11507., e03: -11581.,
        e12: -10841., e31: -10471., e23: -10397.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_trivector_1() {
      let result = MULTIVECTOR_A.join(TRIVECTOR_A);
      let expected = Multivector {
        s: -5741.,
        e0: -28143., e1: 6787., e2: 10971., e3: 8581.,
        e01: 1552., e02: -2622., e03: 1066.,
        e12: 2574., e31:  1082., e23:   60.,
        e021: 12839., e013: 12469., e032: 12247., e123: 11729.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_trivector_2() {
      let result = MULTIVECTOR_A.join(TRIVECTOR_C);
      let expected = Multivector {
        s: 6521.,
        e0: 31883., e1: -7915., e2: -12961., e3: -10099.,
        e01: -2016., e02:  3360., e03: -1344.,
        e12: -3820., e31: -1828., e23:  -500.,
        e021: -14393., e013: -14171., e032: -14023., e123: -13801.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_pseudoscalar_1() {
      let result = MULTIVECTOR_A.join(PSEUDOSCALAR_A);
      let expected = Multivector {
        s: 4367.,
        e0: 794., e1: 1191., e2: 1985., e3: 2779.,
        e01: 9131., e02: 11513., e03: 12307.,
        e12: 7543., e31:  6749., e23: 5161.,
        e021: 21041., e013: 18659., e032: 17071., e123: 16277.,
        e0123: 14689.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_pseudoscalar_2() {
      let result = MULTIVECTOR_A.join(PSEUDOSCALAR_C);
      let expected = Multivector {
        s: -4499.,
        e0: -818., e1: -1227., e2: -2045., e3: -2863.,
        e01: -9407., e02: -11861., e03: -12679.,
        e12: -7771., e31:  -6953., e23: -5317.,
        e021: -21677., e013: -19223., e032: -17587., e123: -16769.,
        e0123: -15133.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod scalar {
    use super::*;
    #[test]
    fn join_multivector_1() {
      let result = SCALAR_A.join(MULTIVECTOR_A);
      let expected = Scalar { s: 5069. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_scalar_1() {
      let result = SCALAR_A.join(SCALAR_B);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_vector_1() {
      let result = SCALAR_A.join(VECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_bivector_1() {
      let result = SCALAR_A.join(BIVECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_trivector_1() {
      let result = SCALAR_A.join(TRIVECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_pseudoscalar_1() {
      let result = SCALAR_A.join(PSEUDOSCALAR_A);
      let expected = Scalar { s: 54389. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod vector {
    use super::*;
    #[test]
    fn join_multivector_1() {
      let result = VECTOR_A.join(MULTIVECTOR_A);
      let expected = Multivector {
        s: -29454.,
        e0: 5587., e1: 5809., e2: 6031., e3: 6179.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_scalar_1() {
      let result = VECTOR_A.join(SCALAR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_vector_1() {
      let result = VECTOR_A.join(VECTOR_B);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_bivector_1() {
      let result = VECTOR_A.join(BIVECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_trivector_1() {
      let result = VECTOR_A.join(TRIVECTOR_A);
      let expected = Scalar { s: -212714. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_pseudoscalar_1() {
      let result = VECTOR_A.join(PSEUDOSCALAR_A);
      let expected = Vector {
        e0: 59947., e1: 62329., e2: 64711., e3: 66299.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod bivector {
    use super::*;
    #[test]
    fn join_multivector_1() {
      let result = BIVECTOR_A.join(MULTIVECTOR_A);
      let expected = Multivector {
        s: 30482.,
        e0: -34025., e1: 10821., e2: 7827., e3: 10601.,
        e01: 8621., e02: 8843., e03: 8917., e12: 8473., e31: 8399., e23: 8251.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_scalar_1() {
      let result = BIVECTOR_A.join(SCALAR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_vector_1() {
      let result = BIVECTOR_A.join(VECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_bivector_1() {
      let result = BIVECTOR_A.join(BIVECTOR_C);
      let expected = Scalar { s: -414458. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_trivector_1() {
      let result = BIVECTOR_A.join(TRIVECTOR_A);
      let expected = Vector {
        e0: -241293., e1: 75457., e2: 74181., e3: 76411.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_pseudoscalar_1() {
      let result = BIVECTOR_A.join(PSEUDOSCALAR_A);
      let expected = Bivector {
        e01: 92501., e02: 94883., e03: 95677.,
        e12: 90913., e31: 90119., e23: 88531.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod trivector {
    use super::*;
    #[test]
    fn join_multivector_1() {
      let result = TRIVECTOR_A.join(MULTIVECTOR_A);
      let expected = Multivector {
        s: 5741.,
        e0: -28143., e1: 6787., e2: 10971., e3: 8581.,
        e01: -1552., e02: 2622., e03: -1066.,
        e12: -2574., e31: -1082., e23: -60.,
        e021: 12839., e013: 12469., e032: 12247., e123: 11729.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_scalar_1() {
      let result = TRIVECTOR_A.join(SCALAR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_vector_1() {
      let result = TRIVECTOR_A.join(VECTOR_A);
      let expected = Scalar { s: 212714. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_bivector_1() {
      let result = TRIVECTOR_A.join(BIVECTOR_A);
      let expected = Vector {
        e0: -241293., e1: 75457., e2: 74181., e3: 76411.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_trivector_1() {
      let result = TRIVECTOR_A.join(TRIVECTOR_C);
      let expected = Bivector {
        e01: -1808., e02: 2754., e03: -950.,
        e12: -6118., e31: -4290., e23: -3320.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_pseudoscalar_1() {
      let result = TRIVECTOR_A.join(PSEUDOSCALAR_A);
      let expected = Trivector {
        e021: 137759., e013: 133789., e032: 131407., e123: 125849.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod pseudoscalar {
    use super::*;
    #[test]
    fn join_multivector_1() {
      let result = PSEUDOSCALAR_A.join(MULTIVECTOR_A);
      let expected = Multivector {
        s: 4367.,
        e0: 794., e1: 1191., e2: 1985., e3: 2779.,
        e01: 9131., e02: 11513., e03: 12307.,
        e12: 7543., e31: 6749., e23: 5161.,
        e021: 21041., e013: 18659., e032: 17071., e123: 16277.,
        e0123: 14689.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_scalar_1() {
      let result = PSEUDOSCALAR_A.join(SCALAR_A);
      let expected = Scalar { s: 54389. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_vector_1() {
      let result = PSEUDOSCALAR_A.join(VECTOR_A);
      let expected = Vector {
        e0: 59947., e1: 62329., e2: 64711., e3: 66299.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_vector_2() {
      let result = PSEUDOSCALAR_A.join(BIVECTOR_A);
      let expected = Bivector {
        e01: 92501., e02: 94883., e03: 95677.,
        e12: 90913., e31: 90119., e23: 88531.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_vector_3() {
      let result = PSEUDOSCALAR_A.join(TRIVECTOR_A);
      let expected = Trivector {
        e021: 137759., e013: 133789., e032: 131407., e123: 125849.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn join_bivector_1() {
      let result = PSEUDOSCALAR_A.join(PSEUDOSCALAR_C);
      let expected = Pseudoscalar { e0123: -162373. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }
}
