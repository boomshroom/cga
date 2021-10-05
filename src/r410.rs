// Written by a generator written by enki.
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Sub};

use num_traits::{One, Zero};
use simba::simd::SimdRealField as Field;

/// The general multivector type. Used internally to implement the operations on the various specialized types.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct R410<T> {
    pub(crate) s: T,
    pub(crate) e1: T,
    pub(crate) e2: T,
    pub(crate) e3: T,
    pub(crate) ep: T,
    pub(crate) en: T,
    pub(crate) e12: T,
    pub(crate) e13: T,
    pub(crate) e1p: T,
    pub(crate) e1n: T,
    pub(crate) e23: T,
    pub(crate) e2p: T,
    pub(crate) e2n: T,
    pub(crate) e3p: T,
    pub(crate) e3n: T,
    pub(crate) epn: T,
    pub(crate) e123: T,
    pub(crate) e12p: T,
    pub(crate) e12n: T,
    pub(crate) e13p: T,
    pub(crate) e13n: T,
    pub(crate) e1pn: T,
    pub(crate) e23p: T,
    pub(crate) e23n: T,
    pub(crate) e2pn: T,
    pub(crate) e3pn: T,
    pub(crate) e123p: T,
    pub(crate) e123n: T,
    pub(crate) e12pn: T,
    pub(crate) e13pn: T,
    pub(crate) e23pn: T,
    pub(crate) e123pn: T,
}

impl R410<f64> {
    pub const fn zero() -> Self {
        Self {
            s: 0.0,
            e1: 0.0,
            e2: 0.0,
            e3: 0.0,
            ep: 0.0,
            en: 0.0,
            e12: 0.0,
            e13: 0.0,
            e1p: 0.0,
            e1n: 0.0,
            e23: 0.0,
            e2p: 0.0,
            e2n: 0.0,
            e3p: 0.0,
            e3n: 0.0,
            epn: 0.0,
            e123: 0.0,
            e12p: 0.0,
            e12n: 0.0,
            e13p: 0.0,
            e13n: 0.0,
            e1pn: 0.0,
            e23p: 0.0,
            e23n: 0.0,
            e2pn: 0.0,
            e3pn: 0.0,
            e123p: 0.0,
            e123n: 0.0,
            e12pn: 0.0,
            e13pn: 0.0,
            e23pn: 0.0,
            e123pn: 0.0,
        }
    }
}

impl<T: Zero + PartialEq> Zero for R410<T> {
    fn zero() -> Self {
        Self {
            s: T::zero(),
            e1: T::zero(),
            e2: T::zero(),
            e3: T::zero(),
            ep: T::zero(),
            en: T::zero(),
            e12: T::zero(),
            e13: T::zero(),
            e1p: T::zero(),
            e1n: T::zero(),
            e23: T::zero(),
            e2p: T::zero(),
            e2n: T::zero(),
            e3p: T::zero(),
            e3n: T::zero(),
            epn: T::zero(),
            e123: T::zero(),
            e12p: T::zero(),
            e12n: T::zero(),
            e13p: T::zero(),
            e13n: T::zero(),
            e1pn: T::zero(),
            e23p: T::zero(),
            e23n: T::zero(),
            e2pn: T::zero(),
            e3pn: T::zero(),
            e123p: T::zero(),
            e123n: T::zero(),
            e12pn: T::zero(),
            e13pn: T::zero(),
            e23pn: T::zero(),
            e123pn: T::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        *self == Zero::zero()
    }
}

impl<T: Field + Copy> One for R410<T> {
    fn one() -> Self {
        Self {
            s: T::one(),
            ..Self::zero()
        }
    }
}

impl<T: Field> R410<T> {
    pub fn e1() -> Self {
        Self {
            e1: T::one(),
            ..Self::zero()
        }
    }
    pub fn e2() -> Self {
        Self {
            e2: T::one(),
            ..Self::zero()
        }
    }
    pub fn e3() -> Self {
        Self {
            e3: T::one(),
            ..Self::zero()
        }
    }
    pub fn ep() -> Self {
        Self {
            ep: T::one(),
            ..Self::zero()
        }
    }
    pub fn en() -> Self {
        Self {
            en: T::one(),
            ..Self::zero()
        }
    }
    pub fn e12() -> Self {
        Self {
            e12: T::one(),
            ..Self::zero()
        }
    }
    pub fn e13() -> Self {
        Self {
            e13: T::one(),
            ..Self::zero()
        }
    }
    pub fn e1p() -> Self {
        Self {
            e1p: T::one(),
            ..Self::zero()
        }
    }
    pub fn e1n() -> Self {
        Self {
            e1n: T::one(),
            ..Self::zero()
        }
    }
    pub fn e23() -> Self {
        Self {
            e23: T::one(),
            ..Self::zero()
        }
    }
    pub fn e2p() -> Self {
        Self {
            e2p: T::one(),
            ..Self::zero()
        }
    }
    pub fn e2n() -> Self {
        Self {
            e2n: T::one(),
            ..Self::zero()
        }
    }
    pub fn e3p() -> Self {
        Self {
            e3p: T::one(),
            ..Self::zero()
        }
    }
    pub fn e3n() -> Self {
        Self {
            e3n: T::one(),
            ..Self::zero()
        }
    }
    pub fn epn() -> Self {
        Self {
            epn: T::one(),
            ..Self::zero()
        }
    }
    pub fn e123() -> Self {
        Self {
            e123: T::one(),
            ..Self::zero()
        }
    }
    pub fn e12p() -> Self {
        Self {
            e12p: T::one(),
            ..Self::zero()
        }
    }
    pub fn e12n() -> Self {
        Self {
            e12n: T::one(),
            ..Self::zero()
        }
    }
    pub fn e13p() -> Self {
        Self {
            e13p: T::one(),
            ..Self::zero()
        }
    }
    pub fn e13n() -> Self {
        Self {
            e13n: T::one(),
            ..Self::zero()
        }
    }
    pub fn e1pn() -> Self {
        Self {
            e1pn: T::one(),
            ..Self::zero()
        }
    }
    pub fn e23p() -> Self {
        Self {
            e23p: T::one(),
            ..Self::zero()
        }
    }
    pub fn e23n() -> Self {
        Self {
            e23n: T::one(),
            ..Self::zero()
        }
    }
    pub fn e2pn() -> Self {
        Self {
            e2pn: T::one(),
            ..Self::zero()
        }
    }
    pub fn e3pn() -> Self {
        Self {
            e3pn: T::one(),
            ..Self::zero()
        }
    }
    pub fn e123p() -> Self {
        Self {
            e123p: T::one(),
            ..Self::zero()
        }
    }
    pub fn e123n() -> Self {
        Self {
            e123n: T::one(),
            ..Self::zero()
        }
    }
    pub fn e12pn() -> Self {
        Self {
            e12pn: T::one(),
            ..Self::zero()
        }
    }
    pub fn e13pn() -> Self {
        Self {
            e13pn: T::one(),
            ..Self::zero()
        }
    }
    pub fn e23pn() -> Self {
        Self {
            e23pn: T::one(),
            ..Self::zero()
        }
    }
    pub fn e123pn() -> Self {
        Self {
            e123pn: T::one(),
            ..Self::zero()
        }
    }

    pub fn i() -> Self {
        Self::e123pn()
    }
}

impl<T: Zero + PartialEq> From<T> for R410<T> {
    #[inline]
    fn from(s: T) -> Self {
        Self { s, ..Self::zero() }
    }
}

/*
impl<T: Field+Copy+Display> fmt::Display for R410<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut zero = true;
        let ret = self.mvec.iter().enumerate().filter_map(|(i, &coeff)| {
            if (coeff.simd_gt(T::simd_default_epsilon()) | coeff.simd_lt(-T::simd_default_epsilon())).any() {
                non_zero = false;
                Some(format!("{}{}",
                        coeff,
                        if i > 0 { basis[i] } else { "" }
                    )
                )
            } else {
                None
            }
        }).collect::<Vec<String>>().join(" + ");
        if zero { write!(f,"{}", T::zero()) } else { write!(f, "{}", ret) }
    }
}
*/

// Dual
// Poincare duality operator.
impl<T: Field + Copy> R410<T> {
    #[inline]
    pub fn dual(self) -> Self {
        self / Self::i()
    }
}

impl<T: Field + Copy> Not for R410<T> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        self.dual()
    }
}

impl<T: Field + Copy> Neg for R410<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        self * -T::one()
    }
}

// Reverse
// Reverse the order of the basis blades.
impl<T: Neg<Output = T>> R410<T> {
    #[inline]
    pub fn reverse(self) -> Self {
        Self {
            e12: -self.e12,
            e13: -self.e13,
            e1p: -self.e1p,
            e1n: -self.e1n,
            e23: -self.e23,
            e2p: -self.e2p,
            e2n: -self.e2n,
            e3p: -self.e3p,
            e3n: -self.e3n,
            epn: -self.epn,
            e123: -self.e123,
            e12p: -self.e12p,
            e12n: -self.e12n,
            e13p: -self.e13p,
            e13n: -self.e13n,
            e1pn: -self.e1pn,
            e23p: -self.e23p,
            e23n: -self.e23n,
            e2pn: -self.e2pn,
            e3pn: -self.e3pn,
            ..self
        }
    }
}

