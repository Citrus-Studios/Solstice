use std::{rc::Rc, sync::Mutex};

use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use bevy_mod_raycast::RayCastMesh;
use bevy_rapier3d::{
    physics::{ColliderBundle, ColliderPositionSync},
    prelude::ColliderShape,
};
use rand::Rng;

use noise::{NoiseFn, Perlin, Seedable, Terrace};

use crate::{constants::SEED, RaycastSet};

#[derive(Component)]
pub struct GeneratorOptions {
    pub width: u32,
    pub length: u32,
    pub height: u32,
}

struct TerrainGenerator<'a> {
    hollowground_handle: Handle<Mesh>,
    materials: &'a ResMut<'a, Assets<StandardMaterial>>,
    meshes: &'a Assets<Mesh>,
    i: f32,
    j: f32,
    offset: Vec3,
    positions: Vec<[f32; 3]>,
}

impl<'a> TerrainGenerator<'a> {
    pub fn new(
        hollowground_handle: Handle<Mesh>,
        materials: &'a ResMut<'a, Assets<StandardMaterial>>,
        meshes: &'a Assets<Mesh>,
        i: f32,
        j: f32,
        offset: Vec3,
    ) -> Self {
        TerrainGenerator {
            hollowground_handle,
            materials: &materials,
            meshes,
            i,
            j,
            offset,
            positions: Vec::new()
        }
    }
}

pub fn generate_terrain(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,

    generator_options: Res<GeneratorOptions>,
) {
    let meshes_rc = Rc::new(Mutex::new(meshes));
    let hollowground_handle = asset_server.load("models/ground1/hollow_ground.obj");

    // let commands = Arc::new(Box::leak(Box::new(commands)));
    // let hollowground_handle = &hollowground_handle;
    // let materials = Arc::new(materials);

    let perlin = Perlin::default().set_seed(*SEED);


    let handle_temp = match meshes_rc.clone().lock().unwrap().get(&hollowground_handle) {
        Some(e) => e.clone(),
        _ => return
    };

    let mesh_handle = meshes_rc.clone().lock().unwrap().add(handle_temp);
    
    // gen_block(
    //     &mut commands,
    //     hollowground_handle.clone(),
    //     &mut materials,
    //     meshes,
    //     0.0,
    //     0.0,
    //     Vec3::new(0.0, 0.0, 0.0),
    // );

    let meshes_rc_clone = meshes_rc.clone();
    let meshes_rc_clone_lock_unwrap = meshes_rc_clone.lock().unwrap();
    
    // generates terrain given a width and a length
    let mut terrain_gen = TerrainGenerator::new(
        hollowground_handle,
        &materials,
        meshes_rc_clone_lock_unwrap.as_ref().clone(),
        0.0,
        0.0,
        Vec3::ZERO,
    );

    for i in 0..generator_options.width {
        for j in 0..generator_options.length {
            let n = perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);
            //info!(n);
            if n > 0.0 {
                terrain_gen.i = i as f32;
                terrain_gen.j = j as f32;
                let mut rng = rand::thread_rng();
                // Generate a block
                    gen_block(&mut terrain_gen);
                if rng.gen_range(1..=10) >= 5 {
                    gen_block(&mut terrain_gen);
                }
                if rng.gen_range(1..=10) >= 3 {
                    gen_block(&mut terrain_gen);
                }

                // Spires
                // if rng.gen_range(1..=10) >= 9 {
                //     let height = rng.gen_range(3..=10);
                //     for x in 1..=height {
                //         // Generate a spire
                //         gen_spire(
                //             &mut commands,
                //             hollowground_handle.clone(),
                //             &mut materials,
                //             i as f32,
                //             j as f32,
                //             x as f32,
                //         );
                //     }
                // }
            }
        }
    }
    let meshes_rc_clone = meshes_rc.clone();
    let mut meshes_rc_clone_lock_unwrap = meshes_rc_clone.lock().unwrap();
    let final_mesh = Rc::new(Mutex::new(meshes_rc_clone_lock_unwrap.get_mut(mesh_handle).unwrap()));

    final_mesh.clone().lock().unwrap().set_attribute(
        "Vertex_Position",
        VertexAttributeValues::Float32x3(terrain_gen.positions),
    );
    let meshes_rc_clone = meshes_rc.clone();
    let mut meshes_rc_clone_lock_unwrap = meshes_rc_clone.lock().unwrap();
    let final_mesh_handle = meshes_rc_clone_lock_unwrap.add(final_mesh.clone().lock().unwrap().clone());

    info!("{:?}", final_mesh_handle);

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
}

