use std::fmt;
use crate::devices::{Device, DeviceType};

pub struct DoorLock {
    id: u32,
    name: String,
    is_locked: bool,
}

impl DoorLock {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            is_locked: true,
        }
    }

}

impl Device for DoorLock {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Lock
    }

    fn is_on(&self) -> bool {
        !self.is_locked
    }

    fn turn_on(&mut self) {
        self.is_locked =  false;
    }

    fn turn_off(&mut self) {
        self.is_locked = true;
    }
}

impl fmt::Display for DoorLock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = if self.is_locked {"LOCKED"} else {"UNLOCKED"};
        write!(
            f,
            "[Lock] #{id} {name} - {state}",
            id = self.id,
            name = self.name,
            state = state
        )
    }
}