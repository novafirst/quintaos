#![no_std]
#![feature(abi_x86_interrupt)]

pub mod entity;
pub mod capability;
pub mod scheduler;
pub mod message;

pub use entity::{Entity, EntityId};
pub use capability::{Capability, CapabilityId, Rights, register_capability, revoke, CAPABILITY_REGISTRY};
pub use scheduler::{Scheduler, MAX_ENTITIES};
pub use message::{Message, MessageQueue, MessageId, MessageType, MAX_MESSAGES};

pub use spin::Mutex;
pub use x86_64::instructions::hlt;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

