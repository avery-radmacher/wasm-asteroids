use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Add for Vec2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vec2D {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl Vec2D {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn one() -> Self {
        Self { x: 1.0, y: 0.0 }
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn len_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn len(self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn cross(self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn normalize(&self) -> Self {
        self.scale(1.0 / self.len())
    }

    pub fn rotate(&self, angle: f64) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.y * cos + self.x * sin,
        }
    }

    pub fn scale(&self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn rem_euclid_assign(&mut self, rhs: &Self) {
        self.x = self.x.rem_euclid(rhs.x);
        self.y = self.y.rem_euclid(rhs.y);
    }
}
