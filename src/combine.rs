use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::player::Player;

#[derive(Debug)]
pub enum CombinableObject {
    Box,
}

#[derive(Component, Default)]
pub struct Combinable {
    pub highlighted: bool,
    pub original_material: Handle<StandardMaterial>,
}

pub struct CombinePlugin;

impl Plugin for CombinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighlightMaterial>()
            .add_system(combine_player)
            .add_system(highlight_combinable_entity);
    }
}

type PlayerComponents<'a> = (
    &'a mut Player,
    &'a mut Transform,
    &'a mut Handle<StandardMaterial>,
    &'a mut Handle<Mesh>,
    &'a mut Collider,
);

type CombinableEntityComponents<'a> = (
    &'a Transform,
    &'a Handle<StandardMaterial>,
    &'a Handle<Mesh>,
    &'a Collider,
);

fn combine_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<PlayerComponents, With<Player>>,
    combinables: Query<CombinableEntityComponents, (With<Combinable>, Without<Player>)>,
) {
    if keyboard_input.just_pressed(KeyCode::A) {
        for (
            mut player,
            mut player_transform,
            mut player_material,
            mut player_mesh,
            mut player_collider,
        ) in players.iter_mut()
        {
            if let Some(entity_to_combine) = player.entity_to_combine {
                if let Ok((transform, material, mesh, collider)) =
                    combinables.get(entity_to_combine)
                {
                    player.entity_to_combine = None;
                    player.combined_with = Some(CombinableObject::Box);
                    let actual_material = materials.get(material).unwrap().clone();
                    *player_material = materials.add(
                        // TODO: Make generic
                        Color::rgb(
                            actual_material.base_color.r(),
                            actual_material.base_color.g(),
                            1.0,
                        )
                        .into(),
                    );
                    *player_mesh = mesh.clone();
                    *player_transform = *transform;
                    *player_collider = collider.clone();
                    commands.entity(entity_to_combine).despawn();
                }
            }
        }
    }
}

struct HighlightMaterial {
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for HighlightMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        HighlightMaterial {
            material: materials.add(Color::YELLOW.into()),
        }
    }
}

fn highlight_combinable_entity(
    mut collision_events: EventReader<CollisionEvent>,
    mut players: Query<&mut Player>,
    mut materials: Query<&mut Handle<StandardMaterial>, With<Combinable>>,
    mut combinables: Query<&mut Combinable>,
    highlight_material: Res<HighlightMaterial>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(player_entity, combinable_entity, _) => {
                if let Ok(mut player) = players.get_mut(*player_entity) {
                    if let Ok(mut combinable) = combinables.get_mut(*combinable_entity) {
                        if let Ok(mut material) = materials.get_mut(*combinable_entity) {
                            *material = highlight_material.material.clone();
                            player.entity_to_combine = Some(*combinable_entity);
                            combinable.highlighted = true;
                        }
                    }
                }
            }
            CollisionEvent::Stopped(player_entity, combinable_entity, _) => {
                if let Ok(mut player) = players.get_mut(*player_entity) {
                    if let Ok(mut combinable) = combinables.get_mut(*combinable_entity) {
                        if let Ok(mut material) = materials.get_mut(*combinable_entity) {
                            *material = combinable.original_material.clone();
                            combinable.highlighted = false;
                            player.entity_to_combine = None;
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_box(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) -> Entity {
    let size = 2.;

    let box_material = materials.add(StandardMaterial {
        base_color: Color::PINK,
        ..default()
    });

    commands
        .spawn()
        .insert_bundle(PbrBundle {
            transform: Transform {
                translation: position,
                scale: Vec3::ONE,
                ..default()
            },
            mesh: meshes.add(Mesh::from(shape::Cube { size })),
            material: box_material.clone(),
            ..default()
        })
        .insert(Collider::cuboid(size / 2., size / 2., size / 2.))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Combinable {
            highlighted: false,
            original_material: box_material,
        })
        .id()
}
