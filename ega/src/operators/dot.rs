use super::return_empty;
use crate::*;

/// The inner product
pub trait Dot<Rhs> {
  type Output;

  /// The inner product
  fn dot(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_dot {
  ($dot_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl Dot<$rhs> for $lhs {
      type Output = $output;

      #[inline]
      fn dot(self, rhs: $rhs) -> Self::Output {
        $dot_fn(self, rhs)
      }
    }
  };
}

impl_dot! { multivector_dot_multivector: Multivector, Multivector => Multivector }
impl_dot! { multivector_dot_scalar: Multivector, Scalar => Multivector }
impl_dot! { multivector_dot_vector: Multivector, Vector => Multivector }
impl_dot! { multivector_dot_bivector: Multivector, Bivector => Multivector }
impl_dot! { multivector_dot_trivector: Multivector, Trivector => Multivector }
impl_dot! { multivector_dot_pseudoscalar: Multivector, Pseudoscalar => Multivector }

impl_dot! { scalar_dot_multivector: Scalar, Multivector => Multivector }
impl_dot! { scalar_dot_scalar: Scalar, Scalar => Scalar }
impl_dot! { scalar_dot_vector: Scalar, Vector => Vector }
impl_dot! { scalar_dot_bivector: Scalar, Bivector => Bivector }
impl_dot! { scalar_dot_trivector: Scalar, Trivector => Trivector }
impl_dot! { scalar_dot_pseudoscalar: Scalar, Pseudoscalar => Pseudoscalar }

impl_dot! { vector_dot_multivector: Vector, Multivector => Multivector }
impl_dot! { vector_dot_scalar: Vector, Scalar => Vector }
impl_dot! { vector_dot_vector: Vector, Vector => Scalar }
impl_dot! { vector_dot_bivector: Vector, Bivector => Vector }
impl_dot! { vector_dot_trivector: Vector, Trivector => Bivector }
impl_dot! { vector_dot_pseudoscalar: Vector, Pseudoscalar => Trivector }

impl_dot! { bivector_dot_multivector: Bivector, Multivector => Multivector }
impl_dot! { bivector_dot_scalar: Bivector, Scalar => Bivector }
impl_dot! { bivector_dot_vector: Bivector, Vector => Vector }
impl_dot! { bivector_dot_bivector: Bivector, Bivector => Scalar }
impl_dot! { bivector_dot_trivector: Bivector, Trivector => Vector }
impl_dot! { bivector_dot_pseudoscalar: Bivector, Pseudoscalar => Bivector }

impl_dot! { trivector_dot_multivector: Trivector, Multivector => Multivector }
impl_dot! { trivector_dot_scalar: Trivector, Scalar => Trivector }
impl_dot! { trivector_dot_vector: Trivector, Vector => Bivector }
impl_dot! { trivector_dot_bivector: Trivector, Bivector => Vector }
impl_dot! { trivector_dot_trivector: Trivector, Trivector => Scalar }
impl_dot! { trivector_dot_pseudoscalar: Trivector, Pseudoscalar => Vector }

impl_dot! { pseudoscalar_dot_multivector: Pseudoscalar, Multivector => Multivector }
impl_dot! { pseudoscalar_dot_scalar: Pseudoscalar, Scalar => Pseudoscalar }
impl_dot! { pseudoscalar_dot_vector: Pseudoscalar, Vector => Trivector }
impl_dot! { pseudoscalar_dot_bivector: Pseudoscalar, Bivector => Bivector }
impl_dot! { pseudoscalar_dot_trivector: Pseudoscalar, Trivector => Vector }
impl_dot! { pseudoscalar_dot_pseudoscalar: Pseudoscalar, Pseudoscalar => Empty }

impl_dot! { return_empty: Empty, Empty => Empty }
impl_dot! { return_empty: Multivector, Empty => Empty }
impl_dot! { return_empty: Scalar, Empty => Empty }
impl_dot! { return_empty: Vector, Empty => Empty }
impl_dot! { return_empty: Bivector, Empty => Empty }
impl_dot! { return_empty: Trivector, Empty => Empty }
impl_dot! { return_empty: Pseudoscalar, Empty => Empty }
impl_dot! { return_empty: Empty, Multivector => Empty }
impl_dot! { return_empty: Empty, Scalar => Empty }
impl_dot! { return_empty: Empty, Vector => Empty }
impl_dot! { return_empty: Empty, Bivector => Empty }
impl_dot! { return_empty: Empty, Trivector => Empty }
impl_dot! { return_empty: Empty, Pseudoscalar => Empty }

// Multivector

#[rustfmt::skip]
#[inline]
fn multivector_dot_multivector(
  lhs: Multivector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.s*b.s - a.e123*b.e123
        + a.e1*b.e1
        + a.e2*b.e2
        + a.e3*b.e3
        - a.e23*b.e23
        - a.e31*b.e31
        - a.e12*b.e12;
  let e0 = a.s*b.e0 + a.e0*b.s
         - a.e1*b.e01 + a.e01*b.e1
         - a.e2*b.e02 + a.e02*b.e2
         - a.e3*b.e03 + a.e03*b.e3
         + a.e23*b.e032 + a.e032*b.e23
         + a.e31*b.e013 + a.e013*b.e31
         + a.e12*b.e021 + a.e021*b.e12
         + a.e123*b.e0123 - a.e0123*b.e123;
  let e1 = a.s*b.e1 + a.e1*b.s
         - a.e2*b.e12 + a.e12*b.e2
         + a.e3*b.e31 - a.e31*b.e3
         - a.e23*b.e123 - a.e123*b.e23;
  let e2 = a.s*b.e2 + a.e2*b.s
         + a.e1*b.e12 - a.e12*b.e1
         - a.e3*b.e23 + a.e23*b.e3
         - a.e31*b.e123 - a.e123*b.e31;
  let e3 = a.s*b.e3 + a.e3*b.s
         - a.e1*b.e31 + a.e31*b.e1
         + a.e2*b.e23 - a.e23*b.e2
         - a.e12*b.e123 - a.e123*b.e12;
  let e01 = a.s*b.e01 + a.e01*b.s
          - a.e2*b.e021 - a.e021*b.e2
          + a.e3*b.e013 + a.e013*b.e3
          - a.e23*b.e0123 - a.e0123*b.e23;
  let e02 = a.s*b.e02 + a.e02*b.s
          + a.e1*b.e021 + a.e021*b.e1
          - a.e3*b.e032 - a.e032*b.e3
          - a.e31*b.e0123 - a.e0123*b.e31;
  let e03 = a.s*b.e03 + a.e03*b.s
          - a.e1*b.e013 - a.e013*b.e1
          + a.e2*b.e032 + a.e032*b.e2
          - a.e12*b.e0123 - a.e0123*b.e12;
  let e23 = a.s*b.e23 + a.e23*b.s
          + a.e1*b.e123 + a.e123*b.e1;
  let e31 = a.s*b.e31 + a.e31*b.s
          + a.e2*b.e123 + a.e123*b.e2;
  let e12 = a.s*b.e12 + a.e12*b.s
          + a.e3*b.e123 + a.e123*b.e3;
  let e123 = a.s*b.e123 + a.e123*b.s;
  let e032 = a.s*b.e032 + a.e032*b.s
           + a.e1*b.e0123 - a.e0123*b.e1;
  let e013 = a.s*b.e013 + a.e013*b.s
           + a.e2*b.e0123 - a.e0123*b.e2;
  let e021 = a.s*b.e021 + a.e021*b.s
           + a.e3*b.e0123 - a.e0123*b.e3;
  let e0123 = a.s*b.e0123 + a.e0123*b.s;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[inline]
fn multivector_dot_scalar(lhs: Multivector, rhs: Scalar) -> Multivector {
  MultivectorArray::from(
    lhs.to_multivector_array().elements.map(|e| e * rhs.s),
  )
  .to_multivector()
}

#[rustfmt::skip]
#[inline]
fn multivector_dot_vector(lhs: Multivector, rhs: Vector) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e1*b.e1 + a.e2*b.e2 + a.e3*b.e3;
  let e0 = a.s*b.e0
         + a.e01*b.e1 + a.e02*b.e2 + a.e03*b.e3;
  let e1 = a.s*b.e1
         + a.e12*b.e2 - a.e31*b.e3;
  let e2 = a.s*b.e2
         + a.e23*b.e3 - a.e12*b.e1;
  let e3 = a.s*b.e3
         + a.e31*b.e1 - a.e23*b.e2;
  let e01 = a.e013*b.e3 - a.e021*b.e2;
  let e02 = a.e021*b.e1 - a.e032*b.e3;
  let e03 = a.e032*b.e2 - a.e013*b.e1;
  let e23 = a.e123*b.e1;
  let e31 = a.e123*b.e2;
  let e12 = a.e123*b.e3;
  let e123 = 0f32;
  let e032 = -a.e0123*b.e1;
  let e013 = -a.e0123*b.e2;
  let e021 = -a.e0123*b.e3;
  let e0123 = 0f32;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_dot_bivector(lhs: Multivector, rhs: Bivector) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e123, e032, e013, e021, e0123] = [0f32; 5];

  let s = -a.e23*b.e23 - a.e31*b.e31 - a.e12*b.e12;
  let e0 = -a.e1*b.e01 - a.e2*b.e02 - a.e3*b.e03
         + a.e032*b.e23 + a.e013*b.e31 + a.e021*b.e12;
  let e1 = a.e3*b.e31 - a.e2*b.e12 - a.e123*b.e23;
  let e2 = a.e1*b.e12 - a.e3*b.e23 - a.e123*b.e31;
  let e3 = a.e2*b.e23 - a.e1*b.e31 - a.e123*b.e12;
  let e01 = a.s*b.e01 - a.e0123*b.e23;
  let e02 = a.s*b.e02 - a.e0123*b.e31;
  let e03 = a.s*b.e03 - a.e0123*b.e12;
  let e23 = a.s*b.e23;
  let e31 = a.s*b.e31;
  let e12 = a.s*b.e12;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_dot_trivector(
  lhs: Multivector,
  rhs: Trivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = -a.e123*b.e123;
  let e0 = a.e23*b.e032 + a.e31*b.e013 + a.e12*b.e021
         - a.e0123*b.e123;
  let e1 = -a.e23*b.e123;
  let e2 = -a.e31*b.e123;
  let e3 = -a.e12*b.e123;
  let e01 = a.e3*b.e013 - a.e2*b.e021;
  let e02 = a.e1*b.e021 - a.e3*b.e032;
  let e03 = a.e2*b.e032 - a.e1*b.e013;
  let e23 = a.e1*b.e123;
  let e31 = a.e2*b.e123;
  let e12 = a.e3*b.e123;
  let e123 = a.s*b.e123;
  let e032 = a.s*b.e032;
  let e013 = a.s*b.e013;
  let e021 = a.s*b.e021;
  let e0123 = 0f32;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_dot_pseudoscalar(
  lhs: Multivector,
  rhs: Pseudoscalar,
) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e1, e2, e3, s, e23, e31, e12, e123] = [0f32; 8];

  let e0 = a.e123*b.e0123;
  let e01 = -a.e23*b.e0123;
  let e02 = -a.e31*b.e0123;
  let e03 = -a.e12*b.e0123;
  let e032 = a.e1*b.e0123;
  let e013 = a.e2*b.e0123;
  let e021 = a.e3*b.e0123;
  let e0123 = a.s*b.e0123;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

// Scalar

#[inline]
fn scalar_dot_multivector(lhs: Scalar, rhs: Multivector) -> Multivector {
  MultivectorArray::from(
    rhs.to_multivector_array().elements.map(|e| lhs.s * e),
  )
  .to_multivector()
}

#[inline]
fn scalar_dot_scalar(lhs: Scalar, rhs: Scalar) -> Scalar {
  Scalar { s: lhs.s * rhs.s }
}

#[inline]
fn scalar_dot_vector(lhs: Scalar, rhs: Vector) -> Vector {
  VectorArray::from(rhs.to_vector_array().elements.map(|e| lhs.s * e))
    .to_vector()
}

#[inline]
fn scalar_dot_bivector(lhs: Scalar, rhs: Bivector) -> Bivector {
  BivectorArray::from(rhs.to_bivector_array().elements.map(|e| lhs.s * e))
    .to_bivector()
}

#[inline]
fn scalar_dot_trivector(lhs: Scalar, rhs: Trivector) -> Trivector {
  TrivectorArray::from(rhs.to_trivector_array().elements.map(|e| lhs.s * e))
    .to_trivector()
}

#[inline]
fn scalar_dot_pseudoscalar(lhs: Scalar, rhs: Pseudoscalar) -> Pseudoscalar {
  Pseudoscalar {
    e0123: lhs.s * rhs.e0123,
  }
}

// Vector

#[rustfmt::skip]
#[inline]
fn vector_dot_multivector(lhs: Vector, rhs: Multivector) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.e1*b.e1 + a.e2*b.e2 + a.e3*b.e3;
  let e0 = a.e0*b.s
         - a.e1*b.e01 - a.e2*b.e02 - a.e3*b.e03;
  let e1 = a.e1*b.s
         + a.e3*b.e31 - a.e2*b.e12;
  let e2 = a.e2*b.s
         + a.e1*b.e12 - a.e3*b.e23;
  let e3 = a.e3*b.s
         + a.e2*b.e23 - a.e1*b.e31;
  let e01 = a.e3*b.e013 - a.e2*b.e021;
  let e02 = a.e1*b.e021 - a.e3*b.e032;
  let e03 = a.e2*b.e032 - a.e1*b.e013;
  let e23 = a.e1*b.e123;
  let e31 = a.e2*b.e123;
  let e12 = a.e3*b.e123;
  let e123 = 0f32;
  let e032 = a.e1*b.e0123;
  let e013 = a.e2*b.e0123;
  let e021 = a.e3*b.e0123;
  let e0123 = 0f32;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[inline]
fn vector_dot_scalar(lhs: Vector, rhs: Scalar) -> Vector {
  VectorArray::from(lhs.to_vector_array().elements.map(|e| e * rhs.s))
    .to_vector()
}

#[rustfmt::skip]
#[inline]
fn vector_dot_vector(lhs: Vector, rhs: Vector) -> Scalar {
  let (a, b) = (lhs, rhs);
  let s = a.e1*b.e1 + a.e2*b.e2 + a.e3*b.e3;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn vector_dot_bivector(lhs: Vector, rhs: Bivector) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = -a.e1*b.e01 - a.e2*b.e02 - a.e3*b.e03;
  let e1 = a.e3*b.e31 - a.e2*b.e12;
  let e2 = a.e1*b.e12 - a.e3*b.e23;
  let e3 = a.e2*b.e23 - a.e1*b.e31;

  Vector { e0, e1, e2, e3 }
}

#[rustfmt::skip]
#[inline]
fn vector_dot_trivector(lhs: Vector, rhs: Trivector) -> Bivector {
  let (a, b) = (lhs, rhs);

  let e01 = a.e3*b.e013 - a.e2*b.e021;
  let e02 = a.e1*b.e021 - a.e3*b.e032;
  let e03 = a.e2*b.e032 - a.e1*b.e013;
  let e23 = a.e1*b.e123;
  let e31 = a.e2*b.e123;
  let e12 = a.e3*b.e123;

  Bivector { e23, e31, e12, e01, e02, e03 }
}

#[rustfmt::skip]
#[inline]
fn vector_dot_pseudoscalar(lhs: Vector, rhs: Pseudoscalar) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e123 = 0f32;
  let e032 = a.e1*b.e0123;
  let e013 = a.e2*b.e0123;
  let e021 = a.e3*b.e0123;

  Trivector { e123, e032, e013, e021 }
}

// Bivector

#[rustfmt::skip]
#[inline]
fn bivector_dot_multivector(lhs: Bivector, rhs: Multivector) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e0123, e123, e032, e013, e021] = [0f32; 5];

  let s = -a.e23*b.e23 - a.e31*b.e31 - a.e12*b.e12;
  let e0 = a.e01*b.e1 + a.e02*b.e2 + a.e03*b.e3
         + a.e23*b.e032 + a.e31*b.e013 + a.e12*b.e021;
  let e1 = a.e12*b.e2 - a.e31*b.e3 - a.e23*b.e123;
  let e2 = a.e23*b.e3 - a.e12*b.e1 - a.e31*b.e123;
  let e3 = a.e31*b.e1 - a.e23*b.e2 - a.e12*b.e123;
  let e01 = a.e01*b.s - a.e23*b.e0123;
  let e02 = a.e02*b.s - a.e31*b.e0123;
  let e03 = a.e03*b.s - a.e12*b.e0123;
  let e23 = a.e23*b.s;
  let e31 = a.e31*b.s;
  let e12 = a.e12*b.s;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[inline]
