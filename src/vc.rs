use std::ops::{Add, Div, Mul, Sub};
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    pub fn pow2(&self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        self.x * self.x + self.y * self.y
    }

    pub fn dot(&self, v: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy,
    {
        self.x * v.x + self.y * v.y
    }

    pub fn cross(&self, v: Self) -> T
    where
        T: Sub<Output = T> + Mul<Output = T> + Copy,
    {
        self.x * v.y - self.y * v.x
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Div> Div<T> for Vec2<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
