use bevy::{prelude::*, gltf::GltfMesh, asset::LoadState, ecs::schedule::ShouldRun};

use super::ModelHandles;

pub const NONE_HANDLE: Option<Handle<GltfMesh>> = None;
pub const NUM_MODELS: usize = 2;

const MODEL_PATHS: [&'static str; NUM_MODELS] = [
    "models/buildings/well_pump.gltf",
    "models/pipes/pipe_base.gltf",
];

pub fn initiate_load(asset_server: Res<AssetServer>, mut model_handles: ResMut<ModelHandles>) {
    info!("load start");
    for (i, path) in MODEL_PATHS.iter().enumerate() {
        let e = Some(asset_server.load(&format!("{}{}", path, "#Mesh0")));
        model_handles.handles[i] = e;
    }
    info!("load done");
}

pub fn get_load_states(asset_server: Res<AssetServer>, model_handles: Res<ModelHandles>) -> ShouldRun {
    for handle in model_handles.handles.clone() {
        if asset_server.get_load_state(handle.unwrap()) != LoadState::Loaded {
            return ShouldRun::NoAndCheckAgain
        }
    }
    return ShouldRun::Yes
}