use std::sync::Arc;

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

    // let commands = Arc::new(Box::leak(Box::new(commands)));
    // let hollowground_handle = &hollowground_handle;
    // let materials = Arc::new(materials);

    let perlin = Perlin::default();

    for i in 0..generator_options.width {
        for j in 0..generator_options.length {
            let n = perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);
            //info!(n);
            if n > 0.0 {
                commands.spawn_bundle(PbrBundle {
                    mesh: hollowground_handle.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(randomcolor.0, randomcolor.1, randomcolor.2),
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz((i as f32) * 3.0, -2.0, (j as f32) * 3.0),
                    ..Default::default()
                })
                .insert(RayCastMesh::<RaycastSet>::default());
            }
        }
    }
}