use super::return_zero;
use crate::*;

/// Scalar Product
///
/// Not to be confused with scalar-multiplication, the Scalar Product is
/// equivalent to the grade-0 element of the geometric product.
pub trait ScalarProduct<Rhs> {
  /// Scalar Product
  ///
  /// Not to be confused with scalar-multiplication, the Scalar Product is
  /// equivalent to the grade-0 element of the geometric product.
  fn scalar_product(self, rhs: Rhs) -> Scalar;
}

macro_rules! impl_scalar_product {
  ($dot_fn:ident: $lhs:ty, $rhs:ty) => {
    impl ScalarProduct<$rhs> for $lhs {
      #[inline]
      fn scalar_product(self, rhs: $rhs) -> Scalar {
        $dot_fn(self, rhs)
      }
    }
  };
}

impl_scalar_product! { multivector_scalar_product_multivector: Multivector, Multivector }
impl_scalar_product! { multivector_scalar_product_scalar: Multivector, Scalar }
impl_scalar_product! { multivector_scalar_product_vector: Multivector, Vector }
impl_scalar_product! { multivector_scalar_product_bivector: Multivector, Bivector }
impl_scalar_product! { multivector_scalar_product_trivector: Multivector, Trivector }
impl_scalar_product! { scalar_scalar_product_multivector: Scalar, Multivector }
impl_scalar_product! { scalar_scalar_product_scalar: Scalar, Scalar }
impl_scalar_product! { vector_scalar_product_multivector: Vector, Multivector }
impl_scalar_product! { vector_scalar_product_vector: Vector, Vector }
impl_scalar_product! { bivector_scalar_product_multivector: Bivector, Multivector }
impl_scalar_product! { bivector_scalar_product_bivector: Bivector, Bivector }
impl_scalar_product! { trivector_scalar_product_multivector: Trivector, Multivector }
impl_scalar_product! { trivector_scalar_product_trivector: Trivector, Trivector }

impl_scalar_product! { return_zero: Multivector, Empty }
impl_scalar_product! { return_zero: Multivector, Pseudoscalar }
impl_scalar_product! { return_zero: Scalar, Empty }
impl_scalar_product! { return_zero: Scalar, Vector }
impl_scalar_product! { return_zero: Scalar, Bivector }
impl_scalar_product! { return_zero: Scalar, Trivector }
impl_scalar_product! { return_zero: Scalar, Pseudoscalar }
impl_scalar_product! { return_zero: Vector, Empty }
impl_scalar_product! { return_zero: Vector, Scalar }
impl_scalar_product! { return_zero: Vector, Bivector }
impl_scalar_product! { return_zero: Vector, Trivector }
impl_scalar_product! { return_zero: Vector, Pseudoscalar }
impl_scalar_product! { return_zero: Bivector, Empty }
impl_scalar_product! { return_zero: Bivector, Scalar }
impl_scalar_product! { return_zero: Bivector, Vector }
impl_scalar_product! { return_zero: Bivector, Trivector }
impl_scalar_product! { return_zero: Bivector, Pseudoscalar }
impl_scalar_product! { return_zero: Trivector, Empty }
impl_scalar_product! { return_zero: Trivector, Scalar }
impl_scalar_product! { return_zero: Trivector, Vector }
impl_scalar_product! { return_zero: Trivector, Bivector }
impl_scalar_product! { return_zero: Trivector, Pseudoscalar }
impl_scalar_product! { return_zero: Pseudoscalar, Multivector }
impl_scalar_product! { return_zero: Pseudoscalar, Empty }
impl_scalar_product! { return_zero: Pseudoscalar, Scalar }
impl_scalar_product! { return_zero: Pseudoscalar, Vector }
impl_scalar_product! { return_zero: Pseudoscalar, Bivector }
impl_scalar_product! { return_zero: Pseudoscalar, Trivector }
impl_scalar_product! { return_zero: Pseudoscalar, Pseudoscalar }
impl_scalar_product! { return_zero: Empty, Empty }
impl_scalar_product! { return_zero: Empty, Multivector }
impl_scalar_product! { return_zero: Empty, Scalar }
impl_scalar_product! { return_zero: Empty, Vector }
impl_scalar_product! { return_zero: Empty, Bivector }
impl_scalar_product! { return_zero: Empty, Trivector }
impl_scalar_product! { return_zero: Empty, Pseudoscalar }

#[rustfmt::skip]
#[inline]
fn multivector_scalar_product_multivector(
  lhs: Multivector,
  rhs: Multivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.s*b.s
        + a.e1*b.e1
        + a.e2*b.e2
        + a.e3*b.e3
        - a.e23*b.e23
        - a.e31*b.e31
        - a.e12*b.e12
        - a.e123*b.e123;

  Scalar { s }
}

