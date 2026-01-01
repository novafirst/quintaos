#![no_std]
#![cfg(test)]

use super::*;
use spin::Mutex;

// Test static registry for isolated tests
static TEST_ENTITY_CAPS: Mutex<[Option<Capability>; 16]> = Mutex::new([None; 16]);
static TEST_ENTITY_REGISTRY: Mutex<[Option<Entity>; 16]> = Mutex::new([None; 16]);

#[test_case]
fn test_entity_creation() {
    let entity = Entity::new(42, &TEST_ENTITY_CAPS);
    TEST_ENTITY_REGISTRY.lock()[0] = Some(entity);

    let registry = TEST_ENTITY_REGISTRY.lock();
    assert!(registry[0].is_some(), "Entity was not stored in registry");
    assert_eq!(registry[0].as_ref().unwrap().id, 42);
}

#[test_case]
fn test_capability_registration() {
    let cap = Capability {
        id: 1,
        subject: 42,
        object: 42,
        rights: Rights::INVOKE | Rights::READ,
        attenuated: false,
    };

    // Clear registry first
    let mut registry = CAPABILITY_REGISTRY.lock();
    for slot in registry.iter_mut() {
        *slot = None;
    }

    // Register capability
    register_capability(cap).expect("Failed to register capability");

    // Verify
    let registry = CAPABILITY_REGISTRY.lock();
    assert!(registry.iter().any(|s| matches!(s, Some(c) if c.id == 1)));
}

#[test_case]
fn test_capability_attenuation() {
    let cap = Capability {
        id: 2,
        subject: 42,
        object: 42,
        rights: Rights::INVOKE | Rights::READ | Rights::WRITE,
        attenuated: false,
    };

    let attenuated_cap = cap.attenuate(Rights::INVOKE | Rights::READ).expect("Attenuation failed");
    assert!(attenuated_cap.can(Rights::INVOKE));
    assert!(attenuated_cap.can(Rights::READ));
    assert!(!attenuated_cap.can(Rights::WRITE));
    assert!(attenuated_cap.attenuated, "Cap should be marked as attenuated");
}

#[test_case]
fn test_capability_revocation() {
    let cap = Capability {
        id: 99,
        subject: 1,
        object: 1,
        rights: Rights::all(),
        attenuated: false,
    };

    // Register and then revoke
    register_capability(cap).expect("Failed to register capability");
    let revoked = revoke(99);
    assert!(revoked, "Capability was not revoked");

    let registry = CAPABILITY_REGISTRY.lock();
    assert!(!registry.iter().any(|s| matches!(s, Some(c) if c.id == 99)));
}

