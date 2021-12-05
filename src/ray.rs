use glam::{Affine3A, Vec4, Vec4Swizzles};
#[derive(Debug)]
pub struct Ray {
    pub origin: Vec4,
    pub direction: Vec4,
}

impl Ray {
    pub fn new(origin: Vec4, direction: Vec4) -> Ray {
        assert!(direction.is_normalized());
        assert_eq!(origin.w, 1.0);
        assert_eq!(direction.w, 0.0);
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec4 {
        self.origin + t * self.direction
    }
    pub fn transform(&self, m: &Affine3A) -> Ray {
        Ray {
            origin: m.transform_point3(self.origin.xyz()).extend(1.0),
            direction: m.transform_vector3(self.direction.xyz()).extend(0.0),
        }
    }
}
