use crate::bivector::*;
use crate::macros::accessors;
use crate::operators::*;

pub trait Vector {
  fn e0(&self) -> f32;
  fn e1(&self) -> f32;
  fn e2(&self) -> f32;
  fn e3(&self) -> f32;
}

impl<V: Vector, Rhs: Vector> Meet<Rhs> for V {
  type Output = BivectorVal;

  #[inline]
  fn meet(&self, rhs: Rhs) -> Self::Output {
    let (p, q) = (self, rhs);
    let e23 = p.e2() * q.e3() - p.e3() * q.e2();
    let e31 = p.e3() * q.e1() - p.e1() * q.e3();
    let e12 = p.e1() * q.e2() - p.e2() * q.e1();
    let e01 = p.e0() * q.e1() - p.e1() * q.e0();
    let e02 = p.e0() * q.e2() - p.e2() * q.e0();
    let e03 = p.e0() * q.e3() - p.e3() * q.e0();
    BivectorVal {
      elements: [e23, e31, e12, e01, e02, e03],
    }
  }
}

#[derive(Clone, Debug)]
pub struct VectorVal {
  pub(crate) elements: [f32; 4],
}
impl Vector for VectorVal {
  accessors! { elements[f32]: e0[0], e1[1], e2[2], e3[3] }
}

#[derive(Copy, Clone, Debug)]
pub struct VectorRef<'multivector> {
  pub(crate) array_ref: &'multivector [f32; 4],
}
impl Vector for VectorRef<'_> {
  accessors! { array_ref[f32]: e0[0], e1[1], e2[2], e3[3] }
}
