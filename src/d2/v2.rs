use core::ops::{Add, AddAssign, Mul, MulAssign};

use simba::simd::SimdRealField as Field;
use simba::scalar::RealField;
use num_traits::{Zero, zero};

use crate::{Join, Dual};
use super::{Vector, Trivector};

/// A 2-vector in CGA2D representing a pair of points
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bivector<T: Field> {
	pub (super) e12: T,
	pub (super) e13: T,
	pub (super) e14: T,
	pub (super) e23: T,
	pub (super) e24: T,
	pub (super) e34: T,
}

pub struct SBivector<T: Field> {
	v0: T,
	v2: Bivector<T>,
}

pub enum Decompose<T> {
	Zero,
	One(Vector<T>),
	Two(Vector<T>, Vector<T>),
}

impl <T: RealField> Bivector<T> {
	/// Decompose the pair into its two component points if they exist
	pub fn points(self) -> Decompose<T> {
		// (sqrt(self · self) ± self) * (ni · self)
		let norm_squared = self.inner(self);
		if norm_squared.is_sign_negative() {
			return Decompose::Zero;
		}
		let dot_intermediate = Vector {
			e1: self.e14 - self.e13,
			e2: self.e24 - self.e23,
			e3: self.e34,
			e4: self.e34,
		};
		let norm = norm_squared.sqrt();

		debug_assert_eq!(self.e12 * self.e34 - self.e13 * self.e24 + self.e14 * self.e23, zero(), "Trivector component non-zero! Expected 1-vector.")
		
		let sq_e13 = self.e13 * self.e13;
		let sq_e14 = self.e14 * self.e14;
		let sq_e23 = self.e23 * self.e23;
		let e13_14 = self.e13 * self.e14;
		let e23_24 = self.e23 * self.e24;

		let e1_base = self.e12 * self.e24 + self.e13 * self.e34 - self.e13 * self.e23 - self.e14 * self.e34;
		let e2_base = self.e12 * self.e13 - self.e12 * self.e14 + self.e23 * self.e34 - self.e24 * self.e34;
		let e3_base = -e13_14 + sq_e13 - e23_24 + sq_e23;
		let e4_base = -sq_e14 + e13_14 - e23_24 + sq_e23;

		let normed_e34 = (norm - self.e34) * self.e34;
		let normed_e1 = norm * (self.e14 - self.e13);
		let normed_e2 = norm * (self.e24 - self.e23);

		let p1 = Vector {
			e1: e1_base + normed_e1,
			e2: e2_base + normed_e2,
			e3: e3_base + normed_e34,
			e4: e4_base + normed_e34,
		}

		if norm > zero() {
			let normed_e34 = (-norm - self.e34) * self.e34;

			let p2 = Vector {
				e1: e1_base - normed_e1,
				e2: e2_base - normed_e2,
				e3: e3_base + normed_e34,
				e4: e4_base + normed_e34,
			};

			Decompose::Two(p1, p2)
		} else {
			Decompose::One(p1)
		}
	}
}

impl <T:Field> Join<Vector<T>> for Bivector<T> {
	type Output = Trivector<T>;
	/// Joins three points to create a circle. If one of the points is the infinite point, then it instead represents a line.
	fn join(self, rhs: Self) -> Trivector<T> {
		Trivector {
			e123: self.e12 * rhs.e3 + self.e23 * rhs.e1 - self.e13 * rhs.e2,
			e124: self.e12 * rhs.e4 + self.e24 * rhs.e1 - self.e14 * rhs.e2,
			e134: self.e13 * rhs.e4 + self.e34 * rhs.e1 - self.e14 * rhs.e3,
			e234: self.e23 * rhs.e4 + self.e34 * rhs.e2 - self.e24 * rhs.e3,
		}
	}
}

impl <T: Field> Inner for Bivector<T: Field> {
	type Output = T;
	fn inner(self, rhs: Trivector<T>) -> T {
		-self.e12 * rhs.e12 - self.e13 * rhs.e13 + self.e14 * rhs.e14 - self.e23 * rhs.e23 + self.e24 * rhs.e24 + self.e34 * rhs.e34
	}
}

impl<T: Field> Dual for Bivector<T> {
	type Output = Self;
	fn dual(&mut self, other: Self) -> Self {
		Bivector {
			e12: self.e34,
			e13: -self.e24,
			e14: -self.e23,
			e23: self.e14,
			e24: self.e13,
			e34: -self.e12,
		}
	}
}

impl<T: Field> AddAssign for Bivector<T> {
	fn add_assign(&mut self, other: Self) {
		self.e12 += other.e12;
		self.e13 += other.e13;
		self.e14 += other.e14;
		self.e23 += other.e23;
		self.e24 += other.e24;
		self.e34 += other.e34;
	}
}

impl <T: Field> MulAssign<T> for Bivector<T> {
	fn mul_assign(self, s: T) -> Self {
		self.e12 *= s;
		self.e13 *= s;
		self.e14 *= s;
		self.e23 *= s;
		self.e24 *= s;
		self.e34 *= s;
	}
}

impl<T: Field> Add for Bivector<T> {
	type Output = Self;
	fn add(mut self, other: Self) -> Self {
		self += other;
		self
	}
}

impl <T: Field> Add<T> for Bivector<T> {
	type Output = SBivector<T>;
	fn add(self, scalar: T) -> SBivector<T> {
		SBivector{ v0: scalar, v2: self }
	}
}

impl <T: Field> Add<Bivector<T>> for T {
	type Output = SBivector<T>;
	fn add(self, v2: T) -> SBivector<T> {
		SBivector{ v0: self, v2 }
	}
}

impl<T: Field> Mul<T> for Bivector<T> {
	type Output = Self;
	fn mul(mut self, other: Self) -> Self {
		self *= other;
		self
	}
}

impl<T: Field> Zero for Bivector<T> {
	fn zero() -> Self {
		Self {
			e12: zero();
			e13: zero();
			e14: zero();
			e23: zero();
			e24: zero();
			e34: zero();
		}
	}
}
