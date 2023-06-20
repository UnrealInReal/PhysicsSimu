use crate::{components::PhysicsState, rigidbody::Plane};

pub fn sphere_plane_distance(radius: f32, state: &PhysicsState, plane: &Plane) -> f32 {
    debug_assert!(plane.normal.is_normalized());
    (state.translation.t - plane.origin).dot(plane.normal) - radius
}

pub fn sphere_plane_collision_detect(
    radius: f32,
    state_prev: &PhysicsState,
    state_next: &PhysicsState,
    plane: &Plane,
) -> Option<f32> {
    let d_prev = sphere_plane_distance(radius, state_prev, plane);
    let d_next = sphere_plane_distance(radius, state_next, plane);
    if d_prev >= 0. && d_next < 0. {
        Some(d_prev / (d_prev - d_next))
    } else {
        None
    }
}

pub fn sphere_plane_collision_response(state: &PhysicsState, _plane: &Plane) -> PhysicsState {
    PhysicsState::new(state.translation.t, -state.linear_velocity.v * 0.9)
}
