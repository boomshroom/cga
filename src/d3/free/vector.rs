use core::ops::{Add, AddAssign, Mul, MulAssign, BitOr};

use simba::simd::SimdRealField as Field;
use simba::scalar::RealField;
use num_traits::{Zero, zero, one};

use crate::{Inner, Outer};
use super::Bivector;
use super::super::transform::Rotor;
use super::super::round::Point;
use super::super::dual::DPlane;
use super::super::r410::R410;

#[derive(Copy, Clone)]
pub struct Vector<T: Field> {
	pub (crate) e1: T,
	pub (crate) e2: T,
	pub (crate) e3: T,
}

impl<T: Field> Vector<T> {
	pub fn new(e1: T, e2: T, e3: T) -> Self {
		Self {e1, e2, e3}
	}

	/// Constructs the plane perpendicular to this vector at a distance `dist` from the origin.
	pub fn normal(self, dist: T) -> DPlane<T> {
		DPlane{
			e1: self.e1,
			e2: self.e2,
			e3: self.e3,
			ei: dist,
		}
	}

}

impl<T: Field + Copy> From<[T; 3]> for Vector<T> {
	fn from(v: [T; 3]) -> Self {
		Self { e1: v[0], e2: v[1], e3: v[2] }
	}
}

impl<T: Field + Copy> Vector<T> {
	#[inline]
	fn into_multivec(self) -> R410<T> {
		let Vector{e1, e2, e3} = self;
		R410 { e1, e2, e3, ..zero() }
	}

	/// Constructs the point at the coordinates pointed to by this vector.
	#[inline]
	pub fn into_point(self) -> Point<T> {
		let half = T::from_subset(&0.5);
		let no = R410 { en: half, ep: -half, ..zero() };
		let ni = R410 { en: one(), ep: one(), ..zero() };

		/*
		let square = self.norm_squared() * half;
		Point {
			e1: self.e1,
			e2: self.e2,
			e3: self.e3,
			ep: square - half.clone(),
			en: square + half,
		}*/

		let v = self.into_multivec();
		let R410 { e1, e2, e3, ep, en, ..} = v + ni * (v * v) * half + no;
		Point { e1, e2, e3, ep, en }
	}

	pub fn norm_squared(self) -> T {
		self.inner(self)
	}
}

impl<T: Field> Inner for Vector<T> {
	type Output = T;
	fn inner(self, rhs: Self) -> T {
		self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3
	}
}

impl<T: Field + Copy> Outer for Vector<T> {
	type Output = Bivector<T>;
	fn outer(self, rhs: Self) -> Bivector<T> {
		Bivector {
			e12: self.e1 * rhs.e2 - self.e2 * rhs.e1,
			e13: self.e1 * rhs.e3 - self.e3 * rhs.e1,
			e23: self.e2 * rhs.e3 - self.e3 * rhs.e2,
		}
	}
}

impl <T: Field + Copy> Mul for Vector<T> {
	type Output = Rotor<T>;
	fn mul(self, rhs: Self) -> Rotor<T> {
		Rotor {
			s: self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3,
			e12: self.e1 * rhs.e2 - self.e2 * rhs.e1,
			e13: self.e1 * rhs.e3 - self.e3 * rhs.e1,
			e23: self.e2 * rhs.e3 - self.e3 * rhs.e2,
		}
	}
}