#[inline]
fn multivector_scalar_product_scalar(lhs: Multivector, rhs: Scalar) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.s * b.s;

  Scalar { s }
}

#[inline]
fn multivector_scalar_product_vector(lhs: Multivector, rhs: Vector) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.e1 * b.e1 + a.e2 * b.e2 + a.e3 * b.e3;

  Scalar { s }
}

#[inline]
fn multivector_scalar_product_bivector(
  lhs: Multivector,
  rhs: Bivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = -a.e23 * b.e23 - a.e31 * b.e31 - a.e12 * b.e12;

  Scalar { s }
}

#[inline]
fn multivector_scalar_product_trivector(
  lhs: Multivector,
  rhs: Trivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = -a.e123 * b.e123;

  Scalar { s }
}

#[inline]
fn scalar_scalar_product_multivector(lhs: Scalar, rhs: Multivector) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.s * b.s;

  Scalar { s }
}

#[inline]
fn scalar_scalar_product_scalar(lhs: Scalar, rhs: Scalar) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.s * b.s;

  Scalar { s }
}

#[inline]
fn vector_scalar_product_multivector(lhs: Vector, rhs: Multivector) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.e1 * b.e1 + a.e2 * b.e2 + a.e3 * b.e3;

  Scalar { s }
}

#[inline]
fn vector_scalar_product_vector(lhs: Vector, rhs: Vector) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = a.e1 * b.e1 + a.e2 * b.e2 + a.e3 * b.e3;

  Scalar { s }
}

#[inline]
fn bivector_scalar_product_multivector(
  lhs: Bivector,
  rhs: Multivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = -a.e23 * b.e23 - a.e31 * b.e31 - a.e12 * b.e12;

  Scalar { s }
}

#[inline]
fn bivector_scalar_product_bivector(lhs: Bivector, rhs: Bivector) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = -a.e23 * b.e23 - a.e31 * b.e31 - a.e12 * b.e12;

  Scalar { s }
}

#[inline]
fn trivector_scalar_product_multivector(
  lhs: Trivector,
  rhs: Multivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = -a.e123 * b.e123;

  Scalar { s }
}