// Conjugate
// Clifford Conjugation
impl<T: Neg<Output = T>> R410<T> {
    #[inline]
    pub fn conjugate(self) -> Self {
        Self {
            e1: -self.e1,
            e2: -self.e2,
            e3: -self.e3,
            ep: -self.ep,
            en: -self.en,
            e12: -self.e12,
            e13: -self.e13,
            e1p: -self.e1p,
            e1n: -self.e1n,
            e23: -self.e23,
            e2p: -self.e2p,
            e2n: -self.e2n,
            e3p: -self.e3p,
            e3n: -self.e3n,
            epn: -self.epn,
            e123pn: -self.e123pn,
            ..self
        }
    }
}

// Involute
// Main involution
impl<T: Neg<Output = T>> R410<T> {
    #[inline]
    pub fn involute(self) -> Self {
        Self {
            e1: -self.e1,
            e2: -self.e2,
            e3: -self.e3,
            ep: -self.ep,
            en: -self.en,
            e123: -self.e123,
            e12p: -self.e12p,
            e12n: -self.e12n,
            e13p: -self.e13p,
            e13n: -self.e13n,
            e1pn: -self.e1pn,
            e23p: -self.e23p,
            e23n: -self.e23n,
            e2pn: -self.e2pn,
            e3pn: -self.e3pn,
            e123pn: -self.e123pn,
            ..self
        }
    }
}

// Mul
// The geometric product.
impl<T: Field + Copy> Mul for R410<T> {
    type Output = R410<T>;

