use core::marker::PhantomData;
use core::ops::{Div, Mul};

use num_traits::{one, zero};

use super::super::dual::DLine;
use super::super::flat::Plane;
use super::{Point, Sphere};
use crate::{Field, Multivec, Outer, R410, Space, Euclidean};

/// A tangent vector
pub struct TVector<T: Field, S = Euclidean> {
    e1o: T,
    e2o: T,
    e3o: T,
    _pd: PhantomData<S>,
}

impl<T: Field + Copy, S: Space> TVector<T, S> {
    /// Extends the tangent vector into a line passing through its location
    pub fn into_line(self) -> Line<T, S> {
        Line::from_mv(self.into_mv() ^ S::infinity())
    }

    pub fn position(self) -> Point<T, S> {
        let v = self.into_mv();
        Point::from_mv(v / (-S::infinity() | v))
    }

    pub fn direction(self) -> DVector<T, S> {
        let v = self.into_mv();
        DVector::from_mv(-(S::infinity() | v) ^ S::infinity())
    }
}

impl<T: Field + Copy, S: Space> Multivec for TVector<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let TVector { e1o, e2o, e3o, _pd } = self;
        R410 {
            e1p: -e1o,
            e1n: e1o,
            e2p: -e2o,
            e2n: e2o,
            e3p: -e3o,
            e3n: e3o,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 { e1n, e2n, e3n, .. } = v;
        Self {
            e1o: e1n,
            e2o: e2n,
            e3o: e3n,
            _pd: PhantomData,
        }
    }
}

impl<T: Field + Copy, S: Space> Div<T> for TVector<T, S> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: T) -> Self {
        Self::from_mv(self.into_mv() / rhs)
    }
}

impl<T: Field + Copy, S: Space> Mul<T> for TVector<T, S> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self::from_mv(self.into_mv() * rhs)
    }
}
