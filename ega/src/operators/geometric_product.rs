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
        - a.e12*b.e12
        - a.e31*b.e31
        - a.e23*b.e23
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
  }
}
