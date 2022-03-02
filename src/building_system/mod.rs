use bevy::{prelude::*};
use bevy_mod_raycast::{RayCastMethod, RayCastSource, Intersection};


use crate::{RaycastSet};

#[derive(Component)]
pub struct RaycastCursor {
    pub visible: bool
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
    mut rc_query: Query<&RaycastCursor> 
) {
    let mut intersections = Vec::new();

    for mut e in &mut r_query.iter_mut() {
        let f: &mut Vec<(Entity, Intersection)> = e.intersections_mut();
        intersections.append(f);
    }

    if !intersections.is_empty() {
        let (_, mut closest_intersection) = intersections.pop().unwrap();

        for (_, intersection) in intersections {
            if intersection.distance() < closest_intersection.distance() {
                closest_intersection = intersection;
            }
        }

        let vis = rc_query.get_single_mut();
        if vis.is_ok() {
            let d = d_query.get_single_mut();
            if vis.unwrap().visible {
                if d.is_ok() {
                    let (mut rc_cursor_transform, mut rc_cursor_visible) = d.unwrap();
                    rc_cursor_transform.translation = closest_intersection.position();
                    rc_cursor_visible.is_visible = true;
                    //info!("Pos: {:?}  Normal: {:?}", closest_intersection.position(), closest_intersection.normal());
                }
            } else {
                if d.is_ok() {
                    let (_, mut rc_cursor_visible) = d.unwrap();
                    rc_cursor_visible.is_visible = false;
                }
            }
        }
    }
}