use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct Trivector {
  pub(crate) elements: [f32; 4],
}

impl Trivector {
  accessors! { pub elements[f32]: e123[0], e032[1], e013[2], e021[3] }
}

impl From<[f32; 4]> for Trivector {
  fn from([e123, e032, e013, e021]: [f32; 4]) -> Trivector {
    Trivector {
      elements: [e123, e032, e013, e021],
    }
  }
}

impl Debug for Trivector {
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
