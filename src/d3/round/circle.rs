use core::marker::PhantomData;

use num_traits::zero;


use super::super::dual::DLine;
use super::super::flat::Plane;
use super::{Point, Sphere};
use crate::{Field, Multivec, Outer, R410, Space, Euclidean};

#[derive(Copy, Clone, Debug)]
pub struct Circle<T: Field, S = Euclidean> {
    pub(crate) e123: T,
    pub(crate) e12p: T,
    pub(crate) e12n: T,
    pub(crate) e13p: T,
    pub(crate) e13n: T,
    pub(crate) e23p: T,
    pub(crate) e23n: T,
    pub(crate) e1pn: T,
    pub(crate) e2pn: T,
    pub(crate) e3pn: T,
    _pd: PhantomData<S>,
}

impl<T: Field + Copy, S: Space> Circle<T, S> {
    /// Extends the circle into the infinite plane containing it.
    pub fn extend(self) -> Plane<T, S> {
        Plane::from_mv(self.into_mv() ^ S::infinity())
    }

    /// Constructs the dual form of the perpendicular line through the circle's center
    pub fn axis(self) -> DLine<T, S> {
        DLine::from_mv(S::infinity() | self.into_mv())
    }
}

/*
impl<T: Field + Copy + AbsDiffEq, S: Space> Circle<T, S> {
    /// Compresses this circle to a tangent bivector.
    /// Returns `None` if the radius is not 0.
    pub fn as_tangent(self) -> Option<TBivector<T, S>> {
        if abs_diff_eq!(self.into_mv().norm_squared(), T::zero()) {
            None
        } else {
            Some(TBivector::from_mv(self.into_mv()))
        }
    }
}
*/

impl<T: Field + Copy, S: Space> Multivec for Circle<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Circle {
            e123,
            e12p,
            e12n,
            e13p,
            e13n,
            e23p,
            e23n,
            e1pn,
            e2pn,
            e3pn,
            _pd,
        } = self;
        R410 {
            e123,
            e12p,
            e12n,
            e13p,
            e13n,
            e23p,
            e23n,
            e1pn,
            e2pn,
            e3pn,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 {
            e123,
            e12p,
            e12n,
            e13p,
            e13n,
            e23p,
            e23n,
            e1pn,
            e2pn,
            e3pn,
            ..
        } = v;
        Self {
            e123,
            e12p,
            e12n,
            e13p,
            e13n,
            e23p,
            e23n,
            e1pn,
            e2pn,
            e3pn,
            _pd: PhantomData,
        }
    }
}

/// Construct the sphere passing through the given point and this circle.
impl<T: Field + Copy, S: Space> Outer<Point<T, S>> for Circle<T, S> {
    type Output = Sphere<T, S>;
}
