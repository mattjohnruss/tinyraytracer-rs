mod geometry;
mod materials;

use geometry::{Ray, Sphere, Vec3, dot};
use materials::Material;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;
const FOV: f32 = std::f32::consts::PI / 3.0;

const BACKGROUND_COLOUR: Vec3<f32> = Vec3 {
    x: 0.2,
    y: 0.7,
    z: 0.8,
};

type FrameBuffer = Vec<Vec3<f32>>;

struct Light {
    position: Vec3<f32>,
    intensity: f32,
}

impl Light {
    fn new(position: Vec3<f32>, intensity: f32) -> Self {
        Light {
            position,
            intensity,
        }
    }
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn clamp_to_u8(x: f32, min: f32, max: f32) -> u8 {
    (255.0 * clamp(x, min, max)) as u8
}

fn save_framebuffer(framebuffer: &FrameBuffer) -> Result<()> {
    let mut file = BufWriter::new(File::create("output.ppm")?);
    write!(file, "P6\n{} {}\n255\n", WIDTH, HEIGHT)?;

    for v in framebuffer {
        let pixel: [u8; 3] = [
            clamp_to_u8(v.x, 0.0, 1.0),
            clamp_to_u8(v.y, 0.0, 1.0),
            clamp_to_u8(v.z, 0.0, 1.0),
        ];

        file.write_all(&pixel)?;
    }

    Ok(())
}

fn scene_intersect(ray: &Ray, spheres: &[Sphere]) -> Option<(Vec3<f32>, Vec3<f32>, Material)> {
    let mut spheres_distance = std::f32::MAX;

    let mut hit = Vec3::default();
    let mut normal = Vec3::default();
    let mut material = Material::default();

    for sphere in spheres {
        if let Some(distance) = sphere.ray_intersect(ray) {
            if distance < spheres_distance {
                spheres_distance = distance;
                hit = ray.origin + ray.direction * distance;
                normal = (hit - sphere.centre).normalise();
                material = sphere.material;
            }
        }
    }

    const MAX_DISTANCE: f32 = 1000.0;

    if spheres_distance < MAX_DISTANCE {
        Some((hit, normal, material))
    } else {
        None
    }
}

fn cast_ray(ray: &Ray, spheres: &[Sphere], lights: &[Light]) -> Vec3<f32> {
    if let Some((point, normal, material)) = scene_intersect(ray, spheres) {
        let mut intensity = 0.0;
        for light in lights {
            let light_direction = (light.position - point).normalise();
            intensity += light.intensity * 0.0f32.max(dot(&light_direction, &normal));
        }

        material.diffuse_colour * intensity
    } else {
        BACKGROUND_COLOUR
    }
}

fn render(spheres: &[Sphere], lights: &[Light]) -> Result<()> {
    let mut framebuffer = Vec::with_capacity((WIDTH * HEIGHT) as usize);

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let (i, j) = (i as f32, j as f32);
            let (w, h) = (WIDTH as f32, HEIGHT as f32);
            let x = (2.0 * (i + 0.5) / w - 1.0) * (FOV / 2.0).tan() * w / h;
            let y = -(2.0 * (j + 0.5) / h - 1.0) * (FOV / 2.0).tan();

            let origin = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let direction = Vec3 { x, y, z: -1.0 }.normalise();
            let ray = Ray { origin, direction };

            framebuffer.push(cast_ray(&ray, spheres, lights));
        }
    }

    save_framebuffer(&framebuffer)?;
    Ok(())
}

fn main() -> Result<()> {
    let ivory = Material::new(Vec3::new(0.4, 0.4, 0.3));
    let red_rubber = Material::new(Vec3::new(0.3, 0.1, 0.1));

    let spheres = vec![
        Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0, ivory),
        Sphere::new(Vec3::new(-1.0, -1.5, -12.0), 2.0, red_rubber),
        Sphere::new(Vec3::new(1.5, -0.5, -18.0), 3.0, red_rubber),
        Sphere::new(Vec3::new(7.0, 5.0, -18.0), 4.0, ivory),
    ];

    let lights = vec![
        Light::new(Vec3::new(-20.0, 20.0, 20.0), 1.5),
    ];

    render(&spheres, &lights)?;
    Ok(())
}
