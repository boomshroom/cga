use core::ops::{Add, AddAssign, Mul, MulAssign, BitOr};

use simba::simd::SimdRealField as Field;
use simba::scalar::RealField;
use num_traits::{Zero, zero};

use crate::Inner;

#[derive(Copy, Clone)]
pub struct Trivector<T: Field> {
	pub (crate) e123: T,
}

impl<T: Field> Inner for Trivector<T> {
	type Output = T;
	fn inner(self, rhs: Self) -> T {
		-self.e123 * rhs.e123
	}
}

