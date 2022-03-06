use bevy::prelude::*;
use bevy_mod_raycast::RayCastMesh;
use bevy_rapier3d::{physics::{ColliderPositionSync, ColliderBundle}, prelude::{ColliderShape}};
use rand::Rng;

use noise::{NoiseFn, Perlin, Seedable, utils::{PlaneMapBuilder, NoiseMapBuilder}};

use crate::{RaycastSet, constants::SEED};

#[derive(Component)]
pub struct GeneratorOptions {
    pub width: u32,
    pub length: u32,
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

    let perlin = Perlin::default();

    for i in 0..generator_options.width {
        for j in 0..generator_options.length {
            let n = perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);
            //info!(n);
            if n > 0.0 {
                tokio::spawn(make_block(&mut commands, hollowground_handle, materials, randomcolor, i, j));
            }
        }
    }
}

pub async fn make_block(
    commands: &mut Commands<'static, 'static>,
    hollowground_handle: Handle<Mesh>,
    mut materials: ResMut<'static, Assets<StandardMaterial>>,
    randomcolor: (f32, f32, f32),
    i: u32, j: u32,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: hollowground_handle.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(randomcolor.0, randomcolor.1, randomcolor.2),
            ..Default::default()
        }),
        transform: Transform::from_xyz((i as f32) * 3.0, -2.0, (j as f32) * 3.0).with_scale(Vec3::new(
            1.0/2.0,
            1.0/2.0,
            1.0/2.0,
        )),
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::compound(vec![(
            [(i as f32) * 3.0, -2.0, (j as f32) * 3.0].into(), 
            ColliderShape::cuboid(1.0/2.0, 1.0/3.0/2.0, 1.0/2.0))]
        ).into(),
        position: [(i as f32) * 3.0, -2.0, (j as f32) * 3.0].into(),
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(RayCastMesh::<RaycastSet>::default());
}