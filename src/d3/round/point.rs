use simba::scalar::RealField;
use simba::simd::SimdValue;

use num_traits::{zero, one};

use super::super::dual::{DPlane, DSphere};
use super::super::flat::{FPoint, Line};
use super::super::free::Vector;
use super::super::transform::Transform;
use super::Pair;
use crate::{R410, Multivec, Field, Scalar, Inner, Outer};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point<T> {
    pub(crate) e1: T,
    pub(crate) e2: T,
    pub(crate) e3: T,
    pub(crate) ep: T,
    pub(crate) en: T,
}

impl<T: Field + Copy> Point<T> {
    #[inline]
    pub fn new(v: impl Into<Vector<T>>) -> Self {
        v.into().into_point()
    }

    #[inline]
    pub fn no() -> Self {
    	let half = T::from_subset(&0.5);
    	Point {
    		e1: zero(),
    		e2: zero(),
    		e3: zero(),
    		ep: -half,
    		en: half,
    	}
    }

    #[inline]
    pub(crate) fn ni() -> Self {
    	Point {
    		e1: zero(),
    		e2: zero(),
    		e3: zero(),
    		ep: one(),
    		en: one(),
    	}
    }

    /// Convert this point into a flat point.
    pub fn into_flat(self) -> FPoint<T> {
        FPoint {
            e1i: self.e1,
            e2i: self.e2,
            e3i: self.e3,
            epn: self.ep - self.en,
        }
    }

    /// Calculate the euclidean distance between two points
    #[inline]
    pub fn distance(self, other: Self) -> T {
        let n2 = T::from_subset(&-2.0);
        (self.normalize().dot(other.normalize())*n2).simd_sqrt()
    }

    /// The scalar product of two points. Returns negative a half of the distance squared between them.
    #[inline]
    pub fn dot(self, other: Self) -> T {
        self.inner(other).0
    }

    #[inline]
    fn normalize(self) -> Self {
        Self::from_mv(self.into_mv() / self.norm())
    }

    #[inline]
    fn norm(self) -> T {
        self.dot(Self::ni())
    }

    /// Constructs the dual form of the plane halfway between the two points.
    pub fn midplane(self, other: Self) -> DPlane<T> {
        debug_assert_eq!(self.ep - other.ep, self.en - other.en);
        DPlane {
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e3,
            e3: self.e3 - other.e3,
            ei: self.ep - other.ep,
        }
    }

    /// Constructs the dual form of the sphere centered at this point with the given radius.
    pub fn into_sphere(self, radius: T) -> DSphere<T> {
        let half = T::from_subset(&0.5);
        let r2 = radius.clone() * radius * half;
        DSphere {
            e1: self.e1,
            e2: self.e2,
            e3: self.e3,
            ep: self.ep - r2,
            en: self.en - r2,
        }
    }

    /// Constructs the line along `dir` that passes through self.
    #[inline]
    pub fn extend_along_vec(self, dir: Vector<T>) -> Line<T> {
        let dir = dir.into_translator();
        self.outer(dir.transform(self)).extend()
    }
}

impl<T: Field + Copy> Multivec for Point<T> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Point { e1, e2, e3, ep, en } = self;
        R410 { e1, e2, e3, ep, en, ..zero() }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 { e1, e2, e3, ep, en, .. } = v;
        Self { e1, e2, e3, ep, en }
    }
}

impl<T: RealField + Copy> Point<T> {
    /// gets the vector pointing to this point from the origin
    #[inline]
    pub fn from_origin(self) -> Option<Vector<T>> {
        let pair = Self::no().outer(self);
        if self.norm().is_zero() {
            None
        } else {
            // (x + x^2(ep + en)/2 + (en/2 - ep/2)) | (en/2 - ep/2)
            // x^2(ep + en)/2 | en/2 - x^2(ep + en)/2 | ep/2
            // x^2/2((en + ep)|(en - ep))
            // x^2/2(-2)
            // -x^2

            // (x + x^2ni/2 + no) | ni
            // x^2ni/2 | ni - 1
            // -x^2/2 - 1
            Some(pair.extend().into_vector().normalize() * Self::no().distance(self))
        }
    }
}

impl<T: Field + Copy> Inner for Point<T> {
    type Output = Scalar<T>;
}

impl<T: Field + Copy> Outer for Point<T> {
    type Output = Pair<T>;
    /// Join two points into a pair.
    #[inline]
    fn outer(self, rhs: Self) -> Pair<T> {
        Pair::from_mv(self.into_mv() ^ rhs.into_mv())
    }
}

