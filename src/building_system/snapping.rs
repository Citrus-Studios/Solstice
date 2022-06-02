use std::ops::Add;

use bevy::prelude::*;

use crate::{
    algorithms::{distance_vec3, ChildrenMethods},
    constants::SNAP_DISTANCE,
    player_system::gui_system::gui_startup::SelectedBuilding,
    terrain_generation_system::generator::TerrainBlockType,
};

use super::{
    building::{EntityQuery, RangeOps},
    building_components::*,
    building_functions::{move_cursor_bp, MoveTransform},
    buildings::{BuildingReferenceComponent, BuildingType},
    raycasting::BuildCursor,
};

pub fn snapping(
    mut commands: Commands,

    build_cursor: Res<BuildCursor>,
    selected_building: Res<SelectedBuilding>,

    (cbp_entity_query, cbp_collider_entity_query): (
        EntityQuery<CursorBp>,
        EntityQuery<CursorBpCollider>,
    ),

    (
        mut transform_query,
        mut moved_query,
        children_query,
        parent_query,
        well_query,
        building_ref_query,
        mut building_rot_query,
    ): (
        Query<&mut Transform>,
        Query<&mut Moved>,
        Query<&Children>,
        Query<&Parent>,
        Query<&TerrainBlockType>,
        Query<&BuildingReferenceComponent>,
        Query<&mut BuildingRotation>,
    ),
) {
    if build_cursor.intersection.is_none() || selected_building.id.is_none() {
        return;
    }

    if cbp_entity_query.is_empty() {
        return;
    }

    if selected_building.changed {
        return;
    }

    // Basically um, idk, ask me in the discord or something if you really want to know lol :P
    let intersected_entity = match parent_query.get(build_cursor.intersection.unwrap().0) {
        Ok(e) => e.0,
        Err(_) => build_cursor.intersection.unwrap().0,
    };

    let intersection = build_cursor.intersection.unwrap().1;
    let selected_building_type = selected_building.id.clone().unwrap();

    let cbp_entity = cbp_entity_query.single();
    let cbp_collider_entity = cbp_collider_entity_query.single();

    let [mut cursor_bp_transform, mut cursor_bp_collider_transform, relative_transform] =
        match transform_query.get_many_mut([cbp_entity, cbp_collider_entity, intersected_entity]) {
            Ok(e) => e,
            Err(_) => return,
        };

    let cbp_building = building_ref_query.get(cbp_entity).unwrap().0.clone();

    let mut rot = build_cursor.rotation;

    match selected_building_type {
        // I must do wellpumps seperately because they snap to something that isn't a building
        BuildingType::Wellpump => {
            commands.entity(cbp_entity).insert(Placeable::No);

            if let Ok(TerrainBlockType::Well) = well_query.get(intersected_entity) {
                let goal_translation = relative_transform.translation.add(Vec3::new(0.0, 1.5, 0.0));

                if distance_vec3(goal_translation, intersection.point) <= SNAP_DISTANCE {
                    let goal_transform = Transform::from_translation(goal_translation)
                        .with_rotation(Quat::from_axis_angle(Vec3::Y, rot));

                    let mut moved = moved_query.get_mut(cbp_collider_entity).unwrap();

                    move_cursor_bp(
                        &mut cursor_bp_transform,
                        &mut cursor_bp_collider_transform,
                        cbp_building.shape_data.collider_offset,
                        goal_transform,
                        &mut moved,
                        rot,
                        &mut building_rot_query.get_mut(cbp_collider_entity).unwrap(),
                    );

                    commands.entity(cbp_entity).insert(Placeable::Yes);
                }
            }
        }
        _ => {
            if let Ok(e) = building_ref_query.get(intersected_entity) {
                let intersected_building = &e.0;

                if intersected_building
                    .snap_data
                    .buildings
                    .contains(&selected_building_type)
                {
                    for (b, e) in intersected_building.snap_data.buildings.iter().zip(
                        intersected_building
                            .snap_data
                            .transform
                            .iter()
                            .zip(intersected_building.snap_data.rotation_allowed.iter()),
                    ) {
                        if *b == selected_building_type {
                            for ((trans, axis), can_rotate) in e.0.iter().zip(e.1.iter()) {
                                let snap_translation =
                                    relative_transform.with_add_translation(*trans).translation;

                                if distance_vec3(snap_translation, intersection.point)
                                    <= SNAP_DISTANCE
                                {
                                    let rot_range = can_rotate.clone().add(
                                        building_rot_query
                                            .get(children_query.first_child(intersected_entity))
                                            .unwrap()
                                            .0,
                                    );
                                    rot = rot.clamp(rot_range.start, rot_range.end);

                                    let real_axis = relative_transform.rotation.mul_vec3(*axis);

                                    let snap_transform =
                                        Transform::from_translation(snap_translation)
                                            .with_rotation(Quat::from_axis_angle(real_axis, rot));

                                    let (mut moved, mut building_rot) = (
                                        moved_query.get_mut(cbp_collider_entity).unwrap(),
                                        building_rot_query.get_mut(cbp_collider_entity).unwrap(),
                                    );

                                    move_cursor_bp(
                                        &mut cursor_bp_transform,
                                        &mut cursor_bp_collider_transform,
                                        cbp_building.shape_data.collider_offset,
                                        snap_transform,
                                        &mut moved,
                                        rot,
                                        &mut building_rot,
                                    );

                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
