use core::ops::{Add, AddAssign, Mul, MulAssign};

use simba::simd::SimdRealField as Field;
use simba::scalar::RealField;

use crate::Outer;
use super::{Sphere, Point};
use super::super::flat::Plane;

#[derive(Copy, Clone, Debug)]
pub struct Circle<T: Field> {
	pub (crate) e123: T,
	pub (crate) e12p: T,
	pub (crate) e12n: T,
	pub (crate) e13p: T,
	pub (crate) e13n: T,
	pub (crate) e23p: T,
	pub (crate) e23n: T,
	pub (crate) e1pn: T,
	pub (crate) e2pn: T,
	pub (crate) e3pn: T,
}

impl <T: Field> Circle<T> {
	/// Extends the circle into the infinite plane containing it.
	pub fn extend(self) -> Plane<T> {
		Plane {
			e123i: self.e123,
			e12pn: self.e12p - self.e12n,
			e13pn: self.e13p - self.e13n,
			e23pn: self.e23p - self.e23n,
		}
	}
}

impl<T: Field + Copy> Outer<Point<T>> for Circle<T> {
	type Output = Sphere<T>;
	/// Construct the sphere passing through the given point and this circle.
	fn outer(self, p: Point<T>) -> Sphere<T> {
		Sphere {
			e123p: self.e123 * p.ep - self.e12p * p.e3 + self.e13p * p.e2 - self.e23p * p.e1,
			e123n: self.e123 * p.en - self.e12n * p.e3 + self.e13n * p.e2 - self.e23p * p.e1,
			e12pn: self.e12p * p.en - self.e12n * p.ep + self.e1pn * p.e2 - self.e2pn * p.e1,
			e13pn: self.e13p * p.en - self.e13n * p.ep + self.e1pn * p.e3 - self.e3pn * p.e1,
			e23pn: self.e23p * p.en - self.e23n * p.ep + self.e2pn * p.e3 - self.e3pn * p.e2,
		}
	}
}