    #[inline(always)]
    fn mul(self, b: R410<T>) -> Self {
        let a = self;
        Self {
            s: b.s * a.s + b.e1 * a.e1 + b.e2 * a.e2 + b.e3 * a.e3 + b.ep * a.ep
                - b.en * a.en
                - b.e12 * a.e12
                - b.e13 * a.e13
                - b.e1p * a.e1p
                + b.e1n * a.e1n
                - b.e23 * a.e23
                - b.e2p * a.e2p
                + b.e2n * a.e2n
                - b.e3p * a.e3p
                + b.e3n * a.e3n
                + b.epn * a.epn
                - b.e123 * a.e123
                - b.e12p * a.e12p
                + b.e12n * a.e12n
                - b.e13p * a.e13p
                + b.e13n * a.e13n
                + b.e1pn * a.e1pn
                - b.e23p * a.e23p
                + b.e23n * a.e23n
                + b.e2pn * a.e2pn
                + b.e3pn * a.e3pn
                + b.e123p * a.e123p
                - b.e123n * a.e123n
                - b.e12pn * a.e12pn
                - b.e13pn * a.e13pn
                - b.e23pn * a.e23pn
                - b.e123pn * a.e123pn,
            e1: b.e1 * a.s + b.s * a.e1 - b.e12 * a.e2 - b.e13 * a.e3 - b.e1p * a.ep
                + b.e1n * a.en
                + b.e2 * a.e12
                + b.e3 * a.e13
                + b.ep * a.e1p
                - b.en * a.e1n
                - b.e123 * a.e23
                - b.e12p * a.e2p
                + b.e12n * a.e2n
                - b.e13p * a.e3p
                + b.e13n * a.e3n
                + b.e1pn * a.epn
                - b.e23 * a.e123
                - b.e2p * a.e12p
                + b.e2n * a.e12n
                - b.e3p * a.e13p
                + b.e3n * a.e13n
                + b.epn * a.e1pn
                + b.e123p * a.e23p
                - b.e123n * a.e23n
                - b.e12pn * a.e2pn
                - b.e13pn * a.e3pn
                - b.e23p * a.e123p
                + b.e23n * a.e123n
                + b.e2pn * a.e12pn
                + b.e3pn * a.e13pn
                - b.e123pn * a.e23pn
                - b.e23pn * a.e123pn,
            e2: b.e2 * a.s + b.e12 * a.e1 + b.s * a.e2 - b.e23 * a.e3 - b.e2p * a.ep + b.e2n * a.en
                - b.e1 * a.e12
                + b.e123 * a.e13
                + b.e12p * a.e1p
                - b.e12n * a.e1n
                + b.e3 * a.e23
                + b.ep * a.e2p
                - b.en * a.e2n
                - b.e23p * a.e3p
                + b.e23n * a.e3n
                + b.e2pn * a.epn
                + b.e13 * a.e123
                + b.e1p * a.e12p
                - b.e1n * a.e12n
                - b.e123p * a.e13p
                + b.e123n * a.e13n
                + b.e12pn * a.e1pn
                - b.e3p * a.e23p
                + b.e3n * a.e23n
                + b.epn * a.e2pn
                - b.e23pn * a.e3pn
                + b.e13p * a.e123p
                - b.e13n * a.e123n
                - b.e1pn * a.e12pn
                + b.e123pn * a.e13pn
                + b.e3pn * a.e23pn
                + b.e13pn * a.e123pn,
            e3: b.e3 * a.s + b.e13 * a.e1 + b.e23 * a.e2 + b.s * a.e3 - b.e3p * a.ep + b.e3n * a.en
                - b.e123 * a.e12
                - b.e1 * a.e13
                + b.e13p * a.e1p
                - b.e13n * a.e1n
                - b.e2 * a.e23
                + b.e23p * a.e2p
                - b.e23n * a.e2n
                + b.ep * a.e3p
                - b.en * a.e3n
                + b.e3pn * a.epn
                - b.e12 * a.e123
                + b.e123p * a.e12p
                - b.e123n * a.e12n
                + b.e1p * a.e13p
                - b.e1n * a.e13n
                + b.e13pn * a.e1pn
                + b.e2p * a.e23p
                - b.e2n * a.e23n
                + b.e23pn * a.e2pn
                + b.epn * a.e3pn
                - b.e12p * a.e123p
                + b.e12n * a.e123n
                - b.e123pn * a.e12pn
                - b.e1pn * a.e13pn
                - b.e2pn * a.e23pn
                - b.e12pn * a.e123pn,
            ep: b.ep * a.s + b.e1p * a.e1 + b.e2p * a.e2 + b.e3p * a.e3 + b.s * a.ep + b.epn * a.en
                - b.e12p * a.e12
                - b.e13p * a.e13
                - b.e1 * a.e1p
                - b.e1pn * a.e1n
                - b.e23p * a.e23
                - b.e2 * a.e2p
                - b.e2pn * a.e2n
                - b.e3 * a.e3p
                - b.e3pn * a.e3n
                - b.en * a.epn
                - b.e123p * a.e123
                - b.e12 * a.e12p
                - b.e12pn * a.e12n
                - b.e13 * a.e13p
                - b.e13pn * a.e13n
                - b.e1n * a.e1pn
                - b.e23 * a.e23p
                - b.e23pn * a.e23n
                - b.e2n * a.e2pn
                - b.e3n * a.e3pn
                + b.e123 * a.e123p
                + b.e123pn * a.e123n
                + b.e12n * a.e12pn
                + b.e13n * a.e13pn
                + b.e23n * a.e23pn
                + b.e123n * a.e123pn,
            en: b.en * a.s + b.e1n * a.e1 + b.e2n * a.e2 + b.e3n * a.e3 + b.epn * a.ep + b.s * a.en
                - b.e12n * a.e12
                - b.e13n * a.e13
                - b.e1pn * a.e1p
                - b.e1 * a.e1n
                - b.e23n * a.e23
                - b.e2pn * a.e2p
                - b.e2 * a.e2n
                - b.e3pn * a.e3p
                - b.e3 * a.e3n
                - b.ep * a.epn
                - b.e123n * a.e123
                - b.e12pn * a.e12p
                - b.e12 * a.e12n
                - b.e13pn * a.e13p
                - b.e13 * a.e13n
                - b.e1p * a.e1pn
                - b.e23pn * a.e23p
                - b.e23 * a.e23n
                - b.e2p * a.e2pn
                - b.e3p * a.e3pn
                + b.e123pn * a.e123p
                + b.e123 * a.e123n
                + b.e12p * a.e12pn
                + b.e13p * a.e13pn
                + b.e23p * a.e23pn
                + b.e123p * a.e123pn,
            e12: b.e12 * a.s + b.e2 * a.e1 - b.e1 * a.e2 + b.e123 * a.e3 + b.e12p * a.ep
                - b.e12n * a.en
                + b.s * a.e12
                - b.e23 * a.e13
                - b.e2p * a.e1p
                + b.e2n * a.e1n
                + b.e13 * a.e23
                + b.e1p * a.e2p
                - b.e1n * a.e2n
                - b.e123p * a.e3p
                + b.e123n * a.e3n
                + b.e12pn * a.epn
                + b.e3 * a.e123
                + b.ep * a.e12p
                - b.en * a.e12n
                - b.e23p * a.e13p
                + b.e23n * a.e13n
                + b.e2pn * a.e1pn
                + b.e13p * a.e23p
                - b.e13n * a.e23n
                - b.e1pn * a.e2pn
                + b.e123pn * a.e3pn
                - b.e3p * a.e123p
                + b.e3n * a.e123n
                + b.epn * a.e12pn
                - b.e23pn * a.e13pn
                + b.e13pn * a.e23pn
                + b.e3pn * a.e123pn,
            e13: b.e13 * a.s + b.e3 * a.e1 - b.e123 * a.e2 - b.e1 * a.e3 + b.e13p * a.ep
                - b.e13n * a.en
                + b.e23 * a.e12
                + b.s * a.e13
                - b.e3p * a.e1p
                + b.e3n * a.e1n
                - b.e12 * a.e23
                + b.e123p * a.e2p
                - b.e123n * a.e2n
                + b.e1p * a.e3p
                - b.e1n * a.e3n
                + b.e13pn * a.epn
                - b.e2 * a.e123
                + b.e23p * a.e12p
                - b.e23n * a.e12n
                + b.ep * a.e13p
                - b.en * a.e13n
                + b.e3pn * a.e1pn
                - b.e12p * a.e23p
                + b.e12n * a.e23n
                - b.e123pn * a.e2pn
                - b.e1pn * a.e3pn
                + b.e2p * a.e123p
                - b.e2n * a.e123n
                + b.e23pn * a.e12pn
                + b.epn * a.e13pn
                - b.e12pn * a.e23pn
                - b.e2pn * a.e123pn,
            e1p: b.e1p * a.s + b.ep * a.e1
                - b.e12p * a.e2
                - b.e13p * a.e3
                - b.e1 * a.ep
                - b.e1pn * a.en
                + b.e2p * a.e12
                + b.e3p * a.e13
                + b.s * a.e1p
                + b.epn * a.e1n
                - b.e123p * a.e23
                - b.e12 * a.e2p
                - b.e12pn * a.e2n
                - b.e13 * a.e3p
                - b.e13pn * a.e3n
                - b.e1n * a.epn
                - b.e23p * a.e123
                - b.e2 * a.e12p
                - b.e2pn * a.e12n
                - b.e3 * a.e13p
                - b.e3pn * a.e13n
                - b.en * a.e1pn
                + b.e123 * a.e23p
                + b.e123pn * a.e23n
                + b.e12n * a.e2pn
                + b.e13n * a.e3pn
                - b.e23 * a.e123p
                - b.e23pn * a.e123n
                - b.e2n * a.e12pn
                - b.e3n * a.e13pn
                + b.e123n * a.e23pn
                + b.e23n * a.e123pn,
            e1n: b.e1n * a.s + b.en * a.e1
                - b.e12n * a.e2
                - b.e13n * a.e3
                - b.e1pn * a.ep
                - b.e1 * a.en
                + b.e2n * a.e12
                + b.e3n * a.e13
                + b.epn * a.e1p
                + b.s * a.e1n
                - b.e123n * a.e23
                - b.e12pn * a.e2p
                - b.e12 * a.e2n
                - b.e13pn * a.e3p
                - b.e13 * a.e3n
                - b.e1p * a.epn
                - b.e23n * a.e123
                - b.e2pn * a.e12p
                - b.e2 * a.e12n
                - b.e3pn * a.e13p
                - b.e3 * a.e13n
                - b.ep * a.e1pn
                + b.e123pn * a.e23p
                + b.e123 * a.e23n
                + b.e12p * a.e2pn
                + b.e13p * a.e3pn
                - b.e23pn * a.e123p
                - b.e23 * a.e123n
                - b.e2p * a.e12pn
                - b.e3p * a.e13pn
                + b.e123p * a.e23pn
                + b.e23p * a.e123pn,
            e23: b.e23 * a.s + b.e123 * a.e1 + b.e3 * a.e2 - b.e2 * a.e3 + b.e23p * a.ep
                - b.e23n * a.en
                - b.e13 * a.e12
                + b.e12 * a.e13
                - b.e123p * a.e1p
                + b.e123n * a.e1n
                + b.s * a.e23
                - b.e3p * a.e2p
                + b.e3n * a.e2n
                + b.e2p * a.e3p
                - b.e2n * a.e3n
                + b.e23pn * a.epn
                + b.e1 * a.e123
                - b.e13p * a.e12p
                + b.e13n * a.e12n
                + b.e12p * a.e13p
                - b.e12n * a.e13n
                + b.e123pn * a.e1pn
                + b.ep * a.e23p
                - b.en * a.e23n
                + b.e3pn * a.e2pn
                - b.e2pn * a.e3pn
                - b.e1p * a.e123p
                + b.e1n * a.e123n
                - b.e13pn * a.e12pn
                + b.e12pn * a.e13pn
                + b.epn * a.e23pn
                + b.e1pn * a.e123pn,
            e2p: b.e2p * a.s + b.e12p * a.e1 + b.ep * a.e2
                - b.e23p * a.e3
                - b.e2 * a.ep
                - b.e2pn * a.en
                - b.e1p * a.e12
                + b.e123p * a.e13
                + b.e12 * a.e1p
                + b.e12pn * a.e1n
                + b.e3p * a.e23
                + b.s * a.e2p
                + b.epn * a.e2n
                - b.e23 * a.e3p
                - b.e23pn * a.e3n
                - b.e2n * a.epn
                + b.e13p * a.e123
                + b.e1 * a.e12p
                + b.e1pn * a.e12n
                - b.e123 * a.e13p
                - b.e123pn * a.e13n
                - b.e12n * a.e1pn
                - b.e3 * a.e23p
                - b.e3pn * a.e23n
                - b.en * a.e2pn
                + b.e23n * a.e3pn
                + b.e13 * a.e123p
                + b.e13pn * a.e123n
                + b.e1n * a.e12pn
                - b.e123n * a.e13pn
                - b.e3n * a.e23pn
                - b.e13n * a.e123pn,
            e2n: b.e2n * a.s + b.e12n * a.e1 + b.en * a.e2
                - b.e23n * a.e3
                - b.e2pn * a.ep
                - b.e2 * a.en
                - b.e1n * a.e12
                + b.e123n * a.e13
                + b.e12pn * a.e1p
                + b.e12 * a.e1n
                + b.e3n * a.e23
                + b.epn * a.e2p
                + b.s * a.e2n
                - b.e23pn * a.e3p
                - b.e23 * a.e3n
                - b.e2p * a.epn
                + b.e13n * a.e123
                + b.e1pn * a.e12p
                + b.e1 * a.e12n
                - b.e123pn * a.e13p
                - b.e123 * a.e13n
                - b.e12p * a.e1pn
                - b.e3pn * a.e23p
                - b.e3 * a.e23n
                - b.ep * a.e2pn
                + b.e23p * a.e3pn
                + b.e13pn * a.e123p
                + b.e13 * a.e123n
                + b.e1p * a.e12pn
                - b.e123p * a.e13pn
                - b.e3p * a.e23pn
                - b.e13p * a.e123pn,
            e3p: b.e3p * a.s + b.e13p * a.e1 + b.e23p * a.e2 + b.ep * a.e3
                - b.e3 * a.ep
                - b.e3pn * a.en
                - b.e123p * a.e12
                - b.e1p * a.e13
                + b.e13 * a.e1p
                + b.e13pn * a.e1n
                - b.e2p * a.e23
                + b.e23 * a.e2p
                + b.e23pn * a.e2n
                + b.s * a.e3p
                + b.epn * a.e3n
                - b.e3n * a.epn
                - b.e12p * a.e123
                + b.e123 * a.e12p
                + b.e123pn * a.e12n
                + b.e1 * a.e13p
                + b.e1pn * a.e13n
                - b.e13n * a.e1pn
                + b.e2 * a.e23p
                + b.e2pn * a.e23n
                - b.e23n * a.e2pn
                - b.en * a.e3pn
                - b.e12 * a.e123p
                - b.e12pn * a.e123n
                + b.e123n * a.e12pn
                + b.e1n * a.e13pn
                + b.e2n * a.e23pn
                + b.e12n * a.e123pn,
            e3n: b.e3n * a.s + b.e13n * a.e1 + b.e23n * a.e2 + b.en * a.e3
                - b.e3pn * a.ep
                - b.e3 * a.en
                - b.e123n * a.e12
                - b.e1n * a.e13
                + b.e13pn * a.e1p
                + b.e13 * a.e1n
                - b.e2n * a.e23
                + b.e23pn * a.e2p
                + b.e23 * a.e2n
                + b.epn * a.e3p
                + b.s * a.e3n
                - b.e3p * a.epn
                - b.e12n * a.e123
                + b.e123pn * a.e12p
                + b.e123 * a.e12n
                + b.e1pn * a.e13p
                + b.e1 * a.e13n
                - b.e13p * a.e1pn
                + b.e2pn * a.e23p
                + b.e2 * a.e23n
                - b.e23p * a.e2pn
                - b.ep * a.e3pn
                - b.e12pn * a.e123p
                - b.e12 * a.e123n
                + b.e123p * a.e12pn
                + b.e1p * a.e13pn
                + b.e2p * a.e23pn
                + b.e12p * a.e123pn,
            epn: b.epn * a.s + b.e1pn * a.e1 + b.e2pn * a.e2 + b.e3pn * a.e3 + b.en * a.ep
                - b.ep * a.en
                - b.e12pn * a.e12
                - b.e13pn * a.e13
                - b.e1n * a.e1p
                + b.e1p * a.e1n
                - b.e23pn * a.e23
                - b.e2n * a.e2p
                + b.e2p * a.e2n
                - b.e3n * a.e3p
                + b.e3p * a.e3n
                + b.s * a.epn
                - b.e123pn * a.e123
                - b.e12n * a.e12p
                + b.e12p * a.e12n
                - b.e13n * a.e13p
                + b.e13p * a.e13n
                + b.e1 * a.e1pn
                - b.e23n * a.e23p
                + b.e23p * a.e23n
                + b.e2 * a.e2pn
                + b.e3 * a.e3pn
                + b.e123n * a.e123p
                - b.e123p * a.e123n
                - b.e12 * a.e12pn
                - b.e13 * a.e13pn
                - b.e23 * a.e23pn
                - b.e123 * a.e123pn,
            e123: b.e123 * a.s + b.e23 * a.e1 - b.e13 * a.e2 + b.e12 * a.e3 - b.e123p * a.ep
                + b.e123n * a.en
                + b.e3 * a.e12
                - b.e2 * a.e13
                + b.e23p * a.e1p
                - b.e23n * a.e1n
                + b.e1 * a.e23
                - b.e13p * a.e2p
                + b.e13n * a.e2n
                + b.e12p * a.e3p
                - b.e12n * a.e3n
                + b.e123pn * a.epn
                + b.s * a.e123
                - b.e3p * a.e12p
                + b.e3n * a.e12n
                + b.e2p * a.e13p
                - b.e2n * a.e13n
                + b.e23pn * a.e1pn
                - b.e1p * a.e23p
                + b.e1n * a.e23n
                - b.e13pn * a.e2pn
                + b.e12pn * a.e3pn
                + b.ep * a.e123p
                - b.en * a.e123n
                + b.e3pn * a.e12pn
                - b.e2pn * a.e13pn
                + b.e1pn * a.e23pn
                + b.epn * a.e123pn,
            e12p: b.e12p * a.s + b.e2p * a.e1 - b.e1p * a.e2
                + b.e123p * a.e3
                + b.e12 * a.ep
                + b.e12pn * a.en
                + b.ep * a.e12
                - b.e23p * a.e13
                - b.e2 * a.e1p
                - b.e2pn * a.e1n
                + b.e13p * a.e23
                + b.e1 * a.e2p
                + b.e1pn * a.e2n
                - b.e123 * a.e3p
                - b.e123pn * a.e3n
                - b.e12n * a.epn
                + b.e3p * a.e123
                + b.s * a.e12p
                + b.epn * a.e12n
                - b.e23 * a.e13p
                - b.e23pn * a.e13n
                - b.e2n * a.e1pn
                + b.e13 * a.e23p
                + b.e13pn * a.e23n
                + b.e1n * a.e2pn
                - b.e123n * a.e3pn
                - b.e3 * a.e123p
                - b.e3pn * a.e123n
                - b.en * a.e12pn
                + b.e23n * a.e13pn
                - b.e13n * a.e23pn
                - b.e3n * a.e123pn,
            e12n: b.e12n * a.s + b.e2n * a.e1 - b.e1n * a.e2
                + b.e123n * a.e3
                + b.e12pn * a.ep
                + b.e12 * a.en
                + b.en * a.e12
                - b.e23n * a.e13
                - b.e2pn * a.e1p
                - b.e2 * a.e1n
                + b.e13n * a.e23
                + b.e1pn * a.e2p
                + b.e1 * a.e2n
                - b.e123pn * a.e3p
                - b.e123 * a.e3n
                - b.e12p * a.epn
                + b.e3n * a.e123
                + b.epn * a.e12p
                + b.s * a.e12n
                - b.e23pn * a.e13p
                - b.e23 * a.e13n
                - b.e2p * a.e1pn
                + b.e13pn * a.e23p
                + b.e13 * a.e23n
                + b.e1p * a.e2pn
                - b.e123p * a.e3pn
                - b.e3pn * a.e123p
                - b.e3 * a.e123n
                - b.ep * a.e12pn
                + b.e23p * a.e13pn
                - b.e13p * a.e23pn
                - b.e3p * a.e123pn,
            e13p: b.e13p * a.s + b.e3p * a.e1 - b.e123p * a.e2 - b.e1p * a.e3
                + b.e13 * a.ep
                + b.e13pn * a.en
                + b.e23p * a.e12
                + b.ep * a.e13
                - b.e3 * a.e1p
                - b.e3pn * a.e1n
                - b.e12p * a.e23
                + b.e123 * a.e2p
                + b.e123pn * a.e2n
                + b.e1 * a.e3p
                + b.e1pn * a.e3n
                - b.e13n * a.epn
                - b.e2p * a.e123
                + b.e23 * a.e12p
                + b.e23pn * a.e12n
                + b.s * a.e13p
                + b.epn * a.e13n
                - b.e3n * a.e1pn
                - b.e12 * a.e23p
                - b.e12pn * a.e23n
                + b.e123n * a.e2pn
                + b.e1n * a.e3pn
                + b.e2 * a.e123p
                + b.e2pn * a.e123n
                - b.e23n * a.e12pn
                - b.en * a.e13pn
                + b.e12n * a.e23pn
                + b.e2n * a.e123pn,
            e13n: b.e13n * a.s + b.e3n * a.e1 - b.e123n * a.e2 - b.e1n * a.e3
                + b.e13pn * a.ep
                + b.e13 * a.en
                + b.e23n * a.e12
                + b.en * a.e13
                - b.e3pn * a.e1p
                - b.e3 * a.e1n
                - b.e12n * a.e23
                + b.e123pn * a.e2p
                + b.e123 * a.e2n
                + b.e1pn * a.e3p
                + b.e1 * a.e3n
                - b.e13p * a.epn
                - b.e2n * a.e123
                + b.e23pn * a.e12p
                + b.e23 * a.e12n
                + b.epn * a.e13p
                + b.s * a.e13n
                - b.e3p * a.e1pn
                - b.e12pn * a.e23p
                - b.e12 * a.e23n
                + b.e123p * a.e2pn
                + b.e1p * a.e3pn
                + b.e2pn * a.e123p
                + b.e2 * a.e123n
                - b.e23p * a.e12pn
                - b.ep * a.e13pn
                + b.e12p * a.e23pn
                + b.e2p * a.e123pn,
            e1pn: b.e1pn * a.s + b.epn * a.e1 - b.e12pn * a.e2 - b.e13pn * a.e3 - b.e1n * a.ep
                + b.e1p * a.en
                + b.e2pn * a.e12
                + b.e3pn * a.e13
                + b.en * a.e1p
                - b.ep * a.e1n
                - b.e123pn * a.e23
                - b.e12n * a.e2p
                + b.e12p * a.e2n
                - b.e13n * a.e3p
                + b.e13p * a.e3n
                + b.e1 * a.epn
                - b.e23pn * a.e123
                - b.e2n * a.e12p
                + b.e2p * a.e12n
                - b.e3n * a.e13p
                + b.e3p * a.e13n
                + b.s * a.e1pn
                + b.e123n * a.e23p
                - b.e123p * a.e23n
                - b.e12 * a.e2pn
                - b.e13 * a.e3pn
                - b.e23n * a.e123p
                + b.e23p * a.e123n
                + b.e2 * a.e12pn
                + b.e3 * a.e13pn
                - b.e123 * a.e23pn
                - b.e23 * a.e123pn,
            e23p: b.e23p * a.s + b.e123p * a.e1 + b.e3p * a.e2 - b.e2p * a.e3
                + b.e23 * a.ep
                + b.e23pn * a.en
                - b.e13p * a.e12
                + b.e12p * a.e13
                - b.e123 * a.e1p
                - b.e123pn * a.e1n
                + b.ep * a.e23
                - b.e3 * a.e2p
                - b.e3pn * a.e2n
                + b.e2 * a.e3p
                + b.e2pn * a.e3n
                - b.e23n * a.epn
                + b.e1p * a.e123
                - b.e13 * a.e12p
                - b.e13pn * a.e12n
                + b.e12 * a.e13p
                + b.e12pn * a.e13n
                - b.e123n * a.e1pn
                + b.s * a.e23p
                + b.epn * a.e23n
                - b.e3n * a.e2pn
                + b.e2n * a.e3pn
                - b.e1 * a.e123p
                - b.e1pn * a.e123n
                + b.e13n * a.e12pn
                - b.e12n * a.e13pn
                - b.en * a.e23pn
                - b.e1n * a.e123pn,
            e23n: b.e23n * a.s + b.e123n * a.e1 + b.e3n * a.e2 - b.e2n * a.e3
                + b.e23pn * a.ep
                + b.e23 * a.en
                - b.e13n * a.e12
                + b.e12n * a.e13
                - b.e123pn * a.e1p
                - b.e123 * a.e1n
                + b.en * a.e23
                - b.e3pn * a.e2p
                - b.e3 * a.e2n
                + b.e2pn * a.e3p
                + b.e2 * a.e3n
                - b.e23p * a.epn
                + b.e1n * a.e123
                - b.e13pn * a.e12p
                - b.e13 * a.e12n
                + b.e12pn * a.e13p
                + b.e12 * a.e13n
                - b.e123p * a.e1pn
                + b.epn * a.e23p
                + b.s * a.e23n
                - b.e3p * a.e2pn
                + b.e2p * a.e3pn
                - b.e1pn * a.e123p
                - b.e1 * a.e123n
                + b.e13p * a.e12pn
                - b.e12p * a.e13pn
                - b.ep * a.e23pn
                - b.e1p * a.e123pn,
            e2pn: b.e2pn * a.s + b.e12pn * a.e1 + b.epn * a.e2 - b.e23pn * a.e3 - b.e2n * a.ep
                + b.e2p * a.en
                - b.e1pn * a.e12
                + b.e123pn * a.e13
                + b.e12n * a.e1p
                - b.e12p * a.e1n
                + b.e3pn * a.e23
                + b.en * a.e2p
                - b.ep * a.e2n
                - b.e23n * a.e3p
                + b.e23p * a.e3n
                + b.e2 * a.epn
                + b.e13pn * a.e123
                + b.e1n * a.e12p
                - b.e1p * a.e12n
                - b.e123n * a.e13p
                + b.e123p * a.e13n
                + b.e12 * a.e1pn
                - b.e3n * a.e23p
                + b.e3p * a.e23n
                + b.s * a.e2pn
                - b.e23 * a.e3pn
                + b.e13n * a.e123p
                - b.e13p * a.e123n
                - b.e1 * a.e12pn
                + b.e123 * a.e13pn
                + b.e3 * a.e23pn
                + b.e13 * a.e123pn,
            e3pn: b.e3pn * a.s + b.e13pn * a.e1 + b.e23pn * a.e2 + b.epn * a.e3 - b.e3n * a.ep
                + b.e3p * a.en
                - b.e123pn * a.e12
                - b.e1pn * a.e13
                + b.e13n * a.e1p
                - b.e13p * a.e1n
                - b.e2pn * a.e23
                + b.e23n * a.e2p
                - b.e23p * a.e2n
                + b.en * a.e3p
                - b.ep * a.e3n
                + b.e3 * a.epn
                - b.e12pn * a.e123
                + b.e123n * a.e12p
                - b.e123p * a.e12n
                + b.e1n * a.e13p
                - b.e1p * a.e13n
                + b.e13 * a.e1pn
                + b.e2n * a.e23p
                - b.e2p * a.e23n
                + b.e23 * a.e2pn
                + b.s * a.e3pn
                - b.e12n * a.e123p
                + b.e12p * a.e123n
                - b.e123 * a.e12pn
                - b.e1 * a.e13pn
                - b.e2 * a.e23pn
                - b.e12 * a.e123pn,
            e123p: b.e123p * a.s + b.e23p * a.e1 - b.e13p * a.e2 + b.e12p * a.e3
                - b.e123 * a.ep
                - b.e123pn * a.en
                + b.e3p * a.e12
                - b.e2p * a.e13
                + b.e23 * a.e1p
                + b.e23pn * a.e1n
                + b.e1p * a.e23
                - b.e13 * a.e2p
                - b.e13pn * a.e2n
                + b.e12 * a.e3p
                + b.e12pn * a.e3n
                - b.e123n * a.epn
                + b.ep * a.e123
                - b.e3 * a.e12p
                - b.e3pn * a.e12n
                + b.e2 * a.e13p
                + b.e2pn * a.e13n
                - b.e23n * a.e1pn
                - b.e1 * a.e23p
                - b.e1pn * a.e23n
                + b.e13n * a.e2pn
                - b.e12n * a.e3pn
                + b.s * a.e123p
                + b.epn * a.e123n
                - b.e3n * a.e12pn
                + b.e2n * a.e13pn
                - b.e1n * a.e23pn
                - b.en * a.e123pn,
            e123n: b.e123n * a.s + b.e23n * a.e1 - b.e13n * a.e2 + b.e12n * a.e3
                - b.e123pn * a.ep
                - b.e123 * a.en
                + b.e3n * a.e12
                - b.e2n * a.e13
                + b.e23pn * a.e1p
                + b.e23 * a.e1n
                + b.e1n * a.e23
                - b.e13pn * a.e2p
                - b.e13 * a.e2n
                + b.e12pn * a.e3p
                + b.e12 * a.e3n
                - b.e123p * a.epn
                + b.en * a.e123
                - b.e3pn * a.e12p
                - b.e3 * a.e12n
                + b.e2pn * a.e13p
                + b.e2 * a.e13n
                - b.e23p * a.e1pn
                - b.e1pn * a.e23p
                - b.e1 * a.e23n
                + b.e13p * a.e2pn
                - b.e12p * a.e3pn
                + b.epn * a.e123p
                + b.s * a.e123n
                - b.e3p * a.e12pn
                + b.e2p * a.e13pn
                - b.e1p * a.e23pn
                - b.ep * a.e123pn,
            e12pn: b.e12pn * a.s + b.e2pn * a.e1 - b.e1pn * a.e2 + b.e123pn * a.e3 + b.e12n * a.ep
                - b.e12p * a.en
                + b.epn * a.e12
                - b.e23pn * a.e13
                - b.e2n * a.e1p
                + b.e2p * a.e1n
                + b.e13pn * a.e23
                + b.e1n * a.e2p
                - b.e1p * a.e2n
                - b.e123n * a.e3p
                + b.e123p * a.e3n
                + b.e12 * a.epn
                + b.e3pn * a.e123
                + b.en * a.e12p
                - b.ep * a.e12n
                - b.e23n * a.e13p
                + b.e23p * a.e13n
                + b.e2 * a.e1pn
                + b.e13n * a.e23p
                - b.e13p * a.e23n
                - b.e1 * a.e2pn
                + b.e123 * a.e3pn
                - b.e3n * a.e123p
                + b.e3p * a.e123n
                + b.s * a.e12pn
                - b.e23 * a.e13pn
                + b.e13 * a.e23pn
                + b.e3 * a.e123pn,
            e13pn: b.e13pn * a.s + b.e3pn * a.e1 - b.e123pn * a.e2 - b.e1pn * a.e3 + b.e13n * a.ep
                - b.e13p * a.en
                + b.e23pn * a.e12
                + b.epn * a.e13
                - b.e3n * a.e1p
                + b.e3p * a.e1n
                - b.e12pn * a.e23
                + b.e123n * a.e2p
                - b.e123p * a.e2n
                + b.e1n * a.e3p
                - b.e1p * a.e3n
                + b.e13 * a.epn
                - b.e2pn * a.e123
                + b.e23n * a.e12p
                - b.e23p * a.e12n
                + b.en * a.e13p
                - b.ep * a.e13n
                + b.e3 * a.e1pn
                - b.e12n * a.e23p
                + b.e12p * a.e23n
                - b.e123 * a.e2pn
                - b.e1 * a.e3pn
                + b.e2n * a.e123p
                - b.e2p * a.e123n
                + b.e23 * a.e12pn
                + b.s * a.e13pn
                - b.e12 * a.e23pn
                - b.e2 * a.e123pn,
            e23pn: b.e23pn * a.s + b.e123pn * a.e1 + b.e3pn * a.e2 - b.e2pn * a.e3 + b.e23n * a.ep
                - b.e23p * a.en
                - b.e13pn * a.e12
                + b.e12pn * a.e13
                - b.e123n * a.e1p
                + b.e123p * a.e1n
                + b.epn * a.e23
                - b.e3n * a.e2p
                + b.e3p * a.e2n
                + b.e2n * a.e3p
                - b.e2p * a.e3n
                + b.e23 * a.epn
                + b.e1pn * a.e123
                - b.e13n * a.e12p
                + b.e13p * a.e12n
                + b.e12n * a.e13p
                - b.e12p * a.e13n
                + b.e123 * a.e1pn
                + b.en * a.e23p
                - b.ep * a.e23n
                + b.e3 * a.e2pn
                - b.e2 * a.e3pn
                - b.e1n * a.e123p
                + b.e1p * a.e123n
                - b.e13 * a.e12pn
                + b.e12 * a.e13pn
                + b.s * a.e23pn
                + b.e1 * a.e123pn,
            e123pn: b.e123pn * a.s + b.e23pn * a.e1 - b.e13pn * a.e2 + b.e12pn * a.e3
                - b.e123n * a.ep
                + b.e123p * a.en
                + b.e3pn * a.e12
                - b.e2pn * a.e13
                + b.e23n * a.e1p
                - b.e23p * a.e1n
                + b.e1pn * a.e23
                - b.e13n * a.e2p
                + b.e13p * a.e2n
                + b.e12n * a.e3p
                - b.e12p * a.e3n
                + b.e123 * a.epn
                + b.epn * a.e123
                - b.e3n * a.e12p
                + b.e3p * a.e12n
                + b.e2n * a.e13p
                - b.e2p * a.e13n
                + b.e23 * a.e1pn
                - b.e1n * a.e23p
                + b.e1p * a.e23n
                - b.e13 * a.e2pn
                + b.e12 * a.e3pn
                + b.en * a.e123p
                - b.ep * a.e123n
                + b.e3 * a.e12pn
                - b.e2 * a.e13pn
                + b.e1 * a.e23pn
                + b.s * a.e123pn,
        }
    }
}

