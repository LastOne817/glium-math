use std::ops::Mul;
use super::vector::vector3::*;
use super::matrix::matrix4::*;

#[derive(Copy, Clone)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
        Quaternion::from_angle_axis((theta / 2.0).cos(), (theta / 2.0).sin() * axis.normalize())
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

