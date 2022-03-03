pub use bevy::{prelude::*};

use super::RaycastCursor;

pub fn visualizer(
    rc_query: Query<&RaycastCursor>,
    asset_server: Res<AssetServer>
) {
    let intersection_op = rc_query.single().intersection;
    
    if intersection_op.is_some() {
        let intersection = intersection_op.unwrap();
        let pipe_model: Handle<Mesh> = asset_server.load("models/pipes/pipe_base.obj");
    }
}