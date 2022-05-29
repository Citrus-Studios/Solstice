use bevy::{
    core_pipeline::ClearColor,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::{Quat, Vec3},
    pbr::{DirectionalLight, DirectionalLightBundle, PbrBundle},
    prelude::*,
    DefaultPlugins,
};
use bevy_obj::ObjPlugin;
use bevy_rapier3d::{
    plugin::TimestepMode,
    prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
};
use building_system::{BuildingSystemPlugin, RaycastSet};
use constants::HALF_SIZE;
use player_system::PlayerSystemPlugin;
use terrain_generation_system::GeneratorPlugin;

pub mod building_system;
pub mod player_system;
pub mod terrain_generation_system;

pub mod algorithms;
pub mod constants;

pub mod material_palette;
pub mod model_loader;

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ObjPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(GeneratorPlugin)
        .add_plugin(BuildingSystemPlugin)
        .add_plugin(PlayerSystemPlugin)
        // startup system
        .add_startup_system(startup)
        // resources
        .insert_resource(RapierConfiguration {
            gravity: [0.0, -9.81, 0.0].into(),
            timestep_mode: TimestepMode::Interpolated {
                dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            },
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(
            14.0 / 255.0,
            125.0 / 255.0,
            127.0 / 255.0,
        )))
        .run();
}

fn startup(
    mut commands: Commands,

    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    info!("startup start");
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
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            perceptual_roughness: 1.0,
            emissive: Color::rgb(0.0 / 255.0, 255.0 / 255.0, 251.0 / 255.0),
            ..default()
        }),
        transform: Transform::from_xyz(100.0, 112.2, 100.0),
        ..default()
    });
    info!("startup done");
}
