use core::hash::Hash;

use std::{ops::Range, sync::Arc};

use bevy::{
    ecs::schedule::ShouldRun, gltf::GltfMesh, math::Vec3, pbr::StandardMaterial, prelude::*,
    utils::HashMap,
};
use bevy_rapier3d::prelude::Collider;

use crate::{constants::GLOBAL_PIPE_ID, model_loader::combine_gltf_mesh};

use lazy_static::lazy_static;

use super::{load_models::get_load_states, ModelHandles};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum BuildingType {
    Wellpump,
    Pipe,
}

#[derive(Clone)]
pub struct Building {
    pub building_id: BuildingId,
    pub iridium_data: BuildingIridiumData,
    pub shape_data: BuildingShapeData,
    pub snap_data: BuildingSnapData,
}

#[derive(Clone)]
pub struct BuildingId {
    pub building_type: BuildingType,
    pub building_name: String,
}

#[derive(Clone)]
pub struct BuildingIridiumData {
    pub io: BuildingIO,
    pub storage: Option<u32>,
    pub current: Option<u32>,
    pub generation: Option<u32>,
    pub cost: u32,
}

#[derive(Clone)]
pub enum BuildingIO {
    None,
    In,
    Out,
    InOut,
}

/// Contains a reference to a building
#[derive(Component)]
pub struct BuildingReferenceComponent(pub Arc<Building>);

#[derive(Clone)]
pub struct BuildingShapeData {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
    pub path: String,
    pub collider: Collider,
    pub collider_offset: Vec3,
}

#[derive(Clone)]
pub struct BuildingSnapData {
    /// Buildings that can snap to this building
    pub buildings: Vec<BuildingType>,

    /// Where those buildings can snap, can have multiple per snappable building
    ///
    /// (translation, axes of rotation)
    pub transform: Vec<Vec<(Vec3, Vec3)>>,

    pub rotation_allowed: Vec<Vec<Range<f32>>>,
}

/// Contains all of the buildings in a hashmap
pub struct BuildingsResource(pub HashMap<BuildingType, Building>);

/// Contains a reference to every building
pub struct BuildingArcs(pub HashMap<BuildingType, Arc<Building>>);

/// Building initialization done
pub struct BuildingInitDone(pub bool);

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
        unsafe {
            GLOBAL_PIPE_ID += 1;
        }
        Self {
            c1,
            c2,
            c3,
            c4,
            id: unsafe { GLOBAL_PIPE_ID },
        }
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
        Cost: $cost:literal,
        MeshPath: $meshtype:literal,
        Collider: $coll:expr
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
                cost: $cost,
            },
            shape_data: BuildingShapeData {
                mesh: None,
                material: None,
                path: $meshtype.to_string(),
                collider: $coll.coll,
                collider_offset: $coll.trans,
            },
            snap_data: BuildingSnapData {
                buildings: Vec::new(),
                transform: Vec::new(),
                rotation_allowed: Vec::new(),
            }
        }
    };

    (
        Type: $buildingtype:ident,
        Name: $name:literal,
        Flow: $flow:ident,
        Storage: $storage:expr,
        Current: $current:expr,
        Generation: $generation:expr,
        Cost: $cost:literal,
        MeshPath: $meshtype:literal,
        Collider: $coll:expr,
        Snapping:
        $((
            Building: $snap_building:expr,
            Positions: $($snap_position:expr),+;
            Axis: $($snap_axis:expr),+;
            RotationAllowed: $($snap_rotation:expr),+;
        )),+
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
                cost: $cost,
            },
            shape_data: BuildingShapeData {
                mesh: None,
                material: None,
                path: $meshtype.to_string(),
                collider: $coll.coll,
                collider_offset: $coll.trans,
            },
            snap_data: BuildingSnapData {
                buildings: vec![$($snap_building),+],
                transform: {
                    let pos = vec![$(vec![$($snap_position),+]);+];
                    let axis = vec![$(vec![$($snap_axis),+]);+];

                    let mut return_vec = Vec::new();

                    for (p, a) in pos.iter().zip(axis.iter()) {
                        return_vec.push(p.iter().copied().zip(a.iter().copied()).collect());
                    }

                    return_vec
                },
                rotation_allowed: vec![$(vec![$($snap_rotation),+]);+],
            }
        }
    }
}

