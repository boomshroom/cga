use simba::simd::SimdRealField as Field;

use crate::Inner;

#[derive(Copy, Clone)]
pub struct Trivector<T: Field> {
    pub(crate) e123: T,
}

impl<T: Field> Inner for Trivector<T> {
    type Output = T;
    fn inner(self, rhs: Self) -> T {
        -self.e123 * rhs.e123
    }
}
