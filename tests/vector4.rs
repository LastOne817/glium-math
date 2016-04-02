extern crate gmath;

use std::f32::EPSILON;
use gmath::vector::vector4::*;

#[test]
fn add() {
    let v1 = Vector4::new( 1.2, -3.0, -4.2, 2.7);
    let v2 = Vector4::new(-3.4,  5.2, -1.0, 0.8);
    assert_eq!(v1 + v2, Vector4::new(-2.2, 2.2, -5.2, 3.5));
}

#[test]
fn sub() {
    let v1 = Vector4::new( 1.2, -3.0, -4.2, 2.7);
    let v2 = Vector4::new(-3.4,  5.2, -1.0, 0.8);
    assert_eq!(v1 - v2, Vector4::new(4.6, -8.2, -3.2, 1.9));
}

#[test]
fn mul() {
    let v1 = Vector4::new( 1.2, -3.0, -4.2, 2.7);
    assert_eq!(v1 * 2.0, Vector4::new(2.4, -6.0, -8.4, 5.4));
    assert_eq!(-3.0 * v1, Vector4::new(-3.6, 9.0, 12.6, -8.1));
}

#[test]
fn div() {
    let v1 = Vector4::new( 1.2, -3.0, -4.2, 2.7);
    assert_eq!(v1 / 2.0, Vector4::new(0.6, -1.5, -2.1, 1.35));
}

#[test]
fn dot() {
    let v1 = Vector4::new( 1.0, -3.0, -4.0, 2.0);
    let v2 = Vector4::new(-3.0,  5.0, -1.0, 0.0);
    assert!((Vector4::dot(v1, v2) + 14.0) < EPSILON);
}

#[test]
fn indexing() {
    let v1 = Vector4::new( 1.2, -3.0, -4.2, 2.7);
    assert!((v1[3] - 2.7) < EPSILON);
}

#[test]
#[should_panic(expected = "Index out of range")]
fn indexing_fail() {
    let v1 = Vector4::new( 1.2, -3.0, -4.2, 2.7);
    assert!((v1[4] - 2.7) < EPSILON);
}
