use super::Nil;
use crate::values::*;

pub trait GradeSelect {
  /// Get the grade 0 [`Scalar`] element
  fn grade_0(&self) -> Scalar;

  /// Get the grade 1 [`Vector`] elements
  fn grade_1(&self) -> Vector;

  /// Get the grade 2 [`Bivector`] elements
  fn grade_2(&self) -> Bivector;

  /// Get the grade 3 [`Trivector`] elements
  fn grade_3(&self) -> Trivector;

  /// Get the grade 4 [`Pseudoscalar`] element
  fn grade_4(&self) -> Pseudoscalar;
}

impl GradeSelect for Multivector {
  #[inline]
  fn grade_0(&self) -> Scalar {
    Scalar::from(self.scalar())
  }

  #[inline]
  fn grade_1(&self) -> Vector {
    let mut a = [0f32; 4];
    a.copy_from_slice(&self.elements[0..=3]);
    Vector::from(a)
  }

  #[inline]
  fn grade_2(&self) -> Bivector {
    let mut a = [0f32; 6];
    a.copy_from_slice(&self.elements[5..=10]);
    Bivector::from(a)
  }

  #[inline]
  fn grade_3(&self) -> Trivector {
    let mut a = [0f32; 4];
    a.copy_from_slice(&self.elements[12..=15]);
    Trivector::from(a)
  }

  #[inline]
  fn grade_4(&self) -> Pseudoscalar {
    Pseudoscalar::from(self.e0123())
  }
}

#[rustfmt::skip]
impl GradeSelect for Empty {
  #[inline]
  fn grade_0(&self) -> Scalar { Scalar::nil() }
  #[inline]
  fn grade_1(&self) -> Vector { Vector::nil() }
  #[inline]
  fn grade_2(&self) -> Bivector { Bivector::nil() }
  #[inline]
  fn grade_3(&self) -> Trivector { Trivector::nil() }
  #[inline]
  fn grade_4(&self) -> Pseudoscalar { Pseudoscalar::nil() }
}

#[rustfmt::skip]
impl GradeSelect for Scalar {
  #[inline]
  fn grade_0(&self) -> Scalar { *self }
  #[inline]
  fn grade_1(&self) -> Vector { Vector::nil() }
  #[inline]
  fn grade_2(&self) -> Bivector { Bivector::nil() }
  #[inline]
  fn grade_3(&self) -> Trivector { Trivector::nil() }
  #[inline]
  fn grade_4(&self) -> Pseudoscalar { Pseudoscalar::nil() }
}

#[rustfmt::skip]
impl GradeSelect for Vector {
  #[inline]
  fn grade_0(&self) -> Scalar { Scalar::nil() }
  #[inline]
  fn grade_1(&self) -> Vector { *self }
  #[inline]
  fn grade_2(&self) -> Bivector { Bivector::nil() }
  #[inline]
  fn grade_3(&self) -> Trivector { Trivector::nil() }
  #[inline]
  fn grade_4(&self) -> Pseudoscalar { Pseudoscalar::nil() }
}

#[rustfmt::skip]
impl GradeSelect for Bivector {
  #[inline]
  fn grade_0(&self) -> Scalar { Scalar::nil() }
  #[inline]
  fn grade_1(&self) -> Vector { Vector::nil() }
  #[inline]
  fn grade_2(&self) -> Bivector { *self }
  #[inline]
  fn grade_3(&self) -> Trivector { Trivector::nil() }
  #[inline]
  fn grade_4(&self) -> Pseudoscalar { Pseudoscalar::nil() }
}

#[rustfmt::skip]
impl GradeSelect for Trivector {
  #[inline]
  fn grade_0(&self) -> Scalar { Scalar::nil() }
  #[inline]
  fn grade_1(&self) -> Vector { Vector::nil() }
  #[inline]
  fn grade_2(&self) -> Bivector { Bivector::nil() }
  #[inline]
  fn grade_3(&self) -> Trivector { *self }
  #[inline]
  fn grade_4(&self) -> Pseudoscalar { Pseudoscalar::nil() }
}

#[rustfmt::skip]
impl GradeSelect for Pseudoscalar {
  #[inline]
  fn grade_0(&self) -> Scalar { Scalar::nil() }
  #[inline]
  fn grade_1(&self) -> Vector { Vector::nil() }
  #[inline]
  fn grade_2(&self) -> Bivector { Bivector::nil() }
  #[inline]
  fn grade_3(&self) -> Trivector { Trivector::nil() }
  #[inline]
  fn grade_4(&self) -> Pseudoscalar { *self }
}
