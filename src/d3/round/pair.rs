use core::marker::PhantomData;

use num_traits::zero;
use simba::scalar::RealField;

use super::super::dual::DPlane;
use super::super::flat::Line;
use super::{Circle, Point};
use crate::{Field, Inner, Multivec, Outer, Scalar, R410, Space, Euclidean};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pair<T, S = Euclidean> {
    pub(crate) e12: T,
    pub(crate) e13: T,
    pub(crate) e23: T,
    pub(crate) e1p: T,
    pub(crate) e1n: T,
    pub(crate) e2p: T,
    pub(crate) e2n: T,
    pub(crate) e3p: T,
    pub(crate) e3n: T,
    pub(crate) epn: T,
    pub(super) _pd: PhantomData<S>,
}

impl<T: RealField + Copy, S: Space> Pair<T, S> {
    #[inline]
    pub fn decompose(self) -> Option<(Point<T, S>, Point<T, S>)> {
        let s = self.inner(self).0.try_sqrt()?;

        let pair = self.into_mv();
        let plane = S::infinity() | pair;

        if plane.e1.is_zero() && plane.e2.is_zero() && plane.e3.is_zero() {
            // Second point is infinity
            //Some((Point::from_mv(Point::no().into_mv() | pair).normalize(), Point::ni()))
            None
        } else {
            Some((
                Point::from_mv((-pair + s) | plane).normalize(),
                Point::from_mv((pair + s) | plane).normalize(),
            ))
        }
    }

    #[inline]
    pub fn norm_squared(self) -> T {
        self.into_mv().norm_squared()
    }

    /// Extends the pair into the infinite line connecting the two.
    #[inline]
    pub fn extend(self) -> Line<T, S> {
        Line::from_mv(self.into_mv() ^ S::infinity())
    }

    /// Constructs the dual form of the plane halfway between the two points.
    #[inline]
    pub fn midplane(self) -> DPlane<T, S> {
        // ei · self
        DPlane::from_mv(S::infinity() | self.into_mv())
    }
}

impl<T: Field + Copy, S: Space> Multivec for Pair<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Pair {
            e12,
            e13,
            e23,
            e1p,
            e1n,
            e2p,
            e2n,
            e3p,
            e3n,
            epn,
            _pd,
        } = self;
        R410 {
            e12,
            e13,
            e23,
            e1p,
            e1n,
            e2p,
            e2n,
            e3p,
            e3n,
            epn,
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
            epn,
            ..
        } = v;
        Self {
            e12,
            e13,
            e23,
            e1p,
            e1n,
            e2p,
            e2n,
            e3p,
            e3n,
            epn,
            _pd: PhantomData,
        }
    }
}

/// Construct the circle passing through all 3 points.
impl<T: Field + Copy, S: Space> Outer<Point<T, S>> for Pair<T, S> {
    type Output = Circle<T>;
}

impl<T: Field + Copy, S: Space> Inner for Pair<T, S> {
    type Output = Scalar<T>;
}

/// Construct the point at the intersection of the given plane and the line between the pair of points.
impl<T: Field + Copy, S: Space> Inner<DPlane<T, S>> for Pair<T, S> {
    type Output = Point<T, S>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decompose() {
        let p1 : Point<_> = Point::new([1.0, 0.0, 0.0]).normalize();
        let p2 : Point<_> = Point::new([3.0, 4.0, 5.0]).normalize();
        let p3 : Point<_> = Point::new([3.0, 4.0, 0.0]).normalize();

        assert_eq!(p1.outer(p2).decompose(), Some((p1, p2)));
        assert_eq!(p1.outer(p3).decompose(), Some((p1, p3)));
        assert_eq!(p2.outer(p3).decompose(), Some((p2, p3)));
        assert_eq!(p2.outer(p1).decompose(), Some((p2, p1)));
        assert_eq!(p3.outer(p1).decompose(), Some((p3, p1)));
        assert_eq!(p3.outer(p2).decompose(), Some((p3, p2)));

        /*
        assert_eq!(p1.outer(Point::ni()).decompose(), Some((p1, Point::ni())));
        assert_eq!(p2.outer(Point::ni()).decompose(), Some((p2, Point::ni())));
        assert_eq!(p3.outer(Point::ni()).decompose(), Some((p3, Point::ni())));

        assert_eq!(Point::ni().outer(p1).decompose(), Some((p1, Point::ni())));
        assert_eq!(Point::ni().outer(p2).decompose(), Some((p2, Point::ni())));
        assert_eq!(Point::ni().outer(p3).decompose(), Some((p3, Point::ni())));
        */
        assert_eq!(p1.outer(Point::ni()).decompose(), None);
        assert_eq!(p2.outer(Point::ni()).decompose(), None);
        assert_eq!(p3.outer(Point::ni()).decompose(), None);

        assert_eq!(Point::ni().outer(p1).decompose(), None);
        assert_eq!(Point::ni().outer(p2).decompose(), None);
        assert_eq!(Point::ni().outer(p3).decompose(), None);
    }
}
