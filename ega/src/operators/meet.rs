use crate::values::*;

pub trait Meet<Rhs> {
  type Output;

  /// The outer product
  fn meet(&self, rhs: &Rhs) -> Self::Output;
}

macro_rules! impl_meet {
  ($meet_fn:ident: $lhs:ty, $rhs:ty => $output:ty) => {
    impl Meet<$rhs> for $lhs {
      type Output = $output;
      #[inline]
      fn meet(&self, rhs: &$rhs) -> Self::Output {
        $meet_fn(self, rhs)
      }
    }
  };
}

impl_meet! { multivector_meet_multivector: Multivector, Multivector => Multivector }
impl_meet! { multivector_meet_scalar: Multivector, Scalar => Multivector }
impl_meet! { multivector_meet_vector: Multivector, Vector => Multivector }
impl_meet! { multivector_meet_bivector: Multivector, Bivector => Multivector }
impl_meet! { multivector_meet_trivector: Multivector, Trivector => Multivector }
impl_meet! { multivector_meet_pseudoscalar: Multivector, Pseudoscalar => Pseudoscalar }

impl_meet! { scalar_meet_multivector: Scalar, Multivector => Multivector }
impl_meet! { scalar_meet_scalar: Scalar, Scalar => Scalar }
impl_meet! { scalar_meet_vector: Scalar, Vector => Vector }
impl_meet! { scalar_meet_bivector: Scalar, Bivector => Bivector }
impl_meet! { scalar_meet_trivector: Scalar, Trivector => Trivector }
impl_meet! { scalar_meet_pseudoscalar: Scalar, Pseudoscalar => Pseudoscalar }

impl_meet! { vector_meet_multivector: Vector, Multivector => Multivector }
impl_meet! { vector_meet_scalar: Vector, Scalar => Vector }
impl_meet! { vector_meet_vector: Vector, Vector => Bivector }
impl_meet! { vector_meet_bivector: Vector, Bivector => Trivector }
impl_meet! { vector_meet_trivector: Vector, Trivector => Pseudoscalar }
impl_meet! { vector_meet_pseudoscalar: Vector, Pseudoscalar => Empty }

impl_meet! { bivector_meet_multivector: Bivector, Multivector => Multivector }
impl_meet! { bivector_meet_scalar: Bivector, Scalar => Bivector }
impl_meet! { bivector_meet_vector: Bivector, Vector => Trivector }
impl_meet! { bivector_meet_bivector: Bivector, Bivector => Pseudoscalar }
impl_meet! { bivector_meet_trivector: Bivector, Trivector => Empty }
impl_meet! { bivector_meet_pseudoscalar: Bivector, Pseudoscalar => Empty }

impl_meet! { trivector_meet_multivector: Trivector, Multivector => Multivector }
impl_meet! { trivector_meet_scalar: Trivector, Scalar => Trivector }
impl_meet! { trivector_meet_vector: Trivector, Vector => Pseudoscalar }
impl_meet! { trivector_meet_bivector: Trivector, Bivector => Empty }
impl_meet! { trivector_meet_trivector: Trivector, Trivector => Empty }
impl_meet! { trivector_meet_pseudoscalar: Trivector, Pseudoscalar => Empty }

impl_meet! { pseudoscalar_meet_multivector: Pseudoscalar, Multivector => Pseudoscalar }
impl_meet! { pseudoscalar_meet_scalar: Pseudoscalar, Scalar => Pseudoscalar }
impl_meet! { pseudoscalar_meet_vector: Pseudoscalar, Vector => Empty }
impl_meet! { pseudoscalar_meet_bivector: Pseudoscalar, Bivector => Empty }
impl_meet! { pseudoscalar_meet_trivector: Pseudoscalar, Trivector => Empty }
impl_meet! { pseudoscalar_meet_pseudoscalar: Pseudoscalar, Pseudoscalar => Empty }

