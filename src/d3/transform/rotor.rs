use core::ops::Mul;

use num_traits::zero;

use super::super::free::Vector;
use super::Transform;
use crate::{Field, Multivec, R410};

#[derive(Copy, Clone, Debug)]
pub struct Rotor<T: Field> {
    pub(crate) s: T,
    pub(crate) e12: T,
    pub(crate) e13: T,
    pub(crate) e23: T,
}

impl<T: Field + Copy> Multivec for Rotor<T> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Self { s, e12, e13, e23 } = self;
        R410 {
            s,
            e12,
            e13,
            e23,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 {
            s, e12, e13, e23, ..
        } = v;
        Self { s, e12, e13, e23 }
    }
}

impl<T: Field + Copy> Transform<Vector<T>> for Rotor<T> {
    #[inline]
    fn transform(self, p: Vector<T>) -> Vector<T> {
        let t = self.into_mv();
        Vector::from_mv(t.reverse() * p.into_mv() * t)
    }
}

impl<T: Field + Copy> Mul for Rotor<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::from_mv(self.into_mv() * rhs.into_mv())
    }
}
