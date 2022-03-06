use bevy::pbr::{NotShadowCaster, AlphaMode::Blend};
pub use bevy::{prelude::*};

use crate::algorithms::distance_vec3;

use super::raycasting::BuildCursor;

#[derive(Component)]
pub struct DeleteNextFrame;

#[derive(Component)]
pub struct PipePlacement {
    pub placed: bool,
    pub transform: Option<Transform>,
}

pub fn visualizer(
    mut bc_res: ResMut<BuildCursor>,
    mut pp_res: ResMut<PipePlacement>,

    delete_query: Query<Entity, With<DeleteNextFrame>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,

    mouse_input: Res<Input<MouseButton>>,
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

        if pp_res.placed {
            
        }
        
        if mouse_input.just_pressed(MouseButton::Left) {
            if pp_res.placed {
                pp_res.placed = false;
                let first_position = pp_res.transform.unwrap().translation;
                let pipe_cyl_translation = (first_position + translation) / 2.0;
                let pipe_cyl_rotation = Quat::from_rotation_arc(Vec3::Y, (first_position - translation).normalize());
                let pipe_cyl_mesh: Handle<Mesh> = asset_server.load("models/pipes/pipe_cylinder.obj");
                let distance = distance_vec3(first_position, translation);

                commands.spawn_bundle(PbrBundle {
                    mesh: pipe_cyl_mesh,
                    material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                    transform: Transform::from_translation(pipe_cyl_translation).with_rotation(pipe_cyl_rotation).with_scale(Vec3::new(1.0, distance, 1.0)),
                    ..Default::default()
                });
            } else {
                pp_res.placed = true;
                pp_res.transform = Some(transform_cache);
            }
        } else {
            if pp_res.placed {
                let first_position = pp_res.transform.unwrap().translation;
                let pipe_cyl_translation = (first_position + translation) / 2.0;
                let pipe_cyl_rotation = Quat::from_rotation_arc(Vec3::Y, (first_position - translation).normalize());
                let pipe_cyl_mesh: Handle<Mesh> = asset_server.load("models/pipes/pipe_cylinder.obj");
                let distance = distance_vec3(first_position, translation);

                commands.spawn_bundle(PbrBundle {
                    mesh: pipe_cyl_mesh,
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(0.0, 0.2, 1.0, 0.5),
                        alpha_mode: Blend,
                        ..Default::default()
                    }),
                    transform: Transform::from_translation(pipe_cyl_translation).with_rotation(pipe_cyl_rotation).with_scale(Vec3::new(1.0, distance, 1.0)),
                    ..Default::default()
                })
                .insert(DeleteNextFrame)
                .insert(NotShadowCaster);
            }
        }
    }
}