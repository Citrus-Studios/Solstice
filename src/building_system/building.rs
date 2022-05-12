use std::{f32::consts::PI, ops::Add};

use bevy::{pbr::{NotShadowCaster, AlphaMode::Blend}, input::mouse::MouseWheel, gltf::GltfMesh};
pub use bevy::{prelude::*};

use bevy_rapier3d::prelude::*;

use crate::{algorithms::distance_vec3, player_system::gui_system::gui_startup::{GuiButtonId, SelectedBuilding}, constants::HALF_PI};

use super::{raycasting::BuildCursor, buildings::{string_to_building}, MaterialHandles, building_components::*, building_functions::*};

pub fn building(
    mut commands: Commands,

    delete_query: Query<Entity, With<DeleteNextFrame>>,

    (mut pipe_prev_query, mut pipe_prev_mat_query, mut pipe_prev_shape_query): (
        Query<(Entity, &mut Transform), With<PipePreview>>, 
        Query<&mut Handle<StandardMaterial>, With<PipePreview>>, 
        Query<&mut Collider, With<PipePreview>>,
    ),
    
    rapier_context: Res<RapierContext>,
    asset_server: Res<AssetServer>,
    mut selected_building: ResMut<SelectedBuilding>,

    (mut materials, gltf_meshes, mut meshes, mut images): (
        ResMut<Assets<StandardMaterial>>, 
        ResMut<Assets<GltfMesh>>, 
        ResMut<Assets<Mesh>>, 
        ResMut<Assets<Image>>,
    ),

    mut cursor_bp_query: Query<(&mut Transform, Entity), (With<CursorBp>, Without<PipePreview>)>,
    mut cursor_bp_collider_query: Query<(Entity, &mut Moved, &mut Transform), (With<CursorBpCollider>, Without<PipePreview>, Without<CursorBp>)>,

    (mut bc_res, mut pp_res): (ResMut<BuildCursor>, ResMut<PipePlacement>),
    mut bp_material_handles: ResMut<MaterialHandles>,

    gui_hover_query: Query<&Interaction, With<GuiButtonId>>,
    (mouse_input, keyboard_input): (Res<Input<MouseButton>>, Res<Input<KeyCode>>),
    mut mouse_scroll_event: EventReader<MouseWheel>,
) {
    if bp_material_handles.blueprint.is_none() {
        bp_material_handles.blueprint = Some(materials.add(StandardMaterial {
            base_color: Color::rgba(87.0/255.0, 202.0/255.0, 1.0, 0.5),
            alpha_mode: Blend,
            ..Default::default()
        }));
    }

    if bp_material_handles.obstructed.is_none() {
        bp_material_handles.obstructed = Some(materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 0.0, 0.0, 0.5),
            alpha_mode: Blend,
            ..Default::default()
        }));
    }
    

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
        let translation = intersection.position();

        let mut transform_cache = Transform::from_translation(translation).with_rotation(quat);

        let building_id = selected_building.id.clone().unwrap();
        let mut building = string_to_building(building_id.to_string());

        let mut hovered = false;
        for interaction in gui_hover_query.iter() {
            match interaction {
                Interaction::None => (),
                _ => { hovered = true; break }
            }
        }

        let entity_op = pipe_prev_query.get_single();
        let pipe_prev_entity_op = match entity_op {
            Ok(e) => Some(e.0),
            Err(_) => None,
        };

        if building.shape_data.mesh.is_none() {
            building.shape_data.load_from_path(&asset_server, &gltf_meshes, &mut meshes, &mut materials, &mut images);
        }

        if selected_building.changed {
            // There shouldn't be multiple, but just in case.
            for (_, e) in cursor_bp_query.iter() {
                commands.entity(e).despawn_recursive();
            }

            let clone = building.shape_data.clone();
            spawn_cursor_bp(&mut commands, clone.mesh.unwrap(), &bp_material_handles, clone.collider.clone(), clone.collider_offset, transform_cache);
            selected_building.changed = false;
        } else {
            let (cursor_bp_transform, _) = cursor_bp_query.single_mut();
            let (_, mut moved, t) = cursor_bp_collider_query.single_mut();
            if cursor_bp_transform.clone() != transform_cache {
                move_cursor_bp(cursor_bp_transform, t, building.shape_data.collider_offset, transform_cache, &mut moved);
            }
        }

        match building_id.as_str() {
            // z offset for pipe cyl compared to pipe base: -0.0675
            "Pipe" => {

                // All pipe shit vvvvv
                let pipe_cyl_mesh: Handle<Mesh> = asset_server.load("models/pipes/pipe_cylinder.obj");
                let pipe_cyl_offset = Vec3::new(0.0, 0.25, 0.0675);
                // Rotate the offset and add it to the translation
                let offset_transform = transform_cache.with_translation(translation.add(quat.mul_vec3(pipe_cyl_offset)));

                let trans = offset_transform.translation;
        
                if mouse_input.just_pressed(MouseButton::Left) && !hovered {
                    // If you click, and the first point is already placed
                    // Place the second point and the pipe IF no collision

                    if pp_res.placed {
                        let first_position = pp_res.transform.unwrap().translation;
                        let (entity, _) = pipe_prev_query.single();
                        let inter = check_pipe_collision(entity, rapier_context);

                        if !inter {
                            let transform_c = transform_between_points(first_position, trans);

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
                        }
                        
                    // If you click and the first point is not placed
                    // Place the first point
                    } else {
                        pp_res.placed = true;
                        pp_res.transform = Some(offset_transform);

                        commands.spawn_bundle(PbrBundle {
                            mesh: pipe_cyl_mesh,
                            material: bp_material_handles.blueprint.clone().unwrap(),
                            transform: offset_transform.with_scale(Vec3::new(1.0, 0.02, 1.0)),
                            ..Default::default()
                        })
                        .insert(Collider::cuboid(0.135, 0.01, 0.135))
                        .insert(Sensor(true))
                        .insert(PipePreview)
                        .insert(NotShadowCaster);

                        commands.spawn_bundle(PbrBundle {
                            mesh: building.shape_data.mesh.unwrap(),
                            material: bp_material_handles.blueprint.clone().unwrap(),
                            transform: transform_cache,
                            ..Default::default()
                        })
                        .insert(NotShadowCaster);
                    }
                // If you're not clicking
                } else {
                    // If the first point is placed
                    // Update the preview
                    if pp_res.placed {
                        let first_position = pp_res.transform.unwrap().translation;
                        let transform_c = transform_between_points(first_position, trans);
                        let distance = distance_vec3(first_position, trans);
                        
                        let (_, mut transform) = pipe_prev_query.single_mut();

                        let transform_mut = transform.as_mut();
                        *transform_mut = transform_c;

                        if distance > 0.001 {
                            let mut collider_shape = pipe_prev_shape_query.single_mut();
                            let mut cuboid_mut = collider_shape.as_cuboid_mut().unwrap();
                            let mut half_extents = cuboid_mut.half_extents(); half_extents.y = distance / 2.0;

                            cuboid_mut.sed_half_extents(half_extents);
                        }
                    } // If the first isn't placed and you're not clicking, do nothing
                }
            },

            // every other building
            _ => {
                if pipe_prev_entity_op.is_some() {
                    commands.entity(pipe_prev_entity_op.unwrap()).despawn();
                }
        
                if mouse_input.just_pressed(MouseButton::Left) && !hovered {
                    spawn_bp(&mut commands, building.shape_data.clone(), building.iridium_data.cost, transform_cache);
                    selected_building.id = None;
                }
            }
        }
    }
}

pub fn check_cursor_bp_collision(
    mut cursor_bp: Query<&mut Handle<StandardMaterial>, With<CursorBp>>,
    mut cursor_bp_collider: Query<(Entity, &mut Moved), With<CursorBpCollider>>,
    rapier_context: Res<RapierContext>,
    bp_material_handles: Res<MaterialHandles>,
) {
    for (mut mat, (e, mut moved)) in cursor_bp.iter_mut().zip(cursor_bp_collider.iter_mut()) {
        if moved.0 {
            if e.is_intersecting(&rapier_context) {
                *mat = bp_material_handles.obstructed.clone().unwrap();
            } else {
                *mat = bp_material_handles.blueprint.clone().unwrap();
            }
            moved.0 = false;
        }
    }
}

fn transform_between_points(a: Vec3, b: Vec3) -> Transform {
    let translation = (a + b) / 2.0;
    let rotation = Quat::from_rotation_arc(Vec3::Y, (a - b).normalize());
    let distance = distance_vec3(a, b);

    Transform::from_translation(translation).with_rotation(rotation).with_scale(Vec3::new(1.0, distance, 1.0))
}