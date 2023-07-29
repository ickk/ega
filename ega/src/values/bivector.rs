use crate::macros::*;
use core::fmt::{Debug, Formatter};

impl From<(f32, f32, f32, f32, f32, f32)> for Bivector {
  fn from(
    (e23, e31, e12, e01, e02, e03): (f32, f32, f32, f32, f32, f32),
  ) -> Bivector {
    Bivector {
      elements: [e23, e31, e12, e01, e02, e03],
    }
  }
}

#[derive(Clone, PartialEq)]
pub struct Bivector {
  pub(crate) elements: [f32; 6],
}

impl Bivector {
  accessors! { pub elements[f32]: e23[0], e31[1], e12[2], e01[3], e02[4], e03[5] }
}

impl Debug for Bivector {
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
