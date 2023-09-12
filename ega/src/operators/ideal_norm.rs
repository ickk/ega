use crate::*;

/// The Ideal norm, ||A||_inf
///
/// aka "Infinity Norm" or "Vanishing Norm"
pub trait IdealNorm {
  /// The Ideal norm, ||A||_inf
  fn ideal_norm(self) -> Pseudoscalar;
}

// needs sqrt function, so relies on std or libm
#[cfg(any(feature = "std", feature = "libm"))]
impl<T: IdealNormSquared> IdealNorm for T {
  #[inline]
  fn ideal_norm(self) -> Pseudoscalar {
    Pseudoscalar {
      e0123: self.ideal_norm_squared().e0123.sqrt(),
    }
  }
}

pub trait IdealNormSquared {
  fn ideal_norm_squared(self) -> Pseudoscalar;
}

#[cfg(any(test, doctest))]
mod tests {
  // use super::*;
  // use crate::test_values::*;

  // #[test]
  // fn ideal_norm_multivector() {
  //   let result = MULTIVECTOR_A.ideal_norm();
  //
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }

  // #[test]
  // fn ideal_norm_scalar() {
  //   let result = SCALAR_A.ideal_norm();
  //   let expected = Scalar { s: 137. };
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }

  // #[test]
  // fn ideal_norm_vector() {
  //   let result = VECTOR_A.ideal_norm();
  //
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }

  // #[test]
  // fn ideal_norm_bivector() {
  //   let result = BIVECTOR_A.ideal_norm();
  //
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }

  // #[test]
  // fn ideal_norm_trivector() {
  //   let result = TRIVECTOR_A.ideal_norm();
  //
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }

  // #[test]
  // fn ideal_norm_pseudoscalar() {
  //   let result = PSEUDOSCALAR_A.ideal_norm();
  //
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }
}
