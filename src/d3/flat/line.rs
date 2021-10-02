use core::ops::{Add, AddAssign, Mul, MulAssign};

use simba::simd::SimdRealField as Field;
use simba::scalar::RealField;
use num_traits::{Zero, zero, one};

use crate::{Z, Join, Inner, Dual};

use super::super::dual::DLine;
use super::super::transform::Motor;

#[derive(Copy, Clone, Debug)]
pub struct Line<T: Field> {
	/// Corresponds to both e12p and e12n
	pub(crate) e12i: T,
	/// Corresponds to both e13p and e13n
	pub(crate) e13i: T,
	/// Corresponds to both e23p and e23n
	pub(crate) e23i: T,
	pub(crate) e1pn: T,
	pub(crate) e2pn: T,
	pub(crate) e3pn: T,
}

impl <T: Field> Dual for Line<T> {
	type Output = DLine<T>;
	fn dual(self) -> DLine<T> {
		DLine {
			e12: self.e3pn,
			e13: -self.e2pn,
			e23: self.e1pn,
			e1i: self.e23i,
			e2i: -self.e13i,
			e3i: self.e12i,
		}
	}
}

impl <T: Field + Copy> Mul for Line<T> {
	type Output = Motor<T>;
	fn mul(self, rhs: Self) -> Motor<T> {
		Motor {
			s: self.e1pn * rhs.e1pn + self.e2pn * rhs.e2pn + self.e3pn * rhs.e3pn,
			e12: self.e1pn * rhs.e2pn - self.e2pn * rhs.e1pn,
			e13: self.e1pn * rhs.e3pn - self.e3pn * rhs.e1pn,
			e23: self.e2pn * rhs.e3pn - self.e3pn * rhs.e2pn,
			e1i: self.e12i * rhs.e2pn - self.e2pn * rhs.e12i - self.e13i * rhs.e3pn + self.e3pn * rhs.e13i,
			e2i: self.e12i * rhs.e1pn - self.e1pn * rhs.e12i - self.e23i * rhs.e3pn + self.e3pn * rhs.e23i,
			e3i: self.e13i * rhs.e1pn - self.e1pn * rhs.e13i + self.e23i * rhs.e2pn - self.e2pn * rhs.e23i,
			e123i: -self.e12i * rhs.e3pn - self.e3pn * rhs.e12i + self.e13i * rhs.e2pn + self.e2pn * rhs.e13i - self.e1pn * rhs.e23i - self.e23i * rhs.e1pn,
		}
	}
}