use num_traits::zero;

use super::Point;
use super::super::dual::DSphere;
use crate::{R410, Multivec, Field, Dual};

#[derive(Copy, Clone, Debug)]
pub struct Sphere<T: Field> {
    pub(crate) e123p: T,
    pub(crate) e123n: T,
    pub(crate) e12pn: T,
    pub(crate) e13pn: T,
    pub(crate) e23pn: T,
}

impl<T: Field + Copy> Multivec for Sphere<T> {
    type Element = T;
    #[inline]
	fn into_mv(self) -> R410<T> {
		let Sphere{e123p, e123n, e12pn, e13pn, e23pn} = self;
		R410{ e123p, e123n, e12pn, e13pn, e23pn, ..zero() }
	}

    #[inline]
	fn from_mv(v: R410<T>) -> Self {
		let R410{e123p, e123n, e12pn, e13pn, e23pn, ..} = v;
		Self{ e123p, e123n, e12pn, e13pn, e23pn }
	}
}

impl<T: Field + Copy> Sphere<T> {
    #[inline]
	pub fn center(self) -> Point<T> {
		let ni = Point::ni().into_mv();
		let s = self.into_mv();
		let p = s * ni * s;
        Point::from_mv(p / (p | ni))
	}
}

/// converts the sphere into its dual form of a point with radius.
impl<T: Field + Copy> Dual for Sphere<T> {
    type Output = DSphere<T>;
    /*
    fn dual(self) -> DSphere<T> {
        DSphere {
            e1: self.e23pn,
            e2: self.e13pn,
            e3: -self.e12pn,
            ep: self.e123n,
            en: -self.e123p,
        }
    }
    */
}
