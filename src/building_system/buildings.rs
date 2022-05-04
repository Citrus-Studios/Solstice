use std::marker::PhantomData;

use bevy::{prelude::{Mesh, Handle, Scene, Res, Assets, AssetServer, ResMut, Image}, math::Vec3, pbr::StandardMaterial, gltf::GltfMesh};

use crate::{constants::GLOBAL_PIPE_ID, model_loader::{combine_gltf_mesh, combine_gltf_primitives}};

pub enum BuildingType {
    Wellpump,
    Pipe,
}

pub struct Building {
    pub building_id: BuildingId,
    pub iridium_data: BuildingIridiumData,
    pub shape_data: BuildingShapeData,
}

pub struct BuildingId {
    pub building_type: BuildingType,
    pub building_name: String,
}

pub struct BuildingIridiumData {
    pub io: BuildingIO,
    pub storage: Option<u32>,
    pub current: Option<u32>,
    pub generation: Option<u32>,
}

pub enum BuildingIO {
    None,
    In,
    Out,
    InOut,
}

#[derive(Clone)]
pub struct BuildingShapeData {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
    pub path: String,
    // Will be converted to a trimesh collider
    pub collider: Option<Mesh>,
    pub collider_handle: Option<Handle<Mesh>>,
    pub collider_offset: Vec3,
}

pub enum Or<A, B> {
    A(A),
    B(B),
    None,
}

pub struct Pipe<T, U, V, W> {
    pub c1: Or<T, Building>,
    pub c2: Or<U, Building>,
    pub c3: Or<V, Building>,
    pub c4: Or<W, Building>,
    pub id: u32,
}

impl<T, U, V, W> Pipe<T, U, V, W> {
    pub fn new(
        c1: Or<T, Building>,
        c2: Or<U, Building>,
        c3: Or<V, Building>,
        c4: Or<W, Building>,
    ) -> Self {
        unsafe { GLOBAL_PIPE_ID += 1; }
        Self {
            c1,
            c2,
            c3,
            c4,
            id: unsafe { GLOBAL_PIPE_ID },
        }
    }
}

impl BuildingShapeData {
    pub fn load_from_path(&mut self, 
        asset_server: &Res<AssetServer>, 
        gltf_meshes: &ResMut<Assets<GltfMesh>>, 
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        images: &mut ResMut<Assets<Image>>,
    ) {
        let gltf_mesh: Handle<GltfMesh> = asset_server.load(&format!("{}{}", &self.path.clone(), "#Mesh0").to_string());
        let primitives = &gltf_meshes.get(gltf_mesh).unwrap().primitives;

        let bundle = combine_gltf_mesh(primitives.clone(), meshes, materials, images);

        self.mesh = Some(bundle.mesh);
        self.material = Some(bundle.material);

        let gltf_mesh: Handle<GltfMesh> = asset_server.load(&format!("{}{}", &self.path.clone(), "#Mesh1").to_string());
        let primitives = &gltf_meshes.get(gltf_mesh).unwrap().primitives;

        let mesh = combine_gltf_primitives(primitives.clone(), meshes);

        self.collider = Some(mesh.clone());
        self.collider_handle = Some(meshes.add(mesh));
    }
}

macro_rules! Building {
    (
        Type: $buildingtype:ident, 
        Name: $name:literal, 
        Flow: $flow:ident, 
        Storage: $storage:expr, 
        Current: $current:expr, 
        Generation: $generation:expr,
        Mesh: $mesh:expr,
        Material: $material:expr,
        MeshPath: $meshtype:literal, 
        Collider: $collider:expr, 
        ColliderHandle: $colliderhandle:expr,
        Offset: ($x:literal, $y:literal, $z:literal)
    ) => {
        Building {
            building_id: BuildingId {
                building_type: BuildingType::$buildingtype,
                building_name: $name.to_string(),
            },
            iridium_data: BuildingIridiumData {
                io: BuildingIO::$flow,
                storage: match $storage {
                    -1 => None,
                    _ => Some($storage as u32)
                },
                current:  match $current {
                    -1 => None,
                    _ => Some($current as u32)
                },
                generation:  match $generation {
                    -1 => None,
                    _ => Some($generation as u32)
                },
            },
            shape_data: BuildingShapeData {
                mesh: $mesh,
                material: $material,
                path: $meshtype.to_string(),
                collider: $collider,
                collider_handle: $colliderhandle,
                collider_offset: Vec3::new($x, $y, $z),
            },
        }
    }
}

pub fn string_to_building(name: String) -> Building {
    match name.as_str() {
        "Well Pump" => Building!(
            Type: Wellpump, 
            Name: "Well Pump", 
            Flow: InOut, 
            Storage: 50, 
            Current: 0, 
            Generation: 5, 
            Mesh: None, 
            Material: None,
            MeshPath: "models/buildings/well_pump.gltf", 
            Collider: None, 
            ColliderHandle: None,
            Offset: (0.0, 0.0, 0.0)
        ),
        "Pipe" => Building!(
            Type: Pipe, 
            Name: "Pipe", 
            Flow: InOut, 
            Storage: 0, 
            Current: 0, 
            Generation: 0, 
            Mesh: None, 
            Material: None,
            MeshPath: "models/pipes/pipe_base.gltf", 
            Collider: None, 
            ColliderHandle: None,
            Offset: (0.0, 0.0, 0.0)
        ),
        _ => panic!("Could not match {} to any building", name)
    }
}