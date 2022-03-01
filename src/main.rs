
use std::f32::consts::PI;

use bevy::{prelude::*, core::FixedTimestep, render::primitives::Aabb};

use bevy_obj::ObjPlugin;

use bevy_mod_raycast::{
    DefaultPluginState, DefaultRaycastingPlugin, RayCastMesh, RayCastMethod, RayCastSource,
    RaycastSystem, SimplifiedMesh,
};

use building_system::{update_raycast_with_cursor, raycast};
use constants::DELTA_TIME;

use heron::{PhysicsLayer, PhysicsPlugin, CollisionShape, RigidBody, Gravity, AxisAngle, Velocity,
    rapier_plugin::{PhysicsWorld, ShapeCastCollisionType},
    *
};


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
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(setup)
        .insert_resource(GeneratorOptions {
            width: 10,
            height: 1,
        })
        .insert_resource(Gravity::from(Vec3::new(0.0, -20.0, 0.0))) // gravity
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
        .add_system_to_stage(
            CoreStage::PostUpdate,
            raycast.after(RaycastSystem::UpdateRaycast)
        )
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
    }).insert(CollisionShape::Cuboid {
        half_extends: Vec3::new(1.0, 1.0, 1.0),
        border_radius: None,
    })
    .insert(
        Velocity::from_linear(Vec3::X * 10.0)
            .with_angular(AxisAngle::new(Vec3::Z, 0.5 * PI))
    )
    .insert(RayCastMesh::<RaycastSet>::default()
    );


    // player 
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert(Player {
        name: "None".to_string(),
        speed: 10.0
    })
    .insert(CollisionShape::Cuboid {
        half_extends: Vec3::new(1.0, 1.0, 1.0),
        border_radius: None,
    })
    .insert(RigidBody::Dynamic)
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