use bevy::prelude::{Component, Entity, Res, Transform};
use bevy_rapier3d::plugin::RapierContext;

use std::fmt::Debug;

use super::buildings::BuildingReferenceComponent;

/// Entities with this component will be deleted next frame
#[derive(Component)]
pub struct DeleteNextFrame;

/// The entity with this component is the first position of the pipe
#[derive(Component)]
pub struct PipePlacement {
    pub placed: bool,
    pub transform: Option<Transform>,
}

/// The entity with this component is the pipe cylinder between the two endpoints **before** it is placed
#[derive(Component)]
pub struct PipePreviewCylinder;

#[derive(Component)]
pub struct PipePreviewCylinderCollider;

#[derive(Component)]
pub struct PipePreviewPlacement;

#[derive(Component)]
pub struct PipePreview;

#[derive(Component)]
pub struct TestComponent;

/// Entities with this component are cursor blueprint preview things **before** they are placed
#[derive(Component)]
pub struct CursorBp;

/// Entities with this component are the collider attached to the cursor blueprint **before** they are placed
#[derive(Component)]
pub struct CursorBpCollider;

pub struct ChangeBuilding {
    pub b: bool,
}

/// Entities with this component are blueprints that have yet to be filled
#[derive(Component, Debug)]
pub struct PlacedBlueprint {
    pub cost: u32,
    pub current: u32,
}

#[derive(Component)]
pub struct PipeBlueprint {
    pub cost: u32,
    pub current: u32,
    // connections.. will add later
}

#[derive(Component)]
pub struct PipeFirst;

#[derive(Component)]
pub struct PipeSecond;

#[derive(Component)]
pub struct PipeCylinder;

/// Entities with this component have `true` when they are moved
#[derive(Component)]
pub struct Moved(pub bool);

#[derive(Component)]
pub struct Placeable(pub bool);

/// Adding this component to the cursor blueprint will mark it for trying to place it after collisions are calculated and checked
#[derive(Component)]
pub struct TryPlace;

#[derive(Component)]
pub struct BuiltPipeEnd;

#[derive(Component)]
pub struct BuildingRotation(pub f32);

impl Debug for BuildingReferenceComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BuildingReferenceComponent")
            .field(&self.0.building_id.building_type)
            .finish()
    }
}

pub trait IsColliding {
    /// Checks if `self` is intersecting in the given `RapierContext`
    fn is_intersecting(self, context: &Res<RapierContext>) -> bool;
}

impl IsColliding for Entity {
    fn is_intersecting(self, context: &Res<RapierContext>) -> bool {
        for (_, _, c) in context.intersections_with(self) {
            if c {
                return true;
            }
        }
        false
    }
}
