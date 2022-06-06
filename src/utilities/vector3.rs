#![allow(dead_code)]

use num::{Float, Num};
use rand::{prelude::ThreadRng, Rng};
use std::{
    borrow::Borrow,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::math::{fmax, fmin};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: Num + Copy + MulAssign + DivAssign + Borrow<T>,
{
    #[inline(always)]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    #[inline(always)]
    pub fn to_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
    #[inline(always)]
    pub fn multiply_scalar(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
    #[inline(always)]
    pub fn divide_scalar(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
    #[inline(always)]
    pub fn dot(v1: Self, v2: Self) -> T {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }
    #[inline(always)]
    pub fn cross(v1: Self, v2: Self) -> Self {
        Self {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
}

impl<T> Vector3<T>
where
    T: Num + Float + DivAssign + MulAssign,
{
    #[inline(always)]
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    #[inline(always)]
    pub fn magnitude2(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    #[inline(always)]
    pub fn normalize(&mut self) -> Self {
        let mag = self.magnitude();
        if mag.is_zero() {
            return *self;
        }
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
        *self
    }
    #[inline(always)]
    pub fn norm(&self) -> Self {
        let mag = self.magnitude();
        if mag.is_zero() {
            return *self;
        }
        Vector3::new(self.x / mag, self.y / mag, self.z / mag)
    }
    pub fn limit(&mut self, max: T) {
        if self.magnitude() > max {
            self.normalize();
            *self *= max;
        }
    }
    #[inline(always)]
    pub fn get_axis(&self, axis: u8) -> T {
        match axis {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }
}

impl Vector3<f32> {
    #[inline(always)]
    pub fn min(&self, v: Self) -> Self {
        Vector3::new(fmin(self.x, v.x), fmin(self.y, v.y), fmin(self.z, v.z))
    }
    #[inline(always)]
    pub fn max(&self, v: Self) -> Self {
        Vector3::new(fmax(self.x, v.x), fmax(self.y, v.y), fmax(self.z, v.z))
    }
    #[inline(always)]
    pub fn min_axis(&self) -> f32 {
        fmin(fmin(self.x, self.y), self.z)
    }
    #[inline(always)]
    pub fn max_axis(&self) -> f32 {
        fmax(fmax(self.x, self.y), self.z)
    }
    pub fn to_rgbau8(self) -> [u8; 4] {
        [
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
            255,
        ]
    }
    #[inline(always)]
    pub fn random_vec(min: f32, max: f32, rng: &mut ThreadRng) -> Self {
        Self::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
    #[inline(always)]
    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Vector3::random_vec(-1.0, 1.0, rng);
            if p.magnitude2() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    #[inline(always)]
    pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.magnitude2() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    #[inline(always)]
    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        let mut v = Vector3::random_in_unit_sphere(rng);
        v.normalize()
    }
    #[inline(always)]
    pub fn random_in_hemisphere(normal: Vector3<f32>, rng: &mut ThreadRng) -> Self {
        let v = Vector3::random_in_unit_sphere(rng);
        if Vector3::dot(v, normal) > 0.0 {
            v
        } else {
            v * (-1.0)
        }
    }
    #[inline(always)]
    pub fn random_cosine_direction(rng: &mut ThreadRng) -> Self {
        let r1 = rng.gen::<f32>();
        let r2 = rng.gen::<f32>();
        let z = (1.0 - r2).sqrt();

        let phi = 2.0 * std::f32::consts::PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();

        Vector3::new(x, y, z)
    }
    #[inline(always)]
    pub fn random_cosine_direction_exponent(exponent: f32, rng: &mut ThreadRng) -> Self {
        let r1 = rng.gen::<f32>();
        let r2 = rng.gen::<f32>().powf(1.0 / (exponent + 1.0));
        let sin_theta = (1.0 - r2 * r2).sqrt();

        let phi = 2.0 * std::f32::consts::PI * r1;
        let x = phi.cos() * sin_theta;
        let y = phi.sin() * sin_theta;
        let z = r2;
        Vector3::new(x, y, z)
    }
    #[inline(always)]
    pub fn near_zero(&self) -> bool {
        let cutoff = 1e-8;
        (self.x.abs() < cutoff) && (self.y.abs() < cutoff) && (self.z.abs() < cutoff)
    }
    #[inline(always)]
    pub fn reflect(v: Self, n: Self) -> Self {
        v - n * (2.0 * Vector3::dot(v, n))
    }
    #[inline(always)]
    pub fn refract(v: Self, n: Self, etai_over_etat: f32) -> Self {
        let cos_theta = Vector3::dot(v * (-1.0), n).min(1.0);
        let r_out_perp = (v + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * (-1.0) * (1.0 - r_out_perp.magnitude2()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl<T> Mul for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> Add for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> AddAssign for Vector3<T>
where
    T: Num + AddAssign,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> SubAssign for Vector3<T>
where
    T: Num + SubAssign,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Num + Copy,
{
    type Output = Vector3<T>;
    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector3<T>
where
    T: Num + MulAssign + Copy,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Num + Copy,
{
    type Output = Vector3<T>;
    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector3<T>
where
    T: Num + DivAssign + Copy,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn limit_vectors() {
        let mut velocity = Vector3::new(4.0, 3.0, 1.0);
        let acceleration = Vector3::new(1.0, 1.0, 2.0);
        velocity += acceleration;
        velocity.limit(5.0);
        assert_eq!(5.0, velocity.magnitude());
    }

    #[test]

    fn try_dot_p() {
        let v1 = Vector3::new(1.0, 3.0, 1.0);
        let v2 = Vector3::new(2.0, 4.0, 1.0);
        let res = 15.0;
        assert_eq!(Vector3::dot(v1, v2), res);
    }
}
