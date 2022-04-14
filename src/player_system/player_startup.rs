use bevy::{prelude::{Mesh, Commands, ResMut, Assets, shape, Color, Transform, BuildChildren, PerspectiveCameraBundle, AssetServer, Res, Handle}, pbr::{StandardMaterial, PbrBundle}};
use bevy_mod_picking::RayCastSource;
use bevy_rapier3d::{prelude::{RigidBodyType, ColliderShape, RigidBodyMassPropsFlags, ColliderMaterial, CoefficientCombineRule, InteractionGroups, ColliderFlags}, physics::{RigidBodyBundle, ColliderPositionSync, ColliderBundle}};

use crate::{building_system::RaycastSet, terrain_generation_system::mutate_mesh::MutateMesh};

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

    let lock_xyz_rotation = RigidBodyMassPropsFlags::ROTATION_LOCKED_Y
        | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z
        | RigidBodyMassPropsFlags::ROTATION_LOCKED_X;

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
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::round_cuboid(0.4, 0.4, 0.4, 0.1).into(),
        position: [100.0, 20.0, 100.0].into(),
        material: ColliderMaterial { 
            friction: 0.0,
            friction_combine_rule: CoefficientCombineRule::Min,
            ..Default::default() 
        }.into(),
        flags: ColliderFlags {
            collision_groups: InteractionGroups::new(0b1110, 0b1111),
            solver_groups: InteractionGroups::new(0b1111, 0b1110),
            ..Default::default()
        }.into(),
        ..Default::default()
    })
    .insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        mass_properties: lock_xyz_rotation.into(),
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

    done.done = true
}