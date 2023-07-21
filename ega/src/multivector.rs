use crate::macros::{accessors, sub_val_constructors};
use crate::{
  bivector::*, pseudo_scalar::*, reflectors::*, scalar::*, trivector::*,
  vector::*,
};

#[derive(Clone, Debug)]
pub struct MultivectorVal {
  elements: [f32; 16],
}

impl Vector for MultivectorVal {
  accessors! { elements[f32]: e0[0], e1[1], e2[2], e3[3] }
}
impl Scalar for MultivectorVal {
  accessors! { elements[f32]: scalar[4] }
}
impl Bivector for MultivectorVal {
  accessors! {
    elements[f32]: e23[5], e31[6], e12[7], e01[8], e02[9], e03[10]
  }
}
impl PseudoScalar for MultivectorVal {
  accessors! { elements[f32]: e0123[11] }
}
impl Trivector for MultivectorVal {
  accessors! { elements[f32]: e123[12], e032[13], e013[14], e021[15] }
}

impl MultivectorVal {
  sub_val_constructors! {
    pub Self.elements {
      vector_owned -> VectorVal.elements[0..=3],
      vector -> VectorRef.array_ref[0..=3],
      plane_owned -> PlaneVal.elements[0..=3],
      plane -> PlaneRef.array_ref[0..=3],

      bivector_owned -> BivectorVal.elements[5..=10],
      bivector -> BivectorRef.array_ref[5..=10],
      line_owned -> LineVal.elements[5..=10],
      line -> LineRef.array_ref[5..=10],

      trivector_owned -> TrivectorVal.elements[12..=15],
      trivector -> TrivectorRef.array_ref[12..=15],
      point_owned -> PointVal.elements[12..=15],
      point -> PointRef.array_ref[12..=15],

      rotator_owned -> RotatorVal._elements[4..=7],
      rotator -> RotatorRef._array_ref[4..=7],

      translator_owned -> TranslatorVal._elements[8..=11],
      translator -> TranslatorRef._array_ref[8..=11],

      motor_owned -> MotorVal._elements[4..=11],
      motor -> MotorRef._array_ref[4..=11],
    }
  }
}
