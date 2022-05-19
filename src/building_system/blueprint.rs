use bevy::prelude::*;
use bevy_rapier3d::prelude::CollisionGroups;

use crate::{player_system::gui_system::gui_startup::SelectedBuilding, constants::FABRICATOR_SPEED};

use super::{raycasting::BuildCursor, building_components::PlacedBlueprint, BlueprintFillMaterial, buildings::BuildingReferenceComponent};

const FABRICATOR_PER_SEC: u32 = (FABRICATOR_SPEED * 100) / 30;

pub fn update_blueprints(
    children_query: Query<&Children>,
    mut groups_query: Query<&mut CollisionGroups>,
    mut material_query: Query<&mut Handle<StandardMaterial>>,
    building_ref_query: Query<&BuildingReferenceComponent>,
    mut pb_query: Query<&mut PlacedBlueprint>,

    build_cursor_res: Res<BuildCursor>,
    selected_building: Res<SelectedBuilding>,
    (mouse_input, keyboard_input): (Res<Input<MouseButton>>, Res<Input<KeyCode>>),
    bp_fill_materials: Res<BlueprintFillMaterial>,
) {
    if build_cursor_res.intersection.is_some() && selected_building.id.is_none() && mouse_input.pressed(MouseButton::Left) { // Add portafab selected bool
        let entity = build_cursor_res.entity.unwrap();
        let clicked_blueprint_result = pb_query.get_mut(entity);
        if clicked_blueprint_result.is_ok() {
            let mut clicked_blueprint = clicked_blueprint_result.unwrap();
            let mut material = material_query.get_mut(entity).unwrap();

            clicked_blueprint.current += FABRICATOR_PER_SEC;

            if clicked_blueprint.current >= clicked_blueprint.cost {
                *material = building_ref_query.get(entity).unwrap().0.shape_data.material.clone().unwrap();

                let mut collision_groups = groups_query.get_mut(children_query.get(entity).unwrap()[0]).unwrap();
                *collision_groups = CollisionGroups::default();
            } else {
                *material = bp_fill_materials.get_bp_fill_material(clicked_blueprint.current, clicked_blueprint.cost);
            }
        }
    }
}