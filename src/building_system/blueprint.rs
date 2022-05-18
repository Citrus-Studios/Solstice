use bevy::prelude::*;
use bevy_rapier3d::prelude::ActiveCollisionTypes;

use crate::player_system::gui_system::gui_startup::SelectedBuilding;

use super::{raycasting::BuildCursor, building_components::PlacedBlueprint, building::EntityQuery, BlueprintFillMaterial};

pub fn update_blueprints(
    blueprint_query: EntityQuery<PlacedBlueprint>,

    children_query: Query<&Children>,
    mut act_query: Query<&mut ActiveCollisionTypes>,
    mut material_query: Query<&mut Handle<StandardMaterial>>,
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

            clicked_blueprint.current += 0_84;
            clicked_blueprint.current = clicked_blueprint.current.min(clicked_blueprint.cost);

            *material = bp_fill_materials.get_bp_fill_material(clicked_blueprint.current, clicked_blueprint.cost);
        }
    }
}