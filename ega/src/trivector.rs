use crate::macros::*;
use core::fmt::{Debug, Formatter};

pub trait Trivector {
  fn e123(&self) -> f32;
  fn e032(&self) -> f32;
  fn e013(&self) -> f32;
  fn e021(&self) -> f32;
}

impl From<(f32, f32, f32, f32)> for TrivectorVal {
  fn from((e0, e1, e2, e3): (f32, f32, f32, f32)) -> TrivectorVal {
    TrivectorVal {
      elements: [e0, e1, e2, e3],
    }
  }
}

impl Debug for TrivectorVal {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt.write_fmt(format_args!(
      "Trivector {{ e123: {}, e032: {}, e013: {}, e021: {} }}",
      &self.e123(),
      &self.e032(),
      &self.e013(),
      &self.e021(),
    ))
  }
}

#[derive(Clone, PartialEq)]
pub struct TrivectorVal {
  pub(crate) elements: [f32; 4],
}
impl Trivector for TrivectorVal {
  accessors! { elements[f32]: e123[0], e032[1], e013[2], e021[3] }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TrivectorRef<'multivector> {
  pub(crate) array_ref: &'multivector [f32; 4],
}
impl Trivector for TrivectorRef<'_> {
  accessors! { array_ref[f32]: e123[0], e032[1], e013[2], e021[3] }
}

impl std::ops::Neg for TrivectorVal {
  type Output = TrivectorVal;
  #[inline]
  fn neg(self) -> Self::Output {
    TrivectorVal {
      elements: [-self.e123(), -self.e032(), -self.e013(), -self.e021()],
    }
  }
}
impl std::ops::Neg for TrivectorRef<'_> {
  type Output = TrivectorVal;
  #[inline]
  fn neg(self) -> Self::Output {
    TrivectorVal {
      elements: [-self.e123(), -self.e032(), -self.e013(), -self.e021()],
    }
  }
}

pub trait AsTrivector: Trivector {
  fn trivector(&self) -> TrivectorRef;
  fn trivector_owned(&self) -> TrivectorVal;
}
impl AsTrivector for TrivectorVal {
  sub_val_constructors! {
    Self.elements {
      trivector_owned -> TrivectorVal.elements[0..=3],
      trivector -> TrivectorRef.array_ref[0..=3],
    }
  }
}
impl AsTrivector for TrivectorRef<'_> {
  sub_val_constructors! {
    Self.array_ref {
      trivector_owned -> TrivectorVal.elements[0..=3],
      trivector -> TrivectorRef.array_ref[0..=3],
    }
  }
}
