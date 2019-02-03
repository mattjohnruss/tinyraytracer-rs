use crate::geometry::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub diffuse_colour: Vec3<f32>,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            diffuse_colour: Self::DEFAULT_COLOUR,
        }
    }
}

impl Material {
    const DEFAULT_COLOUR: Vec3<f32> = Vec3 {
        x: 0.4,
        y: 0.4,
        z: 0.3,
    };

    pub fn new(diffuse_colour: Vec3<f32>) -> Self {
        Material { diffuse_colour }
    }
}
