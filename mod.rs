use std::fmt;
use crate::devices::{Device, DeviceType};


pub struct Light {
    id: u32,
    name: String,
    is_on: bool,
    brightness: u8,
}

impl Light {
pub fn new(id: u32, name: &str, brightness: u8) -> Self {
    Self {
        id,
        name: name.to_string(),
        is_on: false,
        brightness,
    }
}
}

impl Device for Light {
    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Light
    }

    fn is_on(&self) -> bool {
        self.is_on
    }

    fn turn_on(&mut self) {
        self.is_on = true;
    }

    fn turn_off(&mut self) {
        self.is_on = false;
    }


}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = if self.is_on { "ON" } else { "OFF" };
        write!(
            f,
            "[Light] #{id} {name} - {state}, brightness: {b}%",
            id = self.id,
            name = self.name,
            state = state,
            b = self.brightness
        ) 
    }
}