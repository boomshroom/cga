use core::fmt::{self, Display, Formatter, LowerExp, UpperExp};
use core::marker::PhantomData;

use simba::scalar::{RealField, SubsetOf, SupersetOf};
use simba::simd::SimdValue;

use num_traits::{one, zero};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use super::super::direction::DVector;
use super::super::dual::{DPlane, DSphere};
use super::super::flat::{FPoint, Line};
use super::super::free::Vector;

use super::{Pair, Circle, Sphere};
use crate::{Euclidean, Field, Inner, Multivec, Outer, Scalar, Space, R410};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point<T, S = Euclidean> {
    pub(crate) e1: T,
    pub(crate) e2: T,
    pub(crate) e3: T,
    pub(crate) ep: T,
    pub(crate) en: T,
    pub(crate) _pd: PhantomData<S>,
}

impl_traits!(Point{e1, e2, e3, ep, en, [_pd: PhantomData]});

impl<T: Field + Copy, S: Space> From<[T; 3]> for Point<T, S> {
    fn from(v: [T; 3]) -> Self {
        Self::new(v)
    }
}

impl<T: Field + Copy, S: Space> From<Vector<T>> for Point<T, S> {
    fn from(v: Vector<T>) -> Self {
        Self::new(v)
    }
}

/// The point at the origin
#[inline]
pub fn origin<T: Field + Copy>() -> R410<T> {
    let half = T::from_subset(&0.5);
    R410 {
        ep: -half,
        en: half,
        ..zero()
    }
}

impl<T: Field + Copy, S: Space> Point<T, S> {
    #[inline]
    pub fn new(v: impl Into<Vector<T>>) -> Self {
        v.into().into_point()
    }

    /// The point at the origin
    #[inline]
    pub(crate) fn no() -> Self {
        Point::from_mv(origin())
    }

    /// The point at infinity
    #[inline]
    pub(crate) fn ni() -> Self {
        Point::from_mv(S::infinity())
    }

    /// Convert this point into a flat point.
    #[inline]
    pub fn into_flat(self) -> FPoint<T, S> {
        // self ^ ni
        FPoint::from_mv(self.into_mv() ^ S::infinity())
    }

    /// Calculate the euclidean distance between two points
    #[inline]
    pub fn distance(self, other: Self) -> T {
        let n2 = T::from_subset(&-2.0);
        (self.normalize().dot(other.normalize()) * n2).simd_sqrt()
    }

    /// The scalar product of two points. Returns negative a half of the distance squared between them.
    #[inline]
    pub fn dot(self, other: Self) -> T {
        self.inner(other).0
    }

    #[inline]
    pub(crate) fn normalize(self) -> Self {
        let norm = self.norm();
        assert!(!norm.is_zero());
        Self::from_mv(self.into_mv() / norm)
    }

    #[inline]
    fn norm(self) -> T {
        self.dot(Self::ni())
    }

    /// Constructs the dual form of the plane halfway between the two points.
    pub fn midplane(self, other: Self) -> DPlane<T, S> {
        DPlane::from_mv(self.into_mv() - other.into_mv())
    }

    /// Constructs the dual form of the sphere centered at this point with the given radius.
    pub fn into_sphere(self, radius: T) -> DSphere<T> {
        let half = T::from_subset(&0.5);
        let r2 = radius.clone() * radius * half;
        DSphere::from_mv(self.into_mv() - S::infinity() * r2)
    }

    /// Constructs the line in the direction of `dir` that passes through self.
    #[inline]
    pub fn extend_along_vec(self, dir: DVector<T, S>) -> Line<T, S> {
        self.outer(dir)
    }
}

impl<T: Field + Copy, S: Space> Multivec for Point<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Point {
            e1,
            e2,
            e3,
            ep,
            en,
            _pd,
        } = self;
        R410 {
            e1,
            e2,
            e3,
            ep,
            en,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 {
            e1, e2, e3, ep, en, ..
        } = v;
        Self {
            e1,
            e2,
            e3,
            ep,
            en,
            _pd: PhantomData,
        }
    }
}

impl<T: RealField + Copy, S: Space> Point<T, S> {
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

impl<T: Field + Copy + Display, S: Space> Display for Point<T, S> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Point({})", self.into_mv())
    }
}

impl<T: Field + Copy + LowerExp, S: Space> LowerExp for Point<T, S> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Point({:e})", self.into_mv())
    }
}

impl<T: Field + Copy + UpperExp, S: Space> UpperExp for Point<T, S> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Point({:E})", self.into_mv())
    }
}

impl<T: Field + Copy, S: Space> Inner for Point<T, S> {
    type Output = Scalar<T>;
}

impl<T: Field + Copy, S: Space> Inner<Sphere<T, S>> for Point<T, S> {
    type Output = Circle<T, S>;
}

/// Join two points into a pair.
impl<T: Field + Copy, S: Space> Outer for Point<T, S> {
    type Output = Pair<T, S>;
}

impl<T: Field + Copy, S: Space> Outer<FPoint<T, S>> for Point<T, S>
where
    FPoint<T, S>: Multivec<Element = T>,
{
    type Output = Line<T, S>;
}

impl<T: Field + Copy, S: Space> Outer<DVector<T, S>> for Point<T, S> {
    type Output = Line<T, S>;
}

