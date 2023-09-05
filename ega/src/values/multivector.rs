use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub struct Multivector {
  /// The components ordered as
  /// ```skip
  /// [
  ///       e0,    e1,    e2,    e3,
  ///   scalar,   e23,   e31,   e12,
  ///      e01,   e02,   e03, e0123,
  ///     e123,  e032,  e013,  e021,
  /// ]
  /// ```
  pub elements: [f32; 16],
}

impl Multivector {
  accessors! { pub elements[f32]:
       e0[0],     e1[1],     e2[2],     e3[3],
        s[4],    e23[5],    e31[6],    e12[7],
      e01[8],    e02[9],   e03[10], e0123[11],
    e123[12],  e032[13],  e013[14],  e021[15],
  }

  // alias for `e0123`
  accessors! { pub elements[f32]: pseudoscalar[11] }
  // alias for `s`
  accessors! { pub elements[f32]: scalar[4] }
}

impl From<[f32; 16]> for Multivector {
  /// Construct a `Multivector` from an array containing
  /// ```skip
  /// [
  ///       e0,    e1,    e2,    e3,
  ///   scalar,   e23,   e31,   e12,
  ///      e01,   e02,   e03, e0123,
  ///     e123,  e032,  e013,  e021,
  /// ]
  /// ```
  fn from(elements: [f32; 16]) -> Multivector {
    Multivector { elements }
  }
}

impl Debug for Multivector {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    if fmt.alternate() {
      // pretty print
      let width = fmt.width().unwrap_or(3);
      let precision = fmt.precision().unwrap_or(0);
      fmt.write_fmt(format_args!(
        "Multivector {{\n\
        \x20     e0: {e0:width$.precision$},\
        \x20   e1: {e1:width$.precision$},\
        \x20   e2: {e2:width$.precision$},\
        \x20   e3: {e3:width$.precision$},\n\
        \x20 scalar: {s:width$.precision$},\
        \x20  e23: {e23:width$.precision$},\
        \x20  e31: {e31:width$.precision$},\
        \x20  e12: {e12:width$.precision$},\n\
        \x20    e01: {e01:width$.precision$},\
        \x20  e02: {e02:width$.precision$},\
        \x20  e03: {e03:width$.precision$},\
        \x20e0123: {e0123:width$.precision$},\n\
        \x20   e123: {e123:width$.precision$},\
        \x20 e032: {e032:width$.precision$},\
        \x20 e013: {e013:width$.precision$},\
        \x20 e021: {e021:width$.precision$},\n\
        }}",
        e0 = &self.e0(),
        e1 = &self.e1(),
        e2 = &self.e2(),
        e3 = &self.e3(),
        s = &self.s(),
        e23 = &self.e23(),
        e31 = &self.e31(),
        e12 = &self.e12(),
        e01 = &self.e01(),
        e02 = &self.e02(),
        e03 = &self.e03(),
        e0123 = &self.e0123(),
        e123 = &self.e123(),
        e032 = &self.e032(),
        e013 = &self.e013(),
        e021 = &self.e021(),
      ))
    } else {
      fmt
        .debug_struct("Multivector")
        .field("e0", &self.e0())
        .field("e1", &self.e1())
        .field("e2", &self.e2())
        .field("e3", &self.e3())
        .field("scalar", &self.s())
        .field("e23", &self.e23())
        .field("e31", &self.e31())
        .field("e12", &self.e12())
        .field("e01", &self.e01())
        .field("e02", &self.e02())
        .field("e03", &self.e03())
        .field("e0123", &self.e0123())
        .field("e123", &self.e123())
        .field("e032", &self.e032())
        .field("e013", &self.e013())
        .field("e021", &self.e021())
        .finish()
    }
  }
}
