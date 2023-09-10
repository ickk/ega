use crate::*;

/// The Hodge dual
pub trait HodgeDual {
  type Output;

  /// The Hodge dual
  fn hodge_dual(self) -> Self::Output;

  /// The inverse of the Hodge dual
  fn hodge_undual(self) -> Self::Output;
}

impl HodgeDual for Multivector {
  type Output = Multivector;

  #[inline]
  fn hodge_dual(self) -> Self {
    Multivector {
      e0: -self.e123,
      e1: -self.e032,
      e2: -self.e013,
      e3: -self.e021,
      s: self.e0123,
      e23: self.e01,
      e31: self.e02,
      e12: self.e03,
      e01: self.e23,
      e02: self.e31,
      e03: self.e12,
      e0123: self.s,
      e123: self.e0,
      e032: self.e1,
      e013: self.e2,
      e021: self.e3,
    }
  }

  #[inline]
  fn hodge_undual(self) -> Self {
    Multivector {
      e0: self.e123,
      e1: self.e032,
      e2: self.e013,
      e3: self.e021,
      s: self.e0123,
      e23: self.e01,
      e31: self.e02,
      e12: self.e03,
      e01: self.e23,
      e02: self.e31,
      e03: self.e12,
      e0123: self.s,
      e123: -self.e0,
      e032: -self.e1,
      e013: -self.e2,
      e021: -self.e3,
    }
  }
}

impl HodgeDual for Scalar {
  type Output = Pseudoscalar;

  #[inline]
  fn hodge_dual(self) -> Pseudoscalar {
    Pseudoscalar { e0123: self.s }
  }

  #[inline]
  fn hodge_undual(self) -> Pseudoscalar {
    Pseudoscalar { e0123: self.s }
  }
}

impl HodgeDual for Vector {
  type Output = Trivector;

  #[inline]
  fn hodge_dual(self) -> Trivector {
    Trivector {
      e123: self.e0,
      e032: self.e1,
      e013: self.e2,
      e021: self.e3,
    }
  }

  #[inline]
  fn hodge_undual(self) -> Trivector {
    Trivector {
      e123: -self.e0,
      e032: -self.e1,
      e013: -self.e2,
      e021: -self.e3,
    }
  }
}

impl HodgeDual for Bivector {
  type Output = Bivector;

  #[inline]
  fn hodge_dual(self) -> Bivector {
    Bivector {
      e23: self.e01,
      e31: self.e02,
      e12: self.e03,
      e01: self.e23,
      e02: self.e31,
      e03: self.e12,
    }
  }

  #[inline]
  fn hodge_undual(self) -> Bivector {
    Bivector {
      e23: self.e01,
      e31: self.e02,
      e12: self.e03,
      e01: self.e23,
      e02: self.e31,
      e03: self.e12,
    }
  }
}

impl HodgeDual for Trivector {
  type Output = Vector;

  #[inline]
  fn hodge_dual(self) -> Vector {
    Vector {
      e0: -self.e123,
      e1: -self.e032,
      e2: -self.e013,
      e3: -self.e021,
    }
  }

  #[inline]
  fn hodge_undual(self) -> Vector {
    Vector {
      e0: self.e123,
      e1: self.e032,
      e2: self.e013,
      e3: self.e021,
    }
  }
}

impl HodgeDual for Pseudoscalar {
  type Output = Scalar;

  #[inline]
  fn hodge_dual(self) -> Scalar {
    Scalar { s: self.e0123 }
  }

  #[inline]
  fn hodge_undual(self) -> Scalar {
    Scalar { s: self.e0123 }
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
  const SCALAR_A: Scalar = Scalar { s: 137. };
  const VECTOR_A: Vector = Vector {
    e0:  151., e1:  157., e2:  163., e3:  167.
  };
  const BIVECTOR_A: Bivector = Bivector {
    e23:  223., e31:  227., e12:  229.,
    e01:  233., e02:  239., e03:  241.,
  };
  const TRIVECTOR_A: Trivector = Trivector {
    e123:  317., e032:  331., e013:  337., e021:  347.
  };
  const PSEUDOSCALAR_A: Pseudoscalar = Pseudoscalar { e0123:  397. };

  mod hodge_dual {
    use super::*;

    #[test]
    fn multivector() {
      let result = MULTIVECTOR_A.hodge_dual();
      let expected = Multivector {
          e0: -41.,   e1: -43.,   e2: -47.,    e3: -53.,
           s:  37.,  e23:  23.,  e31:  29.,   e12:  31.,
         e01:  13.,  e02:  17.,  e03:  19., e0123:  11.,
        e123:   2., e032:   3., e013:   5.,  e021:   7.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }

    #[test]
    fn scalar() {
      let result = SCALAR_A.hodge_dual();
      let expected = Pseudoscalar { e0123: 137. };
      assert_eq!(dbg!(result), dbg!(expected));
    }

    #[test]
    fn vector() {
      let result = VECTOR_A.hodge_dual();
      let expected = Trivector { e123: 151., e032: 157., e013: 163., e021: 167. };
      assert_eq!(dbg!(result), dbg!(expected));
    }

    #[test]
    fn bivector() {
      let result = BIVECTOR_A.hodge_dual();
      let expected = Bivector {
        e23: 233., e31: 239., e12: 241.,
        e01: 223., e02: 227., e03: 229.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }

    #[test]
    fn trivector() {
      let result = TRIVECTOR_A.hodge_dual();
      let expected = Vector { e0: -317., e1: -331., e2: -337., e3: -347. };
      assert_eq!(dbg!(result), dbg!(expected));
    }

    #[test]
    fn pseudoscalar() {
      let result = PSEUDOSCALAR_A.hodge_dual();
      let expected = Scalar { s: 397. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod hodge_undual {
    use super::*;

    #[test]
    fn multivector() {
      let result = MULTIVECTOR_A.hodge_undual();
      let expected = Multivector {
          e0:  41.,   e1:  43.,   e2:  47.,    e3:  53.,
           s:  37.,  e23:  23.,  e31:  29.,   e12:  31.,
         e01:  13.,  e02:  17.,  e03:  19., e0123:  11.,
        e123:  -2., e032:  -3., e013:  -5.,  e021:  -7.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }

    #[test]
    fn scalar() {
      let result = SCALAR_A.hodge_undual();
      let expected = Pseudoscalar { e0123: 137. };
      assert_eq!(dbg!(result), dbg!(expected));
    }

    #[test]
    fn vector() {
      let result = VECTOR_A.hodge_undual();
      let expected = Trivector {
        e123: -151., e032: -157., e013: -163., e021: -167.
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }

    #[test]
    fn bivector() {
      let result = BIVECTOR_A.hodge_undual();
      let expected = Bivector {
        e23: 233., e31: 239., e12: 241.,
        e01: 223., e02: 227., e03: 229.,
      };
      assert_eq!(dbg!(result), dbg!(expected));
    }

    #[test]
    fn trivector() {
      let result = TRIVECTOR_A.hodge_undual();
      let expected = Vector { e0: 317., e1: 331., e2: 337., e3: 347. };
      assert_eq!(dbg!(result), dbg!(expected));
    }

    #[test]
    fn pseudoscalar() {
      let result = PSEUDOSCALAR_A.hodge_undual();
      let expected = Scalar { s: 397. };
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }
}
