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

/// The Ideal norm, ||A||_inf
///
/// aka "Infinity Norm" or "Vanishing Norm"
pub trait IdealNorm {
  /// The Ideal norm, ||A||_inf
  fn ideal_norm(self) -> Scalar;
}

// needs sqrt function, so relies on std or libm
#[cfg(any(feature = "std", feature = "libm"))]
impl<T, O> IdealNorm for T
where
  T: HodgeDual<Output = O>,
  O: NormSquared,
{
  #[inline]
  fn ideal_norm(self) -> Scalar {
    Scalar {
      s: self.hodge_dual().norm_squared().s.abs().sqrt(),
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

impl<T, O> NormSquared for T
where
  T: Copy + Reverse + Conjugate + GeometricProduct<T, Output = O>,
  O: GradeSelect + Reverse,
{
  #[inline]
  fn norm_squared(self) -> Scalar {
    self.geometric_product(self.reverse()).grade_0()
  }
}

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

  mod norm {
    use super::*;

    // #[test]
    // fn norm_multivector() {
    //   let result = MULTIVECTOR_A.norm();
    //   let expected = Multivector {
    //       e0:   -2.,   e1:   -3.,   e2:   -5.,    e3:   -7.,
    //        s:  -11.,  e23:  -13.,  e31:  -17.,   e12:  -19.,
    //      e01:  -23.,  e02:  -29.,  e03:  -31., e0123:  -37.,
    //     e123:  -41., e032:  -43., e013:  -47.,  e021:  -53.,
    //   };
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
    //   let expected = Vector { e0: -151., e1: -157., e2: -163., e3: -167. };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }

    // #[test]
    // fn norm_bivector() {
    //   let result = BIVECTOR_A.norm();
    //   let expected = Bivector {
    //     e23: -223., e31: -227., e12: -229.,
    //     e01: -233., e02: -239., e03: -241.,
    //   };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }

    // #[test]
    // fn norm_trivector() {
    //   let result = TRIVECTOR_A.norm();
    //   let expected = Trivector { e021: -347., e013: -337., e032: -331., e123: -317.};
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }

    // #[test]
    // fn norm_pseudoscalar() {
    //   let result = PSEUDOSCALAR_A.norm();
    //   let expected = Pseudoscalar { e0123: -397. };
    //   assert_eq!(dbg!(result), dbg!(expected));
    // }
  }
}
