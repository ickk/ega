use super::return_empty;
use crate::*;

/// The outer product
pub trait Meet<Rhs> {
  type Output;

  /// The outer product
  fn meet(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_meet {
  ($meet_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl Meet<$rhs> for $lhs {
      type Output = $output;

      #[inline]
      fn meet(self, rhs: $rhs) -> Self::Output {
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
  lhs: Multivector,
  rhs: Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let s = a.s*b.s;
  let e0 = a.s*b.e0 + a.e0*b.s;
  let e1 = a.s*b.e1 + a.e1*b.s;
  let e2 = a.s*b.e2 + a.e2*b.s;
  let e3 = a.s*b.e3 + a.e3*b.s;
  let e23 = a.s*b.e23 + a.e23*b.s
          + a.e2*b.e3 - a.e3*b.e2;
  let e31 = a.s*b.e31 + a.e31*b.s
          - a.e1*b.e3 + a.e3*b.e1;
  let e12 = a.s*b.e12 + a.e12*b.s
          + a.e1*b.e2 - a.e2*b.e1;
  let e01 = a.s*b.e01 + a.e0*b.e1
          + a.e01*b.s - a.e1*b.e0;
  let e02 = a.s*b.e02 + a.e0*b.e2
          + a.e02*b.s - a.e2*b.e0;
  let e03 = a.s*b.e03 + a.e0*b.e3
          + a.e03*b.s - a.e3*b.e0;
  let e123 = a.s*b.e123 + a.e123*b.s
           + a.e1*b.e23 + a.e23*b.e1
           + a.e2*b.e31 + a.e31*b.e2
           + a.e3*b.e12 + a.e12*b.e3;
  let e032 = a.s*b.e032 + a.e032*b.s
           - a.e0*b.e23 - a.e23*b.e0
           + a.e2*b.e03 + a.e03*b.e2
           - a.e3*b.e02 - a.e02*b.e3;
  let e013 = a.s*b.e013 + a.e013*b.s
           - a.e0*b.e31 - a.e31*b.e0
           - a.e1*b.e03 - a.e03*b.e1
           + a.e3*b.e01 + a.e01*b.e3;
  let e021 = a.s*b.e021 + a.e021*b.s
           - a.e0*b.e12 - a.e12*b.e0
           + a.e1*b.e02 + a.e02*b.e1
           - a.e2*b.e01 - a.e01*b.e2;
  let e0123 = a.e0123*b.s + a.s*b.e0123
            + a.e0*b.e123 - a.e123*b.e0
            + a.e1*b.e032 - a.e032*b.e1
            + a.e2*b.e013 - a.e013*b.e2
            + a.e3*b.e021 - a.e021*b.e3
            + a.e23*b.e01 + a.e01*b.e23
            + a.e31*b.e02 + a.e02*b.e31
            + a.e12*b.e03 + a.e03*b.e12;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

#[inline]
fn multivector_meet_scalar(lhs: Multivector, rhs: Scalar) -> Multivector {
  let elements = lhs.to_multivector_array().elements.map(|e| e * rhs.s);
  MultivectorArray::from(elements).to_multivector()
}

#[rustfmt::skip]
#[inline]
fn multivector_meet_vector(lhs: Multivector, rhs: Vector) -> Multivector {
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
  let e123 = a.e23*b.e1
           + a.e31*b.e2
           + a.e12*b.e3;
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
  lhs: Multivector,
  rhs: Bivector,
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
  let e0123 = a.e23*b.e01 + a.e01*b.e23
            + a.e31*b.e02 + a.e02*b.e31
            + a.e12*b.e03 + a.e03*b.e12;

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
  lhs: Multivector,
  rhs: Trivector,
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
  lhs: Multivector,
  rhs: Pseudoscalar,
) -> Pseudoscalar {
  Pseudoscalar {
    e0123: lhs.s * rhs.e0123,
  }
}

// Scalar

#[inline]
fn scalar_meet_scalar(lhs: Scalar, rhs: Scalar) -> Scalar {
  Scalar { s: lhs.s * rhs.s }
}

#[inline]
fn scalar_meet_vector(lhs: Scalar, rhs: Vector) -> Vector {
  let elements = rhs.to_vector_array().elements.map(|e| lhs.s * e);
  VectorArray::from(elements).to_vector()
}

#[inline]
fn scalar_meet_bivector(lhs: Scalar, rhs: Bivector) -> Bivector {
  let elements = rhs.to_bivector_array().elements.map(|e| lhs.s * e);
  BivectorArray::from(elements).to_bivector()
}

#[inline]
fn scalar_meet_trivector(lhs: Scalar, rhs: Trivector) -> Trivector {
  let elements = rhs.to_trivector_array().elements.map(|e| lhs.s * e);
  TrivectorArray::from(elements).to_trivector()
}

#[inline]
fn scalar_meet_pseudoscalar(lhs: Scalar, rhs: Pseudoscalar) -> Pseudoscalar {
  Pseudoscalar::from(lhs.s * rhs.e0123)
}

#[inline]
fn scalar_meet_multivector(lhs: Scalar, rhs: Multivector) -> Multivector {
  let elements = rhs.to_multivector_array().elements.map(|e| lhs.s * e);
  MultivectorArray::from(elements).to_multivector()
}

// Vector

#[inline]
fn vector_meet_scalar(lhs: Vector, rhs: Scalar) -> Vector {
  let elements = lhs.to_vector_array().elements.map(|e| rhs.s * e);
  VectorArray::from(elements).to_vector()
}

#[rustfmt::skip]
#[inline]
fn vector_meet_vector(lhs: Vector, rhs: Vector) -> Bivector {
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
fn vector_meet_bivector(lhs: Vector, rhs: Bivector) -> Trivector {
  let (p, l) = (lhs, rhs);

  let e123 = p.e1*l.e23 + p.e2*l.e31 + p.e3*l.e12;
  let e032 = p.e2*l.e03 - p.e0*l.e23 - p.e3*l.e02;
  let e013 = p.e3*l.e01 - p.e0*l.e31 - p.e1*l.e03;
  let e021 = p.e1*l.e02 - p.e0*l.e12 - p.e2*l.e01;

  Trivector { e123, e032, e013, e021 }
}

#[rustfmt::skip]
#[inline]
fn vector_meet_trivector(lhs: Vector, rhs: Trivector) -> Pseudoscalar {
  let (p, x) = (lhs, rhs);
  let e0123 = p.e0*x.e123 + p.e1*x.e032 + p.e2*x.e013 + p.e3*x.e021;

  Pseudoscalar { e0123 }
}

#[rustfmt::skip]
#[inline]
fn vector_meet_multivector(lhs: Vector, rhs: Multivector) -> Multivector {
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
fn bivector_meet_scalar(lhs: Bivector, rhs: Scalar) -> Bivector {
  let elements = lhs.to_bivector_array().elements.map(|e| rhs.s * e);
  BivectorArray::from(elements).to_bivector()
}

#[rustfmt::skip]
#[inline]
fn bivector_meet_vector(lhs: Bivector, rhs: Vector) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e123 = a.e23*b.e1 + a.e31*b.e2 + a.e12*b.e3;
  let e032 = a.e03*b.e2 - a.e23*b.e0 - a.e02*b.e3;
  let e013 = a.e01*b.e3 - a.e31*b.e0 - a.e03*b.e1;
  let e021 = a.e02*b.e1 - a.e12*b.e0 - a.e01*b.e2;

  Trivector { e123, e032, e013, e021}
}

#[rustfmt::skip]
#[inline]
fn bivector_meet_bivector(lhs: Bivector, rhs: Bivector) -> Pseudoscalar {
  let (l, m) = (lhs, rhs);
  let e0123 = l.e01*m.e23 + l.e23*m.e01
            + l.e02*m.e31 + l.e31*m.e02
            + l.e03*m.e12 + l.e12*m.e03;

  Pseudoscalar { e0123 }
}

#[rustfmt::skip]
#[inline]
fn bivector_meet_multivector(
  lhs: Bivector,
  rhs: Multivector,
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
  let e0123 = a.e23*b.e01 + a.e01*b.e23
            + a.e02*b.e31 + a.e31*b.e02
            + a.e03*b.e12 + a.e12*b.e03;

  Multivector {
      e0,    e1,    e2,    e3,
       s,   e23,   e31,   e12,
     e01,   e02,   e03, e0123,
    e123,  e032,  e013,  e021,
  }
}

// Trivector

#[inline]
fn trivector_meet_scalar(lhs: Trivector, rhs: Scalar) -> Trivector {
  let elements = lhs.to_trivector_array().elements.map(|e| rhs.s * e);
  TrivectorArray::from(elements).to_trivector()
}

#[rustfmt::skip]
#[inline]
fn trivector_meet_multivector(
  lhs: Trivector,
  rhs: Multivector,
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
fn trivector_meet_vector(lhs: Trivector, rhs: Vector) -> Pseudoscalar {
  let (a, b) = (lhs, rhs);
  let e0123 = -a.e123*b.e0 - a.e032*b.e1 - a.e013*b.e2 - a.e021*b.e3;

  Pseudoscalar { e0123 }
}

// Pseudoscalar

#[inline]
fn pseudoscalar_meet_multivector(
  lhs: Pseudoscalar,
  rhs: Multivector,
) -> Pseudoscalar {
  Pseudoscalar {
    e0123: lhs.e0123 * rhs.s,
  }
}

#[inline]
fn pseudoscalar_meet_scalar(lhs: Pseudoscalar, rhs: Scalar) -> Pseudoscalar {
  Pseudoscalar {
    e0123: lhs.e0123 * rhs.s,
  }
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
  const TRIVECTOR_C: Trivector = Trivector {
    e123: -373., e032: -379., e013: -383., e021: -389.
  };
  const PSEUDOSCALAR_A: Pseudoscalar = Pseudoscalar { e0123:  397. };
  const PSEUDOSCALAR_C: Pseudoscalar = Pseudoscalar { e0123: -409. };

  mod multivector {
    use super::*;
    #[test]
    fn meet_multivector_1() {
      let result = MULTIVECTOR_A.meet(MULTIVECTOR_B);
      let expected = Multivector {
          e0:  795.,   e1:  890.,   e2: 1102.,    e3: 1292.,
           s:  803.,  e23: 1704.,  e31: 2368.,   e12: 2262.,
         e01: 2691.,  e02: 3067.,  e03: 3125., e0123: 5951.,
        e123: 8748., e032: 3283., e013: 3771.,  e021: 4057.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_multivector_2() {
      let result = MULTIVECTOR_A.meet(MULTIVECTOR_C);
      let expected = Multivector {
          e0:  -795.,   e1:   452.,   e2: -1102.,    e3:   270.,
           s:  -803.,  e23:   744.,  e31: -1940.,   e12:  -914.,
         e01: -2447.,  e02:  -845.,  e03: -2841., e0123: -1271.,
        e123: -2744., e032: -6645., e013: -4287.,  e021:  2613.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_multivector_3() {
      let result = MULTIVECTOR_A.meet(MULTIVECTOR_D);
      let expected = Multivector {
          e0:  -795.,   e1:  -890.,   e2: -1102.,    e3: -1292.,
           s:  -803.,  e23: -1704.,  e31: -2368.,   e12: -2262.,
         e01: -2691.,  e02: -3067.,  e03: -3125., e0123: -5951.,
        e123: -8748., e032: -3283., e013: -3771.,  e021: -4057.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_1() {
      let result = MULTIVECTOR_A.meet(SCALAR_A);
      let expected = Multivector {
          e0:   274.,   e1:   411.,   e2:   685.,    e3:   959.,
           s:  1507.,  e23:  1781.,  e31:  2329.,   e12:  2603.,
         e01:  3151.,  e02:  3973.,  e03:  4247., e0123:  5069.,
        e123:  5617., e032:  5891., e013:  6439.,  e021:  7261.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_2() {
      let result = MULTIVECTOR_A.meet(SCALAR_C);
      let expected = Multivector {
          e0:  -298.,   e1:  -447.,   e2:  -745.,    e3: -1043.,
           s: -1639.,  e23: -1937.,  e31: -2533.,   e12: -2831.,
         e01: -3427.,  e02: -4321.,  e03: -4619., e0123: -5513.,
        e123: -6109., e032: -6407., e013: -7003.,  e021: -7897.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_1() {
      let result = MULTIVECTOR_A.meet(VECTOR_A);
      let expected = Multivector {
          e0:  1661.,   e1:  1727.,   e2:  1793.,    e3:   1837.,
           s:     0.,  e23:  -306.,  e31:   598.,   e12:   -296.,
         e01:  -139.,  e02:  -429.,  e03:  -723., e0123: -29454.,
        e123:  7985., e032: -1753., e013: -3593.,  e021:  -2065.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_2() {
      let result = MULTIVECTOR_A.meet(VECTOR_C);
      let expected = Multivector {
          e0: -2123.,   e1: -2167.,   e2: -2189.,    e3:  -2321.,
           s:     0.,  e23:   338.,  e31:  -746.,   e12:    388.,
         e01:   185.,  e02:   567.,  e03:   929., e0123:  36920.,
        e123: -9953., e032:  2459., e013:  4535.,  e021:   2531.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector_1() {
      let result = MULTIVECTOR_A.meet(BIVECTOR_A);
      let expected = Multivector {
           s:     0.,  e23:  2453.,  e31:  2497.,   e12:   2519.,
         e01:  2563.,  e02:  2629.,  e03:  2651., e0123:  30482.,
        e123:  3407., e032:  -914., e013:   454.,  e021:   -906.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector_2() {
      let result = MULTIVECTOR_A.meet(BIVECTOR_C);
      let expected = Multivector {
           s:     0.,  e23: -3091.,  e31: -3113.,   e12:  -3223.,
         e01: -3377.,  e02: -3421.,  e03: -3443., e0123: -38978.,
        e123: -4309., e032:  1174., e013:  -644.,  e021:   1188.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_trivector_1() {
      let result = MULTIVECTOR_A.meet(TRIVECTOR_A);
      let expected = Multivector {
        e123: 3487., e032: 3641., e013: 3707., e021: 3817.,
        e0123: 5741.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_trivector_2() {
      let result = MULTIVECTOR_A.meet(TRIVECTOR_C);
      let expected = Multivector {
        e123: -4103., e032: -4169., e013: -4213., e021: -4279.,
        e0123: -6521.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_pseudoscalar_1() {
      let result = MULTIVECTOR_A.meet(PSEUDOSCALAR_A);
      let expected = Pseudoscalar { e0123: 4367. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_pseudoscalar_2() {
      let result = MULTIVECTOR_A.meet(PSEUDOSCALAR_C);
      let expected = Pseudoscalar { e0123: -4499. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod scalar {
    use super::*;
    #[test]
    fn meet_multivector_1() {
      let result = SCALAR_A.meet(MULTIVECTOR_A);
      let expected = Multivector {
          e0:  274.,   e1:  411.,   e2:  685.,    e3:  959.,
           s: 1507.,  e23: 1781.,  e31: 2329.,   e12: 2603.,
         e01: 3151.,  e02: 3973.,  e03: 4247., e0123: 5069.,
        e123: 5617., e032: 5891., e013: 6439.,  e021: 7261.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_multivector_2() {
      let result = SCALAR_A.meet(MULTIVECTOR_D);
      let expected = Multivector {
          e0:  -8083.,   e1:  -8357.,   e2:  -9179.,    e3:  -9727.,
           s: -10001.,  e23: -10823.,  e31: -11371.,   e12: -12193.,
         e01: -13289.,  e02: -13837.,  e03: -14111., e0123: -14659.,
        e123: -14933., e032: -15481., e013: -17399.,  e021: -17947.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_1() {
      let result = SCALAR_A.meet(SCALAR_B);
      let expected = Scalar { s: 19043. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_2() {
      let result = SCALAR_A.meet(SCALAR_C);
      let expected = Scalar { s: -20413. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_1() {
      let result = SCALAR_A.meet(VECTOR_A);
      let expected = Vector {
        e0: 20687., e1: 21509., e2: 22331., e3: 22879.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_2() {
      let result = SCALAR_A.meet(VECTOR_C);
      let expected = Vector {
        e0: -26441., e1: -26989., e2: -27263., e3: -28907.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector_1() {
      let result = SCALAR_A.meet(BIVECTOR_A);
      let expected = Bivector {
        e01: 31921., e02: 32743., e03: 33017.,
        e12: 31373., e31: 31099., e23: 30551.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector_2() {
      let result = SCALAR_A.meet(BIVECTOR_C);
      let expected = Bivector {
        e01: -42059., e02: -42607., e03: -42881.,
        e12: -40141., e31: -38771., e23: -38497.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_trivector_1() {
      let result = SCALAR_A.meet(TRIVECTOR_A);
      let expected = Trivector {
        e021: 47539., e013: 46169., e032: 45347., e123: 43429.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_trivector_2() {
      let result = SCALAR_A.meet(TRIVECTOR_C);
      let expected = Trivector {
        e021: -53293., e013: -52471., e032: -51923., e123: -51101.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_pseudoscalar_1() {
      let result = SCALAR_A.meet(PSEUDOSCALAR_A);
      let expected = Pseudoscalar { e0123: 54389. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_pseudoscalar_2() {
      let result = SCALAR_A.meet(PSEUDOSCALAR_C);
      let expected = Pseudoscalar { e0123: -56033. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod vector {
    use super::*;
    #[test]
    fn meet_multivector_1() {
      let result = VECTOR_A.meet(MULTIVECTOR_A);
      let expected = Multivector {
          e0: 1661.,   e1:  1727.,   e2:  1793.,    e3:  1837.,
           s:    0.,  e23:   306.,  e31:  -598.,   e12:   296.,
         e01:  139.,  e02:   429.,  e03:   723., e0123: 29454.,
        e123: 7985., e032: -1753., e013: -3593.,  e021: -2065.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_multivector_2() {
      let result = VECTOR_A.meet(MULTIVECTOR_D);
      let expected = Multivector {
          e0: -11023.,   e1: -11461.,   e2: -11899.,    e3: -12191.,
           s:      0.,  e23:   -384.,  e31:    960.,   e12:   -576.,
         e01:     52.,  e02:   -500.,  e03:   -868., e0123: -76778.,
        e123: -40795., e032:  12007., e013:  12505.,  e021:  13393.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_1() {
      let result = VECTOR_A.meet(SCALAR_A);
      let expected = Vector {
        e0: 20687., e1: 21509., e2: 22331., e3: 22879.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_2() {
      let result = VECTOR_A.meet(SCALAR_C);
      let expected = Vector {
        e0: -22499., e1: -23393., e2: -24287., e3: -24883.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_1() {
      let result = VECTOR_A.meet(VECTOR_A);
      let expected = Bivector::zero();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_2() {
      let result = VECTOR_A.meet(VECTOR_B);
      let expected = Bivector {
        e01: -132., e02: -868., e03: -50.,
        e12: -760., e31:  -94., e23: 906.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_3() {
      let result = VECTOR_A.meet(VECTOR_C);
      let expected = Bivector {
        e01: 554., e02: 1410., e03:   370.,
        e12: 868., e31:  228., e23: -1160.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector_1() {
      let result = VECTOR_A.meet(BIVECTOR_A);
      let expected = Trivector {
        e021: -35035., e013: -33203., e032: -34303., e123: 110255.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector_2() {
      let result = VECTOR_A.meet(BIVECTOR_C);
      let expected = Trivector {
        e021: 45457., e013: 40605., e032: 43349., e123: -139177.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_trivector_1() {
      let result = VECTOR_A.meet(TRIVECTOR_A);
      let expected = Pseudoscalar { e0123: 212714. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_trivector_2() {
      let result = VECTOR_A.meet(TRIVECTOR_C);
      let expected = Pseudoscalar { e0123: -243218. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_pseudoscalar() {
      let result = VECTOR_A.meet(PSEUDOSCALAR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod bivector {
    use super::*;
    #[test]
    fn meet_multivector_1() {
      let result = BIVECTOR_A.meet(MULTIVECTOR_A);
      let expected = Multivector {
           s:    0.,  e23:  2453.,  e31:  2497.,   e12:  2519.,
         e01: 2563.,  e02:  2629.,  e03:  2651., e0123: 30482.,
        e123: 3407., e032:  -914., e013:   454.,  e021:  -906.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_multivector_2() {
      let result = BIVECTOR_A.meet(MULTIVECTOR_D);
      let expected = Multivector {
           s:      0.,  e23: -16279.,  e31: -16571.,   e12:  -16717.,
         e01: -17009.,  e02: -17447.,  e03: -17593., e0123: -127838.,
        e123: -45071., e032:  13979., e013:  11551.,  e021:   14543.,
        ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_1() {
      let result = BIVECTOR_A.meet(SCALAR_A);
      let expected = Bivector {
        e01: 31921., e02: 32743., e03: 33017.,
        e12: 31373., e31: 31099., e23: 30551.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_2() {
      let result = BIVECTOR_A.meet(SCALAR_C);
      let expected = Bivector {
        e01: -34717., e02: -35611., e03: -35909.,
        e12: -34121., e31: -33823., e23: -33227.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_1() {
      let result = BIVECTOR_A.meet(VECTOR_A);
      let expected = Trivector {
        e021: -35035., e013: -33203., e032: -34303., e123: 110255.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_2() {
      let result = BIVECTOR_A.meet(VECTOR_C);
      let expected = Trivector {
        e021: 43481., e013: 42125., e032: 45509., e123: -137423.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector_1() {
      let result = BIVECTOR_A.meet(BIVECTOR_A);
      let expected = Pseudoscalar { e0123: 322802. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector_2() {
      let result = BIVECTOR_A.meet(BIVECTOR_B);
      let expected = Pseudoscalar { e0123: 368226. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector_3() {
      let result = BIVECTOR_A.meet(BIVECTOR_C);
      let expected = Pseudoscalar { e0123: -414458. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_trivector() {
      let result = BIVECTOR_A.meet(TRIVECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_pseudoscalar() {
      let result = BIVECTOR_A.meet(PSEUDOSCALAR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod trivector {
    use super::*;
    #[test]
    fn meet_multivector_1() {
      let result = TRIVECTOR_A.meet(MULTIVECTOR_A);
      let expected = Multivector {
        e123: 3487., e032: 3641., e013: 3707., e021: 3817.,
        e0123: -5741., ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_multivector_2() {
      let result = TRIVECTOR_A.meet(MULTIVECTOR_D);
      let expected = Multivector {
        e123: -23141., e032: -24163., e013: -24601., e021: -25331.,
        e0123: 86110., ..Multivector::zero()
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_1() {
      let result = TRIVECTOR_A.meet(SCALAR_A);
      let expected = Trivector {
        e021: 47539., e013: 46169., e032: 45347., e123: 43429.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_2() {
      let result = TRIVECTOR_A.meet(SCALAR_C);
      let expected = Trivector {
        e021: -51703., e013: -50213., e032: -49319., e123: -47233.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_1() {
      let result = TRIVECTOR_A.meet(VECTOR_A);
      let expected = Pseudoscalar { e0123: -212714. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector_2() {
      let result = TRIVECTOR_A.meet(VECTOR_C);
      let expected = Pseudoscalar { e0123: 266668. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector() {
      let result = TRIVECTOR_A.meet(BIVECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_trivector() {
      let result = TRIVECTOR_A.meet(TRIVECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_pseudoscalar() {
      let result = TRIVECTOR_A.meet(PSEUDOSCALAR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod pseudoscalar {
    use super::*;
    #[test]
    fn meet_multivector_1() {
      let result = PSEUDOSCALAR_A.meet(MULTIVECTOR_A);
      let expected = Pseudoscalar { e0123: 4367. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_multivector_2() {
      let result = PSEUDOSCALAR_A.meet(MULTIVECTOR_D);
      let expected = Pseudoscalar { e0123: -28981. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_1() {
      let result = PSEUDOSCALAR_A.meet(SCALAR_A);
      let expected = Pseudoscalar { e0123: 54389. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_scalar_2() {
      let result = PSEUDOSCALAR_A.meet(SCALAR_C);
      let expected = Pseudoscalar { e0123: -59153. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_vector() {
      let result = PSEUDOSCALAR_A.meet(VECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_bivector() {
      let result = PSEUDOSCALAR_A.meet(BIVECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_trivector() {
      let result = PSEUDOSCALAR_A.meet(TRIVECTOR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn meet_pseudoscalar() {
      let result = PSEUDOSCALAR_A.meet(PSEUDOSCALAR_A);
      let expected = Empty;
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }
}
