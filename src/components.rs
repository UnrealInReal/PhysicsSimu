use glam::Vec3;

#[derive(Debug)]
pub struct Translation {
    pub t: Vec3,
}

#[derive(Debug)]
pub struct LinearVelocity {
    pub v: Vec3,
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
