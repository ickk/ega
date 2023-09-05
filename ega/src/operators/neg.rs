use crate::values::*;

pub use core::ops::Neg;

impl Neg for Multivector {
  type Output = Multivector;

  #[inline]
  fn neg(self) -> Self::Output {
    Multivector::from(self.elements.map(|e| -e))
  }
}

impl Neg for Empty {
  type Output = Empty;

  #[inline]
  fn neg(self) -> Self::Output {
    Empty
  }
}

impl Neg for Scalar {
  type Output = Scalar;

  #[inline]
  fn neg(self) -> Self::Output {
    Scalar::from([-self.scalar()])
  }
}

impl Neg for Vector {
  type Output = Vector;

  #[inline]
  fn neg(self) -> Self::Output {
    Vector::from(self.elements.map(|e| -e))
  }
}

impl Neg for Bivector {
  type Output = Bivector;

  #[inline]
  fn neg(self) -> Self::Output {
    Bivector::from(self.elements.map(|e| -e))
  }
}

impl Neg for Trivector {
  type Output = Trivector;

  #[inline]
  fn neg(self) -> Self::Output {
    Trivector::from(self.elements.map(|e| -e))
  }
}

impl Neg for Pseudoscalar {
  type Output = Pseudoscalar;

  #[inline]
  fn neg(self) -> Self::Output {
    Pseudoscalar::from([-self.e0123()])
  }
}