// multivector meet ___

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
  let pseudoscalar = a.e0123()*b.scalar() + a.scalar()*b.e0123()
    + a.e0()*b.e123() + a.e1()*b.e032() + a.e2()*b.e013() + a.e3()*b.e021()
    - a.e123()*b.e0() - a.e032()*b.e1() - a.e013()*b.e2() - a.e021()*b.e3()
    + a.e23()*b.e01() + a.e31()*b.e02() + a.e12()*b.e03()
    + a.e01()*b.e23() + a.e02()*b.e31() + a.e03()*b.e12();

  Multivector::from([
    e0, e1, e2, e3,
    scalar, e23, e31, e12,
    e01, e02, e03, pseudoscalar,
    e123, e032, e013, e021,
  ])
}

#[inline]
fn multivector_meet_scalar(lhs: &Multivector, rhs: &Scalar) -> Multivector {
  let elements = lhs.elements.map(|e| e * rhs.scalar());
  Multivector::from(elements)
}

#[rustfmt::skip]
#[inline]
fn multivector_meet_vector(lhs: &Multivector, rhs: &Vector) -> Multivector {
  let (a, b) = (lhs, rhs);

  let scalar = 0f32;

  let e0 = a.scalar()*b.e0();
  let e1 = a.scalar()*b.e1();
  let e2 = a.scalar()*b.e2();
  let e3 = a.scalar()*b.e3();
  let e23 = a.e2()*b.e3() - a.e3()*b.e2();
  let e31 = a.e3()*b.e1() - a.e1()*b.e3();
  let e12 = a.e1()*b.e2() - a.e2()*b.e1();
  let e01 = a.e0()*b.e1() - a.e1()*b.e0();
  let e02 = a.e0()*b.e2() - a.e2()*b.e0();
  let e03 = a.e0()*b.e3() - a.e3()*b.e0();
  let e123 = a.e23()*b.e1() + a.e31()*b.e2() + a.e12()*b.e3();
  let e032 = a.e03()*b.e2() - a.e23()*b.e0() - a.e02()*b.e3();
  let e013 = a.e01()*b.e3() - a.e31()*b.e0() - a.e03()*b.e1();
  let e021 = a.e02()*b.e1() - a.e12()*b.e0() - a.e01()*b.e2();
  let pseudoscalar = -a.e123()*b.e0() - a.e032()*b.e1() - a.e013()*b.e2() - a.e021()*b.e3();

  Multivector::from([
    e0, e1, e2, e3,
    scalar, e23, e31, e12,
    e01, e02, e03, pseudoscalar,
    e123, e032, e013, e021,
  ])
}

#[rustfmt::skip]
#[inline]
fn multivector_meet_bivector(
  lhs: &Multivector,
  rhs: &Bivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let [e0, e1, e2, e3, scalar] = [0f32; 5];

  let e23 = a.scalar()*b.e23();
  let e31 = a.scalar()*b.e31();
  let e12 = a.scalar()*b.e12();
  let e01 = a.scalar()*b.e01();
  let e02 = a.scalar()*b.e02();
  let e03 = a.scalar()*b.e03();
  let e123 = a.e1()*b.e23() + a.e2()*b.e31() + a.e3()*b.e12();
  let e032 = a.e2()*b.e03() - a.e0()*b.e23() - a.e3()*b.e02();
  let e013 = a.e3()*b.e01() - a.e0()*b.e31() - a.e1()*b.e03();
  let e021 = a.e1()*b.e02() - a.e0()*b.e12() - a.e2()*b.e01();
  let pseudoscalar = a.e23()*b.e01() + a.e31()*b.e02() + a.e12()*b.e03()
                   + a.e01()*b.e23() + a.e02()*b.e31() + a.e03()*b.e12();

  Multivector::from([
    e0, e1, e2, e3,
    scalar, e23, e31, e12,
    e01, e02, e03, pseudoscalar,
    e123, e032, e013, e021,
  ])
}

