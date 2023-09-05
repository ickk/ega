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
impl_dot! { multivector_dot_scalar: Multivector, Scalar => Multivector }
impl_dot! { multivector_dot_vector: Multivector, Vector => Multivector }
impl_dot! { multivector_dot_bivector: Multivector, Bivector => Multivector }
impl_dot! { multivector_dot_trivector: Multivector, Trivector => Multivector }
impl_dot! { multivector_dot_pseudoscalar: Multivector, Pseudoscalar => Multivector }

impl_dot! { scalar_dot_multivector: Scalar, Multivector => Multivector }
impl_dot! { scalar_dot_scalar: Scalar, Scalar => Scalar }
impl_dot! { scalar_dot_vector: Scalar, Vector => Vector }
impl_dot! { scalar_dot_bivector: Scalar, Bivector => Bivector }
impl_dot! { scalar_dot_trivector: Scalar, Trivector => Trivector }
impl_dot! { scalar_dot_pseudoscalar: Scalar, Pseudoscalar => Pseudoscalar }

impl_dot! { vector_dot_multivector: Vector, Multivector => Multivector }
impl_dot! { vector_dot_scalar: Vector, Scalar => Vector }
impl_dot! { vector_dot_vector: Vector, Vector => Scalar }
impl_dot! { vector_dot_bivector: Vector, Bivector => Vector }
impl_dot! { vector_dot_trivector: Vector, Trivector => Bivector }
impl_dot! { vector_dot_pseudoscalar: Vector, Pseudoscalar => Trivector }

impl_dot! { bivector_dot_multivector: Bivector, Multivector => Multivector }
impl_dot! { bivector_dot_scalar: Bivector, Scalar => Bivector }
impl_dot! { bivector_dot_vector: Bivector, Vector => Vector }
impl_dot! { bivector_dot_bivector: Bivector, Bivector => Scalar }
impl_dot! { bivector_dot_trivector: Bivector, Trivector => Vector }
impl_dot! { bivector_dot_pseudoscalar: Bivector, Pseudoscalar => Bivector }

impl_dot! { trivector_dot_multivector: Trivector, Multivector => Multivector }
impl_dot! { trivector_dot_scalar: Trivector, Scalar => Trivector }
impl_dot! { trivector_dot_vector: Trivector, Vector => Bivector }
impl_dot! { trivector_dot_bivector: Trivector, Bivector => Vector }
impl_dot! { trivector_dot_trivector: Trivector, Trivector => Scalar }
impl_dot! { trivector_dot_pseudoscalar: Trivector, Pseudoscalar => Vector }

// impl_dot! { pseudoscalar_dot_multivector: Pseudoscalar, Multivector => Pseudoscalar }
// impl_dot! { pseudoscalar_dot_scalar: Pseudoscalar, Scalar => Pseudoscalar }
// impl_dot! { pseudoscalar_dot_vector: Pseudoscalar, Vector => Empty }
// impl_dot! { pseudoscalar_dot_bivector: Pseudoscalar, Bivector => Empty }
// impl_dot! { pseudoscalar_dot_trivector: Pseudoscalar, Trivector => Empty }
// impl_dot! { pseudoscalar_dot_pseudoscalar: Pseudoscalar, Pseudoscalar => Empty }

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
          + a.e1()*b.e021() + a.e021()*b.e1()
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

#[inline]
fn multivector_dot_scalar(lhs: &Multivector, rhs: &Scalar) -> Multivector {
  Multivector::from(lhs.elements.map(|e| e * rhs.scalar()))
}

