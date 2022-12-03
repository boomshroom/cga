use core::fmt::{self, Display, Formatter, LowerExp, UpperExp};
use core::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use simba::scalar::{RealField, SubsetOf, SupersetOf};
#[cfg(feature = "nalgebra")]
use simba::simd::SimdBool;
use simba::simd::SimdValue;

use num_traits::{one, zero, Zero};

use super::super::direction::DVector;
use super::super::dual::DPlane;
use super::super::round::{Point, origin};
use super::super::transform::{Rotor, Translator};
use super::Bivector;
use crate::{Field, Inner, Multivec, Outer, Reflect, Scalar, Space, R410, Euclidean};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<T> {
    pub(crate) e1: T,
    pub(crate) e2: T,
    pub(crate) e3: T,
}

impl_traits! (Vector{e1, e2, e3, []});

impl<T: Field> Vector<T> {
    pub fn new(e1: T, e2: T, e3: T) -> Self {
        Self { e1, e2, e3 }
    }

    pub fn x(self) -> T {
        self.e1
    }
    pub fn y(self) -> T {
        self.e2
    }
    pub fn z(self) -> T {
        self.e3
    }
}

impl<T: Copy> From<[T; 3]> for Vector<T> {
    fn from(v: [T; 3]) -> Self {
        Self {
            e1: v[0],
            e2: v[1],
            e3: v[2],
        }
    }
}

impl<T: Field + Copy> Multivec for Vector<T> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Vector { e1, e2, e3 } = self;
        R410 {
            e1,
            e2,
            e3,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 { e1, e2, e3, .. } = v;
        Self { e1, e2, e3 }
    }
}

impl<T: Field + Copy> Vector<T> {
    /// Constructs the point at the coordinates pointed to by this vector.
    #[inline]
    pub fn into_point<S: Space>(self) -> Point<T, S> {
        let half = T::from_subset(&0.5);
        let no = origin();
        let ni = S::infinity();

        /*
        let square = self.norm_squared() * half;
        Point {
            e1: self.e1,
            e2: self.e2,
            e3: self.e3,
            ep: square - half.clone(),
            en: square + half,
        }*/

        let v = self.into_mv();
        Point::from_mv(v + ni * (v * v) * half + no)
    }

    /// Constructs the plane perpendicular to this vector at a distance `dist` from the origin.
    #[inline]
    pub fn normal<S: Space>(self, dist: T) -> DPlane<T, S> {
        DPlane::from_mv(self.into_mv() + S::infinity() * dist)
    }

    /*
    /// Converts the free vector into a tangent vector.
    #[inline]
    pub fn as_tangent(self) -> TVector<T> {
        TVector::from_mv(self.into_mv() * origin())
    }*/

    /// Convert the vector into a translator that brings the origin to the vector's coordinates.
    #[inline]
    pub fn into_translator<S: Space>(self) -> Translator<T, S> {
        self.as_direction().into_translator()
    }

    /// Converts the free vector into a direction vector.
    /// Also seen as the bivector part of a translator.
    #[inline]
    pub fn as_direction<S: Space>(self) -> DVector<T, S> {
        DVector::from_mv(self.into_mv() * S::infinity())
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let norm = self.norm();
        if norm.is_zero() {
            self
        } else {
            Self::from_mv(self.into_mv() / norm)
        }
    }

    #[inline]
    pub fn norm(self) -> T {
        self.into_mv().norm()
    }

    #[inline]
    pub fn norm_squared(self) -> T {
        self.into_mv().norm_squared()
    }

    pub fn dot(self, rhs: Self) -> T {
        self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3
    }
}

impl<T: Field + Copy + Display> Display for Vector<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Vector({})", self.into_mv())
    }
}

impl<T: Field + Copy + LowerExp> LowerExp for Vector<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Vector({:e})", self.into_mv())
    }
}

impl<T: Field + Copy + UpperExp> UpperExp for Vector<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Vector({:E})", self.into_mv())
    }
}

impl<T: Field + Copy> Inner for Vector<T> {
    type Output = Scalar<T>;
    /*
    #[inline]
    fn inner(self, rhs: Self) -> T {
        self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3
    }
    */
}

