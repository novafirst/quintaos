#![no_std]
#![no_main]

use quintaos_kernel::{
    Capability, Entity, EntityId,
    Rights, register_capability,
    Scheduler,
    Mutex,
    hlt_loop,
};

/// Genesis entity and capabilities
pub const GENESIS_ENTITY_ID: EntityId = 0;
pub static GENESIS_CAPABILITIES: Mutex<[Option<Capability>; 16]> =
    Mutex::new([const { None }; 16]);

/// Scheduler singleton
pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler {
    entities: Mutex::new([const { None }; 64]),
});

/// Kernel entry point
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Create the genesis entity
    let genesis_entity = Entity::new(GENESIS_ENTITY_ID, &GENESIS_CAPABILITIES);

    // Register the entity
    SCHEDULER
        .lock()
        .register_entity(genesis_entity)
        .expect("Failed to register genesis entity");

    // Create root capability
    let root_cap = Capability {
        id: 0,
        subject: GENESIS_ENTITY_ID,
        object: GENESIS_ENTITY_ID,
        rights: Rights::all(),
        attenuated: false,
    };

    // Register the capability
    register_capability(root_cap).expect("Failed to register root capability");

    // Enter halt loop
    hlt_loop();
}

/// Panic handler required for no_std
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hlt_loop();
}

