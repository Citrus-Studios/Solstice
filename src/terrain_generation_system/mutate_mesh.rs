use bevy::{
    math::Vec3,
    prelude::Mesh,
    render::{
        mesh::{Indices, MeshVertexAttribute, VertexAttributeValues},
        render_resource::VertexFormat,
    },
};
use bevy_rapier3d::rapier::prelude::{ColliderShape, SharedShape};
use nalgebra::Point3;

use super::relevant_attributes::RelevantAttributes;

pub trait MutateMesh {
    /// Combines two meshes
    ///
    /// **NOTE:** slow af, use `RelevantAttributes` as a builder struct instead
    fn combine_mesh(self, mesh_2: Mesh, offset: Vec3) -> Self;

    /// Extracts the `RelevantAttributes` of `self`
    fn relevant_attributes(self) -> RelevantAttributes;

    /// Turns `self` into a trimesh `SharedShape` (can be used as a collider)
    fn into_shared_shape(&self) -> SharedShape;

    /// Overwrites the attributes of a mesh
    fn set_attributes(&mut self, attr: RelevantAttributes);
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

        self.insert_attribute(VERTEX_POS_ATTR, VertexAttributeValues::Float32x3(pos));
        self.insert_attribute(VERTEX_NORM_ATTR, VertexAttributeValues::Float32x3(norm));
        self.insert_attribute(VERTEX_UV_ATTR, VertexAttributeValues::Float32x2(uvs));
        self.set_indices(Some(Indices::U32(indices)));

        self
    }

    fn relevant_attributes(self) -> RelevantAttributes {
        let positions = match self.attribute(VERTEX_POS_ATTR).unwrap() {
            VertexAttributeValues::Float32x3(e) => e.clone(),
            _ => panic!("WHAT"),
        };

        let normals = match self.attribute(VERTEX_NORM_ATTR).unwrap() {
            VertexAttributeValues::Float32x3(e) => e.clone(),
            _ => panic!("WHAT"),
        };

        let uvs = match self.attribute(VERTEX_UV_ATTR).unwrap() {
            VertexAttributeValues::Float32x2(e) => e.clone(),
            _ => panic!("WHAT"),
        };

        let indices = match self.indices().unwrap() {
            Indices::U32(e) => e.clone(),
            _ => panic!("WHAT"),
        };

        RelevantAttributes::new()
            .pos(positions)
            .norm(normals)
            .uv(uvs)
            .ind(indices)
    }

    fn into_shared_shape(&self) -> SharedShape {
        let attr = self.clone().relevant_attributes();

        let mut points: Vec<Point3<f32>> = Vec::new();
        for vertex in attr.pos {
            points.push(Point3::from_slice(&vertex));
        }

        // assert_eq!(0, indices.len() % 3);
        let mut indices = Vec::new();
        for i in 0..attr.ind.len() {
            if i % 3 == 0 {
                indices.push([attr.ind[i], attr.ind[i + 1], attr.ind[i + 2]]);
            }
        }

        ColliderShape::trimesh(points, indices)
    }

    fn set_attributes(&mut self, attr: RelevantAttributes) {
        self.insert_attribute(VERTEX_POS_ATTR, VertexAttributeValues::Float32x3(attr.pos));
        self.insert_attribute(
            VERTEX_NORM_ATTR,
            VertexAttributeValues::Float32x3(attr.norm),
        );
        self.insert_attribute(VERTEX_UV_ATTR, VertexAttributeValues::Float32x2(attr.uv));
        self.set_indices(Some(Indices::U32(attr.ind)));
    }
}

const VERTEX_POS_ATTR: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Position", 0, VertexFormat::Float32x3);
const VERTEX_NORM_ATTR: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Normal", 1, VertexFormat::Float32x3);
const VERTEX_UV_ATTR: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Uv", 2, VertexFormat::Float32x2);
