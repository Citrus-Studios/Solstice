use std::{f32::consts::PI, ops::Add};

use bevy::{pbr::NotShadowCaster, input::mouse::MouseWheel};
pub use bevy::{prelude::*};

use bevy_mod_raycast::{SimplifiedMesh, RayCastMesh};
use bevy_rapier3d::{prelude::*, rapier::crossbeam::channel::Select};

use crate::{algorithms::distance_vec3, player_system::{gui_system::gui_startup::{GuiButtonId, SelectedBuilding}, player::CameraComp}, constants::{HALF_PI, PIPE_CYLINDER_OFFSET}};

use super::{raycasting::BuildCursor, buildings::{string_to_building_enum, BuildingArcs, BuildingsResource, BuildingReferenceComponent}, MaterialHandles, building_components::*, building_functions::*, BlueprintFillMaterial, RaycastSet};

#[derive(Component, Clone)]
pub struct Pipe {
    pub pt_1: Transform,
    pub pt_2: Transform,
}

impl Pipe {
    pub fn new(pt_1: Transform, pt_2: Transform) -> Self {
        Pipe { pt_1, pt_2 }
    }

    pub fn cylinder_transform(&self) -> Transform {
        transform_between_points(
            self.pt_1.translation.add(self.pt_1.rotation.mul_vec3(*PIPE_CYLINDER_OFFSET)), 
            self.pt_2.translation.add(self.pt_2.rotation.mul_vec3(*PIPE_CYLINDER_OFFSET))
        )
    }
}

enum PipeBools {
    ClickFirstPointPlacedNoIntersection,
    ClickFirstPointNotPlacedNoIntersection,
    NoClickFirstPointPlaced,
    Other,
}

impl PipeBools {
    fn match_bools(clicked: bool, hovered: bool, placed: bool, intersection: bool) -> PipeBools {
        match (clicked, hovered, placed, intersection) {
            (true, false, true, false) => PipeBools::ClickFirstPointPlacedNoIntersection,
            (true, false, false, _) => PipeBools::ClickFirstPointNotPlacedNoIntersection,
            (false, _, true, _) => PipeBools::NoClickFirstPointPlaced,
            _ => PipeBools::Other
        }
    }
}

/// Query for entities with component T
pub type EntityQuery<'a, 'b, T> = Query<'a, 'b, Entity, With<T>>;

