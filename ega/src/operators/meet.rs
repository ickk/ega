use super::return_empty;
use crate::values::*;

pub trait Meet<Rhs> {
  type Output;

  /// The outer product
  fn meet(&self, rhs: &Rhs) -> Self::Output;
}

macro_rules! impl_meet {
  ($meet_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl Meet<$rhs> for $lhs {
      type Output = $output;
      #[inline]
      fn meet(&self, rhs: &$rhs) -> Self::Output {
        $meet_fn(self, rhs)
      }
    }
  };
}

impl_meet! { multivector_meet_multivector: Multivector, Multivector => Multivector }
impl_meet! { multivector_meet_scalar: Multivector, Scalar => Multivector }
impl_meet! { multivector_meet_vector: Multivector, Vector => Multivector }
impl_meet! { multivector_meet_bivector: Multivector, Bivector => Multivector }
impl_meet! { multivector_meet_trivector: Multivector, Trivector => Multivector }
impl_meet! { multivector_meet_pseudoscalar: Multivector, Pseudoscalar => Pseudoscalar }

impl_meet! { scalar_meet_multivector: Scalar, Multivector => Multivector }
impl_meet! { scalar_meet_scalar: Scalar, Scalar => Scalar }
impl_meet! { scalar_meet_vector: Scalar, Vector => Vector }
impl_meet! { scalar_meet_bivector: Scalar, Bivector => Bivector }
impl_meet! { scalar_meet_trivector: Scalar, Trivector => Trivector }
impl_meet! { scalar_meet_pseudoscalar: Scalar, Pseudoscalar => Pseudoscalar }

impl_meet! { vector_meet_multivector: Vector, Multivector => Multivector }
impl_meet! { vector_meet_scalar: Vector, Scalar => Vector }
impl_meet! { vector_meet_vector: Vector, Vector => Bivector }
impl_meet! { vector_meet_bivector: Vector, Bivector => Trivector }
impl_meet! { vector_meet_trivector: Vector, Trivector => Pseudoscalar }
impl_meet! { return_empty: Vector, Pseudoscalar => Empty }

impl_meet! { bivector_meet_multivector: Bivector, Multivector => Multivector }
impl_meet! { bivector_meet_scalar: Bivector, Scalar => Bivector }
impl_meet! { bivector_meet_vector: Bivector, Vector => Trivector }
impl_meet! { bivector_meet_bivector: Bivector, Bivector => Pseudoscalar }
impl_meet! { return_empty: Bivector, Trivector => Empty }
impl_meet! { return_empty: Bivector, Pseudoscalar => Empty }

impl_meet! { trivector_meet_multivector: Trivector, Multivector => Multivector }
impl_meet! { trivector_meet_scalar: Trivector, Scalar => Trivector }
impl_meet! { trivector_meet_vector: Trivector, Vector => Pseudoscalar }
impl_meet! { return_empty: Trivector, Bivector => Empty }
impl_meet! { return_empty: Trivector, Trivector => Empty }
impl_meet! { return_empty: Trivector, Pseudoscalar => Empty }

impl_meet! { pseudoscalar_meet_multivector: Pseudoscalar, Multivector => Pseudoscalar }
impl_meet! { pseudoscalar_meet_scalar: Pseudoscalar, Scalar => Pseudoscalar }
impl_meet! { return_empty: Pseudoscalar, Vector => Empty }
impl_meet! { return_empty: Pseudoscalar, Bivector => Empty }
impl_meet! { return_empty: Pseudoscalar, Trivector => Empty }
impl_meet! { return_empty: Pseudoscalar, Pseudoscalar => Empty }

impl_meet! { return_empty: Empty, Empty => Empty }
impl_meet! { return_empty: Multivector, Empty => Empty }
impl_meet! { return_empty: Scalar, Empty => Empty }
impl_meet! { return_empty: Vector, Empty => Empty }
impl_meet! { return_empty: Bivector, Empty => Empty }
impl_meet! { return_empty: Trivector, Empty => Empty }
impl_meet! { return_empty: Pseudoscalar, Empty => Empty }
impl_meet! { return_empty: Empty, Multivector => Empty }
impl_meet! { return_empty: Empty, Scalar => Empty }
impl_meet! { return_empty: Empty, Vector => Empty }
impl_meet! { return_empty: Empty, Bivector => Empty }
impl_meet! { return_empty: Empty, Trivector => Empty }
impl_meet! { return_empty: Empty, Pseudoscalar => Empty }

