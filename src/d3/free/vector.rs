use core::ops::{Add, Mul, Div, Neg, MulAssign, DivAssign};

use simba::simd::SimdValue;
use simba::scalar::{RealField, SubsetOf, SupersetOf};

use num_traits::{Zero, one, zero};

use super::super::dual::DPlane;
use super::super::round::Point;
use super::super::transform::{Rotor, Translator};
use super::Bivector;
use crate::{R410, Multivec, Field, Scalar, Inner, Outer, Reflect};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<T> {
    pub(crate) e1: T,
    pub(crate) e2: T,
    pub(crate) e3: T,
}

impl<T: Field> Vector<T> {
    pub fn new(e1: T, e2: T, e3: T) -> Self {
        Self { e1, e2, e3 }
    }

    /// Constructs the plane perpendicular to this vector at a distance `dist` from the origin.
    pub fn normal(self, dist: T) -> DPlane<T> {
        DPlane {
            e1: self.e1,
            e2: self.e2,
            e3: self.e3,
            ei: dist,
        }
    }

    pub fn x(self) -> T { self.e1 }
    pub fn y(self) -> T { self.e2 }
    pub fn z(self) -> T { self.e3 }
}

impl<T: Field + Copy> From<[T; 3]> for Vector<T> {
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
        R410 { e1, e2, e3, ..zero() }
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
    pub fn into_point(self) -> Point<T> {
        let half = T::from_subset(&0.5);
        let no = Point::no().into_mv();
        let ni = Point::ni().into_mv();

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

    /// Convert the vector into a translator that brings the origin to the vector's coordinates.
    #[inline]
    pub fn into_translator(self) -> Translator<T> {
        let half = T::from_subset(&-0.5);
        Translator::from_mv((self * half).into_mv() * Point::ni().into_mv() + one::<R410<T>>())
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

impl<T: Field + Copy> Zero for Vector<T> {
    #[inline]
    fn zero() -> Self { Self::from_mv(zero()) }
    #[inline]
    fn is_zero(&self) -> bool { self == &Self::zero() }
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
    fn neg(self) -> Self { self * -one::<T>() }
}

impl<T, U> SubsetOf<Vector<U>> for Vector<T>
where U: SupersetOf<T>
{
    fn to_superset(&self) -> Vector<U> {
        Vector{
            e1: U::from_subset(&self.e1),
            e2: U::from_subset(&self.e2),
            e3: U::from_subset(&self.e3),
        }
    }
    fn from_superset_unchecked(element: &Vector<U>) -> Self {
        Vector{
            e1: element.e1.to_subset_unchecked(),
            e2: element.e2.to_subset_unchecked(),
            e3: element.e3.to_subset_unchecked(),
        }
    }
    fn is_in_subset(element: &Vector<U>) -> bool {
        element.e1.is_in_subset() && element.e2.is_in_subset() && element.e3.is_in_subset()
    }
}

impl<T> SimdValue for Vector<T>
where
    T: Field,
    <T as SimdValue>::Element: RealField
{

    type Element = Vector<T::Element>;
    type SimdBool = T::SimdBool;
    fn lanes() -> usize { T::lanes() }
    fn splat(val: Self::Element) -> Self {
        Vector {
            e1: T::splat(val.e1),
            e2: T::splat(val.e2),
            e3: T::splat(val.e3),
        }
    }
    fn extract(&self, i: usize) -> Self::Element {
        Vector {
            e1: self.e1.extract(i),
            e2: self.e2.extract(i),
            e3: self.e3.extract(i),
        }
    }
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        Vector {
            e1: self.e1.extract_unchecked(i),
            e2: self.e2.extract_unchecked(i),
            e3: self.e3.extract_unchecked(i),
        }
    }
    fn replace(&mut self, i: usize, val: Self::Element) {
        self.e1.replace(i, val.e1);
        self.e2.replace(i, val.e2);
        self.e3.replace(i, val.e3);
    }
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        self.e1.replace_unchecked(i, val.e1);
        self.e2.replace_unchecked(i, val.e2);
        self.e3.replace_unchecked(i, val.e3);
    }
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        Vector {
            e1: self.e1.select(cond, other.e1),
            e2: self.e2.select(cond, other.e2),
            e3: self.e3.select(cond, other.e3),
        }
    }
}

#[cfg(feature = "nalgebra")]
impl<T: RealField + Copy> nalgebra::Normed for Vector<T> {
    type Norm = T;
    #[inline]
    fn norm(&self) -> T {
        let n = self.inner(*self);
        assert!(n.is_positive(), "Degenerate vector: {:?}", n);
        n.sqrt()
    }
    #[inline]
    fn norm_squared(&self) -> T { self.inner(*self) }
    #[inline]
    fn scale_mut(&mut self, n: T) { *self *= n }
    #[inline]
    fn unscale_mut(&mut self, n: T) { *self /= n }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_norm() {
        let o = Vector::new(0.0, 0.0, 0.0);
        let p1 = Vector::new(1.0, 0.0, 0.0);
        let p2 = Vector::new(3.0, 4.0, 5.0);
        let p3 = Vector::new(3.0, 4.0);

        assert_eq!(o.norm_squared(), 0.0);
        assert_eq!(p1.norm_squared(), 1.0);
        assert_eq!(p2.norm_squared(), 50.0);
        assert_eq!(p3.norm_squared(), 25.0);
        assert_eq!(o.norm(), 0.0);
        assert_eq!(p1.norm(), 1.0);
        assert_eq!(p3.norm(), 5.0);
    }
}