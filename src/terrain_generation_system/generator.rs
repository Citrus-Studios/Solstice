use std::time::{Instant, Duration};

use bevy::{
    prelude::*,
};
use bevy_mod_raycast::RayCastMesh;
use bevy_rapier3d::{
    prelude::{ActiveCollisionTypes, InteractionGroups, Collider, CollisionGroups, RigidBody, SolverGroups}, rapier::prelude::{ColliderShape, ColliderFlags, SharedShape},
};
use bevy::render::render_resource::PrimitiveTopology::TriangleList;

use rand::{Rng, prelude::ThreadRng};

use noise::{NoiseFn, Perlin, Seedable};

use crate::{constants::SEED, RaycastSet, algorithms::distance_vec2, terrain_generation_system::compound_collider_builder::CompoundColliderBuilder};

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
    SpireHollow
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

    // let commands = Arc::new(Box::leak(Box::new(commands)));
    // let hollowground_handle = &hollowground_handle;
    // let materials = Arc::new(materials);

    let perlin = Perlin::default().set_seed(*SEED);
    let spire_perlin = Perlin::default().set_seed(*SEED / 2);

    let hollowground_ref = match meshes.get(&hollowground_handle) { Some(e) => e, _ => return};
    let ground1_ref = match meshes.get(&ground1_handle) { Some(e) => e, _ => return};
    let spires_hollow_ref = match meshes.get(&spires_hollow_handle) { Some(e) => e, _ => return};

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

    let hollowground_ccb = CompoundColliderBuilder::from_vec(hollowground_collider_vec);
    let ground1_ccb = CompoundColliderBuilder::from_vec(ground1_collider_vec);
    let spires_hollow_ccb = CompoundColliderBuilder::from_vec(spires_hollow_collider_vec);

    // BEEG vec
    let mut world_gen_array: Vec<Vec<Vec<Option<TerrainBlockType>>>> = vec![vec![vec![None; 100]; 100]; 100];

    let mut attr = RelevantAttributes::new();

    // generates terrain given a width and a length
    for i in 0..generator_options.radius {
        let i_usize = i as usize;
        let i_i32 = i as i32;
        for j in 0..generator_options.radius {
            let j_i32 = j as i32;
            if (((i_i32 - 50) * (i_i32 - 50)) + ((j_i32 - 50) * (j_i32 - 50))) < 2500 {
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

    // Iterates through every single block and adds meshes and colliders accordingly
    for (z, xy_plane) in world_gen_array.into_iter().enumerate() {
        let z_pos = z as f32 * 3.0;
        for (y, row) in xy_plane.into_iter().enumerate() {
            let y_pos = y as f32 * 3.0;
            for (x, i) in row.into_iter().enumerate() {
                if i.is_some() {
                    let x_pos = x as f32 * 3.0;
                    let translation = Vec3::new(x_pos, y_pos, z_pos);

                    let (mesh, collider) = match i.unwrap() {
                        TerrainBlockType::Solid => (ground1_ref.clone(), ground1_ccb.build()),
                        TerrainBlockType::Hollow => (hollowground_ref.clone(), hollowground_ccb.build()),
                        TerrainBlockType::SpireHollow => (spires_hollow_ref.clone(), spires_hollow_ccb.build()),
                    };

                    commands.spawn()
                        .insert(collider)
                        .insert(Transform::from_translation(translation))
                        .insert(CollisionGroups { memberships: 0b0001, filters: 0b1110 })
                        .insert(SolverGroups { memberships: 0b1110, filters: 0b0001 })
                        .insert(ActiveCollisionTypes::STATIC_STATIC)
                    ;

                    attr = attr.combine_with_mesh(mesh, translation);
                }
            }
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

    done.done = true;
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

