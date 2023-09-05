use crate::values::*;

pub trait Dot<Rhs> {
  type Output;

  /// The inner product
  fn dot(&self, rhs: &Rhs) -> Self::Output;
}

macro_rules! impl_dot {
  ($dot_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl Dot<$rhs> for $lhs {
      type Output = $output;
      #[inline]
      fn dot(&self, rhs: &$rhs) -> Self::Output {
        $dot_fn(self, rhs)
      }
    }
  };
}

impl_dot! { multivector_dot_multivector: Multivector, Multivector => Multivector }

// Multivector

#[rustfmt::skip]
#[inline]
fn multivector_dot_multivector(
  lhs: &Multivector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let scalar = a.s()*b.s() - a.e123()*b.e123()
             + a.e1()*b.e1() + a.e2()*b.e2() + a.e3()*b.e3()
             - a.e23()*b.e23() - a.e31()*b.e31() - a.e12()*b.e12();
  let e0 = a.s()*b.e0() + a.e0()*b.s()
         + a.e123()*b.e0123() - a.e0123()*b.e123()
         + a.e01()*b.e1() + a.e02()*b.e2() + a.e03()*b.e3()
         - a.e1()*b.e01() - a.e2()*b.e02() - a.e3()*b.e03()
         + a.e23()*b.e032() + a.e31()*b.e013() + a.e12()*b.e021()
         + a.e032()*b.e23() + a.e013()*b.e31() + a.e021()*b.e12();
  let e1 = a.s()*b.e1() + a.e1()*b.s()
         + a.e3()*b.e31() + a.e12()*b.e2()
         - a.e2()*b.e12() - a.e31()*b.e3()
         - a.e123()*b.e23() - a.e23()*b.e123();
  let e2 = a.s()*b.e2() + a.e2()*b.s()
         + a.e1()*b.e12() + a.e23()*b.e3()
         - a.e3()*b.e23() - a.e12()*b.e1()
         - a.e123()*b.e31() - a.e31()*b.e123();
  let e3 = a.s()*b.e3() + a.e3()*b.s()
         + a.e2()*b.e23() + a.e31()*b.e1()
         - a.e1()*b.e31() - a.e23()*b.e2()
         - a.e123()*b.e12() - a.e12()*b.e123();
  let e01 = a.s()*b.e01() + a.e01()*b.s()
          + a.e3()*b.e013() + a.e013()*b.e3()
          - a.e2()*b.e021() - a.e021()*b.e2()
          - a.e0123()*b.e23() - a.e23()*b.e0123();
  let e02 = a.s()*b.e02() + a.e02()*b.s()
          + a.e2()*b.e021() + a.e021()*b.e1()
          - a.e3()*b.e032() - a.e032()*b.e3()
          - a.e0123()*b.e31() - a.e31()*b.e0123();
  let e03 = a.s()*b.e03() + a.e03()*b.s()
          + a.e2()*b.e032() + a.e032()*b.e2()
          - a.e1()*b.e013() - a.e013()*b.e1()
          - a.e0123()*b.e12() - a.e12()*b.e0123();
  let e23 = a.s()*b.e23() + a.e23()*b.s() + a.e1()*b.e123() + a.e123()*b.e1();
  let e31 = a.s()*b.e31() + a.e31()*b.s() + a.e2()*b.e123() + a.e123()*b.e2();
  let e12 = a.s()*b.e12() + a.e12()*b.s() + a.e3()*b.e123() + a.e123()*b.e3();
  let e123 = a.s()*b.e123() + a.e123()*b.s();
  let e032 = a.s()*b.e032() + a.e032()*b.s() + a.e1()*b.e0123() - a.e0123()*b.e1();
  let e013 = a.s()*b.e013() + a.e013()*b.s() + a.e2()*b.e0123() - a.e0123()*b.e2();
  let e021 = a.s()*b.e021() + a.e021()*b.s() + a.e3()*b.e0123() - a.e0123()*b.e3();
  let e0123 = a.s()*b.e0123() + a.e0123()*b.s();

  Multivector::from([
        e0,    e1,    e2,    e3,
    scalar,   e23,   e31,   e12,
       e01,   e02,   e03, e0123,
      e123,  e032,  e013,  e021,
  ])
}
