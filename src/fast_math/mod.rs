//! Fast math wrappers to improve optimizations involving subtracting like terms and multiplying/adding zeroes.

use core::fmt::{self, Display, Formatter};
use core::intrinsics::{fadd_fast, fdiv_fast, fmul_fast, frem_fast, fsub_fast};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::{Bounded, Num, One, Signed, Zero};
use simba::scalar::{ComplexField, Field, RealField, SubsetOf};
use simba::simd::SimdValue;

use crate::Z;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct F32(f32);

impl F32 {
    #[inline]
    pub fn new(x: f32) -> Self {
        assert!(
            x.is_finite(),
            "Fast math operations unsafe with infinity and NaN"
        );
        F32(x)
    }
}

impl From<Z> for F32 {
    #[inline]
    fn from(Z: Z) -> Self {
        Self(0.0)
    }
}

impl Display for F32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Add for F32 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        unsafe { Self(fadd_fast(self.0, rhs.0)) }
    }
}

impl AddAssign for F32 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for F32 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        unsafe { Self(fsub_fast(self.0, rhs.0)) }
    }
}

impl SubAssign for F32 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul for F32 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        unsafe { Self(fmul_fast(self.0, rhs.0)) }
    }
}

impl MulAssign for F32 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Div for F32 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        debug_assert!(!rhs.is_zero(), "Division by zero");
        unsafe { Self(fdiv_fast(self.0, rhs.0)) }
    }
}

impl DivAssign for F32 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl Rem for F32 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        debug_assert!(!rhs.is_zero(), "Division by zero");
        unsafe { Self(frem_fast(self.0, rhs.0)) }
    }
}

impl RemAssign for F32 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl Neg for F32 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        unsafe { Self(fsub_fast(0.0, self.0)) }
    }
}

impl Zero for F32 {
    #[inline]
    fn zero() -> Self {
        Self(0.0)
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.0 == 0.0 || self.0 == -0.0
    }
}

impl One for F32 {
    #[inline]
    fn one() -> Self {
        Self(1.0)
    }
}

impl Num for F32 {
    type FromStrRadixErr = <f32 as Num>::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        f32::from_str_radix(str, radix).map(Self)
    }
}

impl SimdValue for F32 {
    type Element = Self;
    type SimdBool = bool;
    fn lanes() -> usize {
        1
    }
    fn splat(x: Self) -> Self {
        x
    }
    fn extract(&self, i: usize) -> Self {
        debug_assert_eq!(i, 0);
        *self
    }
    unsafe fn extract_unchecked(&self, _i: usize) -> Self {
        *self
    }
    fn replace(&mut self, i: usize, val: Self) {
        debug_assert_eq!(i, 0);
        *self = val
    }
    unsafe fn replace_unchecked(&mut self, _i: usize, val: Self) {
        *self = val
    }
    fn select(self, cond: bool, other: Self) -> Self {
        if cond {
            self
        } else {
            other
        }
    }
}

impl Field for F32 {}
impl SubsetOf<F32> for F32 {
    fn to_superset(&self) -> Self {
        *self
    }
    fn from_superset_unchecked(element: &Self) -> Self {
        *element
    }
    fn is_in_subset(_element: &Self) -> bool {
        true
    }
}
impl SubsetOf<F32> for f64 {
    #[inline]
    fn to_superset(&self) -> F32 {
        F32::new(*self as f32)
    }
    fn from_superset_unchecked(element: &F32) -> Self {
        element.0 as f64
    }
    fn is_in_subset(_element: &F32) -> bool {
        true
    }
}

