use num_traits::zero;

use crate::{R410, Multivec, Field, Scalar, Inner};

#[derive(Copy, Clone)]
pub struct Trivector<T> {
    pub(crate) e123: T,
}

impl<T: Field + Copy> Inner for Trivector<T> {
    type Output = Scalar<T>;
}

impl<T: Field + Copy> Multivec for Trivector<T> {
    type Element = T;
    fn into_mv(self) -> R410<Self::Element> { R410{ e123: self.e123, ..zero() } }
    fn from_mv(v: R410<Self::Element>) -> Self { Self{ e123: v.e123 } }
}
