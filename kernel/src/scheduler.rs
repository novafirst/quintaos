use crate::{Entity, Mutex};

pub const MAX_ENTITIES: usize = 64;

pub struct Scheduler {
    pub entities: Mutex<[Option<Entity>; MAX_ENTITIES]>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            entities: Mutex::new([const { None }; MAX_ENTITIES]),
        }
    }

    pub fn register_entity(&self, entity: Entity) -> Result<(), ()> {
        let mut entities = self.entities.lock();
        for slot in entities.iter_mut() {
            if slot.is_none() {
                *slot = Some(entity);
                return Ok(());
            }
        }
        Err(())
    }
}