#[rustfmt::skip]
#[inline]
fn multivector_dot_vector(
  lhs: &Multivector,
  rhs: &Vector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let scalar = a.e1()*b.e1() + a.e2()*b.e2() + a.e3()*b.e3();
  let e0 = a.s()*b.e0() + a.e01()*b.e1() + a.e02()*b.e2() + a.e03()*b.e3();
  let e1 = a.s()*b.e1() + a.e12()*b.e2() - a.e31()*b.e3();
  let e2 = a.s()*b.e2() + a.e23()*b.e3() - a.e12()*b.e1();
  let e3 = a.s()*b.e3() + a.e31()*b.e1() - a.e23()*b.e2();
  let e01 = a.e013()*b.e3() - a.e021()*b.e2();
  let e02 = a.e021()*b.e1() - a.e032()*b.e3();
  let e03 = a.e032()*b.e2() - a.e013()*b.e1();
  let e23 = a.e123()*b.e1();
  let e31 = a.e123()*b.e2();
  let e12 = a.e123()*b.e3();
  let e123 = 0f32;
  let e032 = -a.e0123()*b.e1();
  let e013 = -a.e0123()*b.e2();
  let e021 = -a.e0123()*b.e3();
  let e0123 = 0f32;

  Multivector::from([
        e0,    e1,    e2,    e3,
    scalar,   e23,   e31,   e12,
       e01,   e02,   e03, e0123,
      e123,  e032,  e013,  e021,
  ])
}

#[rustfmt::skip]
#[inline]
fn multivector_dot_bivector(
  lhs: &Multivector,
  rhs: &Bivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e123, e032, e013, e021, e0123] = [0f32; 5];

  let scalar = -a.e23()*b.e23() - a.e31()*b.e31() - a.e12()*b.e12();
  let e0 = -a.e1()*b.e01() - a.e2()*b.e02() - a.e3()*b.e03()
         + a.e032()*b.e23() + a.e013()*b.e31() + a.e021()*b.e12();
  let e1 = a.e3()*b.e31() - a.e2()*b.e12() - a.e123()*b.e23();
  let e2 = a.e1()*b.e12() - a.e3()*b.e23() - a.e123()*b.e31();
  let e3 = a.e2()*b.e23() - a.e1()*b.e31() - a.e123()*b.e12();
  let e01 = a.s()*b.e01() - a.e0123()*b.e23();
  let e02 = a.s()*b.e02() - a.e0123()*b.e31();
  let e03 = a.s()*b.e03() - a.e0123()*b.e12();
  let e23 = a.s()*b.e23();
  let e31 = a.s()*b.e31();
  let e12 = a.s()*b.e12();

  Multivector::from([
        e0,    e1,    e2,    e3,
    scalar,   e23,   e31,   e12,
       e01,   e02,   e03, e0123,
      e123,  e032,  e013,  e021,
  ])
}

#[rustfmt::skip]
#[inline]
fn multivector_dot_trivector(
  lhs: &Multivector,
  rhs: &Trivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let scalar = -a.e123()*b.e123();
  let e0 = a.e23()*b.e032() + a.e31()*b.e013() + a.e12()*b.e021() - a.e0123()*b.e123();
  let e1 = -a.e23()*b.e123();
  let e2 = -a.e31()*b.e123();
  let e3 = -a.e12()*b.e123();
  let e01 = a.e3()*b.e013() - a.e2()*b.e021();
  let e02 = a.e1()*b.e021() - a.e3()*b.e032();
  let e03 = a.e2()*b.e032() - a.e1()*b.e013();
  let e23 = a.e1()*b.e123();
  let e31 = a.e2()*b.e123();
  let e12 = a.e3()*b.e123();
  let e123 = a.s()*b.e123();
  let e032 = a.s()*b.e032();
  let e013 = a.s()*b.e013();
  let e021 = a.s()*b.e021();
  let e0123 = 0f32;

  Multivector::from([
        e0,    e1,    e2,    e3,
    scalar,   e23,   e31,   e12,
       e01,   e02,   e03, e0123,
      e123,  e032,  e013,  e021,
  ])
}

