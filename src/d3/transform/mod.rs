mod motor;
mod rotor;
mod translate;

pub use motor::Motor;
pub use rotor::Rotor;
pub use translate::Translator;

use crate::Multivec;

pub trait Transform<T: Multivec<Element = Self::Element>>: Multivec {
    #[inline]
    fn transform(self, obj: T) -> T {
        T::from_mv(self.into_mv() >> obj.into_mv())
    }
}
