use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub struct Multivector {
  pub(crate) elements: [f32; 16],
}

impl Multivector {
  accessors! { pub elements[f32]:
    e0[0], e1[1], e2[2], e3[3],
    scalar[4], e23[5], e31[6], e12[7],
    e01[8], e02[9], e03[10], e0123[11],
    e123[12], e032[13], e013[14], e021[15],
  }

  accessors! { pub elements[f32]: pseudoscalar[11] }
}

impl From<[f32; 16]> for Multivector {
  #[rustfmt::skip]
  fn from(
    [
      e0, e1, e2, e3,
      scalar, e23, e31, e12,
      e01, e02, e03, e0123,
      e123, e032, e013, e021
    ]: [f32; 16],
  ) -> Multivector {
    Multivector {
      elements: [
        e0, e1, e2, e3,
        scalar, e23, e31, e12,
        e01, e02, e03, e0123,
        e123, e032, e013, e021,
      ],
    }
  }
}

impl Debug for Multivector {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt.write_fmt(format_args!(
      "Multivector {{ \
        (e0: {e0}, e1: {e1}, e2: {e2}, e3: {e3}) \
        (scalar: {scalar} e23: {e23}, e31: {e31}, e12: {e12}) \
        (e01: {e01}, e02: {e02}, e03: {e03}, e0123: {e0123}) \
        (e123: {e123}, e032: {e032}, e013: {e013}, e021: {e021}) \
      }}",
      e0 = &self.e0(),
      e1 = &self.e1(),
      e2 = &self.e2(),
      e3 = &self.e3(),
      scalar = &self.scalar(),
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
  }
}
