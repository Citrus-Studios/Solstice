use bevy::prelude::*;
use bevy_mod_raycast::RayCastMesh;
use bevy_rapier3d::{physics::{ColliderPositionSync, ColliderBundle}, prelude::{ColliderShape}};
use rand::Rng;

use crate::RaycastSet;

#[derive(Component)]
pub struct GeneratorOptions {
    pub width: u32,
    pub height: u32,
}

pub fn generate_terrain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    generator_options: Res<GeneratorOptions>,
) {
    let hollowground_handle = asset_server.load("models/ground1/ground1.obj");

    let mut rng = rand::thread_rng();
    let randomcolor = (
        rng.gen_range(0.0..255.0)/255.0, 
        rng.gen_range(0.0..255.0)/255.0, 
        rng.gen_range(0.0..255.0)/255.0
    );

    commands.spawn_bundle(PbrBundle {
        mesh: hollowground_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(randomcolor.0, randomcolor.1, randomcolor.2),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, -2.0, 0.0).with_scale(Vec3::new(
            1.0/2.0,
            1.0/2.0,
            1.0/2.0,
        )),
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::compound(vec![(
            [0.0, -2.0, 0.0].into(), 
            ColliderShape::cuboid(1.0/2.0, 1.0/3.0/2.0, 1.0/2.0))]
        ).into(),
        position: [0.0, -2.0, 0.0].into(),
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(RayCastMesh::<RaycastSet>::default());
}