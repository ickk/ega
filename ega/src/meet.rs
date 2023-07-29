use crate::{
  bivector::*,
  multivector::*,
  pseudo_scalar::*,
  trivector::*,
  vector::*,
};

pub trait Meet<Rhs> {
  type Output;

  /// The outer product
  fn meet(&self, rhs: &Rhs) -> Self::Output;
}

#[inline]
fn vector_meet_vector(lhs: &Vector, rhs: &Vector) -> Bivector {
  let (p, q) = (lhs, rhs);
  let e23 = p.e2() * q.e3() - p.e3() * q.e2();
  let e31 = p.e3() * q.e1() - p.e1() * q.e3();
  let e12 = p.e1() * q.e2() - p.e2() * q.e1();
  let e01 = p.e0() * q.e1() - p.e1() * q.e0();
  let e02 = p.e0() * q.e2() - p.e2() * q.e0();
  let e03 = p.e0() * q.e3() - p.e3() * q.e0();
  Bivector {
    elements: [e23, e31, e12, e01, e02, e03],
  }
}

#[inline]
fn vector_meet_bivector(
  lhs: &Vector,
  rhs: &Bivector,
) -> Trivector {
  let (p, l) = (lhs, rhs);
  let e123 = p.e1() * l.e23() + p.e2() * l.e31() + p.e3() * l.e12();
  let e032 = p.e2() * l.e03() - p.e0() * l.e23() - p.e3() * l.e02();
  let e013 = p.e3() * l.e01() - p.e0() * l.e31() - p.e1() * l.e03();
  let e021 = p.e1() * l.e02() - p.e0() * l.e12() - p.e2() * l.e01();
  Trivector {
    elements: [e123, e032, e013, e021],
  }
}
#[inline]
fn bivector_meet_vector(
  lhs: &Bivector,
  rhs: &Vector,
) -> Trivector {
  -vector_meet_bivector(rhs, lhs)
}

#[inline]
fn bivector_meet_bivector(
  lhs: &Bivector,
  rhs: &Bivector,
) -> PseudoScalar {
  let (l, m) = (lhs, rhs);
  let e0123 = l.e01() * m.e23()
    + l.e02() * m.e31()
    + l.e03() * m.e12()
    + l.e23() * m.e01()
    + l.e31() * m.e02()
    + l.e12() * m.e03();
  PseudoScalar { e0123 }
}

#[inline]
fn vector_meet_trivector(
  lhs: &Vector,
  rhs: &Trivector,
) -> PseudoScalar {
  let (p, x) = (lhs, rhs);
  let e0123 = p.e0() * x.e123()
    + p.e1() * x.e032()
    + p.e2() * x.e013()
    + p.e3() * x.e021();
  PseudoScalar { e0123 }
}
#[inline]
fn trivector_meet_vector(
  lhs: &Trivector,
  rhs: &Vector,
) -> PseudoScalar {
  -vector_meet_trivector(rhs, lhs)
}

#[rustfmt::skip]
#[inline]
fn multivector_meet_multivector(
  lhs: &Multivector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let scalar = a.scalar()*b.scalar();
  let e0 = a.scalar()*b.e0() + a.e0()*b.scalar();
  let e1 = a.scalar()*b.e1() + a.e1()*b.scalar();
  let e2 = a.scalar()*b.e2() + a.e2()*b.scalar();
  let e3 = a.scalar()*b.e3() + a.e3()*b.scalar();
  let e23 = a.scalar()*b.e23() + a.e2()*b.e3()
          + a.e23()*b.scalar() - a.e3()*b.e2();
  let e31 = a.scalar()*b.e31() - a.e1()*b.e3()
          + a.e31()*b.scalar() + a.e3()*b.e1();
  let e12 = a.scalar()*b.e12() + a.e1()*b.e2()
          + a.e12()*b.scalar() - a.e2()*b.e1();
  let e01 = a.scalar()*b.e01() + a.e0()*b.e1()
          + a.e01()*b.scalar() - a.e1()*b.e0();
  let e02 = a.scalar()*b.e02() + a.e0()*b.e2()
          + a.e02()*b.scalar() - a.e2()*b.e0();
  let e03 = a.scalar()*b.e03() + a.e0()*b.e3()
          + a.e03()*b.scalar() - a.e3()*b.e0();
  let e123 = a.scalar()*b.e123() + a.e1()*b.e23() + a.e2()*b.e31() + a.e3()*b.e12()
           + a.e123()*b.scalar() + a.e23()*b.e1() + a.e31()*b.e2() + a.e12()*b.e3();
  let e032 = a.scalar()*b.e032() - a.e0()*b.e23() + a.e2()*b.e03() - a.e3()*b.e02()
           + a.e032()*b.scalar() - a.e23()*b.e0() - a.e02()*b.e3() + a.e03()*b.e2();
  let e013 = a.scalar()*b.e013() - a.e0()*b.e31() - a.e1()*b.e03() + a.e3()*b.e01()
           + a.e013()*b.scalar() - a.e31()*b.e0() + a.e01()*b.e3() - a.e03()*b.e1();
  let e021 = a.scalar()*b.e021() - a.e0()*b.e12() + a.e1()*b.e02() - a.e2()*b.e01()
           + a.e021()*b.scalar() - a.e12()*b.e0() - a.e01()*b.e2() + a.e02()*b.e1();
  let pseudo_scalar = a.e0123()*b.scalar() + a.scalar()*b.e0123()
    + a.e0()*b.e123() + a.e1()*b.e032() + a.e2()*b.e013() + a.e3()*b.e021()
    - a.e123()*b.e0() - a.e032()*b.e1() - a.e013()*b.e2() - a.e021()*b.e3()
    + a.e23()*b.e01() + a.e31()*b.e02() + a.e12()*b.e03()
    + a.e01()*b.e23() + a.e02()*b.e31() + a.e03()*b.e12();

  Multivector {
    elements: [
      e0,e1,e2,e3,
      scalar,e23,e31,e12,
      e01,e02,e03,pseudo_scalar,
      e123,e032,e013,e021,
    ],
  }
}

