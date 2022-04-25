use std::{thread::sleep, time::Duration};

use bevy::{prelude::*, gltf::GltfMesh, asset::LoadState, ecs::schedule::ShouldRun};

const NUM_MODELS: usize = 1;

const MODEL_PATHS: [&'static str; NUM_MODELS] = [
    "models/buildings/well_pump.gltf",
];

static mut MODEL_HANDLES: [Option<Handle<GltfMesh>>; NUM_MODELS] = [None; NUM_MODELS];

pub fn initiate_load(asset_server: Res<AssetServer>) {
    for (i, path) in MODEL_PATHS.iter().enumerate() {
        let e = Some(asset_server.load(&format!("{}{}", path, "#Mesh0")));
        unsafe { MODEL_HANDLES[i] = e; }
    }
}

pub fn get_load_states(asset_server: Res<AssetServer>) -> ShouldRun {
    for handle in unsafe { MODEL_HANDLES.clone() } {
        if asset_server.get_load_state(handle.unwrap()) != LoadState::Loaded {
            return ShouldRun::NoAndCheckAgain
        }
    }
    return ShouldRun::Yes
}