impl<T: Field + Copy> Div for R410<T> {
    type Output = Self;
    #[inline]
    fn div(self, b: Self) -> Self {
        self * b.recip()
    }
}

// Wedge
// The outer product. (MEET)
impl<T: Field + Copy> BitXor for R410<T> {
    type Output = Self;

    #[inline]
    fn bitxor(self, b: Self) -> Self {
        let a = self;
        Self {
            s: b.s * a.s,
            e1: b.e1 * a.s + b.s * a.e1,
            e2: b.e2 * a.s + b.s * a.e2,
            e3: b.e3 * a.s + b.s * a.e3,
            ep: b.ep * a.s + b.s * a.ep,
            en: b.en * a.s + b.s * a.en,
            e12: b.e12 * a.s + b.e2 * a.e1 - b.e1 * a.e2 + b.s * a.e12,
            e13: b.e13 * a.s + b.e3 * a.e1 - b.e1 * a.e3 + b.s * a.e13,
            e1p: b.e1p * a.s + b.ep * a.e1 - b.e1 * a.ep + b.s * a.e1p,
            e1n: b.e1n * a.s + b.en * a.e1 - b.e1 * a.en + b.s * a.e1n,
            e23: b.e23 * a.s + b.e3 * a.e2 - b.e2 * a.e3 + b.s * a.e23,
            e2p: b.e2p * a.s + b.ep * a.e2 - b.e2 * a.ep + b.s * a.e2p,
            e2n: b.e2n * a.s + b.en * a.e2 - b.e2 * a.en + b.s * a.e2n,
            e3p: b.e3p * a.s + b.ep * a.e3 - b.e3 * a.ep + b.s * a.e3p,
            e3n: b.e3n * a.s + b.en * a.e3 - b.e3 * a.en + b.s * a.e3n,
            epn: b.epn * a.s + b.en * a.ep - b.ep * a.en + b.s * a.epn,
            e123: b.e123 * a.s + b.e23 * a.e1 - b.e13 * a.e2 + b.e12 * a.e3 + b.e3 * a.e12
                - b.e2 * a.e13
                + b.e1 * a.e23
                + b.s * a.e123,
            e12p: b.e12p * a.s + b.e2p * a.e1 - b.e1p * a.e2 + b.e12 * a.ep + b.ep * a.e12
                - b.e2 * a.e1p
                + b.e1 * a.e2p
                + b.s * a.e12p,
            e12n: b.e12n * a.s + b.e2n * a.e1 - b.e1n * a.e2 + b.e12 * a.en + b.en * a.e12
                - b.e2 * a.e1n
                + b.e1 * a.e2n
                + b.s * a.e12n,
            e13p: b.e13p * a.s + b.e3p * a.e1 - b.e1p * a.e3 + b.e13 * a.ep + b.ep * a.e13
                - b.e3 * a.e1p
                + b.e1 * a.e3p
                + b.s * a.e13p,
            e13n: b.e13n * a.s + b.e3n * a.e1 - b.e1n * a.e3 + b.e13 * a.en + b.en * a.e13
                - b.e3 * a.e1n
                + b.e1 * a.e3n
                + b.s * a.e13n,
            e1pn: b.e1pn * a.s + b.epn * a.e1 - b.e1n * a.ep + b.e1p * a.en + b.en * a.e1p
                - b.ep * a.e1n
                + b.e1 * a.epn
                + b.s * a.e1pn,
            e23p: b.e23p * a.s + b.e3p * a.e2 - b.e2p * a.e3 + b.e23 * a.ep + b.ep * a.e23
                - b.e3 * a.e2p
                + b.e2 * a.e3p
                + b.s * a.e23p,
            e23n: b.e23n * a.s + b.e3n * a.e2 - b.e2n * a.e3 + b.e23 * a.en + b.en * a.e23
                - b.e3 * a.e2n
                + b.e2 * a.e3n
                + b.s * a.e23n,
            e2pn: b.e2pn * a.s + b.epn * a.e2 - b.e2n * a.ep + b.e2p * a.en + b.en * a.e2p
                - b.ep * a.e2n
                + b.e2 * a.epn
                + b.s * a.e2pn,
            e3pn: b.e3pn * a.s + b.epn * a.e3 - b.e3n * a.ep + b.e3p * a.en + b.en * a.e3p
                - b.ep * a.e3n
                + b.e3 * a.epn
                + b.s * a.e3pn,
            e123p: b.e123p * a.s + b.e23p * a.e1 - b.e13p * a.e2 + b.e12p * a.e3 - b.e123 * a.ep
                + b.e3p * a.e12
                - b.e2p * a.e13
                + b.e23 * a.e1p
                + b.e1p * a.e23
                - b.e13 * a.e2p
                + b.e12 * a.e3p
                + b.ep * a.e123
                - b.e3 * a.e12p
                + b.e2 * a.e13p
                - b.e1 * a.e23p
                + b.s * a.e123p,
            e123n: b.e123n * a.s + b.e23n * a.e1 - b.e13n * a.e2 + b.e12n * a.e3 - b.e123 * a.en
                + b.e3n * a.e12
                - b.e2n * a.e13
                + b.e23 * a.e1n
                + b.e1n * a.e23
                - b.e13 * a.e2n
                + b.e12 * a.e3n
                + b.en * a.e123
                - b.e3 * a.e12n
                + b.e2 * a.e13n
                - b.e1 * a.e23n
                + b.s * a.e123n,
            e12pn: b.e12pn * a.s + b.e2pn * a.e1 - b.e1pn * a.e2 + b.e12n * a.ep - b.e12p * a.en
                + b.epn * a.e12
                - b.e2n * a.e1p
                + b.e2p * a.e1n
                + b.e1n * a.e2p
                - b.e1p * a.e2n
                + b.e12 * a.epn
                + b.en * a.e12p
                - b.ep * a.e12n
                + b.e2 * a.e1pn
                - b.e1 * a.e2pn
                + b.s * a.e12pn,
            e13pn: b.e13pn * a.s + b.e3pn * a.e1 - b.e1pn * a.e3 + b.e13n * a.ep - b.e13p * a.en
                + b.epn * a.e13
                - b.e3n * a.e1p
                + b.e3p * a.e1n
                + b.e1n * a.e3p
                - b.e1p * a.e3n
                + b.e13 * a.epn
                + b.en * a.e13p
                - b.ep * a.e13n
                + b.e3 * a.e1pn
                - b.e1 * a.e3pn
                + b.s * a.e13pn,
            e23pn: b.e23pn * a.s + b.e3pn * a.e2 - b.e2pn * a.e3 + b.e23n * a.ep - b.e23p * a.en
                + b.epn * a.e23
                - b.e3n * a.e2p
                + b.e3p * a.e2n
                + b.e2n * a.e3p
                - b.e2p * a.e3n
                + b.e23 * a.epn
                + b.en * a.e23p
                - b.ep * a.e23n
                + b.e3 * a.e2pn
                - b.e2 * a.e3pn
                + b.s * a.e23pn,
            e123pn: b.e123pn * a.s + b.e23pn * a.e1 - b.e13pn * a.e2 + b.e12pn * a.e3
                - b.e123n * a.ep
                + b.e123p * a.en
                + b.e3pn * a.e12
                - b.e2pn * a.e13
                + b.e23n * a.e1p
                - b.e23p * a.e1n
                + b.e1pn * a.e23
                - b.e13n * a.e2p
                + b.e13p * a.e2n
                + b.e12n * a.e3p
                - b.e12p * a.e3n
                + b.e123 * a.epn
                + b.epn * a.e123
                - b.e3n * a.e12p
                + b.e3p * a.e12n
                + b.e2n * a.e13p
                - b.e2p * a.e13n
                + b.e23 * a.e1pn
                - b.e1n * a.e23p
                + b.e1p * a.e23n
                - b.e13 * a.e2pn
                + b.e12 * a.e3pn
                + b.en * a.e123p
                - b.ep * a.e123n
                + b.e3 * a.e12pn
                - b.e2 * a.e13pn
                + b.e1 * a.e23pn
                + b.s * a.e123pn,
        }
    }
}

