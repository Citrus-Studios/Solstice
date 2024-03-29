use bevy::{prelude::*, render::mesh::PrimitiveTopology};

use super::mutate_mesh::MutateMesh;

/// The relevant vertex attributes of a `Mesh`
#[derive(Clone, Debug)]
pub struct RelevantAttributes {
    pub pos: Vec<[f32; 3]>,
    pub norm: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    pub ind: Vec<u32>,
}

impl RelevantAttributes {
    pub fn new() -> RelevantAttributes {
        RelevantAttributes {
            pos: Vec::new(),
            norm: Vec::new(),
            uv: Vec::new(),
            ind: Vec::new(),
        }
    }

    pub fn pos(mut self, pos: Vec<[f32; 3]>) -> Self {
        self.pos = pos;
        self
    }

    pub fn norm(mut self, norm: Vec<[f32; 3]>) -> Self {
        self.norm = norm;
        self
    }

    pub fn uv(mut self, uv: Vec<[f32; 2]>) -> Self {
        self.uv = uv;
        self
    }

    pub fn ind(mut self, ind: Vec<u32>) -> Self {
        self.ind = ind;
        self
    }

    /// Sets all the UVs of `self` to the specified position
    pub fn set_all_uv(&mut self, uv_set: Vec2) {
        for uv in self.uv.iter_mut() {
            uv[0] = uv_set.x;
            uv[1] = uv_set.y;
        }
    }

    /// Appends another `RelevantAttributes` to `self` with respect to indices
    pub fn append_with_indices(&mut self, mut attr: RelevantAttributes) {
        let add = self.pos.len() as u32;
        self.pos.append(&mut attr.pos);
        self.norm.append(&mut attr.norm);
        self.uv.append(&mut attr.uv);
        for i in attr.ind.iter_mut() {
            *i += add;
        }
        self.ind.append(&mut attr.ind);
    }

    /// Appends another `RelevantAttributes` to `self`
    pub fn append(mut self, mut attr: RelevantAttributes) -> Self {
        self.pos.append(&mut attr.pos);
        self.norm.append(&mut attr.norm);
        self.uv.append(&mut attr.uv);
        self.ind.append(&mut attr.ind);
        self
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.pos.iter_mut().for_each(|p| {
            p[0] += translation.x;
            p[1] += translation.y;
            p[2] += translation.z;
        });
    }

    /// Extracts `mesh`'s `RelevantAttributes` and appends them to `self`
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

impl Into<Mesh> for RelevantAttributes {
    fn into(self) -> Mesh {
        let mut ret = Mesh::new(PrimitiveTopology::TriangleList);
        ret.set_attributes(self);
        ret
    }
}
