use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Vector {
  pub e0: f32,
  pub e1: f32,
  pub e2: f32,
  pub e3: f32,
}

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub(crate) struct VectorArray {
  /// The components ordered as `[e0, e1, e2, e3]`
  pub elements: [f32; 4],
}

impl Vector {
  #[inline(always)]
  pub(crate) const fn to_vector_array(self) -> VectorArray {
    // SAFETY: `Vector` & `VectorArray` share identical layout due to repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

impl VectorArray {
  accessors! { pub elements[f32]: e0[0], e1[1], e2[2], e3[3] }

  #[inline(always)]
  pub const fn to_vector(self) -> Vector {
    // SAFETY: `Vector` & `VectorArray` share identical layout due to repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

impl From<[f32; 4]> for VectorArray {
  /// Construct a `Vector` from an array containing `[e0, e1, e2, e3]`
  #[inline]
  fn from(elements: [f32; 4]) -> VectorArray {
    VectorArray { elements }
  }
}

impl Debug for VectorArray {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    let width = fmt.width().unwrap_or(8);
    let precision = fmt.precision().unwrap_or(2);
    if fmt.alternate() {
      // pretty print
      fmt.write_fmt(format_args!(
        "Vector [\n\
        \x20 e0: {e0:width$.precision$},\
        \x20e1: {e1:width$.precision$},\
        \x20e2: {e2:width$.precision$},\
        \x20e3: {e3:width$.precision$},\n\
        ]",
        e0 = &self.e0(),
        e1 = &self.e1(),
        e2 = &self.e2(),
        e3 = &self.e3(),
      ))
    } else {
      fmt.write_str("Vector ")?;
      fmt
        .debug_list()
        .entry(&format_args!("e0: {:width$.precision$}", self.e0()))
        .entry(&format_args!("e1: {:width$.precision$}", self.e1()))
        .entry(&format_args!("e2: {:width$.precision$}", self.e2()))
        .entry(&format_args!("e3: {:width$.precision$}", self.e3()))
        .finish()
    }
  }
}
