use glam::{Vec3, Vec4};

use crate::shapes::{Light, Material, Shape, Sphere};

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub light: Light,
}

impl World {
    pub fn new(light: Light) -> World {
        World {
            objects: Vec::new(),
            light,
        }
    }

    pub fn add(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }
}
impl Default for World {
    fn default() -> Self {
        let mut world = World::new(Light::point(
            Vec4::new(-10.0, 10.0, -10.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
        ));
        world.add(Box::new(Sphere::new(
            Vec4::new(0.0, 0.0, -1.0, 0.0),
            0.5,
            Material {
                color: Vec3::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
        )));

        world
    }
}
