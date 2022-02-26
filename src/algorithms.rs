pub enum Connection {
    Open,
    None,
    ConnectedTo(T)
}

pub struct Pipe<T, U, V, W> {
    c1: Connection<T>,
    c2: Connection<U>,
    c3: Connection<V>,
    c4: Connection<W>,
    id: u32
}

impl Pipe<T, U, V, W> {
    pub fn new(con_1: Connection, con_2: Connection, con_3: Connection, con_4: Connection) -> Pipe {
        GLOBAL_PIPE_ID += 1;
        Pipe {
            c1: con_1,
            c2: con_2,
            c3: con_3,
            c4: con_4,
            id: GLOBAL_PIPE_ID
        }
    }

    pub fn connected_to(self, goal_id: u32) -> bool {

    }
}