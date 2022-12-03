use core::fmt::{self, Display, Formatter, LowerExp, UpperExp};
use core::marker::PhantomData;

use num_traits::zero;

use super::super::dual::DSphere;
use super::Point;
use crate::{Dual, Field, Multivec, R410, Space, Euclidean};

#[derive(Copy, Clone, Debug)]
pub struct Sphere<T: Field, S = Euclidean> {
    pub(crate) e123p: T,
    pub(crate) e123n: T,
    pub(crate) e12pn: T,
    pub(crate) e13pn: T,
    pub(crate) e23pn: T,
    _pd: PhantomData<S>,
}

impl<T: Field + Copy, S: Space> Multivec for Sphere<T, S> {
    type Element = T;
    #[inline]
    fn into_mv(self) -> R410<T> {
        let Sphere {
            e123p,
            e123n,
            e12pn,
            e13pn,
            e23pn,
            _pd,
        } = self;
        R410 {
            e123p,
            e123n,
            e12pn,
            e13pn,
            e23pn,
            ..zero()
        }
    }

    #[inline]
    fn from_mv(v: R410<T>) -> Self {
        let R410 {
            e123p,
            e123n,
            e12pn,
            e13pn,
            e23pn,
            ..
        } = v;
        Self {
            e123p,
            e123n,
            e12pn,
            e13pn,
            e23pn,
            _pd: PhantomData,
        }
    }
}

impl<T: Field + Copy, S: Space> Sphere<T, S> {
    #[inline]
    pub fn center(self) -> Point<T, S> {
        let ni = S::infinity();
        let s = self.into_mv();
        let p = s * ni * s;
        Point::from_mv(p / (p | ni))
    }
}

impl<T: Field + Copy + Display, S: Space> Display for Sphere<T, S> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Sphere({})", self.into_mv())
    }
}

impl<T: Field + Copy + LowerExp, S: Space> LowerExp for Sphere<T, S> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Sphere({:e})", self.into_mv())
    }
}

impl<T: Field + Copy + UpperExp, S: Space> UpperExp for Sphere<T, S> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Sphere({:E})", self.into_mv())
    }
}

/// converts the sphere into its dual form of a point with radius.
impl<T: Field + Copy, S: Space> Dual for Sphere<T, S> {
    type Output = DSphere<T, S>;
    /*
    fn dual(self) -> DSphere<T> {
        DSphere {
            e1: self.e23pn,
            e2: self.e13pn,
            e3: -self.e12pn,
            ep: self.e123n,
            en: -self.e123p,
        }
    }
    */
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_center() {
        let p1 = Point::new([1.0, 0.0, 0.0]).normalize();
        let p2 = Point::new([3.0, 4.0, 5.0]).normalize();
        let p3 = Point::new([3.0, 4.0, 0.0]).normalize();

        assert_abs_diff_eq!(p1.into_sphere(1.0).undual().center(), p1);
        assert_abs_diff_eq!(p2.into_sphere(1.0).undual().center(), p2);
        assert_abs_diff_eq!(p3.into_sphere(1.0).undual().center(), p3);

        assert_abs_diff_eq!(p1.into_sphere(5.0).undual().center(), p1);
        assert_abs_diff_eq!(p2.into_sphere(5.0).undual().center(), p2);
        assert_abs_diff_eq!(p3.into_sphere(5.0).undual().center(), p3);

        assert_abs_diff_eq!(p1.into_sphere(0.2).undual().center(), p1);
        assert_abs_diff_eq!(p2.into_sphere(0.2).undual().center(), p2);
        assert_abs_diff_eq!(p3.into_sphere(0.2).undual().center(), p3);
    }
}
