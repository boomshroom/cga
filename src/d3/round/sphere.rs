use core::ops::{Add, AddAssign, Mul, MulAssign};

use simba::simd::SimdRealField as Field;
use simba::scalar::RealField;

use crate::Dual;
use super::super::dual::DSphere;

#[derive(Copy, Clone, Debug)]
pub struct Sphere<T: Field> {
	pub (crate) e123p: T,
	pub (crate) e123n: T,
	pub (crate) e12pn: T,
	pub (crate) e13pn: T,
	pub (crate) e23pn: T,
}

impl<T: Field> Dual for Sphere<T> {
	type Output = DSphere<T>;
	/// converts the sphere into its dual form of a point with radius.
	fn dual(self) -> DSphere<T> {
		DSphere {
			e1: self.e23pn,
			e2: self.e13pn,
			e3: -self.e12pn,
			ep: self.e123n,
			en: -self.e123p,
		}
	}
}