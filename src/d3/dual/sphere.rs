use simba::simd::SimdRealField as Field;

use super::super::flat::Line;
use super::super::round::{Circle, Pair, Sphere};
use crate::Inner;

#[derive(Copy, Clone, Debug)]
pub struct DSphere<T: Field> {
    pub(crate) e1: T,
    pub(crate) e2: T,
    pub(crate) e3: T,
    pub(crate) ep: T,
    pub(crate) en: T,
}

impl<T: Field> DSphere<T> {
    /// Converts the dual sphere to standard form.
    pub fn undual(self) -> Sphere<T> {
        Sphere {
            e123p: self.en,
            e123n: self.ep,
            e23pn: -self.e1,
            e13pn: self.e2,
            e12pn: -self.e3,
        }
    }
}
impl<T: Field + Copy> DSphere<T> {
    pub fn radius(self) -> T {
        self.inner(self).simd_sqrt()
    }
}

impl<T: Field> Inner for DSphere<T> {
    type Output = T;
    fn inner(self, rhs: Self) -> T {
        self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3 + self.ep * rhs.ep - self.en * rhs.en
    }
}

impl<T: Field + Copy> Inner<Sphere<T>> for DSphere<T> {
    type Output = Circle<T>;
    fn inner(self, rhs: Sphere<T>) -> Circle<T> {
        Circle {
            e123: self.en * rhs.e123n - self.ep * rhs.e123p,
            e12p: self.en * rhs.e12pn + self.e3 * rhs.e123p,
            e12n: self.ep * rhs.e12pn + self.e3 * rhs.e123n,
            e13p: self.en * rhs.e13pn + self.e2 * rhs.e123p,
            e13n: self.ep * rhs.e13pn - self.e2 * rhs.e123n,
            e23p: self.en * rhs.e23pn + self.e1 * rhs.e123p,
            e23n: self.ep * rhs.e23pn + self.e1 * rhs.e123n,
            e1pn: -self.e3 * rhs.e13pn - self.e2 * rhs.e12pn,
            e2pn: self.e1 * rhs.e12pn - self.e3 * rhs.e23pn,
            e3pn: self.e2 * rhs.e23pn + self.e1 * rhs.e13pn,
        }
    }
}

impl<T: Field + Copy> Inner<Line<T>> for DSphere<T> {
    type Output = Pair<T>;
    fn inner(self, rhs: Line<T>) -> Pair<T> {
        Pair {
            e12: (self.ep - self.en) * rhs.e12i,
            e13: (self.ep - self.en) * rhs.e13i,
            e23: (self.ep - self.en) * rhs.e23i,
            e1p: -self.en * rhs.e1pn - self.e2 * rhs.e12i - self.e3 * rhs.e13i,
            e1n: -self.ep * rhs.e1pn - self.e2 * rhs.e12i - self.e3 * rhs.e13i,
            e2p: -self.en * rhs.e2pn + self.e1 * rhs.e12i - self.e3 * rhs.e23i,
            e2n: -self.ep * rhs.e2pn + self.e1 * rhs.e12i - self.e3 * rhs.e23i,
            e3p: -self.en * rhs.e3pn + self.e1 * rhs.e13i + self.e2 * rhs.e23i,
            e3n: -self.ep * rhs.e3pn + self.e1 * rhs.e13i + self.e2 * rhs.e23i,
            epn: self.e1 * rhs.e1pn + self.e2 * rhs.e2pn + self.e3 * rhs.e3pn,
        }
    }
}
