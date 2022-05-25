use std::time::{Instant};

use bevy::{
    prelude::*,
};
use bevy_mod_raycast::RayCastMesh;
use bevy_rapier3d::{
    prelude::{ActiveCollisionTypes, Collider, CollisionGroups},
};
use bevy::render::render_resource::PrimitiveTopology::TriangleList;

use rand::{Rng, prelude::ThreadRng};

use noise::{NoiseFn, Perlin, Seedable};

use crate::{constants::SEED, RaycastSet, terrain_generation_system::compound_collider_builder::CompoundColliderBuilder};

use super::{relevant_attributes::RelevantAttributes, mutate_mesh::MutateMesh};

#[derive(Component)]
pub struct GeneratorOptions {
    pub radius: u32,
    pub height: u32,
}

pub struct TerrainGenDone {
    pub done: bool,
}

#[derive(Copy, Clone)]
pub enum TerrainBlockType {
    Solid,
    Hollow,
    SpireHollow,
    Well
}

pub fn generate_terrain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut done: ResMut<TerrainGenDone>,

    generator_options: Res<GeneratorOptions>,
) {
    if done.done {
        return;
    }
    info!("We got here");

    let time = Instant::now();

    let ground1_handle: Handle<Mesh> = asset_server.load("models/ground1/ground1.obj");
    let hollowground_handle: Handle<Mesh> = asset_server.load("models/ground1/hollow_ground.obj");
    let spires_hollow_handle: Handle<Mesh> = asset_server.load("models/ground1/spires_hollow.obj");
    let well_handle: Handle<Mesh> = asset_server.load("models/ground1/well_ground.obj");

    // let commands = Arc::new(Box::leak(Box::new(commands)));
    // let hollowground_handle = &hollowground_handle;
    // let materials = Arc::new(materials);

    let perlin = Perlin::default().set_seed(*SEED);
    let spire_perlin = Perlin::default().set_seed(*SEED / 2);

    let hollowground_ref = match meshes.get(&hollowground_handle) { Some(e) => e, _ => return};
    let ground1_ref = match meshes.get(&ground1_handle) { Some(e) => e, _ => return};
    let spires_hollow_ref = match meshes.get(&spires_hollow_handle) { Some(e) => e, _ => return};
    let well_ref = match meshes.get(&well_handle) { Some(e) => e, _ => return};

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

    let y_cuboid = Collider::cuboid(0.25, 1.25, 0.25);
    let spires_hollow_collider_vec = vec![
        (Vec3::new(0.0, 1.25, 0.0), q, Collider::cuboid(1.5, 0.25, 1.5)),
        
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
            (Vec3::new(0.0, 0.929, 0.0), q, Collider::cuboid(0.455703, 0.071, 1.5)),
            (Vec3::new(0.0, -0.929, 0.0), q, Collider::cuboid(0.455703, 0.071, 1.5)),
        ]);
        e
    };

    let hollowground_ccb = CompoundColliderBuilder::from_vec(hollowground_collider_vec);
    let ground1_ccb = CompoundColliderBuilder::from_vec(ground1_collider_vec);
    let spires_hollow_ccb = CompoundColliderBuilder::from_vec(spires_hollow_collider_vec);
    let well_ccb = CompoundColliderBuilder::from_vec(well_collider_vec);

    // BEEG vec
    let mut world_gen_array = vec![vec![vec![None; 100]; 100]; 100];

    let mut attr = RelevantAttributes::new();

    // generates terrain given a width and a length
    for i in 0..generator_options.radius {
        let i_usize = i as usize;
        let i_i32 = i as i32;
        for j in 0..generator_options.radius {
            let j_i32 = j as i32;
            if (((i_i32 - 50) * (i_i32 - 50)) + ((j_i32 - 50) * (j_i32 - 50))) < 2500 && world_gen_array[i_usize][50][j as usize].is_none() {
                let n = perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);
                
                if n > 0.0 {
                    let mut rng = rand::thread_rng();
                    let j_usize = j as usize;

                    world_gen_array[i_usize][50][j_usize] = Some(rng.random_pick(0.5, TerrainBlockType::Solid, TerrainBlockType::Hollow));
                    if n >= 0.3 {
                        world_gen_array[i_usize][49][j_usize] = Some(rng.random_pick(0.5, TerrainBlockType::Solid, TerrainBlockType::Hollow));
                        if n >= 0.6 {
                            world_gen_array[i_usize][48][j_usize] = Some(rng.random_pick(0.5, TerrainBlockType::Solid, TerrainBlockType::Hollow));
                            if n >= 0.95 {
                                world_gen_array[i_usize][47][j_usize] = Some(rng.random_pick(0.5, TerrainBlockType::Solid, TerrainBlockType::Hollow));
                            }
                        }
                    }

                    let well_decider = rng.gen_ratio(1, 50);

                    if well_decider {
                        let big_well = rng.gen_ratio(1, 5) && 
                            (i_usize > 0 && i_usize < 99 && j_usize > 0 && j_usize < 99);
                        
                        if big_well {
                            generate_well_cluster(&mut world_gen_array, i_usize, j_usize);
                        } else {
                            generate_well_column(&mut world_gen_array, i_usize, j_usize);
                        }
                    }
                    
                    if spire_perlin.get([(i as f64) * 0.1, (j as f64) * 0.1]) > 0.5 {
                        let height = rng.gen_range(3..=7);
                        for y in 1..=height {
                            world_gen_array[i_usize][50 + y][j_usize] = Some(rng.random_pick(0.5, TerrainBlockType::Solid, TerrainBlockType::SpireHollow));
                        }
                    }
                }                
            }
        }
    }

    let mut num_shapes = 0;

    // Iterates through every single block and adds meshes and colliders accordingly
    for (z, xy_plane) in world_gen_array.into_iter().enumerate() {
        let z_pos = z as f32 * 3.0;
        let mut plane_ccb = CompoundColliderBuilder::new();
        for (y, row) in xy_plane.into_iter().enumerate() {
            let y_pos = y as f32 * 3.0;
            for (x, i) in row.into_iter().enumerate() {
                if i.is_some() {
                    let x_pos = x as f32 * 3.0;
                    let translation = Vec3::new(x_pos, y_pos, z_pos);

                    let (mesh, collider) = match i.unwrap() {
                        TerrainBlockType::Solid => { num_shapes += 1; (ground1_ref.clone(), ground1_ccb.clone()) },
                        TerrainBlockType::Hollow => { num_shapes += 12; (hollowground_ref.clone(), hollowground_ccb.clone()) },
                        TerrainBlockType::SpireHollow => { num_shapes += 9; (spires_hollow_ref.clone(), spires_hollow_ccb.clone()) },
                        TerrainBlockType::Well => { num_shapes += 15; (well_ref.clone(), well_ccb.clone()) },
                    };

                    plane_ccb.append_with_transform(collider, (Quat::IDENTITY, translation));
                    attr = attr.combine_with_mesh(mesh, translation);
                }
            }
        }
        if !plane_ccb.is_empty() {
            commands.spawn()
                .insert_bundle((
                    plane_ccb.build(),
                    Transform::default(),
                    CollisionGroups { memberships: 0b00000001, filters: 0b11111110 },
                    ActiveCollisionTypes::STATIC_STATIC
                ))
            ;
        }
    }

    let mesh = Mesh::new(TriangleList).set_attributes(attr);
    let final_mesh_handle = meshes.add(mesh);

    commands
        .spawn_bundle(PbrBundle {
            mesh: final_mesh_handle.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(50.0 / 255.0, 56.0 / 255.0, 53.0 / 255.0),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(RayCastMesh::<RaycastSet>::default());

    info!("Generation time: {:?}", time.elapsed());
    info!("Number of shapes: {}", num_shapes);

    done.done = true;
}

fn generate_well_cluster(world: &mut Vec<Vec<Vec<Option<TerrainBlockType>>>>, x: usize, z: usize) {
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

fn generate_well_column(world: &mut Vec<Vec<Vec<Option<TerrainBlockType>>>>, x: usize, z: usize) {
    for y in 25..=50 {
        world[x][y][z] = Some(TerrainBlockType::Well);
    }
}

pub trait Pick<T> {
    fn random_pick(&mut self, bias: f32, n1: T, n2: T) -> T;
}

impl<T> Pick<T> for ThreadRng {
    // Bias is the bias towards n1
    fn random_pick(&mut self, bias: f32, n1: T, n2: T) -> T {
        if !(0.0 <= bias && bias <= 1.0) { println!("Warning: Bias should be between 0.0 and 1.0"); }
        if self.gen_range(0.0..1.0) <= bias {
            return n1;
        } else {
            return n2;
        }
    }
}



// trait SpawnCollider {
//     fn spawn_locked_collider(&mut self, col: SharedShape, trans: Vec3);
// }

// impl SpawnCollider for Commands<'_, '_> {
//     fn spawn_locked_collider(&mut self, col: SharedShape, trans: Vec3) {
//         self.spawn_bundle(ColliderBundle {
//             shape: col.into(),
//             position: trans.into(),
//             flags: ColliderFlags {
//                 active_collision_types: ActiveCollisionTypes::STATIC_STATIC.into(),
//                 ..Default::default()
//             }.into(),
//             ..Default::default()
//         });
//     }
// }

