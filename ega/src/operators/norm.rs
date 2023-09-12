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
      s: self.norm_squared().s.sqrt(),
    }
  }
}

/// The squared (Euclidean) norm, ||A||^2
pub trait NormSquared {
  /// The squared (Euclidean) norm, ||A||^2
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

impl NormSquared for Empty {
  #[inline]
  fn norm_squared(self) -> Scalar {
    zero()
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

  #[test]
  fn norm_multivector() {
    let result = MULTIVECTOR_A.norm();

    let expected_scalar_product =
      MULTIVECTOR_A.scalar_product(MULTIVECTOR_A.reverse()).sqrt();
    assert_eq!(dbg!(result), dbg!(expected_scalar_product));

    let expected_geometric_product = MULTIVECTOR_A
      .geometric_product(MULTIVECTOR_A.reverse())
      .grade_0()
      .sqrt();
    assert_eq!(dbg!(result), dbg!(expected_geometric_product));

    let expected_dot_product =
      MULTIVECTOR_A.dot(MULTIVECTOR_A.reverse()).grade_0().sqrt();
    assert_eq!(dbg!(result), dbg!(expected_dot_product));
  }

  #[test]
  fn norm_scalar() {
    let result = SCALAR_A.norm();
    let expected = Scalar { s: 137. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn norm_vector() {
    let result = VECTOR_A.norm();

    let expected_scalar_product =
      VECTOR_A.scalar_product(VECTOR_A.reverse()).sqrt();
    assert_eq!(dbg!(result), dbg!(expected_scalar_product));

    let expected_geometric_product =
      (VECTOR_A.geometric_product(VECTOR_A.reverse()).grade_0()).sqrt();
    assert_eq!(dbg!(result), dbg!(expected_geometric_product));

    let expected_dot_product = VECTOR_A.dot(VECTOR_A).sqrt();
    assert_eq!(dbg!(result), dbg!(expected_dot_product));
  }

  #[test]
  fn norm_bivector() {
    let result = BIVECTOR_A.norm();

    let expected_scalar_product =
      BIVECTOR_A.scalar_product(BIVECTOR_A.reverse()).sqrt();
    assert_eq!(dbg!(result), dbg!(expected_scalar_product));

    let expected_geometric_product =
      (BIVECTOR_A.geometric_product(BIVECTOR_A.reverse()).grade_0()).sqrt();
    assert_eq!(dbg!(result), dbg!(expected_geometric_product));

    let expected_dot_product = BIVECTOR_A.dot(BIVECTOR_A.reverse()).sqrt();
    assert_eq!(dbg!(result), dbg!(expected_dot_product));
  }

  #[test]
  fn norm_trivector() {
    let result = TRIVECTOR_A.norm();

    let expected_scalar_product =
      TRIVECTOR_A.scalar_product(TRIVECTOR_A.reverse()).sqrt();
    assert_eq!(dbg!(result), dbg!(expected_scalar_product));

    let expected_geometric_product = TRIVECTOR_A
      .geometric_product(TRIVECTOR_A.reverse())
      .grade_0()
      .sqrt();
    assert_eq!(dbg!(result), dbg!(expected_geometric_product));

    let expected_dot_product = TRIVECTOR_A.dot(TRIVECTOR_A.reverse()).sqrt();
    assert_eq!(dbg!(result), dbg!(expected_dot_product));
  }
}
