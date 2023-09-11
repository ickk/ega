use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Bivector {
  pub e23: f32,
  pub e31: f32,
  pub e12: f32,
  pub e01: f32,
  pub e02: f32,
  pub e03: f32,
}

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub(crate) struct BivectorArray {
  /// The components ordered as `[e23, e31, e12, e01, e02, e03]`
  pub elements: [f32; 6],
}

impl Bivector {
  #[inline(always)]
  pub(crate) const fn to_bivector_array(self) -> BivectorArray {
    // SAFETY: `Bivector` & `BivectorArray` share identical layout due to
    // repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

impl BivectorArray {
  accessors! { pub elements[f32]: e23[0], e31[1], e12[2], e01[3], e02[4], e03[5] }

  #[inline(always)]
  pub(crate) const fn to_bivector(self) -> Bivector {
    // SAFETY: `Bivector` & `BivectorArray` share identical layout due to
    // repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

impl From<[f32; 6]> for BivectorArray {
  /// Construct a `Biector` from an array containing
  /// `[e23, e31, e12, e01, e02, e03]`
  #[inline]
  fn from(elements: [f32; 6]) -> BivectorArray {
    BivectorArray { elements }
  }
}

impl Debug for Bivector {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    if fmt.alternate() {
      // pretty print
      let width = fmt.width().unwrap_or(8);
      let precision = fmt.precision().unwrap_or(2);
      fmt.write_fmt(format_args!(
        "Bivector {{\n\
        \x20 e23: {e23:width$.precision$},\
        \x20e31: {e31:width$.precision$},\
        \x20e12: {e12:width$.precision$},\n\
        \x20 e01: {e01:width$.precision$},\
        \x20e02: {e02:width$.precision$},\
        \x20e03: {e03:width$.precision$},\n\
        }}",
        e23 = &self.e23,
        e31 = &self.e31,
        e12 = &self.e12,
        e01 = &self.e01,
        e02 = &self.e02,
        e03 = &self.e03,
      ))
    } else {
      fmt
        .debug_struct("Bivector")
        .field("e23", &self.e23)
        .field("e31", &self.e31)
        .field("e12", &self.e12)
        .field("e01", &self.e01)
        .field("e02", &self.e02)
        .field("e03", &self.e03)
        .finish()
    }
  }
}

impl Debug for BivectorArray {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    let width = fmt.width().unwrap_or(8);
    let precision = fmt.precision().unwrap_or(2);
    if fmt.alternate() {
      // pretty print
      fmt.write_fmt(format_args!(
        "Bivector [\n\
        \x20 e23: {e23:width$.precision$},\
        \x20e31: {e31:width$.precision$},\
        \x20e12: {e12:width$.precision$},\n\
        \x20 e01: {e01:width$.precision$},\
        \x20e02: {e02:width$.precision$},\
        \x20e03: {e03:width$.precision$},\n\
        ]",
        e23 = &self.e23(),
        e31 = &self.e31(),
        e12 = &self.e12(),
        e01 = &self.e01(),
        e02 = &self.e02(),
        e03 = &self.e03(),
      ))
    } else {
      fmt.write_str("Bivector ")?;
      fmt
        .debug_list()
        .entry(&format_args!("e23: {:width$.precision$}", self.e23()))
        .entry(&format_args!("e31: {:width$.precision$}", self.e31()))
        .entry(&format_args!("e12: {:width$.precision$}", self.e12()))
        .entry(&format_args!("e01: {:width$.precision$}", self.e01()))
        .entry(&format_args!("e02: {:width$.precision$}", self.e02()))
        .entry(&format_args!("e03: {:width$.precision$}", self.e03()))
        .finish()
    }
  }
}
