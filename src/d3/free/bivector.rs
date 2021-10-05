use core::ops::{Mul, Div};

use num_traits::zero;

use simba::simd::SimdRealField as Field;

use super::{Trivector, Vector};
use super::super::transform::Rotor;
use crate::{R410, Multivec, Scalar, Inner, Outer};

#[derive(Copy, Clone, Debug)]
pub struct Bivector<T: Field> {
    pub(crate) e12: T,
    pub(crate) e13: T,
    pub(crate) e23: T,
}

impl<T: Field + Copy> Multivec for Bivector<T> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Bivector{e12, e13, e23} = self;
        R410{e12, e13, e23, ..zero()}
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410{e12, e13, e23, ..} = v;
        Bivector{e12, e13, e23}
    }
}

impl<T: Field> Bivector<T> {
    pub fn dot(self, rhs: Self) -> T {
        -self.e12 * rhs.e12 - self.e13 * rhs.e13 - self.e23 * rhs.e23
    }
}

impl<T: Field + Copy> Bivector<T> {
    #[inline]
    pub fn normalized(self) -> Self {
        self / self.norm()
    }
    #[inline]
    pub fn norm(self) -> T {
        self.norm_squared().simd_sqrt()
    }
    #[inline]
    pub fn norm_squared(self) -> T {
        self.into_mv().norm_squared()
    }

    #[inline]
    pub fn with_angle(self, angle: T) -> Rotor<T> {
        let half = T::from_subset(&0.5);
        let (sin, cos) = (angle * half).simd_sin_cos();
        Rotor{
            s: cos,
            e12: self.e12 * sin,
            e13: self.e13 * sin,
            e23: self.e23 * sin,
        }
    }
}

impl<T: Field + Copy> Inner for Bivector<T> {
    type Output = Scalar<T>;
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

impl<T: Field + Copy> Div<T> for Bivector<T> {
    type Output = Bivector<T>;
    #[inline]
    fn div(self, rhs: T) -> Bivector<T> {
        Self::from_mv(self.into_mv() / rhs)
    }
}

#[cfg(test)]
mod test {
    use super::Bivector;

    #[test]
    pub fn norm_squared() {
        let e12 = Bivector{e12: 1.0, e13: 0.0, e23: 0.0};
        let e13 = Bivector{e12: 0.0, e13: 1.0, e23: 0.0};
        let e23 = Bivector{e12: 0.0, e13: 0.0, e23: 1.0};

        assert_eq!(e12.norm_squared(), 1.0);
        assert_eq!(e13.norm_squared(), 1.0);
        assert_eq!(e23.norm_squared(), 1.0);

        let b1 = Bivector{e12: 3.0, e13: 4.0, e23: 0.0};
        assert_eq!(b1.norm_squared(), 25.0);

        let zero = Bivector{e12: 0.0, e13: 0.0, e23: 0.0};
        assert_eq!(zero.norm_squared(), 0.0);
    }

    #[test]
    pub fn norm() {
        let e12 = Bivector{e12: 1.0, e13: 0.0, e23: 0.0};
        let e13 = Bivector{e12: 0.0, e13: 1.0, e23: 0.0};
        let e23 = Bivector{e12: 0.0, e13: 0.0, e23: 1.0};

        assert_eq!(e12.norm(), 1.0);
        assert_eq!(e13.norm(), 1.0);
        assert_eq!(e23.norm(), 1.0);

        let b1 = Bivector{e12: 3.0, e13: 4.0, e23: 0.0};
        assert_eq!(b1.norm(), 5.0);

        let zero = Bivector{e12: 0.0, e13: 0.0, e23: 0.0};
        assert_eq!(zero.norm(), 0.0);
    }
}