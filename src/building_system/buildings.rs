use std::marker::PhantomData;

use bevy::prelude::{Mesh, Handle};
use bevy_rapier3d::prelude::SharedShape;

use crate::constants::GLOBAL_PIPE_ID;

pub enum BuildingType {
    Wellpump,
}

pub struct Building {
    pub building_id: BuildingId,
    pub iridium_data: BuildingIridiumData,
    pub shape_data: BuildingShapeData,
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
    pub c1: Or<T, Building>,
    pub c2: Or<U, Building>,
    pub c3: Or<V, Building>,
    pub c4: Or<W, Building>,
    pub id: u32,
}

impl<T, U, V, W> Pipe<T, U, V, W> {
    pub fn new(
        c1: Or<T, Building>,
        c2: Or<U, Building>,
        c3: Or<V, Building>,
        c4: Or<W, Building>,
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