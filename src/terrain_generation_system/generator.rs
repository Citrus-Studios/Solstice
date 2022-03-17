use bevy::{prelude::*, render::mesh::{VertexAttributeValues, Indices}};
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

pub struct TerrainGenDone {
    pub done: bool
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
    indices: Vec<u32>
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
            indices: Vec::new()
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
        return
    }

    info!("hewo?");

    let hollowground_handle = asset_server.load("models/ground1/hollow_ground.obj");    

    // let commands = Arc::new(Box::leak(Box::new(commands)));
    // let hollowground_handle = &hollowground_handle;
    // let materials = Arc::new(materials);

    let perlin = Perlin::default().set_seed(*SEED);


    let handle_temp = match meshes.get(&hollowground_handle) {
        Some(e) => e.clone(),
        _ => return
    };

    let mesh_handle = meshes.add(handle_temp);
    
    // gen_block(
    //     &mut commands,
    //     hollowground_handle.clone(),
    //     &mut materials,
    //     meshes,
    //     0.0,
    //     0.0,
    //     Vec3::new(0.0, 0.0, 0.0),
    // );
    
    // generates terrain given a width and a length
    let mut terrain_gen = TerrainGenerator::new(
        hollowground_handle,
        &materials,
        meshes.as_ref().clone(),
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
                // let mut rng = rand::thread_rng();
                // Generate a block
                terrain_gen.offset.y = 0.0;
                    gen_block(&mut terrain_gen);
                if n >= 0.3 {
                    terrain_gen.offset.y = -3.0;
                    gen_block(&mut terrain_gen);
                }
                if n >= 0.6 {
                    terrain_gen.offset.y = -6.0;
                    gen_block(&mut terrain_gen);
                }

                if n >= 0.95 {
                    terrain_gen.offset.y = -9.0;
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
    let positions = terrain_gen.positions.clone();
    let normals = terrain_gen.normals.clone();
    let uvs = terrain_gen.uvs.clone();
    let indices = terrain_gen.indices.clone();
    // let textures = terrain_gen.textures.clone();
    // let colors = terrain_gen.colors.clone();

    drop(terrain_gen);

    let final_mesh = meshes.get_mut(mesh_handle).unwrap();

    final_mesh.set_attribute(
        "Vertex_Position",
        VertexAttributeValues::Float32x3(positions),
    );

    final_mesh.set_attribute(
        "Vertex_Normal",
        VertexAttributeValues::Float32x3(normals),
    );

    final_mesh.set_attribute(
        "Vertex_Uv",
        VertexAttributeValues::Float32x2(uvs),
    );

    final_mesh.set_indices(Some(Indices::U32(indices)));

    // final_mesh.set_attribute(
    //     "Vertex_Texture",
    //     VertexAttributeValues::Sint32x4(textures),
    // );

    // final_mesh.set_attribute(
    //     "Vertex_Color",
    //     VertexAttributeValues::Float32x4(colors),
    // );

    let final_mesh_clone = final_mesh.clone();
    // info!("{:?}", &final_mesh_clone);
    let final_mesh_handle = meshes.add(final_mesh_clone);

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
    
    done.done = true;
}

fn gen_block(
    terrain_gen: &mut TerrainGenerator
) {
    let hollowground_mesh = &mut terrain_gen.meshes.get(terrain_gen.hollowground_handle.clone()).unwrap().clone();

    let mut hollowground_mesh_clone = hollowground_mesh.clone();

    let hollowground_vertices = match hollowground_mesh_clone.attribute_mut("Vertex_Position").unwrap() {
        bevy::render::mesh::VertexAttributeValues::Float32x3(e) => e,
        _ => panic!("WHAT")
    };

    let mut hollowground_normals = match hollowground_mesh.clone().attribute("Vertex_Normal").unwrap() {
        bevy::render::mesh::VertexAttributeValues::Float32x3(e) => e.to_vec(),
        _ => panic!("WHAT")
    };

    let mut hollowground_uvs = match hollowground_mesh.clone().attribute("Vertex_Uv").unwrap() {
        bevy::render::mesh::VertexAttributeValues::Float32x2(e) => e.to_vec(),
        _ => panic!("WHAT")
    };

    let mut hollowground_indices = match hollowground_mesh.clone().indices().unwrap().clone() {
        Indices::U32(e) => e,
        _ => panic!("WHAT")
    };

    let mut final_vertices = Vec::new();

    let vertices_len = hollowground_vertices.clone().len();

    for vertex in hollowground_vertices {
        final_vertices.push([
            vertex[0] + ((terrain_gen.i as f32) * 3.0) + terrain_gen.offset.x, 
            vertex[1] + (-2.0) + terrain_gen.offset.y, 
            vertex[2] + ((terrain_gen.j as f32) * 3.0) + terrain_gen.offset.z
        ]);
    }

    let hollowground_indices_len = hollowground_indices.len();

    let len_cache = ((terrain_gen.indices.len() / hollowground_indices_len) * vertices_len) as u32;

    for indice in hollowground_indices.iter_mut() {
        *indice += len_cache;
    }

    terrain_gen.positions.append(&mut final_vertices);
    terrain_gen.normals.append(&mut hollowground_normals);
    terrain_gen.uvs.append(&mut hollowground_uvs);
    terrain_gen.indices.append(&mut hollowground_indices);
    // terrain_gen.textures.append(&mut hollowground_textures);
    // terrain_gen.colors.append(&mut hollowground_colors);

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
                vertice[2] + offset.z
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

        let self_clone = self.clone();
        let positions = match self_clone.attribute("Vertex_Position").unwrap() {
            VertexAttributeValues::Float32x3(e) => e,
            _ => panic!("WHAT")
        };

        let self_clone_2 = self.clone();
        let normals = match self_clone_2.attribute("Vertex_Normal").unwrap() {
            VertexAttributeValues::Float32x3(e) => e,
            _ => panic!("WHAT")
        };

        let self_clone_3 = self.clone();
        let uvs = match self_clone_3.attribute("Vertex_Uv").unwrap() {
            VertexAttributeValues::Float32x2(e) => e,
            _ => panic!("WHAT")
        };

        let self_clone_4 = self.clone();
        let indices = match self_clone_4.indices().unwrap() {
            Indices::U32(e) => e,
            _ => panic!("WHAT")
        };

        (positions.clone(), normals.clone(), uvs.clone(), indices.clone())
    }
}

