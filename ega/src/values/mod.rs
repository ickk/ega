pub mod bivector;
pub mod empty;
pub mod multivector;
pub mod pseudo_scalar;
pub mod reflectors;
pub mod scalar;
pub mod trivector;
pub mod vector;

pub use bivector::*;
pub use empty::*;
pub use multivector::*;
pub use pseudo_scalar::*;
pub use reflectors::*;
pub use scalar::*;
pub use trivector::*;
pub use vector::*;

use crate::operators::*;
pub trait EgaVector:
  private::Sealed
  + Copy
  + Clone
  + core::fmt::Debug
  + Nil
  + Neg
  + GradeSelect
  + Meet<Multivector>
  + Meet<Empty>
  + Meet<Scalar>
  + Meet<Vector>
  + Meet<Bivector>
  + Meet<Trivector>
  + Meet<Pseudoscalar>
{
}

impl EgaVector for Multivector {}
impl EgaVector for Empty {}
impl EgaVector for Scalar {}
impl EgaVector for Vector {}
impl EgaVector for Bivector {}
impl EgaVector for Trivector {}
impl EgaVector for Pseudoscalar {}

impl private::Sealed for Multivector {}
impl private::Sealed for Empty {}
impl private::Sealed for Scalar {}
impl private::Sealed for Vector {}
impl private::Sealed for Bivector {}
impl private::Sealed for Trivector {}
impl private::Sealed for Pseudoscalar {}

mod private {
  pub trait Sealed {}
}

/// Create accessor functions for elements of a field
///
/// usage: `accessors { <pub?> <field>[<type>]: <a1>[<i1>], <a2>[<i2>] }`
///
/// where you want to create accessors `self.a1()`, `self.a2()` that index into
/// `self.field[i1]` & `self.field[i2]`
macro_rules! accessors {
  (
    $visability:vis $field_name:ident[$element_type:ty]: $(
      $accessor_name:ident[$index:literal]
    ),+ $(,)?
  ) => {
    $(
      #[inline]
      $visability fn $accessor_name(&self) -> $element_type {
        self.$field_name[$index]
      }
    )+
  }
}
pub(crate) use accessors;
