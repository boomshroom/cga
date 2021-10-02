use num_traits::{zero, one};
use simba::simd::SimdRealField as Field;

use super::{Join, Dual};

mod v1;
mod v2;
mod v3;

pub use v1::Vector;
pub use v2::Bivector;
pub use v3::Trivector;

pub struct Multivector<T: Field> {
	v0: T,
	v1: Vector<T>,
	v2: Bivector<T>,
	v3: Trivector<T>,
	v4: Quadvector<T>,
}

pub struct Quadvector<T: Field> {
	e1234: T,
}

pub type Pseudovector<T> = Trivector<T>;
pub type Pseudoscalar<T> = Quadvector<T>;

pub type Point<T> = Vector<T>;
pub type Pair<T> = Bivector<T>;
pub type Circle<T> = Trivector<T>;

impl<T: Field> Multivector<T> {
	/// The null-vector that represents a point at infinity.
	pub fn ni() -> Vector<T> {
		Vector { e3: one(), e4: one(), ..zero() }
	}

	/// The null-vector that represents a point at the origin.
	pub fn no() -> Vector<T> {
		let half = T::from_subset(&0.5);
		Vector { e4: half.clone(), e3: -half, ..zero() }
	}

	/// A point at the location (x, y)
	pub fn point(x: T, y: T) -> Vector<T> {
		let half = T::from_subset(&0.5);
		Self::no() + Self::ni() * half * (x * x + y * y) + Vector { e1: x, e2: y, ..zero() }
	}

	/// Forms the line connecting the two points that extends infinitely.
	pub fn line(p1: Vector<T>, p2: Vector<T>) -> Trivector<T> {
		p1.join(p2).join(Self::ni())
	}

	/// Forms the circle with the center `p` and radius `r`
	pub fn circle(p: Vector<T>, r: T) -> Trivector<T> {
		let half = T::from_subset(&0.5);
		(p - r * r * half * Self::ni()).dual()
	}
}