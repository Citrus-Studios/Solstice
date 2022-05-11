use std::ops::Add;

use bevy::{prelude::*, pbr::NotShadowCaster};
use bevy_mod_raycast::{SimplifiedMesh, RayCastMesh};
use bevy_rapier3d::{plugin::RapierContext, prelude::*};

use super::{MaterialHandles, building_components::*, buildings::BuildingShapeData, RaycastSet};

pub fn check_pipe_collision(e: Entity, context: Res<RapierContext>) -> bool {
    for (_, _, c) in context.intersections_with(e) {
        if c {
            return true
        }
    }
    return false
}

// TODO: collision
pub fn spawn_cursor_bp(
    commands: &mut Commands, 
    mesh: Handle<Mesh>, 
    bp_materials: &ResMut<MaterialHandles>, 
    collider: Collider, 
    collider_offset: Vec3, 
    transform: Transform,
) {    
    commands.spawn_bundle(PbrBundle {
        mesh,
        material: bp_materials.blueprint.clone().unwrap(),
        transform,
        ..Default::default()
    })
    .insert(NotShadowCaster)
    .insert(CursorBp)
    .with_children(|parent| {
        parent.spawn()
            .insert(collider)
            .insert(transform.with_add_translation(collider_offset)) // bevy-rapier issue, should be fixed later
            .insert(Sensor(true))
            .insert(ActiveCollisionTypes::all())
            .insert(CursorBpCollider)
            .insert(Moved(true))
        ;
    });
}

// hi lemon
pub fn move_cursor_bp(
    mut transform: Mut<Transform>,
    mut collider_transform: Mut<Transform>,
    collider_offset: Vec3,
    new_transform: Transform,
    mut moved: &mut Moved,
) {
    let trans = transform.as_mut();
    *trans = new_transform;

    let coll_trans = collider_transform.as_mut();
    *coll_trans = new_transform.with_add_translation(collider_offset);

    moved.0 = true;
}

// TODO: everything
pub fn spawn_bp(commands: &mut Commands, shape_data: BuildingShapeData, cost: u32, transform: Transform) {
    commands.spawn_bundle(PbrBundle {
        mesh: shape_data.mesh.unwrap(),
        material: shape_data.material.unwrap(),
        transform,
        ..Default::default()
    })
    .insert(SimplifiedMesh {
        mesh: shape_data.simplified_mesh_handle.unwrap(),
    })
    .insert(RayCastMesh::<RaycastSet>::default())
    .insert(PlacedBlueprint {
        cost,
        current: 0,
    })
    .with_children(|parent| {
        parent.spawn()
            .insert(shape_data.collider)
            .insert(transform.with_add_translation(shape_data.collider_offset)) // bevy-rapier issue, should be fixed later
        ;
    })
    ;
}

trait MoveTransform {
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