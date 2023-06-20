use std::ops::Deref;

use crate::math::Vec3;

#[derive(Debug)]
pub struct Translation {
    pub t: Vec3,
}

impl Deref for Translation {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

#[derive(Debug)]
pub struct LinearVelocity {
    pub v: Vec3,
}

impl Deref for LinearVelocity {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

#[derive(Debug)]
pub struct PhysicsState {
    pub translation: Translation,
    pub linear_velocity: LinearVelocity,
}

impl PhysicsState {
    pub fn new(translation: Vec3, linear_velocity: Vec3) -> Self {
        Self {
            translation: Translation { t: translation },
            linear_velocity: LinearVelocity { v: linear_velocity },
        }
    }
}
