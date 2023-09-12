use crate::*;

/// The Euclidean norm, ||A||
pub trait Norm {
  /// The Euclidean norm, ||A||
  fn norm(self) -> Scalar;
}

// needs sqrt function, so relies on std or libm
#[cfg(any(feature = "std", feature = "libm"))]
impl<T: NormSquared> Norm for T {
  #[inline]
  fn norm(self) -> Scalar {
    Scalar {
      s: self.norm_squared().s.abs().sqrt(),
    }
  }
}

/// The squared norm, ||A||^2
pub trait NormSquared {
  /// The squared norm, ||A||^2
  ///
  /// A * reverse(A)
  fn norm_squared(self) -> Scalar;
}

impl NormSquared for Multivector {
  #[inline]
  fn norm_squared(self) -> Scalar {
    let s = self.s * self.s
      + self.e1 * self.e1
      + self.e2 * self.e2
      + self.e3 * self.e3
      + self.e12 * self.e12
      + self.e31 * self.e31
      + self.e23 * self.e23
      + self.e123 * self.e123;

    Scalar { s }
  }
}

impl NormSquared for Scalar {
  #[inline]
  fn norm_squared(self) -> Scalar {
    let s = self.s * self.s;

    Scalar { s }
  }
}

impl NormSquared for Vector {
  #[inline]
  fn norm_squared(self) -> Scalar {
    let s = self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3;

    Scalar { s }
  }
}

impl NormSquared for Bivector {
  #[inline]
  fn norm_squared(self) -> Scalar {
    let s = self.e12 * self.e12 + self.e31 * self.e31 + self.e23 * self.e23;

    Scalar { s }
  }
}

impl NormSquared for Trivector {
  #[inline]
  fn norm_squared(self) -> Scalar {
    let s = self.e123 * self.e123;

    Scalar { s }
  }
}

impl NormSquared for Pseudoscalar {
  #[inline]
  fn norm_squared(self) -> Scalar {
    zero()
  }
}

#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

  // #[test]
  // fn norm_multivector() {
  //   let result = MULTIVECTOR_A.norm();
  //
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }

  #[test]
  fn norm_scalar() {
    let result = SCALAR_A.norm();
    let expected = Scalar { s: 137. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  // #[test]
  // fn norm_vector() {
  //   let result = VECTOR_A.norm();
  //
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }

  // #[test]
  // fn norm_bivector() {
  //   let result = BIVECTOR_A.norm();
  //
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }

  // #[test]
  // fn norm_trivector() {
  //   let result = TRIVECTOR_A.norm();
  //
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }

  // #[test]
  // fn norm_pseudoscalar() {
  //   let result = PSEUDOSCALAR_A.norm();
  //
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }
}
