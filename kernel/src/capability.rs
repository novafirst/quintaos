use crate::entity::EntityId;
use crate::Mutex;
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Rights: u32 {
        const INVOKE = 0b0001;
        const READ   = 0b0010;
        const WRITE  = 0b0100;
        const DELETE = 0b1000;
        const ALL    = Self::INVOKE.bits() | Self::READ.bits() | Self::WRITE.bits() | Self::DELETE.bits();
    }
}

pub type CapabilityId = u64;

#[derive(Clone)]
pub struct Capability {
    pub id: CapabilityId,
    pub subject: EntityId,
    pub object: EntityId,
    pub rights: Rights,
    pub attenuated: bool,
}

impl Capability {
    pub fn attenuate(&self, rights: Rights) -> Option<Capability> {
        let new_rights = self.rights & rights;
        if new_rights.is_empty() {
            None
        } else {
            Some(Capability {
                id: self.id + 1,
                subject: self.subject,
                object: self.object,
                rights: new_rights,
                attenuated: true,
            })
        }
    }

    pub fn can(&self, right: Rights) -> bool {
        self.rights.contains(right)
    }
}

pub static CAPABILITY_REGISTRY: Mutex<[Option<Capability>; 256]> =
    Mutex::new([const { None }; 256]);

pub fn register_capability(cap: Capability) -> Result<(), ()> {
    let mut registry = CAPABILITY_REGISTRY.lock();
    for slot in registry.iter_mut() {
        if slot.is_none() {
            *slot = Some(cap);
            return Ok(());
        }
    }
    Err(())
}

pub fn revoke(cap_id: CapabilityId) -> bool {
    let mut registry = CAPABILITY_REGISTRY.lock();
    for slot in registry.iter_mut() {
        if let Some(cap) = slot {
            if cap.id == cap_id {
                *slot = None;
                return true;
            }
        }
    }
    false
}

