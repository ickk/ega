use crate::values::*;

/// The reverse
pub trait Reverse {
  /// The reverse
  fn reverse(&self) -> Self;
}

impl Reverse for Multivector {
  #[inline]
  fn reverse(&self) -> Multivector {
    Multivector {
      e01: -self.e01,
      e02: -self.e02,
      e03: -self.e03,
      e23: -self.e23,
      e31: -self.e23,
      e12: -self.e12,
      e123: -self.e123,
      e032: -self.e032,
      e013: -self.e013,
      e021: -self.e021,
      ..*self
    }
  }
}

impl Reverse for Empty {
  #[inline]
  fn reverse(&self) -> Self {
    Empty
  }
}

impl Reverse for Scalar {
  #[inline]
  fn reverse(&self) -> Self {
    *self
  }
}

impl Reverse for Vector {
  #[inline]
  fn reverse(&self) -> Self {
    *self
  }
}

impl Reverse for Bivector {
  #[inline]
  fn reverse(&self) -> Self {
    -*self
  }
}

impl Reverse for Trivector {
  #[inline]
  fn reverse(&self) -> Self {
    -*self
  }
}

impl Reverse for Pseudoscalar {
  #[inline]
  fn reverse(&self) -> Self {
    *self
  }
}
