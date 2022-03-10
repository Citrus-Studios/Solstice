use bevy::pbr::{NotShadowCaster, AlphaMode::Blend};
pub use bevy::{prelude::*};
use bevy_rapier3d::{physics::*, prelude::*};

use crate::algorithms::distance_vec3;

use super::raycasting::BuildCursor;

#[derive(Component)]
pub struct DeleteNextFrame;

#[derive(Component)]
pub struct PipePlacement {
    pub placed: bool,
    pub transform: Option<Transform>,
}
// soidhfoisd
#[derive(Component)]
pub struct PipePreview;

pub fn visualizer(
    mut bc_res: ResMut<BuildCursor>,
    mut pp_res: ResMut<PipePlacement>,

    delete_query: Query<Entity, With<DeleteNextFrame>>,

    mut pipe_prev_query: Query<(Entity, &mut Transform), With<PipePreview>>,
    mut pipe_prev_mat_query: Query<&mut Handle<StandardMaterial>, With<PipePreview>>,
    mut pipe_prev_collider_query: Query<&mut ColliderPositionComponent, With<PipePreview>>,
    mut pipe_prev_shape_query: Query<&mut ColliderShapeComponent, With<PipePreview>>,

    narrow_phase: Res<NarrowPhase>,

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

        let pipe_cyl_mesh: Handle<Mesh> = asset_server.load("models/pipes/pipe_cylinder.obj");
        
        if mouse_input.just_pressed(MouseButton::Left) {
            if pp_res.placed {
                let first_position = pp_res.transform.unwrap().translation;
                let pipe_cyl_translation = (first_position + translation) / 2.0;
                let pipe_cyl_rotation = Quat::from_rotation_arc(Vec3::Y, (first_position - translation).normalize());
                
                let distance = distance_vec3(first_position, translation);

                let transform_c = Transform::from_translation(pipe_cyl_translation).with_rotation(pipe_cyl_rotation).with_scale(Vec3::new(1.0, distance, 1.0));

                pp_res.placed = false;

                commands.spawn_bundle(PbrBundle {
                    mesh: pipe_cyl_mesh,
                    material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                    transform: transform_c,
                    ..Default::default()
                });

                let (entity, _) = pipe_prev_query.single();

                commands.entity(entity).despawn();
            } else {
                pp_res.placed = true;
                pp_res.transform = Some(transform_cache);

                commands.spawn_bundle(PbrBundle {
                    mesh: pipe_cyl_mesh,
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(0.0, 0.2, 1.0, 0.5),
                        alpha_mode: Blend,
                        ..Default::default()
                    }),
                    transform: Transform::from_translation(translation).with_rotation(quat).with_scale(Vec3::new(1.0, 0.0, 1.0)),
                    ..Default::default()
                })
                .insert_bundle(ColliderBundle {
                    collider_type: ColliderType::Sensor.into(),
                    shape: ColliderShape::cylinder(0.0, 0.13).into(),
                    position: (translation, quat).into(),
                    ..Default::default()
                })
                .insert(PipePreview)
                .insert(NotShadowCaster);
            }
        } else {
            if pp_res.placed {
                
                let first_position = pp_res.transform.unwrap().translation;
                let pipe_cyl_translation = (first_position + translation) / 2.0;
                let pipe_cyl_rotation = Quat::from_rotation_arc(Vec3::Y, (first_position - translation).normalize());
                let distance = distance_vec3(first_position, translation);

                let transform_c = Transform::from_translation(pipe_cyl_translation).with_rotation(pipe_cyl_rotation).with_scale(Vec3::new(1.0, distance, 1.0));
                let (entity, mut transform) = pipe_prev_query.single_mut();

                let mut inter = false;
                for (_, _, c) in narrow_phase.intersections_with(entity.handle()) {
                    if c {
                        inter = true;
                        info!("yes intersection! :D");
                    } else {
                        info!("no intersection! >:(");
                    }
                }

                // let inter = narrow_phase.intersections_with(entity.handle()).into_iter().peekable().peek().is_some();

                let transform_mut = transform.as_mut();
                *transform_mut = transform_c;

                let material = materials.get_mut(pipe_prev_mat_query.single_mut().clone()).unwrap();

                if distance > 0.001 {
                    let mut collider_position_ = pipe_prev_collider_query.single_mut();
                    let collider_position = collider_position_.as_mut();

                    let mut collider_shape_ = pipe_prev_shape_query.single_mut();
                    let collider_shape = collider_shape_.make_mut();

                    collider_position.0.translation = transform_c.translation.into();
                    collider_position.0.rotation = transform_c.rotation.into();
                    
                    // I am absolutely in awe that this actually works
                    let cylinder_mut = collider_shape.as_cylinder_mut().unwrap();

                    cylinder_mut.half_height = distance / 2.0;
                    cylinder_mut.radius = 0.13;
                }

                if inter {
                    material.base_color = Color::rgba(1.0, 0.0, 0.0, 0.5);
                } else {
                    material.base_color = Color::rgba(0.0, 0.2, 1.0, 0.5);
                }
            }
        }
    }
}