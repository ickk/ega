use crate::macros::{accessors, sub_val_constructors};
use core::fmt::{Debug, Formatter};

pub trait Vector {
  fn e0(&self) -> f32;
  fn e1(&self) -> f32;
  fn e2(&self) -> f32;
  fn e3(&self) -> f32;
}

impl From<(f32, f32, f32, f32)> for VectorVal {
  fn from((e0, e1, e2, e3): (f32, f32, f32, f32)) -> VectorVal {
    VectorVal {
      elements: [e0, e1, e2, e3],
    }
  }
}

impl Debug for VectorVal {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt.write_fmt(format_args!(
      "Vector {{ e0: {}, e1: {}, e2: {}, e3: {} }}",
      &self.e0(),
      &self.e1(),
      &self.e2(),
      &self.e3(),
    ))
  }
}

#[derive(Clone, PartialEq)]
pub struct VectorVal {
  pub(crate) elements: [f32; 4],
}
impl Vector for VectorVal {
  accessors! { elements[f32]: e0[0], e1[1], e2[2], e3[3] }
}
impl Vector for &VectorVal {
  accessors! { elements[f32]: e0[0], e1[1], e2[2], e3[3] }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VectorRef<'multivector> {
  pub(crate) array_ref: &'multivector [f32; 4],
}
impl Vector for VectorRef<'_> {
  accessors! { array_ref[f32]: e0[0], e1[1], e2[2], e3[3] }
}

pub trait AsVector: Vector {
  fn vector(&self) -> VectorRef;
  fn vector_owned(&self) -> VectorVal;
}
impl AsVector for VectorVal {
  sub_val_constructors! {
    Self.elements {
      vector_owned -> VectorVal.elements[0..=3],
      vector -> VectorRef.array_ref[0..=3],
    }
  }
}
impl AsVector for VectorRef<'_> {
  sub_val_constructors! {
    Self.array_ref {
      vector_owned -> VectorVal.elements[0..=3],
      vector -> VectorRef.array_ref[0..=3],
    }
  }
}