#[rustfmt::skip]
#[inline]
fn multivector_dot_pseudoscalar(
  lhs: &Multivector,
  rhs: &Pseudoscalar,
) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e1, e2, e3, scalar, e23, e31, e12, e123] = [0f32; 8];

  let e0 = a.e123()*b.e0123();
  let e01 = -a.e23()*b.e0123();
  let e02 = -a.e31()*b.e0123();
  let e03 = -a.e12()*b.e0123();
  let e032 = a.e1()*b.e0123();
  let e013 = a.e2()*b.e0123();
  let e021 = a.e3()*b.e0123();
  let e0123 = a.s()*b.e0123();

  Multivector::from([
        e0,    e1,    e2,    e3,
    scalar,   e23,   e31,   e12,
       e01,   e02,   e03, e0123,
      e123,  e032,  e013,  e021,
  ])
}

// Scalar

#[inline]
fn scalar_dot_multivector(lhs: &Scalar, rhs: &Multivector) -> Multivector {
  Multivector::from(rhs.elements.map(|e| lhs.scalar() * e))
}

#[inline]
fn scalar_dot_scalar(lhs: &Scalar, rhs: &Scalar) -> Scalar {
  Scalar::from(lhs.scalar() * rhs.scalar())
}

#[inline]
fn scalar_dot_vector(lhs: &Scalar, rhs: &Vector) -> Vector {
  Vector::from(rhs.elements.map(|e| lhs.scalar() * e))
}

#[inline]
fn scalar_dot_bivector(lhs: &Scalar, rhs: &Bivector) -> Bivector {
  Bivector::from(rhs.elements.map(|e| lhs.scalar() * e))
}

#[inline]
fn scalar_dot_trivector(lhs: &Scalar, rhs: &Trivector) -> Trivector {
  Trivector::from(rhs.elements.map(|e| lhs.scalar() * e))
}

#[inline]
fn scalar_dot_pseudoscalar(lhs: &Scalar, rhs: &Pseudoscalar) -> Pseudoscalar {
  Pseudoscalar::from(lhs.scalar() * rhs.pseudoscalar())
}

// Vector

#[rustfmt::skip]
#[inline]
fn vector_dot_multivector(
  lhs: &Vector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let scalar = a.e1()*b.e1() + a.e2()*b.e2() + a.e3()*b.e3();
  let e0 = a.e0()*b.s() - a.e1()*b.e01() - a.e2()*b.e02() - a.e3()*b.e03();
  let e1 = a.e1()*b.s() + a.e3()*b.e31() - a.e2()*b.e12();
  let e2 = a.e2()*b.s() + a.e1()*b.e12() - a.e3()*b.e23();
  let e3 = a.e3()*b.s() + a.e2()*b.e23() - a.e1()*b.e31();
  let e01 = a.e3()*b.e013() - a.e2()*b.e021();
  let e02 = a.e1()*b.e021() - a.e3()*b.e032();
  let e03 = a.e2()*b.e032() - a.e1()*b.e013();
  let e23 = a.e1()*b.e123();
  let e31 = a.e2()*b.e123();
  let e12 = a.e3()*b.e123();
  let e123 = 0f32;
  let e032 = a.e1()*b.e0123();
  let e013 = a.e2()*b.e0123();
  let e021 = a.e3()*b.e0123();
  let e0123 = 0f32;

  Multivector::from([
        e0,    e1,    e2,    e3,
    scalar,   e23,   e31,   e12,
       e01,   e02,   e03, e0123,
      e123,  e032,  e013,  e021,
  ])
}

#[rustfmt::skip]
#[inline]
fn vector_dot_scalar(
  lhs: &Vector,
  rhs: &Scalar,
) -> Vector {
  Vector::from(lhs.elements.map(|e| e * rhs.scalar()))
}

#[rustfmt::skip]
#[inline]
fn vector_dot_vector(
  lhs: &Vector,
  rhs: &Vector,
) -> Scalar {
  let (a, b) = (lhs, rhs);
  let scalar = a.e1()*b.e1() + a.e2()*b.e2() + a.e3()*b.e3();

  Scalar::from(scalar)
}

