use simba::simd::SimdRealField as Field;

#[derive(Copy, Clone)]
pub struct Translator<T: Field> {
    pub(crate) s: T,
    pub(crate) e1i: T,
    pub(crate) e2i: T,
    pub(crate) e3i: T,
}
