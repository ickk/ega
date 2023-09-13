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

// I don't think this works in general
// impl Inverse for Bivector {
//   #[inline]
//   fn inverse(self) -> Self {
//     simple_inverse(self)
//   }
// }

impl Inverse for Trivector {
  #[inline]
  fn inverse(self) -> Self {
    simple_inverse(self)
  }
}

// this is valid for elements of a single grade
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

  #[test]
  fn inverse_scalar() {
    let inverse = SCALAR_A.inverse();
    let product = SCALAR_A * inverse;

    assert_eq!(dbg!(Scalar { s: 1.0 }), dbg!(product));
  }

  #[test]
  fn inverse_vector() {
    let inverse = VECTOR_A.inverse();
    let product = VECTOR_A * inverse;

    assert_eq!(dbg!(Multivector { s: 1., ..zero() }), dbg!(product));
  }

  // #[test]
  // fn inverse_bivector() {
  //   let inverse = BIVECTOR_A.inverse();
  //   let product = BIVECTOR_A * inverse;

  //   assert_eq!(dbg!(Multivector { s: 1., ..zero() }), dbg!(product));
  // }

  #[test]
  fn inverse_trivector_1() {
    let inverse = TRIVECTOR_A.inverse();
    let product = TRIVECTOR_A * inverse;

    assert_eq!(dbg!(Multivector { s: 1., ..zero() }), dbg!(product));
  }
}
