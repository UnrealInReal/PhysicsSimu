use crate::{
    physics_state::{PhysicsState, Translation},
    rigidbody::Plane,
};

pub fn sphere_plane_distance(radius: f32, translation: &Translation, plane: &Plane) -> f32 {
    debug_assert!(plane.normal.is_normalized());
    (translation.t - plane.origin.t).dot(plane.normal) - radius
}

pub fn sphere_plane_collision_detect(
    radius: f32,
    trans_prev: &Translation,
    trans_next: &Translation,
    plane: &Plane,
) -> Option<f32> {
    let d_prev = sphere_plane_distance(radius, trans_prev, plane);
    let d_next = sphere_plane_distance(radius, trans_next, plane);
    if d_prev >= 0. && d_next < 0. {
        Some(d_prev / (d_prev - d_next))
    } else {
        None
    }
}

// TODO: better responese
pub fn sphere_plane_collision_response(state: &PhysicsState, _plane: &Plane) -> PhysicsState {
    PhysicsState::new(state.translation.t, -state.linear_velocity.v * 0.9)
}
