use crate::building_system::building::building;
use crate::building_system::snapping::snapping;
use bevy::{
    core::FixedTimestep,
    gltf::GltfMesh,
    pbr::{AlphaMode, PbrBundle, StandardMaterial},
    prelude::{
        default, info, shape, Assets, Color, Commands, CoreStage, Handle, Mesh,
        ParallelSystemDescriptorCoercion, Plugin, ResMut, SystemSet, Transform,
    },
};
use bevy_mod_raycast::RayCastMesh;

use crate::player_system::player::player_camera_system;

use self::{
    blueprint::update_blueprints,
    building_components::*,
    buildings::{
        building_init_done, building_init_not_done_and_get_load_states, load_buildings_in_resource,
        load_buildings_into_resource, BuildingInitDone,
    },
    load_models::{initiate_load, NONE_HANDLE, NUM_MODELS},
    placement::check_cursor_bp_collision,
    raycasting::{raycast, BuildCursor, LatestCursorPosition, RaycastCursor},
};

pub mod blueprint;
pub mod building;
pub mod building_components;
pub mod building_functions;
pub mod buildings;
pub mod load_models;
pub mod placement;
pub mod raycasting;
pub mod snapping;

pub struct RaycastSet;

pub struct ModelHandles {
    handles: [Option<Handle<GltfMesh>>; NUM_MODELS],
}

#[derive(Clone)]
pub struct MaterialHandles {
    blueprint: Handle<StandardMaterial>,
    obstructed: Handle<StandardMaterial>,
}

pub struct BlueprintFillMaterial(Vec<Handle<StandardMaterial>>);

pub struct BuildingSystemPlugin;

pub struct GlobalPipeId(pub u32);

pub struct PipeCylinderMaterial(pub Handle<StandardMaterial>);

impl Plugin for BuildingSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ModelHandles {
            handles: [NONE_HANDLE; NUM_MODELS],
        })
        .insert_resource(ChangeBuilding { b: false })
        .insert_resource(BuildingInitDone(false))
        .insert_resource(GlobalPipeId(0))
        .insert_resource(LatestCursorPosition(None))
        .add_startup_system(building_system_startup)
        .add_startup_system(initiate_load)
        .add_startup_system(load_buildings_into_resource)
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_run_criteria(building_init_not_done_and_get_load_states)
                .with_system(load_buildings_in_resource.before(building)),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_run_criteria(building_init_done)
                .with_system(raycast)
                .with_system(building.after(raycast))
                .with_system(snapping.after(building))
                .with_system(check_cursor_bp_collision.after(snapping)),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(30.0))
                .with_system(update_blueprints),
        )
        .add_system(player_camera_system);
    }
}

pub fn building_system_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("building start");
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(RayCastMesh::<RaycastSet>::default());
    // lil thingy where cursor
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                subdivisions: 3,
            })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(RaycastCursor { visible: false });

    commands.insert_resource(BuildCursor {
        intersection: None,
        rotation: 0.0,
    });
    commands.insert_resource(PipePlacement {
        placed: false,
        transform: None,
    });
    commands.insert_resource(BlueprintFillMaterial::generate(&mut materials, 50));
    commands.insert_resource(MaterialHandles::generate(&mut materials));
    commands.insert_resource(PipeCylinderMaterial(materials.add(StandardMaterial {
        base_color: Color::rgb(0.5913826, 0.5913826, 0.5913826),
        metallic: 0.0,
        perceptual_roughness: 0.5,
        ..default()
    })));

    info!("building done");
}

impl BlueprintFillMaterial {
    pub fn generate(materials: &mut ResMut<Assets<StandardMaterial>>, num: u32) -> Self {
        let mut mat = BlueprintFillMaterial(Vec::with_capacity(50));

        for i in 0..(num as usize) {
            let ix = (i as f32) / (num as f32);
            mat.0.push(materials.add(StandardMaterial {
                base_color: Color::rgba((87.0 / 255.0) * ix, (202.0 / 255.0) * ix, (1.0) * ix, 0.5),
                reflectance: 0.0,
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }));
        }

        mat
    }

    pub fn get_bp_fill_material(&self, filled: u32, cost: u32) -> Handle<StandardMaterial> {
        self.get_fill_percent((filled as f32) / (cost as f32))
    }

    pub fn get_fill_percent(&self, pct: f32) -> Handle<StandardMaterial> {
        let len = self.0.len() as f32;
        self.0[((pct * len).round() as usize).min(len as usize - 1)].clone()
    }
}

impl MaterialHandles {
    pub fn generate(materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        MaterialHandles {
            blueprint: materials.add(StandardMaterial {
                base_color: Color::rgba(87.0 / 255.0, 202.0 / 255.0, 1.0, 0.5),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            obstructed: materials.add(StandardMaterial {
                base_color: Color::rgba(1.0, 0.0, 0.0, 0.5),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
        }
    }
}