#[rustfmt::skip]
#[inline]
fn multivector_meet_trivector(
  lhs: &Multivector,
  rhs: &Trivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let [e0, e1, e2, e3, scalar, e23, e31, e12, e01, e02, e03] = [0f32; 11];

  let e123 = a.scalar()*b.e123();
  let e032 = a.scalar()*b.e032();
  let e013 = a.scalar()*b.e013();
  let e021 = a.scalar()*b.e021();
  let pseudoscalar = a.e0()*b.e123() + a.e1()*b.e032() + a.e2()*b.e013() + a.e3()*b.e021();

  Multivector::from([
    e0, e1, e2, e3,
    scalar, e23, e31, e12,
    e01, e02, e03, pseudoscalar,
    e123, e032, e013, e021,
  ])
}

#[inline]
fn multivector_meet_pseudoscalar(
  lhs: &Multivector,
  rhs: &Pseudoscalar,
) -> Pseudoscalar {
  Pseudoscalar::from(lhs.scalar() * rhs.e0123())
}

// scalar meet ___
#[inline]
fn scalar_meet_scalar(lhs: &Scalar, rhs: &Scalar) -> Scalar {
  Scalar::from(lhs.scalar() * rhs.scalar())
}

#[inline]
fn scalar_meet_vector(lhs: &Scalar, rhs: &Vector) -> Vector {
  let elements = rhs.elements.map(|e| lhs.scalar() * e);
  Vector::from(elements)
}

#[inline]
fn scalar_meet_bivector(lhs: &Scalar, rhs: &Bivector) -> Bivector {
  let elements = rhs.elements.map(|e| lhs.scalar() * e);
  Bivector::from(elements)
}

#[inline]
fn scalar_meet_trivector(lhs: &Scalar, rhs: &Trivector) -> Trivector {
  let elements = rhs.elements.map(|e| lhs.scalar() * e);
  Trivector::from(elements)
}

#[inline]
fn scalar_meet_pseudoscalar(lhs: &Scalar, rhs: &Pseudoscalar) -> Pseudoscalar {
  Pseudoscalar::from(lhs.scalar() * rhs.pseudoscalar())
}

#[inline]
fn scalar_meet_multivector(lhs: &Scalar, rhs: &Multivector) -> Multivector {
  let elements = rhs.elements.map(|e| lhs.scalar() * e);
  Multivector::from(elements)
}

// vector meet ___

#[inline]
fn vector_meet_scalar(lhs: &Vector, rhs: &Scalar) -> Vector {
  let elements = lhs.elements.map(|e| rhs.scalar() * e);
  Vector::from(elements)
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

  Bivector::from([e23, e31, e12, e01, e02, e03])
}

#[inline]
fn vector_meet_bivector(lhs: &Vector, rhs: &Bivector) -> Trivector {
  let (p, l) = (lhs, rhs);
  let e123 = p.e1() * l.e23() + p.e2() * l.e31() + p.e3() * l.e12();
  let e032 = p.e2() * l.e03() - p.e0() * l.e23() - p.e3() * l.e02();
  let e013 = p.e3() * l.e01() - p.e0() * l.e31() - p.e1() * l.e03();
  let e021 = p.e1() * l.e02() - p.e0() * l.e12() - p.e2() * l.e01();

  Trivector::from([e123, e032, e013, e021])
}

#[inline]
fn vector_meet_trivector(lhs: &Vector, rhs: &Trivector) -> Pseudoscalar {
  let (p, x) = (lhs, rhs);
  let e0123 = p.e0() * x.e123()
    + p.e1() * x.e032()
    + p.e2() * x.e013()
    + p.e3() * x.e021();

  Pseudoscalar::from([e0123])
}

#[inline]
fn vector_meet_pseudoscalar(_: &Vector, _: &Pseudoscalar) -> Empty {
  Empty
}

