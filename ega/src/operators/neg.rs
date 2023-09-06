use crate::values::*;

pub use core::ops::Neg;

impl Neg for Multivector {
  type Output = Multivector;

  #[inline]
  fn neg(self) -> Self::Output {
    MultivectorArray::from(self.to_multivector_array().elements.map(|e| -e))
      .to_multivector()
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
    Scalar::from(-self.s)
  }
}

impl Neg for Vector {
  type Output = Vector;

  #[inline]
  fn neg(self) -> Self::Output {
    VectorArray::from(self.to_vector_array().elements.map(|e| -e)).to_vector()
  }
}

impl Neg for Bivector {
  type Output = Bivector;

  #[inline]
  fn neg(self) -> Self::Output {
    BivectorArray::from(self.to_bivector_array().elements.map(|e| -e))
      .to_bivector()
  }
}

impl Neg for Trivector {
  type Output = Trivector;

  #[inline]
  fn neg(self) -> Self::Output {
    TrivectorArray::from(self.to_trivector_array().elements.map(|e| -e))
      .to_trivector()
  }
}

impl Neg for Pseudoscalar {
  type Output = Pseudoscalar;

  #[inline]
  fn neg(self) -> Self::Output {
    Pseudoscalar::from(-self.e0123)
  }
}