pub fn building(
    // so many parameters...
    mut commands: Commands,

    delete_query: EntityQuery<DeleteNextFrame>,

    mut pipe_prev_query: EntityQuery<PipePreview>, 
    
    camera_query: EntityQuery<CameraComp>,
    mut cursor_bp_query: EntityQuery<CursorBp>,
    mut cursor_bp_collider_query: EntityQuery<CursorBpCollider>,
    
    rapier_context: Res<RapierContext>,
    asset_server: Res<AssetServer>,
    mut selected_building: ResMut<SelectedBuilding>,

    mut materials: ResMut<Assets<StandardMaterial>>,

    (mut bc_res, mut pp_res): (ResMut<BuildCursor>, ResMut<PipePlacement>),
    bp_material_handles: ResMut<MaterialHandles>,

    gui_hover_query: Query<&Interaction, With<GuiButtonId>>,

    (mouse_input, keyboard_input, bp_fill_materials, building_arcs, buildings_res): (
        Res<Input<MouseButton>>, 
        Res<Input<KeyCode>>, 
        Res<BlueprintFillMaterial>,
        Res<BuildingArcs>,
        Res<BuildingsResource>,
    ),

    mut mouse_scroll_event: EventReader<MouseWheel>,

    (mut transform_query, mut moved_query): (
        Query<&mut Transform>,
        Query<&mut Moved>,
    ),
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

    let mut rot = bc_res.rotation;
    
    if intersection_op.is_some() && selected_building.id.is_some() {
        let intersection = intersection_op.unwrap();
        let normal = intersection.normal().normalize();

        let camera_pos = transform_query.get(camera_query.single()).unwrap().translation;
        let projected = camera_pos.project_onto_plane(normal);
        let zero_vec = Quat::from_rotation_arc(Vec3::Y, normal).mul_vec3(Vec3::Z);
        rot -= (projected.angle_between_clockwise(zero_vec, normal) / (PI/16.0)).round() * (PI/16.0);

        // my brain
        let quat = Quat::from_axis_angle(normal, rot).mul_quat(Quat::from_rotation_arc(Vec3::Y, normal));
        let translation = intersection.position();

        let transform_cache = Transform::from_translation(translation).with_rotation(quat);

        let building_id = selected_building.id.clone().unwrap();
        let building = buildings_res.0.get(&string_to_building_enum(selected_building.id.clone().unwrap())).unwrap();

        let mut hovered = false;
        for interaction in gui_hover_query.iter() {
            match interaction {
                Interaction::None => (),
                _ => { hovered = true; break }
            }
        }

        let entity_op = pipe_prev_query.get_single();
        let pipe_prev_entity_op = match entity_op {
            Ok(e) => Some(e),
            Err(_) => None,
        };

        let cbp_entity;

        if selected_building.changed {
            // There shouldn't be multiple, but just in case.
            for e in cursor_bp_query.iter() {
                commands.entity(e).despawn_recursive();
            }

            let clone = building.shape_data.clone();
            
            cbp_entity = spawn_cursor_bp(
                &mut commands, 
                building_arcs.0.get(&building.building_id.building_type).unwrap().clone(), 
                clone.mesh.unwrap(), 
                &bp_material_handles, 
                clone.collider.clone(), 
                clone.collider_offset, 
                transform_cache
            );

            selected_building.changed = false;
        } else {
            let cursor_bp_entity = cursor_bp_query.single_mut();
            cbp_entity = cursor_bp_entity;

            let cursor_bp_collider_entity = cursor_bp_collider_query.single_mut();

            let mut e = transform_query.many_mut([cursor_bp_entity, cursor_bp_collider_entity]);
            let cbp = e.split_at_mut(1);

            let cursor_bp_transform = cbp.0[0].as_mut();
            let cursor_bp_collider_transform = cbp.1[0].as_mut();

            let mut moved = moved_query.get_mut(cursor_bp_collider_entity).unwrap();

            if cursor_bp_transform.clone() != transform_cache && transform_cache != pp_res.transform.unwrap_or(Transform::from_xyz(f32::MAX, f32::MAX, f32::MAX)) {
                move_cursor_bp(cursor_bp_transform, cursor_bp_collider_transform, building.shape_data.collider_offset, transform_cache, &mut moved);
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
                let inter = check_pipe_collision(pipe_prev_query.single(), rapier_context);

                match PipeBools::match_bools(mouse_input.just_pressed(MouseButton::Left), hovered, pp_res.placed, inter) {
                    // (just clicked, hovering over gui, first pipe point placed, intersecting)
                    PipeBools::ClickFirstPointPlacedNoIntersection => {
                        // Place the whole pipe blueprint
                        let first_position = pp_res.transform.unwrap().translation;
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
                        bc_res.rotation = 0.0;
                    },
                    // (just clicked, hovering over gui, first pipe point placed, _)
                    PipeBools::ClickFirstPointNotPlacedNoIntersection => {
                        // Place the first pipe point
                        pp_res.placed = true;
                        pp_res.transform = Some(offset_transform);

                        // spawn pipe preview
                        commands.spawn_bundle(PbrBundle {
                            mesh: pipe_cyl_mesh,
                            material: bp_material_handles.blueprint.clone(),
                            transform: offset_transform.with_scale(Vec3::new(1.0, 0.02, 1.0)),
                            ..Default::default()
                        })
                        .insert(Collider::cuboid(0.135, 0.5, 0.135))
                        .insert(Sensor(true))
                        .insert(PipePreview)
                        .insert(NotShadowCaster);

                        commands.spawn_bundle(PbrBundle {
                            mesh: building.shape_data.mesh.clone().unwrap(),
                            material: bp_material_handles.blueprint.clone(),
                            transform: transform_cache,
                            ..Default::default()
                        })
                        .insert(NotShadowCaster);

                        bc_res.rotation += PI;
                    },
                    // (just clicked, _, first pipe point placed, _)
                    PipeBools::NoClickFirstPointPlaced => {
                        // Update the preview, change pipe cylinder transform
                        let first_position = pp_res.transform.unwrap().translation;
                        let transform_c = transform_between_points(first_position, trans);

                        let entity = pipe_prev_query.single_mut();
                        let mut transform = transform_query.get_mut(entity).unwrap();

                        transform.scale.y = transform.scale.y.min(0.01);

                        let transform_mut = transform.as_mut();
                        *transform_mut = transform_c;
                    }
                    _ => ()
                }
            },

            // every other building
            _ => {
                if pipe_prev_entity_op.is_some() {
                    commands.entity(pipe_prev_entity_op.unwrap()).despawn();
                }
        
                if mouse_input.just_pressed(MouseButton::Left) && !hovered {
                    commands.entity(cbp_entity).insert(TryPlace);
                }
            }
        }
    }
}

