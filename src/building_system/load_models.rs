use bevy::{asset::LoadState, ecs::schedule::ShouldRun, gltf::GltfMesh, prelude::*};

use super::ModelHandles;

pub const NONE_HANDLE: Option<Handle<GltfMesh>> = None;
pub const NUM_MODELS: usize = 7;

const MODEL_PATHS: [&'static str; NUM_MODELS] = [
    "models/buildings/well_pump.gltf",
    "models/pipes/pipe_base.gltf",
    "models/ground1/ground1.gltf",
    "models/ground1/hollow_ground.gltf",
    "models/ground1/spires_full.gltf",
    "models/ground1/spires_hollow.gltf",
    "models/ground1/well_ground.gltf",
];

pub fn initiate_load(asset_server: Res<AssetServer>, mut model_handles: ResMut<ModelHandles>) {
    info!("load start");
    for (i, path) in MODEL_PATHS.iter().enumerate() {
        let e = Some(asset_server.load(&format!("{}{}", path, "#Mesh0")));
        model_handles.handles[i] = e;
    }
    info!("load done");
}

pub fn get_load_states(
    asset_server: Res<AssetServer>,
    model_handles: Res<ModelHandles>,
) -> ShouldRun {
    for handle in model_handles.handles.clone() {
        match asset_server.get_load_state(handle.as_ref().unwrap()) {
            LoadState::Loaded => continue,
            LoadState::Failed => warn!("Failed to load model, Handle: {:?}", handle.unwrap()),
            _ => return ShouldRun::NoAndCheckAgain,
        }
    }
    return ShouldRun::Yes;
}
