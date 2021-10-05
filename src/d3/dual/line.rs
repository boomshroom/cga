use simba::simd::SimdRealField as Field;
use num_traits::zero;

use crate::{R410, Multivec, Inner};

use super::super::flat::{FPoint, Line, Plane};
use super::super::round::{Pair, Sphere};

#[derive(Copy, Clone, Debug)]
pub struct DLine<T> {
    pub(crate) e12: T,
    pub(crate) e13: T,
    pub(crate) e23: T,
    pub(crate) e1i: T,
    pub(crate) e2i: T,
    pub(crate) e3i: T,
}

impl<T: Field + Copy> DLine<T> {
    /// Converts the dual line to standard form.
    #[inline]
    pub fn undual(self) -> Line<T> {
        Line {
            e3pn: self.e12,
            e2pn: -self.e13,
            e1pn: self.e23,
            e23i: self.e1i,
            e13i: -self.e2i,
            e12i: self.e3i,
        }
    }
}

impl<T: Field + Copy> Multivec for DLine<T> {
    type Element = T;
    #[inline]
	fn into_mv(self) -> R410<T> {
		let DLine{e12, e13, e23, e1i, e2i, e3i} = self;
		R410{ e12, e13, e23, e1p: e1i, e1n: e1i, e2p: e2i, e2n: e2i, e3p: e3i, e3n: e3i, ..zero() }
	}

    #[inline]
	fn from_mv(v: R410<T>) -> Self {
		let R410{e12, e13, e23, e1p, e2p, e3p, ..} = v;
		Self{ e12, e13, e23, e1i: e1p, e2i: e2p, e3i: e3p }
	}
}

impl<T: Field + Copy> Inner<Sphere<T>> for DLine<T> {
    type Output = Pair<T>;
    #[inline]
    fn inner(self, rhs: Sphere<T>) -> Pair<T> {
    	Pair::from_mv(self.into_mv() | rhs.into_mv())
    	/*
        Pair {
            e12: (rhs.e123n - rhs.e123p) * self.e3i,
            e13: -(rhs.e123n - rhs.e123p) * self.e2i,
            e23: (rhs.e123n - rhs.e123p) * self.e1i,
            e1p: -self.e23 * rhs.e123p - self.e2i * rhs.e12pn - self.e3i * rhs.e13pn,
            e1n: -self.e23 * rhs.e123n - self.e2i * rhs.e12pn - self.e3i * rhs.e13pn,
            e2p: self.e13 * rhs.e123p + self.e1i * rhs.e12pn - self.e3i * rhs.e23pn,
            e2n: self.e13 * rhs.e123n + self.e1i * rhs.e12pn - self.e3i * rhs.e23pn,
            e3p: -self.e12 * rhs.e123p + self.e1i * rhs.e13pn + self.e2i * rhs.e23pn,
            e3n: -self.e12 * rhs.e123n + self.e1i * rhs.e13pn + self.e2i * rhs.e23pn,
            epn: -self.e12 * rhs.e12pn - self.e13 * rhs.e13pn - self.e23 * rhs.e23pn,
        }*/
    }
}

impl<T: Field + Copy> Inner<Plane<T>> for DLine<T> {
    type Output = FPoint<T>;
    #[inline]
    fn inner(self, rhs: Plane<T>) -> FPoint<T> {
        FPoint {
            e1i: -self.e23 * rhs.e123i - self.e2i * rhs.e12pn - self.e3i * rhs.e13pn,
            e2i: self.e13 * rhs.e123i + self.e1i * rhs.e12pn - self.e3i * rhs.e23pn,
            e3i: -self.e12 * rhs.e123i + self.e1i * rhs.e13pn + self.e2i * rhs.e23pn,
            epn: -self.e12 * rhs.e12pn - self.e13 * rhs.e13pn - self.e23 * rhs.e23pn,
        }
    }
}
