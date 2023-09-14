// we allow `std` in tests for convenience
// otherwise it's conditionally available with the `std` cargo-feature
#![cfg_attr(not(any(feature = "std", test, doctest)), no_std)]

mod operators;
mod optional_features;
mod values;

pub use operators::*;
pub use values::*;

#[rustfmt::skip]
#[cfg(any(test, doctest))]
mod test_values {
  use crate::*;

  pub const MULTIVECTOR_A: Multivector = Multivector {
    s: 11.,
    e0: 2., e1: 3., e2: 5., e3: 7.,
    e01: 23., e02: 29., e03: 31.,
    e12: 19., e31: 17., e23: 13.,
    e021: 53., e013: 47., e032: 43., e123: 41.,
    e0123: 37.,
  };
  pub const MULTIVECTOR_B: Multivector = Multivector {
    s: 73.,
    e0: 59., e1: 61., e2: 67., e3: 71.,
    e01: 97., e02: 101., e03: 103.,
    e12: 89., e31: 83., e23: 79.,
    e021: 131., e013: 127., e032: 113., e123: 109.,
    e0123: 107.,
  };
  pub const MULTIVECTOR_C: Multivector = Multivector {
    s: -73.,
    e0: -59., e1: 61., e2: -67., e3: 71.,
    e01: -97., e02: 101., e03: -103.,
    e12: 89., e31: -83., e23: 79.,
    e021: 131., e013: -127., e032: 113., e123: -109.,
    e0123: 107.,
  };
  pub const MULTIVECTOR_D: Multivector = Multivector {
    s: -73.,
    e0: -59., e1: -61., e2: -67., e3: -71.,
    e01: -97., e02: -101., e03: -103.,
    e12: -89., e31: -83., e23: -79.,
    e021: -131., e013: -127., e032: -113., e123: -109.,
    e0123: -107.,
  };
  pub const SCALAR_A: Scalar = Scalar { s: 137. };
  pub const SCALAR_B: Scalar = Scalar { s: 139. };
  pub const SCALAR_C: Scalar = Scalar { s: -149. };
  pub const VECTOR_A: Vector = Vector {
    e0: 151., e1: 157., e2: 163., e3: 167.,
  };
  pub const VECTOR_B: Vector = Vector {
    e0: 173., e1: 179., e2: 181., e3: 191.,
  };
  pub const VECTOR_C: Vector = Vector {
    e0: -193., e1: -197., e2: -199., e3: -211.,
  };
  pub const BIVECTOR_A: Bivector = Bivector {
    e01: 233., e02: 239., e03: 241.,
    e12: 229., e31: 227., e23: 223.,
  };
  pub const BIVECTOR_B: Bivector = Bivector {
    e01: 269., e02: 271., e03: 277.,
    e12: 263., e31: 257., e23: 251.,
  };
  pub const BIVECTOR_C: Bivector = Bivector {
    e01: -307., e02: -311., e03: -313.,
    e12: -293., e31: -283., e23: -281.,
  };
  pub const TRIVECTOR_A: Trivector = Trivector {
    e021: 347., e013: 337., e032: 331., e123: 317.,
  };
  pub const TRIVECTOR_B: Trivector = Trivector {
    e021: 367., e013: 359., e032: 353., e123: 349.,
  };
  pub const TRIVECTOR_C: Trivector = Trivector {
    e021: -389., e013: -383., e032: -379., e123: -373.,
  };
  pub const PSEUDOSCALAR_A: Pseudoscalar = Pseudoscalar { e0123: 397. };
  pub const PSEUDOSCALAR_B: Pseudoscalar = Pseudoscalar { e0123: 401. };
  pub const PSEUDOSCALAR_C: Pseudoscalar = Pseudoscalar { e0123: -409. };
}