fn bivector_dot_scalar(lhs: Bivector, rhs: Scalar) -> Bivector {
  BivectorArray::from(lhs.to_bivector_array().elements.map(|e| e * rhs.s))
    .to_bivector()
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_vector(lhs: Bivector, rhs: Vector) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e01*b.e1 + a.e02*b.e2 + a.e03*b.e3;
  let e1 = a.e12*b.e2 - a.e31*b.e3;
  let e2 = a.e23*b.e3 - a.e12*b.e1;
  let e3 = a.e31*b.e1 - a.e23*b.e2;

  Vector { e0, e1, e2, e3 }
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_bivector(lhs: Bivector, rhs: Bivector) -> Scalar {
  let (a, b) = (lhs, rhs);
  let s = -a.e23*b.e23 - a.e31*b.e31 - a.e12*b.e12;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_trivector(lhs: Bivector, rhs: Trivector) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e23*b.e032 + a.e31*b.e013 + a.e12*b.e021;
  let e1 = -a.e23*b.e123;
  let e2 = -a.e31*b.e123;
  let e3 = -a.e12*b.e123;

  Vector { e0, e1, e2, e3 }
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_pseudoscalar(lhs: Bivector, rhs: Pseudoscalar) -> Bivector {
  let (a, b) = (lhs, rhs);
  let [e23, e31, e12] = [0f32; 3];

  let e01 = -a.e23*b.e0123;
  let e02 = -a.e31*b.e0123;
  let e03 = -a.e12*b.e0123;

  Bivector { e23, e31, e12, e01, e02, e03 }
}

// Trivector

#[rustfmt::skip]
#[inline]
fn trivector_dot_multivector(
  lhs: Trivector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = -a.e123*b.e123;
  let e0 = a.e123*b.e0123
         + a.e032*b.e23 + a.e013*b.e31 + a.e021*b.e12;
  let e1 = -a.e123*b.e23;
  let e2 = -a.e123*b.e31;
  let e3 = -a.e123*b.e12;
  let e01 = a.e013*b.e3 - a.e021*b.e2;
  let e02 = a.e021*b.e1 - a.e032*b.e3;
  let e03 = a.e032*b.e2 - a.e013*b.e1;
  let e23 = a.e123*b.e1;
  let e31 = a.e123*b.e2;
  let e12 = a.e123*b.e3;
  let e123 = a.e123*b.s;
  let e032 = a.e032*b.s;
  let e013 = a.e013*b.s;
  let e021 = a.e021*b.s;
  let e0123 = 0f32;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[inline]
fn trivector_dot_scalar(lhs: Trivector, rhs: Scalar) -> Trivector {
  TrivectorArray::from(lhs.to_trivector_array().elements.map(|e| e * rhs.s))
    .to_trivector()
}

#[rustfmt::skip]
#[inline]
fn trivector_dot_vector(lhs: Trivector, rhs: Vector) -> Bivector {
  let (a, b) = (lhs, rhs);

  let e01 = a.e013*b.e3 - a.e021*b.e2;
  let e02 = a.e021*b.e1 - a.e032*b.e3;
  let e03 = a.e032*b.e2 - a.e013*b.e1;
  let e23 = a.e123*b.e1;
  let e31 = a.e123*b.e2;
  let e12 = a.e123*b.e3;

  Bivector { e23, e31, e12, e01, e02, e03 }
}

#[rustfmt::skip]
#[inline]
fn trivector_dot_bivector(lhs: Trivector, rhs: Bivector) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e032*b.e23 + a.e013*b.e31 + a.e021*b.e12;
  let e1 = -a.e123*b.e23;
  let e2 = -a.e123*b.e31;
  let e3 = -a.e123*b.e12;

  Vector { e0, e1, e2, e3 }
}

#[inline]
fn trivector_dot_trivector(lhs: Trivector, rhs: Trivector) -> Scalar {
  let (a, b) = (lhs, rhs);
  let s = -a.e123 * b.e123;

  Scalar { s }
}

#[inline]
fn trivector_dot_pseudoscalar(lhs: Trivector, rhs: Pseudoscalar) -> Vector {
  let (a, b) = (lhs, rhs);
  let [e1, e2, e3] = [0f32; 3];

  let e0 = a.e123 * b.e0123;

  Vector { e0, e1, e2, e3 }
}

// Pseudoscalar

#[rustfmt::skip]
#[inline]
fn pseudoscalar_dot_multivector(
  lhs: Pseudoscalar,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e1, e2, e3, s, e23, e31, e12, e123] = [0f32; 8];

  let e0 = -a.e0123*b.e123;
  let e01 = -a.e0123*b.e23;
  let e02 = -a.e0123*b.e31;
  let e03 = -a.e0123*b.e12;
  let e032 = -a.e0123*b.e1;
  let e013 = -a.e0123*b.e2;
  let e021 = -a.e0123*b.e3;
  let e0123 = a.e0123*b.s;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[inline]
fn pseudoscalar_dot_scalar(lhs: Pseudoscalar, rhs: Scalar) -> Pseudoscalar {
  let (a, b) = (lhs, rhs);
  let e0123 = a.e0123 * b.s;

  Pseudoscalar { e0123 }
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_dot_vector(lhs: Pseudoscalar, rhs: Vector) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e123 = -a.e0123*0f32;
  let e032 = -a.e0123*b.e1;
  let e013 = -a.e0123*b.e2;
  let e021 = -a.e0123*b.e3;

  Trivector { e123, e032, e013, e021 }
}

#[rustfmt::skip]
#[inline]
fn pseudoscalar_dot_bivector(lhs: Pseudoscalar, rhs: Bivector) -> Bivector {
  let (a, b) = (lhs, rhs);
  let [e23, e31, e12] = [0f32; 3];

  let e01 = -a.e0123*b.e23;
  let e02 = -a.e0123*b.e31;
  let e03 = -a.e0123*b.e12;

  Bivector { e23, e31, e12, e01, e02, e03 }
}

#[inline]
fn pseudoscalar_dot_trivector(lhs: Pseudoscalar, rhs: Trivector) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = -a.e0123 * b.e123;
  let [e1, e2, e3] = [0f32; 3];

  Vector { e0, e1, e2, e3 }
}

#[inline]
fn pseudoscalar_dot_pseudoscalar(_: Pseudoscalar, _: Pseudoscalar) -> Empty {
  Empty
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
  const BIVECTOR_B: Bivector = Bivector {
    e23:  251., e31:  257., e12:  263.,
    e01:  269., e02:  271., e03:  277.,
  };
  const BIVECTOR_C: Bivector = Bivector {
    e23: -281., e31: -283., e12: -293.,
    e01: -307., e02: -311., e03: -313.,
  };
  const TRIVECTOR_A: Trivector = Trivector {
    e123:  317., e032:  331., e013:  337., e021:  347.
  };
  const TRIVECTOR_B: Trivector = Trivector {
    e123:  349., e032:  353., e013:  359., e021:  367.
  };
  const TRIVECTOR_C: Trivector = Trivector {
    e123: -373., e032: -379., e013: -383., e021: -389.
  };
  const PSEUDOSCALAR_A: Pseudoscalar = Pseudoscalar { e0123:  397. };
  const PSEUDOSCALAR_C: Pseudoscalar = Pseudoscalar { e0123: -409. };

  mod multivector {
    use super::*;
    #[test]
    fn dot_multivector_1() {
      let result = MULTIVECTOR_A.dot(MULTIVECTOR_B);
      let expected = Multivector {
          e0: 23311.,   e1: -3564.,   e2: -4676.,    e3: -4116.,
           s: -6780.,  e23:  4646.,  e31:  5446.,   e12:  6040.,
         e01: -1548.,  e02: -1880.,  e03: -1732., e0123:  3878.,
        e123:  4192., e032:  2446., e013:  2884.,  e021:  3432.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_multivector_2() {
      let result = MULTIVECTOR_A.dot(MULTIVECTOR_C);
      let expected = Multivector {
          e0: 15805.,   e1: -4876.,   e2:   3632.,    e3:  1244.,
           s:  2704.,  e23:  2094.,  e31:  -5446.,   e12:  1740.,
         e01: -1716.,  e02:    28.,  e03: -13524., e0123: -1524.,
        e123: -4192., e032: -3832., e013:  -1814.,  e021: -4306.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_multivector_3() {
      let result = MULTIVECTOR_A.dot(MULTIVECTOR_D);
      let expected = Multivector {
          e0: -23311.,   e1:  3564.,   e2:  4676.,    e3:  4116.,
           s:   6780.,  e23: -4646.,  e31: -5446.,   e12: -6040.,
         e01:   1548.,  e02:  1880.,  e03:  1732., e0123: -3878.,
        e123:  -4192., e032: -2446., e013: -2884.,  e021: -3432.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_scalar_1() {
      let result = MULTIVECTOR_A.dot(SCALAR_A);
      let expected = Multivector {
          e0:   274.,   e1:   411.,   e2:   685.,    e3:   959.,
           s:  1507.,  e23:  1781.,  e31:  2329.,   e12:  2603.,
         e01:  3151.,  e02:  3973.,  e03:  4247., e0123:  5069.,
        e123:  5617., e032:  5891., e013:  6439.,  e021:  7261.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_scalar_2() {
      let result = MULTIVECTOR_A.dot(SCALAR_C);
      let expected = Multivector {
          e0:  -298.,   e1:  -447.,   e2:  -745.,    e3: -1043.,
           s: -1639.,  e23: -1937.,  e31: -2533.,   e12: -2831.,
         e01: -3427.,  e02: -4321.,  e03: -4619., e0123: -5513.,
        e123: -6109., e032: -6407., e013: -7003.,  e021: -7897.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_vector_1() {
      let result = MULTIVECTOR_A.dot(VECTOR_A);
      let expected = Multivector {
          e0: 15176.,   e1:  1985.,   e2:   981.,    e3:   2387.,
           s:  2455.,  e23:  6437.,  e31:  6683.,   e12:   6847.,
         e01:  -790.,  e02:  1140.,  e03:  -370., e0123:      0.,
        e123:     0., e032: -5809., e013: -6031.,  e021:  -6179.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_vector_2() {
      let result = MULTIVECTOR_A.dot(VECTOR_C);
      let expected = Multivector {
          e0: -18966.,   e1: -2361.,   e2: -1189.,    e3:  -3083.,
           s:  -3063.,  e23: -8077.,  e31: -8159.,   e12:  -8651.,
         e01:    630.,  e02: -1368.,  e03:   702., e0123:      0.,
        e123:      0., e032:  7289., e013:  7363.,  e021:   7807.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_bivector_1() {
      let result = MULTIVECTOR_A.dot(BIVECTOR_A);
      let expected = Multivector {
          e0:  28814.,   e1: -8699.,   e2: -10181.,    e3: -8955.,
           s: -11109.,  e23:  2453.,  e31:   2497.,   e12:  2519.,
         e01:  -5688.,  e02: -5770.,  e03:  -5822.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_bivector_2() {
      let result = MULTIVECTOR_A.dot(BIVECTOR_C);
      let expected = Multivector {
          e0: -36246.,   e1: 11005.,   e2: 12691.,    e3: 11457.,
           s:  14031.,  e23: -3091.,  e31: -3113.,   e12: -3223.,
         e01:  7020.,  e02:   7050.,  e03:  7398.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_trivector_1() {
      let result = MULTIVECTOR_A.dot(TRIVECTOR_A);
      let expected = Multivector {
          e0:   4896.,   e1: -4121.,   e2: -5389.,    e3:  -6023.,
           s: -12997.,  e23:   951.,  e31:  1585.,   e12:   2219.,
         e01:    624.,  e02: -1276.,  e03:   644., e0123:      0.,
        e123:   3487., e032:  3641., e013:  3707.,  e021:   3817.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_trivector_2() {
      let result = MULTIVECTOR_A.dot(TRIVECTOR_C);
      let expected = Multivector {
          e0:  -5028.,   e1:  4849.,   e2:  6341.,    e3:   7087.,
           s:  15293.,  e23: -1119.,  e31: -1865.,   e12:  -2611.,
         e01:   -736.,  e02:  1486.,  e03:  -746., e0123:      0.,
        e123:  -4103., e032: -4169., e013: -4213.,  e021:  -4279.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_pseudoscalar_1() {
      let result = MULTIVECTOR_A.dot(PSEUDOSCALAR_A);
      let expected = Multivector {
          e0: 16277.,
         e01: -5161.,  e02: -6749.,  e03: -7543., e0123:  4367.,
        e032:  1191., e013:  1985., e021:  2779.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_pseudoscalar_2() {
      let result = MULTIVECTOR_A.dot(PSEUDOSCALAR_C);
      let expected = Multivector {
          e0: -16769.,
         e01:   5317.,  e02:  6953.,  e03:  7771., e0123:  -4499.,
        e032:  -1227., e013: -2045., e021: -2863.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod scalar {
    use super::*;
    #[test]
    fn dot_multivector_1() {
      let result = SCALAR_A.dot(MULTIVECTOR_A);
      let expected = Multivector {
          e0:  274.,   e1:  411.,   e2:  685.,    e3:  959.,
           s: 1507.,  e23: 1781.,  e31: 2329.,   e12: 2603.,
         e01: 3151.,  e02: 3973.,  e03: 4247., e0123: 5069.,
        e123: 5617., e032: 5891., e013: 6439.,  e021: 7261.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_scalar_1() {
      let result = SCALAR_A.dot(SCALAR_B);
      let expected = Scalar { s: 19043. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_scalar_2() {
      let result = SCALAR_A.dot(SCALAR_C);
      let expected = Scalar { s: -20413. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_vector_1() {
      let result = SCALAR_A.dot(VECTOR_A);
      let expected = Vector {
        e0: 20687., e1: 21509., e2: 22331., e3: 22879.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_bivector_1() {
      let result = SCALAR_A.dot(BIVECTOR_A);
      let expected = Bivector {
        e01: 31921., e02: 32743., e03: 33017.,
        e12: 31373., e31: 31099., e23: 30551.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_trivector_1() {
      let result = SCALAR_A.dot(TRIVECTOR_A);
      let expected = Trivector {
        e021: 47539., e013: 46169., e032: 45347., e123: 43429.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_pseudoscalar_1() {
      let result = SCALAR_A.dot(PSEUDOSCALAR_A);
      let expected = Pseudoscalar { e0123: 54389. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod vector {
    use super::*;
    #[test]
    fn dot_multivector_1() {
      let result = VECTOR_A.dot(MULTIVECTOR_A);
      let expected = Multivector {
          e0: -11854.,   e1:  1469.,   e2:  2605.,    e3: 1287.,
           s:   2455.,  e23:  6437.,  e31:  6683.,   e12: 6847.,
         e01:   -790.,  e02:  1140.,  e03:  -370., e0123:    0.,
        e123:      0., e032:  5809., e013:  6031.,  e021: 6179.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_scalar_1() {
      let result = VECTOR_A.dot(SCALAR_A);
      let expected = Vector {
        e0: 20687., e1: 21509., e2: 22331., e3: 22879.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_vector_1() {
      let result = VECTOR_A.dot(VECTOR_A);
      let expected = Scalar { s: 79107. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_vector_2() {
      let result = VECTOR_A.dot(VECTOR_B);
      let expected = Scalar { s: 89503. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_vector_3() {
      let result = VECTOR_A.dot(VECTOR_C);
      let expected = Scalar { s: -98603. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_bivector_1() {
      let result = VECTOR_A.dot(BIVECTOR_A);
      let expected = Vector { e0: -115785., e1: 582., e2: -1288., e3: 710. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_trivector_1() {
      let result = VECTOR_A.dot(TRIVECTOR_A);
      let expected = Bivector {
        e01: -282., e02: -798., e03: 1044.,
        e12: 52939., e31: 51671., e23: 49769.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_pseudoscalar() {
      let result = VECTOR_A.dot(PSEUDOSCALAR_A);
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
    fn dot_multivector_1() {
      let result = BIVECTOR_A.dot(MULTIVECTOR_A);
      let expected = Multivector {
          e0:  35976.,   e1: -9587.,   e2: -8433.,    e3: -9823.,
           s: -11109.,  e23:  2453.,  e31:  2497.,   e12:  2519.,
         e01:  -5688.,  e02: -5770.,  e03: -5822.,
         ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_scalar_1() {
      let result = BIVECTOR_A.dot(SCALAR_A);
      let expected = Bivector {
        e01: 31921., e02: 32743., e03: 33017.,
        e12: 31373., e31: 31099., e23: 30551.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_vector_1() {
      let result = BIVECTOR_A.dot(VECTOR_A);
      let expected = Vector {
        e0: 115785., e1: -582., e2: 1288., e3: -710.,
       };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_bivector_1() {
      let result = BIVECTOR_A.dot(BIVECTOR_A);
      let expected = Scalar { s: -153699. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_bivector_2() {
      let result = BIVECTOR_A.dot(BIVECTOR_B);
      let expected = Scalar { s: -174539. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_bivector_3() {
      let result = BIVECTOR_A.dot(BIVECTOR_C);
      let expected = Scalar { s: 194001. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_trivector_1() {
      let result = BIVECTOR_A.dot(TRIVECTOR_A);
      let expected = Vector {
        e0: 229775., e1: -70691., e2: -71959., e3: -72593.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_pseudoscalar() {
      let result = BIVECTOR_A.dot(PSEUDOSCALAR_A);
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
    fn dot_multivector_1() {
      let result = TRIVECTOR_A.dot(MULTIVECTOR_A);
      let expected = Multivector {
          e0:  28354.,   e1: -4121.,   e2: -5389.,    e3: -6023.,
           s: -12997.,  e23:   951.,  e31:  1585.,   e12:  2219.,
         e01:    624.,  e02: -1276.,  e03:   644., e0123:     0.,
        e123:   3487., e032:  3641., e013:  3707.,  e021:  3817.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_scalar_1() {
      let result = TRIVECTOR_A.dot(SCALAR_A);
      let expected = Trivector {
        e021: 47539., e013: 46169., e032: 45347., e123: 43429.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_vector_1() {
      let result = TRIVECTOR_A.dot(VECTOR_A);
      let expected = Bivector {
        e01: -282., e02: -798., e03: 1044.,
        e12: 52939., e31: 51671., e23: 49769.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_bivector_1() {
      let result = TRIVECTOR_A.dot(BIVECTOR_A);
      let expected = Vector {
        e0: 229775., e1: -70691., e2: -71959., e3: -72593.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_trivector_1() {
      let result = TRIVECTOR_A.dot(TRIVECTOR_A);
      let expected = Scalar { s: -100489. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_trivector_2() {
      let result = TRIVECTOR_A.dot(TRIVECTOR_B);
      let expected = Scalar { s: -110633. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_trivector_3() {
      let result = TRIVECTOR_A.dot(TRIVECTOR_C);
      let expected = Scalar { s: 118241. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_pseudoscalar() {
      let result = TRIVECTOR_A.dot(PSEUDOSCALAR_A);
      let expected = Vector { e0: 125849., ..zero() };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod pseudoscalar {
    use super::*;
    #[test]
    fn dot_multivector_1() {
      let result = PSEUDOSCALAR_A.dot(MULTIVECTOR_A);
      let expected = Multivector {
           e0: -16277.,
          e01:  -5161.,  e02: -6749.,  e03: -7543.,
         e032:  -1191., e013: -1985., e021: -2779.,
        e0123:   4367.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_scalar_1() {
      let result = PSEUDOSCALAR_A.dot(SCALAR_A);
      let expected = Pseudoscalar { e0123: 54389. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_vector_1() {
      let result = PSEUDOSCALAR_A.dot(VECTOR_A);
      let expected = Trivector {
        e032: -62329., e013: -64711., e021: -66299., e123: 0.
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_bivector_1() {
      let result = PSEUDOSCALAR_A.dot(BIVECTOR_A);
      let expected = Bivector {
        e01: -88531., e02: -90119., e03: -90913.,
        ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_trivector_1() {
      let result = PSEUDOSCALAR_A.dot(TRIVECTOR_A);
      let expected = Vector {
        e0: -125849., ..zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn dot_pseudoscalar_1() {
      let result = PSEUDOSCALAR_A.dot(PSEUDOSCALAR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }
}
