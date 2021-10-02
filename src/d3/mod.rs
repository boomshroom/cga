mod r410;
//use simba::simd::SimdRealField as Field;

mod flat;
mod round;
mod transform;
mod dual;
mod free;

pub use round::{Point, Pair, Circle, Sphere};
pub use flat::{Line, Plane};

/*
#[derive(Debug, Copy, Clone, Default)]
struct Multivector<S, V, B, T, Q, P> {
    s: S,
    e1: V,
    e2: V,
    e3: V,
    e4: V,
    e5: V,
    e12: B,
    e13: B,
    e14: B,
    e15: B,
    e23: B,
    e24: B,
    e25: B,
    e34: B,
    e35: B,
    e45: B,
    e123: T,
    e124: T,
    e125: T,
    e134: T,
    e135: T,
    e145: T,
    e234: T,
    e235: T,
    e245: T,
    e345: T,
    e1234: Q,
    e1235: Q,
    e1245: Q,
    e1345: Q,
    e2345: Q,
    e12345: P,
}

impl <S, V, B, T, Q, P> Multivector<S, V, B, T, Q, P> {
    pub fn grade_0(self) -> S {
        self.s
    }

    pub fn grade_1(self) -> Point<V> {
        let Multivector { e1, e2, e3, e4, e5, .. } = self;
        Point{ e1, e2, e3, e4, e5 }
    }

    pub fn grade_2(self) -> Pair<B> {
        let Multivector { e12, e13, e14, e15, e23, e24, e25, e34, e35, e45 .. } = self;
        Pair{ e12, e13, e14, e15, e23, e24, e25, e34, e35, e45 }
    }

    pub fn grade_3(self) -> Circle<T> {
        let Multivector { e123, e124, e125, e134, e135, e145, e234, e235, e245, e345, .. } = self;
        Circle{ e123, e124, e125, e134, e135, e145, e234, e235, e245, e345 }
    }

    pub fn grade_4(self) -> Sphere<Q> {
        let Multivector { e1234, e1235, e1245, e1345, e2345, .. } = self;
        Sphere{ e1234, e1235, e1245, e1345, e2345 }
    }

    pub fn grade_5(self) -> P {
        self.s
    }
}
*/
