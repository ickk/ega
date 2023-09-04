use core::fmt::{Debug, Formatter};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Empty;

impl From<[f32; 0]> for Empty {
  fn from(_: [f32; 0]) -> Empty {
    Empty {}
  }
}

impl Debug for Empty {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
    fmt.write_fmt(format_args!("Empty"))
  }
}
