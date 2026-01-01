#![no_std]

use spin::Mutex;
use crate::EntityId;

pub type StateId = u64;

pub const MAX_STATES: usize = 128;

pub struct State {
    pub id: StateId,
    pub owner: EntityId,
    pub data: u64, // ephemeral/persistent state placeholder
}

// Kernel-level state registry
pub static STATE_REGISTRY: Mutex<[Option<State>; MAX_STATES]> = Mutex::new([None; MAX_STATES]);

