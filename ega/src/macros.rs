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

macro_rules! sub_val_constructors {
  (
    $visability:vis Self.$self_field:ident { $(
      $func_name:ident -> $struct_name:ident.$struct_field:ident[$slice:expr]
    ),+ $(,)? }
  ) => {
    $(
      #[inline]
      $visability fn $func_name(&self) -> $struct_name {
        $struct_name {
          $struct_field: unsafe {
            self.$self_field[$slice].try_into().unwrap_unchecked()
          }
        }
      }
    )+
  }
}
pub(crate) use sub_val_constructors;
