use std::fmt;

pub mod light;
pub mod heater;
pub mod lock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    Light,
    Heater,
    Lock,
}

pub trait Device: fmt::Display{
    fn id(&self) -> u32;
    fn name(&self) -> &str;
    fn device_type(&self) -> DeviceType;

    fn is_on(&self) -> bool;

    fn turn_on(&mut self);
    fn turn_off(&mut self);
}