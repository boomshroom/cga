//! Conformal and Projective Geometric Algebra work nearly identically in alternate spaces.
//! This module defines singleton types that can be used as type parameters for various
//! objects to change the space they act within.

use core::cmp::PartialEq;

use num_traits::{one, zero};

use crate::{Field, R410};

pub trait Space: PartialEq + Copy {
    /// Hyperbolic and Spherical geometry appear to swap when taking the dual.
    type Dual: Space;
    fn split<T: Field + Copy>(e: T) -> IBasis<T>;
    fn join<T: Field + Copy>(ep: T, en: T) -> T;

    #[inline]
    fn infinity<T: Field + Copy>() -> R410<T> {
        let IBasis { ep, en } = Self::split(one());
        R410 { ep, en, ..zero() }
    }
}

pub struct IBasis<T> {
    pub ep: T,
    pub en: T,
}

/// Standard Euclidean space with no curvature. Infinity acts as a null basis.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Euclidean;
/// Hyperbolic space with negative curvature. Infinity acts as a positive basis.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Hyperbolic;
/// Spherical space with positive curvature. Infinity acts as a negative basis.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Spherical;

impl Space for Euclidean {
    type Dual = Euclidean;

    #[inline]
    fn split<T: Field + Copy>(e: T) -> IBasis<T> {
        IBasis { ep: e, en: e }
    }
    #[inline]
    fn join<T: Field + Copy>(ep: T, en: T) -> T {
        debug_assert_eq!(ep, en);
        ep
    }
}

impl Space for Hyperbolic {
    type Dual = Spherical;

    #[inline]
    fn split<T: Field + Copy>(ep: T) -> IBasis<T> {
        IBasis { ep, en: zero() }
    }
    #[inline]
    fn join<T: Field + Copy>(ep: T, _en: T) -> T {
        ep
    }
}

impl Space for Spherical {
    type Dual = Hyperbolic;

    #[inline]
    fn split<T: Field + Copy>(en: T) -> IBasis<T> {
        IBasis { en, ep: zero() }
    }
    #[inline]
    fn join<T: Field + Copy>(_ep: T, en: T) -> T {
        en
    }
}
