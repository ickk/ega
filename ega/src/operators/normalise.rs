use crate::*;

/// Normalise
pub trait Normalise {
  /// The normalised value
  fn normalise(self) -> Self;
}

impl<T> Normalise for T
where
  T: Copy + Norm + IdealNorm + Mul<f32, Output = T> + Add<T, Output = T>
{
  #[inline]
  fn normalise(self) -> Self {
    let ideal_norm = self.ideal_norm();
    let norm = self.norm();
    dbg!(&ideal_norm, &norm);
    self * (1. / norm.s)
  }
}

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

  // #[test]
  // fn normalise_multivector() {
  //   let result = MULTIVECTOR_A.normalise();
  //   let expected = Multivector {
  //       e0:   -2.,   e1:   -3.,   e2:   -5.,    e3:   -7.,
  //        s:  -11.,  e23:  -13.,  e31:  -17.,   e12:  -19.,
  //      e01:  -23.,  e02:  -29.,  e03:  -31., e0123:  -37.,
  //     e123:  -41., e032:  -43., e013:  -47.,  e021:  -53.,
  //   };
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }

  #[test]
  fn normalise_scalar() {
    let result = SCALAR_A.normalise();
    let expected = Scalar { s: 1. };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn normalise_vector() {
    let result = VECTOR_A.normalise();
    let expected = Vector {
      e0: 0.53687042,
      e1: 0.5582030416,
      e2: 0.5795356631,
      e3: 0.593757391,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn normalise_bivector() {
    let result = BIVECTOR_A.normalise();
    let expected = Bivector {
      e01: -0.0029964384,
      e02: 0.0015937436,
      e03: 0.0013381047,
      e12: 0.5841171741,
      e31: 0.5790156722,
      e23: 0.5688127875,
    };
    dbg!(result, expected);
    let result_squared = result.dot(result);
    dbg!(result_squared);
    assert_eq!(dbg!(result), dbg!(expected));
  }

  #[test]
  fn normalise_trivector() {
    let result = TRIVECTOR_A.normalise();
    let expected = Trivector {
      e021: 1.0946372747,
      e013: 1.0630915165,
      e032: 1.0441640615,
      e123: 1.,
    };
    assert_eq!(dbg!(result), dbg!(expected));
  }

  // #[test]
  // fn normalise_pseudoscalar() {
  //   let result = PSEUDOSCALAR_A.normalise();
  //   let expected = Pseudoscalar { e0123: 397. };
  //   assert_eq!(dbg!(result), dbg!(expected));
  // }
}
