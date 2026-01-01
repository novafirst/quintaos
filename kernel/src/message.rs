use crate::EntityId;
use crate::Mutex;

pub const MAX_MESSAGES: usize = 128;

pub type MessageId = u64;

#[derive(Clone)]
pub enum MessageType {
    Request,
    Response,
    Event,
}

#[derive(Clone)]
pub struct Message {
    pub id: MessageId,
    pub from: EntityId,
    pub to: EntityId,
    pub msg_type: MessageType,
    pub payload: u64,
}

pub struct MessageQueue {
    pub messages: Mutex<[Option<Message>; MAX_MESSAGES]>,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            messages: Mutex::new([const { None }; MAX_MESSAGES]),
        }
    }

    pub fn push(&self, msg: Message) -> Result<(), ()> {
        let mut queue = self.messages.lock();
        for slot in queue.iter_mut() {
            if slot.is_none() {
                *slot = Some(msg);
                return Ok(());
            }
        }
        Err(())
    }

    pub fn pop(&self) -> Option<Message> {
        let mut queue = self.messages.lock();
        for slot in queue.iter_mut() {
            if slot.is_some() {
                return slot.take();
            }
        }
        None
    }
}

