use simba::simd::SimdRealField as Field;

use super::super::dual::DPlane;
use super::super::round::Sphere;
use crate::Dual;

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

impl<T: Field> Dual for Plane<T> {
    type Output = DPlane<T>;
    fn dual(self) -> DPlane<T> {
        DPlane {
            e1: -self.e23pn,
            e2: self.e13pn,
            e3: -self.e12pn,
            ei: self.e123i,
        }
    }
}
