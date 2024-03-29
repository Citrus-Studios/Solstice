use std::time::Instant;

use bevy::asset::LoadState;
use bevy::gltf::GltfPrimitive;

use bevy::utils::HashMap;
use bevy::{gltf::GltfMesh, prelude::*};

use bevy_rapier3d::prelude::{ActiveCollisionTypes, Collider, CollisionGroups};

use rand::{prelude::ThreadRng, Rng};

use noise::{NoiseFn, Perlin, Seedable};

use crate::building_system::buildings::InsertNoReturn;
use crate::model_loader::{combine_gltf_mesh, translate_gltf_primitives};
use crate::terrain_generation_system::terrain_block::{Blocks, TerrainBlockData, TerrainBlockName};
use crate::{
    constants::SEED, terrain_generation_system::compound_collider_builder::CompoundColliderBuilder,
};

#[derive(Component)]
pub struct GeneratorOptions {
    pub radius: u32,
    pub height: u32,
}

pub struct TerrainGenDone {
    pub done: bool,
}

pub fn generate_terrain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut images: ResMut<Assets<Image>>,
    mut done: ResMut<TerrainGenDone>,

    generator_options: Res<GeneratorOptions>,
) {
    if done.done {
        return;
    }

    let time = Instant::now();

    let perlin = Perlin::default().set_seed(*SEED);
    let spire_perlin = Perlin::default().set_seed(*SEED / 2);

    let z_cuboid = Collider::cuboid(0.25, 0.25, 1.5);
    let y_cuboid = Collider::cuboid(0.25, 1.5, 0.25);
    let x_cuboid = Collider::cuboid(1.5, 0.25, 0.25);
    let q = Quat::default();

    let hollowground_collider_vec = vec![
        (Vec3::new(-1.25, 1.25, 0.0), q, z_cuboid.clone()),
        (Vec3::new(1.25, 1.25, 0.0), q, z_cuboid.clone()),
        (Vec3::new(1.25, -1.25, 0.0), q, z_cuboid.clone()),
        (Vec3::new(-1.25, -1.25, 0.0), q, z_cuboid.clone()),
        (Vec3::new(-1.25, 0.0, 1.25), q, y_cuboid.clone()),
        (Vec3::new(1.25, 0.0, 1.25), q, y_cuboid.clone()),
        (Vec3::new(1.25, 0.0, -1.25), q, y_cuboid.clone()),
        (Vec3::new(-1.25, 0.0, -1.25), q, y_cuboid.clone()),
        (Vec3::new(0.0, -1.25, 1.25), q, x_cuboid.clone()),
        (Vec3::new(0.0, 1.25, 1.25), q, x_cuboid.clone()),
        (Vec3::new(0.0, 1.25, -1.25), q, x_cuboid.clone()),
        (Vec3::new(0.0, -1.25, -1.25), q, x_cuboid.clone()),
    ];

    let ground1_collider_vec = vec![(Vec3::ZERO, q, Collider::cuboid(1.5, 1.5, 1.5))];

    let spires_solid_collider_vec = vec![(Vec3::ZERO, q, Collider::cuboid(1.5, 1.5, 1.5))];

    let y_cuboid = Collider::cuboid(0.25, 1.25, 0.25);
    let spires_hollow_collider_vec = vec![
        (
            Vec3::new(0.0, 1.25, 0.0),
            q,
            Collider::cuboid(1.5, 0.25, 1.5),
        ),
        (Vec3::new(-1.25, -0.25, 1.25), q, y_cuboid.clone()),
        (Vec3::new(1.25, -0.25, 1.25), q, y_cuboid.clone()),
        (Vec3::new(1.25, -0.25, -1.25), q, y_cuboid.clone()),
        (Vec3::new(-1.25, -0.25, -1.25), q, y_cuboid.clone()),
        (Vec3::new(0.0, -1.25, 1.25), q, x_cuboid.clone()),
        (Vec3::new(0.0, -1.25, -1.25), q, x_cuboid.clone()),
        (Vec3::new(1.25, -1.25, 0.0), q, z_cuboid.clone()),
        (Vec3::new(-1.25, -1.25, 0.0), q, z_cuboid.clone()),
    ];

    let well_collider_vec = {
        let mut e = hollowground_collider_vec.clone();
        e.append(&mut vec![
            (Vec3::ZERO, q, Collider::cylinder(1.5, 0.5)),
            (
                Vec3::new(0.0, 0.929, 0.0),
                q,
                Collider::cuboid(0.455703, 0.071, 1.5),
            ),
            (
                Vec3::new(0.0, -0.929, 0.0),
                q,
                Collider::cuboid(0.455703, 0.071, 1.5),
            ),
        ]);
        e
    };

    let hollowground_ccb = CompoundColliderBuilder::from_vec(hollowground_collider_vec);
    let ground1_ccb = CompoundColliderBuilder::from_vec(ground1_collider_vec);
    let spires_hollow_ccb = CompoundColliderBuilder::from_vec(spires_hollow_collider_vec);
    let spires_solid_ccb = CompoundColliderBuilder::from_vec(spires_solid_collider_vec);
    let well_ccb = CompoundColliderBuilder::from_vec(well_collider_vec);

    let mut blocks = Blocks::new();
    let mut results = Vec::new();

    results.push(blocks.add(
        "models/ground1/ground1.gltf#Mesh0",
        ground1_ccb,
        &asset_server,
        &gltf_meshes,
    ));

    results.push(blocks.add(
        "models/ground1/hollow_ground.gltf#Mesh0",
        hollowground_ccb,
        &asset_server,
        &gltf_meshes,
    ));

    results.push(blocks.add(
        "models/ground1/spires_hollow.gltf#Mesh0",
        spires_hollow_ccb,
        &asset_server,
        &gltf_meshes,
    ));

    results.push(blocks.add(
        "models/ground1/spires_full.gltf#Mesh0",
        spires_solid_ccb,
        &asset_server,
        &gltf_meshes,
    ));

    results.push(blocks.add(
        "models/ground1/well_ground.gltf#Mesh0",
        well_ccb,
        &asset_server,
        &gltf_meshes,
    ));

    for result in results {
        match result {
            Err(e) => match e {
                LoadState::Loading => continue,
                LoadState::Failed => panic!("Load failed"),
                LoadState::Unloaded => panic!("Unloaded"),
                _ => unreachable!(),
            },
            _ => (),
        }
    }

    // BEEG vec
    let mut world_gen_array = vec![vec![vec![None; 100]; 100]; 100];

    // generates terrain given a width and a length
    for i in 0..generator_options.radius {
        let i_usize = i as usize;
        let i_i32 = i as i32;
        for j in 0..generator_options.radius {
            let j_i32 = j as i32;
            if (((i_i32 - 50) * (i_i32 - 50)) + ((j_i32 - 50) * (j_i32 - 50))) < 2500
                && world_gen_array[i_usize][50][j as usize].is_none()
            {
                let n = perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);

                if n > 0.0 {
                    let mut rng = rand::thread_rng();
                    let j_usize = j as usize;

                    world_gen_array[i_usize][50][j_usize] =
                        Some(rng.random_pick(0.5, "ground1", "hollow_ground"));
                    if n >= 0.3 {
                        world_gen_array[i_usize][49][j_usize] =
                            Some(rng.random_pick(0.5, "ground1", "hollow_ground"));
                        if n >= 0.6 {
                            world_gen_array[i_usize][48][j_usize] =
                                Some(rng.random_pick(0.5, "ground1", "hollow_ground"));
                            if n >= 0.95 {
                                world_gen_array[i_usize][47][j_usize] =
                                    Some(rng.random_pick(0.5, "ground1", "hollow_ground"));
                            }
                        }
                    }

                    let well_decider = rng.gen_ratio(1, 100);

                    if well_decider {
                        let big_well = rng.gen_ratio(1, 5)
                            && (i_usize > 0 && i_usize < 99 && j_usize > 0 && j_usize < 99);

                        if big_well {
                            generate_well_cluster(&mut world_gen_array, i_usize, j_usize);
                        } else {
                            generate_well_column(&mut world_gen_array, i_usize, j_usize);
                        }
                    }

                    if spire_perlin.get([(i as f64) * 0.1, (j as f64) * 0.1]) > 0.5 {
                        let height = rng.gen_range(3..=7);
                        for y in 1..=height {
                            world_gen_array[i_usize][50 + y][j_usize] =
                                Some(rng.random_pick(0.5, "spires_hollow", "spires_full"));
                        }
                    }
                }
            }
        }
    }

    let mut primitives: Vec<GltfPrimitive> = Vec::new();

    // Iterates through every single block and adds meshes and colliders accordingly
    for (z, xy_plane) in world_gen_array.into_iter().enumerate() {
        let z_pos = z as f32 * 3.0;
        for (y, row) in xy_plane.into_iter().enumerate() {
            let y_pos = y as f32 * 3.0;
            for (x, i) in row.into_iter().enumerate() {
                //info!("x: {}, y: {}, z: {}", x, y, z);
                if i.is_some() {
                    let x_pos = x as f32 * 3.0;
                    let translation = Vec3::new(x_pos, y_pos, z_pos);

                    let mut data = blocks.get(&i.unwrap()).unwrap().clone();

                    translate_gltf_primitives(&mut data.model.primitives, &mut meshes, translation);
                    primitives.append(&mut data.model.primitives);
                    commands.spawn().insert_bundle((
                        data.collider.build(),
                        Transform::from_translation(translation),
                        CollisionGroups {
                            memberships: 0b00000001,
                            filters: 0b11111110,
                        },
                        ActiveCollisionTypes::STATIC_STATIC,
                        TerrainBlockName(i.unwrap()),
                    ));
                }
            }
        }
    }

    let bundle = combine_gltf_mesh(primitives, &mut meshes, &mut materials, &mut images);

    commands.spawn_bundle(bundle);

    info!("Generation time: {:?}", time.elapsed());

    done.done = true;
}

fn generate_well_cluster(world: &mut Vec<Vec<Vec<Option<&str>>>>, x: usize, z: usize) {
    let mut rng = rand::thread_rng();
    let x_i32 = x as i32;
    let z_i32 = z as i32;

    for i in -1..=1 {
        for j in -1..=1 {
            if rng.gen_bool(0.5) {
                generate_well_column(world, (x_i32 + i) as usize, (z_i32 + j) as usize);
            }
        }
    }
}

fn generate_well_column(world: &mut Vec<Vec<Vec<Option<&str>>>>, x: usize, z: usize) {
    for y in 35..=50 {
        world[x][y][z] = Some("well_ground");
    }
}

pub trait Pick<T> {
    fn random_pick(&mut self, bias: f32, n1: T, n2: T) -> T;
}

impl<T> Pick<T> for ThreadRng {
    // Bias is the bias towards n1
    fn random_pick(&mut self, bias: f32, n1: T, n2: T) -> T {
        if !(0.0 <= bias && bias <= 1.0) {
            println!("Warning: Bias should be between 0.0 and 1.0");
        }
        if self.gen_range(0.0..1.0) <= bias {
            return n1;
        } else {
            return n2;
        }
    }
}
