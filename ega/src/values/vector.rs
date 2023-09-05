use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct Vector {
  /// The components ordered as `[e0, e1, e2, e3]`
  pub elements: [f32; 4],
}

impl Vector {
  accessors! { pub elements[f32]: e0[0], e1[1], e2[2], e3[3] }
}

impl From<[f32; 4]> for Vector {
  /// Construct a `Vector` from an array containing `[e0, e1, e2, e3]`
  fn from(elements: [f32; 4]) -> Vector {
    Vector { elements }
  }
}

impl Debug for Vector {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt
      .debug_struct("Vector")
      .field("e0", &self.e0())
      .field("e1", &self.e1())
      .field("e2", &self.e2())
      .field("e3", &self.e3())
      .finish()
  }
}