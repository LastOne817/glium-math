#[macro_use]
extern crate glium;
extern crate obj;

use obj::*;
use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};

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

impl Vector3 {
    pub fn normalize(&self) -> Vector3 {
        let len = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        *self / len
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

    pub fn transform(x: f32, y: f32, z: f32) -> Matrix4 {
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

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + other.w * self.x + self.y * other.z - self.z * other.y,
            y: self.w * other.y + other.w * self.y + self.z * other.x - self.x * other.z,
            z: self.w * other.z + other.w * self.z + self.x * other.y - self.y * other.x,
        }
    }
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Quaternion {
        Quaternion { w: w, x: x, y: y, z: z }
    }

    pub fn from_vector(x: f32, y: f32, z: f32) -> Quaternion {
        let theta = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();

        Quaternion { w: theta, x: x / theta, y: y / theta, z: z / theta }
    }

    pub fn one() -> Quaternion {
        Quaternion { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn inverse(&self) -> Quaternion {
        let square_sum = &self.w.powi(2) + &self.x.powi(2) + &self.y.powi(2) + &self.z.powi(2);
        Quaternion { w: - &self.w / square_sum, x: &self.x / square_sum, y: &self.y / square_sum, z: &self.z / square_sum }
    }

    pub fn from_angle_axis(theta: f32, v: Vector3) -> Quaternion {
        Quaternion { w: theta, x: v.x, y: v.y, z: v.z }
    }

    pub fn rotate_angle_axis(theta: f32, axis: Vector3) -> Quaternion {
        let p = Quaternion::new(0.0, axis.x, axis.y, axis.z);
        let q = Quaternion::from_angle_axis((theta / 2.0).cos(), (theta / 2.0).sin() * axis.normalize());
        q * p * q.inverse()
    }

    pub fn to_matrix(&self) -> Matrix4 {
        let (w, x, y, z) = (&self.w, &self.x, &self.y, &self.z);
        Matrix4::new(
            1.0 - 2.0 * y.powi(2) - 2.0 * z.powi(2), 2.0 * x * y - 2.0 * w * z, 2.0 * x * z + 2.0 * w * y, 0.0,
            2.0 * x * y + 2.0 * w * z, 1.0 - 2.0 * x.powi(2) - 2.0 * z.powi(2), 2.0 * y * z - 2.0 * w * x, 0.0,
            2.0 * x * z - 2.0 * w * y, 2.0 * y * z + 2.0 * w * x, 1.0 - 2.0 * x.powi(2) - 2.0 * y.powi(2), 0.0,
            0.0, 0.0, 0.0, 1.0)
    }
}
