use num::{Float, Zero};
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<T>(pub T, pub T);

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> Point<T> {
    pub fn dot(self, other: Point<T>) -> T {
        self.0 * other.0 + self.1 * other.1
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> Point<T> {
    pub fn len_sqr(self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
}

impl<T: ops::Sub<Output = T> + ops::Mul<Output = T> + Copy> Point<T> {
    pub fn cross(self, other: Point<T>) -> T {
        self.0 * other.1 - self.1 * other.0
    }
}

impl<T: Float + Copy> Point<T> {
    pub fn angle(self, other: Point<T>) -> T {
        self.cross(other).atan2(self.dot(other))
    }
}

impl<T: Zero> Point<T> {
    pub fn origin() -> Point<T> {
        Point(T::zero(), T::zero())
    }
}

impl<T: Zero> Default for Point<T> {
    fn default() -> Point<T> {
        Self::origin()
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Point<T> {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::MulAssign<T> for Point<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: T) -> Point<T> {
        Point(self.0 / rhs, self.1 / rhs)
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::DivAssign<T> for Point<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T: ops::Add<Output = T>> ops::Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Point<T> {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Point<T>) {
        *self = *self + rhs;
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Point<T> {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Point<T>) {
        *self = *self - rhs;
    }
}

impl<T: ops::Neg<Output = T>> ops::Neg for Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Point<T> {
        Point(-self.0, -self.1)
    }
}