#[rustfmt::skip]
mod impls {
  use super::*;

  macro_rules! impl_meet {
    ($meet_fn:ident: $lhs:ty, $rhs:ty, $output:ty) => {
      impl Meet<$rhs> for $lhs {
        type Output = $output;
        #[inline]
        fn meet(&self, rhs: &$rhs) -> Self::Output {
          $meet_fn(self, rhs)
        }
      }
    };
  }

  // vector ^ vector
  impl_meet! { vector_meet_vector: Vector, Vector, Bivector }
  // vector ^ bivector
  impl_meet! { vector_meet_bivector: Vector, Bivector, Trivector }
  // bivector ^ vector
  impl_meet! { bivector_meet_vector: Bivector, Vector, Trivector }
  // bivector ^ bivector
  impl_meet! { bivector_meet_bivector: Bivector, Bivector, PseudoScalar }
  // vector ^ trivector
  impl_meet! { vector_meet_trivector: Vector, Trivector, PseudoScalar }
  // trivector ^ vector
  impl_meet! { trivector_meet_vector: Trivector, Vector, PseudoScalar }
  // multivector ^ multivector
  impl_meet! { multivector_meet_multivector: Multivector, Multivector, Multivector }
}

#[cfg(any(test, doctest))]
mod tests {
  use super::*;

  #[test]
  fn vector_meet_vector() {
    {
      // null
      let u = Vector::from((0., 0., 0., 0.));
      let v = Vector::from((1., 1., 1., 1.));
      let u_meet_v = u.meet(&v);
      let expected = Bivector::from((0., 0., 0., 0., 0., 0.));
      assert_eq!(u_meet_v, expected)
    }
    {
      // x & y planes meet at a line in z
      let u = Vector::from((0., 1., 0., 0.));
      let v = Vector::from((0., 0., 1., 0.));
      let u_meet_v = u.meet(&v);
      let expected = Bivector::from((0., 0., 1., 0., 0., 0.));
      assert_eq!(u_meet_v, expected)
    }
    {
      // x & z planes meet at a line in -y
      let u = Vector::from((0., 1., 0., 0.));
      let v = Vector::from((0., 0., 0., 1.));
      let u_meet_v = u.meet(&v);
      let expected = Bivector::from((0., -1., 0., 0., 0., 0.));
      assert_eq!(u_meet_v, expected)
    }
    {
      let u = Vector::from((-1., 1., 0., 0.));
      let v = Vector::from((-1., 0., 1., 0.));
      let u_meet_v = u.meet(&v);
      let expected = Bivector::from((0., 0., 1., 1., -1., 0.));
      assert_eq!(u_meet_v, expected)
    }
  }

  // #[test]
  // fn vector_meet_bivector() {
  //   {
  //     // null
  //     let p = VectorVal::from((0., 0., 0., 0.));
  //     let l = BivectorVal::from((0., 0., 0., 0., 0., 0.));
  //     let p_meet_l = p.meet(l);
  //     let expected = TrivectorVal::from((0., 0., 0., 0., 0., 0.));
  //     assert_eq!(u_meet_v, expected)
  //   }
  // }
}
