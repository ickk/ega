use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct Trivector {
  /// The components ordered as `[e123, e032, e013, e021]`
  pub elements: [f32; 4],
}

impl Trivector {
  accessors! { pub elements[f32]: e123[0], e032[1], e013[2], e021[3] }
}

impl From<[f32; 4]> for Trivector {
  /// Construct a `Trivector` from an array containing
  /// `[e123, e032, e013, e021]`
  fn from(elements: [f32; 4]) -> Trivector {
    Trivector { elements }
  }
}

impl Debug for Trivector {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt
      .debug_struct("Trivector")
      .field("e123", &self.e123())
      .field("e032", &self.e032())
      .field("e013", &self.e013())
      .field("e021", &self.e021())
      .finish()
  }
}
