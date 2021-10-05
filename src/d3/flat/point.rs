use num_traits::zero;
use core::ops::Mul;

use crate::{R410, Multivec, Field};
use super::super::transform::{Translator};

#[derive(Copy, Clone, Debug)]
pub struct FPoint<T: Field + Copy> {
    /// Corresponds to both e1p and e1n
    pub(crate) e1i: T,
    /// Corresponds to both e2p and e2n
    pub(crate) e2i: T,
    /// Corresponds to both e3p and e3n
    pub(crate) e3i: T,
    pub(crate) epn: T,
}
impl<T: Field + Copy> Multivec for FPoint<T> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Self{ e1i, e2i, e3i, epn } = self;
        R410{
            e1p: e1i,
            e1n: e1i,
            e2p: e2i,
            e2n: e2i,
            e3p: e3i,
            e3n: e3i,
            epn,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410{ e1p, e2p, e3p, epn, .. } = v;
        Self{ e1i: e1p, e2i: e2p, e3i: e3p, epn }
    }
}

impl<T: Field + Copy> Mul for FPoint<T> {
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