fn gen_block(
    terrain_gen: &mut TerrainGenerator
) {
    let mut hollowground_mesh = match terrain_gen.meshes.get(terrain_gen.hollowground_handle.clone()).unwrap().attribute("Vertex_Position").unwrap() {
        bevy::render::mesh::VertexAttributeValues::Float32x3(e) => e.to_vec(),
        _ => panic!("WHAT")
    };

    for vertex in hollowground_mesh.iter_mut() {
        vertex[0] += ((terrain_gen.i as f32) * 3.0) + terrain_gen.offset.x;
        vertex[1] += (-2.0) + terrain_gen.offset.y;
        vertex[2] += ((terrain_gen.j as f32) * 3.0) + terrain_gen.offset.z;
    }

    terrain_gen.positions.append(&mut hollowground_mesh);

    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: hollowground_handle.clone(),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::rgb(50.0 / 255.0, 56.0 / 255.0, 53.0 / 255.0),
    //             ..Default::default()
    //         }),
    //         transform: Transform::from_xyz(
    //             ((i as f32) * 3.0) + offset.x,
    //             (-2.0) + offset.y,
    //             ((j as f32) * 3.0) + offset.z,
    //         )
    //         //.with_scale(Vec3::new(0.5, 0.5, 0.5))
    //         ,
    //         ..Default::default()
    //     })
    //     .insert(RayCastMesh::<RaycastSet>::default())
    //     .insert(ColliderPositionSync::Discrete)
    //     .insert(RayCastMesh::<RaycastSet>::default())
    //     .with_children(|parent| {
    //         parent.spawn_bundle(ColliderBundle {
    //             shape: ColliderShape::cuboid(1.5, 1.5, 1.5).into(),
    //             position: (Vec3::new((i as f32) * 3.0, -2.0, (j as f32) * 3.0) + offset).into(),
    //             ..Default::default()
    //         });
    //     });
}


fn gen_spire(
    commands: &mut Commands,
    hollowground_handle: Handle<Mesh>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    i: f32,
    j: f32,
    x: f32,
) {


    // commands
    // .spawn_bundle(PbrBundle {
    //     mesh: hollowground_handle.clone(),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::rgb(153.0 / 255.0, 132.0 / 255.0, 0.0),
    //         ..Default::default()
    //     }),
    //     transform: Transform::from_xyz(
    //         (i as f32) * 3.0,
    //         x as f32,
    //         (j as f32) * 3.0,
    //     )
    //     //.with_scale(Vec3::new(0.5, 0.5, 0.5))
    //     ,
    //     ..Default::default()
    // })
    // .insert(RayCastMesh::<RaycastSet>::default())
    // .insert(ColliderPositionSync::Discrete)
    // .insert(RayCastMesh::<RaycastSet>::default())
    // .with_children(|parent| {
    //     parent.spawn_bundle(ColliderBundle {
    //         shape: ColliderShape::cuboid(1.5, 1.5, 1.5).into(),
    //         position: Vec3::new(
    //             (i as f32) * 3.0,
    //             x as f32,
    //             (j as f32) * 3.0,
    //         )
    //         .into(),
    //         ..Default::default()
    //     });
    // });
}