use core::ops::{Add, AddAssign, Mul, MulAssign, BitOr};

use simba::simd::SimdRealField as Field;
use simba::scalar::RealField;
use num_traits::{Zero, zero};

#[derive(Copy, Clone)]
pub struct Rotor<T: Field> {
	pub (crate) s: T,
	pub (crate) e12: T,
	pub (crate) e13: T,
	pub (crate) e23: T,
}

