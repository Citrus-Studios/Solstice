use bevy::pbr::{NotShadowCaster, AlphaMode::Blend};
pub use bevy::{prelude::*};

use super::raycasting::BuildCursor;

#[derive(Component)]
pub struct DeleteNextFrame;

pub fn visualizer(
    mut bc_res: ResMut<BuildCursor>,
    delete_query: Query<Entity, With<DeleteNextFrame>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,

    keyboard_input: Res<Input<KeyCode>>
) {
    for entity in delete_query.iter() {
        commands.entity(entity).despawn();
    }

    if keyboard_input.pressed(KeyCode::R) {
        bc_res.rotation += 0.1;
    }

    let intersection_op = bc_res.intersection;

    let rot = bc_res.rotation;

    let pipe_model: Handle<Mesh> = asset_server.load("models/pipes/pipe_base.obj");
    
    if intersection_op.is_some() {
        let intersection = intersection_op.unwrap();
        let normal = intersection.normal().normalize();

        // my brain
        let quat = Quat::from_axis_angle(normal, rot).mul_quat(Quat::from_rotation_arc(Vec3::Y, normal));
        let translation = intersection.position() + (normal * 0.3);

        let transform_cache = Transform::from_translation(translation).with_rotation(quat);

        // Spawn pipe for deletion next frame
        commands.spawn_bundle(PbrBundle {
            mesh: pipe_model,
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.0, 0.2, 1.0, 0.5),
                alpha_mode: Blend,
                ..Default::default()
            }),
            transform: transform_cache,
            ..Default::default()
        })
        .insert(NotShadowCaster)
        .insert(DeleteNextFrame);
    }
}