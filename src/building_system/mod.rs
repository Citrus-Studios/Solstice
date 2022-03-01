use bevy::{prelude::*, render::primitives::Aabb, ecs::query::QueryIter};
use bevy_mod_raycast::{RayCastMethod, RayCastSource, RayCastMesh, Intersection, ray_mesh_intersection, RaycastSystem};


use crate::{player::CameraComp, RaycastSet};

#[derive(Component)]
pub struct RaycastCursor;

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
    mut d_query: Query<&mut Transform, With<RaycastCursor>>,
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

        let mut rc_cursor = d_query.single_mut();
        rc_cursor.translation = closest_intersection.position() + (closest_intersection.normal() * 0.2);

        //info!("Pos: {:?}  Normal: {:?}", closest_intersection.position(), closest_intersection.normal());


    }


}




// pub fn cast_mouse(
//     ignored_entities: Query<()>,
//     c_query: Query<(&CameraComp, &Transform)>,
//     mut events: EventReader<PickingEvent>
// ) {
    
//     let (camera, c_transform) = c_query.single();
//     let c_translation = c_transform.translation.clone();

//     for event in events.iter() {
//         match event {
//             PickingEvent::Hover(e) => info!("{:?}", e),
//             _ => ()
//         }
//     }
// }