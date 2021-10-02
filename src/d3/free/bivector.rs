use core::ops::Mul;

use simba::simd::SimdRealField as Field;

use super::{Trivector, Vector};
use crate::{Inner, Outer};

#[derive(Copy, Clone)]
pub struct Bivector<T: Field> {
    pub(crate) e12: T,
    pub(crate) e13: T,
    pub(crate) e23: T,
}

impl<T: Field> Inner for Bivector<T> {
    type Output = T;
    fn inner(self, rhs: Self) -> T {
        -self.e12 * rhs.e12 - self.e13 * rhs.e13 - self.e23 * rhs.e23
    }
}

impl<T: Field> Outer<Vector<T>> for Bivector<T> {
    type Output = Trivector<T>;
    fn outer(self, rhs: Vector<T>) -> Trivector<T> {
        Trivector {
            e123: self.e12 * rhs.e3 - self.e13 * rhs.e2 + self.e23 * rhs.e1,
        }
    }
}

impl<T: Field + Copy> Mul<Trivector<T>> for Bivector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Trivector<T>) -> Vector<T> {
        Vector {
            e1: -self.e23 * rhs.e123,
            e2: self.e13 * rhs.e123,
            e3: -self.e12 * rhs.e123,
        }
    }
}
