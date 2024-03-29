mod geometry;
mod materials;

use crate::geometry::{Ray, Sphere, Vec2, Vec3, dot, reflect};
use crate::materials::Material;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const NANOS_PER_SEC: u32 = 1_000_000_000;

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;
const FOV: f32 = (std::f32::consts::PI / 2.0) as u32 as f32;

const BACKGROUND_COLOUR: Vec3<f32> = Vec3 {
    x: 0.2,
    y: 0.7,
    z: 0.8,
};

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

struct State {
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
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

fn cast_ray(ray: &Ray, spheres: &[Sphere], lights: &[Light]) -> Option<Vec3<f32>> {
    if let Some((point, normal, material)) = scene_intersect(ray, spheres) {
        let mut diffuse_intensity = 0.0;
        let mut specular_intensity = 0.0;

        for light in lights {
            let light_direction = (light.position - point).normalise();
            let light_distance = (light.position - point).length();

            let shadow_origin = if dot(light_direction, normal) < 0.0 {
                point - normal*1.0e-3
            } else {
                point + normal*1.0e-3
            };

            let shadow_ray = Ray {
                origin: shadow_origin,
                direction: light_direction,
            };

            if let Some((shadow_point, _, _)) = scene_intersect(&shadow_ray, spheres) {
                if (shadow_point - shadow_origin).length() < light_distance {
                    continue;
                }
            }

            diffuse_intensity +=
                light.intensity * 0.0f32.max(dot(light_direction, normal));

            let reflection = reflect(-light_direction, normal);
            specular_intensity +=
                0.0f32.max(dot(-reflection, ray.direction))
                .powf(material.specular_exponent) * light.intensity;
        }

        Some(material.diffuse_colour * diffuse_intensity * material.albedo.x
             + Vec3::new(1.0, 1.0, 1.0) * specular_intensity * material.albedo.y)
    } else {
        None
    }
}

fn update(state: &mut State, _dt: f64) {
    //println!("dt = {}", dt);
    state.spheres[0].centre.x += 0.05;
    state.spheres[1].centre.y += 0.05;
    state.spheres[2].centre.z += 0.05;
    state.spheres[3].centre.z -= 0.05;
}

fn render(
    canvas: &mut Canvas<Window>,
    spheres: &[Sphere],
    lights: &[Light],
) -> Result<()> {
    let mut v = BACKGROUND_COLOUR;

    let max = v.x.max(v.y.max(v.z));
    if max > 1.0 {
        v = v * (1.0/max);
    }

    let pixel: [u8; 3] = [
        clamp_to_u8(v.x, 0.0, 1.0),
        clamp_to_u8(v.y, 0.0, 1.0),
        clamp_to_u8(v.z, 0.0, 1.0),
    ];

    canvas.set_draw_color(Color::RGB(pixel[0], pixel[1], pixel[2]));
    canvas.clear();

    //for j in 0..HEIGHT {
        //for i in 0..WIDTH {
    (0..WIDTH).for_each(|i| {
        (0..HEIGHT).for_each(|j| {
            let (w, h) = (WIDTH as f32, HEIGHT as f32);
            let x = (2.0 * (i as f32 + 0.5) / w - 1.0) * (FOV / 2.0).tan() * w / h;
            let y = -(2.0 * (j as f32 + 0.5) / h - 1.0) * (FOV / 2.0).tan();

            let origin = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };

            let direction = Vec3 { x, y, z: -1.0 }.normalise();
            let ray = Ray { origin, direction };

            if let Some(mut v) = cast_ray(&ray, spheres, lights) {
                let max = v.x.max(v.y.max(v.z));
                if max > 1.0 {
                    v = v * (1.0/max);
                }

                let pixel: [u8; 3] = [
                    clamp_to_u8(v.x, 0.0, 1.0),
                    clamp_to_u8(v.y, 0.0, 1.0),
                    clamp_to_u8(v.z, 0.0, 1.0),
                ];

                canvas.set_draw_color(Color::RGB(pixel[0], pixel[1], pixel[2]));
                canvas.draw_point(sdl2::rect::Point::new(i, j)).unwrap();
            }
        });
    });

    canvas.present();
    Ok(())
}

fn main() -> Result<()> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("tinyraytracer-rs", WIDTH as u32, HEIGHT as u32)
        .opengl()
        .position_centered()
        .build()?;

    let mut canvas = window
        .into_canvas()
        //.present_vsync()
        .build()?;

    let mut event_pump = sdl_context.event_pump()?;

    let ivory = Material::new(Vec2::new(0.6, 0.3), Vec3::new(0.4, 0.4, 0.3), 50.0);
    let red_rubber = Material::new(Vec2::new(0.9, 0.1), Vec3::new(0.3, 0.1, 0.1), 10.0);

    let mut state = State {
        spheres: vec![
            Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0, ivory),
            Sphere::new(Vec3::new(-1.0, -1.5, -12.0), 2.0, red_rubber),
            Sphere::new(Vec3::new(1.5, -0.5, -18.0), 3.0, red_rubber),
            Sphere::new(Vec3::new(7.0, 5.0, -18.0), 4.0, ivory),
        ],
        lights: vec![
            Light::new(Vec3::new(-20.0, 20.0,  20.0), 1.5),
            Light::new(Vec3::new( 30.0, 50.0, -25.0), 1.8),
            Light::new(Vec3::new( 30.0, 20.0,  30.0), 1.7),
        ]
    };

    let target_updates_per_second = 60;
    let seconds_per_update = 1.0 / target_updates_per_second as f64;

    let mut previous_time = Instant::now();
    let mut delta: f64 = 0.0;

    let mut frames: u32 = 0;
    let mut updates: u32 = 0;

    let mut timer = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // TODO implement this!
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    unimplemented!("Saving screenshot");
                },
                _ => {}
            }
        }

        let current_time = Instant::now();
        delta += current_time
            .duration_since(previous_time)
            .subsec_nanos() as f64 / (NANOS_PER_SEC as f64 * seconds_per_update);

        previous_time = current_time;

        while delta >= 1.0 {
            update(&mut state, delta);
            updates += 1;
            delta -= 1.0;
        }

        render(&mut canvas, &state.spheres, &state.lights)?;
        frames += 1;

        let timer_now = Instant::now();

        if timer_now.duration_since(timer).as_secs() >= 1 {
            timer = timer_now;
            println!("updates: {}, frames: {}", updates, frames);

            updates = 0;
            frames = 0;
        }
    }

    Ok(())
}
