#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PseudoScalar {
  pub(crate) e0123: f32,
}

impl PseudoScalar {
  pub fn e0123(&self) -> f32 {
    self.e0123
  }
}

impl From<f32> for PseudoScalar {
  fn from(e0123: f32) -> PseudoScalar {
    PseudoScalar { e0123 }
  }
}

impl std::ops::Neg for PseudoScalar {
  type Output = PseudoScalar;

  #[inline]
  fn neg(self) -> Self::Output {
    PseudoScalar { e0123: -self.e0123 }
  }
}
