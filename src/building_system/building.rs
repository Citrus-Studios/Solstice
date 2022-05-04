use std::f32::consts::PI;

use bevy::{pbr::{NotShadowCaster, AlphaMode::Blend}, input::mouse::MouseWheel, gltf::GltfMesh, ecs::system::QuerySingleError};
pub use bevy::{prelude::*};
use bevy_mod_raycast::{SimplifiedMesh, RayCastMesh};
use bevy_rapier3d::prelude::*;

use crate::{algorithms::distance_vec3, player_system::gui_system::gui_startup::{GuiButtonId, SelectedBuilding}, constants::HALF_PI};

use super::{raycasting::BuildCursor, buildings::{string_to_building, BuildingShapeData}, RaycastSet, MaterialHandles};


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

#[derive(Component)]
pub struct CursorBp;

pub struct ChangeBuilding {
    pub b: bool
}

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

    (mut materials, mut gltf_meshes, mut meshes, mut images): (
        ResMut<Assets<StandardMaterial>>, 
        ResMut<Assets<GltfMesh>>, 
        ResMut<Assets<Mesh>>, 
        ResMut<Assets<Image>>,
    ),

    mut cursor_bp_query: Query<(&mut Transform, &mut Handle<StandardMaterial>), (With<CursorBp>, Without<PipePreview>)>,

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
        let mut translation = intersection.position();

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

        let cursor_bp = cursor_bp_query.get_single_mut();

        match cursor_bp {
            Ok(e) => {
                move_cursor_bp(e, &bp_material_handles, transform_cache);
            },
            Err(e) => {
                match e {
                    QuerySingleError::NoEntities(_) => {
                        let clone = building.shape_data.clone();
                        spawn_cursor_bp(&mut commands, clone.mesh.unwrap(), &bp_material_handles, clone.collider.unwrap(), transform_cache);
                    },
                    QuerySingleError::MultipleEntities(_) => panic!("Multiple cursor blueprints! aaaaaaaaaa"),
                }
            },
        }

        match building_id.as_str() {
            // 0.0675
            "Pipe" => {

                // All pipe shit vvvvv
                let pipe_cyl_mesh: Handle<Mesh> = asset_server.load("models/pipes/pipe_cylinder.obj");        
                translation += normal * 0.3;
                transform_cache.translation = translation;
        
                if mouse_input.just_pressed(MouseButton::Left) && !hovered {
                    // If you click, and the first point is already placed
                    // Place the second point IF no collision

                    if pp_res.placed {
                        let (entity, _) = pipe_prev_query.single();
                        let inter = check_pipe_collision(entity, rapier_context);

                        if !inter {
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
                        }
                        
                    // If you click and the first point is not placed
                    // Place the first point
                    } else {
                        pp_res.placed = true;
                        pp_res.transform = Some(transform_cache);

                        commands.spawn_bundle(PbrBundle {
                            mesh: pipe_cyl_mesh,
                            material: bp_material_handles.blueprint.clone().unwrap(),
                            transform: Transform::from_translation(translation).with_rotation(quat).with_scale(Vec3::new(1.0, 0.0, 1.0)),
                            ..Default::default()
                        })
                        .insert(Collider::cuboid(0.135, 0.0, 0.135))
                        .insert(Sensor(true))
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
                        let inter = check_pipe_collision(entity, rapier_context);

                        let transform_mut = transform.as_mut();
                        *transform_mut = transform_c;

                        let material = materials.get_mut(pipe_prev_mat_query.single_mut().clone()).unwrap();

                        if distance > 0.001 {
                            let mut collider_shape = pipe_prev_shape_query.single_mut();
                            let mut cuboid_mut = collider_shape.as_cuboid_mut().unwrap();
                            let mut half_extents = cuboid_mut.half_extents(); half_extents.y = distance / 2.0;

                            cuboid_mut.sed_half_extents(half_extents);
                        }

                        if inter {
                            material.base_color = Color::rgba(1.0, 0.0, 0.0, 0.5);
                        } else {
                            material.base_color = Color::rgba(0.0, 0.2, 1.0, 0.5);
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
                    spawn_bp(&mut commands, building.shape_data.clone(), transform_cache);
                    selected_building.id = None;
                }
            }
        }
    }
}

fn check_pipe_collision(e: Entity, context: Res<RapierContext>) -> bool {
    for (_, _, c) in context.intersections_with(e) {
        if c {
            return true
        }
    }
    return false
}

// TODO: collision
fn spawn_cursor_bp(commands: &mut Commands, mesh: Handle<Mesh>, bp_materials: &ResMut<MaterialHandles>, collider_mesh: Mesh, transform: Transform) {    
    commands.spawn_bundle(PbrBundle {
        mesh,
        material: bp_materials.blueprint.clone().unwrap(),
        transform,
        ..Default::default()
    })
    .insert(Collider::bevy_mesh(&collider_mesh).unwrap())
    .insert(Sensor(true))
    .insert(NotShadowCaster)
    .insert(CursorBp);
}

fn move_cursor_bp(
    (mut transform, mut material): (Mut<Transform>, Mut<Handle<StandardMaterial>>),
    bp_materials: &ResMut<MaterialHandles>, 
    new_transform: Transform,
) {
    let trans = transform.as_mut();
    *trans = new_transform;

    let mat = material.as_mut();
    *mat = bp_materials.blueprint.clone().unwrap();
}

// TODO: everything
fn spawn_bp(commands: &mut Commands, shape_data: BuildingShapeData, transform: Transform) {
    commands.spawn_bundle(PbrBundle {
        mesh: shape_data.mesh.unwrap(),
        material: shape_data.material.unwrap(),
        transform,
        ..Default::default()
    })
    .insert(Collider::bevy_mesh(&shape_data.collider.unwrap()).unwrap())
    .insert(SimplifiedMesh {
        mesh: shape_data.collider_handle.unwrap(),
    })
    .insert(RayCastMesh::<RaycastSet>::default());
}