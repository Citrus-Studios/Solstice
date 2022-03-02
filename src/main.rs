
use bevy::{prelude::*, core::FixedTimestep};

use bevy_obj::ObjPlugin;

use bevy_mod_raycast::{
    DefaultRaycastingPlugin, RayCastMesh, RayCastSource,
    RaycastSystem
};

use bevy_rapier3d::{physics::{RigidBodyBundle, ColliderBundle, ColliderPositionSync, RapierPhysicsPlugin, NoUserData}, render::ColliderDebugRender};
use building_system::{update_raycast_with_cursor, raycast, RaycastCursor};
use constants::DELTA_TIME;

use player::{Player, player_movement_system, CameraComp, player_camera_system};
use terrain_generation_system::generator::{GeneratorOptions, generate_terrain};

pub mod player;
pub mod constants;

pub mod terrain_generation_system;
pub mod building_system;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultRaycastingPlugin::<RaycastSet>::default())
        .add_plugin(ObjPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup)
        .insert_resource(GeneratorOptions {
            width: 10,
            height: 1,
        })
        .add_startup_system(generate_terrain)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(DELTA_TIME as f64))
                .with_system(player_movement_system)
        )
        .add_system(player_camera_system)
        .add_system_to_stage(
            CoreStage::PreUpdate,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays),
        )
        .add_system(raycast)
        .run();
}

pub struct RaycastSet;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert(RayCastMesh::<RaycastSet>::default()
    );

    // lil thingy where cursor
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.1, subdivisions: 3 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
        })
        .insert(RaycastCursor { visible: false, intersection: None });


    // player 
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert(Player {
        name: "None".to_string(),
        speed: 5.0
    })
    .insert_bundle(RigidBodyBundle::default())
    .insert_bundle(ColliderBundle {
        position: [1.0, 1.0, 1.0].into(),
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(ColliderDebugRender::with_id(0))
    .with_children(|child| {
        // camera
        child.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 4.5, 0.0),
            ..Default::default()
        })
        .insert(CameraComp {
            yaw: 0.0,
            roll: 0.0,
            zoom: 5.0,
        })
        .insert(RayCastSource::<RaycastSet>::new());
    });
}