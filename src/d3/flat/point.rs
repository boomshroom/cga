use core::ops::{Add, AddAssign, Mul, MulAssign};

use simba::simd::SimdRealField as Field;
use simba::scalar::RealField;
use num_traits::{Zero, zero, one};

use crate::Join;
use super::{Line, Plane};
use super::super::free::{Vector, Bivector};
use super::super::transform::Translator;

#[derive(Copy, Clone, Debug)]
pub struct FPoint<T: Field + Copy> {
	/// Corresponds to both e1p and e1n
	pub (crate) e1i: T,
	/// Corresponds to both e2p and e2n
	pub (crate) e2i: T,
	/// Corresponds to both e3p and e3n
	pub (crate) e3i: T,
	pub (crate) epn: T,
}

impl <T: Field + Copy> FPoint<T> {
	/// Constructs the line along `dir` that passes through self.
	pub fn extend_along_vec(self, dir: Vector<T>) -> Line<T> {
		Line {
			e12i: self.e2i * dir.e1 - self.e1i * dir.e2,
			e13i: self.e3i * dir.e1 - self.e1i * dir.e3,
			e23i: self.e3i * dir.e2 - self.e2i * dir.e3,
			e1pn: self.epn * dir.e1,
			e2pn: self.epn * dir.e2,
			e3pn: self.epn * dir.e3,
		}
	}

	/// Constructs the plane along `dir` that passes through self.
	pub fn extend_along_bivec(self, dir: Bivector<T>) -> Plane<T> {
		Plane {
			e123i: self.e1i * dir.e23 - self.e2i * dir.e13 + self.e3i * dir.e12,
			e12pn: self.epn * dir.e12,
			e13pn: self.epn * dir.e13,
			e23pn: self.epn * dir.e23,
		}
	}
}

impl <T: Field + Copy> Mul for FPoint<T> {
	type Output = Translator<T>;
	fn mul(self, rhs: Self) -> Translator<T> {
		Translator {
			s: self.epn * rhs.epn,
			e1i: self.e1i * rhs.epn - self.epn * rhs.e1i,
			e2i: self.e2i * rhs.epn - self.epn * rhs.e2i,
			e3i: self.e3i * rhs.epn - self.epn * rhs.e3i,
		}
	}
}