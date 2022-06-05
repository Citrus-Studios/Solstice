use bevy::{
    gltf::GltfPrimitive, pbr::PbrBundle, prelude::*, render::render_resource::PrimitiveTopology,
};

use crate::{
    material_palette::{FlatMaterial, MaterialPalette},
    terrain_generation_system::{mutate_mesh::MutateMesh, relevant_attributes::RelevantAttributes},
};

pub fn translate_gltf_primitives(
    primitives: &Vec<GltfPrimitive>,
    meshes: &mut ResMut<Assets<Mesh>>,
    translation: Vec3,
) {
    for primitive in primitives {
        let mesh = meshes.get_mut(&primitive.mesh).unwrap();
        let mut attr = mesh.clone().relevant_attributes();

        attr.translate(translation);
        mesh.set_attributes(attr);
    }
}

/// Combines a vector of `GltfPrimitive`s into a single `Mesh` and `StandardMaterial`
pub fn combine_gltf_mesh(
    primitives: Vec<GltfPrimitive>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
) -> PbrBundle {
    let mut attr_vec = Vec::new();
    let mut material_num_vec = Vec::new();
    let mut material_palette = MaterialPalette::new();

    for primitive in primitives {
        let mesh = meshes.get(primitive.mesh).unwrap().clone();
        let flat_material: FlatMaterial = materials
            .get(primitive.material.clone().unwrap())
            .unwrap()
            .clone()
            .into();

        if material_palette.contains(&flat_material) {
            material_num_vec.push(material_palette.find(&flat_material).unwrap());
        } else {
            material_palette.push(flat_material);
            material_num_vec.push(material_palette.palette.len() - 1);
        }

        attr_vec.push(mesh.relevant_attributes());
    }

    info!("num materials: {}", material_palette.palette.len());

    let textures = material_palette.compile();
    let mut return_attr = RelevantAttributes::new();

    for (mut attr, mat_num) in attr_vec.into_iter().zip(material_num_vec) {
        attr.set_all_uv(textures.get_uv_pos(mat_num as u32));
        return_attr.append_with_indices(attr);
    }

    // info!("{:?}", return_attr);

    let mesh = meshes.add(return_attr.into());

    PbrBundle {
        mesh,
        material: materials.add(textures.into_standard_material(images)),
        ..Default::default()
    }
}

/// Like `combine_gltf_mesh()` but ignores materials
pub fn combine_gltf_primitives(
    primitives: Vec<GltfPrimitive>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> Mesh {
    let mut attr = RelevantAttributes::new();

    for primitive in primitives {
        let mesh = meshes.get(primitive.mesh).unwrap().clone();
        attr = attr.combine_with_mesh(mesh, Vec3::ZERO);
    }

    attr.into()
}
