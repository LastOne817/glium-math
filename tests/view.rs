#[macro_use]
extern crate gmath;

use gmath::view::*;
use gmath::vector::vector3::*;
use gmath::matrix::matrix4::*;

#[test]
fn view() {
    let view = view_matrix(&Vector3::new(15.0, 10.0, 0.0), &Vector3::new(-15.0, -10.0, 0.0), &Vector3::new(0.0, 1.0, 0.0));
    assert_eq!(view, Matrix4::new(0.0,         -0.5547002, -0.8320503,  0.0,
                                  0.0,          0.8320503, -0.5547002,  0.0,
                                  0.8320503,    0.0,        0.0,        0.0,
                                  0.0,          0.0,        18.027756,  1.0));
}
