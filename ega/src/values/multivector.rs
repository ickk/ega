use crate::macros::*;

#[derive(Clone, Debug)]
pub struct Multivector {
  pub(crate) elements: [f32; 16],
}

// TODO: These should really be called "VectorAccessors"
impl Multivector {
  accessors! { pub elements[f32]:
    e0[0], e1[1], e2[2], e3[3],
    scalar[4], e23[5], e31[6], e12[7],
    e01[8], e02[9], e03[10], e0123[11],
    e123[12], e032[13], e013[14], e021[15],
  }
}

// TODO: This should actually be a grade selection operator
// impl AsVector for MultivectorVal {
//   sub_val_constructors! {
//     Self.elements {
//       vector_owned -> VectorVal.elements[0..=3],
//       vector -> VectorRef.array_ref[0..=3],
//     }
//   }
// }

// impl MultivectorVal {
//   sub_val_constructors! {
//     pub Self.elements {
//       bivector_owned -> BivectorVal.elements[5..=10],
//       bivector -> BivectorRef.array_ref[5..=10],

//       trivector_owned -> TrivectorVal.elements[12..=15],
//       trivector -> TrivectorRef.array_ref[12..=15],

//       rotator_owned -> RotatorVal._elements[4..=7],
//       rotator -> RotatorRef._array_ref[4..=7],

//       translator_owned -> TranslatorVal._elements[8..=11],
//       translator -> TranslatorRef._array_ref[8..=11],

//       motor_owned -> MotorVal._elements[4..=11],
//       motor -> MotorRef._array_ref[4..=11],
//     }
//   }
// }
