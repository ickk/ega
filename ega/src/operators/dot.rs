use super::return_empty;
use crate::values::*;

pub trait Dot<Rhs> {
  type Output;

  /// The inner product
  fn dot(&self, rhs: &Rhs) -> Self::Output;
}

macro_rules! impl_dot {
  ($dot_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl Dot<$rhs> for $lhs {
      type Output = $output;
      #[inline]
      fn dot(&self, rhs: &$rhs) -> Self::Output {
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
  lhs: &Multivector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.s*b.s - a.e123*b.e123
        + a.e1*b.e1 + a.e2*b.e2 + a.e3*b.e3
        - a.e23*b.e23 - a.e31*b.e31 - a.e12*b.e12;
  let e0 = a.s*b.e0 + a.e0*b.s
         + a.e123*b.e0123 - a.e0123*b.e123
         + a.e01*b.e1 + a.e02*b.e2 + a.e03*b.e3
         - a.e1*b.e01 - a.e2*b.e02 - a.e3*b.e03
         + a.e23*b.e032 + a.e31*b.e013 + a.e12*b.e021
         + a.e032*b.e23 + a.e013*b.e31 + a.e021*b.e12;
  let e1 = a.s*b.e1 + a.e1*b.s
         + a.e3*b.e31 + a.e12*b.e2
         - a.e2*b.e12 - a.e31*b.e3
         - a.e123*b.e23 - a.e23*b.e123;
  let e2 = a.s*b.e2 + a.e2*b.s
         + a.e1*b.e12 + a.e23*b.e3
         - a.e3*b.e23 - a.e12*b.e1
         - a.e123*b.e31 - a.e31*b.e123;
  let e3 = a.s*b.e3 + a.e3*b.s
         + a.e2*b.e23 + a.e31*b.e1
         - a.e1*b.e31 - a.e23*b.e2
         - a.e123*b.e12 - a.e12*b.e123;
  let e01 = a.s*b.e01 + a.e01*b.s
          + a.e3*b.e013 + a.e013*b.e3
          - a.e2*b.e021 - a.e021*b.e2
          - a.e0123*b.e23 - a.e23*b.e0123;
  let e02 = a.s*b.e02 + a.e02*b.s
          + a.e1*b.e021 + a.e021*b.e1
          - a.e3*b.e032 - a.e032*b.e3
          - a.e0123*b.e31 - a.e31*b.e0123;
  let e03 = a.s*b.e03 + a.e03*b.s
          + a.e2*b.e032 + a.e032*b.e2
          - a.e1*b.e013 - a.e013*b.e1
          - a.e0123*b.e12 - a.e12*b.e0123;
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
fn multivector_dot_scalar(lhs: &Multivector, rhs: &Scalar) -> Multivector {
  MultivectorArray::from(
    lhs.to_multivector_array().elements.map(|e| e * rhs.s),
  )
  .to_multivector()
}

#[rustfmt::skip]
#[inline]
fn multivector_dot_vector(lhs: &Multivector, rhs: &Vector) -> Multivector {
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
fn multivector_dot_bivector(lhs: &Multivector, rhs: &Bivector) -> Multivector {
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
  lhs: &Multivector,
  rhs: &Trivector,
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
  lhs: &Multivector,
  rhs: &Pseudoscalar,
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
fn scalar_dot_multivector(lhs: &Scalar, rhs: &Multivector) -> Multivector {
  MultivectorArray::from(
    rhs.to_multivector_array().elements.map(|e| lhs.s * e),
  )
  .to_multivector()
}

#[inline]
fn scalar_dot_scalar(lhs: &Scalar, rhs: &Scalar) -> Scalar {
  Scalar { s: lhs.s * rhs.s }
}

#[inline]
fn scalar_dot_vector(lhs: &Scalar, rhs: &Vector) -> Vector {
  VectorArray::from(rhs.to_vector_array().elements.map(|e| lhs.s * e))
    .to_vector()
}

#[inline]
fn scalar_dot_bivector(lhs: &Scalar, rhs: &Bivector) -> Bivector {
  BivectorArray::from(rhs.to_bivector_array().elements.map(|e| lhs.s * e))
    .to_bivector()
}

#[inline]
fn scalar_dot_trivector(lhs: &Scalar, rhs: &Trivector) -> Trivector {
  TrivectorArray::from(rhs.to_trivector_array().elements.map(|e| lhs.s * e))
    .to_trivector()
}

#[inline]
fn scalar_dot_pseudoscalar(lhs: &Scalar, rhs: &Pseudoscalar) -> Pseudoscalar {
  Pseudoscalar {
    e0123: lhs.s * rhs.e0123,
  }
}

// Vector

#[rustfmt::skip]
#[inline]
fn vector_dot_multivector(lhs: &Vector, rhs: &Multivector) -> Multivector {
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
fn vector_dot_scalar(lhs: &Vector, rhs: &Scalar) -> Vector {
  VectorArray::from(lhs.to_vector_array().elements.map(|e| e * rhs.s))
    .to_vector()
}

#[rustfmt::skip]
#[inline]
fn vector_dot_vector(lhs: &Vector, rhs: &Vector) -> Scalar {
  let (a, b) = (lhs, rhs);
  let s = a.e1*b.e1 + a.e2*b.e2 + a.e3*b.e3;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn vector_dot_bivector(lhs: &Vector, rhs: &Bivector) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = -a.e1*b.e01 - a.e2*b.e02 - a.e3*b.e03;
  let e1 = a.e3*b.e31 - a.e2*b.e12;
  let e2 = a.e1*b.e12 - a.e3*b.e23;
  let e3 = a.e2*b.e23 - a.e1*b.e31;

  Vector { e0, e1, e2, e3 }
}

#[rustfmt::skip]
#[inline]
fn vector_dot_trivector(lhs: &Vector, rhs: &Trivector) -> Bivector {
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
fn vector_dot_pseudoscalar(lhs: &Vector, rhs: &Pseudoscalar) -> Trivector {
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
fn bivector_dot_multivector(lhs: &Bivector, rhs: &Multivector) -> Multivector {
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
fn bivector_dot_scalar(lhs: &Bivector, rhs: &Scalar) -> Bivector {
  BivectorArray::from(lhs.to_bivector_array().elements.map(|e| e * rhs.s))
    .to_bivector()
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_vector(lhs: &Bivector, rhs: &Vector) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e01*b.e1 + a.e02*b.e2 + a.e03*b.e3;
  let e1 = a.e12*b.e2 - a.e31*b.e3;
  let e2 = a.e23*b.e3 - a.e12*b.e1;
  let e3 = a.e31*b.e1 - a.e23*b.e2;

  Vector { e0, e1, e2, e3 }
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_bivector(lhs: &Bivector, rhs: &Bivector) -> Scalar {
  let (a, b) = (lhs, rhs);
  let s = -a.e23*b.e23 - a.e31*b.e31 - a.e12*b.e12;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_trivector(lhs: &Bivector, rhs: &Trivector) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e23*b.e032 + a.e31*b.e013 + a.e12*b.e021;
  let e1 = -a.e23*b.e123;
  let e2 = -a.e31*b.e123;
  let e3 = -a.e12*b.e123;

  Vector { e0, e1, e2, e3 }
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_pseudoscalar(lhs: &Bivector, rhs: &Pseudoscalar) -> Bivector {
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
  lhs: &Trivector,
  rhs: &Multivector,
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
fn trivector_dot_scalar(lhs: &Trivector, rhs: &Scalar) -> Trivector {
  TrivectorArray::from(lhs.to_trivector_array().elements.map(|e| e * rhs.s))
    .to_trivector()
}

#[rustfmt::skip]
#[inline]
fn trivector_dot_vector(lhs: &Trivector, rhs: &Vector) -> Bivector {
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
fn trivector_dot_bivector(lhs: &Trivector, rhs: &Bivector) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e032*b.e23 + a.e013*b.e31 + a.e021*b.e12;
  let e1 = -a.e123*b.e23;
  let e2 = -a.e123*b.e31;
  let e3 = -a.e123*b.e12;

  Vector { e0, e1, e2, e3 }
}

#[inline]
fn trivector_dot_trivector(lhs: &Trivector, rhs: &Trivector) -> Scalar {
  let (a, b) = (lhs, rhs);
  let s = -a.e123 * b.e123;

  Scalar { s }
}

#[inline]
fn trivector_dot_pseudoscalar(lhs: &Trivector, rhs: &Pseudoscalar) -> Vector {
  let (a, b) = (lhs, rhs);
  let [e1, e2, e3] = [0f32; 3];

  let e0 = a.e123 * b.e0123;

  Vector { e0, e1, e2, e3 }
}

// Pseudoscalar

#[rustfmt::skip]
#[inline]
fn pseudoscalar_dot_multivector(
  lhs: &Pseudoscalar,
  rhs: &Multivector,
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
fn pseudoscalar_dot_scalar(lhs: &Pseudoscalar, rhs: &Scalar) -> Pseudoscalar {
  let (a, b) = (lhs, rhs);
  let e0123 = a.e0123 * b.s;

  Pseudoscalar { e0123 }
}

#[inline]
fn pseudoscalar_dot_vector(lhs: &Pseudoscalar, rhs: &Vector) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e123 = -a.e0123 * 0f32;
  let e032 = -a.e0123 * b.e1;
  let e013 = -a.e0123 * b.e2;
  let e021 = -a.e0123 * b.e3;

  Trivector { e123, e032, e013, e021 }
}

#[inline]
fn pseudoscalar_dot_bivector(lhs: &Pseudoscalar, rhs: &Bivector) -> Bivector {
  let (a, b) = (lhs, rhs);
  let [e23, e31, e12] = [0f32; 3];

  let e01 = -a.e0123 * b.e23;
  let e02 = -a.e0123 * b.e31;
  let e03 = -a.e0123 * b.e12;

  Bivector { e23, e31, e12, e01, e02, e03 }
}

#[inline]
fn pseudoscalar_dot_trivector(lhs: &Pseudoscalar, rhs: &Trivector) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = -a.e0123 * b.e123;
  let [e1, e2, e3] = [0f32; 3];

  Vector { e0, e1, e2, e3 }
}

#[inline]
fn pseudoscalar_dot_pseudoscalar(_: &Pseudoscalar, _: &Pseudoscalar) -> Empty {
  Empty
}

#[cfg(any(test, doctest))]
mod tests {
  use super::*;

  /// [
  ///       e0,    e1,    e2,    e3,
  ///   scalar,   e23,   e31,   e12,
  ///      e01,   e02,   e03, e0123,
  ///     e123,  e032,  e013,  e021,
  /// ]
  #[rustfmt::skip]
  #[test]
  fn multivector_dot_multivector() {
    {
      // [bivector.net evaluator](https://bivector.net/tools.html#seven)
      // (
      //      2e0   +  3e1   +  5e2   +  7e3
      //   + 11     + 13e23  + 17e31  + 19e12
      //   + 23e01  + 29e02  + 31e03  + 37e0123
      //   + 41e123 + 43e032 + 47e013 + 53e021
      // )
      // |
      // (
      //      59e0   +  61e1   + 67e2    + 71e3
      //   +  73     +  79e23  + 83e31   + 89e12
      //   +  97e01  + 101e02  + 103e03  + 107e0123
      //   + 109e123 + 113e032 + 127e013 + 131e021
      // )
      // =
      // [
      //   23311e0,   −3564e1,   −4676e2,  −4116e3,
      //   −6780,      4646e23,   5446e31,  6040e12,
      //   −1548e01,  −1880e02,  −1732e03,  3878e0123,
      //    4192e123,  2446e032,  2884e013, 3432e021,
      // ];
      let a = Multivector {
          e0:  2.,   e1:  3.,   e2:  5.,    e3:  7.,
           s: 11.,  e23: 13.,  e31: 17.,   e12: 19.,
         e01: 23.,  e02: 29.,  e03: 31., e0123: 37.,
        e123: 41., e032: 43., e013: 47.,  e021: 53.,
      };
      let b = Multivector {
          e0:  59.,   e1:  61.,   e2:  67.,    e3:  71.,
           s:  73.,  e23:  79.,  e31:  83.,   e12:  89.,
         e01:  97.,  e02: 101.,  e03: 103., e0123: 107.,
        e123: 109., e032: 113., e013: 127.,  e021: 131.,
      };
      let result = a.dot(&b);

      let expected = Multivector {
          e0: 23311.,   e1: -3564.,   e2: -4676.,    e3: -4116.,
           s: -6780.,  e23:  4646.,  e31:  5446.,   e12: 6040.,
         e01: -1548.,  e02: -1880.,  e03: -1732., e0123: 3878.,
        e123:  4192., e032:  2446., e013:  2884.,  e021: 3432.,
      };

      assert_eq!(dbg!(result), dbg!(expected));
    }

    {
      // [bivector.net evaluator](https://bivector.net/tools.html#seven)
      // (
      //      2e0   +  3e1   +  5e2   +  7e3
      //   + 11     + 13e23  + 17e31  + 19e12
      //   + 23e01  + 29e02  + 31e03  + 37e0123
      //   + 41e123 + 43e032 + 47e013 + 53e021
      // )
      // |
      // (
      //   -  59e0   +  61e1   - 67e2    + 71e3
      //   -  73     +  79e23  - 83e31   + 89e12
      //   -  97e01  + 101e02  - 103e03  + 107e0123
      //   - 109e123 + 113e032 - 127e013 + 131e021
      // )
      // =
      // [
      //   15805e0,   -4876e1,    3632e2,    1244e3,
      //    2704,      2094e23,  -5446e31,   1740e12,
      //   -1716e01,     28e02, -13524e03,  -1524e0123,
      //   -4192e123, -3832e032, -1814e013, -4306e021,
      // ];
      let a = Multivector {
          e0:  2.,   e1:  3.,   e2:  5.,    e3:  7.,
           s: 11.,  e23: 13.,  e31: 17.,   e12: 19.,
         e01: 23.,  e02: 29.,  e03: 31., e0123: 37.,
        e123: 41., e032: 43., e013: 47.,  e021: 53.,
      };
      let b = Multivector {
          e0:  -59.,   e1:  61.,   e2:  -67.,    e3:  71.,
           s:  -73.,  e23:  79.,  e31:  -83.,   e12:  89.,
         e01:  -97.,  e02: 101.,  e03: -103., e0123: 107.,
        e123: -109., e032: 113., e013: -127.,  e021: 131.,
      };
      let result = a.dot(&b);
      let expected = Multivector {
          e0: 15805.,   e1: -4876.,   e2:   3632.,    e3:  1244.,
           s:  2704.,  e23:  2094.,  e31:  -5446.,   e12:  1740.,
         e01: -1716.,  e02:    28.,  e03: -13524., e0123: -1524.,
        e123: -4192., e032: -3832., e013:  -1814.,  e021: -4306.,
      };

      assert_eq!(dbg!(result), dbg!(expected));
    }
  }
}
