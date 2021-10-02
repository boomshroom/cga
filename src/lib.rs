#![cfg_attr(feature = "unstable", feature(core_intrinsics))]

#[cfg(feature = "unstable")]
mod fast_math;

#[cfg(feature = "unstable")]
pub use fast_math::F32;

pub mod d3;

/// A constant zero to save on calculations and space when using general multivectors.
#[derive(Debug, Copy, Clone, Default)]
struct Z;

impl From<Z> for f32 {
    fn from(Z: Z) -> Self {
        0.0
    }
}

/// Connects two objects into a higher dimensional object that passes through both.
/// The grade-increasing part of the geometric product.
/// Also known as the wedge product or outer product.
pub trait Join<RHS = Self> {
    type Output;
    fn join(self, rhs: RHS) -> Self::Output;
}

/// The meet operator to find the intersection between two objects
/// Also known as the regressive product
pub trait Meet<RHS = Self> {
    type Output;
    fn meet(self, rhs: RHS) -> Self::Output;
}

pub trait Dual {
    type Output;
    fn dual(self) -> Self::Output;
}

/// Denotes a type that can act as mirror to reflect other objects
pub trait Reflect<RHS> {
    /// Relect `object` across `self`
    fn reflect(self, object: RHS) -> RHS;
}

/// The grade-reducing part of the geometric product.
/// Also known as the dot product.
pub trait Inner<RHS = Self> {
    type Output;
    fn inner(self, rhs: RHS) -> Self::Output;
}

pub trait Outer<RHS = Self> {
    type Output;
    fn outer(self, rhs: RHS) -> Self::Output;
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
{
    type Output = <<V as Dual>::Output as Inner<W>>::Output;
    fn meet(self, rhs: W) -> Self::Output {
        self.dual().inner(rhs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
