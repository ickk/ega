use crate::macros::accessors;

pub trait Bivector {
  fn e23(&self) -> f32;
  fn e31(&self) -> f32;
  fn e12(&self) -> f32;
  fn e01(&self) -> f32;
  fn e02(&self) -> f32;
  fn e03(&self) -> f32;
}

#[derive(Clone, Debug)]
pub struct BivectorVal {
  pub(crate) elements: [f32; 6],
}
impl Bivector for BivectorVal {
  accessors! { elements[f32]: e23[0], e31[1], e12[2], e01[3], e02[4], e03[5] }
}

#[derive(Copy, Clone, Debug)]
pub struct BivectorRef<'multivector> {
  pub(crate) array_ref: &'multivector [f32; 6],
}
impl Bivector for BivectorRef<'_> {
  accessors! { array_ref[f32]: e23[0], e31[1], e12[2], e01[3], e02[4], e03[5] }
}
