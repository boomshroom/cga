use core::ops::Mul;
use core::marker::PhantomData;

use num_traits::{one, zero, Num};

use simba::scalar::{RealField, SubsetOf, SupersetOf};

use crate::{Dual, Field, Inner, Multivec, R410, Space, Euclidean};

use super::super::dual::DLine;
use super::super::direction::DVector;
use super::super::free::Vector;
use super::super::transform::Motor;
use super::super::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line<T, S = Euclidean> {
    /// Corresponds to both e12p and e12n
    pub(crate) e12i: T,
    /// Corresponds to both e13p and e13n
    pub(crate) e13i: T,
    /// Corresponds to both e23p and e23n
    pub(crate) e23i: T,
    pub(crate) e1pn: T,
    pub(crate) e2pn: T,
    pub(crate) e3pn: T,
    _pd: PhantomData<S>,
}

/*
L = vx eo1 + vy eo2 + vz eo3 + mx e23 + my e31 + mz e12
L ^ ei
*/

impl_traits!(Line{e12i, e13i, e23i, e1pn, e2pn, e3pn, [_pd: PhantomData]});

impl<T: Field + Copy, S: Space> Multivec for Line<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Line {
            e12i,
            e13i,
            e23i,
            e1pn,
            e2pn,
            e3pn,
            _pd,
        } = self;
        R410 {
            e12p: S::split(e12i).ep,
            e12n: S::split(e12i).en,
            e13p: S::split(e13i).ep,
            e13n: S::split(e13i).en,
            e23p: S::split(e23i).ep,
            e23n: S::split(e23i).en,
            e1pn,
            e2pn,
            e3pn,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 {
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
            e12i: S::join(e12p, e12n),
            e13i: S::join(e13p, e13n),
            e23i: S::join(e23p, e23n),
            e1pn,
            e2pn,
            e3pn,
            _pd: PhantomData,
        }
    }
}

impl<T: Field + Copy, S: Space> Line<T, S> {
    #[inline]
    pub fn into_vector(self) -> Vector<T> {
        let mink = R410 {
            epn: one(),
            ..zero()
        };
        Vector::from_mv(mink | self.into_mv())
    }

    #[inline]
    pub fn direction(self) -> DVector<T> {
        DVector::from_mv(-S::infinity() | self.into_mv())
    }

    /// Gets a relative coordinate for a point's position on this line. Exact value may not be meaningful, but can be used for comparisions.
    /// Returns None if the point is infinite.
    #[inline]
    pub fn position_on(self, p: Point<T, S>) -> Option<T> {
        if p.inner(Point::ni()).0.is_zero() {
            None
        } else {
            Some(-(self.into_vector().into_mv() | p.normalize().into_mv()).s)
        }
    }
}

impl<T: Field + Copy, S: Space> Dual for Line<T, S> {
    type Output = DLine<T, S::Dual>;
    /*
    #[inline]
    fn dual(self) -> DLine<T> {
        DLine {
            e12: self.e3pn,
            e13: -self.e2pn,
            e23: self.e1pn,
            e1i: self.e23i,
            e2i: -self.e13i,
            e3i: self.e12i,
        }
    }
    */
}

impl<T: Field + Copy, S: Space> Mul for Line<T, S> {
    type Output = Motor<T>;
    fn mul(self, rhs: Self) -> Motor<T> {
        Motor {
            s: self.e1pn * rhs.e1pn + self.e2pn * rhs.e2pn + self.e3pn * rhs.e3pn,
            e12: self.e1pn * rhs.e2pn - self.e2pn * rhs.e1pn,
            e13: self.e1pn * rhs.e3pn - self.e3pn * rhs.e1pn,
            e23: self.e2pn * rhs.e3pn - self.e3pn * rhs.e2pn,
            e1i: self.e12i * rhs.e2pn - self.e2pn * rhs.e12i - self.e13i * rhs.e3pn
                + self.e3pn * rhs.e13i,
            e2i: self.e12i * rhs.e1pn - self.e1pn * rhs.e12i - self.e23i * rhs.e3pn
                + self.e3pn * rhs.e23i,
            e3i: self.e13i * rhs.e1pn - self.e1pn * rhs.e13i + self.e23i * rhs.e2pn
                - self.e2pn * rhs.e23i,
            e123i: -self.e12i * rhs.e3pn - self.e3pn * rhs.e12i
                + self.e13i * rhs.e2pn
                + self.e2pn * rhs.e13i
                - self.e1pn * rhs.e23i
                - self.e23i * rhs.e1pn,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::d3::free::Vector;
    use crate::d3::transform::Transform;
    use crate::{Outer, Euclidean};

    #[test]
    fn test_position_on() {
        let p1 : Point<_> = Point::new([1.0, 0.0, 0.0]);
        let p2 : Point<_> = Point::new([3.0, 4.0, 5.0]);
        let p3 : Point<_> = Point::new([3.0, 4.0, 0.0]);

        let l1 = p1.outer(p2).extend();
        let l2 = p2.outer(p1).extend();
        assert_eq!(l1.position_on(p1), Some(2.0));
        assert_eq!(l1.position_on(p2), Some(47.0));
        assert_eq!(l2.position_on(p1), Some(-2.0));
        assert_eq!(l2.position_on(p2), Some(-47.0));
        assert!(
            l1.position_on(p1).unwrap() < l1.position_on(p2).unwrap(),
            "pos1: {:?}, pos2: {:?}",
            l1.position_on(p1),
            l1.position_on(p2)
        );
        assert!(
            l2.position_on(p1).unwrap() > l2.position_on(p2).unwrap(),
            "pos1: {:?}, pos2: {:?}",
            l2.position_on(p1),
            l2.position_on(p2)
        );

        let v = Vector::new(3.0, 4.0, 5.0).normalize();
        let l = p1.extend_along_vec(v.as_direction());
        let p2 = v.into_translator::<Euclidean>().transform(p1);
        let p3 = (v * -0.5).into_translator::<Euclidean>().transform(p1);
        assert!(
            l.position_on(p1).unwrap() < l.position_on(p2).unwrap(),
            "pos1: {:?}, pos2: {:?}",
            l1.position_on(p1),
            l1.position_on(p2)
        );
        assert!(
            l.position_on(p3).unwrap() < l.position_on(p1).unwrap(),
            "pos1: {:?}, pos2: {:?}",
            l1.position_on(p3),
            l1.position_on(p1)
        );

        assert_eq!(l1.position_on(Point::ni()), None);
        assert_eq!(l2.position_on(Point::ni()), None);
        assert_eq!(l.position_on(Point::ni()), None);

        assert_eq!(l1.position_on(Point::no()), Some(-0.0));
        assert_eq!(l2.position_on(Point::no()), Some(-0.0));
        assert_eq!(l.position_on(Point::no()), Some(-0.0));
    }
}
