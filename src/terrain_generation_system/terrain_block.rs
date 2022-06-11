use bevy::{gltf::GltfMesh, prelude::*};

use super::compound_collider_builder::CompoundColliderBuilder;

#[derive(Component, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TerrainBlockType {
    Solid,
    Hollow,
    SpireSolid,
    SpireHollow,
    Well,
}

#[derive(Clone)]
pub struct TerrainBlockData {
    pub collider: CompoundColliderBuilder,
    pub model: GltfMesh,
}
