use rand::prelude::ThreadRng;

use crate::{ray::Ray, utilities::vector3::Vector3};

pub struct Camera {
    pub origin: Vector3<f32>,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub lens_radius: f32,
    pub exposure: f32,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub w: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
}

impl Camera {
    pub fn new(
        lookfrom: Vector3<f32>,
        lookat: Vector3<f32>,
        vup: Vector3<f32>,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        exposure: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).norm();
        let u = Vector3::cross(vup, w).norm();
        let v = Vector3::cross(w, u);

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;

        let lens_radius = aperture / 2.0;
        Self {
            origin: lookfrom,
            viewport_height,
            viewport_width,
            lens_radius,
            exposure,
            u,
            v,
            w,
            horizontal,
            vertical,
            lower_left_corner: lookfrom - horizontal * (0.5) - vertical * 0.5 - w * focus_dist,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut ThreadRng) -> Ray {
        let rd = Vector3::random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