// Vee
// The regressive product. (JOIN)
impl<T: Field + Copy> BitAnd for R410<T> {
    type Output = R410<T>;

    #[inline]
    fn bitand(self, b: R410<T>) -> Self {
        self.dual() | b
    }
}

// Dot
// The inner product.
impl<T: Field + Copy> BitOr for R410<T> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, b: Self) -> Self {
        let a = self;
        Self {
            s: b.s * a.s + b.e1 * a.e1 + b.e2 * a.e2 + b.e3 * a.e3 + b.ep * a.ep
                - b.en * a.en
                - b.e12 * a.e12
                - b.e13 * a.e13
                - b.e1p * a.e1p
                + b.e1n * a.e1n
                - b.e23 * a.e23
                - b.e2p * a.e2p
                + b.e2n * a.e2n
                - b.e3p * a.e3p
                + b.e3n * a.e3n
                + b.epn * a.epn
                - b.e123 * a.e123
                - b.e12p * a.e12p
                + b.e12n * a.e12n
                - b.e13p * a.e13p
                + b.e13n * a.e13n
                + b.e1pn * a.e1pn
                - b.e23p * a.e23p
                + b.e23n * a.e23n
                + b.e2pn * a.e2pn
                + b.e3pn * a.e3pn
                + b.e123p * a.e123p
                - b.e123n * a.e123n
                - b.e12pn * a.e12pn
                - b.e13pn * a.e13pn
                - b.e23pn * a.e23pn
                - b.e123pn * a.e123pn,
            e1: b.e1 * a.s + b.s * a.e1 - b.e12 * a.e2 - b.e13 * a.e3 - b.e1p * a.ep
                + b.e1n * a.en
                + b.e2 * a.e12
                + b.e3 * a.e13
                + b.ep * a.e1p
                - b.en * a.e1n
                - b.e123 * a.e23
                - b.e12p * a.e2p
                + b.e12n * a.e2n
                - b.e13p * a.e3p
                + b.e13n * a.e3n
                + b.e1pn * a.epn
                - b.e23 * a.e123
                - b.e2p * a.e12p
                + b.e2n * a.e12n
                - b.e3p * a.e13p
                + b.e3n * a.e13n
                + b.epn * a.e1pn
                + b.e123p * a.e23p
                - b.e123n * a.e23n
                - b.e12pn * a.e2pn
                - b.e13pn * a.e3pn
                - b.e23p * a.e123p
                + b.e23n * a.e123n
                + b.e2pn * a.e12pn
                + b.e3pn * a.e13pn
                - b.e123pn * a.e23pn
                - b.e23pn * a.e123pn,
            e2: b.e2 * a.s + b.e12 * a.e1 + b.s * a.e2 - b.e23 * a.e3 - b.e2p * a.ep + b.e2n * a.en
                - b.e1 * a.e12
                + b.e123 * a.e13
                + b.e12p * a.e1p
                - b.e12n * a.e1n
                + b.e3 * a.e23
                + b.ep * a.e2p
                - b.en * a.e2n
                - b.e23p * a.e3p
                + b.e23n * a.e3n
                + b.e2pn * a.epn
                + b.e13 * a.e123
                + b.e1p * a.e12p
                - b.e1n * a.e12n
                - b.e123p * a.e13p
                + b.e123n * a.e13n
                + b.e12pn * a.e1pn
                - b.e3p * a.e23p
                + b.e3n * a.e23n
                + b.epn * a.e2pn
                - b.e23pn * a.e3pn
                + b.e13p * a.e123p
                - b.e13n * a.e123n
                - b.e1pn * a.e12pn
                + b.e123pn * a.e13pn
                + b.e3pn * a.e23pn
                + b.e13pn * a.e123pn,
            e3: b.e3 * a.s + b.e13 * a.e1 + b.e23 * a.e2 + b.s * a.e3 - b.e3p * a.ep + b.e3n * a.en
                - b.e123 * a.e12
                - b.e1 * a.e13
                + b.e13p * a.e1p
                - b.e13n * a.e1n
                - b.e2 * a.e23
                + b.e23p * a.e2p
                - b.e23n * a.e2n
                + b.ep * a.e3p
                - b.en * a.e3n
                + b.e3pn * a.epn
                - b.e12 * a.e123
                + b.e123p * a.e12p
                - b.e123n * a.e12n
                + b.e1p * a.e13p
                - b.e1n * a.e13n
                + b.e13pn * a.e1pn
                + b.e2p * a.e23p
                - b.e2n * a.e23n
                + b.e23pn * a.e2pn
                + b.epn * a.e3pn
                - b.e12p * a.e123p
                + b.e12n * a.e123n
                - b.e123pn * a.e12pn
                - b.e1pn * a.e13pn
                - b.e2pn * a.e23pn
                - b.e12pn * a.e123pn,
            ep: b.ep * a.s + b.e1p * a.e1 + b.e2p * a.e2 + b.e3p * a.e3 + b.s * a.ep + b.epn * a.en
                - b.e12p * a.e12
                - b.e13p * a.e13
                - b.e1 * a.e1p
                - b.e1pn * a.e1n
                - b.e23p * a.e23
                - b.e2 * a.e2p
                - b.e2pn * a.e2n
                - b.e3 * a.e3p
                - b.e3pn * a.e3n
                - b.en * a.epn
                - b.e123p * a.e123
                - b.e12 * a.e12p
                - b.e12pn * a.e12n
                - b.e13 * a.e13p
                - b.e13pn * a.e13n
                - b.e1n * a.e1pn
                - b.e23 * a.e23p
                - b.e23pn * a.e23n
                - b.e2n * a.e2pn
                - b.e3n * a.e3pn
                + b.e123 * a.e123p
                + b.e123pn * a.e123n
                + b.e12n * a.e12pn
                + b.e13n * a.e13pn
                + b.e23n * a.e23pn
                + b.e123n * a.e123pn,
            en: b.en * a.s + b.e1n * a.e1 + b.e2n * a.e2 + b.e3n * a.e3 + b.epn * a.ep + b.s * a.en
                - b.e12n * a.e12
                - b.e13n * a.e13
                - b.e1pn * a.e1p
                - b.e1 * a.e1n
                - b.e23n * a.e23
                - b.e2pn * a.e2p
                - b.e2 * a.e2n
                - b.e3pn * a.e3p
                - b.e3 * a.e3n
                - b.ep * a.epn
                - b.e123n * a.e123
                - b.e12pn * a.e12p
                - b.e12 * a.e12n
                - b.e13pn * a.e13p
                - b.e13 * a.e13n
                - b.e1p * a.e1pn
                - b.e23pn * a.e23p
                - b.e23 * a.e23n
                - b.e2p * a.e2pn
                - b.e3p * a.e3pn
                + b.e123pn * a.e123p
                + b.e123 * a.e123n
                + b.e12p * a.e12pn
                + b.e13p * a.e13pn
                + b.e23p * a.e23pn
                + b.e123p * a.e123pn,
            e12: b.e12 * a.s + b.e123 * a.e3 + b.e12p * a.ep - b.e12n * a.en + b.s * a.e12
                - b.e123p * a.e3p
                + b.e123n * a.e3n
                + b.e12pn * a.epn
                + b.e3 * a.e123
                + b.ep * a.e12p
                - b.en * a.e12n
                + b.e123pn * a.e3pn
                - b.e3p * a.e123p
                + b.e3n * a.e123n
                + b.epn * a.e12pn
                + b.e3pn * a.e123pn,
            e13: b.e13 * a.s - b.e123 * a.e2 + b.e13p * a.ep - b.e13n * a.en
                + b.s * a.e13
                + b.e123p * a.e2p
                - b.e123n * a.e2n
                + b.e13pn * a.epn
                - b.e2 * a.e123
                + b.ep * a.e13p
                - b.en * a.e13n
                - b.e123pn * a.e2pn
                + b.e2p * a.e123p
                - b.e2n * a.e123n
                + b.epn * a.e13pn
                - b.e2pn * a.e123pn,
            e1p: b.e1p * a.s - b.e12p * a.e2 - b.e13p * a.e3 - b.e1pn * a.en + b.s * a.e1p
                - b.e123p * a.e23
                - b.e12pn * a.e2n
                - b.e13pn * a.e3n
                - b.e2 * a.e12p
                - b.e3 * a.e13p
                - b.en * a.e1pn
                + b.e123pn * a.e23n
                - b.e23 * a.e123p
                - b.e2n * a.e12pn
                - b.e3n * a.e13pn
                + b.e23n * a.e123pn,
            e1n: b.e1n * a.s - b.e12n * a.e2 - b.e13n * a.e3 - b.e1pn * a.ep + b.s * a.e1n
                - b.e123n * a.e23
                - b.e12pn * a.e2p
                - b.e13pn * a.e3p
                - b.e2 * a.e12n
                - b.e3 * a.e13n
                - b.ep * a.e1pn
                + b.e123pn * a.e23p
                - b.e23 * a.e123n
                - b.e2p * a.e12pn
                - b.e3p * a.e13pn
                + b.e23p * a.e123pn,
            e23: b.e23 * a.s + b.e123 * a.e1 + b.e23p * a.ep - b.e23n * a.en - b.e123p * a.e1p
                + b.e123n * a.e1n
                + b.s * a.e23
                + b.e23pn * a.epn
                + b.e1 * a.e123
                + b.e123pn * a.e1pn
                + b.ep * a.e23p
                - b.en * a.e23n
                - b.e1p * a.e123p
                + b.e1n * a.e123n
                + b.epn * a.e23pn
                + b.e1pn * a.e123pn,
            e2p: b.e2p * a.s + b.e12p * a.e1 - b.e23p * a.e3 - b.e2pn * a.en
                + b.e123p * a.e13
                + b.e12pn * a.e1n
                + b.s * a.e2p
                - b.e23pn * a.e3n
                + b.e1 * a.e12p
                - b.e123pn * a.e13n
                - b.e3 * a.e23p
                - b.en * a.e2pn
                + b.e13 * a.e123p
                + b.e1n * a.e12pn
                - b.e3n * a.e23pn
                - b.e13n * a.e123pn,
            e2n: b.e2n * a.s + b.e12n * a.e1 - b.e23n * a.e3 - b.e2pn * a.ep
                + b.e123n * a.e13
                + b.e12pn * a.e1p
                + b.s * a.e2n
                - b.e23pn * a.e3p
                + b.e1 * a.e12n
                - b.e123pn * a.e13p
                - b.e3 * a.e23n
                - b.ep * a.e2pn
                + b.e13 * a.e123n
                + b.e1p * a.e12pn
                - b.e3p * a.e23pn
                - b.e13p * a.e123pn,
            e3p: b.e3p * a.s + b.e13p * a.e1 + b.e23p * a.e2 - b.e3pn * a.en - b.e123p * a.e12
                + b.e13pn * a.e1n
                + b.e23pn * a.e2n
                + b.s * a.e3p
                + b.e123pn * a.e12n
                + b.e1 * a.e13p
                + b.e2 * a.e23p
                - b.en * a.e3pn
                - b.e12 * a.e123p
                + b.e1n * a.e13pn
                + b.e2n * a.e23pn
                + b.e12n * a.e123pn,
            e3n: b.e3n * a.s + b.e13n * a.e1 + b.e23n * a.e2 - b.e3pn * a.ep - b.e123n * a.e12
                + b.e13pn * a.e1p
                + b.e23pn * a.e2p
                + b.s * a.e3n
                + b.e123pn * a.e12p
                + b.e1 * a.e13n
                + b.e2 * a.e23n
                - b.ep * a.e3pn
                - b.e12 * a.e123n
                + b.e1p * a.e13pn
                + b.e2p * a.e23pn
                + b.e12p * a.e123pn,
            epn: b.epn * a.s + b.e1pn * a.e1 + b.e2pn * a.e2 + b.e3pn * a.e3
                - b.e12pn * a.e12
                - b.e13pn * a.e13
                - b.e23pn * a.e23
                + b.s * a.epn
                - b.e123pn * a.e123
                + b.e1 * a.e1pn
                + b.e2 * a.e2pn
                + b.e3 * a.e3pn
                - b.e12 * a.e12pn
                - b.e13 * a.e13pn
                - b.e23 * a.e23pn
                - b.e123 * a.e123pn,
            e123: b.e123 * a.s - b.e123p * a.ep
                + b.e123n * a.en
                + b.e123pn * a.epn
                + b.s * a.e123
                + b.ep * a.e123p
                - b.en * a.e123n
                + b.epn * a.e123pn,
            e12p: b.e12p * a.s + b.e123p * a.e3 + b.e12pn * a.en - b.e123pn * a.e3n + b.s * a.e12p
                - b.e3 * a.e123p
                - b.en * a.e12pn
                - b.e3n * a.e123pn,
            e12n: b.e12n * a.s + b.e123n * a.e3 + b.e12pn * a.ep - b.e123pn * a.e3p + b.s * a.e12n
                - b.e3 * a.e123n
                - b.ep * a.e12pn
                - b.e3p * a.e123pn,
            e13p: b.e13p * a.s - b.e123p * a.e2
                + b.e13pn * a.en
                + b.e123pn * a.e2n
                + b.s * a.e13p
                + b.e2 * a.e123p
                - b.en * a.e13pn
                + b.e2n * a.e123pn,
            e13n: b.e13n * a.s - b.e123n * a.e2
                + b.e13pn * a.ep
                + b.e123pn * a.e2p
                + b.s * a.e13n
                + b.e2 * a.e123n
                - b.ep * a.e13pn
                + b.e2p * a.e123pn,
            e1pn: b.e1pn * a.s - b.e12pn * a.e2 - b.e13pn * a.e3 - b.e123pn * a.e23
                + b.s * a.e1pn
                + b.e2 * a.e12pn
                + b.e3 * a.e13pn
                - b.e23 * a.e123pn,
            e23p: b.e23p * a.s + b.e123p * a.e1 + b.e23pn * a.en - b.e123pn * a.e1n + b.s * a.e23p
                - b.e1 * a.e123p
                - b.en * a.e23pn
                - b.e1n * a.e123pn,
            e23n: b.e23n * a.s + b.e123n * a.e1 + b.e23pn * a.ep - b.e123pn * a.e1p + b.s * a.e23n
                - b.e1 * a.e123n
                - b.ep * a.e23pn
                - b.e1p * a.e123pn,
            e2pn: b.e2pn * a.s + b.e12pn * a.e1 - b.e23pn * a.e3 + b.e123pn * a.e13 + b.s * a.e2pn
                - b.e1 * a.e12pn
                + b.e3 * a.e23pn
                + b.e13 * a.e123pn,
            e3pn: b.e3pn * a.s + b.e13pn * a.e1 + b.e23pn * a.e2 - b.e123pn * a.e12 + b.s * a.e3pn
                - b.e1 * a.e13pn
                - b.e2 * a.e23pn
                - b.e12 * a.e123pn,
            e123p: b.e123p * a.s - b.e123pn * a.en + b.s * a.e123p - b.en * a.e123pn,
            e123n: b.e123n * a.s - b.e123pn * a.ep + b.s * a.e123n - b.ep * a.e123pn,
            e12pn: b.e12pn * a.s + b.e123pn * a.e3 + b.s * a.e12pn + b.e3 * a.e123pn,
            e13pn: b.e13pn * a.s - b.e123pn * a.e2 + b.s * a.e13pn - b.e2 * a.e123pn,
            e23pn: b.e23pn * a.s + b.e123pn * a.e1 + b.s * a.e23pn + b.e1 * a.e123pn,
            e123pn: b.e123pn * a.s + b.s * a.e123pn,
        }
    }
}

