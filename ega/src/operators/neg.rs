use crate::*;

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

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

  #[test]
  fn negate_multivector() {
    let result = -MULTIVECTOR_A;
    let expected = Multivector {
        e0:   -2.,   e1:   -3.,   e2:   -5.,    e3:   -7.,
         s:  -11.,  e23:  -13.,  e31:  -17.,   e12:  -19.,
       e01:  -23.,  e02:  -29.,  e03:  -31., e0123:  -37.,
      e123:  -41., e032:  -43., e013:  -47.,  e021:  -53.,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn negate_scalar() {
    let result = -SCALAR_A;
    let expected = Scalar { s: -137. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn negate_vector() {
    let result = -VECTOR_A;
    let expected = Vector { e0: -151., e1: -157., e2: -163., e3: -167. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn negate_bivector() {
    let result = -BIVECTOR_A;
    let expected = Bivector {
      e23: -223., e31: -227., e12: -229.,
      e01: -233., e02: -239., e03: -241.,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn negate_trivector() {
    let result = -TRIVECTOR_A;
    let expected = Trivector { e021: -347., e013: -337., e032: -331., e123: -317.};
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn negate_pseudoscalar() {
    let result = -PSEUDOSCALAR_A;
    let expected = Pseudoscalar { e0123: -397. };
    assert_eq!(dbg!(result), dbg!(expected));
  }
}
