use crate::integrate::integrate;
use crate::scene::Scene;

pub fn update(scene: &mut Scene, dt: f32) {
    const G: glam::Vec3 = glam::vec3(0., -10., 0.);

    scene.display_first();
    for sphere in scene.spheres.iter_mut() {
        integrate(&mut sphere.state, dt, G)
    }
}
