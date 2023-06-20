use glam::Vec3;

use crate::components::PhysicsState;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Sphere {
    pub radius: f32,
    pub state: PhysicsState,
}

#[derive(Debug, Component)]
pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
}
