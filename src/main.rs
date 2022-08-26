use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod player;
use player::{spawn_player, PlayerPlugin};

mod combine;
use combine::{spawn_box, CombinePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(setup_cameras)
        .add_startup_system(setup_lights)
        .add_plugin(PlayerPlugin)
        .add_plugin(CombinePlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Default)]
struct Game;

fn setup_lights(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(2.0, 2.0, 0.0),
        point_light: PointLight {
            intensity: 1000.,
            range: 100.,
            ..default()
        },
        ..default()
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn()
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            }),
            ..default()
        });

    spawn_player(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.5, 0.0),
    );

    spawn_box(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(3.0, 1., -3.0),
    );
    spawn_box(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 1., -3.0),
    );
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 5., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
