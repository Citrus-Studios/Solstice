use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_mod_raycast::RayCastMesh;
use bevy_rapier3d::{
    physics::{ColliderBundle, ColliderPositionSync, RigidBodyBundle},
    prelude::{ColliderShape, SharedShape, RigidBodyMassPropsFlags, RigidBodyType, ActiveCollisionTypes, ColliderFlags, InteractionGroups},
};
use bevy::render::render_resource::PrimitiveTopology::TriangleList;
use nalgebra::{Vector3, Point3, Isometry3, OPoint, Point};
use rand::{Rng, prelude::ThreadRng};

use noise::{NoiseFn, Perlin, Seedable, Terrace};

use crate::{constants::SEED, RaycastSet, algorithms::distance_vec2};

use super::{relevant_attributes::RelevantAttributes, mutate_mesh::MutateMesh};

#[derive(Component)]
pub struct GeneratorOptions {
    pub width: u32,
    pub length: u32,
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
    mut done: ResMut<TerrainGenDone>,

    generator_options: Res<GeneratorOptions>,
) {
    if done.done {
        return;
    }

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

    let hollowground_collider = hollowground_ref.clone().into_shared_shape();
    let ground1_collider = ColliderShape::cuboid(1.5, 1.5, 1.5); // probably slightly faster than a trimesh collider
    let spires_hollow_collider = spires_hollow_ref.clone().into_shared_shape();

    // let hollowground_collider = ColliderShape::trimesh(points, indices);
    // let ground1_collider = ColliderShape::cuboid(1.5, 1.5, 1.5);

    // let mut compound_colliders = vec![];

    let mut attr = RelevantAttributes::new();

    let middle = Vec2::new(generator_options.width as f32 / 2.0, generator_options.length as f32 / 2.0);
    // generates terrain given a width and a length
    for i in 0..generator_options.width {
        let mut column_attr = RelevantAttributes::new();
        for j in 0..generator_options.length {
            let n = perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);
            let n_2 = spire_perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);
            let i_pos = (i as f32) * 3.0;
            let j_pos = (j as f32) * 3.0;
            //info!(n);
            let mut rng = rand::thread_rng();
            if n > 0.0 && distance_vec2(middle, Vec2::new(i_pos, j_pos)) < middle.x {
                let mesh = rng.clone().random_pick(0.5, ground1_ref.clone(), hollowground_ref.clone());
                column_attr = column_attr.combine_with_mesh(
                    mesh.clone(),
                    Vec3::new(i_pos, 0.0, j_pos),
                );
                if n >= 0.3 {
                    let mesh = rng.clone().random_pick(0.5, ground1_ref.clone(), hollowground_ref.clone());
                    column_attr = column_attr.combine_with_mesh(
                        mesh,
                        Vec3::new(i_pos, -3.0, j_pos),
                    );
                }
                if n >= 0.6 {
                    let mesh = rng.clone().random_pick(0.5, ground1_ref.clone(), hollowground_ref.clone());
                    column_attr = column_attr.combine_with_mesh(
                        mesh,
                        Vec3::new(i_pos, -6.0, j_pos),
                    );
                }

                if n >= 0.95 {
                    let mesh = rng.clone().random_pick(0.5, ground1_ref.clone(), hollowground_ref.clone());
                    column_attr = column_attr.combine_with_mesh(
                        mesh,
                        Vec3::new(i_pos, -9.0, j_pos),
                    );
                }

                // Spires
                if n_2 > 0.5 {
                    let height = rng.gen_range(3..=7);
                    for x in 1..=height {
                        let mesh = rng.clone().random_pick(0.5, ground1_ref.clone(), spires_hollow_ref.clone());
                        column_attr = column_attr.combine_with_mesh(
                            mesh,
                            Vec3::new(i_pos, (x as f32) * 3.0, j_pos),
                        );
                    }
                }
            }
        }
        
        attr = attr.append_with_indices(column_attr.clone());

        if !column_attr.pos.is_empty() {
            commands.spawn_bundle(ColliderBundle {
                shape: Mesh::new(TriangleList).set_attributes(column_attr).into_shared_shape().into(),
                flags: ColliderFlags {
                    collision_groups: InteractionGroups::new(0b0001, 0b1110),
                    solver_groups: InteractionGroups::new(0b1110, 0b0001),
                    active_collision_types: ActiveCollisionTypes::STATIC_STATIC.into(),
                    ..Default::default()
                }.into(),
                ..Default::default()
            });
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

    done.done = true;
}

pub trait Pick<T> {
    fn random_pick(self, bias: f32, n1: T, n2: T) -> T;
}

impl<T> Pick<T> for ThreadRng {
    // Bias is the bias towards n1
    fn random_pick(mut self, bias: f32, n1: T, n2: T) -> T {
        if !(0.0 <= bias && bias <= 1.0) { println!("Warning: Bias should be between 0.0 and 1.0"); }
        if self.gen_range(0.0..1.0) <= bias {
            return n1;
        } else {
            return n2;
        }
    }
}



trait SpawnCollider {
    fn spawn_locked_collider(&mut self, col: SharedShape, trans: Vec3);
}

impl SpawnCollider for Commands<'_, '_> {
    fn spawn_locked_collider(&mut self, col: SharedShape, trans: Vec3) {
        self.spawn_bundle(ColliderBundle {
            shape: col.into(),
            position: trans.into(),
            flags: ColliderFlags {
                active_collision_types: ActiveCollisionTypes::STATIC_STATIC.into(),
                ..Default::default()
            }.into(),
            ..Default::default()
        });
    }
}

