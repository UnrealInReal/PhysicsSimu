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

pub fn sphere_plane_collision_response(state: &PhysicsState, plane: &Plane) -> PhysicsState {
    let bouness = 0.9;
    let friction = 0.01;
    let v_n = state
        .linear_velocity
        .v
        .project_onto_normalized(plane.normal);
    let v_t = state.linear_velocity.v - v_n;
    let v_n_next = if v_n.length().abs() < 3. {
        0.1 * v_n
    } else {
        -bouness * v_n
    };
    let v_t_next = v_t * (1. - 1.0_f32.min(friction * v_n.length() / v_t.length()));
    PhysicsState::new(state.translation.t, v_n_next + v_t_next)
}
