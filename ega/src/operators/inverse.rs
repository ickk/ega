use crate::*;

pub trait Inverse {
  fn inverse(self) -> Self;
}

impl Inverse for Scalar {
  #[inline]
  fn inverse(self) -> Self {
    Scalar { s: 1.0 / self.s }
  }
}

impl Inverse for Vector {
  #[inline]
  fn inverse(self) -> Self {
    simple_inverse(self)
  }
}

impl Inverse for Trivector {
  #[inline]
  fn inverse(self) -> Self {
    simple_inverse(self)
  }
}

// this is only valid for some kinds of elements
#[inline]
fn simple_inverse<T: Copy + Reverse + NormSquared + Mul<f32, Output = T>>(
  value: T,
) -> T {
  value.reverse() * (1.0 / value.norm_squared().s)
}

#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;
  use ::approx::assert_relative_eq;

  #[test]
  fn inverse_scalar_a() {
    let inverse = SCALAR_A.inverse();
    let product = SCALAR_A * inverse;

    let expected = Scalar::UNIT;
    assert_relative_eq!(dbg!(expected), dbg!(product));
  }

  #[test]
  fn inverse_scalar_b() {
    let inverse = SCALAR_B.inverse();
    let product = SCALAR_B * inverse;

    let expected = Scalar::UNIT;
    assert_relative_eq!(dbg!(expected), dbg!(product));
  }

  #[test]
  fn inverse_scalar_c() {
    let inverse = SCALAR_C.inverse();
    let product = SCALAR_C * inverse;

    let expected = Scalar::UNIT;
    assert_relative_eq!(dbg!(expected), dbg!(product));
  }

  #[test]
  fn inverse_vector_a() {
    let inverse = VECTOR_A.inverse();
    let product = VECTOR_A * inverse;

    let expected = Multivector::from(Scalar::UNIT);
    assert_relative_eq!(dbg!(expected), dbg!(product));
  }

  #[test]
  fn inverse_vector_b() {
    let inverse = VECTOR_B.inverse();
    let product = VECTOR_B * inverse;

    let expected = Multivector::from(Scalar::UNIT);
    assert_relative_eq!(dbg!(expected), dbg!(product));
  }

  #[test]
  fn inverse_vector_c() {
    let inverse = VECTOR_C.inverse();
    let product = VECTOR_C * inverse;

    let expected = Multivector::from(Scalar::UNIT);
    assert_relative_eq!(dbg!(expected), dbg!(product));
  }

  #[test]
  fn inverse_trivector_a() {
    let inverse = TRIVECTOR_A.inverse();
    let product = TRIVECTOR_A * inverse;

    let expected = Multivector::from(Scalar::UNIT);
    assert_relative_eq!(dbg!(expected), dbg!(product));
  }

  #[test]
  fn inverse_trivector_b() {
    let inverse = TRIVECTOR_B.inverse();
    let product = TRIVECTOR_B * inverse;

    let expected = Multivector::from(Scalar::UNIT);
    assert_relative_eq!(dbg!(expected), dbg!(product));
  }

  #[test]
  fn inverse_trivector_c() {
    let inverse = TRIVECTOR_C.inverse();
    let product = TRIVECTOR_C * inverse;

    let expected = Multivector::from(Scalar::UNIT);
    assert_relative_eq!(dbg!(expected), dbg!(product));
  }
}
