use core::ops::Mul;

use num_traits::zero;

use simba::simd::SimdRealField as Field;

use crate::{R410, Multivec};
use super::super::{Point, Sphere};
use super::Transform;

#[derive(Copy, Clone)]
pub struct Translator<T: Field> {
    pub(crate) e1i: T,
    pub(crate) e2i: T,
    pub(crate) e3i: T,
    pub(crate) s: T,
}

impl <T: Field + Copy> Multivec for Translator<T> {
	type Element = T;
	#[inline]
	fn into_mv(self) -> R410<T> {
		let Translator{s, e1i, e2i, e3i} = self;
		R410{ s, e1p: e1i, e1n: e1i, e2p: e2i, e2n: e2i, e3p: e3i, e3n: e3i, ..zero() }
	}

	#[inline]
	fn from_mv(v: R410<T>) -> Self {
		let R410{ s, e1p, e2p, e3p, .. } = v;
		Self{s, e1i: e1p, e2i: e2p, e3i: e3p}
	}
}

impl <T: Field + Copy> Transform<Sphere<T>> for Translator<T> {
	#[inline]
	fn transform(self, s: Sphere<T>) -> Sphere<T> {
		let t = self.into_mv();
		Sphere::from_mv(t.reverse() * s.into_mv() * t)
	}
}

impl <T: Field + Copy> Transform<Point<T>> for Translator<T> {
	#[inline]
	fn transform(self, p: Point<T>) -> Point<T> {
		let t = self.into_mv();
		Point::from_mv(t.reverse() * p.into_mv() * t)
	}
}

impl <T: Field + Copy> Mul for Translator<T> {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		Self::from_mv(self.into_mv() * rhs.into_mv())
	}
}

impl <T: Field + Copy> Mul<T> for Translator<T> {
	type Output = Self;
	fn mul(self, rhs: T) -> Self {
		Self::from_mv(self.into_mv() * rhs)
	}
}