pub fn check_cursor_bp_collision(
    mut commands: Commands,

    cursor_bp: EntityQuery<CursorBp>,
    cursor_bp_collider: EntityQuery<CursorBpCollider>,
    pipe_preview: EntityQuery<PipePreview>,

    rapier_context: Res<RapierContext>,
    bp_material_handles: Res<MaterialHandles>,
    mut selected_building: ResMut<SelectedBuilding>,

    mut moved_query: Query<&mut Moved>,
    mut material_query: Query<&mut Handle<StandardMaterial>>,
    building_ref_query: Query<&BuildingReferenceComponent>,
    try_place_query: Query<&TryPlace>,
) {
    for (cbp_entity, cbp_collider_entity) in cursor_bp.iter().zip(cursor_bp_collider.iter()) {
        let mut moved = moved_query.get_mut(cbp_collider_entity).unwrap();
        let mut intersecting = None;
        let try_place = try_place_query.get(cbp_entity);

        if moved.0 || try_place.is_ok() {
            intersecting = Some(cbp_collider_entity.is_intersecting(&rapier_context));
        }

        if moved.0 {
            let mut mat = material_query.get_mut(cbp_entity).unwrap();

            if intersecting.unwrap() {
                *mat = bp_material_handles.obstructed.clone();
            } else {
                *mat = bp_material_handles.blueprint.clone();
            }

            moved.0 = false;
        }

        if try_place.is_ok() {
            commands.entity(cbp_entity).remove::<TryPlace>();
            if !intersecting.unwrap() {
                commands.entity(cbp_collider_entity)
                    .remove_bundle::<(Moved, CursorBpCollider)>()
                    .insert_bundle((
                        CollisionGroups { memberships: 0, filters: 0 }, 
                        Sensor(false)
                    ))
                ;

                let building = &building_ref_query.get(cbp_entity).unwrap().0;

                commands.entity(cbp_entity)
                    .remove::<CursorBp>()
                    .insert_bundle((
                        RayCastMesh::<RaycastSet>::default(),
                        SimplifiedMesh { mesh: building.shape_data.simplified_mesh_handle.clone().unwrap() },
                        PlacedBlueprint {
                            cost: building.iridium_data.cost,
                            current: 0,
                        }
                    ))
                ;
                selected_building.id = None;
            }
        }
    }

    for e in pipe_preview.iter() {
        let mut mat = material_query.get_mut(e).unwrap();
        if e.is_intersecting(&rapier_context) {
            *mat = bp_material_handles.obstructed.clone();
        } else {
            *mat = bp_material_handles.blueprint.clone();
        }
    }
}

fn transform_between_points(a: Vec3, b: Vec3) -> Transform {
    let translation = (a + b) / 2.0;
    let rotation = Quat::from_rotation_arc(Vec3::Y, (a - b).normalize());
    let distance = distance_vec3(a, b);

    Transform::from_translation(translation).with_rotation(rotation).with_scale(Vec3::new(1.0, distance, 1.0))
}

trait MoreVec3Methods {
    // ((self dot normal) / (normal mag squared)) normal
    fn project_onto_plane(self, plane_normal: Vec3) -> Vec3;
    fn angle_between_clockwise(self, other: Vec3, norm: Vec3) -> f32;
}

impl MoreVec3Methods for Vec3 {
    fn project_onto_plane(self, plane_normal: Vec3) -> Vec3 {
        self - (self.dot(plane_normal) * plane_normal)
    }

    fn angle_between_clockwise(self, other: Vec3, norm: Vec3) -> f32 {
        norm.dot(self.cross(other)).atan2(self.dot(other))
    }
}