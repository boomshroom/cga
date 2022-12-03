use core::ops::{Mul, Add};

#[derive(Copy, Clone)]
pub struct KBlade<const P: u32, const N: u32, const Z: u32> {
    value: f32,
}

#[inline(always)]
const fn flips(p1: u32, n1: u32, z1: u32, p2: u32, n2: u32, z2: u32) -> u32 {
    #[inline(always)]
    const fn two_flips(x1: u32, mut x2: u32) -> u32 {
        let mut mask = !0;
        let mut flips = 0;
        while x2 != 0 {
            let shift = x2.trailing_zeros() + 1;
            mask <<= shift;
            x2 >>= shift;
            flips += (x1 & mask).count_ones();
        }
        flips
    }
    let mut flips = 0;
    // p1 n1 z1 p2 n2 z2
    flips += p2.count_ones() * z1.count_ones();
    // p1 n1 p2 z1 n2 z2
    flips += p2.count_ones() * n1.count_ones();
    // p1 p2 n1 z1 n2 z2
    flips += two_flips(p1, p2);
    // p n1 z1 n2 z2
    flips += n2.count_ones() * z1.count_ones();
    // p n1 n2 z1 z2
    flips += two_flips(n1, n2);
    // p n z1 z2
    flips += two_flips(z1, z2);
    // p n z
    flips
}

impl<const P1: u32, const N1: u32, const Z1: u32, const P2: u32, const N2: u32, const Z2: u32>
Mul<KBlade<P2, N2, Z2>> for KBlade<P1, N1, Z1>
where [(); (P1^P2) as usize]: Sized, [(); (N1^N2) as usize]: Sized, [(); (Z1^Z2) as usize]: Sized,
{
    type Output = KBlade<{P1^P2}, {N1^N2}, {Z1^Z2}>;
    fn mul(self, rhs: KBlade<P2, N2, Z2>) -> Self::Output {
        if Z1 & Z2 != 0 {
            KBlade{ value: 0.0 }
        } else if flips(P1, N1, Z1, P2, N2, Z2) % 2 != 0 {
            KBlade{ value: -self.value * rhs.value }
        } else {
            KBlade{ value: self.value * rhs.value }
        }
    }
}

impl<const P: u32, const N: u32, const Z: u32> Add for KBlade<P, N, Z> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        KBlade{ value: self.value + rhs.value }
    }
}

pub type EBlade<const E: u32> = KBlade<E, 0, 0>;
pub type Scalar = KBlade<0, 0, 0>;

pub type B<E1, E2> = <E1 as Mul<E2>>::Output;

pub type E1 = EBlade<0b001>;
pub type E2 = EBlade<0b010>;
pub type E3 = EBlade<0b100>;

pub type E12 = B<E1, E2>;
pub type E13 = B<E1, E3>;
pub type E23 = B<E2, E3>;

pub struct Vector3 {
    x: E1,
    y: E2,
    z: E3,
}


pub struct Rotor3 {
    s: Scalar,
    xy: E12,
    xz: E13,
    yz: E23,
}


impl Mul for Vector3 {
    type Output = Rotor3;
    fn mul(self, rhs: Self) -> Rotor3 {
        Rotor3 {
            s: self.x * rhs.x + self.y * rhs.y + self.z * rhs.z,
            xy: self.x * rhs.y + self.y * rhs.x,
            xz: self.x * rhs.z + self.z * rhs.x,
            yz: self.y * rhs.z + self.z * rhs.y,
        }
    }
}
