use std::fmt::Debug;

use glam::{Affine3A, Vec3, Vec4, Vec4Swizzles};

use crate::{math::reflect, ray::Ray};

pub trait Shape: Debug {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn normal_at(&self, point: Vec4) -> Vec4;
    fn material(&self) -> Material;
    fn transform(&self) -> Affine3A;
}
#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a dyn Shape,
}
impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a dyn Shape) -> Self {
        Self { t, object }
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub material: Material,
    pub transform: Affine3A,
}
impl Sphere {
    pub fn new(center: Vec4, radius: f32, material: Material) -> Sphere {
        assert_eq!(center.w, 1.0);
        Sphere {
            material,
            transform: Affine3A::from_translation(center.xyz())
                * Affine3A::from_scale(Vec3::splat(radius)),
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let ray = ray.transform(&self.transform.inverse());
        let sphere_to_ray = ray.origin - Vec4::W;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.origin.dot(ray.direction);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = (b * b) - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        vec![Intersection::new(t1, self), Intersection::new(t2, self)]
    }
    fn normal_at(&self, point: Vec4) -> Vec4 {
        (point - Vec4::new(0.0, 0.0, 0.0, 1.0)).normalize()
    }
    fn material(&self) -> Material {
        self.material
    }
    fn transform(&self) -> Affine3A {
        self.transform
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Vec3,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}
impl Material {
    pub fn new(color: Vec3, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Material {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
    pub fn lighting(
        &self,
        light: &Light,
        point: Vec4,
        eye_vector: Vec4,
        normal_vector: Vec4,
    ) -> Vec3 {
        assert_eq!(light.position.w, 1.0);
        assert_eq!(eye_vector.w, 0.0);
        assert_eq!(normal_vector.w, 0.0);

        let effective_color = self.color * light.color;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normal_vector);
        let diffuse;
        let specular;
        if light_dot_normal < 0.0 {
            diffuse = Vec3::ZERO;
            specular = Vec3::ZERO;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = reflect(-lightv, normal_vector);
            let reflect_dot_eye = reflectv.dot(eye_vector);

            if reflect_dot_eye <= 0.0 {
                specular = Vec3::ZERO;
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.color * self.specular * factor;
            }
        };

        ambient + diffuse + specular
    }
}
impl Default for Material {
    fn default() -> Self {
        Self::new(Vec3::ONE, 0.1, 0.9, 0.9, 200.0)
    }
}
#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vec4,
    pub color: Vec3,
}
impl Light {
    pub fn point(position: Vec4, color: Vec3) -> Light {
        assert_eq!(position.w, 1.0);
        Light { position, color }
    }
}
