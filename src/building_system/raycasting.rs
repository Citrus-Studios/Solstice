use bevy::{prelude::*};
use bevy_mod_raycast::{RayCastMethod, RayCastSource, Intersection};


use crate::{RaycastSet};

#[derive(Component)]
pub struct RaycastCursor {
    pub visible: bool
}

#[derive(Component)]
pub struct BuildCursor {
    pub intersection: Option<Intersection>,
    pub rotation: f32,
    pub entity: Option<Entity>,
}

pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<RaycastSet>>,
) {
    for mut pick_source in &mut query.iter_mut() {
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
        }
    }
}

pub fn raycast(
    mut r_query: Query<&mut RayCastSource<RaycastSet>>,
    mut d_query: Query<(&mut Transform, &mut Visibility), With<RaycastCursor>>,
    mut rc_query: Query<&mut RaycastCursor>,
    mut bc_res: ResMut<BuildCursor>,

    keyboard_input: Res<Input<KeyCode>>
) {
    let mut intersections = Vec::new();

    for mut e in &mut r_query.iter_mut() {
        let f: &mut Vec<(Entity, Intersection)> = e.intersections_mut();
        intersections.append(f);
    }

    let mut rcc = rc_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::K) {
        rcc.visible = (1 - rcc.visible as i8) != 0 ;
    }

    let d = d_query.get_single_mut();

    if !intersections.is_empty() {
        let (mut closest_e, mut closest_intersection) = intersections.pop().unwrap();

        for (e, intersection) in intersections {
            if intersection.distance() < closest_intersection.distance() {
                closest_intersection = intersection;
                closest_e = e;
            }
        }
        
        bc_res.intersection = Some(closest_intersection);
        bc_res.entity = Some(closest_e);

        if rcc.visible {
            if d.is_ok() {
                let (mut rc_cursor_transform, mut rc_cursor_visible) = d.unwrap();

                rc_cursor_transform.translation = closest_intersection.position();
                rc_cursor_visible.is_visible = true;
            }
        } else {
            if d.is_ok() {
                let (_, mut rc_cursor_visible) = d.unwrap();

                rc_cursor_visible.is_visible = false;
            }
        }
    } else {
        bc_res.intersection = None;
        
        if d.is_ok() {
            let (_, mut rc_cursor_visible) = d.unwrap();

            rc_cursor_visible.is_visible = false;
        }
    }
}