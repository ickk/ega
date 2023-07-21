pub trait PseudoScalar {
  fn e0123(&self) -> f32;

  fn pseudo_scalar(&self) -> f32 {
    self.e0123()
  }
}

impl PseudoScalar for f32 {
  fn e0123(&self) -> f32 {
    *self
  }
}
