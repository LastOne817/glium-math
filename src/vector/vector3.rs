extern crate glium;

use glium::uniforms::*;
use glium::vertex::*;
use glium::CapabilitiesSource;
use std::ops::{Add, Sub, Mul, Div, Index, IndexMut, Neg};
use super::vector4::*;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 0.0001 &&
        (self.y - other.y).abs() < 0.0001 &&
        (self.z - other.z).abs() < 0.0001
    }
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

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn normalize(&self) -> Vector3 {
        let len = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        *self / len
    }

    pub fn dot(a: Vector3, b: Vector3) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
        Vector3::new(
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        )
    }

    pub fn abs(&self) -> f32 {
        (&self.x.powi(2) + &self.y.powi(2) + &self.z.powi(2)).sqrt()
    }

    pub fn extend(&self) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, 1.0)
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

impl AsUniformValue for Vector3 {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec3([self[0], self[1], self[2]])
    }
}

unsafe impl Attribute for Vector3 {
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32
    }

    fn is_supported<C>(_: &C) -> bool where C: CapabilitiesSource {
        true
    }
}
