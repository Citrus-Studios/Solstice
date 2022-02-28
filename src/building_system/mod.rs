use bevy::{prelude::*, render::primitives::Aabb, ecs::query::QueryIter};
use bevy_mod_raycast::{RayCastMethod, RayCastSource, RayCastMesh, Intersection, ray_mesh_intersection, RaycastSystem};


use crate::{player::CameraComp, RaycastSet};

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
    mut query: Query<&mut RayCastSource<RaycastSet>>,
) {
    // for event in events.iter() {
    //     info!("{:?}", event.position());
    // }


    // this is a mess, help
    
    let mut intersections = Vec::new();

    for e in &mut query.iter_mut() {
        let &mut f = match e.intersect_list() {
            Some(g) => g,
            _ => &Vec::new()
        };

        if !f.is_empty() {
            intersections.append(&mut *f);
        }
    }

    info!("{:?}", intersections);
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