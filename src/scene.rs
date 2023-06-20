use crate::{
    components::PhysicsState,
    rigidbody::{Plane, Sphere},
};
use glam::Vec3;

#[derive(Debug)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
}

impl Scene {
    pub fn one_sphere_at_100_height() -> Self {
        Self {
            spheres: vec![Sphere {
                radius: 1.0,
                state: PhysicsState::new(Vec3::Y * 100., Vec3::ZERO),
            }],
            planes: vec![Plane {
                origin: Vec3::ZERO,
                normal: Vec3::Y,
            }],
        }
    }

    pub fn display(&self) {
        println!("{self:?}");
    }

    pub fn display_first(&self) {
        println!("{:?}", self.spheres.first().unwrap().state)
    }
}
