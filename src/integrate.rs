use super::components::*;

pub fn integrate(state: &mut PhysicsState, dt: f32, a: glam::Vec3) {
    state.translation.t += state.linear_velocity.v * dt;
    state.linear_velocity.v += a * dt;
}
