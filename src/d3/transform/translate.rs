use core::marker::PhantomData;
use core::ops::Mul;

use num_traits::zero;

use simba::simd::SimdRealField as Field;

use super::super::{Point, Sphere};
use super::super::dual::DSphere;
//use super::super::tangent::{TVector, TBivector};
use super::Transform;
use crate::{Euclidean, Multivec, Space, R410};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translator<T: Field, S = Euclidean> {
    pub(crate) e1i: T,
    pub(crate) e2i: T,
    pub(crate) e3i: T,
    pub(crate) s: T,
    _pd: PhantomData<S>,
}

impl<T: Field + Copy, S: Space> Multivec for Translator<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Translator {
            s,
            e1i,
            e2i,
            e3i,
            _pd,
        } = self;
        R410 {
            s,
            e1p: S::split(e1i).ep,
            e1n: S::split(e1i).en,
            e2p: S::split(e2i).ep,
            e2n: S::split(e2i).en,
            e3p: S::split(e3i).ep,
            e3n: S::split(e3i).en,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 {
            s,
            e1p,
            e1n,
            e2p,
            e2n,
            e3p,
            e3n,
            ..
        } = v;
        Self {
            s,
            e1i: S::join(e1p, e1n),
            e2i: S::join(e2p, e2n),
            e3i: S::join(e3p, e3n),
            _pd: PhantomData,
        }
    }
}

impl<T: Field + Copy, S: Space> Transform<Sphere<T>> for Translator<T, S> {}
impl<T: Field + Copy, S: Space> Transform<DSphere<T>> for Translator<T, S> {}
impl<T: Field + Copy, S: Space> Transform<Point<T>> for Translator<T, S> {}
//impl<T: Field + Copy, S: Space> Transform<TVector<T>> for Translator<T, S> {}
//impl<T: Field + Copy, S: Space> Transform<TBivector<T>> for Translator<T, S> {}

impl<T: Field + Copy, S: Space> Mul for Translator<T, S> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::from_mv(self.into_mv() * rhs.into_mv())
    }
}

impl<T: Field + Copy, S: Space> Mul<T> for Translator<T, S> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self::from_mv(self.into_mv() * rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::d3::free::Vector;
    #[test]
    fn test_from_vec() {
        use crate::Euclidean;
        use crate::d3::Transform;
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(3.0, 4.0, 5.0);
        let v3 = Vector::new(3.0, 4.0, 0.0);

        assert_eq!(
            v1.into_translator::<Euclidean>().transform(v2.into_point()),
            (v1 + v2).into_point::<Euclidean>()
        );
        assert_eq!(
            v2.into_translator::<Euclidean>().transform(v1.into_point()),
            (v1 + v2).into_point::<Euclidean>()
        );
        assert_eq!(
            v1.into_translator::<Euclidean>().transform(v3.into_point()),
            (v1 + v3).into_point::<Euclidean>()
        );
        assert_eq!(
            v3.into_translator::<Euclidean>().transform(v1.into_point()),
            (v1 + v3).into_point::<Euclidean>()
        );
        assert_eq!(
            v3.into_translator::<Euclidean>().transform(v2.into_point()),
            (v3 + v2).into_point::<Euclidean>()
        );
        assert_eq!(
            v2.into_translator::<Euclidean>().transform(v3.into_point()),
            (v3 + v2).into_point::<Euclidean>()
        );
    }

    #[test]
    fn test_from_points() {
        
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(3.0, 4.0, 5.0);
        let v3 = Vector::new(3.0, 4.0, 0.0);

        assert_eq!(
            ((v1 - v2) * 2.0).into_translator::<Euclidean>(),
            (v1.into_point::<Euclidean>().into_flat() * v2.into_point().into_flat())
        );
        assert_eq!(
            ((v1 - v3) * 2.0).into_translator::<Euclidean>(),
            (v1.into_point::<Euclidean>().into_flat() * v3.into_point().into_flat())
        );
        assert_eq!(
            ((v2 - v3) * 2.0).into_translator::<Euclidean>(),
            (v2.into_point::<Euclidean>().into_flat() * v3.into_point().into_flat())
        );
        assert_eq!(
            ((v2 - v1) * 2.0).into_translator::<Euclidean>(),
            (v2.into_point::<Euclidean>().into_flat() * v1.into_point().into_flat())
        );
        assert_eq!(
            ((v3 - v1) * 2.0).into_translator::<Euclidean>(),
            (v3.into_point::<Euclidean>().into_flat() * v1.into_point().into_flat())
        );
        assert_eq!(
            ((v3 - v2) * 2.0).into_translator::<Euclidean>(),
            (v3.into_point::<Euclidean>().into_flat() * v2.into_point().into_flat())
        );
    }
}
