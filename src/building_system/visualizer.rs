use bevy::pbr::{NotShadowCaster, AlphaMode::Blend};
pub use bevy::{prelude::*};

use super::{BuildCursor};

#[derive(Component)]
pub struct DeleteNextFrame;

pub fn visualizer(
    bc_query: Query<&BuildCursor>,
    delete_query: Query<Entity, With<DeleteNextFrame>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    let intersection_op = bc_query.single().intersection;
    let rot = bc_query.single().rotation;

    let pipe_model: Handle<Mesh> = asset_server.load("models/pipes/pipe_base.obj");

    for entity in delete_query.iter() {
        commands.entity(entity).despawn();
    }
    
    if intersection_op.is_some() {
        let intersection = intersection_op.unwrap();
        let normal = intersection.normal();

        // Spawn pipe for deletion next frame
        commands.spawn_bundle(PbrBundle {
            mesh: pipe_model,
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.0, 0.2, 1.0, 0.5),
                alpha_mode: Blend,
                ..Default::default()
            }),
            transform: Transform::from_translation(intersection.position()),
            ..Default::default()
        })
        .insert(NotShadowCaster)
        .insert(DeleteNextFrame);
    }
}