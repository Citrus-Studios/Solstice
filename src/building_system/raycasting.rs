use bevy::prelude::*;
use bevy_mod_raycast::Ray3d;
use bevy_rapier3d::{plugin::RapierContext, prelude::{InteractionGroups, RayIntersection}, math::Real};


use crate::player_system::player::CameraComp;

#[derive(Component)]
pub struct RaycastCursor {
    pub visible: bool
}

#[derive(Component)]
pub struct BuildCursor {
    pub intersection: Option<(Entity, RayIntersection)>,
    pub rotation: f32,
}

pub struct LatestCursorPosition(pub Option<Vec2>);

pub fn raycast(
    mut bc_res: ResMut<BuildCursor>,

    mut cursor: EventReader<CursorMoved>,
    camera_transform_q: Query<&GlobalTransform, With<CameraComp>>,
    camera_q: Query<&Camera, With<CameraComp>>,
    rapier_context: Res<RapierContext>,

    windows: Res<Windows>,
    images: Res<Assets<Image>>,
    mut latest: ResMut<LatestCursorPosition>,
) {
    let cursor_pos_op = cursor.iter().last();
    match cursor_pos_op {
        Some(e) => latest.0 = Some(e.position),
        None => (),
    }

    let cursor_pos = match latest.0 { Some(e) => e, _ => return };

    let ray = Ray3d::from_screenspace(
        cursor_pos,
        &windows,
        &images,
        camera_q.single(),
        camera_transform_q.single()
    ).unwrap();

    let intersection = rapier_context.cast_ray_and_get_normal(
        ray.origin(), 
        ray.direction(), 
        Real::MAX, 
        true, 
        InteractionGroups {
            memberships: 0b111111000,
            filter: 0b1111100101,
        }, 
        None
    );

    bc_res.intersection = intersection;
}