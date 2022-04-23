use bevy::{pbr::PbrBundle, asset::{AssetPath, Asset}, prelude::*, gltf::{Gltf, GltfMesh, GltfPrimitive}};

use crate::{material_palette::MaterialPalette, terrain_generation_system::{relevant_attributes::RelevantAttributes, mutate_mesh::MutateMesh}};

pub fn combine_gltf_mesh(primitives: Vec<GltfPrimitive>, meshes: Res<Assets<Mesh>>, materials: Res<Assets<StandardMaterial>>) -> PbrBundle {
    let mut attr = RelevantAttributes::new();
    let mut material_palette = MaterialPalette::new();

    for primitive in primitives {
        let mesh = meshes.get(primitive.mesh).unwrap().clone();
        let material = materials.get(primitive.material.unwrap()).unwrap().clone();

        attr.append_with_indices(mesh.relevant_attributes());
        material_palette.push(material.into());
    }

    PbrBundle::default()
}