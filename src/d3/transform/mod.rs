mod motor;
mod rotor;
mod translate;

pub use motor::Motor;
pub use rotor::Rotor;
pub use translate::Translator;

pub trait Transform<T> {
	fn transform(self, obj: T) -> T;
}