use bevy::prelude::*;
use physics_simu::{scene::Scene, simulation::physics_update};

const FIXED_TIMESTEP: f32 = 1. / 60.;

fn main() {
    println!("Hello, physics!");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scene::complicate_scene())
        .add_startup_system(setup)
        .add_system(update_physics.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
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
    for (i, sphere) in scene.spheres.iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: sphere.radius,
                    ..default()
                })),
                material: materials.add(
                    Color::rgb(
                        0.1 + sphere.radius / 2.0,
                        0.2 + sphere.radius / 3.0,
                        0.9 - sphere.radius / 4.0,
                    )
                    .into(),
                ),
                transform: Transform::from_translation(
                    sphere.state.translation.t.to_array().into(),
                ),
                ..default()
            },
            PhysicsSphere { index: i },
        ));
    }

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5., 10.0, 5.),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-10., 10.0, 10.),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-15.0, 12., 30.).looking_at(Vec3::Y * 7., Vec3::Y),
        ..default()
    });
}

fn update_physics(
    fixed_time: Res<FixedTime>,
    mut scene: ResMut<Scene>,
    mut query: Query<(&mut Transform, &PhysicsSphere)>,
) {
    physics_update(&mut scene, fixed_time.period.as_secs_f32());
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