impl ComplexField for F32 {
    type RealField = Self;
    fn from_real(re: Self) -> Self {
        re
    }
    fn real(self) -> Self {
        self
    }
    fn imaginary(self) -> Self {
        Self::zero()
    }
    fn modulus(self) -> Self {
        Self(self.0.modulus())
    }
    fn modulus_squared(self) -> Self {
        self * self
    }
    fn argument(self) -> Self {
        Self(self.0.argument())
    }
    fn norm1(self) -> Self {
        Self(self.0.norm1())
    }
    fn scale(self, factor: Self) -> Self {
        self * factor
    }
    fn unscale(self, factor: Self) -> Self {
        self / factor
    }
    fn floor(self) -> Self {
        Self(self.0.floor())
    }
    fn ceil(self) -> Self {
        Self(self.0.ceil())
    }
    fn round(self) -> Self {
        Self(self.0.round())
    }
    fn trunc(self) -> Self {
        Self(self.0.trunc())
    }
    fn fract(self) -> Self {
        Self(self.0.fract())
    }
    fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.0.mul_add(a.0, b.0))
    }
    fn abs(self) -> Self {
        Self(self.0.abs())
    }
    fn hypot(self, other: Self) -> Self {
        Self(self.0.hypot(other.0))
    }
    fn recip(self) -> Self {
        debug_assert!(!self.is_zero(), "Division by zero");
        Self(self.0.recip())
    }
    fn conjugate(self) -> Self {
        Self(self.0.conjugate())
    }
    fn sin(self) -> Self {
        Self(self.0.sin())
    }
    fn cos(self) -> Self {
        Self(self.0.cos())
    }
    fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.0.sin_cos();
        (Self(s), Self(c))
    }
    fn tan(self) -> Self {
        Self(self.0.tan())
    }
    fn asin(self) -> Self {
        debug_assert!(
            self.0 < 1.0 && self.0 > -1.0,
            "Arcsin argument outside domain"
        );
        Self(self.0.asin())
    }
    fn acos(self) -> Self {
        debug_assert!(
            self.0 < 1.0 && self.0 > -1.0,
            "Arccos argument outside domain"
        );
        Self(self.0.acos())
    }
    fn atan(self) -> Self {
        Self(self.0.atan())
    }
    fn sinh(self) -> Self {
        Self(self.0.sinh())
    }
    fn cosh(self) -> Self {
        Self(self.0.cosh())
    }
    fn tanh(self) -> Self {
        Self(self.0.tanh())
    }
    fn asinh(self) -> Self {
        Self(self.0.asinh())
    }
    fn acosh(self) -> Self {
        debug_assert!(self.0 >= 1.0, "Arccosh argument outside domain");
        Self(self.0.acosh())
    }
    fn atanh(self) -> Self {
        debug_assert!(
            self.0 < 1.0 && self.0 > -1.0,
            "Arctanh argument outside domain"
        );
        Self(self.0.atanh())
    }
    fn log(self, base: Self) -> Self {
        debug_assert!(self.0 > 0.0, "log argument outside domain");
        Self(self.0.log(base.0))
    }
    fn log2(self) -> Self {
        debug_assert!(self.0 > 0.0, "log argument outside domain");
        Self(self.0.log2())
    }
    fn log10(self) -> Self {
        debug_assert!(self.0 > 0.0, "log argument outside domain");
        Self(self.0.log10())
    }
    fn ln(self) -> Self {
        debug_assert!(self.0 > 0.0, "log argument outside domain");
        Self(self.0.ln())
    }
    fn ln_1p(self) -> Self {
        debug_assert!(self.0 > -1.0, "log argument outside domain");
        Self(self.0.ln_1p())
    }
    fn sqrt(self) -> Self {
        debug_assert!(self.0 >= 0.0, "Imaginary square root");
        Self(self.0.sqrt())
    }
    fn exp(self) -> Self {
        Self(self.0.exp())
    }
    fn exp2(self) -> Self {
        Self(self.0.exp2())
    }
    fn exp_m1(self) -> Self {
        Self(self.0.exp_m1())
    }
    fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }
    fn powf(self, n: Self) -> Self {
        Self(self.0.powf(n.0))
    }
    fn powc(self, n: Self) -> Self {
        Self(self.0.powc(n.0))
    }
    fn cbrt(self) -> Self {
        Self(self.0.cbrt())
    }
    fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    fn try_sqrt(self) -> Option<Self> {
        self.0.try_sqrt().map(Self)
    }
}

impl AbsDiffEq for F32 {
    type Epsilon = Self;
    fn default_epsilon() -> Self::Epsilon {
        Self(f32::default_epsilon())
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self) -> bool {
        (*self - *other).abs() <= epsilon
    }
}
impl RelativeEq for F32 {
    fn default_max_relative() -> Self {
        Self(f32::EPSILON)
    }
    fn relative_eq(&self, other: &Self, epsilon: Self, max_relative: Self) -> bool {
        self.0.relative_eq(&other.0, epsilon.0, max_relative.0)
    }
}
impl UlpsEq for F32 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon.0, max_ulps)
    }
}

impl Signed for F32 {
    fn abs(&self) -> Self {
        Self(self.0.abs())
    }
    fn abs_sub(&self, other: &Self) -> Self {
        Self(Signed::abs_sub(&self.0, &other.0))
    }
    fn signum(&self) -> Self {
        Self(self.0.signum())
    }
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}
impl Bounded for F32 {
    fn min_value() -> Self {
        Self(f32::min_value())
    }
    fn max_value() -> Self {
        Self(f32::max_value())
    }
}

impl RealField for F32 {
    fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }
    fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
    fn copysign(self, to: Self) -> Self {
        Self(self.0.copysign(to.0))
    }
    fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }
    fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }
    fn clamp(self, min: Self, max: Self) -> Self {
        Self(self.0.clamp(min.0, max.0))
    }
    fn atan2(self, other: Self) -> Self {
        Self(self.0.atan2(other.0))
    }
    fn pi() -> Self {
        Self(f32::pi())
    }
    fn two_pi() -> Self {
        Self(f32::two_pi())
    }
    fn frac_pi_2() -> Self {
        Self(f32::frac_pi_2())
    }
    fn frac_pi_3() -> Self {
        Self(f32::frac_pi_3())
    }
    fn frac_pi_4() -> Self {
        Self(f32::frac_pi_4())
    }
    fn frac_pi_6() -> Self {
        Self(f32::frac_pi_6())
    }
    fn frac_pi_8() -> Self {
        Self(f32::frac_pi_8())
    }
    fn frac_1_pi() -> Self {
        Self(f32::frac_1_pi())
    }
    fn frac_2_pi() -> Self {
        Self(f32::frac_2_pi())
    }
    fn frac_2_sqrt_pi() -> Self {
        Self(f32::frac_2_sqrt_pi())
    }
    fn e() -> Self {
        Self(f32::e())
    }
    fn log2_e() -> Self {
        Self(f32::log2_e())
    }
    fn log10_e() -> Self {
        Self(f32::log10_e())
    }
    fn ln_2() -> Self {
        Self(f32::ln_2())
    }
    fn ln_10() -> Self {
        Self(f32::ln_10())
    }
}