#[rustfmt::skip]
#[inline]
fn vector_dot_bivector(
  lhs: &Vector,
  rhs: &Bivector,
) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = -a.e1()*b.e01() - a.e2()*b.e02() - a.e3()*b.e03();
  let e1 = a.e3()*b.e31() - a.e2()*b.e12();
  let e2 = a.e1()*b.e12() - a.e3()*b.e23();
  let e3 = a.e2()*b.e23() - a.e1()*b.e31();

  Vector::from([e0, e1, e2, e3])
}

#[rustfmt::skip]
#[inline]
fn vector_dot_trivector(
  lhs: &Vector,
  rhs: &Trivector,
) -> Bivector {
  let (a, b) = (lhs, rhs);

  let e01 = a.e3()*b.e013() - a.e2()*b.e021();
  let e02 = a.e1()*b.e021() - a.e3()*b.e032();
  let e03 = a.e2()*b.e032() - a.e1()*b.e013();
  let e23 = a.e1()*b.e123();
  let e31 = a.e2()*b.e123();
  let e12 = a.e3()*b.e123();

  Bivector::from([e23, e31, e12, e01, e02, e03])
}

#[rustfmt::skip]
#[inline]
fn vector_dot_pseudoscalar(
  lhs: &Vector,
  rhs: &Pseudoscalar,
) -> Trivector {
  let (a, b) = (lhs, rhs);

  let e123 = 0f32;
  let e032 = a.e1()*b.e0123();
  let e013 = a.e2()*b.e0123();
  let e021 = a.e3()*b.e0123();

  Trivector::from([e123, e032, e013, e021])
}

// Bivector

#[rustfmt::skip]
#[inline]
fn bivector_dot_multivector(
  lhs: &Bivector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);
  let [e0123, e123, e032, e013, e021] = [0f32; 5];

  let scalar = -a.e23()*b.e23() - a.e31()*b.e31() - a.e12()*b.e12();
  let e0 = a.e01()*b.e1() + a.e02()*b.e2() + a.e03()*b.e3()
         + a.e23()*b.e032() + a.e31()*b.e013() + a.e12()*b.e021();
  let e1 = a.e12()*b.e2() - a.e31()*b.e3() - a.e23()*b.e123();
  let e2 = a.e23()*b.e3() - a.e12()*b.e1() - a.e31()*b.e123();
  let e3 = a.e31()*b.e1() - a.e23()*b.e2() - a.e12()*b.e123();
  let e01 = a.e01()*b.s() - a.e23()*b.e0123();
  let e02 = a.e02()*b.s() - a.e31()*b.e0123();
  let e03 = a.e03()*b.s() - a.e12()*b.e0123();
  let e23 = a.e23()*b.s();
  let e31 = a.e31()*b.s();
  let e12 = a.e12()*b.s();

  Multivector::from([
        e0,    e1,    e2,    e3,
    scalar,   e23,   e31,   e12,
       e01,   e02,   e03, e0123,
      e123,  e032,  e013,  e021,
  ])
}

#[inline]
fn bivector_dot_scalar(lhs: &Bivector, rhs: &Scalar) -> Bivector {
  Bivector::from(lhs.elements.map(|e| e * rhs.scalar()))
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_vector(
  lhs: &Bivector,
  rhs: &Vector,
) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e01()*b.e1() + a.e02()*b.e2() + a.e03()*b.e3();
  let e1 = a.e12()*b.e2() - a.e31()*b.e3();
  let e2 = a.e23()*b.e3() - a.e12()*b.e1();
  let e3 = a.e31()*b.e1() - a.e23()*b.e2();

  Vector::from([e0, e1, e2, e3])
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_bivector(
  lhs: &Bivector,
  rhs: &Bivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);
  let scalar = -a.e23()*b.e23() - a.e31()*b.e31() - a.e12()*b.e12();

  Scalar::from(scalar)
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_trivector(
  lhs: &Bivector,
  rhs: &Trivector,
) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e23()*b.e032() + a.e31()*b.e013() + a.e12()*b.e021();
  let e1 = -a.e23()*b.e123();
  let e2 = -a.e31()*b.e123();
  let e3 = -a.e12()*b.e123();

  Vector::from([e0, e1, e2, e3])
}

