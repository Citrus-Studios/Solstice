use std::mem::size_of;

use crate::building_system::building::building;
use bevy::{
    pbr::{PbrBundle, StandardMaterial, AlphaMode},
    prelude::{
        shape, Assets, Color, Commands, CoreStage, Mesh, ParallelSystemDescriptorCoercion, Plugin,
        ResMut, Transform, SystemSet, Handle, info,
    }, gltf::GltfMesh,
};
use bevy_mod_raycast::{RayCastMesh, RaycastSystem};

use crate::player_system::player::player_camera_system;

use self::{raycasting::{raycast, update_raycast_with_cursor, RaycastCursor, BuildCursor}, building::{check_cursor_bp_collision}, load_models::{initiate_load, get_load_states, NUM_MODELS, NONE_HANDLE}, building_components::*};

pub mod raycasting;
pub mod building;
pub mod buildings;
pub mod load_models;
pub mod building_components;
pub mod building_functions;

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
            .add_startup_system(building_system_startup)
            .add_startup_system(initiate_load)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_run_criteria(get_load_states)
                    .with_system(update_raycast_with_cursor)
                    .with_system(raycast.after(RaycastSystem::UpdateRaycast))
                    .with_system(building.after(raycast))
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                check_cursor_bp_collision
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
        self.get_fill_percent((cost as f32) / (filled as f32))
    }

    pub fn get_fill_percent(&self, pct: f32) -> Handle<StandardMaterial> {
        let len = self.0.len() as f32;
        self.0[(pct * len).round() as usize].clone()
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