// Add
// Multivector addition
impl<T: Add<U>, U> Add<R410<U>> for R410<T> {
    type Output = R410<T::Output>;

    #[inline]
    fn add(self, b: R410<U>) -> Self::Output {
        let a = self;
        R410 {
            s: a.s + b.s,
            e1: a.e1 + b.e1,
            e2: a.e2 + b.e2,
            e3: a.e3 + b.e3,
            ep: a.ep + b.ep,
            en: a.en + b.en,
            e12: a.e12 + b.e12,
            e13: a.e13 + b.e13,
            e1p: a.e1p + b.e1p,
            e1n: a.e1n + b.e1n,
            e23: a.e23 + b.e23,
            e2p: a.e2p + b.e2p,
            e2n: a.e2n + b.e2n,
            e3p: a.e3p + b.e3p,
            e3n: a.e3n + b.e3n,
            epn: a.epn + b.epn,
            e123: a.e123 + b.e123,
            e12p: a.e12p + b.e12p,
            e12n: a.e12n + b.e12n,
            e13p: a.e13p + b.e13p,
            e13n: a.e13n + b.e13n,
            e1pn: a.e1pn + b.e1pn,
            e23p: a.e23p + b.e23p,
            e23n: a.e23n + b.e23n,
            e2pn: a.e2pn + b.e2pn,
            e3pn: a.e3pn + b.e3pn,
            e123p: a.e123p + b.e123p,
            e123n: a.e123n + b.e123n,
            e12pn: a.e12pn + b.e12pn,
            e13pn: a.e13pn + b.e13pn,
            e23pn: a.e23pn + b.e23pn,
            e123pn: a.e123pn + b.e123pn,
        }
    }
}

