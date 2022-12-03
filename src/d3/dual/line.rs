use core::marker::PhantomData;

use num_traits::zero;
use simba::simd::SimdRealField as Field;

use crate::{Inner, Multivec, R410, Space, Euclidean};

use super::super::flat::{FPoint, Line, Plane};
use super::super::round::{Pair, Sphere};

#[derive(Copy, Clone, Debug)]
pub struct DLine<T, S = Euclidean> {
    pub(crate) e12: T,
    pub(crate) e13: T,
    pub(crate) e23: T,
    pub(crate) e1i: T,
    pub(crate) e2i: T,
    pub(crate) e3i: T,
    _pd: PhantomData<S>,
}

/*
(a12 + b13 + c23 + d1i + e2i + f3i)(a21 + b31 + c32 + di1 + ei2 + fi3)
a^2 + b^2 + c^2 0
*/

impl<T: Field + Copy, S: Space> DLine<T, S> {
    /// Converts the dual line to standard form.
    #[inline]
    pub fn undual(self) -> Line<T, S::Dual> {
        Line::from_mv(self.into_mv().undual())
    }
}

impl<T: Field + Copy, S: Space> Multivec for DLine<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let DLine {
            e12,
            e13,
            e23,
            e1i,
            e2i,
            e3i,
            _pd,
        } = self;
        R410 {
            e12,
            e13,
            e23,
            e1p: S::split(e1i).ep,
            e1n: S::split(e1i).en,
            e2p: S::split(e2i).ep,
            e2n: S::split(e2i).en,
            e3p: S::split(e3i).ep,
            e3n: S::split(e3i).en,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 {
            e12,
            e13,
            e23,
            e1p,
            e1n,
            e2p,
            e2n,
            e3p,
            e3n,
            ..
        } = v;
        Self {
            e12,
            e13,
            e23,
            e1i: S::join(e1p, e1n),
            e2i: S::join(e2p, e2n),
            e3i: S::join(e3p, e3n),
            _pd: PhantomData,
        }
    }
}

impl<T: Field + Copy, S: Space> Inner<Sphere<T>> for DLine<T, S> {
    type Output = Pair<T, S>;
}

impl<T: Field + Copy, S: Space> Inner<Plane<T, S>> for DLine<T, S> {
    type Output = FPoint<T, S>;
}
