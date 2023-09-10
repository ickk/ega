use crate::*;

/// The reverse
pub trait Reverse {
  /// The reverse
  fn reverse(self) -> Self;
}

impl Reverse for Multivector {
  #[inline]
  fn reverse(self) -> Multivector {
    Multivector {
      e01: -self.e01,
      e02: -self.e02,
      e03: -self.e03,
      e23: -self.e23,
      e31: -self.e31,
      e12: -self.e12,
      e123: -self.e123,
      e032: -self.e032,
      e013: -self.e013,
      e021: -self.e021,
      ..self
    }
  }
}

impl Reverse for Empty {
  #[inline]
  fn reverse(self) -> Self {
    Empty
  }
}

impl Reverse for Scalar {
  #[inline]
  fn reverse(self) -> Self {
    self
  }
}

impl Reverse for Vector {
  #[inline]
  fn reverse(self) -> Self {
    self
  }
}

impl Reverse for Bivector {
  #[inline]
  fn reverse(self) -> Self {
    -self
  }
}

impl Reverse for Trivector {
  #[inline]
  fn reverse(self) -> Self {
    -self
  }
}

impl Reverse for Pseudoscalar {
  #[inline]
  fn reverse(self) -> Self {
    self
  }
}

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

  #[test]
  fn reverse_multivector() {
    let result = MULTIVECTOR_A.reverse();
    let expected = Multivector {
      s: 11., e0: 2., e1: 3., e2: 5., e3: 7.,
      e01: -23., e02: -29., e03: -31., e12: -19., e31: -17., e23: -13.,
      e021: -53., e013: -47., e032: -43., e123: -41.,
      e0123: 37.,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn reverse_scalar() {
    let result = SCALAR_A.reverse();
    let expected = Scalar { s: 137. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn reverse_vector() {
    let result = VECTOR_A.reverse();
    let expected = Vector { e0: 151., e1: 157., e2: 163., e3: 167. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn reverse_bivector() {
    let result = BIVECTOR_A.reverse();
    let expected = Bivector {
      e01: -233., e02: -239., e03: -241.,
      e12: -229., e31: -227., e23: -223.,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn reverse_trivector() {
    let result = TRIVECTOR_A.reverse();
    let expected = Trivector { e021: -347., e013: -337., e032: -331., e123: -317.};
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn reverse_pseudoscalar() {
    let result = PSEUDOSCALAR_A.reverse();
    let expected = Pseudoscalar { e0123: 397. };
    assert_eq!(dbg!(result), dbg!(expected));
  }
}