// Sub
// Multivector subtraction
impl<T: Field + Copy> Sub for R410<T> {
    type Output = Self;

    #[inline]
    fn sub(self, b: Self) -> Self {
        self + (-b)
    }
}

// muls
// multivector/scalar multiplication
impl<T: Field + Copy> Mul<T> for R410<T> {
    type Output = Self;

    #[inline]
    fn mul(self, b: T) -> Self {
        self * Self::from(b)
    }
}

// adds
// multivector/scalar addition
impl<T: Field + Copy> Add<T> for R410<T> {
    type Output = Self;

    #[inline]
    fn add(self, b: T) -> Self {
        self + Self::from(b)
    }
}

// sub
// scalar/multivector subtraction
impl<T: Field + Copy> Sub<T> for R410<T> {
    type Output = Self;

    #[inline]
    fn sub(self, b: T) -> Self {
        self - Self::from(b)
    }
}

impl<T: Field + Copy> Div<T> for R410<T> {
    type Output = Self;
    #[inline]
    fn div(self, b: T) -> Self {
        self * b.simd_recip()
    }
}

// smul
// scalar/multivector multiplication
impl Mul<R410<f64>> for f64 {
    type Output = R410<Self>;

    #[inline]
    fn mul(self, b: R410<Self>) -> Self::Output {
        b * self
    }
}

