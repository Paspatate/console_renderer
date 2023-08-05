use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x:T, y:T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl<T: Add<Output = T>> Add for Vector2<T>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


impl<T> Mul<T> for Vector2<T> where T: Mul<Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let other = &rhs;
        Self {
            x: self.x * *other,
            y: self.y * *other,
        }
    }
}
