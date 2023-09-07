use crate::values::*;

/// The regressive product
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
  }
}
