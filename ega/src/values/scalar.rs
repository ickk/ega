use super::*;
use core::{
  fmt::{Debug, Formatter},
  ops::Deref,
};

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Scalar {
  /// Scalar component
  pub s: f32,
}

impl Scalar {
  pub const UNIT: Scalar = Scalar { s: 1.0 };

  #[inline]
  pub fn sqrt(self) -> Scalar {
    Scalar { s: self.s.sqrt() }
  }

  #[inline]
  pub fn abs(self) -> Scalar {
    Scalar { s: self.s.abs() }
  }
}

impl Deref for Scalar {
  type Target = f32;

  #[inline]
  fn deref(&self) -> &f32 {
    &self.s
  }
}

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub(crate) struct ScalarArray {
  /// The component `[scalar]`
  pub elements: [f32; 1],
}

impl Scalar {
  #[allow(unused)]
  #[inline(always)]
  pub(crate) const fn to_scalar_array(self) -> ScalarArray {
    // SAFETY: `Scalar` & `ScalarArray` have identical layout due to repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

#[allow(unused)]
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
    let width = fmt.width().unwrap_or(8);
    let precision = fmt.precision().unwrap_or(2);
    fmt.write_fmt(format_args!("Scalar [ {:width$.precision$} ]", &self.s()))
  }
}
