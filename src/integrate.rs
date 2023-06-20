use super::physics_state::*;
use crate::math::Vec3;

pub fn integrate(state: &PhysicsState, dt: f32, a: Vec3) -> PhysicsState {
    let previous_v = state.linear_velocity.v;
    let next_v = previous_v + a * dt;
    let next_t = state.translation.t + (previous_v + previous_v) * dt / 2.;
    PhysicsState::new(next_t, next_v)
}