// Multivector

#[rustfmt::skip]
#[inline]
fn multivector_meet_multivector(
  lhs: &Multivector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.s*b.s;
  let e0 = a.s*b.e0 + a.e0*b.s;
  let e1 = a.s*b.e1 + a.e1*b.s;
  let e2 = a.s*b.e2 + a.e2*b.s;
  let e3 = a.s*b.e3 + a.e3*b.s;
  let e23 = a.s*b.e23 + a.e2*b.e3
          + a.e23*b.s - a.e3*b.e2;
  let e31 = a.s*b.e31 - a.e1*b.e3
          + a.e31*b.s + a.e3*b.e1;
  let e12 = a.s*b.e12 + a.e1*b.e2
          + a.e12*b.s - a.e2*b.e1;
  let e01 = a.s*b.e01 + a.e0*b.e1
          + a.e01*b.s - a.e1*b.e0;
  let e02 = a.s*b.e02 + a.e0*b.e2
          + a.e02*b.s - a.e2*b.e0;
  let e03 = a.s*b.e03 + a.e0*b.e3
          + a.e03*b.s - a.e3*b.e0;
  let e123 = a.s*b.e123 + a.e1*b.e23 + a.e2*b.e31 + a.e3*b.e12
           + a.e123*b.s + a.e23*b.e1 + a.e31*b.e2 + a.e12*b.e3;
  let e032 = a.s*b.e032 - a.e0*b.e23 + a.e2*b.e03 - a.e3*b.e02
           + a.e032*b.s - a.e23*b.e0 - a.e02*b.e3 + a.e03*b.e2;
  let e013 = a.s*b.e013 - a.e0*b.e31 - a.e1*b.e03 + a.e3*b.e01
           + a.e013*b.s - a.e31*b.e0 + a.e01*b.e3 - a.e03*b.e1;
  let e021 = a.s*b.e021 - a.e0*b.e12 + a.e1*b.e02 - a.e2*b.e01
           + a.e021*b.s - a.e12*b.e0 - a.e01*b.e2 + a.e02*b.e1;
  let e0123 = a.e0123*b.s + a.s*b.e0123
            + a.e0*b.e123 + a.e1*b.e032 + a.e2*b.e013 + a.e3*b.e021
            - a.e123*b.e0 - a.e032*b.e1 - a.e013*b.e2 - a.e021*b.e3
            + a.e23*b.e01 + a.e31*b.e02 + a.e12*b.e03
            + a.e01*b.e23 + a.e02*b.e31 + a.e03*b.e12;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[inline]
fn multivector_meet_scalar(lhs: &Multivector, rhs: &Scalar) -> Multivector {
  let elements = lhs.to_multivector_array().elements.map(|e| e * rhs.s);
  MultivectorArray::from(elements).to_multivector()
}

#[rustfmt::skip]
#[inline]
fn multivector_meet_vector(lhs: &Multivector, rhs: &Vector) -> Multivector {
  let (a, b) = (lhs, rhs);
  let s = 0f32;

  let e0 = a.s*b.e0;
  let e1 = a.s*b.e1;
  let e2 = a.s*b.e2;
  let e3 = a.s*b.e3;
  let e23 = a.e2*b.e3 - a.e3*b.e2;
  let e31 = a.e3*b.e1 - a.e1*b.e3;
  let e12 = a.e1*b.e2 - a.e2*b.e1;
  let e01 = a.e0*b.e1 - a.e1*b.e0;
  let e02 = a.e0*b.e2 - a.e2*b.e0;
  let e03  = a.e0*b.e3 - a.e3*b.e0;
  let e123 = a.e23*b.e1 + a.e31*b.e2 + a.e12*b.e3;
  let e032 = a.e03*b.e2 - a.e23*b.e0 - a.e02*b.e3;
  let e013 = a.e01*b.e3 - a.e31*b.e0 - a.e03*b.e1;
  let e021 = a.e02*b.e1 - a.e12*b.e0 - a.e01*b.e2;
  let e0123 = -a.e123*b.e0 - a.e032*b.e1 - a.e013*b.e2 - a.e021*b.e3;

  Multivector {
    e0,    e1,    e2,    e3,
     s,   e23,   e31,   e12,
   e01,   e02,   e03, e0123,
  e123,  e032,  e013,  e021,
}
}

#[rustfmt::skip]
#[inline]
fn multivector_meet_bivector(
  lhs: &Multivector,
  rhs: &Bivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e0, e1, e2, e3, s] = [0f32; 5];

  let e23 = a.s*b.e23;
  let e31 = a.s*b.e31;
  let e12 = a.s*b.e12;
  let e01 = a.s*b.e01;
  let e02 = a.s*b.e02;
  let e03 = a.s*b.e03;
  let e123 = a.e1*b.e23 + a.e2*b.e31 + a.e3*b.e12;
  let e032 = a.e2*b.e03 - a.e0*b.e23 - a.e3*b.e02;
  let e013 = a.e3*b.e01 - a.e0*b.e31 - a.e1*b.e03;
  let e021 = a.e1*b.e02 - a.e0*b.e12 - a.e2*b.e01;
  let e0123 = a.e23*b.e01 + a.e31*b.e02 + a.e12*b.e03
            + a.e01*b.e23 + a.e02*b.e31 + a.e03*b.e12;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_meet_trivector(
  lhs: &Multivector,
  rhs: &Trivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e0, e1, e2, e3, s, e23, e31, e12, e01, e02, e03] = [0f32; 11];

  let e123 = a.s*b.e123;
  let e032 = a.s*b.e032;
  let e013 = a.s*b.e013;
  let e021 = a.s*b.e021;
  let e0123 = a.e0*b.e123 + a.e1*b.e032 + a.e2*b.e013 + a.e3*b.e021;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[inline]
fn multivector_meet_pseudoscalar(
  lhs: &Multivector,
  rhs: &Pseudoscalar,
) -> Pseudoscalar {
  Pseudoscalar {
    e0123: lhs.s * rhs.e0123,
  }
}

// Scalar

#[inline]
fn scalar_meet_scalar(lhs: &Scalar, rhs: &Scalar) -> Scalar {
  Scalar { s: lhs.s * rhs.s }
}

#[inline]
fn scalar_meet_vector(lhs: &Scalar, rhs: &Vector) -> Vector {
  let elements = rhs.to_vector_array().elements.map(|e| lhs.s * e);
  VectorArray::from(elements).to_vector()
}

#[inline]
fn scalar_meet_bivector(lhs: &Scalar, rhs: &Bivector) -> Bivector {
  let elements = rhs.to_bivector_array().elements.map(|e| lhs.s * e);
  BivectorArray::from(elements).to_bivector()
}

#[inline]
fn scalar_meet_trivector(lhs: &Scalar, rhs: &Trivector) -> Trivector {
  let elements = rhs.to_trivector_array().elements.map(|e| lhs.s * e);
  TrivectorArray::from(elements).to_trivector()
}

#[inline]
fn scalar_meet_pseudoscalar(lhs: &Scalar, rhs: &Pseudoscalar) -> Pseudoscalar {
  Pseudoscalar::from(lhs.s * rhs.e0123)
}

#[inline]
fn scalar_meet_multivector(lhs: &Scalar, rhs: &Multivector) -> Multivector {
  let elements = rhs.to_multivector_array().elements.map(|e| lhs.s * e);
  MultivectorArray::from(elements).to_multivector()
}

// Vector

#[inline]
fn vector_meet_scalar(lhs: &Vector, rhs: &Scalar) -> Vector {
  let elements = lhs.to_vector_array().elements.map(|e| rhs.s * e);
  VectorArray::from(elements).to_vector()
}

#[rustfmt::skip]
#[inline]
fn vector_meet_vector(lhs: &Vector, rhs: &Vector) -> Bivector {
  let (p, q) = (lhs, rhs);

  let e23 = p.e2*q.e3 - p.e3*q.e2;
  let e31 = p.e3*q.e1 - p.e1*q.e3;
  let e12 = p.e1*q.e2 - p.e2*q.e1;
  let e01 = p.e0*q.e1 - p.e1*q.e0;
  let e02 = p.e0*q.e2 - p.e2*q.e0;
  let e03 = p.e0*q.e3 - p.e3*q.e0;

  Bivector { e23, e31, e12, e01, e02, e03 }
}

#[rustfmt::skip]
#[inline]
fn vector_meet_bivector(lhs: &Vector, rhs: &Bivector) -> Trivector {
  let (p, l) = (lhs, rhs);

  let e123 = p.e1*l.e23 + p.e2*l.e31 + p.e3*l.e12;
  let e032 = p.e2*l.e03 - p.e0*l.e23 - p.e3*l.e02;
  let e013 = p.e3*l.e01 - p.e0*l.e31 - p.e1*l.e03;
  let e021 = p.e1*l.e02 - p.e0*l.e12 - p.e2*l.e01;

  Trivector { e123, e032, e013, e021 }
}

#[rustfmt::skip]
#[inline]
fn vector_meet_trivector(lhs: &Vector, rhs: &Trivector) -> Pseudoscalar {
  let (p, x) = (lhs, rhs);
  let e0123 = p.e0*x.e123 + p.e1*x.e032 + p.e2*x.e013 + p.e3*x.e021;

  Pseudoscalar { e0123 }
}

#[rustfmt::skip]
#[inline]
fn vector_meet_multivector(lhs: &Vector, rhs: &Multivector) -> Multivector {
  let (a, b) = (lhs, rhs);
  let s = 0f32;

  let e0 = a.e0*b.s;
  let e1 = a.e1*b.s;
  let e2 = a.e2*b.s;
  let e3 = a.e3*b.s;
  let e23 = a.e2*b.e3 - a.e3*b.e2;
  let e31 = a.e3*b.e1 - a.e1*b.e3;
  let e12 = a.e1*b.e2 - a.e2*b.e1;
  let e01 = a.e0*b.e1 - a.e1*b.e0;
  let e02 = a.e0*b.e2 - a.e2*b.e0;
  let e03 = a.e0*b.e3 - a.e3*b.e0;
  let e123 = a.e1*b.e23 + a.e2*b.e31 + a.e3*b.e12;
  let e032 = a.e2*b.e03 - a.e0*b.e23 - a.e3*b.e02;
  let e013 = a.e3*b.e01 - a.e0*b.e31 - a.e1*b.e03;
  let e021 = a.e1*b.e02 - a.e0*b.e12 - a.e2*b.e01;
  let e0123 = a.e0*b.e123 + a.e1*b.e032 + a.e2*b.e013 + a.e3*b.e021;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

// Bivector

#[inline]
fn bivector_meet_scalar(lhs: &Bivector, rhs: &Scalar) -> Bivector {
  let elements = lhs.to_bivector_array().elements.map(|e| rhs.s * e);
  BivectorArray::from(elements).to_bivector()
}

#[rustfmt::skip]
#[inline]
fn bivector_meet_vector(lhs: &Bivector, rhs: &Vector) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e123 = a.e23*b.e1 + a.e31*b.e2 + a.e12*b.e3;
  let e032 = a.e03*b.e2 - a.e23*b.e0 - a.e02*b.e3;
  let e013 = a.e01*b.e3 - a.e31*b.e0 - a.e03*b.e1;
  let e021 = a.e02*b.e1 - a.e12*b.e0 - a.e01*b.e2;

  Trivector { e123, e032, e013, e021}
}

#[rustfmt::skip]
#[inline]
fn bivector_meet_bivector(lhs: &Bivector, rhs: &Bivector) -> Pseudoscalar {
  let (l, m) = (lhs, rhs);
  let e0123 = l.e01*m.e23 + l.e02*m.e31 + l.e03*m.e12
            + l.e23*m.e01 + l.e31*m.e02 + l.e12*m.e03;

  Pseudoscalar { e0123 }
}

#[rustfmt::skip]
#[inline]
fn bivector_meet_multivector(
  lhs: &Bivector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e0, e1, e2, e3, s] = [0f32; 5];

  let e23 = a.e23*b.s;
  let e31 = a.e31*b.s;
  let e12 = a.e12*b.s;
  let e01 = a.e01*b.s;
  let e02 = a.e02*b.s;
  let e03 = a.e03*b.s;
  let e123 = a.e23*b.e1 + a.e31*b.e2 + a.e12*b.e3;
  let e032 = a.e03*b.e2 - a.e23*b.e0 - a.e02*b.e3;
  let e013 = a.e01*b.e3 - a.e31*b.e0 - a.e03*b.e1;
  let e021 = a.e02*b.e1 - a.e12*b.e0 - a.e01*b.e2;
  let e0123 = a.e23*b.e01 + a.e31*b.e02 + a.e12*b.e03
            + a.e01*b.e23 + a.e02*b.e31 + a.e03*b.e12;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

// Trivector

#[inline]
fn trivector_meet_scalar(lhs: &Trivector, rhs: &Scalar) -> Trivector {
  let elements = lhs.to_trivector_array().elements.map(|e| rhs.s * e);
  TrivectorArray::from(elements).to_trivector()
}

#[rustfmt::skip]
#[inline]
fn trivector_meet_multivector(
  lhs: &Trivector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e0, e1, e2, e3, s, e23, e31, e12, e01, e02, e03] = [0f32; 11];

  let e123 = a.e123*b.s;
  let e032 = a.e032*b.s;
  let e013 = a.e013*b.s;
  let e021 = a.e021*b.s;
  let e0123 = -a.e123*b.e0 - a.e032*b.e1 - a.e013*b.e2 - a.e021*b.e3;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_meet_vector(lhs: &Trivector, rhs: &Vector) -> Pseudoscalar {
  let (a, b) = (lhs, rhs);
  let e0123 = -a.e123*b.e0 - a.e032*b.e1 - a.e013*b.e2 - a.e021*b.e3;

  Pseudoscalar { e0123 }
}

// Pseudoscalar

#[inline]
fn pseudoscalar_meet_multivector(
  lhs: &Pseudoscalar,
  rhs: &Multivector,
) -> Pseudoscalar {
  Pseudoscalar { e0123: lhs.e0123 * rhs.s }
}

#[inline]
fn pseudoscalar_meet_scalar(lhs: &Pseudoscalar, rhs: &Scalar) -> Pseudoscalar {
  Pseudoscalar { e0123: lhs.e0123 * rhs.s }
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
  fn multivector_meet_multivector() {
    {
      // [bivector.net evaluator](https://bivector.net/tools.html#seven)
      // (
      //      2e0   +  3e1   +  5e2   +  7e3
      //   + 11     + 13e23  + 17e31  + 19e12
      //   + 23e01  + 29e02  + 31e03  + 37e0123
      //   + 41e123 + 43e032 + 47e013 + 53e021
      // )
      // ^
      // (
      //      59e0   +  61e1   + 67e2    + 71e3
      //   +  73     +  79e23  + 83e31   + 89e12
      //   +  97e01  + 101e02  + 103e03  + 107e0123
      //   + 109e123 + 113e032 + 127e013 + 131e021
      // )
      // =
      // [
      //    795e0,    890e1,   1102e2,   1292e3,
      //    803,     1704e23,  2368e31,  2262e12,
      //   2692e01,  3067e02,  3125e03,  5951e0123,
      //   8748e123, 3283e032, 3771e013, 4057e021,
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
      let result = a.meet(&b);
      let expected = Multivector {
          e0:  795.,   e1:  890.,   e2: 1102.,    e3: 1292.,
           s:  803.,  e23: 1704.,  e31: 2368.,   e12: 2262.,
         e01: 2691.,  e02: 3067.,  e03: 3125., e0123: 5951.,
        e123: 8748., e032: 3283., e013: 3771.,  e021: 4057.,
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
      // ^
      // (
      //   -  59e0   +  61e1   - 67e2    + 71e3
      //   -  73     +  79e23  - 83e31   + 89e12
      //   -  97e01  + 101e02  - 103e03  + 107e0123
      //   - 109e123 + 113e032 - 127e013 + 131e021
      // )
      // =
      // [
      //    -795e0,     452e1,   -1102e2,    270e3,
      //   -803,        744e23,  -1940e31,  -914e12,
      //   -2447e01,   -845e02,  -2841e03, -1271e0123,
      //   -2744e123, -6645e032, -4287e013, 2613e021,
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
      let result = a.meet(&b);
      let expected = Multivector {
          e0:  -795.,   e1:   452.,   e2: -1102.,    e3:   270.,
           s:  -803.,  e23:   744.,  e31: -1940.,   e12:  -914.,
         e01: -2447.,  e02:  -845.,  e03: -2841., e0123: -1271.,
        e123: -2744., e032: -6645., e013: -4287.,  e021:  2613.,
      };

      assert_eq!(dbg!(result), dbg!(expected));
    }
  }
}
