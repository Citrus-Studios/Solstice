use crate::building_system::building::building;
use bevy::{
    pbr::{PbrBundle, StandardMaterial, AlphaMode},
    prelude::{
        shape, Assets, Color, Commands, CoreStage, Mesh, ParallelSystemDescriptorCoercion, Plugin,
        ResMut, Transform, SystemSet, Handle, info,
    }, gltf::GltfMesh, core::FixedTimestep,
};
use bevy_mod_raycast::{RayCastMesh, RaycastSystem};

use crate::player_system::player::player_camera_system;

use self::{raycasting::{raycast, update_raycast_with_cursor, RaycastCursor, BuildCursor}, building::{check_cursor_bp_collision}, load_models::{initiate_load, NUM_MODELS, NONE_HANDLE}, building_components::*, blueprint::update_blueprints, buildings::{load_buildings_into_resource, load_buildings_in_resource, BuildingInitDone, building_init_done, building_init_not_done_and_get_load_states}};

pub mod raycasting;
pub mod building;
pub mod buildings;
pub mod load_models;
pub mod building_components;
pub mod building_functions;
pub mod blueprint;

pub struct RaycastSet;

pub struct ModelHandles {
    handles: [Option<Handle<GltfMesh>>; NUM_MODELS]
}

#[derive(Clone)]
pub struct MaterialHandles {
    blueprint: Handle<StandardMaterial>,
    obstructed: Handle<StandardMaterial>,
}

pub struct BlueprintFillMaterial(Vec<Handle<StandardMaterial>>);

pub struct BuildingSystemPlugin;

impl Plugin for BuildingSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(ModelHandles { handles: [NONE_HANDLE; NUM_MODELS] })
            .insert_resource(ChangeBuilding { b: false })
            .insert_resource(BuildingInitDone(false))
            .add_startup_system(building_system_startup)
            .add_startup_system(initiate_load)
            .add_startup_system(load_buildings_into_resource)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_run_criteria(building_init_not_done_and_get_load_states)
                    .with_system(load_buildings_in_resource.before(building))
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_run_criteria(building_init_done)
                    .with_system(update_raycast_with_cursor)
                    .with_system(raycast.after(RaycastSystem::UpdateRaycast))
                    .with_system(building.after(raycast))
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                check_cursor_bp_collision
            )
            .add_system_set(SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(30.0))
                .with_system(update_blueprints)
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
    
    commands.insert_resource(BuildCursor { intersection: None, rotation: 0.0, entity: None });
    commands.insert_resource(PipePlacement { placed: false, transform: None });
    commands.insert_resource(BlueprintFillMaterial::generate(&mut materials, 50));
    commands.insert_resource(MaterialHandles::generate(&mut materials));

    info!("building done");
}

impl BlueprintFillMaterial {
    pub fn generate(materials: &mut ResMut<Assets<StandardMaterial>>, num: u32) -> Self {
        let mut mat = BlueprintFillMaterial(Vec::with_capacity(50));

        for i in 0..(num as usize) {
            let ix = (i as f32) / (num as f32);
            mat.0.push(materials.add(StandardMaterial {
                base_color: Color::rgba((87.0/255.0) * ix, (202.0/255.0) * ix, (1.0) * ix, 0.5),
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
                base_color: Color::rgba(87.0/255.0, 202.0/255.0, 1.0, 0.5),
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