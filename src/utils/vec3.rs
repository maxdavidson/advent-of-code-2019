#![allow(dead_code)]
use num::{Float, Signed, Zero};
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: ops::Add<Output = T>> Vec3<T> {
    pub fn sum(self) -> T {
        self.0 + self.1 + self.2
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> Vec3<T> {
    pub fn dot(self, other: Vec3<T>) -> T {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> Vec3<T> {
    pub fn sqr_len(self) -> T {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
}

impl<T: ops::Sub<Output = T> + ops::Mul<Output = T> + Copy> Vec3<T> {
    pub fn cross(self, other: Vec3<T>) -> Vec3<T> {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl<T: Copy> Vec3<T> {
    fn map(self, f: impl Fn(T) -> T) -> Vec3<T> {
        Vec3(f(self.0), f(self.1), f(self.2))
    }
}

impl<T: Signed + Copy> Vec3<T> {
    pub fn signum(self) -> Vec3<T> {
        self.map(|v| v.signum())
    }

    pub fn abs(self) -> Vec3<T> {
        self.map(|v| v.abs())
    }
}

impl<T: Float + Copy> Vec3<T> {
    pub fn len(self) -> T {
        self.sqr_len().sqrt()
    }

    pub fn angle(self, other: Vec3<T>) -> T {
        self.cross(other).len().atan2(self.dot(other))
    }
}

impl<T: Zero> Vec3<T> {
    pub fn origin() -> Vec3<T> {
        Vec3(T::zero(), T::zero(), T::zero())
    }
}

impl<T: Zero> Default for Vec3<T> {
    fn default() -> Vec3<T> {
        Self::origin()
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::MulAssign<Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        *self = *self * rhs;
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Vec3<T> {
        self.map(|v| v * rhs)
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Vec3<T> {
        self.map(|v| v / rhs)
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T: ops::Add<Output = T>> ops::Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        *self = *self + rhs;
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        *self = *self - rhs;
    }
}