#[rustfmt::skip]
#[inline]
fn vector_meet_multivector(lhs: &Vector, rhs: &Multivector) -> Multivector {
  let (a, b) = (lhs, rhs);

  let scalar = 0f32;
  let e0 = a.e0()*b.scalar();
  let e1 = a.e1()*b.scalar();
  let e2 = a.e2()*b.scalar();
  let e3 = a.e3()*b.scalar();
  let e23 = a.e2()*b.e3() - a.e3()*b.e2();
  let e31 = a.e3()*b.e1() - a.e1()*b.e3();
  let e12 = a.e1()*b.e2() - a.e2()*b.e1();
  let e01 = a.e0()*b.e1() - a.e1()*b.e0();
  let e02 = a.e0()*b.e2() - a.e2()*b.e0();
  let e03 = a.e0()*b.e3() - a.e3()*b.e0();
  let e123 = a.e1()*b.e23() + a.e2()*b.e31() + a.e3()*b.e12();
  let e032 = a.e2()*b.e03() - a.e0()*b.e23() - a.e3()*b.e02();
  let e013 = a.e3()*b.e01() - a.e0()*b.e31() - a.e1()*b.e03();
  let e021 = a.e1()*b.e02() - a.e0()*b.e12() - a.e2()*b.e01();
  let pseudoscalar = a.e0()*b.e123() + a.e1()*b.e032() + a.e2()*b.e013() + a.e3()*b.e021();

  Multivector::from([
    e0, e1, e2, e3,
    scalar, e23, e31, e12,
    e01, e02, e03, pseudoscalar,
    e123, e032, e013, e021,
  ])
}

// bivector meet ___
#[inline]
fn bivector_meet_scalar(lhs: &Bivector, rhs: &Scalar) -> Bivector {
  let elements = lhs.elements.map(|e| rhs.scalar() * e);
  Bivector::from(elements)
}

// #[inline]
// fn bivector_meet_vector(lhs: &Bivector, rhs: &Vector) -> Trivector {
//   -vector_meet_bivector(rhs, lhs)
// }

#[inline]
fn bivector_meet_vector(lhs: &Bivector, rhs: &Vector) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e123 = a.e23() * b.e1() + a.e31() * b.e2() + a.e12() * b.e3();
  let e032 = a.e03() * b.e2() - a.e23() * b.e0() - a.e02() * b.e3();
  let e013 = a.e01() * b.e3() - a.e31() * b.e0() - a.e03() * b.e1();
  let e021 = a.e02() * b.e1() - a.e12() * b.e0() - a.e01() * b.e2();

  Trivector::from([e123, e032, e013, e021])
}

#[rustfmt::skip]
#[inline]
fn bivector_meet_bivector(lhs: &Bivector, rhs: &Bivector) -> Pseudoscalar {
  let (l, m) = (lhs, rhs);
  let e0123 = l.e01() * m.e23() + l.e02() * m.e31() + l.e03() * m.e12()
            + l.e23() * m.e01() + l.e31() * m.e02() + l.e12() * m.e03();

  Pseudoscalar::from([e0123])
}

#[inline]
fn bivector_meet_trivector(_: &Bivector, _: &Trivector) -> Empty {
  Empty
}

#[inline]
fn bivector_meet_pseudoscalar(_: &Bivector, _: &Pseudoscalar) -> Empty {
  Empty
}

#[rustfmt::skip]
#[inline]
fn bivector_meet_multivector(
  lhs: &Bivector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let [e0, e1, e2, e3, scalar] = [0f32; 5];

  let e23 = a.e23() * b.scalar();
  let e31 = a.e31() * b.scalar();
  let e12 = a.e12() * b.scalar();
  let e01 = a.e01() * b.scalar();
  let e02 = a.e02() * b.scalar();
  let e03 = a.e03() * b.scalar();
  let e123 = a.e23() * b.e1() + a.e31() * b.e2() + a.e12() * b.e3();
  let e032 = a.e03() * b.e2() - a.e23() * b.e0() - a.e02() * b.e3();
  let e013 = a.e01() * b.e3() - a.e31() * b.e0() - a.e03() * b.e1();
  let e021 = a.e02() * b.e1() - a.e12() * b.e0() - a.e01() * b.e2();
  let pseudoscalar = a.e23() * b.e01()
    + a.e31() * b.e02()
    + a.e12() * b.e03()
    + a.e01() * b.e23()
    + a.e02() * b.e31()
    + a.e03() * b.e12();

  Multivector::from([
    e0, e1, e2, e3,
    scalar, e23, e31, e12,
    e01, e02, e03, pseudoscalar,
    e123, e032, e013, e021,
  ])
}

// trivector meet ___

