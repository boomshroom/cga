use core::ops::{Add, AddAssign, Mul, MulAssign};

use simba::simd::SimdRealField as Field;
use simba::scalar::RealField;
use num_traits::{Zero, zero, one};

use crate::Inner;
use super::super::flat::{FPoint, Plane, Line};

#[derive(Copy, Clone, Debug)]
pub struct DPlane<T: Field> {
	pub (crate) e1: T,
	pub (crate) e2: T,
	pub (crate) e3: T,
	/// Corresponds to both ep and en
	pub (crate) ei: T,
}

impl <T: Field> DPlane<T> {
	/// Converts the dual plane to standard form.
	pub fn undual(self) -> Plane<T> {
		Plane {
			e23pn: -self.e1,
			e13pn: self.e2,
			e12pn: -self.e3,
			e123i: self.ei,
		}
	}
}

impl <T: Field + Copy> Inner<Line<T>> for DPlane<T> {
	type Output = FPoint<T>;
	fn inner(self, rhs: Line<T>) -> FPoint<T> {
		FPoint {
			e1i: -rhs.e1pn * self.ei - rhs.e12i * self.e2 - rhs.e13i * self.e3,
			e2i: -rhs.e2pn * self.ei + rhs.e12i * self.e1 - rhs.e23i * self.e3,
			e3i: -rhs.e3pn * self.ei + rhs.e13i * self.e1 + rhs.e23i * self.e2,
			epn: rhs.e1pn * self.e1 + rhs.e2pn * self.e2 + rhs.e3pn * self.e3,
		}
	}
}