use std::f32::consts::PI;

use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_mod_raycast::{RayCastMesh, SimplifiedMesh};
use bevy_rapier3d::prelude::*;

use crate::{
    constants::{BLUEPRINT_COLLISION, PIPE_CYLINDER_OFFSET},
    player_system::gui_system::gui_startup::SelectedBuilding,
};

use super::{
    building::EntityQuery,
    building_components::*,
    building_functions::MoveTransform,
    buildings::{BuildingReferenceComponent, BuildingType},
    raycasting::BuildCursor,
    BlueprintFillMaterial, GlobalPipeId, MaterialHandles, RaycastSet,
};

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
    mut bc_res: ResMut<BuildCursor>,
    asset_server: ResMut<AssetServer>,

    (
        mut moved_query,
        children_query,
        _parent_query,
        mut material_query,
        transform_query,
        building_ref_query,
        try_place_query,
        placeable_query,
    ): (
        Query<&mut Moved>,
        Query<&Children>,
        Query<&Parent>,
        Query<&mut Handle<StandardMaterial>>,
        Query<&Transform>,
        Query<&BuildingReferenceComponent>,
        Query<&TryPlace>,
        Query<&Placeable>,
    ),
) {
    for (cbp_entity, cbp_collider_entity) in cursor_bp.iter().zip(cursor_bp_collider.iter()) {
        let mut moved = moved_query.get_mut(cbp_collider_entity).unwrap();
        let try_place = try_place_query.contains(cbp_entity);

        let intersecting = cbp_collider_entity.is_intersecting(&rapier_context);
        let placeable = placeable_query
            .get(cbp_entity)
            .unwrap_or(&Placeable::WithCollision);

        let can_place = match placeable {
            Placeable::Yes => true,
            Placeable::WithCollision => !intersecting,
            Placeable::No => false,
        };

        if moved.0 {
            let mut mat = material_query.get_mut(cbp_entity).unwrap();

            if can_place {
                *mat = bp_material_handles.blueprint.clone();
            } else {
                *mat = bp_material_handles.obstructed.clone();
            }

            moved.0 = false;
        }

        // Tries to place the blueprint if it is not intersecting.
        // Removes all components that associate it with the cursor and replaces them with PlacedBlueprint.
        if try_place {
            commands.entity(cbp_entity).remove::<TryPlace>();
            if can_place {
                let building = &building_ref_query.get(cbp_entity).unwrap().0;

                match building.building_id.building_type {
                    BuildingType::Pipe => {
                        if pipe_preview.is_empty() {
                            let pipe_cyl_mesh: Handle<Mesh> =
                                asset_server.load("models/pipes/pipe_cylinder.obj");

                            let rot = bc_res.rotation;
                            let transform = transform_query.get(cbp_entity).unwrap();
                            let offset_transform =
                                transform.with_add_translation(*PIPE_CYLINDER_OFFSET);

                            commands
                                .spawn()
                                .insert_bundle((
                                    GlobalTransform::identity(),
                                    Transform::default(),
                                    PipePreview,
                                    BuildingReferenceComponent(building.clone()),
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(PbrBundle {
                                            mesh: pipe_cyl_mesh,
                                            material: bp_material_handles.blueprint.clone(),
                                            transform: offset_transform
                                                .with_scale(Vec3::new(1.0, 0.001, 1.0)),
                                            ..Default::default()
                                        })
                                        .insert(PipePreviewCylinder)
                                        .with_children(|parent| {
                                            parent.spawn_bundle((
                                                offset_transform
                                                    .with_scale(Vec3::new(1.0, 0.001, 1.0)),
                                                Collider::cuboid(0.135, 0.5, 0.135),
                                                CollisionGroups {
                                                    memberships: 0b00001000,
                                                    filters: 0b11101111,
                                                },
                                                Sensor(true),
                                                PipePreviewCylinderCollider,
                                                NotShadowCaster,
                                            ));
                                        });

                                    parent
                                        .spawn_bundle(PbrBundle {
                                            mesh: building.shape_data.mesh.clone().unwrap(),
                                            material: bp_material_handles.blueprint.clone(),
                                            transform: *transform,
                                            ..Default::default()
                                        })
                                        .insert_bundle((
                                            PipePreviewPlacement,
                                            NotShadowCaster,
                                            Placeable::Yes,
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn_bundle((
                                                building.shape_data.collider.clone(),
                                                transform.with_add_translation(
                                                    building.shape_data.collider_offset,
                                                ),
                                                Sensor(true),
                                                BuildingRotation(rot),
                                            ));
                                        });
                                })
                                .add_child(cbp_entity);

                            bc_res.rotation += PI;
                        }
                    }
                    _ => {
                        commands
                            .entity(cbp_collider_entity)
                            .remove_bundle::<(Moved, CursorBpCollider)>()
                            .insert_bundle((BLUEPRINT_COLLISION.clone(), Sensor(false)));

                        commands
                            .entity(cbp_entity)
                            .remove::<CursorBp>()
                            .insert(PlacedBlueprint {
                                cost: building.iridium_data.cost,
                                current: 0,
                            });
                        selected_building.id = None;
                    }
                }
            }
        }
    }

    if !pipe_preview.is_empty() {
        let pipe_cylinder = pipe_preview_cylinder.single();
        let first = pipe_placement.single();
        let second = cursor_bp.single();

        let mut intersecting = false;

        for entity in [pipe_cylinder, first, second] {
            if match placeable_query
                .get(entity)
                .unwrap_or(&Placeable::WithCollision)
            {
                Placeable::Yes => false,
                Placeable::WithCollision => {
                    children_query.get(entity).unwrap()[0].is_intersecting(&rapier_context)
                }
                Placeable::No => true,
            } {
                intersecting = true;
                break;
            }
        }

        let set_material = match intersecting {
            true => bp_material_handles.obstructed.clone(),
            false => bp_material_handles.blueprint.clone(),
        };

        material_query
            .get_many_mut([pipe_cylinder, first, second])
            .unwrap()
            .map(|mut material| *material = set_material.clone());

        let pipe_preview_entity = pipe_preview.single();

        if try_place_query.contains(pipe_preview_entity) && !intersecting {
            let place_mat = bp_fill_materials.get_fill_percent(0.0);
            let building_ref = &building_ref_query.get(pipe_preview_entity).unwrap().0;

            commands
                .entity(pipe_preview_entity)
                .remove::<PipePreview>()
                .insert(PipeBlueprint {
                    cost: building_ref.iridium_data.cost,
                    current: 0,
                });

            // First base of the pipe
            commands
                .entity(first)
                .remove::<PipePreviewPlacement>()
                .insert_bundle((PipeFirst, place_mat.clone()));

            // First base of the pipe's collider
            commands
                .entity(children_query.get(first).unwrap()[0])
                .insert_bundle((BLUEPRINT_COLLISION.clone(), Sensor(false)));

            // Second base of the pipe
            commands
                .entity(second)
                .remove::<CursorBp>()
                .insert_bundle((PipeSecond, place_mat.clone()));

            // Second base of the pipe's collider
            commands
                .entity(children_query.get(second).unwrap()[0])
                .remove::<CursorBpCollider>()
                .insert_bundle((BLUEPRINT_COLLISION.clone(), Sensor(false)));

            // Pipe cylinder in between both
            commands
                .entity(pipe_cylinder)
                .remove::<PipePreviewCylinder>()
                .insert_bundle((PipeCylinder, place_mat.clone()));

            // Pipe cylinder collider
            commands
                .entity(children_query.get(pipe_cylinder).unwrap()[0])
                .remove::<PipePreviewCylinderCollider>()
                .insert_bundle((BLUEPRINT_COLLISION.clone(), Sensor(false)));

            selected_building.id = None;
            selected_building.changed = true;
            global_pipe_id.0 += 1;
        }

        if try_place_query.contains(pipe_preview_entity) {
            commands.entity(pipe_preview_entity).remove::<TryPlace>();
        }
    }
}
