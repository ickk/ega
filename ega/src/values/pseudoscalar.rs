use super::*;
use ::core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct Pseudoscalar {
  /// Pseudoscalar component
  pub e0123: f32,
}

#[derive(Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub(crate) struct PseudoscalarArray {
  /// The component `[e0123]`
  pub elements: [f32; 1],
}

impl Pseudoscalar {
  pub const UNIT: Pseudoscalar = Pseudoscalar { e0123: 1.0 };

  #[allow(unused)]
  #[inline(always)]
  pub(crate) const fn to_pseudoscalar_array(self) -> PseudoscalarArray {
    // SAFETY: `Pseudoscalar` & `PseudoscalarArray` share identical layout due
    // to repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

#[allow(unused)]
impl PseudoscalarArray {
  accessors! { pub elements[f32]: pseudoscalar[0], e0123[0] }

  #[inline(always)]
  pub const fn to_pseudoscalar(self) -> Pseudoscalar {
    // SAFETY: `Pseudoscalar` & `PseudoscalarArray` share identical layout due
    // to repr(C)
    unsafe { core::mem::transmute(self) }
  }
}

impl From<f32> for Pseudoscalar {
  /// Construct a `Pseudoscalar` from an f32
  #[inline]
  fn from(e0123: f32) -> Pseudoscalar {
    Pseudoscalar { e0123 }
  }
}

impl From<[f32; 1]> for PseudoscalarArray {
  /// Construct a `Pseudoscalar` from an array containing `[e0123]`
  #[inline]
  fn from([e0123]: [f32; 1]) -> PseudoscalarArray {
    PseudoscalarArray { elements: [e0123] }
  }
}

impl Debug for Pseudoscalar {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt.write_str("PseudoScalar ")?;
    fmt.debug_set().entry(&self.e0123).finish()
  }
}

impl Debug for PseudoscalarArray {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    let width = fmt.width().unwrap_or(8);
    let precision = fmt.precision().unwrap_or(2);
    fmt.write_fmt(format_args!(
      "Pseudoscalar [ {:width$.precision$} ]",
      self.e0123()
    ))
  }
}
