use crate::macros::*;
use core::fmt::{Debug, Formatter};

pub trait Bivector {
  fn e23(&self) -> f32;
  fn e31(&self) -> f32;
  fn e12(&self) -> f32;
  fn e01(&self) -> f32;
  fn e02(&self) -> f32;
  fn e03(&self) -> f32;
}

impl From<(f32, f32, f32, f32, f32, f32)> for BivectorVal {
  fn from(
    (e23, e31, e12, e01, e02, e03): (f32, f32, f32, f32, f32, f32),
  ) -> BivectorVal {
    BivectorVal {
      elements: [e23, e31, e12, e01, e02, e03],
    }
  }
}

#[derive(Clone, PartialEq)]
pub struct BivectorVal {
  pub(crate) elements: [f32; 6],
}
impl Bivector for BivectorVal {
  accessors! { elements[f32]: e23[0], e31[1], e12[2], e01[3], e02[4], e03[5] }
}

impl Debug for BivectorVal {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt.write_fmt(format_args!(
      "Bivector {{ (e23: {}, e31: {}, e12: {}) (e01: {}, e02: {}, e03: {}) }}",
      &self.e23(),
      &self.e31(),
      &self.e12(),
      &self.e01(),
      &self.e02(),
      &self.e03()
    ))
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BivectorRef<'multivector> {
  pub(crate) array_ref: &'multivector [f32; 6],
}
impl Bivector for BivectorRef<'_> {
  accessors! { array_ref[f32]: e23[0], e31[1], e12[2], e01[3], e02[4], e03[5] }
}

pub trait AsBivector: Bivector {
  fn bivector(&self) -> BivectorRef;
  fn bivector_owned(&self) -> BivectorVal;
}
impl AsBivector for BivectorVal {
  sub_val_constructors! {
    Self.elements {
      bivector_owned -> BivectorVal.elements[0..=5],
      bivector -> BivectorRef.array_ref[0..=5],
    }
  }
}
impl AsBivector for BivectorRef<'_> {
  sub_val_constructors! {
    Self.array_ref {
      bivector_owned -> BivectorVal.elements[0..=5],
      bivector -> BivectorRef.array_ref[0..=5],
    }
  }
}
