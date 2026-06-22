use std::fmt;
use crate::devices::{Device, DeviceType};

pub struct Heater {
    id: u32,
    name: String,
    is_on: bool,
    temperature: i32,
}

impl Heater {
    pub fn new(id: u32, name: &str, temperature: i32) -> Self {
        Self { 
            id,
            name: name.to_string(),
            is_on: false,
            temperature,
        }
    }
}

impl Device for Heater {

    fn id(&self) -> u32 {
        self.id
    }

    
    fn name(&self) -> &str {
        &self.name
    }

    
    fn device_type(&self) -> DeviceType {
        DeviceType::Heater
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

impl fmt::Display for Heater {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = if self.is_on {"ON"} else {"OFF"};
        write!(
            f,
            "[Heater] #{id} {name} - {state}, set to {t}°C",
            id = self.id,
            name = self.name,
            state = state,
            t = self.temperature
        )
    }
}