#[inline]
fn trivector_scalar_product_trivector(
  lhs: Trivector,
  rhs: Trivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);

  let s = -a.e123 * b.e123;

  Scalar { s }
}

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

  mod multivector {
    use super::*;
    #[test]
    fn scalar_product_multivector_1() {
      let result = MULTIVECTOR_A.scalar_product(MULTIVECTOR_B);
      let expected = MULTIVECTOR_A.geometric_product(MULTIVECTOR_B).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_scalar_1() {
      let result = MULTIVECTOR_A.scalar_product(SCALAR_A);
      let expected = MULTIVECTOR_A.geometric_product(SCALAR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_vector_1() {
      let result = MULTIVECTOR_A.scalar_product(VECTOR_A);
      let expected = MULTIVECTOR_A.geometric_product(VECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_bivector_1() {
      let result = MULTIVECTOR_A.scalar_product(BIVECTOR_A);
      let expected = MULTIVECTOR_A.geometric_product(BIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_trivector_1() {
      let result = MULTIVECTOR_A.scalar_product(TRIVECTOR_A);
      let expected = MULTIVECTOR_A.geometric_product(TRIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_pseudoscalar_1() {
      let result = MULTIVECTOR_A.scalar_product(PSEUDOSCALAR_A);
      let expected = MULTIVECTOR_A.geometric_product(PSEUDOSCALAR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod scalar {
    use super::*;
    #[test]
    fn scalar_product_multivector_1() {
      let result = SCALAR_A.scalar_product(MULTIVECTOR_A);
      let expected = SCALAR_A.geometric_product(MULTIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_scalar_1() {
      let result = SCALAR_A.scalar_product(SCALAR_B);
      let expected = SCALAR_A.geometric_product(SCALAR_B).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_vector_1() {
      let result = SCALAR_A.scalar_product(VECTOR_A);
      let expected = SCALAR_A.geometric_product(VECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_bivector_1() {
      let result = SCALAR_A.scalar_product(BIVECTOR_A);
      let expected = SCALAR_A.geometric_product(BIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_trivector_1() {
      let result = SCALAR_A.scalar_product(TRIVECTOR_A);
      let expected = SCALAR_A.geometric_product(TRIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_pseudoscalar_1() {
      let result = SCALAR_A.scalar_product(PSEUDOSCALAR_A);
      let expected = SCALAR_A.geometric_product(PSEUDOSCALAR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod vector {
    use super::*;
    #[test]
    fn scalar_product_multivector_1() {
      let result = VECTOR_A.scalar_product(MULTIVECTOR_A);
      let expected = VECTOR_A.geometric_product(MULTIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_scalar_1() {
      let result = VECTOR_A.scalar_product(SCALAR_A);
      let expected = VECTOR_A.geometric_product(SCALAR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_vector_1() {
      let result = VECTOR_A.scalar_product(VECTOR_B);
      let expected = VECTOR_A.geometric_product(VECTOR_B).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_bivector_1() {
      let result = VECTOR_A.scalar_product(BIVECTOR_A);
      let expected = VECTOR_A.geometric_product(BIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_trivector_1() {
      let result = VECTOR_A.scalar_product(TRIVECTOR_A);
      let expected = VECTOR_A.geometric_product(TRIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_pseudoscalar_1() {
      let result = VECTOR_A.scalar_product(PSEUDOSCALAR_A);
      let expected = VECTOR_A.geometric_product(PSEUDOSCALAR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod bivector {
    use super::*;
    #[test]
    fn scalar_product_multivector_1() {
      let result = BIVECTOR_A.scalar_product(MULTIVECTOR_A);
      let expected = BIVECTOR_A.geometric_product(MULTIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_scalar_1() {
      let result = BIVECTOR_A.scalar_product(SCALAR_A);
      let expected = BIVECTOR_A.geometric_product(SCALAR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_vector_1() {
      let result = BIVECTOR_A.scalar_product(VECTOR_A);
      let expected = BIVECTOR_A.geometric_product(VECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_bivector_1() {
      let result = BIVECTOR_A.scalar_product(BIVECTOR_B);
      let expected = BIVECTOR_A.geometric_product(BIVECTOR_B).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_trivector_1() {
      let result = BIVECTOR_A.scalar_product(TRIVECTOR_A);
      let expected = BIVECTOR_A.geometric_product(TRIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_pseudoscalar_1() {
      let result = BIVECTOR_A.scalar_product(PSEUDOSCALAR_A);
      let expected = BIVECTOR_A.geometric_product(PSEUDOSCALAR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod trivector {
    use super::*;
    #[test]
    fn scalar_product_multivector_1() {
      let result = TRIVECTOR_A.scalar_product(MULTIVECTOR_A);
      let expected = TRIVECTOR_A.geometric_product(MULTIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_scalar_1() {
      let result = TRIVECTOR_A.scalar_product(SCALAR_A);
      let expected = TRIVECTOR_A.geometric_product(SCALAR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_vector_1() {
      let result = TRIVECTOR_A.scalar_product(VECTOR_A);
      let expected = TRIVECTOR_A.geometric_product(VECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_bivector_1() {
      let result = TRIVECTOR_A.scalar_product(BIVECTOR_A);
      let expected = TRIVECTOR_A.geometric_product(BIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_trivector_1() {
      let result = TRIVECTOR_A.scalar_product(TRIVECTOR_B);
      let expected = TRIVECTOR_A.geometric_product(TRIVECTOR_B).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_pseudoscalar_1() {
      let result = TRIVECTOR_A.scalar_product(PSEUDOSCALAR_A);
      let expected = TRIVECTOR_A.geometric_product(PSEUDOSCALAR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }

  mod pseudoscalar {
    use super::*;
    #[test]
    fn scalar_product_multivector_1() {
      let result = PSEUDOSCALAR_A.scalar_product(MULTIVECTOR_A);
      let expected = PSEUDOSCALAR_A.geometric_product(MULTIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_scalar_1() {
      let result = PSEUDOSCALAR_A.scalar_product(SCALAR_A);
      let expected = PSEUDOSCALAR_A.geometric_product(SCALAR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_vector_1() {
      let result = PSEUDOSCALAR_A.scalar_product(VECTOR_A);
      let expected = PSEUDOSCALAR_A.geometric_product(VECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_bivector_1() {
      let result = PSEUDOSCALAR_A.scalar_product(BIVECTOR_A);
      let expected = PSEUDOSCALAR_A.geometric_product(BIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_trivector_1() {
      let result = PSEUDOSCALAR_A.scalar_product(TRIVECTOR_A);
      let expected = PSEUDOSCALAR_A.geometric_product(TRIVECTOR_A).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
    #[test]
    fn scalar_product_pseudoscalar_1() {
      let result = PSEUDOSCALAR_A.scalar_product(PSEUDOSCALAR_B);
      let expected = PSEUDOSCALAR_A.geometric_product(PSEUDOSCALAR_B).grade_0();
      assert_eq!(dbg!(result), dbg!(expected));
    }
  }
}
