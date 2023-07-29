pub trait PseudoScalar {
  fn e0123(&self) -> f32;

  fn pseudo_scalar(&self) -> f32 {
    self.e0123()
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PseudoScalarVal {
  pub e0123: f32,
}

impl PseudoScalar for PseudoScalarVal {
  fn e0123(&self) -> f32 {
    self.e0123
  }
}

impl From<f32> for PseudoScalarVal {
  fn from(e0123: f32) -> PseudoScalarVal {
    PseudoScalarVal {
      e0123
    }
  }
}
impl std::ops::Neg for PseudoScalarVal {
  type Output = PseudoScalarVal;
  #[inline]
  fn neg(self) -> Self::Output {
    PseudoScalarVal {
      e0123: -self.e0123,
    }
  }
}
