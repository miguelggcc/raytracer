use std::sync::Arc;

use image::Rgb;
use num::clamp;

use crate::utilities::vector3::Vector3;

#[allow(dead_code)]
#[derive(Clone)]
pub enum Texture {
    SolidColor {
        albedo: Vector3<f32>,
    },
    Checker {
        color1: Vector3<f32>,
        color2: Vector3<f32>,
    },
    Image {
        image_v: Arc<Vec<u8>>,
        width: f32,
        height: f32,
    },
    Hdri {
        image_v: Arc<Vec<Rgb<f32>>>,
        width: f32,
        height: f32,
    },
}

impl Texture {
    #[inline(always)]
    pub fn value(&self, u: f32, v: f32, p: Vector3<f32>) -> Vector3<f32> {
        match self {
            Self::SolidColor { albedo } => *albedo,
            Self::Checker { color1, color2 } => {
                let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
                if sines < 0.0 {
                    *color1
                } else {
                    *color2
                }
            }
            Self::Image {
                image_v,
                width,
                height,
            } => {
                if image_v.is_empty() {
                    return Vector3::new(1.0, 0.0, 1.0);
                }
                let u = clamp(u, 0.0, 1.0);
                let v = 1.0 - clamp(v, 0.0, 1.0);
                let w = *width;
                let h = *height;
                let mut i = (u * w) as usize;
                let mut j = (v * h) as usize;

                if i >= w as usize {
                    i = w as usize - 1
                }
                if j >= h as usize {
                    j = h as usize - 1
                }

                let pixel = &image_v[(i + j * w as usize) * 3..(i + j * w as usize) * 3 + 3];
                Vector3::new(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                )
            }
            Self::Hdri {
                image_v,
                width,
                height,
            } => {
                if image_v.is_empty() {
                    return Vector3::new(1.0, 0.0, 1.0);
                }
                let u = clamp(u, 0.0, 1.0);
                let v = 1.0 - clamp(v, 0.0, 1.0);

                let w = *width as usize;
                let h = *height as usize;
                let i = ((u * width) as usize).min(w - 1);
                let j = ((v * height) as usize).min(h - 1);

                let pixel = image_v[(i + j * w)];
                Vector3::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32)
            }
        }
    }
}
