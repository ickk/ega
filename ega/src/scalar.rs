pub trait Scalar {
  fn scalar(&self) -> f32;
}

impl Scalar for f32 {
  fn scalar(&self) -> f32 {
    *self
  }
}
