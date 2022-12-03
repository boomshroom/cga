use core::marker::PhantomData;
use core::ops::Mul;

use num_traits::zero;

use super::super::free::Vector;
use super::super::transform::Translator;
use super::Line;
use crate::{Euclidean, Field, Multivec, Space, R410};

#[derive(Copy, Clone, Debug)]
pub struct FPoint<T, S = Euclidean> {
    /// Corresponds to both e1p and e1n
    pub(crate) e1i: T,
    /// Corresponds to both e2p and e2n
    pub(crate) e2i: T,
    /// Corresponds to both e3p and e3n
    pub(crate) e3i: T,
    pub(crate) epn: T,
    pub(crate) _pd: PhantomData<S>,
}

/*

(a e1i + b e2i + c e3i + d ei^eo)(a e1i + b e2i + c e3i + d ei^eo)
epn * (en - ep)
epnn - epnp
-ep + en
*/

impl<T: Field + Copy, S: Space> Multivec for FPoint<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Self {
            e1i,
            e2i,
            e3i,
            epn,
            _pd,
        } = self;
        R410 {
            e1p: S::split(e1i).ep,
            e1n: S::split(e1i).en,
            e2p: S::split(e2i).ep,
            e2n: S::split(e2i).en,
            e3p: S::split(e3i).ep,
            e3n: S::split(e3i).en,
            epn,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 {
            e1p,
            e1n,
            e2p,
            e2n,
            e3p,
            e3n,
            epn,
            ..
        } = v;
        Self {
            e1i: S::join(e1p, e1n),
            e2i: S::join(e2p, e2n),
            e3i: S::join(e3p, e3n),
            epn,
            _pd: PhantomData,
        }
    }
}

impl<T: Field + Copy, S: Space> FPoint<T, S> {
    /// Constructs the line along `dir` that passes through self.
    #[inline]
    pub fn extend_along_vec(self, dir: Vector<T>) -> Line<T> {
        Line::from_mv(-self.into_mv() ^ dir.into_mv())
    }
}

impl<T: Field + Copy, S: Space> Mul for FPoint<T, S> {
    type Output = Translator<T>;
    /// Constructs the translator the brings an object twice the distance from `rhs` to `self`
    fn mul(self, rhs: Self) -> Translator<T> {
        Translator::from_mv(self.into_mv() * rhs.into_mv())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::d3::Point;
    use crate::Outer;
    #[test]
    fn test_extend() {
        let d = Vector::new(-1.0, 1.0, -4.0);
        let v1 = Vector::new(1.0, 2.0, 4.0);
        let p1 = Point::new(v1);
        let p2 = Point::new(v1 + d);

        let l1 = p1.outer(p2.into_flat());
        let l2 = p2.outer(p1.into_flat());
        assert_eq!(p1.into_flat().extend_along_vec(d), l1);
        assert_eq!(p2.into_flat().extend_along_vec(d), l1);
        assert_eq!(p2.into_flat().extend_along_vec(-d), l2);
        assert_eq!(p1.into_flat().extend_along_vec(-d), l2);
    }
}
