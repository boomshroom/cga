use core::marker::PhantomData;

use num_traits::zero;
use simba::simd::SimdRealField as Field;

use super::super::flat::{FPoint, Line, Plane};
use super::super::transform::Transform;
use crate::{Euclidean, Inner, Multivec, Space, R410};

#[derive(Copy, Clone, Debug)]
pub struct DPlane<T, S = Euclidean> {
    pub(crate) e1: T,
    pub(crate) e2: T,
    pub(crate) e3: T,
    /// Corresponds to both ep and en
    pub(crate) ei: T,
    _pd: PhantomData<S>,
}

/*
    (ep + en) / e123pn = e123n + e123p = e123i
    ep / e123pn = e123n != e123i
*/

impl<T: Field + Copy, S: Space> DPlane<T, S> {
    /// Converts the dual plane to standard form.
    pub fn undual(self) -> Plane<T, S::Dual> {
        Plane::from_mv(self.into_mv().undual())
    }
}

impl<T: Field + Copy, S: Space> Multivec for DPlane<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Self { e1, e2, e3, ei, _pd } = self;
        R410 {
            e1,
            e2,
            e3,
            ep: S::split(ei).ep,
            en: S::split(ei).en,
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
            ei: S::join(ep, en),
            _pd: PhantomData,
        }
    }
}

impl<T: Field + Copy, S: Space> Inner<Line<T, S>> for DPlane<T, S> {
    type Output = FPoint<T, S>;
}
impl<T: Field + Copy, S: Space> Transform<Line<T, S>> for DPlane<T, S> {}