impl<T> SimdValue for Point<T>
where
    T: Field,
    <T as SimdValue>::Element: RealField
{
    type Element = Point<T::Element>;
    type SimdBool = T::SimdBool;
    fn lanes() -> usize { T::lanes() }
    fn splat(val: Self::Element) -> Self {
        Point {
            e1: T::splat(val.e1),
            e2: T::splat(val.e2),
            e3: T::splat(val.e3),
            ep: T::splat(val.ep),
            en: T::splat(val.en),
        }
    }
    fn extract(&self, i: usize) -> Self::Element {
        Point {
            e1: self.e1.extract(i),
            e2: self.e2.extract(i),
            e3: self.e3.extract(i),
            ep: self.ep.extract(i),
            en: self.en.extract(i),
        }
    }
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        Point {
            e1: self.e1.extract_unchecked(i),
            e2: self.e2.extract_unchecked(i),
            e3: self.e3.extract_unchecked(i),
            ep: self.ep.extract_unchecked(i),
            en: self.en.extract_unchecked(i),
        }
    }
    fn replace(&mut self, i: usize, val: Self::Element) {
        self.e1.replace(i, val.e1);
        self.e2.replace(i, val.e2);
        self.e3.replace(i, val.e3);
        self.ep.replace(i, val.ep);
        self.en.replace(i, val.en);
    }
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        self.e1.replace_unchecked(i, val.e1);
        self.e2.replace_unchecked(i, val.e2);
        self.e3.replace_unchecked(i, val.e3);
        self.ep.replace_unchecked(i, val.ep);
        self.en.replace_unchecked(i, val.en);
    }
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        Point {
            e1: self.e1.select(cond, other.e1),
            e2: self.e2.select(cond, other.e2),
            e3: self.e3.select(cond, other.e3),
            ep: self.ep.select(cond, other.ep),
            en: self.en.select(cond, other.en),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pair() {
        let p1 = Point::new([1.0, 2.0, 4.0]);
        let p2 = Point::new([0.0, 3.0, 0.0]);
        assert_eq!(
            p1,
            Point {
                e1: 1.0,
                e2: 2.0,
                e3: 4.0,
                ep: 10.0,
                en: 11.0
            }
        );
        assert_eq!(
            p2,
            Point {
                e1: 0.0,
                e2: 3.0,
                e3: 0.0,
                ep: 4.0,
                en: 5.0
            }
        );

        let p = p1.outer(p2);
        assert_eq!(
            p,
            Pair {
                e12: 3.0,
                e13: 0.0,
                e23: -12.0,
                e1p: 4.0,
                e1n: 5.0,
                e2p: -22.0,
                e2n: -23.0,
                e3p: 16.0,
                e3n: 20.0,
                epn: 6.0,
            }
        );
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn test_pair_fast() {
        use crate::F32;
        let p1 = Point::new([F32::new(1.0), F32::new(2.0), F32::new(4.0)]);
        let p2 = Point::new([F32::new(0.0), F32::new(3.0), F32::new(0.0)]);
        assert_eq!(
            p1,
            Point {
                e1: F32::new(1.0),
                e2: F32::new(2.0),
                e3: F32::new(4.0),
                ep: F32::new(10.0),
                en: F32::new(11.0)
            }
        );
        assert_eq!(
            p2,
            Point {
                e1: F32::new(0.0),
                e2: F32::new(3.0),
                e3: F32::new(0.0),
                ep: F32::new(4.0),
                en: F32::new(5.0)
            }
        );

        let p = p1.outer(p2);
        assert_eq!(
            p,
            Pair {
                e12: F32::new(3.0),
                e13: F32::new(0.0),
                e23: F32::new(-12.0),
                e1p: F32::new(4.0),
                e1n: F32::new(5.0),
                e2p: F32::new(-22.0),
                e2n: F32::new(-23.0),
                e3p: F32::new(16.0),
                e3n: F32::new(20.0),
                epn: F32::new(6.0),
            }
        );
    }

    #[test]
    fn test_distance() {
        let p1 = Point::new([1.0_f32, 2.0, 4.0]);
        let p2 = Point::new([0.0, 3.0, 0.0]);

        assert_eq!(p1.distance(p2), (1.0_f32 + 1.0 + 16.0).sqrt());
        assert_eq!(p1.from_origin(), Some(Vector::new(1.0_f32, 2.0, 4.0)));
        assert_eq!(p2.from_origin(), Some(Vector::new(0.0_f32, 3.0, 0.0)));
        assert_eq!(Point::<f32>::no().from_origin(), Some(Vector::new(0.0_f32, 0.0, 0.0)));
        assert_eq!(Point::<f32>::ni().from_origin(), None);
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn test_distance_fast() {
        use crate::F32;
        let p1 = Point::new([F32::new(1.0), F32::new(2.0), F32::new(4.0)]);
        let p2 = Point::new([F32::new(0.0), F32::new(3.0), F32::new(0.0)]);

        assert_eq!(p1.distance(p2), F32::new((1.0_f32 + 1.0 + 16.0).sqrt()));
        assert_eq!(p1.from_origin(), Some(Vector::new(F32::new(1.0), F32::new(2.0), F32::new(4.0))));
        assert_eq!(p2.from_origin(), Some(Vector::new(F32::new(0.0), F32::new(3.0), F32::new(0.0))));
        assert_eq!(Point::<F32>::no().from_origin(), Some(Vector::new(F32::new(0.0), F32::new(0.0), F32::new(0.0))));
        assert_eq!(Point::<F32>::ni().from_origin(), None);
    }
}
