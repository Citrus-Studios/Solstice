use bevy::{prelude::{Mesh, Commands, ResMut, Assets, shape, Color, Transform, BuildChildren, PerspectiveCameraBundle}, pbr::{StandardMaterial, PbrBundle}};
use bevy_mod_picking::RayCastSource;
use bevy_rapier3d::prelude::*;

use crate::building_system::RaycastSet;

use super::player::{Player, CameraComp};

pub struct PlayerStartupDone {
    pub done: bool
}

pub fn player_start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut done: ResMut<PlayerStartupDone>
) {
    if done.done {
        return
    }
    
    // player 
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(100.0, 175.0, 100.0),
        ..Default::default()
    })
    .insert(Player {
        name: "None".to_string(),
        speed: 5.0
    })

    .insert(Collider::round_cuboid(0.4, 0.4, 0.4, 0.1))
        .insert(Friction { coefficient: 0.0, combine_rule: CoefficientCombineRule::Min })
        .insert(Velocity::default())
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(CollisionGroups { memberships: 0b0010, filters: 0b1111 })

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

    done.done = true
}