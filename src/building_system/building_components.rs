use bevy::prelude::{Component, Transform, Res, Entity};
use bevy_rapier3d::plugin::RapierContext;

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
    pub b: bool
}

/// Entities with this component are blueprints that have yet to be filled
#[derive(Component)]
pub struct PlacedBlueprint {
    pub cost: u32,
    pub current: u32,
}

/// Entities with this component have `true` when they are moved
#[derive(Component)]
pub struct Moved(pub bool);

pub trait IsColliding {
    /// Checks if `self` is intersecting in the given `RapierContext`
    fn is_intersecting(self, context: &Res<RapierContext>) -> bool;
}

impl IsColliding for Entity {
    fn is_intersecting(self, context: &Res<RapierContext>) -> bool {
        for (_, _, c) in context.intersections_with(self) {
            if c { return true }
        }
        false
    }
}