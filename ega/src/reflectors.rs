#[derive(Clone, Debug)]
pub struct Rotator {
  pub(crate) _elements: [f32; 4],
}

#[derive(Clone, Debug)]
pub struct Translator {
  pub(crate) _elements: [f32; 4],
}

#[derive(Clone, Debug)]
pub struct Motor {
  pub(crate) _elements: [f32; 8],
}
