use simba::simd::SimdRealField as Field;

#[derive(Copy, Clone)]
pub struct Rotor<T: Field> {
    pub(crate) s: T,
    pub(crate) e12: T,
    pub(crate) e13: T,
    pub(crate) e23: T,
}
