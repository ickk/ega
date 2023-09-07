use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Trivector {
  pub e123: f32,
  pub e032: f32,
  pub e013: f32,
  pub e021: f32,
}

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub(crate) struct TrivectorArray {
  /// The components ordered as `[e123, e032, e013, e021]`
  pub elements: [f32; 4],
}

impl Trivector {
  #[inline(always)]
  pub(crate) const fn to_trivector_array(self) -> TrivectorArray {
    // SAFETY: `Trivector` & `TrivectorArray` have identical layout due to
    // repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

impl TrivectorArray {
  accessors! { pub elements[f32]: e123[0], e032[1], e013[2], e021[3] }

  #[inline(always)]
  pub const fn to_trivector(self) -> Trivector {
    // SAFETY: `Trivector` & `TrivectorArray` have identical layout due to
    // repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

impl From<[f32; 4]> for TrivectorArray {
  /// Construct a `Trivector` from an array containing
  /// `[e123, e032, e013, e021]`
  #[inline]
  fn from(elements: [f32; 4]) -> TrivectorArray {
    TrivectorArray { elements }
  }
}

impl Debug for TrivectorArray {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    let width = fmt.width().unwrap_or(3);
    let precision = fmt.precision().unwrap_or(0);
    if fmt.alternate() {
      // pretty print
      fmt.write_fmt(format_args!(
        "Trivector [\n\
        \x20 e123: {e123:width$.precision$},\
        \x20e032: {e032:width$.precision$},\
        \x20e013: {e013:width$.precision$},\
        \x20e021: {e021:width$.precision$},\n\
        ]",
        e123 = &self.e123(),
        e032 = &self.e032(),
        e013 = &self.e013(),
        e021 = &self.e021(),
      ))
    } else {
      fmt.write_str("Trivector ")?;
      fmt
        .debug_list()
        .entry(&format_args!("e123: {:width$.precision$}", self.e123()))
        .entry(&format_args!("e032: {:width$.precision$}", self.e032()))
        .entry(&format_args!("e013: {:width$.precision$}", self.e013()))
        .entry(&format_args!("e021: {:width$.precision$}", self.e021()))
        .finish()
    }
  }
}
