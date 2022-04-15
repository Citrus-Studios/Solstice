use std::marker::PhantomData;

use bevy::prelude::{Mesh, Handle};
use bevy_rapier3d::prelude::SharedShape;

use crate::constants::GLOBAL_PIPE_ID;

pub enum BuildingType {
    Wellpump,
}

pub struct Building<M> {
    pub building_id: BuildingId,
    pub iridium_data: BuildingIridiumData,
    pub shape_data: BuildingShapeData,
    pub extra_data: Option<M>,
    pub _marker: PhantomData<M>,
}

pub struct BuildingId {
    pub building_type: BuildingType,
    pub building_name: String,
}

pub struct BuildingIridiumData {
    pub io: BuildingIO,
    pub storage: Option<u32>,
    pub current: Option<u32>,
    pub generation: Option<u32>,
}

pub enum BuildingIO {
    None,
    In,
    Out,
    InOut,
}

pub struct BuildingShapeData {
    pub mesh: Handle<Mesh>,
    pub collider: SharedShape,
}

pub enum Or<A, B> {
    A(A),
    B(B),
    None,
}

pub struct Pipe<T, U, V, W> {
    pub c1: Or<T, Building<T>>,
    pub c2: Or<U, Building<U>>,
    pub c3: Or<V, Building<V>>,
    pub c4: Or<W, Building<W>>,
    pub id: u32,
}

impl<T, U, V, W> Pipe<T, U, V, W> {
    pub fn new(
        c1: Or<T, Building<T>>,
        c2: Or<U, Building<U>>,
        c3: Or<V, Building<V>>,
        c4: Or<W, Building<W>>,
    ) -> Self {
        unsafe { GLOBAL_PIPE_ID += 1; }
        Self {
            c1,
            c2,
            c3,
            c4,
            id: unsafe { GLOBAL_PIPE_ID },
        }
    }
}