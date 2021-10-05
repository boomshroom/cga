use num_traits::zero;

use super::super::flat::Plane;
use super::super::dual::DLine;
use super::{Point, Sphere};
use crate::{R410, Multivec, Field, Outer};

#[derive(Copy, Clone, Debug)]
pub struct Circle<T: Field> {
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
}

impl<T: Field + Copy> Circle<T> {
    /// Extends the circle into the infinite plane containing it.
    pub fn extend(self) -> Plane<T> {
        Plane {
            e123i: self.e123,
            e12pn: self.e12p - self.e12n,
            e13pn: self.e13p - self.e13n,
            e23pn: self.e23p - self.e23n,
        }
    }

    /// Constructs the dual form of the perpendicular line through the circle's center
    pub fn axis(self) -> DLine<T> {
        DLine::from_mv(Point::ni().into_mv() | self.into_mv())
        /*
        DLine {
            e12: self.e12p - self.e12n,
            e13: self.e13p - self.e13n,
            e23: self.e23p - self.e23n,
            e1i: -self.e1pn,
            e2i: -self.e2pn,
            e3i: -self.e3pn,
        }
        */
    }
}
impl<T: Field + Copy> Multivec for Circle<T> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Circle{
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
        } = self;
        R410{
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
        let R410{
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
        Self{
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
        }
    }
}

impl<T: Field + Copy> Outer<Point<T>> for Circle<T> {
    type Output = Sphere<T>;
    /// Construct the sphere passing through the given point and this circle.
    fn outer(self, p: Point<T>) -> Sphere<T> {
        Sphere {
            e123p: self.e123 * p.ep - self.e12p * p.e3 + self.e13p * p.e2 - self.e23p * p.e1,
            e123n: self.e123 * p.en - self.e12n * p.e3 + self.e13n * p.e2 - self.e23p * p.e1,
            e12pn: self.e12p * p.en - self.e12n * p.ep + self.e1pn * p.e2 - self.e2pn * p.e1,
            e13pn: self.e13p * p.en - self.e13n * p.ep + self.e1pn * p.e3 - self.e3pn * p.e1,
            e23pn: self.e23p * p.en - self.e23n * p.ep + self.e2pn * p.e3 - self.e3pn * p.e2,
        }
    }
}
