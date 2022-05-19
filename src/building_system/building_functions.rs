use std::{ops::Add, sync::Arc};

use bevy::{prelude::*, pbr::NotShadowCaster};
use bevy_mod_raycast::{SimplifiedMesh, RayCastMesh};
use bevy_rapier3d::{plugin::RapierContext, prelude::*};

use super::{MaterialHandles, building_components::*, buildings::{BuildingShapeData, Building, BuildingReferenceComponent}, RaycastSet};

/// Deprecated I think
pub fn check_pipe_collision(e: Entity, context: Res<RapierContext>) -> bool {
    for (_, _, c) in context.intersections_with(e) {
        if c {
            return true
        }
    }
    return false
}

/// Spawns the cursor blueprint preview thing
// TODO: collision
pub fn spawn_cursor_bp(
    commands: &mut Commands,
    building_arc: Arc<Building>,
    mesh: Handle<Mesh>, 
    bp_materials: &ResMut<MaterialHandles>, 
    collider: Collider, 
    collider_offset: Vec3, 
    transform: Transform,
) -> Entity {    
    commands.spawn_bundle(PbrBundle {
        mesh,
        material: bp_materials.blueprint.clone(),
        transform,
        ..Default::default()
    })
    .insert(NotShadowCaster)
    .insert(CursorBp)
    .insert(BuildingReferenceComponent(building_arc))
    .with_children(|parent| {
        parent.spawn()
            .insert(collider)
            .insert(transform.with_add_translation(collider_offset)) // bevy-rapier issue, should be fixed later
            .insert(Sensor(true))
            .insert(ActiveCollisionTypes::all())
            .insert(CursorBpCollider)
            .insert(Moved(true))
        ;
    }).id()
}

/// Moves the cursor blueprint preview thing
// hi lemon
pub fn move_cursor_bp(
    transform: &mut Transform,
    collider_transform: &mut Transform,
    collider_offset: Vec3,
    new_transform: Transform,
    mut moved: &mut Moved,
) {
    *transform = new_transform;
    *collider_transform = new_transform.with_add_translation(collider_offset);

    moved.0 = true;
}

pub trait MoveTransform {
    /// Copies `self` and returns it with the added translation (`t`), rotated by `self`'s `rotation`.
    fn with_add_translation(&self, t: Vec3) -> Self;
}

impl MoveTransform for Transform {
    fn with_add_translation(&self, t: Vec3) -> Self {
        let rotated_translation = self.rotation.mul_vec3(t);
        let return_transform = self.to_owned();
        return_transform.with_translation(self.translation.add(rotated_translation))
    }
}

