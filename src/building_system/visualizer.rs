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
    let build_cursor_query = bc_query.get_single();

    if build_cursor_query.is_ok() {
        let intersection_op = build_cursor_query.as_ref().unwrap().intersection;
    
        let rot = build_cursor_query.unwrap().rotation;

        let pipe_model: Handle<Mesh> = asset_server.load("models/pipes/pipe_base.obj");

        for entity in delete_query.iter() {
            commands.entity(entity).despawn();
        }
        
        if intersection_op.is_some() {
            let intersection = intersection_op.unwrap();
            let normal = intersection.normal();

            let quat_vec = Vec4::new(normal.x, normal.y, normal.z, rot);

            // Spawn pipe for deletion next frame
            commands.spawn_bundle(PbrBundle {
                mesh: pipe_model,
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(0.0, 0.2, 1.0, 0.5),
                    alpha_mode: Blend,
                    ..Default::default()
                }),
                transform: Transform::from_translation(intersection.position()).with_rotation(Quat::from_vec4(quat_vec)),
                ..Default::default()
            })
            .insert(NotShadowCaster)
            .insert(DeleteNextFrame);
        }
    }
}