impl BuildingShapeData {
    /// "Loads" the GLTF from the path and replaces the default `None`s in the struct with the stuff that's supposed to go there
    ///
    /// Assumes the path in .path is loaded and is NOT asynchronous
    pub fn load_from_path(
        &mut self,
        asset_server: &Res<AssetServer>,
        gltf_meshes: &ResMut<Assets<GltfMesh>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        images: &mut ResMut<Assets<Image>>,
    ) {
        let gltf_mesh: Handle<GltfMesh> =
            asset_server.load(&format!("{}{}", &self.path.clone(), "#Mesh0").to_string());
        let primitives = &gltf_meshes.get(gltf_mesh).unwrap().primitives;

        let bundle = combine_gltf_mesh(primitives.clone(), meshes, materials, images);

        self.mesh = Some(bundle.mesh);
        self.material = Some(bundle.material);
    }
}

pub fn load_buildings_into_resource(mut commands: Commands) {
    info!("into resource start");
    let mut hash = HashMap::with_capacity(2);

    hash.insert_no_return(
        BuildingType::Wellpump,
        Building!(
            Type: Wellpump,
            Name: "Well Pump",
            Flow: InOut,
            Storage: 50_00,
            Current: 0,
            Generation: 5_00,
            Cost: 100_00,
            MeshPath: "models/buildings/well_pump.gltf",
            Collider: WELLPUMP_COLLIDER.clone(),
            Snapping: (
                Building: BuildingType::Pipe,
                Positions: Vec3::new(0.0, 0.453448, 0.747605);
                Axis: Vec3::Y;
                RotationAllowed: 0.0..0.0;
            )

        ),
    )
    .insert_no_return(
        BuildingType::Pipe,
        Building!(
            Type: Pipe,
            Name: "Pipe",
            Flow: InOut,
            Storage: 0,
            Current: 0,
            Generation: 0,
            Cost: 10_00,
            MeshPath: "models/pipes/pipe_base.gltf",
            Collider: PIPE_COLLIDER.clone()
        ),
    );

    commands.insert_resource(BuildingsResource(hash));
    info!("into resource done");
}

pub fn load_buildings_in_resource(
    mut commands: Commands,

    mut buildings_res: ResMut<BuildingsResource>,
    mut building_init_done: ResMut<BuildingInitDone>,

    asset_server: Res<AssetServer>,
    gltf_meshes: ResMut<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    if building_init_done.0 {
        return;
    }

    let mut hash = HashMap::with_capacity(2);

    for (typ, building) in buildings_res.0.iter_mut() {
        building.shape_data.load_from_path(
            &asset_server,
            &gltf_meshes,
            &mut meshes,
            &mut materials,
            &mut images,
        );
        hash.insert_no_return(typ.clone(), Arc::new(building.clone()));
    }

    commands.insert_resource(BuildingArcs(hash));
    building_init_done.0 = true;
}

/// Tries to match `name` to a building
///
/// Panics if `name` does not match any building
pub fn string_to_building_enum(name: String) -> BuildingType {
    match name.as_str() {
        "Well Pump" => BuildingType::Wellpump,
        "Pipe" => BuildingType::Pipe,
        _ => panic!("Could not match \"{}\" to any building", name),
    }
}

pub fn building_init_done(b: Res<BuildingInitDone>) -> ShouldRun {
    if b.0 {
        ShouldRun::Yes
    } else {
        ShouldRun::NoAndCheckAgain
    }
}

pub fn building_init_not_done_and_get_load_states(
    b: Res<BuildingInitDone>,
    asset_server: Res<AssetServer>,
    model_handles: Res<ModelHandles>,
) -> ShouldRun {
    let load_state = get_load_states(asset_server, model_handles);
    if load_state == ShouldRun::NoAndCheckAgain {
        ShouldRun::NoAndCheckAgain
    } else if b.0 {
        ShouldRun::No
    } else {
        ShouldRun::YesAndCheckAgain
    }
}

#[derive(Clone)]
struct CollTransform {
    coll: Collider,
    trans: Vec3,
}

impl CollTransform {
    fn from_collider(coll: Collider) -> Self {
        CollTransform {
            coll,
            trans: Vec3::ZERO,
        }
    }

    fn with_translation(&mut self, trans: Vec3) -> Self {
        self.trans = trans;
        self.to_owned()
    }
}

trait InsertNoReturn<K, V> {
    fn insert_no_return(&mut self, k: K, v: V) -> &mut Self;
}

impl<K, V> InsertNoReturn<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn insert_no_return(&mut self, k: K, v: V) -> &mut Self {
        self.insert(k, v);
        self
    }
}

lazy_static! {
    static ref WELLPUMP_COLLIDER: CollTransform =
        CollTransform::from_collider(Collider::cylinder(1.11928 / 2.0, 0.89528))
            .with_translation(Vec3::new(0.0, 0.569639, -0.05));
    static ref PIPE_COLLIDER: CollTransform =
        CollTransform::from_collider(Collider::cuboid(0.27 / 2.0, 0.27 / 2.0, 0.3025 / 2.0))
            .with_translation(Vec3::new(0.0, 0.25, 0.01625));
}
