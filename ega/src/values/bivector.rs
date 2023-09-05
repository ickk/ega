use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct Bivector {
  /// The components ordered as `[e23, e31, e12, e01, e02, e03]`
  pub elements: [f32; 6],
}

impl Bivector {
  accessors! { pub elements[f32]: e23[0], e31[1], e12[2], e01[3], e02[4], e03[5] }
}

impl From<[f32; 6]> for Bivector {
  /// Construct a `Biector` from an array containing
  /// `[e23, e31, e12, e01, e02, e03]`
  fn from(elements: [f32; 6]) -> Bivector {
    Bivector { elements }
  }
}

impl Debug for Bivector {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    if fmt.alternate() {
      // pretty print
      let width = fmt.width().unwrap_or(3);
      let precision = fmt.precision().unwrap_or(0);
      fmt.write_fmt(format_args!(
        "Bivector {{\n\
        \x20 e23: {e23:width$.precision$},\
        \x20e31: {e31:width$.precision$},\
        \x20e12: {e12:width$.precision$},\n\
        \x20 e01: {e01:width$.precision$},\
        \x20e02: {e02:width$.precision$},\
        \x20e03: {e03:width$.precision$},\n\
        }}",
        e23 = &self.e23(),
        e31 = &self.e31(),
        e12 = &self.e12(),
        e01 = &self.e01(),
        e02 = &self.e02(),
        e03 = &self.e03(),
      ))
    } else {
      fmt
        .debug_struct("Bivector")
        .field("e23", &self.e23())
        .field("e31", &self.e31())
        .field("e12", &self.e12())
        .field("e01", &self.e01())
        .field("e02", &self.e02())
        .field("e03", &self.e03())
        .finish()
    }
  }
}
