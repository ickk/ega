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
    fn geometric_product_multivector_1() {
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
    fn geometric_product_multivector_2() {
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
    fn geometric_product_multivector_3() {
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
    fn geometric_product_scalar_1() {
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
    fn geometric_product_scalar_2() {
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
    fn geometric_product_vector_1() {
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
    fn geometric_product_vector_2() {
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
    fn geometric_product_bivector_1() {
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
    fn geometric_product_bivector_2() {
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
    fn geometric_product_trivector_1() {
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
    fn geometric_product_trivector_2() {
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
    fn geometric_product_pseudoscalar_1() {
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
    fn geometric_product_pseudoscalar_2() {
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
}
