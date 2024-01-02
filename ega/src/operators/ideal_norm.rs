use crate::*;

/// The Ideal norm, ||A||_∞
///
/// aka "Infinity Norm" or "Vanishing Norm"
pub trait IdealNorm {
  /// The Ideal norm, ||A||_∞
  fn ideal_norm(self) -> Pseudoscalar;
}

// needs sqrt function, so relies on std or libm
#[cfg(any(feature = "std", feature = "libm"))]
impl<T: IdealNormSquared> IdealNorm for T {
  #[inline]
  fn ideal_norm(self) -> Pseudoscalar {
    Pseudoscalar {
      #[cfg(feature = "libm")]
      e0123: Libm::<f32>::sqrt(self.ideal_norm_squared().e0123),
      #[cfg(not(feature = "libm"))]
      e0123: self.ideal_norm_squared().e0123.sqrt(),
    }
  }
}

pub trait IdealNormSquared {
  fn ideal_norm_squared(self) -> Pseudoscalar;
}

impl IdealNormSquared for Multivector {
  #[inline]
  fn ideal_norm_squared(self) -> Pseudoscalar {
    let e0123 = self.e0123 * self.e0123
      + self.e032 * self.e032
      + self.e013 * self.e013
      + self.e021 * self.e021
      + self.e03 * self.e03
      + self.e02 * self.e02
      + self.e01 * self.e01
      + self.e0 * self.e0;

    Pseudoscalar { e0123 }
  }
}

impl IdealNormSquared for Empty {
  #[inline]
  fn ideal_norm_squared(self) -> Pseudoscalar {
    zero()
  }
}

impl IdealNormSquared for Scalar {
  #[inline]
  fn ideal_norm_squared(self) -> Pseudoscalar {
    zero()
  }
}

impl IdealNormSquared for Vector {
  #[inline]
  fn ideal_norm_squared(self) -> Pseudoscalar {
    let e0123 = self.e0 * self.e0;

    Pseudoscalar { e0123 }
  }
}

impl IdealNormSquared for Bivector {
  #[inline]
  fn ideal_norm_squared(self) -> Pseudoscalar {
    let e0123 =
      self.e01 * self.e01 + self.e02 * self.e02 + self.e03 * self.e03;

    Pseudoscalar { e0123 }
  }
}

impl IdealNormSquared for Trivector {
  #[inline]
  fn ideal_norm_squared(self) -> Pseudoscalar {
    let e0123 =
      self.e032 * self.e032 + self.e013 * self.e013 + self.e021 * self.e021;

    Pseudoscalar { e0123 }
  }
}

impl IdealNormSquared for Pseudoscalar {
  #[inline]
  fn ideal_norm_squared(self) -> Pseudoscalar {
    let e0123 = self.e0123 * self.e0123;

    Pseudoscalar { e0123 }
  }
}

#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

  #[test]
  fn ideal_norm_multivector_1() {
    let result = MULTIVECTOR_A.ideal_norm();
    let expected =
      MULTIVECTOR_A.hodge_dual().norm() * Pseudoscalar { e0123: 1. };

    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn ideal_norm_scalar_1() {
    let result = SCALAR_A.ideal_norm();
    let expected = SCALAR_A.hodge_dual().norm() * Pseudoscalar { e0123: 1. };

    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn ideal_norm_vector_1() {
    let result = VECTOR_A.ideal_norm();
    let expected = VECTOR_A.hodge_dual().norm() * Pseudoscalar { e0123: 1. };

    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn ideal_norm_bivector_1() {
    let result = BIVECTOR_A.ideal_norm();
    let expected = BIVECTOR_A.hodge_dual().norm() * Pseudoscalar { e0123: 1. };

    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn ideal_norm_trivector_1() {
    let result = TRIVECTOR_A.ideal_norm();
    let expected =
      TRIVECTOR_A.hodge_dual().norm() * Pseudoscalar { e0123: 1. };

    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn ideal_norm_pseudoscalar_1() {
    let result = PSEUDOSCALAR_A.ideal_norm();
    let expected =
      PSEUDOSCALAR_A.hodge_dual().norm() * Pseudoscalar { e0123: 1. };

    assert_eq!(dbg!(result), dbg!(expected));
  }
}
