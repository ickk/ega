use super::*;
use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Scalar {
  pub s: f32,
}

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ScalarArray {
  /// The component `[scalar]`
  pub elements: [f32; 1],
}

impl Scalar {
  #[inline(always)]
  pub const fn to_scalar_array(self) -> ScalarArray {
    // SAFETY: `Scalar` & `ScalarArray` have identical layout due to repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

impl ScalarArray {
  accessors! { pub elements[f32]: scalar[0], s[0] }

  #[inline(always)]
  pub const fn to_scalar(self) -> Scalar {
    // SAFETY: `Scalar` & `ScalarArray` have identical layout due to repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

impl From<f32> for Scalar {
  /// Construct a `Scalar` from an f32
  #[inline]
  fn from(scalar: f32) -> Scalar {
    Scalar { s: scalar }
  }
}

impl From<[f32; 1]> for ScalarArray {
  /// Construct a `Scalar` from an array containing `[scalar; 1]`
  #[inline]
  fn from([scalar]: [f32; 1]) -> ScalarArray {
    ScalarArray { elements: [scalar] }
  }
}

impl Debug for Scalar {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt.write_str("Scalar ")?;
    fmt.debug_set().entry(&self.s).finish()
  }
}

impl Debug for ScalarArray {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    let width = fmt.width().unwrap_or(3);
    let precision = fmt.precision().unwrap_or(0);
    fmt.write_fmt(format_args!("Scalar [ {:width$.precision$} ]", &self.s()))
  }
}
