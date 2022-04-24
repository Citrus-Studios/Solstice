use bevy::{pbr::PbrBundle, prelude::*, gltf::GltfPrimitive, render::render_resource::PrimitiveTopology};

use crate::{material_palette::MaterialPalette, terrain_generation_system::{relevant_attributes::RelevantAttributes, mutate_mesh::MutateMesh}};

pub fn combine_gltf_mesh(primitives: Vec<GltfPrimitive>, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, images: &mut ResMut<Assets<Image>>) -> PbrBundle {
    let mut attr_vec = Vec::new();
    let mut material_palette = MaterialPalette::new();

    for primitive in primitives {
        let mesh = meshes.get(primitive.mesh).unwrap().clone();
        let material = materials.get(primitive.material.unwrap()).unwrap().clone();

        attr_vec.push(mesh.relevant_attributes());
        material_palette.push(material.into());
    }

    let textures = material_palette.compile(None);
    let mut return_attr = RelevantAttributes::new();

    for (i, mut attr) in attr_vec.into_iter().enumerate() {
        attr.set_all_uv(textures.get_uv_pos(i as u32));
        return_attr.append_with_indices(attr);
    }

    let mesh = meshes.add(Mesh::new(PrimitiveTopology::TriangleList).set_attributes(return_attr));

    PbrBundle {
        mesh,
        material: materials.add(textures.into_standard_material(images)),
        ..Default::default()
    }
}