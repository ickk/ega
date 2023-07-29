use crate::macros::*;
use core::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq)]
pub struct Trivector {
  pub(crate) elements: [f32; 4],
}

impl Trivector {
  accessors! { pub elements[f32]: e123[0], e032[1], e013[2], e021[3] }
}

impl From<(f32, f32, f32, f32)> for Trivector {
  fn from((e0, e1, e2, e3): (f32, f32, f32, f32)) -> Trivector {
    Trivector {
      elements: [e0, e1, e2, e3],
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

impl std::ops::Neg for Trivector {
  type Output = Trivector;
  #[inline]
  fn neg(self) -> Self::Output {
    Trivector {
      elements: [-self.e123(), -self.e032(), -self.e013(), -self.e021()],
    }
  }
}
