use super::F32;

#[cfg_attr(feature = "repr_simd", repr(simd))]
#[cfg_attr(not(feature = "repr_simd"), repr(align(32)))]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct F32xN<const N: usize>([f32; N]);

#[cfg_attr(feature = "repr_simd", repr(simd))]
#[cfg_attr(not(feature = "repr_simd"), repr(align(16)))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BoolxN<const N: usize>([bool; N]);

impl<const N: usize> F32xN<N> {
    #[inline]
    pub fn new(x: [f32; N]) -> Self {
        Self::try_new(x).expect("Fast math operations unsafe with infinity and NaN")
    }

    #[inline]
    pub fn splat(x: f32) -> Self {
        Self::try_splat(x).expect("Fast math operations unsafe with infinity and NaN")
    }

    #[inline]
    pub fn try_new(x: [f32; N]) -> Option<Self> {
        if x.iter().all(f32::is_finite) {
            Some(F32xN(x))
        } else {
            None
        }
    }

    #[inline]
    pub fn try_splat(x: f32) -> Option<Self> {
        if x.is_finite() {
            Some(F32xN([x; N]))
        } else {
            None
        }
    }

    #[inline]
    pub fn into_inner(self) -> [f32; N] {
        self.0
    }
}

macro_rules! impl_fmt {
    ($($trait:path),*) => {$(
        impl<const N: usize> $trait for F32xN<N> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                <[f32] as $trait>::fmt(&self.0[..], f)
            }
        }
    )*};
}

impl_fmt!(Display, LowerExp, UpperExp);

impl<const N: usize> Add for F32xN<N> {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: Self) -> Self {
    	self += rhs;
    	self
    }
}

impl<const N: usize> AddAssign for F32xN<N> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
    	for i in 0..N {
        	self.0[i] = unsafe { Self(fadd_fast(self.0[i], rhs.0[i])) };
    	}
    }
}

impl<const N: usize> Sub for F32xN<N> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl<const N: usize> SubAssign for F32xN<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
    	for i in 0..N {
        	self.0[i] = unsafe { Self(fsub_fast(self.0[i], rhs.0[i])) };
    	}
    }
}

impl<const N: usize> Mul for F32xN<N> {
    type Output = Self;
    #[inline]
    fn mul(mut self, rhs: Self) -> Self {
        self *= rhs;
        self
    }
}

impl<const N: usize> MulAssign for F32xN<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
    	for i in 0..N {
        	self.0[i] = unsafe { Self(fmul_fast(self.0[i], rhs.0[i])) };
    	}
    }
}

impl<const N: usize> Div for F32xN<N> {
    type Output = Self;
    #[inline]
    fn div(mut self, rhs: Self) -> Self {
        self /= rhs;
        self
    }
}

impl<const N: usize> DivAssign for F32xN<N> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
    	assert!(!rhs.0.iter().any(f32::is_zero), "Division by zero");
    	for i in 0..N {
        	self.0[i] = unsafe { Self(fdiv_fast(self.0[i], rhs.0[i])) };
    	}
    }
}

impl<const N: usize> Rem for F32xN<N> {
    type Output = Self;
    #[inline]
    fn rem(mut self, rhs: Self) -> Self {
        self %= rhs;
        self
    }
}

impl<const N: usize> RemAssign for F32xN<N> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
    	assert!(!rhs.0.iter().any(f32::is_zero), "Division by zero");
    	for i in 0..N {
        	self.0[i] = unsafe { Self(frem_fast(self.0[i], rhs.0[i])) };
    	}
    }
}

impl<const N: usize> Neg for F32xN<N> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self([0.0; N]) - self
    }
}

impl<const N: usize> Zero for F32xN<N> {
    #[inline]
    fn zero() -> Self {
        Self([0.0; N])
    }
    #[inline]
    fn is_zero(&self) -> bool {
    	self.0.iter().all(|x| x == 0.0 || x == -0.0)
    }
}

impl<const N: usize> One for F32xN<N> {
    #[inline]
    fn one() -> Self {
        Self([1.0; N])
    }
}

impl<const N: usize> Num for F32xN<N> {
    type FromStrRadixErr = <f32 as Num>::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        f32::from_str_radix(str, radix).map(Self)
    }
}

