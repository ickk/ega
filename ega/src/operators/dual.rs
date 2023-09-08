use crate::*;

/// Duals
pub trait Dual {
  /// The poincare dual
  fn poincare_dual(&self) -> Self;

  /// The Hodge dual relative to the pseudoscalar e0123
  fn dual(&self) -> Self;
}

impl Dual for Multivector {
  #[inline]
  fn poincare_dual(&self) -> Self {
    Multivector {
      s: self.e0123,
      e0: self.e123,
      e1: self.e032,
      e2: self.e013,
      e3: self.e021,
      e01: self.e23,
      e02: self.e31,
      e03: self.e12,
      e12: self.e03,
      e31: self.e02,
      e23: self.e01,
      e021: self.e3,
      e013: self.e2,
      e032: self.e1,
      e123: self.e0,
      e0123: self.s,
    }
  }

  #[inline]
  fn dual(&self) -> Self {
    Multivector {
      s: self.e0123,
      e0: -self.e123,
      e1: -self.e032,
      e2: -self.e013,
      e3: -self.e021,
      e01: self.e23,
      e02: self.e31,
      e03: self.e12,
      e12: self.e03,
      e31: self.e02,
      e23: self.e01,
      e021: self.e3,
      e013: self.e2,
      e032: self.e1,
      e123: self.e0,
      e0123: self.s,
    }
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

  #[test]
  fn multivector_dual() {
    let result = MULTIVECTOR_A.dual();
    let expected = Multivector {
        e0: -41.,   e1: -43.,   e2: -47.,    e3: -53.,
         s:  37.,  e23:  23.,  e31:  29.,   e12:  31.,
       e01:  13.,  e02:  17.,  e03:  19., e0123:  11.,
      e123:   2., e032:   3., e013:   5.,  e021:   7.,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }
}
