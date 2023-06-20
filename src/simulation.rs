use crate::collision::{sphere_plane_collision_detect, sphere_plane_collision_response};
use crate::integrate::integrate;
use crate::math::Vec3;
use crate::rigidbody::Plane;
use crate::scene::Scene;

pub fn physics_update(scene: &mut Scene, dt: f32) {
    const G: Vec3 = Vec3::new(0., -10., 0.);

    for sphere in scene.spheres.iter_mut() {
        let mut timestep_remaining = dt;
        let mut timestep: f32;

        while timestep_remaining > 0.001 {
            timestep = timestep_remaining;
            let mut state_new = integrate(&sphere.state, timestep, G);

            let mut cut_timestep: Option<f32> = None;
            let mut fric = 1.0;
            let mut collision_plane: Option<&Plane> = None;

            for plane in &scene.planes {
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
            }

            timestep_remaining -= timestep;
            sphere.state = state_new;
        }
    }
}
