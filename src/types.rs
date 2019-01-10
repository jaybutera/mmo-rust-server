use std::collections::HashMap;

// Just an id and a discrete 1D position for now
pub struct Entity {
    pub id: u32,
    pub pos: (i32,i32),
}

impl Entity {
    pub fn to_json(&self) -> String {
        format!("{{\"position\":{{\"x\":{},\"y\":{}}}, \"id\":{}}}", self.pos.0, self.pos.1, self.id)
    }
}

pub type Entities = HashMap<u32,Entity>;
