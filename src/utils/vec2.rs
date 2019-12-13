use num::{Float, Zero};
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T>(pub T, pub T);

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> Vec2<T> {
    pub fn dot(self, other: Vec2<T>) -> T {
        self.0 * other.0 + self.1 * other.1
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> Vec2<T> {
    pub fn sqr_len(self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
}

impl<T: ops::Sub<Output = T> + ops::Mul<Output = T> + Copy> Vec2<T> {
    pub fn cross(self, other: Vec2<T>) -> T {
        self.0 * other.1 - self.1 * other.0
    }
}

impl<T: Float + Copy> Vec2<T> {
    pub fn angle(self, other: Vec2<T>) -> T {
        self.cross(other).atan2(self.dot(other))
    }
}

impl<T: Zero> Vec2<T> {
    pub fn origin() -> Vec2<T> {
        Vec2(T::zero(), T::zero())
    }
}

impl<T: Zero> Default for Vec2<T> {
    fn default() -> Vec2<T> {
        Self::origin()
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::MulAssign<Vec2<T>> for Vec2<T> {
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        *self = *self * rhs;
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Vec2<T> {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::DivAssign<Vec2<T>> for Vec2<T> {
    fn div_assign(&mut self, rhs: Vec2<T>) {
        *self = *self / rhs;
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Vec2<T> {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T: ops::Add<Output = T>> ops::Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        *self = *self + rhs;
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        *self = *self - rhs;
    }
}

impl<T: ops::Neg<Output = T>> ops::Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Vec2<T> {
        Vec2(-self.0, -self.1)
    }
}
