#![cfg_attr(feature = "unstable", feature(core_intrinsics))]
#![cfg_attr(all(feature = "unstable", feature = "repr_simd"), feature(repr_simd))]
#[cfg(all(feature = "repr_simd", not(feature = "unstable")))]
compile_error!("#[repr(simd)] requires unstable.");

use num_traits::{zero, Zero};
use simba::simd::SimdRealField as Field;

#[cfg(feature = "unstable")]
mod fast_math;

#[cfg(feature = "unstable")]
pub use fast_math::F32;

mod r410;
use r410::R410;

#[macro_use]
mod traits;

pub mod d3;

mod spaces;

use spaces::IBasis;
pub use spaces::{Euclidean, Hyperbolic, Space, Spherical};

/// A constant zero to save on calculations and space when using general multivectors.
#[derive(Debug, Copy, Clone, Default)]
struct Z;

impl From<Z> for f32 {
    fn from(Z: Z) -> Self {
        0.0
    }
}

pub struct Scalar<T>(pub T);

mod sealed {
    use super::{Field, R410};
    use core::ops::{BitAnd, BitOr, Mul, Not};

    pub trait Multivec: Sized
    where
        R410<Self::Element>: Mul<Output = R410<Self::Element>>
            + BitOr<Output = R410<Self::Element>>
            + BitAnd<Output = R410<Self::Element>>
            + Not<Output = R410<Self::Element>>,
    {
        type Element: Field + Copy;
        fn into_mv(self) -> R410<Self::Element>;
        fn from_mv(v: R410<Self::Element>) -> Self;
    }
}
use sealed::Multivec;

impl<T: Field + Copy> Multivec for Scalar<T> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<Self::Element> {
        R410 {
            s: self.0,
            ..zero()
        }
    }
    #[inline]
    fn from_mv(v: R410<Self::Element>) -> Self {
        Self(v.s)
    }
}

/// Connects two objects into a higher dimensional object that passes through both.
/// The grade-increasing part of the geometric product.
/// Also known as the wedge product or outer product.
pub trait Join<RHS: Multivec<Element = Self::Element> = Self>: Multivec {
    type Output: Multivec<Element = Self::Element>;
    #[inline]
    fn join(self, rhs: RHS) -> Self::Output {
        Multivec::from_mv(self.into_mv() ^ rhs.into_mv())
    }
}

/// The meet operator to find the intersection between two objects
/// Also known as the regressive product
pub trait Meet<RHS: Multivec<Element = Self::Element> = Self>: Multivec {
    type Output: Multivec<Element = Self::Element>;
    #[inline]
    fn meet(self, rhs: RHS) -> Self::Output {
        Multivec::from_mv(self.into_mv() & rhs.into_mv())
    }
}

pub trait Dual: Multivec {
    type Output: Multivec<Element = Self::Element>;
    #[inline]
    fn dual(self) -> Self::Output {
        Multivec::from_mv(!self.into_mv())
    }
}

/// Denotes a type that can act as mirror to reflect other objects
pub trait Reflect<RHS> {
    /// Relect `object` across `self`
    fn reflect(self, object: RHS) -> RHS;
}

/// The grade-reducing part of the geometric product.
/// Also known as the dot product.
pub trait Inner<RHS: Multivec<Element = Self::Element> = Self>: Multivec {
    type Output: Multivec<Element = Self::Element>;
    #[inline]
    fn inner(self, rhs: RHS) -> Self::Output {
        Multivec::from_mv(self.into_mv() | rhs.into_mv())
    }
}

pub trait Outer<RHS: Multivec<Element = Self::Element> = Self>: Multivec {
    type Output: Multivec<Element = Self::Element>;
    #[inline]
    fn outer(self, rhs: RHS) -> Self::Output {
        Multivec::from_mv(self.into_mv() ^ rhs.into_mv())
    }
}

/// The anticommutative part of the geometric product.
/// I didn't make these definitions. Technically 1/2 the actual commutator product.
/// Can be calculated as (xy - yx)/2
pub trait Commutator<RHS = Self> {
    type Output;
    fn commutator(self, rhs: RHS) -> Self::Output;
}

/// The commutative part of the geometric product.
/// I didn't make these definitions. Technically 1/2 the actual anticommutator product.
/// Can be calculated as (xy + yx)/2
pub trait AntiCommutator<RHS = Self> {
    type Output;
    fn anticommutator(self, rhs: RHS) -> Self::Output;
}

impl<V, W> Meet<W> for V
where
    V: Dual,
    <V as Dual>::Output: Inner<W>,
    W: Multivec<Element = Self::Element>,
{
    type Output = <<V as Dual>::Output as Inner<W>>::Output;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
