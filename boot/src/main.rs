#![no_std]
#![no_main]

use quintaos_kernel::{hlt_loop, Scheduler, Entity, Mutex, Capability, Rights, register_capability};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Minimal working example: create scheduler and entity
    static ENTITY_CAPS: Mutex<[Capability; 16]> = Mutex::new([Capability {
        id: 0,
        subject: 0,
        object: 0,
        rights: Rights::ALL,
        attenuated: false,
    }; 16]);

    let scheduler = Scheduler::new();

    let entity = Entity {
        id: 1,
        capabilities: &ENTITY_CAPS,
    };

    let _ = scheduler.add_entity(entity);

    hlt_loop();
}

