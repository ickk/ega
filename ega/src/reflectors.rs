#[derive(Copy, Clone, Debug)]
pub struct RotatorRef<'multivector> {
  pub(crate) _array_ref: &'multivector [f32; 4],
}

#[derive(Clone, Debug)]
pub struct RotatorVal {
  pub(crate) _elements: [f32; 4],
}

#[derive(Copy, Clone, Debug)]
pub struct TranslatorRef<'multivector> {
  pub(crate) _array_ref: &'multivector [f32; 4],
}

#[derive(Clone, Debug)]
pub struct TranslatorVal {
  pub(crate) _elements: [f32; 4],
}

#[derive(Copy, Clone, Debug)]
pub struct MotorRef<'multivector> {
  pub(crate) _array_ref: &'multivector [f32; 8],
}

#[derive(Clone, Debug)]
pub struct MotorVal {
  pub(crate) _elements: [f32; 8],
}
