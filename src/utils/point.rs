use num::{Num, Signed, Zero};
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<T: Num>(pub T, pub T);

impl<T: Num + Zero> Default for Point<T> {
    fn default() -> Point<T> {
        Point(T::zero(), T::zero())
    }
}

impl<T: Num> ops::Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Point<T> {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Num + Copy> ops::AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Point<T>) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
    }
}

impl<T: Num + Copy> ops::Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Point<T> {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Num + Signed> ops::Neg for Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Point<T> {
        Point(-self.0, -self.1)
    }
}
