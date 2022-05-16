use std::{path::Path, sync::Arc};

use crate::{
    camera::Camera,
    material::Material,
    object::Object,
    texture::Texture,
    utilities::vector3::Vector3,
};
use rand::Rng;
#[allow(dead_code)]
pub enum Scenes {
    Basic,
    BasicChecker,
    Earth,
    RectangleLight,
    CornellBox,
}

impl Scenes {
    pub fn get(&self, width: f64, height: f64) -> (Vec<Object>, Camera, Vector3<f64>) {
        match self {
            Self::Basic => {
                let mut rng = rand::thread_rng();
                let look_from = Vector3::new(13.0, 2.0, 3.0);
                let look_at = Vector3::new(0.0, 0.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 10.0;
                let aperture = 0.1;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    20.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );
                let material_ground = Material::Lambertian {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(0.5, 0.5, 0.5),
                    },
                };
                let mut objects = vec![Object::build_sphere(
                    Vector3::new(0.0, -1000.0, -1.0),
                    1000.0,
                    material_ground,
                )];

                for a in -11..11 {
                    for b in -11..11 {
                        let x = a as f64;
                        let y = b as f64;
                        let choose_material = rng.gen_range(1..100);
                        let center = Vector3::new(
                            x + 0.9 * rng.gen::<f64>(),
                            0.2,
                            y + 0.9 * rng.gen::<f64>(),
                        );

                        if choose_material < 80 {
                            // diffuse
                            let albedo = Vector3::random_vec(0.0, 1.0, &mut rng)
                                * Vector3::random_vec(0.0, 1.0, &mut rng);
                            let material = Material::Lambertian {
                                texture: Texture::SolidColor { albedo },
                            };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        } else if choose_material < 95 {
                            //metal
                            let albedo = Vector3::random_vec(0.5, 1.0, &mut rng);
                            let fuzz = rng.gen_range(0.0..0.5);
                            let material = Material::Metal { albedo, fuzz };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        } else {
                            //glass
                            let material = Material::Dielectric {
                                index_of_refraction: 1.5,
                            };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        }
                    }
                }

                let material1 = Material::Dielectric {
                    index_of_refraction: 1.5,
                };
                let material2 = Material::Lambertian {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(0.4, 0.2, 0.1),
                    },
                };
                let material3 = Material::Metal {
                    albedo: Vector3::new(0.7, 0.6, 0.5),
                    fuzz: 0.0,
                };

                objects.push(Object::build_sphere(
                    Vector3::new(0.0, 1.0, 0.0),
                    1.0,
                    material1,
                ));
                objects.push(Object::build_sphere(
                    Vector3::new(-4.0, 1.0, 0.0),
                    1.0,
                    material2,
                ));
                objects.push(Object::build_sphere(
                    Vector3::new(4.0, 1.0, 0.0),
                    1.0,
                    material3,
                ));
                (objects, camera, Vector3::new(0.5, 0.7, 1.0))
            }

            Self::BasicChecker => {
                let mut rng = rand::thread_rng();
                let look_from = Vector3::new(13.0, 2.0, 3.0);
                let look_at = Vector3::new(0.0, 0.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 10.0;
                let aperture = 0.1;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    20.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );
                let material_ground = Material::Lambertian {
                    texture: Texture::Checker {
                        color1: Vector3::new(0.2, 0.3, 0.1),
                        color2: Vector3::new(0.9, 0.9, 0.9),
                    },
                };
                let mut objects = vec![Object::build_sphere(
                    Vector3::new(0.0, -1000.0, -1.0),
                    1000.0,
                    material_ground,
                )];

                for a in -11..11 {
                    for b in -11..11 {
                        let x = a as f64;
                        let y = b as f64;
                        let choose_material = rng.gen_range(1..100);
                        let center = Vector3::new(
                            x + 0.9 * rng.gen::<f64>(),
                            0.2,
                            y + 0.9 * rng.gen::<f64>(),
                        );

                        if choose_material < 80 {
                            // diffuse
                            let albedo = Vector3::random_vec(0.0, 1.0, &mut rng)
                                * Vector3::random_vec(0.0, 1.0, &mut rng);
                            let material = Material::Lambertian {
                                texture: Texture::SolidColor { albedo },
                            };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        } else if choose_material < 95 {
                            //metal
                            let albedo = Vector3::random_vec(0.5, 1.0, &mut rng);
                            let fuzz = rng.gen_range(0.0..0.5);
                            let material = Material::Metal { albedo, fuzz };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        } else {
                            //glass
                            let material = Material::Dielectric {
                                index_of_refraction: 1.5,
                            };
                            objects.push(Object::build_sphere(center, 0.2, material));
                        }
                    }
                }

                let material1 = Material::Dielectric {
                    index_of_refraction: 1.5,
                };
                let material2 = Material::Lambertian {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(0.4, 0.2, 0.1),
                    },
                };
                let material3 = Material::Metal {
                    albedo: Vector3::new(0.7, 0.6, 0.5),
                    fuzz: 0.0,
                };

