use std::f32::consts::PI;

use bevy::{pbr::{NotShadowCaster, AlphaMode::Blend}, input::mouse::MouseWheel};
pub use bevy::{prelude::*};
use bevy_rapier3d::{physics::*, prelude::*};

use crate::{algorithms::distance_vec3, player_system::gui_system::gui_startup::{GuiButtonId, SelectedBuilding}, constants::HALF_PI};

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

#[derive(Component)]
pub struct TestComponent;

pub fn visualizer(
    mut bc_res: ResMut<BuildCursor>,
    mut pp_res: ResMut<PipePlacement>,

    delete_query: Query<Entity, With<DeleteNextFrame>>,
    test_query: Query<Entity, With<TestComponent>>,

    mut pipe_prev_query: Query<(Entity, &mut Transform), With<PipePreview>>,
    mut pipe_prev_mat_query: Query<&mut Handle<StandardMaterial>, With<PipePreview>>,
    mut pipe_prev_collider_query: Query<&mut ColliderPositionComponent, With<PipePreview>>,
    mut pipe_prev_shape_query: Query<&mut ColliderShapeComponent, With<PipePreview>>,

    narrow_phase: Res<NarrowPhase>,

    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,

    gui_hover_query: Query<&Interaction, With<GuiButtonId>>,

    (mouse_input, keyboard_input): (Res<Input<MouseButton>>, Res<Input<KeyCode>>),
    mut mouse_scroll_event: EventReader<MouseWheel>,

    mut selected_building: ResMut<SelectedBuilding>,
) {
    for entity in delete_query.iter() {
        commands.entity(entity).despawn();
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        for event in mouse_scroll_event.iter() {
            bc_res.rotation += event.y * (PI/16.0);
        }
    }

    if keyboard_input.just_pressed(KeyCode::R) {
        bc_res.rotation += HALF_PI;
    }

    let intersection_op = bc_res.intersection;

    let rot = bc_res.rotation;
    
    if intersection_op.is_some() && selected_building.id.is_some() {
        let intersection = intersection_op.unwrap();
        let normal = intersection.normal().normalize();

        // my brain
        let quat = Quat::from_axis_angle(normal, rot).mul_quat(Quat::from_rotation_arc(Vec3::Y, normal));
        let mut translation = intersection.position();

        let mut transform_cache = Transform::from_translation(translation).with_rotation(quat);

        let building_id = selected_building.id.as_ref().unwrap();

        let mut hovered = false;
        for interaction in gui_hover_query.iter() {
            match interaction {
                Interaction::None => (),
                _ => hovered = true
            }
        }

        let entity_op = pipe_prev_query.get_single();
        let pipe_prev_entity_op = match entity_op {
            Ok(e) => Some(e.0),
            Err(_) => None,
        };

        match building_id.as_str() {
            "Well Pump" => {
                let scene: Handle<Scene> = asset_server.load("models/buildings/well_pump.gltf#Scene0");
                let mesh: Handle<Mesh> = asset_server.load("models/buildings/well_pump.obj");
                spawn_cursor_bp(&mut commands, &mut materials, mesh.clone(), transform_cache);
                if mouse_input.just_pressed(MouseButton::Left) && !hovered {
                    spawn_bp(&mut commands, scene.clone(), transform_cache);
                    selected_building.id = None;
                }
            },
            "Pipe" => {


                // All pipe shit vvvvv
                let pipe_cyl_mesh: Handle<Mesh> = asset_server.load("models/pipes/pipe_cylinder.obj");        
                translation += normal * 0.3;
                transform_cache.translation = translation;
        
                if mouse_input.just_pressed(MouseButton::Left) && !hovered {
                    // If you click, and the first point is already placed
                    // Place the second point

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

                        if pipe_prev_entity_op.is_some() {
                            commands.entity(pipe_prev_entity_op.unwrap()).despawn();
                        }

                        selected_building.id = None;
                    // If you click and the first point is not placed
                    // Place the first point
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
                            shape: ColliderShape::cuboid(0.135, 0.0, 0.135).into(),
                            position: (translation, quat).into(),
                            ..Default::default()
                        })
                        .insert(PipePreview)
                        .insert(NotShadowCaster);
                    }
                // If you're not clicking
                } else {
                    // If the first point is placed
                    // Update the preview
                    if pp_res.placed {
                        
                        let first_position = pp_res.transform.unwrap().translation;
                        let pipe_cyl_translation = (first_position + translation) / 2.0;
                        let pipe_cyl_rotation = Quat::from_rotation_arc(Vec3::Y, (first_position - translation).normalize());
                        let distance = distance_vec3(first_position, translation);

                        let transform_c = Transform::from_translation(pipe_cyl_translation).with_rotation(pipe_cyl_rotation).with_scale(Vec3::new(1.0, distance, 1.0));
                        let (entity, mut transform) = pipe_prev_query.single_mut();

                        // let test_entity = test_query.iter().last();
                        // match test_entity {
                        //     Some(e) => entity = e,
                        //     _ => (),
                        // }


                        let mut inter = false;
                        for (_, _, c) in narrow_phase.intersections_with(entity.handle()) {
                            if c {
                                inter = true;
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
                            let cylinder_mut = collider_shape.as_cuboid_mut().unwrap();

                            cylinder_mut.half_extents.data.0[0][1] = distance / 2.0;
                        }

                        if inter {
                            material.base_color = Color::rgba(1.0, 0.0, 0.0, 0.5);
                        } else {
                            material.base_color = Color::rgba(0.0, 0.2, 1.0, 0.5);
                        }
                    } // If the first isn't placed and you're not clicking, do nothing
                }

                let pipe_model: Handle<Mesh> = asset_server.load("models/pipes/pipe_base.obj");
                spawn_cursor_bp(&mut commands, &mut materials, pipe_model.clone(), transform_cache);
            },
            _ => {
                if pipe_prev_entity_op.is_some() {
                    commands.entity(pipe_prev_entity_op.unwrap()).despawn();
                }
            }
        }       
    }
}

// TODO: collision
fn spawn_cursor_bp(commands: &mut Commands, materials: &mut ResMut<Assets<StandardMaterial>>, mesh: Handle<Mesh>, transform: Transform) {
    commands.spawn_bundle(PbrBundle {
        mesh,
        material: materials.add(StandardMaterial {
            base_color: Color::rgba(87.0/255.0, 202.0/255.0, 1.0, 0.5),
            alpha_mode: Blend,
            ..Default::default()
        }),
        transform,
        ..Default::default()
    })
    .insert(NotShadowCaster)
    .insert(DeleteNextFrame);
}

// TODO: everything
fn spawn_bp(commands: &mut Commands, scene: Handle<Scene>, transform: Transform) {
    commands.spawn_bundle((
        transform,
        GlobalTransform::identity()
    )).with_children(|parent| {
        let height = 1.129;
        let mut translation = transform.translation;
        translation.y += height / 2.0;
        parent.spawn_scene(scene);
        parent.spawn_bundle(ColliderBundle {
            shape: ColliderShape::cylinder(height, 0.9).into(),
            position: transform.translation.into(),
            ..Default::default()
        });
    })
    .insert(TestComponent);
    
}