use crate::collision::{sphere_plane_collision_detect, sphere_plane_collision_response};
use crate::integrate::integrate;
use crate::scene::Scene;

pub fn update(scene: &mut Scene, dt: f32) {
    const G: glam::Vec3 = glam::vec3(0., -10., 0.);

    scene.display_first();
    for sphere in scene.spheres.iter_mut() {
        let mut timestep_remaining = dt;
        let mut timestep: f32;

        while timestep_remaining > 0.001 {
            timestep = timestep_remaining;
            let mut state_new = integrate(&sphere.state, timestep, G);

            if let Some(f) = sphere_plane_collision_detect(
                sphere.radius,
                &sphere.state,
                &state_new,
                &scene.planes.first().unwrap(),
            ) {
                timestep *= f;
                state_new = integrate(&sphere.state, timestep, G);
                state_new =
                    sphere_plane_collision_response(&state_new, &scene.planes.first().unwrap());
            }

            timestep_remaining -= timestep;
            sphere.state = state_new;
        }
    }
}
