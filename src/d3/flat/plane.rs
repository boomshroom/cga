use num_traits::zero;
use simba::simd::SimdRealField as Field;

use super::super::dual::DPlane;
use super::super::round::Sphere;
use crate::{R410, Multivec, Dual};

#[derive(Copy, Clone, Debug)]
pub struct Plane<T: Field> {
    /// Corresponds to both e123p and e123n
    pub(crate) e123i: T,
    pub(crate) e12pn: T,
    pub(crate) e13pn: T,
    pub(crate) e23pn: T,
}

impl<T: Field + Copy> Plane<T> {
    /// Interprets the plane as a sphere with infinite radius
    pub fn to_round(self) -> Sphere<T> {
        Sphere {
            e123p: self.e123i,
            e123n: self.e123i,
            e12pn: self.e12pn,
            e13pn: self.e13pn,
            e23pn: self.e23pn,
        }
    }
}

impl<T: Field + Copy> Multivec for Plane<T> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Plane{e123i, e12pn, e13pn, e23pn} = self;
        R410{ e123p: e123i, e123n: e123i, e12pn, e13pn, e23pn, ..zero() }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410{e123p, e12pn, e13pn, e23pn, ..} = v;
        Self{ e123i: e123p, e12pn, e13pn, e23pn }
    }
}

impl<T: Field + Copy> Dual for Plane<T> {
    type Output = DPlane<T>;
    /*
    fn dual(self) -> DPlane<T> {
        DPlane {
            e1: -self.e23pn,
            e2: self.e13pn,
            e3: -self.e12pn,
            ei: self.e123i,
        }
    }
    */
}
