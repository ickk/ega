use crate::values::*;
use core::ops::Neg;

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
    Vector::from([-self.e0(), -self.e1(), -self.e2(), -self.e3()])
  }
}

impl Neg for Bivector {
  type Output = Bivector;

  #[rustfmt::skip]
  #[inline]
  fn neg(self) -> Self::Output {
    Bivector::from([
      -self.e23(), -self.e31(), -self.e12(),
      -self.e01(), -self.e02(), -self.e03(),
    ])
  }
}

impl Neg for Pseudoscalar {
  type Output = Pseudoscalar;

  #[inline]
  fn neg(self) -> Self::Output {
    Pseudoscalar::from([-self.e0123()])
  }
}

impl Neg for Trivector {
  type Output = Trivector;

  #[inline]
  fn neg(self) -> Self::Output {
    Trivector::from([-self.e123(), -self.e032(), -self.e013(), -self.e021()])
  }
}

impl Neg for Multivector {
  type Output = Multivector;

  #[rustfmt::skip]
  #[inline]
  fn neg(self) -> Self::Output {
    Multivector::from([
      -self.e0(), -self.e01(), -self.e02(), -self.e03(),
      -self.scalar(), -self.e23(), -self.e31(), -self.e12(),
      -self.e01(), -self.e02(), -self.e03(), -self.e0123(),
      -self.e123(), -self.e032(), -self.e013(), -self.e021(),
    ])
  }
}
