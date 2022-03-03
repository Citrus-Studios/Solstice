use bevy::pbr::{NotShadowCaster, AlphaMode::Blend};
pub use bevy::{prelude::*};

use super::RaycastCursor;

pub fn visualizer(
    rc_query: Query<&RaycastCursor>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    let intersection_op = rc_query.single().intersection;
    let pipe_model: Handle<Mesh> = asset_server.load("models/pipes/pipe_base.obj");
    
    if intersection_op.is_some() {
        let intersection = intersection_op.unwrap();

        commands.spawn_bundle(PbrBundle {
            mesh: pipe_model,
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.0, 0.2, 1.0, 0.5),
                alpha_mode: Blend,
                ..Default::default()
            }),
            transform: Transform::from_scale(intersection.position()),
            ..Default::default()
        })
        .insert(NotShadowCaster);
    }
}