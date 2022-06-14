use bevy::{asset::LoadState, gltf::GltfMesh, prelude::*, utils::HashMap};

use super::compound_collider_builder::CompoundColliderBuilder;

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TerrainBlockName(pub &'static str);

#[derive(Clone)]
pub struct TerrainBlockData {
    pub path: &'static str,
    pub collider: CompoundColliderBuilder,
    pub model: GltfMesh,
}

pub struct Blocks {
    pub hash: HashMap<&'static str, TerrainBlockData>,
}

impl Blocks {
    pub fn new() -> Self {
        Blocks {
            hash: HashMap::new(),
        }
    }

    pub fn get(&self, block_name: &'static str) -> Option<&TerrainBlockData> {
        self.hash.get(block_name)
    }

    pub fn add(
        &mut self,
        path: &'static str,
        collider: CompoundColliderBuilder,
        asset_server: &Res<AssetServer>,
        gltf_meshes: &Res<Assets<GltfMesh>>,
    ) -> Result<(), LoadState> {
        let model;
        let load_state = asset_server.get_load_state(path);
        match load_state {
            LoadState::Loaded => {
                model = gltf_meshes.get(path).unwrap().clone();
            }
            e => return Result::Err(e),
        }

        let name = path.rsplit('/').next().unwrap().split('.').next().unwrap();
        info!("{name}");
        self.hash.insert(
            name,
            TerrainBlockData {
                path,
                collider,
                model,
            },
        );

        Result::Ok(())
    }
}
