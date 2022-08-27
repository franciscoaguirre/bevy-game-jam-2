use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::combine::CombinableObject;

#[derive(Component, Debug, Default)]
pub struct Player {
    pub entity_to_combine: Option<Entity>,
    pub combined_with: Option<CombinableObject>,
}

#[derive(Component)]
struct GroundCheck;

#[derive(Component)]
struct Grounded(bool);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_player)
            .add_system(jump_player)
            .add_system(reset_is_grounded);
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) -> Entity {
    let cube_size = 1.0;

    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            cube_size / 2.,
            cube_size / 2.,
            cube_size / 2.,
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(CollisionGroups::new(0b0001, 0b0010))
        .insert(GravityScale(5.0))
        .insert(Player::default())
        .insert(Grounded(true))
        .insert_bundle(PbrBundle {
            transform: Transform {
                translation: position,
                scale: Vec3::ONE,
                ..default()
            },
            mesh: meshes.add(Mesh::from(shape::Cube { size: cube_size })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            ..default()
        })
        .insert(Velocity::default())
        .insert(ExternalImpulse::default())
        .with_children(|children| {
            children
                .spawn()
                .insert(Collider::ball(cube_size / 10.))
                .insert(Sensor)
                .insert(GroundCheck)
                .insert(CollisionGroups::new(0b0001, 0b0010))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert_bundle(TransformBundle::from(Transform::from_xyz(
                    0.0,
                    -cube_size / 2.,
                    0.0,
                )));
        })
        .id()
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut velocities: Query<&mut Velocity, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    let speed = 10.;
    for mut velocity in velocities.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        let normalized_velocity = direction.normalize_or_zero() * speed;

        velocity.linvel.x = normalized_velocity.x;
        velocity.linvel.z = normalized_velocity.z;
    }
}

fn jump_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Grounded), With<Player>>,
) {
    let jump_strength = 10.;
    for (mut velocity, mut is_grounded) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && is_grounded.0 {
            is_grounded.0 = false;
            velocity.linvel.y = jump_strength;
        }
    }
}

fn reset_is_grounded(
    parents: Query<&Parent, With<GroundCheck>>,
    mut query: Query<&mut Grounded, With<Player>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(_, ground_check_entity, _) = event {
            if let Ok(parent) = parents.get(*ground_check_entity) {
                if let Ok(mut is_grounded) = query.get_mut(parent.get()) {
                    is_grounded.0 = true;
                }
            }
        }
    }
}
