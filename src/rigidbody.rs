use glam::Vec3;

use crate::components::PhysicsState;

#[derive(Debug)]
pub struct Sphere {
    pub radius: f32,
    pub state: PhysicsState,
}

#[derive(Debug)]
pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
}
