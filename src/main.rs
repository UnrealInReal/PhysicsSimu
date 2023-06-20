use physics_simu::{scene::Scene, simulation::update};

fn main() {
    println!("Hello, physics!");

    let mut scene = Scene::one_sphere_at_100_height();

    for _ in 0..10 {
        update(&mut scene, 1.);
    }
}