impl<const N: usize> SimdValue for F32xN<N> {
    type Element = F32;
    type SimdBool = BoolxN<N>;
    fn lanes() -> usize {
        N
    }
    fn splat(x: Self::Element) -> Self {
        Self([x.0; N])
    }
    fn extract(&self, i: usize) -> Self::Element {
        assert_lt!(i, N);
        F32(self.0[i])
    }
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        F32(self.0[i])
    }
    fn replace(&mut self, i: usize, val: Self::Element) {
        assert_lt!(i, N);
        self.0[i] = F32(val);
    }
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        self.0[i] = F32(val);
    }
    fn select(mut self, cond: bool, other: Self) -> Self {
    	for i in 0..N {
    		if !cond.0[i] {
    			self.0[i] = other.0[i];
    		}
    	}
        self
    }
}

impl<const N: usize> BitAnd for F32xN<N> {
    type Output = Self;
    #[inline]
    fn bitand(mut self, rhs: Self) -> Self {
    	self &= rhs;
    	self
    }
}

impl<const N: usize> BitAndAssign for F32xN<N> {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
    	for i in 0..N {
    		self.0[i] = self.0[i] && rhs.0[i]
    	}
    }
}

impl<const N: usize> BitOr for BoolxN<N> {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        self |= rhs;
        self
    }
}

impl<const N: usize> BitOrAssign for BoolxN<N> {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
    	for i in 0..N {
        	self.0[i] = self.0[i] || rhs.0[i];
    	}
    }
}

impl<const N: usize> BitXor for BoolxN<N> {
    type Output = Self;
    #[inline]
    fn bitxor(mut self, rhs: Self) -> Self {
        self ^= rhs;
        self
    }
}

impl<const N: usize> BitXorAssign for BoolxN<N> {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
    	for i in 0..N {
        	self.0[i] = self.0[i] != rhs.0[i];
    	}
    }
}

impl<const N: usize> Not for BoolxN<N> {
    type Output = Self;
    #[inline]
    fn div(mut self) -> Self {
    	for i in 0..N {
    		self.0[i] = !self.0[i];
    	}
        self
    }
}

fn id<T>(&x: &T) -> T { x }

impl<const N: usize> SimdBool for BoolxN<N>{
    fn bitmask(self) -> u64 {
    	assert_le(N, 64, "Too many elements to fit in bitmask");
    	self.0.iter().enumerate().fold(0, |mask, (i, b)| mask | ((b as u64) << i)
    }
    fn and(self) -> bool { self.0.iter().all(id) }
    fn or(self) -> bool { self.0.iter().any(id) }
    fn xor(self) -> bool { self.0.iter().filter(|&&x|x).count() & 1 != 0 }
    fn all(self) -> bool { self.0.iter().all(id) }
    fn any(self) -> bool { self.0.iter().any(id) }
    fn none(self) -> bool { !self.0.iter().any(id) }
    fn if_else<Res: SimdValue<SimdBool = Self>>(
        self, 
        if_value: impl FnOnce() -> Res, 
        else_value: impl FnOnce() -> Res
    ) -> Res {
    	if_value().select(self, else_value())
    }
    fn if_else2<Res: SimdValue<SimdBool = Self>>(
        self, 
        if_value: impl FnOnce() -> Res, 
        else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res), 
        else_value: impl FnOnce() -> Res
    ) -> Res {
    	if_value().select(self, else_if.1().select(else_if.0(), else_value()))
    }
    fn if_else3<Res: SimdValue<SimdBool = Self>>(
        self, 
        if_value: impl FnOnce() -> Res, 
        else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res), 
        else_else_if: (impl FnOnce() -> Self, impl FnOnce() -> Res), 
        else_value: impl FnOnce() -> Res
    ) -> Res {
    	if_value().select(self, else_if.1().select(else_if.0(), else_else_if.1().select(else_else_if.0(), else_value())))
    }
}

impl<const N: usize> Field for F32xN<N> {}

impl<const N: usize> SubsetOf<F32xN<N>> for F32xN<N> {
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
impl<const N: usize> SubsetOf<F32xN<N>> for f64 {
    fn to_superset(&self) -> F32xN<N> {
        assert!(self.is_finite(), "Fast math operations unsafe with infinity and NaN");
        F32xN([*self as f32; N])
    }
    fn from_superset_unchecked(element: &Self) -> Self {
        *element
    }
    fn is_in_subset(element: &F32xN<N>) -> bool {
        element.0.iter()
    }
}