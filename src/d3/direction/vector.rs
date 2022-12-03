use core::marker::PhantomData;
use core::ops::{Div, Mul, Neg};

use num_traits::{one, zero};

use super::super::transform::Translator;
use crate::{Euclidean, Field, Multivec, Space, R410};

/// A light-like direction vector
/// The bivector part of a translator
#[derive(Debug, Copy, Clone)]
pub struct DVector<T, S = Euclidean> {
    e1i: T,
    e2i: T,
    e3i: T,
    _pd: PhantomData<S>,
}

impl<T: Field + Copy, S: Space> DVector<T, S> {
    /// Converts the direction vector into a translator that moves along the direction the same distance.
    pub fn into_translator(self) -> Translator<T, S> {
        let half = T::from_subset(&0.5);
        (self * half).exp()
    }

    /// Computes e^self. Has the same effect as converting to a translator with twice the distance.
    pub fn exp(self) -> Translator<T, S> {
        Translator::from_mv(-self.into_mv() + one::<T>())
    }
}

impl<T: Field + Copy, S: Space> Multivec for DVector<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let DVector { e1i, e2i, e3i, _pd } = self;
        R410 {
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
        let R410 { e1p, e1n, e2p, e2n, e3p, e3n, .. } = v;
        Self {
            e1i: S::join(e1p, e1n),
            e2i: S::join(e2p, e2n),
            e3i: S::join(e3p, e3n),
            _pd: PhantomData,
        }
    }
}


impl<T: Field + Copy, S: Space> Div<T> for DVector<T, S> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: T) -> Self {
        Self::from_mv(self.into_mv() / rhs)
    }
}

impl<T: Field + Copy, S: Space> Mul<T> for DVector<T, S> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self::from_mv(self.into_mv() * rhs)
    }
}

impl<T: Field + Copy, S: Space> Neg for DVector<T, S> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::from_mv(-self.into_mv())
    }
}
