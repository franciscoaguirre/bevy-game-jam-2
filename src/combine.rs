use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::player::Player;

#[derive(Component, Default)]
pub struct Combinable {
    pub highlighted: bool,
    pub original_material: Handle<StandardMaterial>,
}

pub struct CombinePlugin;

impl Plugin for CombinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighlightMaterial>()
            .add_system(highlight_combinable_entity);
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
    players: Query<&Player>,
    mut materials: Query<&mut Handle<StandardMaterial>, With<Combinable>>,
    mut combinables: Query<&mut Combinable>,
    highlight_material: Res<HighlightMaterial>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(player_entity, combinable_entity, _) => {
                if players.get(*player_entity).is_ok() {
                    if let Ok(mut combinable) = combinables.get_mut(*combinable_entity) {
                        if let Ok(mut material) = materials.get_mut(*combinable_entity) {
                            *material = highlight_material.material.clone();
                            combinable.highlighted = true;
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, combinable_entity, _) => {
                if let Ok(mut combinable) = combinables.get_mut(*combinable_entity) {
                    if let Ok(mut material) = materials.get_mut(*combinable_entity) {
                        combinable.highlighted = false;
                        *material = combinable.original_material.clone();
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
