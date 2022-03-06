use bevy::{prelude::{App, Msaa}, DefaultPlugins};
use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_obj::ObjPlugin;
use bevy_rapier3d::{physics::{RapierPhysicsPlugin, NoUserData, RapierConfiguration}, render::RapierRenderPlugin};
use building_system::{RaycastSet, BuildingSystemPlugin};
use player_system::PlayerSystemPlugin;
use terrain_generation_system::GeneratorPlugin;

pub mod building_system;
pub mod player_system;
pub mod terrain_generation_system; 

pub mod constants;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultRaycastingPlugin::<RaycastSet>::default())
        .add_plugin(ObjPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(GeneratorPlugin)
        .add_plugin(BuildingSystemPlugin)
        .add_plugin(PlayerSystemPlugin)
        .insert_resource(RapierConfiguration {
            gravity: [0.0, -9.81, 0.0].into(),
            ..Default::default()
        })
        .run();
}