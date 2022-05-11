use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

#[derive(Clone)]
pub struct CompoundColliderBuilder {
    colliders: Vec<Collider>,
    offset: Vec<(Quat, Vec3)>,
}

impl CompoundColliderBuilder {
    pub fn new() -> Self {
        CompoundColliderBuilder { colliders: Vec::new(), offset: Vec::new() }
    }

    pub fn from_vec(vec: Vec<(Vec3, Quat, Collider)>) -> Self {
        let mut return_ccb = CompoundColliderBuilder::new();
        for (t, q, e) in vec {
            return_ccb.push(e, (q,t));
        }
        return_ccb
    }

    pub fn to_vec(self) -> Vec<(Vec3, Quat, Collider)> {
        let mut return_vec = Vec::new();
        for (c, (q, t)) in self.colliders.iter().zip(self.offset.iter()) {
            return_vec.push((t.to_owned(), q.to_owned(), c.to_owned()));
        }
        return_vec
    }

    pub fn push(&mut self, collider: Collider, transform: (Quat, Vec3)) {
        self.colliders.push(collider);
        self.offset.push(transform);
    }

    pub fn append(&mut self, c: &mut CompoundColliderBuilder) {
        self.colliders.append(&mut c.colliders);
        self.offset.append(&mut c.offset);
    }

    pub fn transform(&mut self, transform: (Quat, Vec3)) {
        let r_change = transform.0;
        let t_change = transform.1;

        for (r, t) in self.offset.iter_mut() {
            *r = r.mul_quat(r_change).normalize();
            t.add(t_change);
        }
    }

    pub fn with_transform(&self, transform: (Quat, Vec3)) -> Self {
        let r_change = transform.0;
        let t_change = transform.1;

        let mut return_ccb = self.to_owned();

        for (r, t) in return_ccb.offset.iter_mut() {
            *r = r.mul_quat(r_change).normalize();
            t.add(t_change);
        }
        return_ccb
    }

    pub fn append_with_transform(&mut self, c: CompoundColliderBuilder, transform: (Quat, Vec3)) {
        let mut e = c.to_owned(); 
        e.transform(transform);
        self.append(&mut e);
    }

    pub fn build(&self) -> Collider {
        Collider::compound(self.to_owned().to_vec())
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