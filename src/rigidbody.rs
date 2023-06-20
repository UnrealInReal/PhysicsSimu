use crate::math::Vec3;
use crate::physics_state::{PhysicsState, Translation};

#[derive(Debug)]
pub struct Sphere {
    pub radius: f32,
    pub state: PhysicsState,
}

#[derive(Debug)]
pub struct Plane {
    pub origin: Translation,
    pub normal: Vec3,
}
