use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

use simba::simd::SimdRealField as Field;
use num_traits::{Zero, zero};

use crate::{Join, Dual};
use super::{Bivector, Trivector};

/// A 1-vector in CGA2D representing a point
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vector<T: Field> {
	pub (super) e1: T,
	pub (super) e2: T,
	pub (super) e3: T,
	pub (super) e4: T,
}

impl <T: Field> Join for Vector<T: Field> {
	type Output = Bivector<T>;
	/// Joins two points to create a pair.
	fn join(self, rhs: Self) -> Bivector<T> {
		Bivector {
			e12: self.e1 * rhs.e2 - self.e2 * rhs.e1,
			e13: self.e1 * rhs.e3 - self.e3 * rhs.e1,
			e14: self.e1 * rhs.e4 - self.e4 * rhs.e1,
			e23: self.e2 * rhs.e3 - self.e3 * rhs.e2,
			e24: self.e2 * rhs.e4 - self.e4 * rhs.e2,
			e34: self.e3 * rhs.e4 - self.e4 * rhs.e3,
		}
	}
}

impl <T: Field> Inner<Bivector<T>> for Vector<T: Field> {
	type Output = Vector<T>;
	fn inner(self, rhs: Trivector<T>) -> Vector<T> {
		Vector {
			e1: self.e4 * rhs.e14 - self.e2 * rhs.e12 - self.e3 * rhs.e13,
			e2: self.e1 * rhs.e12 - self.e3 * rhs.e23 - self.e4 * rhs.e24,
			e3: self.e1 * rhs.e13 + self.e2 * rhs.e23 + self.e4 * rhs.e34,
			e4: self.e1 * rhs.e14 + self.e2 * rhs.e24 + self.e3 * rhs.e34,
		}
	}
}

impl <T: Field> Inner<Trivector<T>> for Vector<T: Field> {
	type Output = Bivector<T>;
	fn inner(self, rhs: Trivector<T>) -> Bivector<T> {
		Bivector {
			e12: self.e3 * rhs.e123 - self.e4 * rhs.e124,
			e13: -self.e2 * rhs.e123 - self.e4 * rhs.e134,
			e14: -self.e2 * rhs.e124 - self.e3 * rhs.e134,
			e23: self.e1 * rhs.e123 - self.e4 * rhs.e234,
			e24: self.e1 * rhs.e124 - self.e3 * rhs.e234,
			e34: self.e1 * rhs.e134 + self.e2 * rhs.e234,
		}
	}
}

impl <T: Field> Dual for Vector<T: Field> {
	type Output = Trivector<T>;
	fn dual(self) -> Trivector<T> {
		Trivector {
			e234: -self.e1,
			e134: self.e2,
			e124: -self.e3,
			e123: -self.e4,
		}
	}
}

impl<T: Field> AddAssign for Vector<T> {
	fn add_assign(&mut self, other: Self) {
		self.e1 += other.e1;
		self.e2 += other.e2;
		self.e3 += other.e3;
		self.e4 += other.e4;
	}
}

impl<T: Field> SubAssign for Vector<T> {
	fn sub_assign(&mut self, other: Self) {
		self.e1 -= other.e1;
		self.e2 -= other.e2;
		self.e3 -= other.e3;
		self.e4 -= other.e4;
	}
}

impl <T: Field> MulAssign<T> for Vector<T> {
	fn mul_assign(self, s: T) -> Self {
		self.e1 *= s;
		self.e2 *= s;
		self.e3 *= s;
		self.e4 *= s;
	}
}

impl<T: Field> Add for Vector<T> {
	type Output = Self;
	fn add(mut self, other: Self) -> Self {
		self += other;
		self
	}
}

impl<T: Field> Sub for Vector<T> {
	type Output = Self;
	fn sub(mut self, other: Self) -> Self {
		self -= other;
		self
	}
}

impl<T: Field> Mul<T> for Vector<T> {
	type Output = Self;
	fn mul(mut self, other: Self) -> Self {
		self *= other;
		self
	}
}

impl<T: Field> Mul<Vector<T>> for T {
	type Output = Vector<T>;
	fn mul(mut self, other: Vector<T>) -> Vector<T> {
		other *= self;
		other
	}
}

impl<T: Field> Zero for Vector<T> {
	fn zero() -> Self {
		Self {
			e1: zero(),
			e2: zero(),
			e3: zero(),
			e4: zero(),
		}
	}
}
