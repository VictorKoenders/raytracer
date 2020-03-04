#![feature(test)]
#[cfg(test)]
extern crate test;

extern crate rayon;

mod color;
mod math;
mod ray;
mod scene;

pub use crate::color::Color;
pub use crate::math::Vector3;
pub use crate::ray::{Hit, Ray};
pub use crate::scene::{Object, Scene, Sphere};

fn main() {
    let mut scene = Scene::new(Color::black());
    scene.ambient_lights.push(Color::new(0.1, 0.1, 0.1));
    scene
        .directional_lights
        .push((Vector3::new(0.0, 0.0, 1.0), Color::white()));

    scene.add(Sphere {
        center: Vector3::new(11.0, 3.0, 0.0),
        size: 3.0,
        color: Color::red(),
    });

    scene.add(Sphere {
        center: Vector3::new(9.0, -2.5, -2.5),
        size: 3.0,
        color: Color::green(),
    });

    scene.add(Sphere {
        center: Vector3::new(10.0, -1.5, 3.5),
        size: 3.0,
        color: Color::blue(),
    });

    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = 600;

    let pixels: Vec<(u32, u32)> = (0..IMAGE_WIDTH)
        .flat_map(|x| (0..IMAGE_HEIGHT).map(move |y| (x, y)))
        .collect::<Vec<_>>();

    let pixels = pixels
        .into_iter()
        .map(|(x, y)| {
            let fx = (x as f32) / 50. - 5.5;
            let fy = (y as f32) / 50. - 5.5;
            let ray = Ray {
                start: Vector3::new(0.0, fx, fy),
                direction: Vector3::new(1.0, 0.0, 0.0),
            };
            calculate_color(&scene, ray, 10)
        })
        .collect::<Vec<_>>();

    let mut bytes = Vec::with_capacity(pixels.len() * 4);
    for pixel in pixels {
        bytes.push((pixel.r * 255.0) as u8);
        bytes.push((pixel.g * 255.0) as u8);
        bytes.push((pixel.b * 255.0) as u8);
        bytes.push(255);
    }

    let path = std::path::Path::new("out.png");
    let file = std::fs::File::create(path).unwrap();
    let w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&bytes).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    
    fn bench_size_threaded(b: &mut Bencher, width: u32, height: u32) {
        let mut scene = Scene::new(Color::black());
        scene.ambient_lights.push(Color::new(0.1, 0.1, 0.1));
        scene
            .directional_lights
            .push((Vector3::new(0.0, 0.0, 1.0), Color::white()));
        
        for _ in 0..333 {
            scene.add(Sphere {
                center: Vector3::new(11.0, 3.0, 0.0),
                size: 3.0,
                color: Color::red(),
            });

            scene.add(Sphere {
                center: Vector3::new(9.0, -2.5, -2.5),
                size: 3.0,
                color: Color::green(),
            });

            scene.add(Sphere {
                center: Vector3::new(10.0, -1.5, 3.5),
                size: 3.0,
                color: Color::blue(),
            });
        }

        let pixels: Vec<(u32, u32)> = (0..width)
            .flat_map(|x| (0..height).map(move |y| (x, y)))
            .collect::<Vec<_>>();

        b.iter(|| {
            use rayon::prelude::*;
            let pixels = pixels.clone();
            let pixels = pixels
                .into_par_iter()
                .map(|(x, y)| {
                    let fx = (x as f32) / 50. - 5.5;
                    let fy = (y as f32) / 50. - 5.5;
                    let ray = Ray {
                        start: Vector3::new(0.0, fx, fy),
                        direction: Vector3::new(1.0, 0.0, 0.0),
                    };
                    calculate_color(&scene, ray, 5)
                })
                .collect::<Vec<_>>();
            pixels
        });

    }

    #[bench]
    fn bench_800x600_threaded(b: &mut Bencher) {
        bench_size_threaded(b, 800, 600);
    }

    #[bench]
    fn bench_1920x1200_threaded(b: &mut Bencher) {
        bench_size_threaded(b, 1920, 1200);
    }

    fn bench_size(b: &mut Bencher, width: u32, height: u32) {
        let mut scene = Scene::new(Color::black());
        scene.ambient_lights.push(Color::new(0.1, 0.1, 0.1));
        scene
            .directional_lights
            .push((Vector3::new(0.0, 0.0, 1.0), Color::white()));
        
        scene.add(Sphere {
            center: Vector3::new(11.0, 3.0, 0.0),
            size: 3.0,
            color: Color::red(),
        });

        scene.add(Sphere {
            center: Vector3::new(9.0, -2.5, -2.5),
            size: 3.0,
            color: Color::green(),
        });

        scene.add(Sphere {
            center: Vector3::new(10.0, -1.5, 3.5),
            size: 3.0,
            color: Color::blue(),
        });

        let pixels: Vec<(u32, u32)> = (0..width)
            .flat_map(|x| (0..height).map(move |y| (x, y)))
            .collect::<Vec<_>>();

        b.iter(|| {
            let pixels = pixels.clone();
            let pixels = pixels
                .into_iter()
                .map(|(x, y)| {
                    let fx = (x as f32) / 50. - 5.5;
                    let fy = (y as f32) / 50. - 5.5;
                    let ray = Ray {
                        start: Vector3::new(0.0, fx, fy),
                        direction: Vector3::new(1.0, 0.0, 0.0),
                    };
                    calculate_color(&scene, ray, 10)
                })
                .collect::<Vec<_>>();
            pixels
        });

    }

    #[bench]
    fn bench_800x600(b: &mut Bencher) {
        bench_size(b, 800, 600);
    }

    #[bench]
    fn bench_1920x1200(b: &mut Bencher) {
        bench_size(b, 1920, 1200);
    }
}

fn calculate_color(scene: &Scene, mut ray: Ray, max_bounces: usize) -> Color {
    let mut exclude = None;
    let mut color: Option<Color> = None;
    for i in 0..max_bounces {
        let hit = scene.calculate_hit(&ray, &exclude);
        if let Some(hit) = hit {
            exclude = Some(hit.object);

            let mut object_color = hit.object.color();
            for (direction, color) in &scene.directional_lights {
                let shade = direction.dot(hit.normal);
                if shade > 0.0 {
                    object_color.add_directional(*color, shade);
                }
            }

            for ambient in &scene.ambient_lights {
                object_color.add_ambient(*ambient);
            }

            color = Some(match color {
                Some(mut color) => {
                    color.change_towards(object_color, 0.1 / i as f32);
                    color
                }
                None => object_color,
            });

            ray = Ray {
                start: hit.position,
                direction: hit.out_angle(ray.direction),
            };
        } else {
            break;
        }
    }

    color.unwrap_or(scene.background_color)
}
