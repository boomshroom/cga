use simba::scalar::RealField;
use simba::simd::SimdRealField as Field;

use super::super::dual::DPlane;
use super::super::flat::Line;
use super::{Circle, Point};
use crate::{Inner, Outer};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pair<T: Field> {
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
}

impl<T: Field> Pair<T> {
    /// Extends the pair into the infinite line connecting the two.
    pub fn extend(self) -> Line<T> {
        Line {
            e12i: self.e12,
            e13i: self.e13,
            e23i: self.e23,
            e1pn: self.e1p - self.e1n,
            e2pn: self.e2p - self.e2n,
            e3pn: self.e3p - self.e3n,
        }
    }

    /// Constructs the dual form of the plane halfway between the two points.
    fn midplane(self) -> DPlane<T> {
        // ei · self
        DPlane {
            e1: self.e1n - self.e1p,
            e2: self.e2n - self.e2p,
            e3: self.e3n - self.e3p,
            ei: self.epn,
        }
    }
}

impl<T: RealField + Copy> Pair<T> {
    pub fn decompose(self) -> Option<(Point<T>, Point<T>)> {
        let desc = self.inner(self);
        if desc.is_negative() {
            return None;
        }

        let s = desc.sqrt();
        let plane = self.midplane();

        let s_plane = DPlane {
            e1: plane.e1 * s,
            e2: plane.e2 * s,
            e3: plane.e3 * s,
            ei: plane.ei * s,
        };

        let mid = self.inner(plane);
        Some((
            Point {
                e1: s_plane.e1 + mid.e1,
                e2: s_plane.e2 + mid.e2,
                e3: s_plane.e3 + mid.e3,
                ep: s_plane.ei + mid.ep,
                en: s_plane.ei + mid.en,
            },
            Point {
                e1: s_plane.e1 - mid.e1,
                e2: s_plane.e2 - mid.e2,
                e3: s_plane.e3 - mid.e3,
                ep: s_plane.ei - mid.ep,
                en: s_plane.ei - mid.en,
            },
        ))
    }
}

impl<T: Field + Copy> Outer<Point<T>> for Pair<T> {
    type Output = Circle<T>;
    /// Construct the circle passing through all 3 points.
    fn outer(self, rhs: Point<T>) -> Circle<T> {
        Circle {
            e123: self.e12 * rhs.e3 - self.e13 * rhs.e2 + self.e23 * rhs.e1,
            e12p: self.e12 * rhs.ep - self.e1p * rhs.e2 + self.e2p * rhs.e1,
            e12n: self.e12 * rhs.en - self.e1n * rhs.e2 + self.e2n * rhs.e1,
            e13p: self.e13 * rhs.ep - self.e1p * rhs.e3 + self.e3p * rhs.e1,
            e13n: self.e13 * rhs.en - self.e1n * rhs.e3 + self.e3n * rhs.e1,
            e23p: self.e23 * rhs.ep - self.e2p * rhs.e3 + self.e3p * rhs.e2,
            e23n: self.e23 * rhs.en - self.e2n * rhs.e3 + self.e3n * rhs.e2,
            e1pn: self.e1p * rhs.en - self.e1n * rhs.ep + self.epn * rhs.e1,
            e2pn: self.e2p * rhs.en - self.e2n * rhs.ep + self.epn * rhs.e2,
            e3pn: self.e3p * rhs.en - self.e3n * rhs.ep + self.epn * rhs.e3,
        }
    }
}

impl<T: Field> Inner for Pair<T> {
    type Output = T;
    /// Construct the circle passing through all 3 points.
    fn inner(self, rhs: Self) -> T {
        -self.e12 * rhs.e12
            - self.e13 * rhs.e13
            - self.e23 * rhs.e23
            - self.e1p * rhs.e1p
            - self.e2p * rhs.e2p
            - self.e3p * rhs.e3p
            + self.e1n * rhs.e1n
            + self.e2n * rhs.e2n
            + self.e3n * rhs.e3n
            + self.epn * rhs.epn
    }
}

impl<T: Field + Copy> Inner<DPlane<T>> for Pair<T> {
    type Output = Point<T>;
    /// Construct the point at the intersection of the given plane and the line between the pair of points.
    fn inner(self, rhs: DPlane<T>) -> Point<T> {
        Point {
            e1: self.e12 * rhs.e2 + self.e13 * rhs.e3 + self.e1p * rhs.ei - self.e1n * rhs.ei,
            e2: -self.e12 * rhs.e1 + self.e23 * rhs.e3 + self.e2p * rhs.ei - self.e2n * rhs.ei,
            e3: -self.e13 * rhs.e1 - self.e23 * rhs.e2 + self.e3p * rhs.ei - self.e3n * rhs.ei,
            ep: -self.e1p * rhs.e1 - self.e2p * rhs.e2 - self.e3p * rhs.ei - self.epn * rhs.ei,
            en: -self.e1n * rhs.e1 - self.e2n * rhs.e2 - self.e3n * rhs.ei - self.epn * rhs.ei,
        }
    }
}
