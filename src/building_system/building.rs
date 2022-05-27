use std::{f32::consts::PI, ops::Add};

use bevy::{pbr::NotShadowCaster, input::mouse::MouseWheel};
pub use bevy::{prelude::*};


use bevy_rapier3d::{prelude::*};

use crate::{algorithms::distance_vec3, player_system::{gui_system::gui_startup::{GuiButtonId, SelectedBuilding}, player::CameraComp}, constants::{HALF_PI, PIPE_CYLINDER_OFFSET, SNAP_DISTANCE}, terrain_generation_system::generator::TerrainBlockType};

use super::{raycasting::BuildCursor, buildings::{string_to_building_enum, BuildingArcs, BuildingsResource, BuildingReferenceComponent, BuildingType}, MaterialHandles, building_components::*, building_functions::*, BlueprintFillMaterial};

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
    ClickFirstPointPlaced,
    ClickFirstPointNotPlaced,
    NoClickFirstPointPlaced,
    Other,
}

impl PipeBools {
    fn match_bools(clicked: bool, hovered: bool, placed: bool) -> PipeBools {
        match (clicked, hovered, placed) {
            (true, false, true) => PipeBools::ClickFirstPointPlaced,
            (true, false, false) => PipeBools::ClickFirstPointNotPlaced,
            (false, _, true) => PipeBools::NoClickFirstPointPlaced,
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

    (
        pipe_prev_cylinder_query, 
        pipe_prev_cylinder_collider_query, 
        pipe_prev_placement_query, 
        pipe_prev_query,
    ): (
        EntityQuery<PipePreviewCylinder>, 
        EntityQuery<PipePreviewCylinderCollider>,
        EntityQuery<PipePreviewPlacement>,
        EntityQuery<PipePreview>,
    ), 
    
    camera_query: EntityQuery<CameraComp>,
    mut cursor_bp_query: EntityQuery<CursorBp>,
    mut cursor_bp_collider_query: EntityQuery<CursorBpCollider>,
    
    asset_server: Res<AssetServer>,
    mut selected_building: ResMut<SelectedBuilding>,

    _materials: ResMut<Assets<StandardMaterial>>,

    mut bc_res: ResMut<BuildCursor>,
    bp_material_handles: ResMut<MaterialHandles>,

    gui_hover_query: Query<&Interaction, With<GuiButtonId>>,

    (mouse_input, keyboard_input, building_arcs, buildings_res): (
        Res<Input<MouseButton>>, 
        Res<Input<KeyCode>>, 
        Res<BuildingArcs>,
        Res<BuildingsResource>,
    ),

    mut mouse_scroll_event: EventReader<MouseWheel>,

    (mut transform_query, mut moved_query, mut visibility_query, _global_transform_query, children_query, well_query): (
        Query<&mut Transform>,
        Query<&mut Moved>,
        Query<&mut Visibility>,
        Query<&GlobalTransform>,
        Query<&Children>,
        Query<&TerrainBlockType>,
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
        // math...
        let (intersected_entity, intersection) = intersection_op.unwrap();
        let normal = intersection.normal.normalize();

        let camera_pos = transform_query.get(camera_query.single()).unwrap().translation;
        let projected = camera_pos.project_onto_plane(normal);
        let zero_vec = Quat::from_rotation_arc(Vec3::Y, normal).mul_vec3(Vec3::Z);
        rot -= (projected.angle_between_clockwise(zero_vec, normal) / (PI/16.0)).round() * (PI/16.0);

        let quat = Quat::from_axis_angle(normal, rot).mul_quat(Quat::from_rotation_arc(Vec3::Y, normal));
        let translation = intersection.point;

        let transform_cache = Transform::from_translation(translation).with_rotation(quat);

        let building_id = selected_building.id.clone().unwrap();
        let building = buildings_res.0.get(&string_to_building_enum(selected_building.id.clone().unwrap())).unwrap();

        let building_arc = building_arcs.0.get(&building.building_id.building_type).unwrap().clone();

        // check if we're hovering over the gui
        let mut hovered = false;
        for interaction in gui_hover_query.iter() {
            match interaction {
                Interaction::None => (),
                _ => { hovered = true; break }
            }
        }

        let cbp_entity;

        if selected_building.changed {
            // There shouldn't be multiple, but just in case.
            for e in cursor_bp_query.iter() {
                commands.entity(e).despawn_recursive();
            }

            let clone = building.shape_data.clone();
            
            cbp_entity = spawn_cursor_bp(
                &mut commands, 
                building_arc.clone(), 
                clone.mesh.unwrap(), 
                &bp_material_handles, 
                clone.collider.clone(), 
                clone.collider_offset, 
                transform_cache
            );

            selected_building.changed = false;
        } else {
            let cursor_bp_entity = cursor_bp_query.single();
            cbp_entity = cursor_bp_entity;

            let cursor_bp_collider_entity = cursor_bp_collider_query.single();

            let [mut cursor_bp_transform, mut cursor_bp_collider_transform] = 
                transform_query.many_mut([cursor_bp_entity, cursor_bp_collider_entity]);

            let mut moved = moved_query.get_mut(cursor_bp_collider_entity).unwrap();
            
            visibility_query.get_mut(cbp_entity).unwrap().is_visible = true;

            if cursor_bp_transform.clone() != transform_cache {
                move_cursor_bp(
                    &mut cursor_bp_transform, 
                    &mut cursor_bp_collider_transform, 
                    building.shape_data.collider_offset, 
                    transform_cache, 
                    &mut moved
                );
            }
        }

        match building_id.as_str() {
            // z offset for pipe cyl compared to pipe base: -0.0675
            "Pipe" => {

                // All pipe shit vvvvv
                let pipe_cyl_mesh: Handle<Mesh> = asset_server.load("models/pipes/pipe_cylinder.obj");
                let pipe_cyl_offset = Vec3::new(0.0, 0.25, 0.0675);
                // Rotate the offset and add it to the translation
                let offset_transform = transform_cache.with_add_translation(pipe_cyl_offset);

                let trans = offset_transform.translation;

                match PipeBools::match_bools(mouse_input.just_pressed(MouseButton::Left), hovered, !pipe_prev_placement_query.is_empty()) {
                    // (just clicked, hovering over gui, first pipe point placed, intersecting)
                    PipeBools::ClickFirstPointPlaced => {
                        // Place the whole pipe blueprint
                        let first_position = transform_query.get(pipe_prev_placement_query.single()).unwrap().with_add_translation(pipe_cyl_offset).translation;
                        let transform = transform_query.get_many_mut([pipe_prev_cylinder_query.single(), pipe_prev_cylinder_collider_query.single()]).unwrap();
                        update_pipe_cylinder_transform(transform, first_position, trans);

                        // try to place
                        commands.entity(pipe_prev_query.single())
                            .insert(TryPlace)
                        ;
                    },

                    PipeBools::ClickFirstPointNotPlaced => {

                        // spawn pipe preview
                        commands.spawn()
                            .insert_bundle((
                                GlobalTransform::identity(),
                                Transform::default(),
                                PipePreview,
                                BuildingReferenceComponent(building_arc.clone())
                            ))
                            .with_children(|parent| {
                                parent.spawn_bundle(PbrBundle {
                                    mesh: pipe_cyl_mesh,
                                    material: bp_material_handles.blueprint.clone(),
                                    transform: offset_transform.with_scale(Vec3::new(1.0, 0.001, 1.0)),
                                    ..Default::default()
                                })
                                .insert(PipePreviewCylinder)
                                .with_children(|parent| {
                                    parent.spawn_bundle((
                                        offset_transform.with_scale(Vec3::new(1.0, 0.001, 1.0)),
                                        Collider::cuboid(0.135, 0.5, 0.135),
                                        CollisionGroups { memberships: 0b00001000, filters: 0b11101111 },
                                        Sensor(true),
                                        PipePreviewCylinderCollider,
                                        NotShadowCaster
                                    ));
                                });

                                parent.spawn_bundle(PbrBundle {
                                    mesh: building.shape_data.mesh.clone().unwrap(),
                                    material: bp_material_handles.blueprint.clone(),
                                    transform: transform_cache,
                                    ..Default::default()
                                })
                                .insert_bundle((
                                    PipePreviewPlacement,
                                    NotShadowCaster
                                ))
                                .with_children(|parent| {
                                    parent.spawn_bundle((
                                        building.shape_data.collider.clone(),
                                        transform_cache.with_add_translation(building.shape_data.collider_offset),
                                        Sensor(true),
                                    ));
                                });
                            })
                            .add_child(cbp_entity)
                        ;

                        bc_res.rotation += PI;
                    },
                    PipeBools::NoClickFirstPointPlaced => {
                        // Update the preview, change pipe cylinder transform
                        let first_position = transform_query.get(pipe_prev_placement_query.single()).unwrap().with_add_translation(pipe_cyl_offset).translation;
                        let transform = transform_query.get_many_mut([pipe_prev_cylinder_query.single(), pipe_prev_cylinder_collider_query.single()]).unwrap();
                        update_pipe_cylinder_transform(transform, first_position, trans);
                    }
                    _ => ()
                }
            },

            // every other building
            _ => {
                match building_arc.building_id.building_type {
                    BuildingType::Wellpump => {
                        commands.entity(cbp_entity).insert(Placeable(false));
                        if well_query.get(intersected_entity).unwrap_or(&TerrainBlockType::Solid).clone() == TerrainBlockType::Well {
                            let goal_translation = 
                            transform_query.get(intersected_entity).unwrap()
                                .translation.add(Vec3::new(0.0, 1.5, 0.0));

                            if distance_vec3(goal_translation, transform_cache.translation) <= SNAP_DISTANCE {
                                let goal_transform = Transform::from_translation(goal_translation)
                                    .with_rotation(Quat::from_axis_angle(Vec3::Y, rot));

                                let cbp_collider_entity = children_query.get(cbp_entity).unwrap()[0];

                                let [mut cursor_bp_transform, mut cursor_bp_collider_transform] = 
                                    transform_query.many_mut([cbp_entity, cbp_collider_entity]);

                                let mut moved = moved_query.get_mut(cbp_collider_entity).unwrap();

                                move_cursor_bp(
                                    &mut cursor_bp_transform, 
                                    &mut cursor_bp_collider_transform, 
                                    building.shape_data.collider_offset, 
                                    goal_transform, 
                                    &mut moved
                                );

                                commands.entity(cbp_entity).insert(Placeable(true));
                            }
                        }
                    },
                    _ => {
                        commands.entity(cbp_entity).insert(Placeable(true));
                    }
                }
                if mouse_input.just_pressed(MouseButton::Left) && !hovered {
                    commands.entity(cbp_entity).insert(TryPlace);
                }
            }
        }
    } else if selected_building.id.is_some() {
        match cursor_bp_query.get_single() {
            Ok(e) => {
                visibility_query.get_mut(e).unwrap().is_visible = false;
            },
            Err(_) => (),
        }
    }
}

fn update_pipe_cylinder_transform(
    mut transform: [Mut<Transform>; 2],

    first_pos: Vec3,
    second_pos: Vec3,
) {
    let mut set_transform = transform_between_points(first_pos, second_pos);
    set_transform.scale.y = set_transform.scale.y.max(0.001);

    let (cylinder, collider) = transform.split_at_mut(1);

    *cylinder[0] = set_transform;
    *collider[0] = set_transform;
}

fn transform_between_points(a: Vec3, b: Vec3) -> Transform {
    let translation = (a + b) / 2.0;
    let rotation = Quat::from_rotation_arc(Vec3::Y, (a - b).normalize());
    let distance = distance_vec3(a, b);

    Transform::from_translation(translation).with_rotation(rotation).with_scale(Vec3::new(1.0, distance, 1.0))
}

trait MoreVec3Methods {
    // ((self dot normal) / (normal mag squared)) normal
    /// Returns the projection of `self` onto the plane defined by its `normal`
    fn project_onto_plane(self, plane_normal: Vec3) -> Vec3;

    /// Returns the clockwise angle between `self` and `other` 
    /// 
    /// Both must be contained in the plane defined by its normal, `norm`
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