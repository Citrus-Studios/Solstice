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

#[derive(Clone, Debug)]
pub struct RelevantAttributes {
    pub pos: Vec<[f32; 3]>,
    pub norm: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    pub ind: Vec<u32>
}

impl RelevantAttributes {
    pub fn new() -> RelevantAttributes {
        RelevantAttributes {
            pos: Vec::new(),
            norm: Vec::new(),
            uv: Vec::new(),
            ind: Vec::new()
        }
    }

    pub fn pos(mut self, pos: Vec<[f32; 3]>) -> Self {
        self.pos = pos; self
    }

    pub fn norm(mut self, norm: Vec<[f32; 3]>) -> Self {
        self.norm = norm; self
    }

    pub fn uv(mut self, uv: Vec<[f32; 2]>) -> Self {
        self.uv = uv; self
    }

    pub fn ind(mut self, ind: Vec<u32>) -> Self {
        self.ind = ind; self
    }

    pub fn append_with_indices(mut self, mut attr: RelevantAttributes) -> Self {
        let add = self.pos.len() as u32;
        self.pos.append(&mut attr.pos);
        self.norm.append(&mut attr.norm);
        self.uv.append(&mut attr.uv);
        for i in attr.ind.iter_mut() {
            *i += add;
        }
        self.ind.append(&mut attr.ind);
        self
    }

    pub fn append(mut self, mut attr: RelevantAttributes) -> Self {
        self.pos.append(&mut attr.pos);
        self.norm.append(&mut attr.norm);
        self.uv.append(&mut attr.uv);
        self.ind.append(&mut attr.ind);
        self
    }

    pub fn combine_with_mesh(self, mesh: Mesh, offset: Vec3) -> Self {
        let mut attr = mesh.relevant_attributes();
        for vertice in attr.pos.iter_mut() {
            for (i, coord) in vertice.into_iter().enumerate() {
                *coord += offset[i];
            }
        }

        let num_vertices = self.pos.len() as u32;
        for indice in attr.ind.iter_mut() {
            *indice += num_vertices;
        }

        self.append(attr)
    }
}

pub trait MutateMesh {
    fn combine_mesh(self, mesh_2: Mesh, offset: Vec3) -> Self;
    fn relevant_attributes(self) -> RelevantAttributes;
    fn into_shared_shape(self) -> SharedShape;
    fn set_attributes(self, attr: RelevantAttributes) -> Mesh;
}

impl MutateMesh for Mesh {
    fn combine_mesh(mut self, mesh_2: Mesh, offset: Vec3) -> Self {
        let attr_1 = self.clone().relevant_attributes();
        let attr_2 = mesh_2.relevant_attributes();

        let mut pos_offset = Vec::new();

        for vertice in attr_2.pos {
            pos_offset.push([
                vertice[0] + offset.x,
                vertice[1] + offset.y,
                vertice[2] + offset.z,
            ]);
        }

        let num_vertices = attr_1.pos.clone().len() as u32;

        let mut indices_offset = Vec::new();

        for indice in attr_2.ind {
            indices_offset.push(indice + num_vertices);
        }

        let pos = vec![attr_1.pos.clone(), pos_offset].concat();
        let norm = vec![attr_1.norm.clone(), attr_2.norm.clone()].concat();
        let uvs = vec![attr_1.uv.clone(), attr_2.uv.clone()].concat();
        let indices = vec![attr_1.ind.clone(), indices_offset].concat();

        self.set_attribute("Vertex_Position", VertexAttributeValues::Float32x3(pos));
        self.set_attribute("Vertex_Normal", VertexAttributeValues::Float32x3(norm));
        self.set_attribute("Vertex_Uv", VertexAttributeValues::Float32x2(uvs));
        self.set_indices(Some(Indices::U32(indices)));

        self
    }

    fn relevant_attributes(self) -> RelevantAttributes {
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

        RelevantAttributes::new().pos(positions).norm(normals).uv(uvs).ind(indices)
    }

    fn into_shared_shape(self) -> SharedShape {
        let attr = self.clone().relevant_attributes();

        let mut points: Vec<Point3<f32>> = Vec::new();
        for vertex in attr.pos {
            points.push(Point3::from_slice(&vertex));
        }

        // assert_eq!(0, indices.len() % 3);
        let mut indices = Vec::new();
        for i in 0..attr.ind.len() {
            if i % 3 == 0 {
                indices.push([attr.ind[i], attr.ind[i+1], attr.ind[i+2]]);
            }
        }    

        ColliderShape::trimesh(points, indices)
    }

    fn set_attributes(mut self, attr: RelevantAttributes) -> Mesh {
        self.set_attribute("Vertex_Position", VertexAttributeValues::Float32x3(attr.pos));
        self.set_attribute("Vertex_Normal", VertexAttributeValues::Float32x3(attr.norm));
        self.set_attribute("Vertex_Uv", VertexAttributeValues::Float32x2(attr.uv));
        self.set_indices(Some(Indices::U32(attr.ind)));
        self
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

