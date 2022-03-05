use bevy::{prelude::{Mesh, Commands, ResMut, Assets, shape, Color, Transform, BuildChildren, PerspectiveCameraBundle}, pbr::{StandardMaterial, PbrBundle}};
use bevy_mod_picking::RayCastSource;
use bevy_rapier3d::{prelude::{RigidBodyType, ColliderType, ColliderShape}, physics::{RigidBodyBundle, ColliderPositionSync, ColliderBundle}, render::ColliderDebugRender};

use crate::building_system::RaycastSet;

use super::player::{Player, CameraComp};

pub fn player_start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // player 
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert(Player {
        name: "None".to_string(),
        speed: 200.0
    })
    .insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::KinematicVelocityBased.into(),
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(1.0, 1.0, 1.0).into(),
        position: [1.0, 1.0, 1.0].into(),
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
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