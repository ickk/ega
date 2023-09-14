use crate::*;
use ::approx::{AbsDiffEq, RelativeEq, UlpsEq};

macro_rules! impl_approx_eq_self {
  ($type:ty: $field_name_1:ident $(,$field_name:ident)* $(,)?) => {
    impl AbsDiffEq for $type {
      type Epsilon = f32;

      fn default_epsilon() -> f32 {
        f32::default_epsilon()
      }

      fn abs_diff_eq(&self, other: &Self, epsilon: f32) -> bool {
        f32::abs_diff_eq(&self.$field_name_1, &other.$field_name_1, epsilon)
        $(
          && f32::abs_diff_eq(&self.$field_name, &other.$field_name, epsilon)
        )*
      }
    }

    impl RelativeEq for $type {
      fn default_max_relative() -> f32 {
        f32::default_max_relative()
      }

      fn relative_eq(
        &self,
        other: &Self,
        epsilon: f32,
        max_relative: f32,
      ) -> bool {
        f32::relative_eq(&self.$field_name_1, &other.$field_name_1, epsilon, max_relative)
        $(
          && f32::relative_eq(&self.$field_name, &other.$field_name,  epsilon, max_relative)
        )*
      }
    }

    impl UlpsEq for $type {
      fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
      }

      fn ulps_eq(&self, other: &Self, epsilon: f32, max_ulps: u32) -> bool {
        f32::ulps_eq(&self.$field_name_1, &other.$field_name_1, epsilon, max_ulps)
        $(
          && f32::ulps_eq(&self.$field_name, &other.$field_name,  epsilon, max_ulps)
        )*
      }
    }
  };
}

impl_approx_eq_self! {
  Multivector: s, e0, e1, e2, e3, e01, e02, e03, e23, e31, e12, e032, e013, e021, e123, e0123
}
impl_approx_eq_self! {
  Scalar: s
}
impl_approx_eq_self! {
  Vector: e0, e1, e2, e3
}
impl_approx_eq_self! {
  Bivector: e01, e02, e03, e23, e31, e12
}
impl_approx_eq_self! {
  Trivector: e032, e013, e021, e123
}
impl_approx_eq_self! {
  Pseudoscalar: e0123
}
