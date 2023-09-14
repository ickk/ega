use crate::*;

/// Normalise
pub trait Normalise {
  /// The normalised value
  fn normalise(self) -> Self;
}

impl Normalise for Scalar {
  #[inline]
  fn normalise(self) -> Self {
    simple_normalise(self)
  }
}

impl Normalise for Vector {
  #[inline]
  fn normalise(self) -> Self {
    simple_normalise(self)
  }
}

impl Normalise for Trivector {
  #[inline]
  fn normalise(self) -> Self {
    simple_normalise(self)
  }
}

// not valid in the general case
#[inline]
fn simple_normalise<
  T: Copy + Norm + Mul<f32, Output = T> + Add<T, Output = T>,
>(
  value: T,
) -> T {
  value * (1.0 / value.norm().s)
}

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod tests {
  use super::*;
  use crate::test_values::*;

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
}
