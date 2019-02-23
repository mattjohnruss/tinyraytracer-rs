use crate::geometry::{Vec2, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub albedo: Vec2<f32>,
    pub diffuse_colour: Vec3<f32>,
    pub specular_exponent: f32,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            albedo: Vec2::new(1.0, 0.0),
            diffuse_colour: Self::DEFAULT_COLOUR,
            specular_exponent: 1.0,
        }
    }
}

impl Material {
    const DEFAULT_COLOUR: Vec3<f32> = Vec3 {
        x: 0.4,
        y: 0.4,
        z: 0.3,
    };

    pub fn new(albedo: Vec2<f32>, diffuse_colour: Vec3<f32>, specular_exponent: f32) -> Self {
        Material { albedo, diffuse_colour, specular_exponent }
    }
}
