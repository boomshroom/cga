use simba::simd::SimdRealField as Field;

#[derive(Copy, Clone, Debug)]
pub struct Motor<T: Field> {
    pub(crate) s: T,
    pub(crate) e12: T,
    pub(crate) e13: T,
    pub(crate) e23: T,
    pub(crate) e1i: T,
    pub(crate) e2i: T,
    pub(crate) e3i: T,
    pub(crate) e123i: T,
}
