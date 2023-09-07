use crate::values::*;

pub trait Join<Rhs> {
  type Output;

  /// The regressive product
  fn join(&self, rhs: &Rhs) -> Self::Output;
}

macro_rules! impl_join {
  ($join_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl Join<$rhs> for $lhs {
      type Output = $output;
      #[inline]
      fn join(&self, rhs: &$rhs) -> Self::Output {
        $join_fn(self, rhs)
      }
    }
  };
}

impl_join! { multivector_join_multivector: Multivector, Multivector => Multivector }

// Multivector

#[rustfmt::skip]
#[inline]
fn multivector_join_multivector(
  lhs: &Multivector,
  rhs: &Multivector,
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
#[cfg(any(test, doctest))]
// expected results from bivector.net's evaluator
// https://bivector.net/tools.html#seven
mod tests {
  use super::*;
  use crate::operators::Zero;

  // we use prime numbers to reduce the chances of a falsely passing test.
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
  const _TRIVECTOR_B: Trivector = Trivector {
    e123:  349., e032:  353., e013:  359., e021:  367.
  };
  const TRIVECTOR_C: Trivector = Trivector {
    e123: -373., e032: -379., e013: -383., e021: -389.
  };
  const PSEUDOSCALAR_A: Pseudoscalar = Pseudoscalar { e0123:  397. };
  const _PSEUDOSCALAR_B: Pseudoscalar = Pseudoscalar { e0123:  401. };
  const PSEUDOSCALAR_C: Pseudoscalar = Pseudoscalar { e0123: -409. };

  mod multivector {
    use super::*;
    #[test]
    fn join_multivector_1() {
      let result = MULTIVECTOR_A.join(&MULTIVECTOR_B);
      let expected = Multivector {
          e0: -22323.,   e1:  9092.,   e2: 10400.,    e3: 10852.,
           s:  25641.,  e23:  4368.,  e31:  4806.,   e12:  5732.,
         e01:   6624.,  e02:  6484.,  e03:  6978., e0123:  3959.,
        e123:   8420., e032:  8782., e013:  9728.,  e021: 10518.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }
    // #[test]
    // fn join_multivector_2() {
    //   let result = MULTIVECTOR_A.join(&MULTIVECTOR_C);
    //   let expected = Multivector {
    //       e0:  -795.,   e1:   452.,   e2: -1102.,    e3:   270.,
    //        s:  -803.,  e23:   744.,  e31: -1940.,   e12:  -914.,
    //      e01: -2447.,  e02:  -845.,  e03: -2841., e0123: -1271.,
    //     e123: -2744., e032: -6645., e013: -4287.,  e021:  2613.,
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_multivector_3() {
    //   let result = MULTIVECTOR_A.join(&MULTIVECTOR_D);
    //   let expected = Multivector {
    //       e0:  -795.,   e1:  -890.,   e2: -1102.,    e3: -1292.,
    //        s:  -803.,  e23: -1704.,  e31: -2368.,   e12: -2262.,
    //      e01: -2691.,  e02: -3067.,  e03: -3125., e0123: -5951.,
    //     e123: -8748., e032: -3283., e013: -3771.,  e021: -4057.,
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_scalar_1() {
    //   let result = MULTIVECTOR_A.join(&SCALAR_A);
    //   let expected = Multivector {
    //       e0:   274.,   e1:   411.,   e2:   685.,    e3:   959.,
    //        s:  1507.,  e23:  1781.,  e31:  2329.,   e12:  2603.,
    //      e01:  3151.,  e02:  3973.,  e03:  4247., e0123:  5069.,
    //     e123:  5617., e032:  5891., e013:  6439.,  e021:  7261.,
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_scalar_2() {
    //   let result = MULTIVECTOR_A.join(&SCALAR_C);
    //   let expected = Multivector {
    //       e0:  -298.,   e1:  -447.,   e2:  -745.,    e3: -1043.,
    //        s: -1639.,  e23: -1937.,  e31: -2533.,   e12: -2831.,
    //      e01: -3427.,  e02: -4321.,  e03: -4619., e0123: -5513.,
    //     e123: -6109., e032: -6407., e013: -7003.,  e021: -7897.,
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_vector_1() {
    //   let result = MULTIVECTOR_A.join(&VECTOR_A);
    //   let expected = Multivector {
    //       e0:  1661.,   e1:  1727.,   e2:  1793.,    e3:   1837.,
    //        s:     0.,  e23:  -306.,  e31:   598.,   e12:   -296.,
    //      e01:  -139.,  e02:  -429.,  e03:  -723., e0123: -29454.,
    //     e123:  7985., e032: -1753., e013: -3593.,  e021:  -2065.,
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_vector_2() {
    //   let result = MULTIVECTOR_A.join(&VECTOR_C);
    //   let expected = Multivector {
    //       e0: -2123.,   e1: -2167.,   e2: -2189.,    e3:  -2321.,
    //        s:     0.,  e23:   338.,  e31:  -746.,   e12:    388.,
    //      e01:   185.,  e02:   567.,  e03:   929., e0123:  36920.,
    //     e123: -9953., e032:  2459., e013:  4535.,  e021:   2531.,
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_bivector_1() {
    //   let result = MULTIVECTOR_A.join(&BIVECTOR_A);
    //   let expected = Multivector {
    //        s:     0.,  e23:  2453.,  e31:  2497.,   e12:   2519.,
    //      e01:  2563.,  e02:  2629.,  e03:  2651., e0123:  30482.,
    //     e123:  3407., e032:  -914., e013:   454.,  e021:   -906.,
    //     ..Multivector::zero()
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_bivector_2() {
    //   let result = MULTIVECTOR_A.join(&BIVECTOR_C);
    //   let expected = Multivector {
    //        s:     0.,  e23: -3091.,  e31: -3113.,   e12:  -3223.,
    //      e01: -3377.,  e02: -3421.,  e03: -3443., e0123: -38978.,
    //     e123: -4309., e032:  1174., e013:  -644.,  e021:   1188.,
    //     ..Multivector::zero()
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_trivector_1() {
    //   let result = MULTIVECTOR_A.join(&TRIVECTOR_A);
    //   let expected = Multivector {
    //     e123: 3487., e032: 3641., e013: 3707., e021: 3817.,
    //     e0123: 5741.,
    //     ..Multivector::zero()
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_trivector_2() {
    //   let result = MULTIVECTOR_A.join(&TRIVECTOR_C);
    //   let expected = Multivector {
    //     e123: -4103., e032: -4169., e013: -4213., e021: -4279.,
    //     e0123: -6521.,
    //     ..Multivector::zero()
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_pseudoscalar_1() {
    //   let result = MULTIVECTOR_A.join(&PSEUDOSCALAR_A);
    //   let expected = Pseudoscalar { e0123: 4367. };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
    // #[test]
    // fn join_pseudoscalar_2() {
    //   let result = MULTIVECTOR_A.join(&PSEUDOSCALAR_C);
    //   let expected = Pseudoscalar { e0123: -4499. };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
  }
}