impl<T: Field + Copy> Outer for Vector<T> {
    type Output = Bivector<T>;
    #[inline]
    fn outer(self, rhs: Self) -> Bivector<T> {
        Bivector {
            e12: self.e1 * rhs.e2 - self.e2 * rhs.e1,
            e13: self.e1 * rhs.e3 - self.e3 * rhs.e1,
            e23: self.e2 * rhs.e3 - self.e3 * rhs.e2,
        }
    }
}

impl<T: Field + Copy> Reflect<Self> for Vector<T> {
    #[inline]
    fn reflect(self, object: Self) -> Self {
        let m = self.into_mv();
        Self::from_mv(m.reverse() * object.into_mv() * m)
    }
}

impl<T: Field + Copy> Mul for Vector<T> {
    type Output = Rotor<T>;
    /// Constructs the rotor that brings an object twice the angle between the two vectors.
    fn mul(self, rhs: Self) -> Rotor<T> {
        Rotor {
            s: self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3,
            e12: self.e1 * rhs.e2 - self.e2 * rhs.e1,
            e13: self.e1 * rhs.e3 - self.e3 * rhs.e1,
            e23: self.e2 * rhs.e3 - self.e3 * rhs.e2,
        }
    }
}

impl<T: Field + Copy> Add for Vector<T> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::from_mv(self.into_mv() + rhs.into_mv())
    }
}

impl<T: Field + Copy> Sub for Vector<T> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::from_mv(self.into_mv() - rhs.into_mv())
    }
}

impl<T: Field + Copy> Zero for Vector<T> {
    #[inline]
    fn zero() -> Self {
        Self::from_mv(zero())
    }
    #[inline]
    fn is_zero(&self) -> bool {
        (self.e1.is_zero() && self.e2.is_zero() && self.e3.is_zero())
            || self.norm_squared().is_zero()
    }
}

impl<T: Field + Copy> Mul<T> for Vector<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self::from_mv(self.into_mv() * rhs)
    }
}

impl<T: Field + Copy> Div<T> for Vector<T> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: T) -> Self {
        Self::from_mv(self.into_mv() / rhs)
    }
}

impl<T: Field + Copy> MulAssign<T> for Vector<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T: Field + Copy> DivAssign<T> for Vector<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T: Field + Copy> Neg for Vector<T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        self * -one::<T>()
    }
}

#[cfg(feature = "nalgebra")]
impl<T: Field + Copy> nalgebra::Normed for Vector<T> {
    type Norm = T;
    #[inline]
    fn norm(&self) -> T {
        let n = self.into_mv().norm_squared();
        assert!(n.is_simd_positive().all(), "Degenerate vector: {:?}", n);
        n.simd_sqrt()
    }
    #[inline]
    fn norm_squared(&self) -> T {
        self.into_mv().norm_squared()
    }
    #[inline]
    fn scale_mut(&mut self, n: T) {
        *self *= n
    }
    #[inline]
    fn unscale_mut(&mut self, n: T) {
        assert!(!n.is_zero(), "Zero norm: {:?} (vec: {:?})", n, self);
        *self /= n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_norm() {
        let o = Vector::new(0.0, 0.0, 0.0);
        let p1 = Vector::new(1.0, 0.0, 0.0);
        let p2 = Vector::new(3.0, 4.0, 5.0);
        let p3 = Vector::new(3.0, 4.0, 0.0);

        assert_eq!(o.norm_squared(), 0.0);
        assert_eq!(p1.norm_squared(), 1.0);
        assert_eq!(p2.norm_squared(), 50.0);
        assert_eq!(p3.norm_squared(), 25.0);
        assert_eq!(o.norm(), 0.0);
        assert_eq!(p1.norm(), 1.0);
        assert_eq!(p3.norm(), 5.0);

        // Triangle inequality
        assert!((p1 + p2).norm() <= p1.norm() + p2.norm());
        assert!((p1 + p3).norm() <= p1.norm() + p3.norm());
        assert!((p2 + p3).norm() <= p2.norm() + p3.norm());
    }
}
