use std::arch::x86_64::*;

use super::vector3::Vector3;

#[inline(always)]
pub fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
#[inline(always)]
pub fn fmax(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}