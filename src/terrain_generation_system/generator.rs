use bevy::prelude::*;
use bevy_mod_raycast::RayCastMesh;
use bevy_rapier3d::{
    physics::{ColliderBundle, ColliderPositionSync},
    prelude::ColliderShape,
};
use rand::Rng;

use noise::{NoiseFn, Perlin, Seedable};

use crate::{constants::SEED, RaycastSet};

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

    // let commands = Arc::new(Box::leak(Box::new(commands)));
    // let hollowground_handle = &hollowground_handle;
    // let materials = Arc::new(materials);

    let perlin = Perlin::default().set_seed(*SEED);

    // generates terrain given a width and a length
    for i in 0..generator_options.width {
        for j in 0..generator_options.length {
            let n = perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);
            //info!(n);
            if n > 0.0 {
                // Generate a block
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: hollowground_handle.clone(),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(50.0 / 255.0, 56.0 / 255.0, 53.0 / 255.0),
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz((i as f32) * 3.0, -2.0, (j as f32) * 3.0)
                            .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                        ..Default::default()
                    })
                    .insert(RayCastMesh::<RaycastSet>::default())
                    .insert(ColliderPositionSync::Discrete)
                    .insert(RayCastMesh::<RaycastSet>::default())
                    .with_children(|parent| {
                        parent.spawn_bundle(ColliderBundle {
                            shape: ColliderShape::cuboid(1.5, 1.5, 1.5).into(),
                            position: Vec3::new((i as f32) * 3.0, -2.0, (j as f32) * 3.0).into(),
                            ..Default::default()
                        });
                    });

                // Spires
                let mut rng = rand::thread_rng();
                if rng.gen_range(1..=10) >= 7 {
                    let height = rng.gen_range(3..=10);
                    for x in 1..=height {
                        // Generate a spire
                        commands
                            .spawn_bundle(PbrBundle {
                                mesh: hollowground_handle.clone(),
                                material: materials.add(StandardMaterial {
                                    base_color: Color::rgb(
                                        153.0 / 255.0,
                                        132.0 / 255.0,
                                        0.0,
                                    ),
                                    ..Default::default()
                                }),
                                transform: Transform::from_xyz(
                                    (i as f32) * 3.0,
                                    x as f32 - 2.0,
                                    (j as f32) * 3.0,
                                )
                                .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                                ..Default::default()
                            })
                            .insert(RayCastMesh::<RaycastSet>::default())
                            .insert(ColliderPositionSync::Discrete)
                            .insert(RayCastMesh::<RaycastSet>::default())
                            .with_children(|parent| {
                                parent.spawn_bundle(ColliderBundle {
                                    shape: ColliderShape::cuboid(1.5, 1.5, 1.5).into(),
                                    position: Vec3::new((i as f32) * 3.0, x as f32 - 2.0, (j as f32) * 3.0)
                                        .into(),
                                    ..Default::default()
                                });
                            });
                    }
                }
            }
        }
    }
}
