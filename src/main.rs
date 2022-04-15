use bevy::{prelude::{App, Msaa, Commands, OrthographicProjection, Transform, Color}, DefaultPlugins, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, pbr::{DirectionalLightBundle, DirectionalLight}, math::{Vec3, Quat}, core_pipeline::ClearColor};
use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_obj::ObjPlugin;
use bevy_rapier3d::{physics::{RapierPhysicsPlugin, NoUserData, RapierConfiguration}, render::RapierRenderPlugin};
use building_system::{RaycastSet, BuildingSystemPlugin};
use constants::HALF_SIZE;
use player_system::PlayerSystemPlugin;
use terrain_generation_system::GeneratorPlugin;

pub mod building_system;
pub mod player_system;
pub mod terrain_generation_system; 

pub mod constants;
pub mod algorithms;

fn main() {
    App::new()
        // plugins    
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DefaultRaycastingPlugin::<RaycastSet>::default())
        .add_plugin(ObjPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        
        .add_plugin(GeneratorPlugin)
        .add_plugin(BuildingSystemPlugin)
        .add_plugin(PlayerSystemPlugin)

        // startup system
        .add_startup_system(startup)

        // resources
        .insert_resource(RapierConfiguration {
            gravity: [0.0, -9.81, 0.0].into(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(14.0 / 255.0, 125.0 / 255.0, 127.0 / 255.0)))

        .run();
}

fn startup(
    mut commands: Commands,
) {
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..Default::default()
            },
            shadows_enabled: true,
            illuminance: 32000.0,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });
}