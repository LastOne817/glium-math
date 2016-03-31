use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}


impl Index<u8> for Vector4 {
    type Output = f32;

    fn index<'a>(&'a self, _index: u8) -> &'a f32 {
        match _index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<u8> for Vector4 {
    fn index_mut<'a>(&'a mut self, _index: u8) -> &'a mut f32 {
        match _index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of range"),
        }
    }
}

impl Add for Vector4 {
    type Output = Vector4;

    fn add(self, other: Vector4) -> Vector4 {
        Vector4 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: self.w + other.w }
    }
}

impl Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, other: Vector4) -> Vector4 {
        Vector4 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: self.w - other.w }
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, other: f32) -> Vector4 {
        Vector4 { x: other * self.x, y: other * self.y, z: other * self.z, w: other * self.w }
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, other: f32) -> Vector4 {
        Vector4 { x: self.x / other, y: self.y / other, z: self.z / other, w: self.w / other }
    }
}

impl Mul<Vector4> for f32 {
    type Output = Vector4;

    fn mul(self, other: Vector4) -> Vector4 {
        Vector4 { x: self * other.x, y: self * other.y, z: self * other.z, w: self * other.w }
    }
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 { x: x, y: y, z: z, w: w }
    }

    pub fn zero() -> Vector4 {
        Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub fn unit_x() -> Vector4 {
        Vector4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub fn unit_y() -> Vector4 {
        Vector4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 }
    }

    pub fn unit_z() -> Vector4 {
        Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 }
    }

    pub fn unit_w() -> Vector4 {
        Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    pub fn dot(a: Vector4, b: Vector4) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
    }
}

