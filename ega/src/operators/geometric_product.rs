use super::return_empty;
use crate::*;

/// The geometric product
pub trait GeometricProduct<Rhs> {
  type Output;

  /// The geometric product
  fn geometric_product(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_geometric_product {
  ($mul_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl GeometricProduct<$rhs> for $lhs {
      type Output = $output;

      #[inline]
      fn geometric_product(self, rhs: $rhs) -> Self::Output {
        $mul_fn(self, rhs)
      }
    }
  };
}

impl_geometric_product! { multivector_mul_multivector: Multivector, Multivector => Multivector }
impl_geometric_product! { multivector_mul_scalar: Multivector, Scalar => Multivector }
impl_geometric_product! { multivector_mul_vector: Multivector, Vector => Multivector }
impl_geometric_product! { multivector_mul_bivector: Multivector, Bivector => Multivector }
impl_geometric_product! { multivector_mul_trivector: Multivector, Trivector => Multivector }
impl_geometric_product! { multivector_mul_pseudoscalar: Multivector, Pseudoscalar => Multivector }

impl_geometric_product! { scalar_mul_multivector: Scalar, Multivector => Multivector }
impl_geometric_product! { scalar_mul_scalar: Scalar, Scalar => Scalar }
impl_geometric_product! { scalar_mul_vector: Scalar, Vector => Vector }
impl_geometric_product! { scalar_mul_bivector: Scalar, Bivector => Bivector }
impl_geometric_product! { scalar_mul_trivector: Scalar, Trivector => Trivector }
impl_geometric_product! { scalar_mul_pseudoscalar: Scalar, Pseudoscalar => Pseudoscalar }

impl_geometric_product! { vector_mul_multivector: Vector, Multivector => Multivector }
impl_geometric_product! { vector_mul_scalar: Vector, Scalar => Vector }
impl_geometric_product! { vector_mul_vector: Vector, Vector => Multivector }
impl_geometric_product! { vector_mul_bivector: Vector, Bivector => Multivector }
impl_geometric_product! { vector_mul_trivector: Vector, Trivector => Multivector }
impl_geometric_product! { vector_mul_pseudoscalar: Vector, Pseudoscalar => Trivector }

impl_geometric_product! { bivector_mul_multivector: Bivector, Multivector => Multivector }
impl_geometric_product! { bivector_mul_scalar: Bivector, Scalar => Bivector }
impl_geometric_product! { bivector_mul_vector: Bivector, Vector => Multivector }
impl_geometric_product! { bivector_mul_bivector: Bivector, Bivector => Multivector }
impl_geometric_product! { bivector_mul_trivector: Bivector, Trivector => Multivector }
impl_geometric_product! { bivector_mul_pseudoscalar: Bivector, Pseudoscalar => Bivector }

impl_geometric_product! { trivector_mul_multivector: Trivector, Multivector => Multivector }
impl_geometric_product! { trivector_mul_scalar: Trivector, Scalar => Trivector }
impl_geometric_product! { trivector_mul_vector: Trivector, Vector => Multivector }
impl_geometric_product! { trivector_mul_bivector: Trivector, Bivector => Multivector }
impl_geometric_product! { trivector_mul_trivector: Trivector, Trivector => Multivector }
impl_geometric_product! { trivector_mul_pseudoscalar: Trivector, Pseudoscalar => Vector }

impl_geometric_product! { pseudoscalar_mul_multivector: Pseudoscalar, Multivector => Multivector }
impl_geometric_product! { pseudoscalar_mul_scalar: Pseudoscalar, Scalar => Pseudoscalar }
impl_geometric_product! { pseudoscalar_mul_vector: Pseudoscalar, Vector => Trivector }
impl_geometric_product! { pseudoscalar_mul_bivector: Pseudoscalar, Bivector => Bivector }
impl_geometric_product! { pseudoscalar_mul_trivector: Pseudoscalar, Trivector => Vector }
impl_geometric_product! { return_empty: Pseudoscalar, Pseudoscalar => Empty }

impl_geometric_product! { return_empty: Empty, Multivector => Empty }
impl_geometric_product! { return_empty: Empty, Scalar => Empty }
impl_geometric_product! { return_empty: Empty, Vector => Empty }
impl_geometric_product! { return_empty: Empty, Bivector => Empty }
impl_geometric_product! { return_empty: Empty, Trivector => Empty }
impl_geometric_product! { return_empty: Empty, Pseudoscalar => Empty }
impl_geometric_product! { return_empty: Empty, Empty => Empty }
impl_geometric_product! { return_empty: Multivector, Empty => Empty }
impl_geometric_product! { return_empty: Scalar, Empty => Empty }
impl_geometric_product! { return_empty: Vector, Empty => Empty }
impl_geometric_product! { return_empty: Bivector, Empty => Empty }
impl_geometric_product! { return_empty: Trivector, Empty => Empty }
impl_geometric_product! { return_empty: Pseudoscalar, Empty => Empty }

// Multivector

#[rustfmt::skip]
#[inline]
fn multivector_mul_multivector(
  lhs: Multivector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.s*b.s
        + a.e1*b.e1
        + a.e2*b.e2
        + a.e3*b.e3
        - a.e23*b.e23
        - a.e31*b.e31
        - a.e12*b.e12
        - a.e123*b.e123;
  let e0 = a.e0*b.s + a.s*b.e0
         + a.e01*b.e1 - a.e1*b.e01
         + a.e02*b.e2 - a.e2*b.e02
         + a.e03*b.e3 - a.e3*b.e03
         + a.e021*b.e12 + a.e12*b.e021
         + a.e013*b.e31 + a.e31*b.e013
         + a.e032*b.e23 + a.e23*b.e032
         - a.e0123*b.e123 + a.e123*b.e0123;
  let e1 = a.e1*b.s + a.s*b.e1
         + a.e12*b.e2 - a.e2*b.e12
         - a.e31*b.e3 + a.e3*b.e31
         - a.e123*b.e23 - a.e23*b.e123;
  let e2 = a.e2*b.s + a.s*b.e2
         - a.e12*b.e1 + a.e1*b.e12
         + a.e23*b.e3 - a.e3*b.e23
         - a.e123*b.e31 - a.e31*b.e123;
  let e3 = a.e3*b.s + a.s*b.e3
         + a.e31*b.e1 - a.e1*b.e31
         - a.e23*b.e2 + a.e2*b.e23
         - a.e123*b.e12 - a.e12*b.e123;
  let e01 = a.e01*b.s + a.s*b.e01
          - a.e1*b.e0 + a.e0*b.e1
          + a.e12*b.e02 - a.e02*b.e12
          - a.e31*b.e03 + a.e03*b.e31
          - a.e021*b.e2 - a.e2*b.e021
          + a.e013*b.e3 + a.e3*b.e013
          - a.e123*b.e032 + a.e032*b.e123
          - a.e0123*b.e23 - a.e23*b.e0123;
  let e02 = a.e02*b.s + a.s*b.e02
          - a.e2*b.e0 + a.e0*b.e2
          - a.e12*b.e01 + a.e01*b.e12
          + a.e23*b.e03 - a.e03*b.e23
          + a.e021*b.e1 + a.e1*b.e021
          - a.e032*b.e3 - a.e3*b.e032
          - a.e123*b.e013 + a.e013*b.e123
          - a.e0123*b.e31 - a.e31*b.e0123;
  let e03 = a.e03*b.s + a.s*b.e03
          - a.e3*b.e0 + a.e0*b.e3
          + a.e31*b.e01 - a.e01*b.e31
          - a.e23*b.e02 + a.e02*b.e23
          - a.e013*b.e1 - a.e1*b.e013
          + a.e032*b.e2 + a.e2*b.e032
          - a.e123*b.e021 + a.e021*b.e123
          - a.e0123*b.e12 - a.e12*b.e0123;
  let e12 = a.e12*b.s + a.s*b.e12
          - a.e2*b.e1 + a.e1*b.e2
          - a.e23*b.e31 + a.e31*b.e23
          + a.e123*b.e3 + a.e3*b.e123;
  let e31 = a.e31*b.s + a.s*b.e31
          + a.e3*b.e1 - a.e1*b.e3
          + a.e23*b.e12 - a.e12*b.e23
          + a.e123*b.e2 + a.e2*b.e123;
  let e23 = a.e23*b.s + a.s*b.e23
          - a.e3*b.e2 + a.e2*b.e3
          - a.e31*b.e12 + a.e12*b.e31
          + a.e123*b.e1 + a.e1*b.e123;
  let e021 = a.e021*b.s + a.s*b.e021
           - a.e01*b.e2 - a.e2*b.e01
           + a.e02*b.e1 + a.e1*b.e02
           - a.e12*b.e0 - a.e0*b.e12
           + a.e013*b.e23 - a.e23*b.e013
           - a.e032*b.e31 + a.e31*b.e032
           + a.e123*b.e03 - a.e03*b.e123
           - a.e0123*b.e3 + a.e3*b.e0123;
  let e013 = a.e013*b.s + a.s*b.e013
           + a.e01*b.e3 + a.e3*b.e01
           - a.e03*b.e1 - a.e1*b.e03
           - a.e31*b.e0 - a.e0*b.e31
           - a.e021*b.e23 + a.e23*b.e021
           + a.e032*b.e12 - a.e12*b.e032
           + a.e123*b.e02 - a.e02*b.e123
           - a.e0123*b.e2 + a.e2*b.e0123;
  let e032 = a.e032*b.s + a.s*b.e032
           - a.e02*b.e3 - a.e3*b.e02
           + a.e03*b.e2 + a.e2*b.e03
           - a.e23*b.e0 - a.e0*b.e23
           + a.e021*b.e31 - a.e31*b.e021
           - a.e013*b.e12 + a.e12*b.e013
           + a.e123*b.e01 - a.e01*b.e123
           - a.e0123*b.e1 + a.e1*b.e0123;
  let e123 = a.e123*b.s + a.s*b.e123
           + a.e12*b.e3 + a.e3*b.e12
           + a.e31*b.e2 + a.e2*b.e31
           + a.e23*b.e1 + a.e1*b.e23;
  let e0123 = a.e0123*b.s + a.s*b.e0123
            + a.e12*b.e03 + a.e03*b.e12
            + a.e31*b.e02 + a.e02*b.e31
            + a.e23*b.e01 + a.e01*b.e23
            - a.e021*b.e3 + a.e3*b.e021
            - a.e013*b.e2 + a.e2*b.e013
            - a.e032*b.e1 + a.e1*b.e032
            - a.e123*b.e0 + a.e0*b.e123;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_mul_scalar(
  lhs: Multivector,
  rhs: Scalar,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.s*b.s;
  let e0 = a.e0*b.s;
  let e1 = a.e1*b.s;
  let e2 = a.e2*b.s;
  let e3 = a.e3*b.s;
  let e01 = a.e01*b.s;
  let e02 = a.e02*b.s;
  let e03 = a.e03*b.s;
  let e12 = a.e12*b.s;
  let e31 = a.e31*b.s;
  let e23 = a.e23*b.s;
  let e021 = a.e021*b.s;
  let e013 = a.e013*b.s;
  let e032 = a.e032*b.s;
  let e123 = a.e123*b.s;
  let e0123 = a.e0123*b.s;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_mul_vector(
  lhs: Multivector,
  rhs: Vector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e1*b.e1
        + a.e2*b.e2
        + a.e3*b.e3;
  let e0 = a.s*b.e0
         + a.e01*b.e1
         + a.e02*b.e2
         + a.e03*b.e3;
  let e1 = a.s*b.e1
         + a.e12*b.e2
         - a.e31*b.e3;
  let e2 = a.s*b.e2
         - a.e12*b.e1
         + a.e23*b.e3;
  let e3 = a.s*b.e3
         + a.e31*b.e1
         - a.e23*b.e2;
  let e01 = -a.e1*b.e0 + a.e0*b.e1
          - a.e021*b.e2
          + a.e013*b.e3;
  let e02 = -a.e2*b.e0 + a.e0*b.e2
          + a.e021*b.e1
          - a.e032*b.e3;
  let e03 = -a.e3*b.e0 + a.e0*b.e3
          - a.e013*b.e1
          + a.e032*b.e2;
  let e12 = -a.e2*b.e1 + a.e1*b.e2
          + a.e123*b.e3;
  let e31 = a.e3*b.e1 - a.e1*b.e3
          + a.e123*b.e2;
  let e23 = -a.e3*b.e2 + a.e2*b.e3
          + a.e123*b.e1;
  let e021 = -a.e01*b.e2
           + a.e02*b.e1
           - a.e12*b.e0
           - a.e0123*b.e3;
  let e013 = a.e01*b.e3
           - a.e03*b.e1
           - a.e31*b.e0
           - a.e0123*b.e2;
  let e032 = -a.e02*b.e3
           + a.e03*b.e2
           - a.e23*b.e0
           - a.e0123*b.e1;
  let e123 = a.e12*b.e3
           + a.e31*b.e2
           + a.e23*b.e1;
  let e0123 = -a.e021*b.e3
            - a.e013*b.e2
            - a.e032*b.e1
            - a.e123*b.e0;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_mul_bivector(
  lhs: Multivector,
  rhs: Bivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = -a.e23*b.e23
        - a.e31*b.e31
        - a.e12*b.e12;
  let e0 = -a.e1*b.e01
         - a.e2*b.e02
         - a.e3*b.e03
         + a.e021*b.e12
         + a.e013*b.e31
         + a.e032*b.e23;
  let e1 = -a.e2*b.e12
         + a.e3*b.e31
         - a.e123*b.e23;
  let e2 = a.e1*b.e12
         - a.e3*b.e23
         - a.e123*b.e31;
  let e3 = -a.e1*b.e31
         + a.e2*b.e23
         - a.e123*b.e12;
  let e01 = a.s*b.e01
          + a.e12*b.e02 - a.e02*b.e12
          - a.e31*b.e03 + a.e03*b.e31
          - a.e0123*b.e23;
  let e02 = a.s*b.e02
          - a.e12*b.e01 + a.e01*b.e12
          + a.e23*b.e03 - a.e03*b.e23
          - a.e0123*b.e31;
  let e03 = a.s*b.e03
          + a.e31*b.e01 - a.e01*b.e31
          - a.e23*b.e02 + a.e02*b.e23
          - a.e0123*b.e12;
  let e12 = a.s*b.e12
          - a.e23*b.e31 + a.e31*b.e23;
  let e31 = a.s*b.e31
          + a.e23*b.e12 - a.e12*b.e23;
  let e23 = a.s*b.e23
          - a.e31*b.e12 + a.e12*b.e31;
  let e021 = -a.e2*b.e01
           + a.e1*b.e02
           - a.e0*b.e12
           + a.e013*b.e23
           - a.e032*b.e31
           + a.e123*b.e03;
  let e013 = a.e3*b.e01
           - a.e1*b.e03
           - a.e0*b.e31
           - a.e021*b.e23
           + a.e032*b.e12
           + a.e123*b.e02;
  let e032 = -a.e3*b.e02
           + a.e2*b.e03
           - a.e0*b.e23
           + a.e021*b.e31
           - a.e013*b.e12
           + a.e123*b.e01;
  let e123 = a.e3*b.e12
           + a.e2*b.e31
           + a.e1*b.e23;
  let e0123 = a.e12*b.e03 + a.e03*b.e12
            + a.e31*b.e02 + a.e02*b.e31
            + a.e23*b.e01 + a.e01*b.e23;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_mul_trivector(
  lhs: Multivector,
  rhs: Trivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = -a.e123*b.e123;
  let e0 = a.e12*b.e021
         + a.e31*b.e013
         + a.e23*b.e032
         - a.e0123*b.e123;
  let e1 = -a.e23*b.e123;
  let e2 = -a.e31*b.e123;
  let e3 = -a.e12*b.e123;
  let e01 = -a.e2*b.e021
          + a.e3*b.e013
          - a.e123*b.e032 + a.e032*b.e123;
  let e02 = a.e1*b.e021
          - a.e3*b.e032
          - a.e123*b.e013 + a.e013*b.e123;
  let e03 = -a.e1*b.e013
          + a.e2*b.e032
          - a.e123*b.e021 + a.e021*b.e123;
  let e12 = a.e3*b.e123;
  let e31 = a.e2*b.e123;
  let e23 = a.e1*b.e123;
  let e021 = a.s*b.e021
           - a.e23*b.e013
           + a.e31*b.e032
           - a.e03*b.e123;
  let e013 = a.s*b.e013
           + a.e23*b.e021
           - a.e12*b.e032
           - a.e02*b.e123;
  let e032 = a.s*b.e032
           - a.e31*b.e021
           + a.e12*b.e013
           - a.e01*b.e123;
  let e123 = a.s*b.e123;
  let e0123 = a.e3*b.e021
            + a.e2*b.e013
            + a.e1*b.e032
            + a.e0*b.e123;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_mul_pseudoscalar(
  lhs: Multivector,
  rhs: Pseudoscalar,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e123*b.e0123;
  let e01 = -a.e23*b.e0123;
  let e02 = -a.e31*b.e0123;
  let e03 = -a.e12*b.e0123;
  let e021 = a.e3*b.e0123;
  let e013 = a.e2*b.e0123;
  let e032 = a.e1*b.e0123;
  let e0123 = a.s*b.e0123;

  Multivector {
    e0,
    e01, e02, e03,
    e032, e013, e021,
    e0123,
    ..zero()
  }
}

// Scalar

#[rustfmt::skip]
#[inline]
fn scalar_mul_multivector(
  lhs: Scalar,
  mut rhs: Multivector,
) -> Multivector {
  rhs.s *= lhs.s;
  rhs.e0 *= lhs.s;
  rhs.e1 *= lhs.s;
  rhs.e2 *= lhs.s;
  rhs.e3 *= lhs.s;
  rhs.e01 *= lhs.s;
  rhs.e02 *= lhs.s;
  rhs.e03 *= lhs.s;
  rhs.e12 *= lhs.s;
  rhs.e31 *= lhs.s;
  rhs.e23 *= lhs.s;
  rhs.e021 *= lhs.s;
  rhs.e013 *= lhs.s;
  rhs.e032 *= lhs.s;
  rhs.e123 *= lhs.s;
  rhs.e0123 *= lhs.s;

  rhs
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_scalar(
  mut lhs: Scalar,
  rhs: Scalar,
) -> Scalar {
  lhs.s *= rhs.s;

  lhs
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_vector(
  lhs: Scalar,
  mut rhs: Vector,
) -> Vector {
  rhs.e0 *= lhs.s;
  rhs.e1 *= lhs.s;
  rhs.e2 *= lhs.s;
  rhs.e3 *= lhs.s;

  rhs
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_bivector(
  lhs: Scalar,
  mut rhs: Bivector,
) -> Bivector {
  rhs.e01 *= lhs.s;
  rhs.e02 *= lhs.s;
  rhs.e03 *= lhs.s;
  rhs.e12 *= lhs.s;
  rhs.e31 *= lhs.s;
  rhs.e23 *= lhs.s;

  rhs
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_trivector(
  lhs: Scalar,
  mut rhs: Trivector,
) -> Trivector {
  rhs.e021 *= lhs.s;
  rhs.e013 *= lhs.s;
  rhs.e032 *= lhs.s;
  rhs.e123 *= lhs.s;

  rhs
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_pseudoscalar(
  lhs: Scalar,
  mut rhs: Pseudoscalar,
) -> Pseudoscalar {
  rhs.e0123 *= lhs.s;

  rhs
}

// Vector

#[rustfmt::skip]
#[inline]
fn vector_mul_multivector(
  lhs: Vector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e1*b.e1
        + a.e2*b.e2
        + a.e3*b.e3;
  let e0 = a.e0*b.s
         - a.e1*b.e01
         - a.e2*b.e02
         - a.e3*b.e03;
  let e1 = a.e1*b.s
         - a.e2*b.e12
         + a.e3*b.e31;
  let e2 = a.e2*b.s
         + a.e1*b.e12
         - a.e3*b.e23;
  let e3 = a.e3*b.s
         - a.e1*b.e31
         + a.e2*b.e23;
  let e01 = -a.e1*b.e0 + a.e0*b.e1
          - a.e2*b.e021
          + a.e3*b.e013;
  let e02 = -a.e2*b.e0 + a.e0*b.e2
          + a.e1*b.e021
          - a.e3*b.e032;
  let e03 = -a.e3*b.e0 + a.e0*b.e3
          - a.e1*b.e013
          + a.e2*b.e032;
  let e12 = -a.e2*b.e1 + a.e1*b.e2
          + a.e3*b.e123;
  let e31 = a.e3*b.e1 - a.e1*b.e3
          + a.e2*b.e123;
  let e23 = -a.e3*b.e2 + a.e2*b.e3
          + a.e1*b.e123;
  let e021 = -a.e2*b.e01
           + a.e1*b.e02
           - a.e0*b.e12
           + a.e3*b.e0123;
  let e013 = a.e3*b.e01
           - a.e1*b.e03
           - a.e0*b.e31
           + a.e2*b.e0123;
  let e032 = -a.e3*b.e02
           + a.e2*b.e03
           - a.e0*b.e23
           + a.e1*b.e0123;
  let e123 = a.e3*b.e12
           + a.e2*b.e31
           + a.e1*b.e23;
  let e0123 = a.e3*b.e021
            + a.e2*b.e013
            + a.e1*b.e032
            + a.e0*b.e123;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn vector_mul_scalar(
  mut lhs: Vector,
  rhs: Scalar,
) -> Vector {
  lhs.e0 *= rhs.s;
  lhs.e1 *= rhs.s;
  lhs.e2 *= rhs.s;
  lhs.e3 *= rhs.s;

  lhs
}

#[rustfmt::skip]
#[inline]
fn vector_mul_vector(
  lhs: Vector,
  rhs: Vector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e1*b.e1
        + a.e2*b.e2
        + a.e3*b.e3;
  let e01 = -a.e1*b.e0 + a.e0*b.e1;
  let e02 = -a.e2*b.e0 + a.e0*b.e2;
  let e03 = -a.e3*b.e0 + a.e0*b.e3;
  let e12 = -a.e2*b.e1 + a.e1*b.e2;
  let e31 = a.e3*b.e1 - a.e1*b.e3;
  let e23 = -a.e3*b.e2 + a.e2*b.e3;

  Multivector {
    s,
    e01, e02, e03, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_mul_bivector(
  lhs: Vector,
  rhs: Bivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let e0 = -a.e1*b.e01
         - a.e2*b.e02
         - a.e3*b.e03;
  let e1 = -a.e2*b.e12
         + a.e3*b.e31;
  let e2 = a.e1*b.e12
         - a.e3*b.e23;
  let e3 = -a.e1*b.e31
         + a.e2*b.e23;
  let e021 = -a.e2*b.e01
           + a.e1*b.e02
           - a.e0*b.e12;
  let e013 = a.e3*b.e01
           - a.e1*b.e03
           - a.e0*b.e31;
  let e032 = -a.e3*b.e02
           + a.e2*b.e03
           - a.e0*b.e23;
  let e123 = a.e3*b.e12
           + a.e2*b.e31
           + a.e1*b.e23;

  Multivector {
      e0,    e1,    e2,    e3,
    e123,  e032,  e013,  e021,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_mul_trivector(
  lhs: Vector,
  rhs: Trivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let e01 = -a.e2*b.e021
          + a.e3*b.e013;
  let e02 = a.e1*b.e021
          - a.e3*b.e032;
  let e03 = -a.e1*b.e013
          + a.e2*b.e032;
  let e12 = a.e3*b.e123;
  let e31 = a.e2*b.e123;
  let e23 = a.e1*b.e123;
  let e0123 = a.e3*b.e021
            + a.e2*b.e013
            + a.e1*b.e032
            + a.e0*b.e123;

  Multivector {
    e23, e31, e12, e01, e02, e03,
    e0123,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_mul_pseudoscalar(
  lhs: Vector,
  rhs: Pseudoscalar,
) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e021 = a.e3*b.e0123;
  let e013 = a.e2*b.e0123;
  let e032 = a.e1*b.e0123;

  Trivector {
    e032, e013, e021,
    ..zero()
  }
}

// Bivector

#[rustfmt::skip]
#[inline]
fn bivector_mul_multivector(
  lhs: Bivector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = -a.e23*b.e23
        - a.e31*b.e31
        - a.e12*b.e12;
  let e0 = a.e01*b.e1
         + a.e02*b.e2
         + a.e03*b.e3
         + a.e12*b.e021
         + a.e31*b.e013
         + a.e23*b.e032;
  let e1 = a.e12*b.e2
         - a.e31*b.e3
         - a.e23*b.e123;
  let e2 = -a.e12*b.e1
         + a.e23*b.e3
         - a.e31*b.e123;
  let e3 = a.e31*b.e1
         - a.e23*b.e2
         - a.e12*b.e123;
  let e01 = a.e01*b.s
          + a.e12*b.e02 - a.e02*b.e12
          - a.e31*b.e03 + a.e03*b.e31
          - a.e23*b.e0123;
  let e02 = a.e02*b.s
          - a.e12*b.e01 + a.e01*b.e12
          + a.e23*b.e03 - a.e03*b.e23
          - a.e31*b.e0123;
  let e03 = a.e03*b.s
          + a.e31*b.e01 - a.e01*b.e31
          - a.e23*b.e02 + a.e02*b.e23
          - a.e12*b.e0123;
  let e12 = a.e12*b.s
          - a.e23*b.e31 + a.e31*b.e23;
  let e31 = a.e31*b.s
          + a.e23*b.e12 - a.e12*b.e23;
  let e23 = a.e23*b.s
          - a.e31*b.e12 + a.e12*b.e31;
  let e021 = -a.e01*b.e2
           + a.e02*b.e1
           - a.e12*b.e0
           - a.e23*b.e013
           + a.e31*b.e032
           - a.e03*b.e123;
  let e013 = a.e01*b.e3
           - a.e03*b.e1
           - a.e31*b.e0
           + a.e23*b.e021
           - a.e12*b.e032
           - a.e02*b.e123;
  let e032 = -a.e02*b.e3
           + a.e03*b.e2
           - a.e23*b.e0
           - a.e31*b.e021
           + a.e12*b.e013
           - a.e01*b.e123;
  let e123 = a.e12*b.e3
           + a.e31*b.e2
           + a.e23*b.e1;
  let e0123 = a.e12*b.e03 + a.e03*b.e12
            + a.e31*b.e02 + a.e02*b.e31
            + a.e23*b.e01 + a.e01*b.e23;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_mul_scalar(
  mut lhs: Bivector,
  rhs: Scalar,
) -> Bivector {

  lhs.e01 *= rhs.s;
  lhs.e02 *= rhs.s;
  lhs.e03 *= rhs.s;
  lhs.e12 *= rhs.s;
  lhs.e31 *= rhs.s;
  lhs.e23 *= rhs.s;

  lhs
}

#[rustfmt::skip]
#[inline]
fn bivector_mul_vector(
  lhs: Bivector,
  rhs: Vector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e01*b.e1
         + a.e02*b.e2
         + a.e03*b.e3;
  let e1 = a.e12*b.e2
         - a.e31*b.e3;
  let e2 = -a.e12*b.e1
         + a.e23*b.e3;
  let e3 = a.e31*b.e1
         - a.e23*b.e2;
  let e021 = -a.e01*b.e2
           + a.e02*b.e1
           - a.e12*b.e0;
  let e013 = a.e01*b.e3
           - a.e03*b.e1
           - a.e31*b.e0;
  let e032 = -a.e02*b.e3
           + a.e03*b.e2
           - a.e23*b.e0;
  let e123 = a.e12*b.e3
           + a.e31*b.e2
           + a.e23*b.e1;

  Multivector {
    e0, e1, e2, e3,
    e123, e032, e013, e021,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_mul_bivector(
  lhs: Bivector,
  rhs: Bivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = -a.e23*b.e23
        - a.e31*b.e31
        - a.e12*b.e12;
  let e01 = a.e12*b.e02 - a.e02*b.e12
          - a.e31*b.e03 + a.e03*b.e31;
  let e02 = -a.e12*b.e01 + a.e01*b.e12
          + a.e23*b.e03 - a.e03*b.e23;
  let e03 = a.e31*b.e01 - a.e01*b.e31
          - a.e23*b.e02 + a.e02*b.e23;
  let e12 = -a.e23*b.e31 + a.e31*b.e23;
  let e31 = a.e23*b.e12 - a.e12*b.e23;
  let e23 = -a.e31*b.e12 + a.e12*b.e31;
  let e0123 = a.e12*b.e03 + a.e03*b.e12
            + a.e31*b.e02 + a.e02*b.e31
            + a.e23*b.e01 + a.e01*b.e23;

  Multivector {
    s,
    e23, e31, e12,
    e01, e02, e03,
    e0123,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_mul_trivector(
  lhs: Bivector,
  rhs: Trivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e12*b.e021
         + a.e31*b.e013
         + a.e23*b.e032;
  let e1 = -a.e23*b.e123;
  let e2 = -a.e31*b.e123;
  let e3 = -a.e12*b.e123;
  let e021 = -a.e23*b.e013
           + a.e31*b.e032
           - a.e03*b.e123;
  let e013 = a.e23*b.e021
           - a.e12*b.e032
           - a.e02*b.e123;
  let e032 = -a.e31*b.e021
           + a.e12*b.e013
           - a.e01*b.e123;

  Multivector {
    e0, e1, e2, e3,
    e032, e013, e021,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_mul_pseudoscalar(
  lhs: Bivector,
  rhs: Pseudoscalar,
) -> Bivector {
  let (a, b) = (lhs, rhs);

  let e01 = -a.e23*b.e0123;
  let e02 = -a.e31*b.e0123;
  let e03 = -a.e12*b.e0123;

  Bivector {
    e01,   e02,   e03,
    ..zero()
  }
}

// Trivector

#[rustfmt::skip]
#[inline]
fn trivector_mul_multivector(
  lhs: Trivector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = -a.e123*b.e123;
  let e0 = a.e021*b.e12
         + a.e013*b.e31
         + a.e032*b.e23
         + a.e123*b.e0123;
  let e1 = -a.e123*b.e23;
  let e2 = -a.e123*b.e31;
  let e3 = -a.e123*b.e12;
  let e01 = -a.e021*b.e2
          + a.e013*b.e3
          - a.e123*b.e032 + a.e032*b.e123;
  let e02 = a.e021*b.e1
          - a.e032*b.e3
          - a.e123*b.e013 + a.e013*b.e123;
  let e03 = -a.e013*b.e1
          + a.e032*b.e2
          - a.e123*b.e021 + a.e021*b.e123;
  let e12 = a.e123*b.e3;
  let e31 = a.e123*b.e2;
  let e23 = a.e123*b.e1;
  let e021 = a.e021*b.s
           + a.e013*b.e23
           - a.e032*b.e31
           + a.e123*b.e03;
  let e013 = a.e013*b.s
           - a.e021*b.e23
           + a.e032*b.e12
           + a.e123*b.e02;
  let e032 = a.e032*b.s + a.e021*b.e31
           - a.e013*b.e12
           + a.e123*b.e01;
  let e123 = a.e123*b.s;
  let e0123 = -a.e021*b.e3
            - a.e013*b.e2
            - a.e032*b.e1
            - a.e123*b.e0;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_mul_scalar(
  mut lhs: Trivector,
  rhs: Scalar,
) -> Trivector {
  lhs.e021 *= rhs.s;
  lhs.e013 *= rhs.s;
  lhs.e032 *= rhs.s;
  lhs.e123 *= rhs.s;

  lhs
}

#[rustfmt::skip]
#[inline]
fn trivector_mul_vector(
  lhs: Trivector,
  rhs: Vector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let e01 = -a.e021*b.e2
          + a.e013*b.e3;
  let e02 = a.e021*b.e1
          - a.e032*b.e3;
  let e03 = -a.e013*b.e1
          + a.e032*b.e2;
  let e12 = a.e123*b.e3;
  let e31 = a.e123*b.e2;
  let e23 = a.e123*b.e1;
  let e0123 = -a.e021*b.e3
            - a.e013*b.e2
            - a.e032*b.e1
            - a.e123*b.e0;

  Multivector {
    e23, e31, e12,
    e01, e02, e03,
    e0123,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_mul_bivector(
  lhs: Trivector,
  rhs: Bivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e021*b.e12
         + a.e013*b.e31
         + a.e032*b.e23;
  let e1 = -a.e123*b.e23;
  let e2 = -a.e123*b.e31;
  let e3 = -a.e123*b.e12;
  let e021 = a.e013*b.e23
           - a.e032*b.e31
           + a.e123*b.e03;
  let e013 = -a.e021*b.e23
           + a.e032*b.e12
           + a.e123*b.e02;
  let e032 = a.e021*b.e31
           - a.e013*b.e12
           + a.e123*b.e01;

  Multivector {
    e0, e1, e2, e3,
    e032, e013, e021,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_mul_trivector(
  lhs: Trivector,
  rhs: Trivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = -a.e123*b.e123;
  let e01 = -a.e123*b.e032 + a.e032*b.e123;
  let e02 = -a.e123*b.e013 + a.e013*b.e123;
  let e03 = -a.e123*b.e021 + a.e021*b.e123;

  Multivector {
    s,
    e01, e02, e03,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_mul_pseudoscalar(
  lhs: Trivector,
  rhs: Pseudoscalar,
) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e123*b.e0123;

  Vector { e0, ..zero() }
}

// Pseudoscalar

#[rustfmt::skip]
#[inline]
fn pseudoscalar_mul_multivector(
  lhs: Pseudoscalar,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let e0 = -a.e0123*b.e123;
  let e01 = -a.e0123*b.e23;
  let e02 = -a.e0123*b.e31;
  let e03 = -a.e0123*b.e12;
  let e021 = -a.e0123*b.e3;
  let e013 = -a.e0123*b.e2;
  let e032 = -a.e0123*b.e1;
  let e0123 = a.e0123*b.s;

  Multivector {
    e0,
    e01, e02, e03, e0123,
    e032, e013, e021,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_mul_scalar(
  mut lhs: Pseudoscalar,
  rhs: Scalar,
) -> Pseudoscalar {
  lhs.e0123 *= rhs.s;

  lhs
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_mul_vector(
  lhs: Pseudoscalar,
  rhs: Vector,
) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e021 = -a.e0123*b.e3;
  let e013 = -a.e0123*b.e2;
  let e032 = -a.e0123*b.e1;

  Trivector {
    e021, e013, e032,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_mul_bivector(
  lhs: Pseudoscalar,
  rhs: Bivector,
) -> Bivector {
  let (a, b) = (lhs, rhs);

  let e01 = -a.e0123*b.e23;
  let e02 = -a.e0123*b.e31;
  let e03 = -a.e0123*b.e12;

  Bivector {
    e01, e02, e03,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_mul_trivector(
  lhs: Pseudoscalar,
  rhs: Trivector,
) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = -a.e0123*b.e123;

  Vector { e0, ..zero() }
}

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

  mod multivector {
    use super::*;
    #[test]
    fn mul_multivector_1() {
      let result = MULTIVECTOR_A.geometric_product(MULTIVECTOR_B);
      let expected = Multivector {
          e0: 23311.,   e1: -3564.,   e2: -4676.,    e3: -4116.,
           s: -6780.,  e23:  4596.,  e31:  5316.,   e12:  6200.,
         e01: -1389.,  e02: -3031.,  e03:  -879., e0123:  5951.,
        e123:  8748., e032:  3219., e013:  2003.,  e021:  3437.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_multivector_2() {
      let result = MULTIVECTOR_A.geometric_product(MULTIVECTOR_C);
      let expected = Multivector {
        s: 2704.,
        e0: 15805., e1: -4876., e2: 3632., e3: 1244.,
        e01: -12221., e02: 375., e03: -22879.,
        e12: 3656., e31: -5576., e23: -172.,
        e021: 10745., e013: 5225., e032: -23273., e123: -2744.,
        e0123: -1271.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_multivector_3() {
      let result = MULTIVECTOR_A.geometric_product(MULTIVECTOR_D);
      let expected = Multivector {
        s: 6780.,
        e0: -23311., e1: 3564., e2: 4676., e3: 4116.,
        e01: 1389., e02: 3031., e03: 879.,
        e12: -6200., e31: -5316., e23: -4596.,
        e021: -3437., e013: -2003., e032: -3219., e123: -8748.,
        e0123: -5951.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_scalar_1() {
      let result = MULTIVECTOR_A.geometric_product(SCALAR_A);
      let expected = Multivector {
        s: 1507.,
        e0: 274., e1: 411., e2: 685., e3: 959.,
        e01: 3151., e02: 3973., e03: 4247.,
        e12: 2603., e31: 2329., e23: 1781.,
        e021: 7261., e013: 6439., e032: 5891., e123: 5617.,
        e0123: 5069.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_scalar_2() {
      let result = MULTIVECTOR_A.geometric_product(SCALAR_C);
      let expected = Multivector {
        s: -1639.,
        e0: -298., e1: -447., e2: -745., e3: -1043.,
        e01: -3427., e02: -4321., e03: -4619.,
        e12: -2831., e31: -2533., e23: -1937.,
        e021: -7897., e013: -7003., e032: -6407., e123: -6109.,
        e0123: -5513.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_vector_1() {
      let result = MULTIVECTOR_A.geometric_product(VECTOR_A);
      let expected = Multivector {
        s: 2455.,
        e0: 15176., e1: 1985., e2: 981., e3: 2387.,
        e01: -929., e02: 711., e03: -1093.,
        e12: 6551., e31: 7281., e23: 6131.,
        e021: -8244., e013: -9624., e032: -7562., e123: 7985.,
        e0123: -29454.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_vector_2() {
      let result = MULTIVECTOR_A.geometric_product(VECTOR_C);
      let expected = Multivector {
        s: -3063.,
        e0: -18966., e1: -2361., e2: -1189., e3: -3083.,
        e01: 815., e02: -801., e03: 1631.,
        e12: -8263., e31: -8905., e23: -7739.,
        e021: 10338., e013: 11898., e032: 9748., e123: -9953.,
        e0123: 36920.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_bivector_1() {
      let result = MULTIVECTOR_A.geometric_product(BIVECTOR_A);
      let expected = Multivector {
        s: -11109.,
        e0: 28814., e1: -8699., e2: -10181., e3: -8955.,
        e01: -4848., e02: -8710., e03: -3722.,
        e12: 3359., e31: 1237., e23: 2873.,
        e021: 9695., e013: 8281., e032: 9907., e123: 3407.,
        e0123: 30482.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_bivector_2() {
      let result = MULTIVECTOR_A.geometric_product(BIVECTOR_C);
      let expected = Multivector {
        s: 14031.,
        e0: -36246., e1: 11005., e2: 12691., e3: 11457.,
        e01: 6156., e02: 10786., e03: 4582.,
        e12: -4321., e31: -1583., e23: -3487.,
        e021: -12683., e013: -11101., e032: -12641., e123: -4309.,
        e0123: -38978.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_trivector_1() {
      let result = MULTIVECTOR_A.geometric_product(TRIVECTOR_A);
      let expected = Multivector {
        s: -12997.,
        e0: 4896., e1: -4121., e2: -5389., e3: -6023.,
        e01: 684., e02: -194., e03: 3218.,
        e12: 2219., e31: 1585., e23: 951.,
        e021: -4764., e013: -7264., e032: -3146., e123: 3487.,
        e0123: 5741.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_trivector_2() {
      let result = MULTIVECTOR_A.geometric_product(TRIVECTOR_C);
      let expected = Multivector {
        s: 15293.,
        e0: -5028., e1: 4849., e2: 6341., e3: 7087.,
        e01: -1236., e02: -342., e03: -4566.,
        e12: -2611., e31: -1865., e23: -1119.,
        e021: 5820., e013: 8748., e032: 3746., e123: -4103.,
        e0123: -6521.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_pseudoscalar_1() {
      let result = MULTIVECTOR_A.geometric_product(PSEUDOSCALAR_A);
      let expected = Multivector {
        e0: 16277.,
        e01: -5161., e02: -6749., e03: -7543.,
        e021: 2779., e013: 1985., e032: 1191.,
        e0123: 4367.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_pseudoscalar_2() {
      let result = MULTIVECTOR_A.geometric_product(PSEUDOSCALAR_C);
      let expected = Multivector {
        e0: -16769.,
        e01: 5317., e02: 6953., e03: 7771.,
        e021: -2863., e013: -2045., e032: -1227.,
        e0123: -4499.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod scalar {
    use super::*;
    #[test]
    fn mul_multivector_1() {
      let result = SCALAR_A.geometric_product(MULTIVECTOR_A);
      let expected = Multivector {
        s: 1507.,
        e0: 274., e1: 411., e2: 685., e3: 959.,
        e01: 3151., e02: 3973., e03: 4247.,
        e12: 2603., e31: 2329., e23: 1781.,
        e021: 7261., e013: 6439., e032: 5891., e123: 5617.,
        e0123: 5069.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_scalar_1() {
      let result = SCALAR_A.geometric_product(SCALAR_B);
      let expected = Scalar { s: 19043. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_vector_1() {
      let result = SCALAR_A.geometric_product(VECTOR_A);
      let expected = Vector {
        e0: 20687., e1: 21509., e2: 22331., e3: 22879.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_bivector_1() {
      let result = SCALAR_A.geometric_product(BIVECTOR_A);
      let expected = Bivector {
        e01: 31921., e02: 32743., e03: 33017.,
        e12: 31373., e31: 31099., e23: 30551.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_trivector_1() {
      let result = SCALAR_A.geometric_product(TRIVECTOR_A);
      let expected = Trivector {
        e021: 47539., e013: 46169., e032: 45347., e123: 43429.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_pseudoscalar_1() {
      let result = SCALAR_A.geometric_product(PSEUDOSCALAR_A);
      let expected = Pseudoscalar { e0123: 54389. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod vector {
    use super::*;
    #[test]
    fn mul_multivector_1() {
      let result = VECTOR_A.geometric_product(MULTIVECTOR_A);
      let expected = Multivector {
        s: 2455.,
        e0: -11854., e1: 1469., e2: 2605., e3: 1287.,
        e01: -651., e02: 1569., e03: 353.,
        e12: 7143., e31: 6085., e23: 6743.,
        e021: 4114., e013: 2438., e032: 4056., e123: 7985.,
        e0123: 29454.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_scalar_1() {
      let result = VECTOR_A.geometric_product(SCALAR_A);
      let expected = Vector {
        e0: 20687., e1: 21509., e2: 22331., e3: 22879.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_vector_1() {
      let result = VECTOR_A.geometric_product(VECTOR_B);
      let expected = Multivector {
        s: 89503.,
        e01: -132., e02: -868., e03: -50.,
        e12: -760., e31: -94., e23: 906.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_bivector_1() {
      let result = VECTOR_A.geometric_product(BIVECTOR_A);
      let expected = Multivector {
        e0: -115785., e1: 582., e2: -1288., e3: 710.,
        e021: -35035., e013: -33203., e032: -34303., e123: 110255.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_trivector_1() {
      let result = VECTOR_A.geometric_product(TRIVECTOR_A);
      let expected = Multivector {
        e01: -282., e02: -798., e03: 1044.,
        e12: 52939., e31: 51671., e23: 49769.,
        e0123: 212714.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_pseudoscalar_1() {
      let result = VECTOR_A.geometric_product(PSEUDOSCALAR_A);
      let expected = Trivector {
        e021: 66299., e013: 64711., e032: 62329.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod bivector {
    use super::*;
    #[test]
    fn mul_multivector_1() {
      let result = BIVECTOR_A.geometric_product(MULTIVECTOR_A);
      let expected = Multivector {
        s: -11109.,
        e0: 35976., e1: -9587., e2: -8433., e3: -9823.,
        e01: -6528., e02: -2830., e03: -7922.,
        e12: 1679., e31: 3757., e23: 2033.,
        e021: -11507., e013: -7373., e032: -11735., e123: 3407.,
        e0123: 30482.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_scalar_1() {
      let result = BIVECTOR_A.geometric_product(SCALAR_A);
      let expected = Bivector {
        e01: 31921., e02: 32743.,e03: 33017.,
        e12: 31373., e31: 31099., e23: 30551.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_vector_1() {
      let result = BIVECTOR_A.geometric_product(VECTOR_A);
      let expected = Multivector {
        e0: 115785., e1: -582., e2: 1288., e3: -710.,
        e021: -35035., e013: -33203., e032: -34303.,
        e123: 110255.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_bivector_1() {
      let result = BIVECTOR_A.geometric_product(BIVECTOR_B);
      let expected = Multivector {
        s: -174539.,
        e01: -1740., e02: 958., e03: 738.,
        e12: -334., e31: 1170., e23: -848.,
        e0123: 368226.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_trivector_1() {
      let result = BIVECTOR_A.geometric_product(TRIVECTOR_A);
      let expected = Multivector {
        e0: 229775., e1: -70691., e2: -71959., e3: -72593.,
        e021: -76411., e013: -74181., e032: -75457.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_pseudoscalar_1() {
      let result = BIVECTOR_A.geometric_product(PSEUDOSCALAR_A);
      let expected = Bivector {
        e01: -88531., e02: -90119., e03: -90913.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod trivector {
    use super::*;
    #[test]
    fn mul_multivector_1() {
      let result = TRIVECTOR_A.geometric_product(MULTIVECTOR_A);
      let expected = Multivector {
        s: -12997.,
        e0: 28354., e1: -4121., e2: -5389., e3: -6023.,
        e01: 564., e02: -2358., e03: -1930.,
        e12: 2219., e31: 1585., e23: 951.,
        e021: 12398., e013: 14678., e032: 10428., e123: 3487.,
        e0123: -5741.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_scalar_1() {
      let result = TRIVECTOR_A.geometric_product(SCALAR_A);
      let expected = Trivector {
        e021: 47539., e013: 46169., e032: 45347., e123: 43429.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_vector_1() {
      let result = TRIVECTOR_A.geometric_product(VECTOR_A);
      let expected = Multivector {
        e01: -282., e02: -798., e03: 1044.,
        e12: 52939., e31: 51671., e23: 49769.,
        e0123: -212714.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_bivector_1() {
      let result = TRIVECTOR_A.geometric_product(BIVECTOR_A);
      let expected = Multivector {
        e0: 229775., e1: -70691., e2: -71959., e3: -72593.,
        e021: 76411., e013: 74181., e032: 75457.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_trivector_1() {
      let result = TRIVECTOR_A.geometric_product(TRIVECTOR_B);
      let expected = Multivector {
        s: -110633.,
        e01: 3618., e02: 3810., e03: 4764.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_pseudoscalar_1() {
      let result = TRIVECTOR_A.geometric_product(PSEUDOSCALAR_A);
      let expected = Vector {
        e0: 125849.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod pseudoscalar {
    use super::*;
    #[test]
    fn mul_multivector_1() {
      let result = PSEUDOSCALAR_A.geometric_product(MULTIVECTOR_A);
      let expected = Multivector {
        e0: -16277.,
        e01: -5161., e02: -6749., e03: -7543.,
        e021: -2779., e013: -1985., e032: -1191.,
        e0123: 4367.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_scalar_1() {
      let result = PSEUDOSCALAR_A.geometric_product(SCALAR_A);
      let expected = Pseudoscalar { e0123: 54389. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_vector_1() {
      let result = PSEUDOSCALAR_A.geometric_product(VECTOR_A);
      let expected = Trivector {
        e021: -66299., e013: -64711., e032: -62329.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_bivector_1() {
      let result = PSEUDOSCALAR_A.geometric_product(BIVECTOR_A);
      let expected = Bivector {
        e01: -88531., e02: -90119., e03: -90913.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_trivector_1() {
      let result = PSEUDOSCALAR_A.geometric_product(TRIVECTOR_A);
      let expected = Vector {
        e0: -125849.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn mul_pseudoscalar_1() {
      let result = PSEUDOSCALAR_A.geometric_product(PSEUDOSCALAR_B);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }
}
