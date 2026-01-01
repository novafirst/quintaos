use crate::Mutex;
use crate::capability::Capability;

pub type EntityId = u64;

#[derive(Clone)]
pub struct Entity {
    pub id: EntityId,
    pub capabilities: &'static Mutex<[Option<Capability>; 16]>,
}

impl Entity {
    pub fn new(id: EntityId, capabilities: &'static Mutex<[Option<Capability>; 16]>) -> Self {
        Self { id, capabilities }
    }
}

