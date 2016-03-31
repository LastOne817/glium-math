use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3 {
        Vector3 { x: other * self.x, y: other * self.y, z: other * self.z }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Vector3 {
        Vector3 { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 { x: self * other.x, y: self * other.y, z: self * other.z }
    }
}

impl Vector3 {
    pub fn normalize(&self) -> Vector3 {
        let len = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        *self / len
    }
}

impl Index<u8> for Vector3 {
    type Output = f32;

    fn index<'a>(&'a self, _index: u8) -> &'a f32 {
        match _index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<u8> for Vector3 {
    fn index_mut<'a>(&'a mut self, _index: u8) -> &'a mut f32 {
        match _index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range"),
        }
    }
}