                objects.push(Object::build_sphere(
                    Vector3::new(0.0, 1.0, 0.0),
                    1.0,
                    material1,
                ));
                objects.push(Object::build_sphere(
                    Vector3::new(-4.0, 1.0, 0.0),
                    1.0,
                    material2,
                ));
                objects.push(Object::build_sphere(
                    Vector3::new(4.0, 1.0, 0.0),
                    1.0,
                    material3,
                ));
                (objects, camera, Vector3::new(0.5, 0.7, 1.0))
            }
            Self::Earth => {
                let path = Path::new("earthmap.jpg");
                let image = image::open(path)
                    .map_err(|e| format!("Failed to read image from {:?}: {}", path, e))
                    .unwrap();
                let image_v = image.as_bytes();

                let look_from = Vector3::new(13.0, 2.0, 3.0);
                let look_at = Vector3::new(0.0, 0.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 10.0;
                let aperture = 0.1;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    20.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );
                let earth_surface = Material::Lambertian {
                    texture: Texture::Image {
                        image_v: Arc::new(image_v.to_vec()),
                        width: image.width() as f64,
                        height: image.height() as f64,
                    },
                };
                let objects = vec![Object::build_sphere(
                    Vector3::new(0.0, 0.0, 0.0),
                    1.98,
                    earth_surface,
                )];

                (objects, camera, Vector3::new(0.5, 0.7, 1.0))
            }

            Self::RectangleLight => {
                let path = Path::new("marble.jpg");
                let image = image::open(path)
                    .map_err(|e| format!("Failed to read image from {:?}: {}", path, e))
                    .unwrap();
                let image_v = image.as_bytes();

                let look_from = Vector3::new(26.0, 3.0, 6.0);
                let look_at = Vector3::new(0.0, 2.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = (look_at - look_from).magnitude();
                let aperture = 0.1;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    20.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );
                let marble_material = Material::Lambertian {
                    texture: Texture::Image {
                        image_v: Arc::new(image_v.to_vec()),
                        width: image.width() as f64,
                        height: image.height() as f64,
                    },
                };
                let mut objects = vec![Object::build_sphere(
                    Vector3::new(0.0, 2.0, 0.0),
                    1.98,
                    marble_material,
                )];
                let material_ground = Material::Lambertian {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(0.65, 0.65, 0.5),
                    },
                };
                objects.push(Object::build_sphere(
                    Vector3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    material_ground,
                ));

                let diffsphere = Material::DiffuseLight {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(0.2, 0.8, 0.6),
                    },
                };
                objects.push(Object::build_sphere(
                    Vector3::new(0.5, 2.0, 4.0),
                    2.0,
                    diffsphere,
                ));

                let crystal = Material::Dielectric {
                    index_of_refraction: 1.5,
                };
                objects.push(Object::build_sphere(
                    Vector3::new(3.2, 1.0, 1.9),
                    1.0,
                    crystal,
                ));

                let difflight = Material::DiffuseLight {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(4.0, 4.0, 4.0),
                    },
                };
                objects.push(Object::build_xy_rect(
                    3.0, 5.0, 1.0, 3.0, -2.0, difflight,
                ));
                objects.push(Object::build_xy_rect(
                    -80.0,
                    100.0,
                    -10.0,
                    100.0,
                    -2.0,
                    Material::Lambertian {
                        texture: Texture::SolidColor {
                            albedo: Vector3::new(0.65, 0.65, 0.5),
                        },
                    },
                ));

                (objects, camera, Vector3::new(0.1, 0.2, 0.4))
            }

            Self::CornellBox => {
                let look_from = Vector3::new(278.0, 278.0, -800.0);
                let look_at = Vector3::new(278.0, 278.0, 0.0);
                let vup = Vector3::new(0.0, 1.0, 0.0);
                let dist_to_focus = 10.0;
                let aperture = 0.0;

                let camera = Camera::new(
                    look_from,
                    look_at,
                    vup,
                    40.0,
                    width / height,
                    aperture,
                    dist_to_focus,
                );

                let red = Material::Lambertian {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(0.65, 0.05, 0.05),
                    },
                };
                let white = Material::Lambertian {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(0.73, 0.73, 0.73),
                    },
                };
                let green = Material::Lambertian {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(0.12, 0.45, 0.15),
                    },
                };

                let difflight = Material::DiffuseLight {
                    texture: Texture::SolidColor {
                        albedo: Vector3::new(15.0, 15.0, 15.0),
                    },
                };

                let mut objects = vec![Object::build_yz_rect(
                    0.0, 555.0, 0.0, 555.0, 555.0, green,
                )];
                objects.push(Object::build_yz_rect(
                    0.0, 555.0, 0.0, 555.0, 0.0, red,
                ));
                objects.push(Object::build_xz_rect(
                    213.0, 343.0, 227.0, 332.0, 554.0, difflight,
                ));
                objects.push(Object::build_xz_rect(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    0.0,
                    white.clone(),
                ));
                objects.push(Object::build_xz_rect(
                    0.0,
                    555.0,
                    0.0,
                    555.0,
                    555.0,
                    white.clone(),
                ));
                objects.push(Object::build_xy_rect(
                    0.0, 555.0, 0.0, 555.0, 555.0, white,
                ));
                (objects, camera, Vector3::new(0.0, 0.0, 0.0))
            }
        }
    }
}
