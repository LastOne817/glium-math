#[macro_use]
extern crate glium;
extern crate obj;

use obj::*;
use std::ops::{Add, Sub, Mul, Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Copy, Clone)]
pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Copy, Clone)]
pub struct Matrix4 {
    x: Vector4,
    y: Vector4,
    z: Vector4,
    w: Vector4,
}

#[derive(Copy, Clone)]
pub struct Quaternion {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
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

impl Index<u8> for Matrix4 {
    type Output = Vector4;

    fn index<'a>(&'a self, _index: u8) -> &'a Vector4 {
        match _index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<u8> for Matrix4 {
    fn index_mut<'a>(&'a mut self, _index: u8) -> &'a mut Vector4 {
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

impl Add for Matrix4 {
    type Output = Matrix4;

    fn add(self, other: Matrix4) -> Matrix4 {
        Matrix4 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: self.w + other.w }
    }
}

impl Sub for Matrix4 {
    type Output = Matrix4;

    fn sub(self, other: Matrix4) -> Matrix4 {
        Matrix4 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: self.w - other.w }
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut result = Matrix4::zero();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self[i][k] * other[k][j];
                }
                result[i][j] = sum;
            }
        }
        return result;
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: Vector4::dot(self.x, other),
            y: Vector4::dot(self.y, other),
            z: Vector4::dot(self.z, other),
            w: Vector4::dot(self.w, other),
        }
    }
}

impl Vector4 {
    pub fn zero() -> Vector4 {
        Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub fn dot(a: Vector4, b: Vector4) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
    }
}

impl Matrix4 {
    pub fn zero() -> Matrix4 {
        Matrix4 { x: Vector4::zero(), y: Vector4::zero(), z: Vector4::zero(), w: Vector4::zero() }
    }
}
