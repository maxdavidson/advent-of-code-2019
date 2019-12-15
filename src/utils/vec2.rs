use num::{Float, Signed, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T>(pub T, pub T);

impl<T: Signed + Copy> Vec2<T> {
    pub fn up() -> Vec2<T> {
        Vec2(T::zero(), T::one())
    }

    pub fn down() -> Vec2<T> {
        Vec2(T::zero(), -T::one())
    }

    pub fn left() -> Vec2<T> {
        Vec2(-T::one(), T::zero())
    }

    pub fn right() -> Vec2<T> {
        Vec2(T::one(), T::zero())
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Vec2<T> {
    pub fn dot(self, other: Vec2<T>) -> T {
        self.0 * other.0 + self.1 * other.1
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Vec2<T> {
    pub fn sqr_len(self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
}

impl<T: Sub<Output = T> + Mul<Output = T> + Copy> Vec2<T> {
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

impl<T: Mul<Output = T> + Copy> Mul for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<Vec2<T>> for Vec2<T> {
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        *self = *self * rhs;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Vec2<T> {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T: Div<Output = T> + Copy> Div for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl<T: Div<Output = T> + Copy> DivAssign<Vec2<T>> for Vec2<T> {
    fn div_assign(&mut self, rhs: Vec2<T>) {
        *self = *self / rhs;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Vec2<T> {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl<T: Div<Output = T> + Copy> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        *self = *self + rhs;
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        *self = *self - rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Vec2<T> {
        Vec2(-self.0, -self.1)
    }
}
