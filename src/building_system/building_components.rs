use bevy::prelude::{Component, Transform, Res, Entity};
use bevy_rapier3d::plugin::RapierContext;

#[derive(Component)]
pub struct DeleteNextFrame;

#[derive(Component)]
pub struct PipePlacement {
    pub placed: bool,
    pub transform: Option<Transform>,
}
// soidhfoisd
#[derive(Component)]
pub struct PipePreview;

#[derive(Component)]
pub struct TestComponent;

#[derive(Component)]
pub struct CursorBp;

#[derive(Component)]
pub struct CursorBpCollider;

pub struct ChangeBuilding {
    pub b: bool
}

#[derive(Component)]
pub struct PlacedBlueprint {
    pub cost: u32,
    pub current: u32,
}

#[derive(Component)]
pub struct Moved(pub bool);

pub trait IsColliding {
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