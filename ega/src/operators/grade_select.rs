use crate::*;

/// The grade selection operator
pub trait GradeSelect {
  /// Get the grade 0 [`Scalar`] element
  fn grade_0(self) -> Scalar;

  /// Get the grade 1 [`Vector`] elements
  fn grade_1(self) -> Vector;

  /// Get the grade 2 [`Bivector`] elements
  fn grade_2(self) -> Bivector;

  /// Get the grade 3 [`Trivector`] elements
  fn grade_3(self) -> Trivector;

  /// Get the grade 4 [`Pseudoscalar`] element
  fn grade_4(self) -> Pseudoscalar;
}

impl GradeSelect for Multivector {
  #[inline]
  fn grade_0(self) -> Scalar {
    Scalar::from(self.s)
  }

  #[inline]
  fn grade_1(self) -> Vector {
    let mut a = [0f32; 4];
    a.copy_from_slice(&self.to_multivector_array().elements[0..=3]);
    VectorArray::from(a).to_vector()
  }

  #[inline]
  fn grade_2(self) -> Bivector {
    let mut a = [0f32; 6];
    a.copy_from_slice(&self.to_multivector_array().elements[5..=10]);
    BivectorArray::from(a).to_bivector()
  }

  #[inline]
  fn grade_3(self) -> Trivector {
    let mut a = [0f32; 4];
    a.copy_from_slice(&self.to_multivector_array().elements[12..=15]);
    TrivectorArray::from(a).to_trivector()
  }

  #[inline]
  fn grade_4(self) -> Pseudoscalar {
    Pseudoscalar::from(self.e0123)
  }
}

#[rustfmt::skip]
impl GradeSelect for Empty {
  #[inline]
  fn grade_0(self) -> Scalar { Scalar::zero() }
  #[inline]
  fn grade_1(self) -> Vector { Vector::zero() }
  #[inline]
  fn grade_2(self) -> Bivector { Bivector::zero() }
  #[inline]
  fn grade_3(self) -> Trivector { Trivector::zero() }
  #[inline]
  fn grade_4(self) -> Pseudoscalar { Pseudoscalar::zero() }
}

#[rustfmt::skip]
impl GradeSelect for Scalar {
  #[inline]
  fn grade_0(self) -> Scalar { self }
  #[inline]
  fn grade_1(self) -> Vector { Vector::zero() }
  #[inline]
  fn grade_2(self) -> Bivector { Bivector::zero() }
  #[inline]
  fn grade_3(self) -> Trivector { Trivector::zero() }
  #[inline]
  fn grade_4(self) -> Pseudoscalar { Pseudoscalar::zero() }
}

#[rustfmt::skip]
impl GradeSelect for Vector {
  #[inline]
  fn grade_0(self) -> Scalar { Scalar::zero() }
  #[inline]
  fn grade_1(self) -> Vector { self }
  #[inline]
  fn grade_2(self) -> Bivector { Bivector::zero() }
  #[inline]
  fn grade_3(self) -> Trivector { Trivector::zero() }
  #[inline]
  fn grade_4(self) -> Pseudoscalar { Pseudoscalar::zero() }
}

#[rustfmt::skip]
impl GradeSelect for Bivector {
  #[inline]
  fn grade_0(self) -> Scalar { Scalar::zero() }
  #[inline]
  fn grade_1(self) -> Vector { Vector::zero() }
  #[inline]
  fn grade_2(self) -> Bivector { self }
  #[inline]
  fn grade_3(self) -> Trivector { Trivector::zero() }
  #[inline]
  fn grade_4(self) -> Pseudoscalar { Pseudoscalar::zero() }
}

#[rustfmt::skip]
impl GradeSelect for Trivector {
  #[inline]
  fn grade_0(self) -> Scalar { Scalar::zero() }
  #[inline]
  fn grade_1(self) -> Vector { Vector::zero() }
  #[inline]
  fn grade_2(self) -> Bivector { Bivector::zero() }
  #[inline]
  fn grade_3(self) -> Trivector { self }
  #[inline]
  fn grade_4(self) -> Pseudoscalar { Pseudoscalar::zero() }
}

#[rustfmt::skip]
impl GradeSelect for Pseudoscalar {
  #[inline]
  fn grade_0(self) -> Scalar { Scalar::zero() }
  #[inline]
  fn grade_1(self) -> Vector { Vector::zero() }
  #[inline]
  fn grade_2(self) -> Bivector { Bivector::zero() }
  #[inline]
  fn grade_3(self) -> Trivector { Trivector::zero() }
  #[inline]
  fn grade_4(self) -> Pseudoscalar { self }
}

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

  #[test]
  fn multivector_grade_0() {
    let result = MULTIVECTOR_A.grade_0();
    let expected = Scalar { s: 11. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn multivector_grade_1() {
    let result = MULTIVECTOR_A.grade_1();
    let expected = Vector { e0: 2., e1: 3., e2: 5., e3: 7. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn multivector_grade_2() {
    let result = MULTIVECTOR_A.grade_2();
    let expected = Bivector {
      e23: 13., e31: 17., e12: 19.,
      e01: 23., e02: 29., e03: 31.,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn multivector_grade_3() {
    let result = MULTIVECTOR_A.grade_3();
    let expected = Trivector { e123: 41., e032: 43., e013: 47., e021: 53. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn multivector_grade_4() {
    let result = MULTIVECTOR_A.grade_4();
    let expected = Pseudoscalar { e0123: 37. };
    assert_eq!(dbg!(result), dbg!(expected));
  }
}
