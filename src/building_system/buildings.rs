use std::marker::PhantomData;

use crate::constants::GLOBAL_PIPE_ID;

pub enum BuildingType {
    Wellpump,
}

pub struct Building<M> {
    pub building_type: BuildingType,
    pub io: u8,
    pub storage: Option<u32>,
    pub current: Option<u32>,
    pub generation: Option<u32>,
    pub extra_data: Option<M>,
    pub _marker: PhantomData<M>,
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