impl<T: AbsDiffEq, S: Space> AbsDiffEq for Point<T, S>
where
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.e1, &other.e1, epsilon)
            && T::abs_diff_eq(&self.e2, &other.e2, epsilon)
            && T::abs_diff_eq(&self.e3, &other.e3, epsilon)
            && T::abs_diff_eq(&self.ep, &other.ep, epsilon)
            && T::abs_diff_eq(&self.en, &other.en, epsilon)
    }
}

impl<T: RelativeEq, S: Space> RelativeEq for Point<T, S>
where
    T::Epsilon: Copy,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        T::relative_eq(&self.e1, &other.e1, epsilon, max_relative)
            && T::relative_eq(&self.e2, &other.e2, epsilon, max_relative)
            && T::relative_eq(&self.e3, &other.e3, epsilon, max_relative)
            && T::relative_eq(&self.ep, &other.ep, epsilon, max_relative)
            && T::relative_eq(&self.en, &other.en, epsilon, max_relative)
    }
}

impl<T: UlpsEq, S: Space> UlpsEq for Point<T, S>
where
    T::Epsilon: Copy,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.e1, &other.e1, epsilon, max_ulps)
            && T::ulps_eq(&self.e2, &other.e2, epsilon, max_ulps)
            && T::ulps_eq(&self.e3, &other.e3, epsilon, max_ulps)
            && T::ulps_eq(&self.ep, &other.ep, epsilon, max_ulps)
            && T::ulps_eq(&self.en, &other.en, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pair() {
        let p1 : Point<_> = Point::new([1.0, 2.0, 4.0]);
        let p2 : Point<_> = Point::new([0.0, 3.0, 0.0]);
        assert_eq!(
            p1,
            Point {
                e1: 1.0,
                e2: 2.0,
                e3: 4.0,
                ep: 10.0,
                en: 11.0,
                _pd: PhantomData,
            }
        );
        assert_eq!(
            p2,
            Point {
                e1: 0.0,
                e2: 3.0,
                e3: 0.0,
                ep: 4.0,
                en: 5.0,
                _pd: PhantomData,
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
                _pd: PhantomData,
            }
        );
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn test_pair_fast() {
        use crate::F32;
        let p1 : Point<_> = Point::new([F32::new(1.0), F32::new(2.0), F32::new(4.0)]);
        let p2 : Point<_> = Point::new([F32::new(0.0), F32::new(3.0), F32::new(0.0)]);
        assert_eq!(
            p1,
            Point {
                e1: F32::new(1.0),
                e2: F32::new(2.0),
                e3: F32::new(4.0),
                ep: F32::new(10.0),
                en: F32::new(11.0),
                _pd: PhantomData,
            }
        );
        assert_eq!(
            p2,
            Point {
                e1: F32::new(0.0),
                e2: F32::new(3.0),
                e3: F32::new(0.0),
                ep: F32::new(4.0),
                en: F32::new(5.0),
                _pd: PhantomData,
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
                _pd: PhantomData,
            }
        );
    }

    #[test]
    fn test_distance() {
        let v1 = Vector::new(1.0_f32, 2.0, 4.0);
        let v2 = Vector::new(0.0, 3.0, 0.0);
        let p1 : Point<_> = v1.into_point();
        let p2 : Point<_> = v2.into_point();

        assert_eq!(p1.distance(p2), (v1 - v2).norm());
        assert_eq!(p1.from_origin(), Some(v1));
        assert_eq!(p2.from_origin(), Some(v2));
        assert_eq!(
            Point::<f32>::no().from_origin(),
            Some(Vector::new(0.0_f32, 0.0, 0.0))
        );
        assert_eq!(Point::<f32>::ni().from_origin(), None);
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn test_distance_fast() {
        use crate::F32;
        let p1 : Point<_> = Point::new([F32::new(1.0), F32::new(2.0), F32::new(4.0)]);
        let p2 : Point<_> = Point::new([F32::new(0.0), F32::new(3.0), F32::new(0.0)]);

        assert_eq!(p1.distance(p2), F32::new((1.0_f32 + 1.0 + 16.0).sqrt()));
        assert_eq!(
            p1.from_origin(),
            Some(Vector::new(F32::new(1.0), F32::new(2.0), F32::new(4.0)))
        );
        assert_eq!(
            p2.from_origin(),
            Some(Vector::new(F32::new(0.0), F32::new(3.0), F32::new(0.0)))
        );
        assert_eq!(
            Point::<F32>::no().from_origin(),
            Some(Vector::new(F32::new(0.0), F32::new(0.0), F32::new(0.0)))
        );
        assert_eq!(Point::<F32>::ni().from_origin(), None);
    }

    #[test]
    fn test_extend() {
        let d = Vector::new(-1.0, 1.0, -4.0);
        let v1 = Vector::new(1.0, 2.0, 4.0);
        let p1 : Point<_> = Point::new(v1);
        let p2 : Point<_> = Point::new(v1 + d);

        let l1 = p1.outer(p2).extend();
        let l2 = p2.outer(p1).extend();
        let d = d.as_direction();
        assert_eq!(p1.extend_along_vec(d), l1);
        assert_eq!(p2.extend_along_vec(d), l1);
        assert_eq!(p2.extend_along_vec(-d), l2);
        assert_eq!(p1.extend_along_vec(-d), l2);
    }
}
