use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }

    pub fn norm(&self) -> f32 {
        let Self { x, y, z } = *self;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn normalize(&mut self) -> Self {
        *self = (*self) * (1. / self.norm());
        *self
    }
}

impl Add for Vec3f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3f::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3f {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3f::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vec3f::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;
    fn mul(self, rhs: Vec3f) -> Vec3f {
        rhs * self
    }
}

impl Mul for Vec3f {
    type Output = f32;
    fn mul(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
