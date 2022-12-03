use core::marker::PhantomData;

use num_traits::zero;
use simba::simd::SimdRealField as Field;

use super::super::dual::DPlane;
use super::super::round::Sphere;
use crate::{Dual, Euclidean, Multivec, Space, R410};

#[derive(Copy, Clone, Debug)]
pub struct Plane<T: Field, S = Euclidean> {
    /// Corresponds to both e123p and e123n
    pub(crate) e123i: T,
    pub(crate) e12pn: T,
    pub(crate) e13pn: T,
    pub(crate) e23pn: T,
    _pd: PhantomData<S>,
}

impl<T: Field + Copy, S: Space> Plane<T, S> {
    /// Interprets the plane as a sphere with infinite radius
    pub fn to_round(self) -> Sphere<T> {
        Sphere::from_mv(self.into_mv())
    }
}

impl<T: Field + Copy, S: Space> Multivec for Plane<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Plane {
            e123i,
            e12pn,
            e13pn,
            e23pn,
            _pd,
        } = self;
        R410 {
            e123p: S::split(e123i).ep,
            e123n: S::split(e123i).en,
            e12pn,
            e13pn,
            e23pn,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 {
            e123p,
            e123n,
            e12pn,
            e13pn,
            e23pn,
            ..
        } = v;
        Self {
            e123i: S::join(e123p, e123n),
            e12pn,
            e13pn,
            e23pn,
            _pd: PhantomData,
        }
    }
}

impl<T: Field + Copy, S: Space> Dual for Plane<T, S> {
    type Output = DPlane<T, S::Dual>;
}