#[inline]
fn trivector_meet_scalar(lhs: &Trivector, rhs: &Scalar) -> Trivector {
  let elements = lhs.elements.map(|e| rhs.scalar() * e);
  Trivector::from(elements)
}

#[inline]
fn trivector_meet_vector(lhs: &Trivector, rhs: &Vector) -> Pseudoscalar {
  -vector_meet_trivector(rhs, lhs)
}

#[inline]
fn trivector_meet_bivector(_: &Trivector, _: &Bivector) -> Empty {
  Empty
}

#[inline]
fn trivector_meet_trivector(_: &Trivector, _: &Trivector) -> Empty {
  Empty
}

#[inline]
fn trivector_meet_pseudoscalar(_: &Trivector, _: &Pseudoscalar) -> Empty {
  Empty
}

#[rustfmt::skip]
#[inline]
fn trivector_meet_multivector(
  lhs: &Trivector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let [e0, e1, e2, e3, scalar, e23, e31, e12, e01, e02, e03] = [0f32; 11];

  let e123 = a.e123()*b.scalar();
  let e032 = a.e032()*b.scalar();
  let e013 = a.e013()*b.scalar();
  let e021 = a.e021()*b.scalar();
  let pseudoscalar = -a.e123()*b.e0() - a.e032()*b.e1() - a.e013()*b.e2() - a.e021()*b.e3();

  Multivector::from([
    e0, e1, e2, e3,
    scalar, e23, e31, e12,
    e01, e02, e03, pseudoscalar,
    e123, e032, e013, e021,
  ])
}

// pseudoscalar meet ___

#[rustfmt::skip]
#[inline]
fn pseudoscalar_meet_multivector(
  lhs: &Pseudoscalar,
  rhs: &Multivector,
) -> Pseudoscalar {
  Pseudoscalar::from(lhs.e0123()*rhs.scalar())
}

#[inline]
fn pseudoscalar_meet_scalar(lhs: &Pseudoscalar, rhs: &Scalar) -> Pseudoscalar {
  Pseudoscalar::from(lhs.pseudoscalar() * rhs.scalar())
}

#[inline]
fn pseudoscalar_meet_vector(_: &Pseudoscalar, _: &Vector) -> Empty {
  Empty
}

#[inline]
fn pseudoscalar_meet_bivector(_: &Pseudoscalar, _: &Bivector) -> Empty {
  Empty
}

#[inline]
fn pseudoscalar_meet_trivector(_: &Pseudoscalar, _: &Trivector) -> Empty {
  Empty
}

#[inline]
fn pseudoscalar_meet_pseudoscalar(
  _: &Pseudoscalar,
  _: &Pseudoscalar,
) -> Empty {
  Empty
}

#[cfg(any(test, doctest))]
mod tests {
  use super::*;

  #[test]
  fn vector_meet_vector() {
    {
      // null
      let u = Vector::from([0., 0., 0., 0.]);
      let v = Vector::from([1., 1., 1., 1.]);
      let u_meet_v = u.meet(&v);
      let expected = Bivector::from([0., 0., 0., 0., 0., 0.]);
      assert_eq!(u_meet_v, expected)
    }
    {
      // x & y planes meet at a line in z
      let u = Vector::from([0., 1., 0., 0.]);
      let v = Vector::from([0., 0., 1., 0.]);
      let u_meet_v = u.meet(&v);
      let expected = Bivector::from([0., 0., 1., 0., 0., 0.]);
      assert_eq!(u_meet_v, expected)
    }
    {
      // x & z planes meet at a line in -y
      let u = Vector::from([0., 1., 0., 0.]);
      let v = Vector::from([0., 0., 0., 1.]);
      let u_meet_v = u.meet(&v);
      let expected = Bivector::from([0., -1., 0., 0., 0., 0.]);
      assert_eq!(u_meet_v, expected)
    }
    {
      let u = Vector::from([-1., 1., 0., 0.]);
      let v = Vector::from([-1., 0., 1., 0.]);
      let u_meet_v = u.meet(&v);
      let expected = Bivector::from([0., 0., 1., 1., -1., 0.]);
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
