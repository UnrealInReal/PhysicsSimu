use crate::{
    math::Vec3,
    physics_state::{PhysicsState, Translation},
    rigidbody::{Plane, Sphere},
};
use bevy::{prelude::Resource, reflect::TypeUuid};

#[derive(Debug, Resource, TypeUuid)]
#[uuid = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub planes: Vec<Plane>,
}

// TODO: more complicate scene
impl Scene {
    pub fn one_sphere_at_height(height: f32) -> Self {
        Self {
            spheres: vec![Sphere {
                radius: 1.0,
                state: PhysicsState::new(Vec3::Y * height, Vec3::ZERO),
            }],
            planes: vec![Plane {
                origin: Translation { t: Vec3::ZERO },
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
