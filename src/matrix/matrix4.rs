extern crate glium;

use glium::uniforms::*;
use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use super::super::vector::vector4::*;

#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) &&
        (self.y == other.y) &&
        (self.z == other.z) &&
        (self.w == other.w)
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

impl Mul<f32> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: f32) -> Matrix4 {
        Matrix4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<f32> for Matrix4 {
    type Output = Matrix4;

    fn div(self, other: f32) -> Matrix4 {
        Matrix4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Matrix4 {
    pub fn zero() -> Matrix4 {
        Matrix4 { x: Vector4::zero(), y: Vector4::zero(), z: Vector4::zero(), w: Vector4::zero() }
    }

    pub fn one() -> Matrix4 {
        Matrix4 { x: Vector4::unit_x(), y: Vector4::unit_y(), z: Vector4::unit_z(), w: Vector4::unit_w() }
    }

    pub fn new(a00: f32, a01: f32, a02: f32, a03: f32,
               a10: f32, a11: f32, a12: f32, a13: f32,
               a20: f32, a21: f32, a22: f32, a23: f32,
               a30: f32, a31: f32, a32: f32, a33: f32) -> Matrix4 {
        Matrix4 {
            x: Vector4::new(a00, a01, a02, a03),
            y: Vector4::new(a10, a11, a12, a13),
            z: Vector4::new(a20, a21, a22, a23),
            w: Vector4::new(a30, a31, a32, a33),
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4::new(
            1.0, 0.0, 0.0,  x ,
            0.0, 1.0, 0.0,  y ,
            0.0, 0.0, 1.0,  z ,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4::new(
             x , 0.0, 0.0, 0.0,
            0.0,  y , 0.0, 0.0,
            0.0, 0.0,  z , 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn transpose(&self) -> Matrix4 {
        Matrix4::new(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3],
        )
    }
}

impl AsUniformValue for Matrix4 {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Mat4(
            [
                [self[0][0], self[1][0], self[2][0], self[3][0]],
                [self[0][1], self[1][1], self[2][1], self[3][1]],
                [self[0][2], self[1][2], self[2][2], self[3][2]],
                [self[0][3], self[1][3], self[2][3], self[3][3]],
            ]
        )
    }
}
