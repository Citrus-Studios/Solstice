use bevy::prelude::*;
use bevy_mod_raycast::{RayCastMesh, SimplifiedMesh};
use bevy_rapier3d::prelude::*;

use crate::{player_system::gui_system::gui_startup::SelectedBuilding, constants::BLUEPRINT_COLLISION};

use super::{building_components::*, buildings::BuildingReferenceComponent, building::EntityQuery, MaterialHandles, GlobalPipeId, RaycastSet, BlueprintFillMaterial};

pub fn check_cursor_bp_collision(
    mut commands: Commands,

    cursor_bp: EntityQuery<CursorBp>,
    cursor_bp_collider: EntityQuery<CursorBpCollider>,
    pipe_preview_cylinder: EntityQuery<PipePreviewCylinder>,
    pipe_placement: EntityQuery<PipePreviewPlacement>,
    pipe_preview: EntityQuery<PipePreview>,

    rapier_context: Res<RapierContext>,
    bp_material_handles: Res<MaterialHandles>,
    bp_fill_materials: Res<BlueprintFillMaterial>,
    mut selected_building: ResMut<SelectedBuilding>,
    mut global_pipe_id: ResMut<GlobalPipeId>,
    mut meshes: ResMut<Assets<Mesh>>,

    (mut moved_query, children_query, _parent_query, mut material_query, _transform_query, building_ref_query, try_place_query): (
        Query<&mut Moved>,
        Query<&Children>,
        Query<&Parent>,
        Query<&mut Handle<StandardMaterial>>,
        Query<&Transform>,
        Query<&BuildingReferenceComponent>,
        Query<&TryPlace>,
    ),
) {
    for (cbp_entity, cbp_collider_entity) in cursor_bp.iter().zip(cursor_bp_collider.iter()) {
        let mut moved = moved_query.get_mut(cbp_collider_entity).unwrap();
        let try_place = try_place_query.contains(cbp_entity);

        let intersecting = cbp_collider_entity.is_intersecting(&rapier_context);

        if moved.0 {
            let mut mat = material_query.get_mut(cbp_entity).unwrap();

            if intersecting {
                *mat = bp_material_handles.obstructed.clone();
            } else {
                *mat = bp_material_handles.blueprint.clone();
            }

            moved.0 = false;
        }

        // Tries to place the blueprint if it is not intersecting.
        // Removes all components that associate it with the cursor and replaces them with PlacedBlueprint.
        if try_place {
            commands.entity(cbp_entity).remove::<TryPlace>();
            if !intersecting {
                commands.entity(cbp_collider_entity)
                    .remove_bundle::<(Moved, CursorBpCollider)>()
                    .insert_bundle((
                        BLUEPRINT_COLLISION.clone(), 
                        Sensor(false)
                    ))
                ;

                let building = &building_ref_query.get(cbp_entity).unwrap().0;

                commands.entity(cbp_entity)
                    .remove::<CursorBp>()
                    .insert(
                        PlacedBlueprint {
                            cost: building.iridium_data.cost,
                            current: 0,
                        }
                    )
                ;
                selected_building.id = None;
            }
        }
    }

    if !pipe_preview.is_empty() {
        let pipe_cylinder = pipe_preview_cylinder.single();
        let first = pipe_placement.single();
        let second = cursor_bp.single();

        let mut intersecting = false;

        for entity in [pipe_cylinder, first, second] {
            if children_query.get(entity).unwrap()[0].is_intersecting(&rapier_context) {
                intersecting = true;
                break
            }
        }

        let set_material = match intersecting {
            true => bp_material_handles.obstructed.clone(),
            false => bp_material_handles.blueprint.clone(),
        };

        material_query.get_many_mut([pipe_cylinder, first, second]).unwrap().map(|mut material| *material = set_material.clone());

        let pipe_preview_entity = pipe_preview.single();

        if try_place_query.contains(pipe_preview_entity) && !intersecting {
            let place_mat = bp_fill_materials.get_fill_percent(0.0);
            let building_ref = &building_ref_query.get(pipe_preview_entity).unwrap().0;

            commands.entity(pipe_preview_entity)
                .remove::<PipePreview>()
                .insert(PipeBlueprint { cost: building_ref.iridium_data.cost, current: 0 })
            ;

            // First base of the pipe
            commands.entity(first)
                .remove::<PipePreviewPlacement>()
                .insert_bundle((
                    PipeFirst,
                    place_mat.clone(),
                    RayCastMesh::<RaycastSet>::default(),
                    SimplifiedMesh { mesh: building_ref.shape_data.simplified_mesh_handle.clone().unwrap() }
                ))
            ;

            // First base of the pipe's collider
            commands.entity(children_query.get(first).unwrap()[0])
                .insert_bundle((
                    BLUEPRINT_COLLISION.clone(),
                    Sensor(false),
                ))
            ;

            // Second base of the pipe
            commands.entity(second)
                .remove::<CursorBp>()
                .insert_bundle((
                    PipeSecond,
                    place_mat.clone(),
                    RayCastMesh::<RaycastSet>::default(),
                    SimplifiedMesh { mesh: building_ref.shape_data.simplified_mesh_handle.clone().unwrap() }
                ))
            ;

            // Second base of the pipe's collider
            commands.entity(children_query.get(second).unwrap()[0])
                .remove::<CursorBpCollider>()
                .insert_bundle((
                    BLUEPRINT_COLLISION.clone(),
                    Sensor(false),
                ))
            ;

            // Pipe cylinder in between both
            commands.entity(pipe_cylinder)
                .remove::<PipePreviewCylinder>()
                .insert_bundle((
                    PipeCylinder,
                    place_mat.clone()
                ))
            ;

            // Pipe cylinder collider
            commands.entity(children_query.get(pipe_cylinder).unwrap()[0])
                .remove::<PipePreviewCylinderCollider>()
                .insert_bundle((
                    BLUEPRINT_COLLISION.clone(),
                    Sensor(false),
                ))
            ;

            selected_building.id = None;
            selected_building.changed = true;
            global_pipe_id.0 += 1;
        }

        if try_place_query.contains(pipe_preview_entity) {
            commands.entity(pipe_preview_entity).remove::<TryPlace>();
        }
    }
}