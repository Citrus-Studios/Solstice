use std::{
    f32::consts::PI,
    ops::{Add, Range},
};

use bevy::input::mouse::MouseWheel;
pub use bevy::prelude::*;

use crate::{
    algorithms::distance_vec3,
    constants::{HALF_PI, PIPE_BASE_OFFSET, PIPE_CYLINDER_OFFSET},
    player_system::{
        gui_system::gui_startup::{GuiButtonId, SelectedBuilding},
        player::CameraComp,
    },
};

use super::{
    building_components::*,
    building_functions::*,
    buildings::{BuildingArcs, BuildingType, BuildingsResource},
    raycasting::BuildCursor,
    MaterialHandles,
};

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
            self.pt_1
                .translation
                .add(self.pt_1.rotation.mul_vec3(*PIPE_CYLINDER_OFFSET)),
            self.pt_2
                .translation
                .add(self.pt_2.rotation.mul_vec3(*PIPE_CYLINDER_OFFSET)),
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
            _ => PipeBools::Other,
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
    cursor_bp_query: EntityQuery<CursorBp>,
    cursor_bp_collider_query: EntityQuery<CursorBpCollider>,

    selected_building: ResMut<SelectedBuilding>,

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

    (mut transform_query, mut moved_query, mut visibility_query, mut building_rot_query): (
        Query<&mut Transform>,
        Query<&mut Moved>,
        Query<&mut Visibility>,
        Query<&mut BuildingRotation>,
    ),
) {
    for entity in delete_query.iter() {
        commands.entity(entity).despawn();
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        for event in mouse_scroll_event.iter() {
            bc_res.rotation += event.y * (PI / 16.0);
        }
    }

    if keyboard_input.just_pressed(KeyCode::R) {
        bc_res.rotation += HALF_PI;
    }

    let intersection_op = bc_res.intersection;

    let mut rot = bc_res.rotation;

    if intersection_op.is_some() && selected_building.id.is_some() {
        // math...
        let (_, intersection) = intersection_op.unwrap();
        let translation = intersection.point;
        let normal = intersection.normal.normalize();

        let selected_building_type = selected_building.id.clone().unwrap();

        let building = buildings_res.0.get(&selected_building_type).unwrap();

        let building_arc = building_arcs
            .0
            .get(&building.building_id.building_type)
            .unwrap()
            .clone();

        let camera_pos = transform_query
            .get(camera_query.single())
            .unwrap()
            .translation;

        let projected = camera_pos.project_onto_plane(normal);
        let zero_vec = Quat::from_rotation_arc(Vec3::Y, normal).mul_vec3(Vec3::Z);
        rot -= (projected.angle_between_clockwise(zero_vec, normal) / (PI / 16.0)).round()
            * (PI / 16.0);

        let quat =
            Quat::from_axis_angle(normal, rot).mul_quat(Quat::from_rotation_arc(Vec3::Y, normal));

        let mut transform_cache = Transform::from_translation(translation).with_rotation(quat);

        if selected_building_type == BuildingType::Pipe {
            transform_cache = transform_cache.with_add_translation(*PIPE_BASE_OFFSET);
        }

        // check if we're hovering over the gui
        let mut hovered = false;
        for interaction in gui_hover_query.iter() {
            match interaction {
                Interaction::None => (),
                _ => {
                    hovered = true;
                    break;
                }
            }
        }

        let cbp_entity;

        if selected_building.changed {
            // There shouldn't be multiple, but just in case.
            for e in cursor_bp_query.iter() {
                commands.entity(e).despawn_recursive();
            }

            let clone = building.shape_data.clone();

            (cbp_entity, _) = spawn_cursor_bp(
                &mut commands,
                building_arc.clone(),
                clone.mesh.unwrap(),
                &bp_material_handles,
                clone.collider.clone(),
                clone.collider_offset,
                transform_cache,
                rot,
            );
        } else {
            let cursor_bp_entity = cursor_bp_query.single();
            cbp_entity = cursor_bp_entity;

            let cbp_collider_entity = cursor_bp_collider_query.single();

            let [mut cursor_bp_transform, mut cursor_bp_collider_transform] =
                transform_query.many_mut([cbp_entity, cbp_collider_entity]);

            let mut moved = moved_query.get_mut(cbp_collider_entity).unwrap();

            visibility_query.get_mut(cbp_entity).unwrap().is_visible = true;
            let mut building_rot = building_rot_query.get_mut(cbp_collider_entity).unwrap();

            if cursor_bp_transform.clone() != transform_cache {
                move_cursor_bp(
                    &mut cursor_bp_transform,
                    &mut cursor_bp_collider_transform,
                    building.shape_data.collider_offset,
                    transform_cache,
                    &mut moved,
                    rot,
                    &mut building_rot,
                );
            }
        }

        if mouse_input.just_pressed(MouseButton::Left) && !hovered {
            commands.entity(cbp_entity).insert(TryPlace);
        }

        match selected_building_type {
            // z offset for pipe cyl compared to pipe base: -0.0675
            BuildingType::Pipe => {
                // Rotate the offset and add it to the translation
                let offset_transform = transform_cache.with_add_translation(*PIPE_CYLINDER_OFFSET);

                let trans = offset_transform.translation;

                if !pipe_prev_placement_query.is_empty() {
                    let first_position = transform_query
                        .get(pipe_prev_placement_query.single())
                        .unwrap()
                        .with_add_translation(*PIPE_CYLINDER_OFFSET)
                        .translation;
                    let transform = transform_query
                        .get_many_mut([
                            pipe_prev_cylinder_query.single(),
                            pipe_prev_cylinder_collider_query.single(),
                        ])
                        .unwrap();
                    update_pipe_cylinder_transform(transform, first_position, trans);
                }

                match PipeBools::match_bools(
                    mouse_input.just_pressed(MouseButton::Left),
                    hovered,
                    !pipe_prev_placement_query.is_empty(),
                ) {
                    PipeBools::ClickFirstPointPlaced => {
                        commands.entity(pipe_prev_query.single()).insert(TryPlace);
                    }
                    PipeBools::ClickFirstPointNotPlaced => {
                        commands.entity(cbp_entity).insert(TryPlace);
                    }
                    _ => (),
                }
            }

            // every other building
            _ => {}
        }
    } else if selected_building.id.is_some() {
        match cursor_bp_query.get_single() {
            Ok(e) => {
                visibility_query.get_mut(e).unwrap().is_visible = false;
            }
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

    Transform::from_translation(translation)
        .with_rotation(rotation)
        .with_scale(Vec3::new(1.0, distance, 1.0))
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

pub trait RangeOps<T> {
    fn add(self, add: T) -> Self;
}

impl<T: std::ops::Add<Output = T>> RangeOps<T> for Range<T>
where
    T: Copy,
{
    fn add(self, add: T) -> Self {
        (self.start + add)..(self.end + add)
    }
}
