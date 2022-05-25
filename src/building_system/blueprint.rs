use bevy::{prelude::*, pbr::NotShadowCaster};
use bevy_rapier3d::prelude::CollisionGroups;

use crate::{player_system::gui_system::gui_startup::SelectedBuilding, constants::FABRICATOR_SPEED};

use super::{raycasting::BuildCursor, building_components::*, BlueprintFillMaterial, buildings::BuildingReferenceComponent, building::EntityQuery, PipeCylinderMaterial};

const FABRICATOR_PER_UPDATE: u32 = (FABRICATOR_SPEED * 100) / 30;

pub fn update_blueprints(
    mut commands: Commands,

    pipe_cylinder_query: EntityQuery<PipeCylinder>,

    children_query: Query<&Children>,
    mut groups_query: Query<&mut CollisionGroups>,
    mut material_query: Query<&mut Handle<StandardMaterial>>,
    building_ref_query: Query<&BuildingReferenceComponent>,
    mut pb_query: Query<&mut PlacedBlueprint>,
    mut pipe_bp_query: Query<&mut PipeBlueprint>,
    parent_query: Query<&Parent>,

    build_cursor_res: Res<BuildCursor>,
    selected_building: Res<SelectedBuilding>,
    (mouse_input, keyboard_input): (Res<Input<MouseButton>>, Res<Input<KeyCode>>),
    bp_fill_materials: Res<BlueprintFillMaterial>,
    pipe_cylinder_material: Res<PipeCylinderMaterial>,
) {
    if build_cursor_res.intersection.is_some() && selected_building.id.is_none() && mouse_input.pressed(MouseButton::Left) { // Add portafab selected bool
        let entity = build_cursor_res.intersection.unwrap().0;

        // The collider is a child of the actual entity with a mesh
        let parent_op = parent_query.get(entity);
        let parent = match parent_op {
            Ok(e) => e.0,
            Err(_) => { return },
        };

        let clicked_blueprint_result = pb_query.get_mut(parent);

        if clicked_blueprint_result.is_ok() {
            let mut clicked_blueprint = clicked_blueprint_result.unwrap();
            let mut material = material_query.get_mut(parent).unwrap();

            clicked_blueprint.current += FABRICATOR_PER_UPDATE;

            if clicked_blueprint.current >= clicked_blueprint.cost {
                *material = building_ref_query.get(parent).unwrap().0.shape_data.material.clone().unwrap();

                let mut collision_groups = groups_query.get_mut(entity).unwrap();
                *collision_groups = CollisionGroups::default();

                commands.entity(parent).remove_bundle::<(PlacedBlueprint, NotShadowCaster)>();
            } else {
                *material = bp_fill_materials.get_bp_fill_material(clicked_blueprint.current, clicked_blueprint.cost);
            }
        }

        let pipe_bp_result = parent_query.get(parent);

        let clicked_pipe_result = match pipe_bp_result {
            Ok(e) => pipe_bp_query.get_mut(e.0),
            Err(e) => Result::Err(e),
        };

        // dis means you clicked a pipe
        if clicked_pipe_result.is_ok() {
            let pipe_bp = pipe_bp_result.unwrap().0;
            let mut clicked_pipe = clicked_pipe_result.unwrap();
            clicked_pipe.current += FABRICATOR_PER_UPDATE;

            let pipe_parts = children_query.get(pipe_bp).unwrap();

            if clicked_pipe.current >= clicked_pipe.cost {
                let pipe_base_mat = building_ref_query.get(pipe_bp).unwrap().0.shape_data.material.clone().unwrap();

                for part in pipe_parts.iter() {
                    let mut mat = material_query.get_mut(*part).unwrap();
                    if pipe_cylinder_query.contains(*part) {
                        *mat = pipe_cylinder_material.0.clone();

                        commands.entity(*part)
                            .remove::<PipeCylinder>()
                            .insert(CollisionGroups::default())
                        ;

                        commands.entity(children_query.get(*part).unwrap()[0])
                            .insert(CollisionGroups::default())
                        ;
                    } else {
                        *mat = pipe_base_mat.clone();

                        // Remove useless marker components and activate collision
                        commands.entity(*part)
                            .remove_bundle::<(PipeFirst, PipeSecond)>()
                        ;

                        commands.entity(children_query.get(*part).unwrap()[0])
                            .insert(CollisionGroups::default())
                        ;
                    }
                }

                commands.entity(pipe_bp).remove::<PipeBlueprint>();
            } else {
                let material_set = bp_fill_materials.get_bp_fill_material(clicked_pipe.current, clicked_pipe.cost);

                for part in pipe_parts.iter() {
                    let mut mat = material_query.get_mut(*part).unwrap();
                    *mat = material_set.clone();
                }
            }
        }
    }
}