impl Div<R410<f64>> for f64 {
    type Output = R410<Self>;

    #[inline]
    fn div(self, b: R410<Self>) -> Self::Output {
        b.recip() * self
    }
}

// sadd
// scalar/multivector addition
impl Add<R410<f64>> for f64 {
    type Output = R410<Self>;

    #[inline]
    fn add(self, b: R410<Self>) -> Self::Output {
        b + self
    }
}

// sub
// multivector/scalar subtraction
impl Sub<R410<f64>> for f64 {
    type Output = R410<Self>;

    #[inline]
    fn sub(self, b: R410<Self>) -> Self::Output {
        (-b) + self
    }
}

// smul
// scalar/multivector multiplication
impl Mul<R410<f32>> for f32 {
    type Output = R410<Self>;

    #[inline]
    fn mul(self, b: R410<Self>) -> Self::Output {
        b * self
    }
}

impl Div<R410<f32>> for f32 {
    type Output = R410<Self>;

    #[inline]
    fn div(self, b: R410<Self>) -> Self::Output {
        b.recip() * self
    }
}

// sadd
// scalar/multivector addition
impl Add<R410<f32>> for f32 {
    type Output = R410<Self>;

    #[inline]
    fn add(self, b: R410<Self>) -> Self::Output {
        b + self
    }
}

// sub
// multivector/scalar subtraction
impl Sub<R410<f32>> for f32 {
    type Output = R410<Self>;

    #[inline]
    fn sub(self, b: R410<Self>) -> Self::Output {
        (-b) + self
    }
}

impl<T: Field + Copy> R410<T> {
    #[inline]
    pub fn norm_squared(self) -> T {
        (self * self.conjugate()).s
    }
    #[inline]
    pub fn norm(self) -> T {
        self.norm_squared().simd_abs().simd_sqrt()
    }
    #[inline]
    pub fn recip(self) -> Self {
        self.conjugate() / self.norm_squared()
    }

    pub fn inorm(self) -> T {
        self.dual().norm()
    }

    pub fn normalized(self) -> Self {
        self / self.norm()
    }
}
