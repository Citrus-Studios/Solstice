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

use crate::{constants::SEED, RaycastSet, algorithms::distance_vec2};

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

#[derive(Clone)]
struct CompoundColliderBuilder {
    colliders: Vec<Collider>,
    offset: Vec<(Quat, Vec3)>,
}

impl CompoundColliderBuilder {
    fn new() -> Self {
        CompoundColliderBuilder { colliders: Vec::new(), offset: Vec::new() }
    }

    fn from_vec(vec: Vec<(Vec3, Quat, Collider)>) -> Self {
        let mut return_ccb = CompoundColliderBuilder::new();
        for (t, q, e) in vec {
            return_ccb.push(e, (q,t));
        }
        return_ccb
    }

    fn to_vec(self) -> Vec<(Vec3, Quat, Collider)> {
        let mut return_vec = Vec::new();
        for (c, (q, t)) in self.colliders.iter().zip(self.offset.iter()) {
            return_vec.push((t.to_owned(), q.to_owned(), c.to_owned()));
        }
        return_vec
    }

    fn push(&mut self, collider: Collider, transform: (Quat, Vec3)) {
        self.colliders.push(collider);
        self.offset.push(transform);
    }

    fn append(&mut self, c: &mut CompoundColliderBuilder) {
        self.colliders.append(&mut c.colliders);
        self.offset.append(&mut c.offset);
    }

    fn transform(&mut self, transform: (Quat, Vec3)) {
        let r_change = transform.0;
        let t_change = transform.1;

        for (r, t) in self.offset.iter_mut() {
            *r = r.mul_quat(r_change).normalize();
            t.add(t_change);
        }
    }

    fn with_transform(&self, transform: (Quat, Vec3)) -> Self {
        let r_change = transform.0;
        let t_change = transform.1;

        let mut return_ccb = self.to_owned();

        for (r, t) in return_ccb.offset.iter_mut() {
            *r = r.mul_quat(r_change).normalize();
            t.add(t_change);
        }
        return_ccb
    }

    fn append_with_transform(&mut self, c: CompoundColliderBuilder, transform: (Quat, Vec3)) {
        let mut e = c.to_owned(); 
        e.transform(transform);
        self.append(&mut e);
    }

    fn build(self) -> Collider {
        Collider::compound(self.to_vec())
    }
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

    let handle_temp = match meshes.get(&hollowground_handle) {
        Some(e) => e.clone(),
        _ => return,
    };

    if meshes.get(&ground1_handle).is_none() { return }
    if meshes.get(&spires_hollow_handle).is_none() { return }

    let mesh_handle = meshes.add(handle_temp);

    // let mut mesh = meshes.get(mesh_handle).unwrap().clone();
    let hollowground_ref = meshes.get(&hollowground_handle).unwrap();
    let ground1_ref = meshes.get(&ground1_handle).unwrap();
    let spires_hollow_ref = meshes.get(&spires_hollow_handle).unwrap();

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

    let mut world_gen_array: Vec<Vec<Vec<Option<TerrainBlockType>>>> = vec![vec![vec![None; 100]; 100]; 100];

    let mut attr = RelevantAttributes::new();

