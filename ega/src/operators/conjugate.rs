use crate::*;

/// Clifford Conjugate
pub trait Conjugate {
  /// Clifford Conjugate
  fn conjugate(self) -> Self;
}

impl Conjugate for Multivector {
  #[inline]
  fn conjugate(mut self) -> Self {
    self.e0 = -self.e0;
    self.e1 = -self.e1;
    self.e2 = -self.e2;
    self.e3 = -self.e3;

    self.e01 = -self.e01;
    self.e02 = -self.e02;
    self.e03 = -self.e03;
    self.e12 = -self.e12;
    self.e31 = -self.e31;
    self.e23 = -self.e23;

    self
  }
}

impl Conjugate for Empty {
  #[inline]
  fn conjugate(self) -> Self {
    Empty
  }
}

impl Conjugate for Scalar {
  #[inline]
  fn conjugate(self) -> Self {
    self
  }
}

impl Conjugate for Vector {
  #[inline]
  fn conjugate(self) -> Self {
    -self
  }
}

impl Conjugate for Bivector {
  #[inline]
  fn conjugate(self) -> Self {
    -self
  }
}

impl Conjugate for Trivector {
  #[inline]
  fn conjugate(self) -> Self {
    self
  }
}

impl Conjugate for Pseudoscalar {
  #[inline]
  fn conjugate(self) -> Self {
    self
  }
}

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

  #[test]
  fn conjugate_multivector() {
    let result = MULTIVECTOR_A.conjugate();
    let expected = Multivector {
      s: 11.,
      e0: -2., e1: -3., e2: -5., e3: -7.,
      e01: -23., e02: -29., e03: -31.,
      e12: -19., e31: -17., e23: -13.,
      e021: 53., e013: 47., e032: 43., e123: 41.,
      e0123: 37.,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn conjugate_scalar() {
    let result = SCALAR_A.conjugate();
    let expected = Scalar { s: 137. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn conjugate_vector() {
    let result = VECTOR_A.conjugate();
    let expected = Vector {
      e0: -151., e1: -157., e2: -163., e3: -167.,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn conjugate_bivector() {
    let result = BIVECTOR_A.conjugate();
    let expected = Bivector {
      e01: -233., e02: -239., e03: -241.,
      e12: -229., e31: -227., e23: -223.,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn conjugate_trivector() {
    let result = TRIVECTOR_A.conjugate();
    let expected = Trivector {
      e021: 347., e013: 337., e032: 331., e123: 317.
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn conjugate_pseudoscalar() {
    let result = PSEUDOSCALAR_A.conjugate();
    let expected = Pseudoscalar { e0123: 397. };
    assert_eq!(dbg!(result), dbg!(expected));
  }
}
