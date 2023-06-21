use crate::collision::{
    sphere_plane_collision_detect, sphere_plane_collision_response, sphere_plane_distance,
};
use crate::integrate::integrate;
use crate::math::Vec3;
use crate::rigidbody::Plane;
use crate::scene::Scene;

pub fn physics_update(scene: &mut Scene, dt: f32) {
    const G: Vec3 = Vec3::new(0., -10., 0.);
    const SLEEP_VELOCITY: f32 = 0.3;
    const SLEEP_DISTANCE: f32 = 0.3;

    for sphere in scene.spheres.iter_mut() {
        let mut timestep_remaining = dt;
        let mut timestep: f32;
        let mut is_sleep = false;

        while timestep_remaining > 0.001 {
            let velocity_small_enough = sphere.state.linear_velocity.v.length() < SLEEP_VELOCITY;
            timestep = timestep_remaining;
            let mut state_new = integrate(&sphere.state, timestep, G);

            let mut cut_timestep: Option<f32> = None;
            let mut fric = 1.0;
            let mut collision_plane: Option<&Plane> = None;

            for plane in &scene.planes {
                let distance_small_enough =
                    sphere_plane_distance(sphere.radius, &sphere.state.translation, plane)
                        < SLEEP_DISTANCE;
                let force_direction_ok = G.dot(plane.normal) < -0.001;
                if velocity_small_enough && distance_small_enough && force_direction_ok {
                    is_sleep = true;
                    break;
                }
                if let Some(f) = sphere_plane_collision_detect(
                    sphere.radius,
                    &sphere.state.translation,
                    &state_new.translation,
                    plane,
                ) {
                    if f < fric {
                        fric = f;
                        cut_timestep = Some(timestep * fric);
                        collision_plane = Some(plane.clone());
                    }
                }
            }

            if let Some(plane) = collision_plane {
                state_new = integrate(&sphere.state, cut_timestep.unwrap(), G);
                state_new = sphere_plane_collision_response(&state_new, plane);
                timestep_remaining -= cut_timestep.unwrap();
            } else {
                timestep_remaining -= timestep;
            }

            if !is_sleep {
                sphere.state = state_new;
            }
        }
    }
}
