use crate::building_system::building::building;
use bevy::{
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        shape, Assets, Color, Commands, CoreStage, Mesh, ParallelSystemDescriptorCoercion, Plugin,
        ResMut, Transform, SystemSet, Handle,
    }, gltf::GltfMesh,
};
use bevy_mod_raycast::{RayCastMesh, RaycastSystem};

use crate::player_system::player::player_camera_system;

use self::{raycasting::{raycast, update_raycast_with_cursor, RaycastCursor, BuildCursor}, building::{PipePlacement, ChangeBuilding}, load_models::{initiate_load, get_load_states, NUM_MODELS, NONE_HANDLE}};

pub mod raycasting;
pub mod building;
pub mod buildings;
pub mod load_models;

pub struct RaycastSet;

pub struct ModelHandles {
    handles: [Option<Handle<GltfMesh>>; NUM_MODELS]
}

#[derive(Clone)]
pub struct MaterialHandles {
    blueprint: Option<Handle<StandardMaterial>>,
    obstructed: Option<Handle<StandardMaterial>>,
}

pub struct BuildingSystemPlugin;

impl Plugin for BuildingSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(building_system_startup)
            .insert_resource(ModelHandles { handles: [NONE_HANDLE; NUM_MODELS] })
            .insert_resource(MaterialHandles {
                blueprint: None,
                obstructed: None,
            })
            .insert_resource(ChangeBuilding { b: false })
            .add_startup_system(initiate_load)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                update_raycast_with_cursor.before(RaycastSystem::BuildRays),
            )
            .add_system(raycast)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(get_load_states)
                    .with_system(building)
            )
            .add_system(player_camera_system);
    }
}

pub fn building_system_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
    
    commands.insert_resource(BuildCursor { intersection: None, rotation: 0.0 });
    commands.insert_resource(PipePlacement { placed: false, transform: None });
}
