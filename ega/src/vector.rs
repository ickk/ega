use crate::macros::*;
use core::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq)]
pub struct Vector {
  pub(crate) elements: [f32; 4],
}

impl Vector {
  accessors! { pub elements[f32]: e0[0], e1[1], e2[2], e3[3] }
}

impl From<(f32, f32, f32, f32)> for Vector {
  fn from((e0, e1, e2, e3): (f32, f32, f32, f32)) -> Vector {
    Vector {
      elements: [e0, e1, e2, e3],
    }
  }
}

impl Debug for Vector {
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
