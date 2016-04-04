#[macro_use]
extern crate glium;
extern crate gmath;

use std::f32::EPSILON;
use gmath::vector::vector3::*;

#[test]
fn add() {
    let v1 = Vector3 { x: 2.0, y:  3.0, z: -5.0 };
    let v2 = Vector3 { x: 3.0, y: -2.0, z:  1.0 };
    assert_eq!(v1 + v2, Vector3 { x: 5.0, y: 1.0, z: -4.0 });
}

#[test]
fn sub() {
    let v1 = Vector3 { x: 2.0, y:  3.0, z: -5.0 };
    let v2 = Vector3 { x: 3.0, y: -2.0, z:  1.0 };
    assert_eq!(v1 - v2, Vector3 { x: -1.0, y: 5.0, z: -6.0 });
}

#[test]
fn mul() {
    let v1 = Vector3 { x: 2.0, y:  3.0, z: -5.0 };
    assert_eq!(v1 * 3.0, Vector3 { x: 6.0, y: 9.0, z: -15.0 });
    assert_eq!(-2.0 * v1, Vector3 { x: -4.0, y: -6.0, z: 10.0 });
}

#[test]
fn div() {
    let v1 = Vector3 { x: 3.0, y:  6.0, z: -9.0 };
    assert_eq!(v1 / 3.0, Vector3 { x: 1.0, y: 2.0, z: -3.0 });
}

#[test]
fn normalize() {
    let v1 = Vector3 { x: 2.0, y: -1.0, z: -2.0 };
    assert_eq!(v1.normalize(), Vector3 { x: 2.0 / 3.0, y: -1.0 / 3.0, z: -2.0 / 3.0 });
}

#[test]
fn indexing() {
    let v1 = Vector3 { x: 2.0, y: -1.0, z: -2.0 };
    assert!( (v1[0] - 2.0).abs() < EPSILON );
}

#[test]
#[should_panic(expected = "Index out of range")]
fn indexing_fail() {
    let v1 = Vector3 { x: 2.0, y: -1.0, z: -2.0 };
    v1[3];
}

#[test]
fn uniform() {
    uniform! {v: Vector3::new(1.0, 2.0, 3.0)};
}
