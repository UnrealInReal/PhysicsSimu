use super::components::*;

pub fn integrate(state: &mut PhysicsState, dt: f32, a: glam::Vec3) {
    let previous_v = state.linear_velocity.v;
    state.linear_velocity.v += a * dt;
    state.translation.t += (state.linear_velocity.v + previous_v) * dt / 2.;
}