    let middle = Vec2::new(generator_options.radius as f32 / 2.0, generator_options.radius as f32 / 2.0);
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
                    if n > 0.5 {
                        let height = rng.gen_range(3..=7);
                        for y in 1..=height {
                            world_gen_array[i_usize][50 + y][j_usize] = Some(rng.random_pick(0.5, TerrainBlockType::Solid, TerrainBlockType::SpireHollow));
                        }
                    }
                }                
            }
        }
    }

    /* 
    for i in 0..generator_options.radius {
        let mut column_attr = RelevantAttributes::new();
        let mut column_colliders = Vec::new();
        for j in 0..generator_options.radius {
            let n = perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);
            let n_2 = spire_perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);
            let i_pos = (i as f32) * 3.0;
            let j_pos = (j as f32) * 3.0;
            
            let mut rng = rand::thread_rng();
            if n > 0.0 && distance_vec2(middle, Vec2::new(i_pos, j_pos)) < middle.x {
                let (mesh, mut coll) = rng.clone().random_pick(0.5, (ground1_ref.clone(), ground1_collider_vec.clone()), (hollowground_ref.clone(), hollowground_collider_vec.clone()));
                column_attr = column_attr.combine_with_mesh(
                    mesh,
                    Vec3::new(i_pos, 0.0, j_pos),
                );
                column_colliders.append(&mut coll);
                
                if n >= 0.3 {
                    let (mesh, mut coll) = rng.clone().random_pick(0.5, (ground1_ref.clone(), ground1_collider_vec.clone()), (hollowground_ref.clone(), hollowground_collider_vec.clone()));
                    column_attr = column_attr.combine_with_mesh(
                        mesh,
                        Vec3::new(i_pos, -3.0, j_pos),
                    );
                    column_colliders.append(&mut coll);
                }
                if n >= 0.6 {
                    let (mesh, mut coll) = rng.clone().random_pick(0.5, (ground1_ref.clone(), ground1_collider_vec.clone()), (hollowground_ref.clone(), hollowground_collider_vec.clone()));
                    column_attr = column_attr.combine_with_mesh(
                        mesh,
                        Vec3::new(i_pos, -6.0, j_pos),
                    );
                    column_colliders.append(&mut coll);
                }

                if n >= 0.95 {
                    let (mesh, mut coll) = rng.clone().random_pick(0.5, (ground1_ref.clone(), ground1_collider_vec.clone()), (hollowground_ref.clone(), hollowground_collider_vec.clone()));
                    column_attr = column_attr.combine_with_mesh(
                        mesh,
                        Vec3::new(i_pos, -9.0, j_pos),
                    );
                    column_colliders.append(&mut coll);
                }

                // Spires
                if n_2 > 0.5 {
                    let height = rng.gen_range(3..=7);
                    for x in 1..=height {
                        let (mesh, mut coll) = rng.clone().random_pick(0.5, (ground1_ref.clone(), ground1_collider_vec.clone()), (spires_hollow_ref.clone(), spires_hollow_collider_vec.clone()));
                        column_attr = column_attr.combine_with_mesh(
                            mesh,
                            Vec3::new(i_pos, (x as f32) * 3.0, j_pos),
                        );
                        column_colliders.append(&mut coll);
                    }
                }
            }
        }
        
        attr.append_with_indices(column_attr.clone());

        if !column_attr.pos.is_empty() {
            commands.spawn()
                .insert(Collider::compound(column_colliders))
                .insert(CollisionGroups { memberships: 0b0001, filters: 0b1110 })
                .insert(SolverGroups { memberships: 0b1110, filters: 0b0001 })
                .insert(ActiveCollisionTypes::all())
            ;
        }
    }
    */

    for (z, xy_plane) in world_gen_array.into_iter().enumerate() {
        let z_pos = z as f32 * 3.0;
        for (y, row) in xy_plane.into_iter().enumerate() {
            let y_pos = y as f32 * 3.0;
            for (x, i) in row.into_iter().enumerate() {
                if i.is_some() {
                    let x_pos = x as f32 * 3.0;
                    let translation = Vec3::new(x_pos, y_pos, z_pos);

                    let (mesh, collider) = match i.unwrap() {
                        TerrainBlockType::Solid => (ground1_ref.clone(), ground1_ccb.with_transform((Quat::IDENTITY, translation)).build()),
                        TerrainBlockType::Hollow => (hollowground_ref.clone(), hollowground_ccb.with_transform((Quat::IDENTITY, translation)).build()),
                        TerrainBlockType::SpireHollow => (spires_hollow_ref.clone(), spires_hollow_ccb.with_transform((Quat::IDENTITY, translation)).build()),
                    };

                    commands.spawn()
                        .insert(collider)
                        .insert(CollisionGroups { memberships: 0b0001, filters: 0b1110 })
                        .insert(SolverGroups { memberships: 0b1110, filters: 0b0001 })
                        .insert(ActiveCollisionTypes::STATIC_STATIC)
                    ;

                    attr = attr.combine_with_mesh(mesh, translation);
                }
            }
        }
    }

    // info!("{:?}", attr.pos);
    let mesh = Mesh::new(TriangleList).set_attributes(attr);
    
    let final_collider = mesh.clone().into_shared_shape();
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
        // .insert_bundle(ColliderBundle {
        //     shape: final_collider.into(),
        //     position: [0.0, 0.0, 0.0].into(),
        //     flags: ColliderFlags {
        //         active_collision_types: ActiveCollisionTypes::STATIC_STATIC.into(),
        //         ..Default::default()
        //     }.into(),
        //     ..Default::default()
        // });

    info!("Generation time: {:?}", time.elapsed());

    done.done = true;
}

fn spawn_terrain_block(
    commands: Commands,
    block_mesh: Mesh,
    block_collider: Vec<(Vec3, Quat, Collider)>,
) {

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

pub trait Vec3Operations {
    fn add(&mut self, e: Vec3);
}

impl Vec3Operations for Vec3 {
    fn add(&mut self, e: Vec3) {
        self.x += e.x;
        self.y += e.y;
        self.z += e.z;
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

