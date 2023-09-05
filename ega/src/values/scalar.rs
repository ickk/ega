use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct Scalar {
  /// The component `[scalar; 1]`
  pub(crate) elements: [f32; 1],
}

impl Scalar {
  accessors! { pub elements[f32]: scalar[0] }
}

impl From<f32> for Scalar {
  /// Construct a `Scalar` from an f32
  fn from(scalar: f32) -> Scalar {
    Scalar { elements: [scalar] }
  }
}

impl From<[f32; 1]> for Scalar {
  /// Construct a `Scalar` from an array containing `[scalar; 1]`
  fn from([scalar]: [f32; 1]) -> Scalar {
    Scalar { elements: [scalar] }
  }
}

impl Debug for Scalar {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt.write_fmt(format_args!("Scalar {{ {} }}", &self.scalar()))
  }
}
