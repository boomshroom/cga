use core::marker::PhantomData;

use num_traits::zero;
use simba::simd::SimdRealField as Field;

use super::super::flat::Line;
use super::super::round::{Circle, Pair, Sphere, Point};
use crate::{Inner, Multivec, Scalar, R410, Space, Euclidean};

#[derive(Copy, Clone, Debug)]
pub struct DSphere<T, S = Euclidean> {
    pub(crate) e1: T,
    pub(crate) e2: T,
    pub(crate) e3: T,
    pub(crate) ep: T,
    pub(crate) en: T,
    _pd: PhantomData<S>,
}

impl<T: Field + Copy, S: Space> DSphere<T, S> {
    /// Converts the dual sphere to standard form.
    pub fn undual(self) -> Sphere<T, S::Dual> {
        Sphere::from_mv(self.into_mv().undual())
    }

    pub fn dot(self, rhs: Self) -> T {
        self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3 + self.ep * rhs.ep - self.en * rhs.en
    }

    pub fn radius(self) -> T {
        self.inner(self).0.simd_sqrt()
    }

    pub fn center(self) -> Point<T, S> {
        let s = self.into_mv();
        Point::from_mv(s * S::infinity() * s)
    }
}

impl<T: Field + Copy, S: Space> Multivec for DSphere<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Self { e1, e2, e3, ep, en, _pd } = self;
        R410 {
            e1,
            e2,
            e3,
            ep,
            en,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 {
            e1, e2, e3, ep, en, ..
        } = v;
        Self { e1, e2, e3, ep, en, _pd: PhantomData }
    }
}

impl<T: Field + Copy, S: Space> Inner for DSphere<T, S> {
    type Output = Scalar<T>;
}

impl<T: Field + Copy, S: Space> Inner<Sphere<T, S>> for DSphere<T, S> {
    type Output = Circle<T, S>;
}

impl<T: Field + Copy, S: Space> Inner<Line<T, S>> for DSphere<T, S> {
    type Output = Pair<T, S>;
}
