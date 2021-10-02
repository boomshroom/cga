use core::ops::{Add, AddAssign, Mul, MulAssign};

use simba::simd::SimdRealField as Field;
use num_traits::{Zero, zero};

use crate::{Meet, Dual, Reflect};
use super::{Multivector, Vector, Bivector};

/// A 3-vector in CGA2D representing a circle or a line
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Trivector<T: Field> {
	pub (super) e123: T,
	pub (super) e124: T,
	pub (super) e134: T,
	pub (super) e234: T,
}

#[inline(always)]
fn double<T: Field>(x: T) -> T {
	x + x
}

impl<T: Field> Trivector<T> {
	pub fn center(self) -> Vector<T> {
		// Hand-optimized since LLVM doesn't optimize * 0.0 (Why, IEEE?)
		// self.reflect(Multivector::ni())

		let sq_123 = self.e123 * self.e123;
		let sq_124 = self.e124 * self.e124;
		let sq_134 = self.e134 * self.e134;
		let sq_234 = self.e234 * self.e234;

		let e123_124 = double(self.e123 * self.e124);
		let e12_diff = double(self.e123 - self.e124);

		let e1 = self.e234 * e12_diff;
		let e2 = self.e134 * -e12_diff;
		let e3 = e123_124 - sq_123 - sq_124 + sq_134 + sq_234;
		let e4 = -e123_124 + sq_123 + sq_124 + sq_134 + sq_234;

		Vector { e1, e2, e3, e4 }
	}
}

impl<T: Field> Dual for Trivector<T> {
	type Output = Vector<T>;
	fn dual(self) -> Vector<T> {
		Vector {
			e1: self.e234,
			e2: -self.e134,
			e3: self.e124,
			e4: self.e123,
		}
	}
}

impl <T: Field> Reflect<Vector<T>> for Trivector<T> {
	fn reflect(self, o: Vector<T>) -> Vector<T> {
		// self * o * self
		// Trivector components cancel out with themselves.

		let sq_123 = self.e123 * self.e123;
		let sq_124 = self.e124 * self.e124;
		let sq_134 = self.e134 * self.e134;
		let sq_234 = self.e234 * self.e234;

		let e1_234 = double(o.e1 * self.e234);
		let e2_134 = double(o.e2 * self.e134);
		let e3_124 = double(o.e3 * self.e124);
		let e4_123 = double(o.e4 * self.e123);

		let e1 = o.e1 * (-sq_123 + sq_124 + sq_134 - sq_234)
			+ self.e234 * (e2_134 - e3_124 + e4_123);
		let e2 = o.e2 * (-sq_123 + sq_124 - sq_134 + sq_234)
			+ self.e134 * (-e4_123 + e3_124 + e1_234);
		let e3 = o.e3 * (-sq_123 - sq_124 + sq_134 + sq_234)
			+ self.e124 * (e4_123 + e2_134 - e1_234);
		let e4 = o.e4 * (sq_123 + sq_124 + sq_134 + sq_234)
			+ self.e123 * (-e3_124 + e2_134 - e1_234);

		Vector { e1, e2, e3, e4 }
	}
}

impl<T: Field> AddAssign for Trivector<T> {
	fn add_assign(&mut self, other: Self) {
		self.e123 += other.e123;
		self.e124 += other.e124;
		self.e134 += other.e134;
		self.e234 += other.e234;
	}
}

impl <T: Field> MulAssign<T> for Trivector<T> {
	fn mul_assign(self, s: T) -> Self {
		self.e123 *= s;
		self.e124 *= s;
		self.e134 *= s;
		self.e234 *= s;
	}
}

impl<T: Field> Add for Trivector<T> {
	type Output = Self;
	fn add(mut self, other: Self) -> Self {
		self += other;
		self
	}
}

impl<T: Field> Mul<T> for Trivector<T> {
	type Output = Self;
	fn mul(mut self, other: Self) -> Self {
		self *= other;
		self
	}
}

impl<T: Field> Zero for Trivector<T> {
	fn zero() -> Self {
		Self {
			e123: zero(),
			e124: zero(),
			e134: zero(),
			e234: zero(),
		}
	}
}