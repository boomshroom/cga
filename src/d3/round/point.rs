use simba::simd::SimdRealField as Field;

use num_traits::zero;

use super::super::dual::{DPlane, DSphere};
use super::super::flat::FPoint;
use super::super::free::Vector;
use super::super::r410::R410;
use super::Pair;
use crate::{Inner, Outer};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point<T: Field> {
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
    fn into_multivec(self) -> R410<T> {
        let Point { e1, e2, e3, ep, en } = self;
        R410 {
            e1,
            e2,
            e3,
            ep,
            en,
            ..zero()
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
        (self.inner(other) * n2).simd_sqrt()
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
}

impl<T: Field + Copy> Inner for Point<T> {
    type Output = T;
    /// The inner product of two points. Returns negative a half of the distance squared between them.
    #[inline]
    fn inner(self, rhs: Self) -> T {
        let R410 { s, .. } = self.into_multivec() | rhs.into_multivec();
        s
        //self.e1 * rhs.e1 + self.e2 * rhs.e2 + self.e3 * rhs.e3 + self.ep * rhs.ep - self.en * rhs.en
    }
}

impl<T: Field + Copy> Outer for Point<T> {
    type Output = Pair<T>;
    /// Join two points into a pair.
    #[inline]
    fn outer(self, rhs: Self) -> Pair<T> {
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
        } = self.into_multivec() ^ rhs.into_multivec();
        Pair {
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
        }
        /*
        Pair {
            e12: self.e1 * rhs.e2 - self.e2 * rhs.e1,
            e13: self.e1 * rhs.e3 - self.e3 * rhs.e1,
            e23: self.e2 * rhs.e3 - self.e3 * rhs.e2,
            e1p: self.e1 * rhs.ep - self.ep * rhs.e1,
            e1n: self.e1 * rhs.en - self.en * rhs.e1,
            e2p: self.e2 * rhs.ep - self.ep * rhs.e2,
            e2n: self.e2 * rhs.en - self.en * rhs.e2,
            e3p: self.e3 * rhs.ep - self.ep * rhs.e3,
            e3n: self.e3 * rhs.en - self.en * rhs.e3,
            epn: self.ep * rhs.en - self.en * rhs.ep,
        }
        */
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
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn test_distance_fast() {
        use crate::F32;
        let p1 = Point::new([F32::new(1.0), F32::new(2.0), F32::new(4.0)]);
        let p2 = Point::new([F32::new(0.0), F32::new(3.0), F32::new(0.0)]);

        assert_eq!(p1.distance(p2), F32::new((1.0_f32 + 1.0 + 16.0).sqrt()));
    }
}
