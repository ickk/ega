use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct Pseudoscalar {
  /// The component `[e0123; 1]`
  pub(crate) elements: [f32; 1],
}

impl Pseudoscalar {
  accessors! { pub elements[f32]: pseudoscalar[0], e0123[0] }
}

impl From<f32> for Pseudoscalar {
  /// Construct a `Pseudoscalar` from an f32
  fn from(e0123: f32) -> Pseudoscalar {
    Pseudoscalar { elements: [e0123] }
  }
}

impl From<[f32; 1]> for Pseudoscalar {
  /// Construct a `Pseudoscalar` from an array containing `[e0123; 1]`
  fn from([e0123]: [f32; 1]) -> Pseudoscalar {
    Pseudoscalar { elements: [e0123] }
  }
}

impl Debug for Pseudoscalar {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt.write_fmt(format_args!("Pseudoscalar {{ {} }}", &self.e0123()))
  }
}