#[rustfmt::skip]
#[inline]
fn bivector_dot_pseudoscalar(
  lhs: &Bivector,
  rhs: &Pseudoscalar,
) -> Bivector {
  let (a, b) = (lhs, rhs);
  let [e23, e31, e12] = [0f32; 3];

  let e01 = -a.e23()*b.e0123();
  let e02 = -a.e31()*b.e0123();
  let e03 = -a.e12()*b.e0123();

  Bivector::from([e23, e31, e12, e01, e02, e03])
}

// Trivector

#[rustfmt::skip]
#[inline]
fn trivector_dot_multivector(
  lhs: &Trivector,
  rhs: &Multivector,
) -> Multivector {
  let (a, b) = (lhs, rhs);

  let scalar = -a.e123()*b.e123();
  let e0 = a.e123()*b.e0123() + a.e032()*b.e23() + a.e013()*b.e31() + a.e021()*b.e12();
  let e1 = -a.e123()*b.e23();
  let e2 = -a.e123()*b.e31();
  let e3 = -a.e123()*b.e12();
  let e01 = a.e013()*b.e3() - a.e021()*b.e2();
  let e02 = a.e021()*b.e1() - a.e032()*b.e3();
  let e03 = a.e032()*b.e2() - a.e013()*b.e1();
  let e23 = a.e123()*b.e1();
  let e31 = a.e123()*b.e2();
  let e12 = a.e123()*b.e3();
  let e123 = a.e123()*b.s();
  let e032 = a.e032()*b.s();
  let e013 = a.e013()*b.s();
  let e021 = a.e021()*b.s();
  let e0123 = 0f32;

  Multivector::from([
        e0,    e1,    e2,    e3,
    scalar,   e23,   e31,   e12,
       e01,   e02,   e03, e0123,
      e123,  e032,  e013,  e021,
  ])
}

#[inline]
fn trivector_dot_scalar(lhs: &Trivector, rhs: &Scalar) -> Trivector {
  Trivector::from(lhs.elements.map(|e| e * rhs.scalar()))
}

#[rustfmt::skip]
#[inline]
fn trivector_dot_vector(
  lhs: &Trivector,
  rhs: &Vector,
) -> Bivector {
  let (a, b) = (lhs, rhs);

  let e01 = a.e013()*b.e3() - a.e021()*b.e2();
  let e02 = a.e021()*b.e1() - a.e032()*b.e3();
  let e03 = a.e032()*b.e2() - a.e013()*b.e1();
  let e23 = a.e123()*b.e1();
  let e31 = a.e123()*b.e2();
  let e12 = a.e123()*b.e3();

  Bivector::from([e23, e31, e12, e01, e02, e03])
}

#[rustfmt::skip]
#[inline]
fn trivector_dot_bivector(
  lhs: &Trivector,
  rhs: &Bivector,
) -> Vector {
  let (a, b) = (lhs, rhs);

  let e0 = a.e032()*b.e23() + a.e013()*b.e31() + a.e021()*b.e12();
  let e1 = -a.e123()*b.e23();
  let e2 = -a.e123()*b.e31();
  let e3 = -a.e123()*b.e12();

  Vector::from([e0, e1, e2, e3])
}

#[rustfmt::skip]
#[inline]
fn trivector_dot_trivector(
  lhs: &Trivector,
  rhs: &Trivector,
) -> Scalar {
  let (a, b) = (lhs, rhs);
  let scalar = -a.e123()*b.e123();

  Scalar::from(scalar)
}

#[rustfmt::skip]
#[inline]
fn trivector_dot_pseudoscalar(
  lhs: &Trivector,
  rhs: &Pseudoscalar,
) -> Vector {
  let (a, b) = (lhs, rhs);
  let [e1, e2, e3] = [0f32; 3];

  let e0 = a.e123()*b.e0123();

  Vector::from([e0, e1, e2, e3])
}
