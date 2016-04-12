use super::vector::vector3::*;
use super::matrix::matrix4::*;

pub fn view_matrix(position: &Vector3, direction: &Vector3, up: &Vector3) -> Matrix4 {
    let f = direction.normalize();
    let s = Vector3::cross(*up, f);
    let s_norm = s.normalize();
    let u = Vector3::cross(f, s_norm);

    let p = -Vector3::new(
        Vector3::dot(*position, s_norm),
        Vector3::dot(*position, u),
        Vector3::dot(*position, f),
    );

    Matrix4::new(
        s[0], u[0], f[0], 0.0,
        s[1], u[1], f[1], 0.0,
        s[2], u[2], f[2], 0.0,
        p[0], p[1], p[2], 1.0,
    ).transpose()
}

pub fn perspective(width: u32, height: u32, fov: f32) -> Matrix4 {
    let aspect_ratio = height as f32 / width as f32;

    let zfar = 2048.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    Matrix4::new(
    f * aspect_ratio,   0.0,    0.0,                            0.0,
        0.0,            f,      0.0,                            0.0,
        0.0,            0.0,    (zfar+znear)/(zfar-znear),      1.0,
        0.0,            0.0,    -(2.0*zfar*znear)/(zfar-znear), 0.0,
    ).transpose()
}
