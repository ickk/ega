#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Scalar {
  pub(crate) e0123: f32,
}

impl Scalar {
  pub fn e0123(&self) -> f32 {
    self.e0123
  }
}

impl From<f32> for Scalar {
  fn from(e0123: f32) -> Scalar {
    Scalar {
      e0123
    }
  }
}

impl std::ops::Neg for Scalar {
  type Output = Scalar;

  #[inline]
  fn neg(self) -> Self::Output {
    Scalar {
      e0123: -self.e0123,
    }
  }
}
