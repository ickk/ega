use crate::macros::accessors;

pub trait Trivector {
  fn e123(&self) -> f32;
  fn e032(&self) -> f32;
  fn e013(&self) -> f32;
  fn e021(&self) -> f32;
}

#[derive(Clone, Debug)]
pub struct TrivectorVal {
  pub(crate) elements: [f32; 4],
}
impl Trivector for TrivectorVal {
  accessors! { elements[f32]: e123[0], e032[1], e013[2], e021[3] }
}

#[derive(Copy, Clone, Debug)]
pub struct TrivectorRef<'multivector> {
  pub(crate) array_ref: &'multivector [f32; 4],
}
impl Trivector for TrivectorRef<'_> {
  accessors! { array_ref[f32]: e123[0], e032[1], e013[2], e021[3] }
}
