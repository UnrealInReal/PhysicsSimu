use bevy::prelude::*;
use physics_simu::{scene::Scene, simulation::physics_update};

fn main() {
    println!("Hello, physics!");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scene::one_sphere_at_height(10.))
        .add_startup_system(setup)
        .add_system(update_physics)
        .run();
}

#[derive(Component)]
pub struct PhysicsSphere {
    pub index: usize,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene: Res<Scene>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(25.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_translation(Vec3::from_array(
            scene.planes.first().unwrap().origin.to_array(),
        )),
        ..default()
    });
    // sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: scene.spheres.first().unwrap().radius,
                ..default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(
                scene
                    .spheres
                    .first()
                    .unwrap()
                    .state
                    .translation
                    .t
                    .to_array()
                    .into(),
            ),
            ..default()
        },
        PhysicsSphere { index: 0 },
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-15.0, 12., 30.).looking_at(Vec3::Y * 7., Vec3::Y),
        ..default()
    });
}

fn update_physics(
    time: Res<Time>,
    mut scene: ResMut<Scene>,
    mut query: Query<(&mut Transform, &PhysicsSphere)>,
) {
    physics_update(&mut scene, time.delta_seconds()); // TODO: a fixed update for physics
    for (mut t, s) in &mut query {
        t.translation = scene
            .spheres
            .get(s.index)
            .unwrap()
            .state
            .translation
            .t
            .to_array()
            .into();
    }
}
