use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_mod_raycast::RayCastMesh;
use bevy_rapier3d::{
    physics::{ColliderBundle, ColliderPositionSync},
    prelude::ColliderShape,
};
use nalgebra::{Vector3, Point3, Isometry3};
use rand::{Rng, prelude::ThreadRng};

use noise::{NoiseFn, Perlin, Seedable, Terrace};

use crate::{constants::SEED, RaycastSet, algorithms::distance_vec2};

#[derive(Component)]
pub struct GeneratorOptions {
    pub width: u32,
    pub length: u32,
    pub height: u32,
}

pub struct TerrainGenDone {
    pub done: bool,
}

struct TerrainGenerator<'a> {
    hollowground_handle: Handle<Mesh>,
    materials: &'a ResMut<'a, Assets<StandardMaterial>>,
    meshes: &'a Assets<Mesh>,
    i: f32,
    j: f32,
    offset: Vec3,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
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
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
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

    info!("hewo?");

    let ground1_handle: Handle<Mesh> = asset_server.load("models/ground1/ground1.obj");
    let hollowground_handle: Handle<Mesh> = asset_server.load("models/ground1/hollow_ground.obj");
    

    // let commands = Arc::new(Box::leak(Box::new(commands)));
    // let hollowground_handle = &hollowground_handle;
    // let materials = Arc::new(materials);

    let perlin = Perlin::default().set_seed(*SEED);

    let handle_temp = match meshes.get(&hollowground_handle) {
        Some(e) => e.clone(),
        _ => return,
    };

    if meshes.get(&ground1_handle).is_none() { return }

    let mesh_handle = meshes.add(handle_temp);

    let mut mesh = meshes.get(mesh_handle).unwrap().clone();
    let hollowground_ref = meshes.get(&hollowground_handle).unwrap();
    let ground1_ref = meshes.get(&ground1_handle).unwrap();

    let mut compound_colliders = vec![];

    let middle = Vec2::new(generator_options.width as f32 / 2.0, generator_options.length as f32 / 2.0);
    // generates terrain given a width and a length
    for i in 0..generator_options.width {
        for j in 0..generator_options.length {
            let n = perlin.get([(i as f64) * 0.15, (j as f64) * 0.15]);
            let i_pos = (i as f32) * 3.0;
            let j_pos = (j as f32) * 3.0;
            //info!(n);
            let rng = rand::thread_rng();
            if n > 0.0 && distance_vec2(middle, Vec2::new(i_pos, j_pos)) < middle.x {
                mesh = mesh.combine_mesh(
                    rng.clone().random_pick(0.5, ground1_ref, hollowground_ref).clone(),
                    Vec3::new(i_pos, 0.0, j_pos),
                );
                compound_colliders.push((
                    Isometry3::translation(i_pos, 0.0, j_pos),
                    ColliderShape::cuboid(1.5, 1.5, 1.5),
                ));
                if n >= 0.3 {
                    compound_colliders.push((
                        Isometry3::translation(i_pos, -3.0, j_pos),
                        ColliderShape::cuboid(1.5, 1.5, 1.5),
                    ));
                    mesh = mesh.combine_mesh(
                        rng.clone().random_pick(0.5, ground1_ref, hollowground_ref).clone(),
                        Vec3::new(i_pos, -3.0, j_pos),
                    );
                }
                if n >= 0.6 {
                    compound_colliders.push((
                        Isometry3::translation(i_pos, -6.0, j_pos),
                        ColliderShape::cuboid(1.5, 1.5, 1.5),
                    ));
                    mesh = mesh.combine_mesh(
                        rng.clone().random_pick(0.5, ground1_ref, hollowground_ref).clone(),
                        Vec3::new(i_pos, -6.0, j_pos),
                    );
                }

                if n >= 0.95 {
                    compound_colliders.push((
                        Isometry3::translation(i_pos, -9.0, j_pos),
                        ColliderShape::cuboid(1.5, 1.5, 1.5),
                    ));
                    mesh = mesh.combine_mesh(
                        rng.clone().random_pick(0.5, ground1_ref, hollowground_ref).clone(),
                        Vec3::new(i_pos, -9.0, j_pos),
                    );
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
        .insert(RayCastMesh::<RaycastSet>::default())
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::compound(compound_colliders).into(),
            ..Default::default()
        });

    done.done = true;
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

pub trait Pick<T> {
    fn random_pick(self, bias: f32, n1: T, n2: T) -> T;
}

impl<T> Pick<T> for ThreadRng {
    // Bias is the bias towards n1
    fn random_pick(mut self, bias: f32, n1: T, n2: T) -> T {
        if !(0.0 <= bias && bias <= 1.0) { panic!("Bias must be between 0.0 and 1.0") }
        if self.gen_range(0.0..1.0) <= bias {
            return n1;
        } else {
            return n2;
        }
    }
}

pub trait CombineMesh {
    fn combine_mesh(self, mesh_2: Mesh, offset: Vec3) -> Self;
    fn relevant_attributes(self) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>);
}

impl CombineMesh for Mesh {
    fn combine_mesh(mut self, mesh_2: Mesh, offset: Vec3) -> Self {
        let (pos_1, norm_1, uvs_1, indices_1) = self.clone().relevant_attributes();
        let (pos_2, norm_2, uvs_2, indices_2) = mesh_2.relevant_attributes();

        let mut pos_offset = Vec::new();

        for vertice in pos_2 {
            pos_offset.push([
                vertice[0] + offset.x,
                vertice[1] + offset.y,
                vertice[2] + offset.z,
            ]);
        }

        let num_vertices = pos_1.clone().len() as u32;

        let mut indices_offset = Vec::new();

        for indice in indices_2 {
            indices_offset.push(indice + num_vertices);
        }

        let pos = vec![pos_1.clone(), pos_offset].concat();
        let norm = vec![norm_1.clone(), norm_2.clone()].concat();
        let uvs = vec![uvs_1.clone(), uvs_2.clone()].concat();
        let indices = vec![indices_1.clone(), indices_offset].concat();

        self.set_attribute("Vertex_Position", VertexAttributeValues::Float32x3(pos));
        self.set_attribute("Vertex_Normal", VertexAttributeValues::Float32x3(norm));
        self.set_attribute("Vertex_Uv", VertexAttributeValues::Float32x2(uvs));
        self.set_indices(Some(Indices::U32(indices)));

        self
    }

    fn relevant_attributes(self) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>) {
        let positions = match self.attribute("Vertex_Position").unwrap() {
            VertexAttributeValues::Float32x3(e) => e.clone(),
            _ => panic!("WHAT"),
        };

        let normals = match self.attribute("Vertex_Normal").unwrap() {
            VertexAttributeValues::Float32x3(e) => e.clone(),
            _ => panic!("WHAT"),
        };

        let uvs = match self.attribute("Vertex_Uv").unwrap() {
            VertexAttributeValues::Float32x2(e) => e.clone(),
            _ => panic!("WHAT"),
        };

        let indices = match self.indices().unwrap() {
            Indices::U32(e) => e.clone(),
            _ => panic!("WHAT"),
        };

        (positions, normals, uvs, indices)
    }
}
