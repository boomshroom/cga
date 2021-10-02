use core::ops::{Add, AddAssign, Mul, MulAssign, BitOr};

use simba::simd::SimdRealField as Field;
use simba::scalar::RealField;
use num_traits::{Zero, zero};

#[derive(Copy, Clone)]
pub struct Translator<T: Field> {
	pub (crate) s: T,
	pub (crate) e1i: T,
	pub (crate) e2i: T,
	pub (crate) e3i: T,
}
