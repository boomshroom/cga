use core::marker::PhantomData;

use num_traits::zero;

use super::super::dual::DLine;
use super::super::flat::Plane;
use super::super::round::{Point, Sphere};
use crate::{Field, Multivec, Outer, R410, Space, Euclidean};


/// An object containing the position and orientation of a tangent plane.
#[derive(Copy, Clone, Debug)]
pub struct TBivector<T: Field, S = Euclidean> {
    pub(crate) e123: T,
    pub(crate) e12p: T,
    pub(crate) e12n: T,
    pub(crate) e13p: T,
    pub(crate) e13n: T,
    pub(crate) e23p: T,
    pub(crate) e23n: T,
    _pd: PhantomData<S>,
}

impl<T: Field + Copy, S: Space> TBivector<T, S> {
    /// Extends the tangent bivector into a plane passing through its location
    pub fn into_plane(self) -> Plane<T, S> {
        Plane::from_mv(self.into_mv() ^ S::infinity())
    }
}

impl<T: Field + Copy, S: Space> Multivec for TBivector<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let TBivector { e123, e12p, e12n, e13p, e13n, e23p, e23n, _pd } = self;
        R410 { e123, e12p, e12n, e13p, e13n, e23p, e23n, ..zero() }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 { e123, e12p, e12n, e13p, e13n, e23p, e23n, e1pn, e2pn, e3pn, .. } = v;
        assert!(e1pn.is_zero() && e2pn.is_zero(), && e3pn.is_zero());
        Self { e123, e12p, e12n, e13p, e13n, e23p, e23n, _pd: PhantomData }
    }
}