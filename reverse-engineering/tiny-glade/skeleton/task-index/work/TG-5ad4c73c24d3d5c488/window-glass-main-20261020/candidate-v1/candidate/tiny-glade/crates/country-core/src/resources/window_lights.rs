//! Canonical window components and the single light-state owner.
//! B native consumers use a usize key and u32 decorator command identifier.
//! Named safe values do not specify the original Rust/Bevy ABI.
use bevy_ecs::{component::Component, resource::Resource};
use std::collections::HashMap;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowLight {
    pub key: usize,
    pub decorator_id: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LightInfo {
    pub brightness: f32,
    pub timer: f32,
    pub actual_on: u8,
    pub target_on: u8,
}

impl LightInfo {
    /// Original insertion writes all four fields, including a zero timer.
    pub fn new(lights_on: bool) -> Self {
        Self { brightness: 0.0, timer: 0.0,
            actual_on: lights_on as u8, target_on: lights_on as u8 }
    }
}

#[derive(Resource, Debug, Default)]
pub struct GlobalWindowLightsControl {
    pub lights: HashMap<usize, LightInfo>,
    pub lights_on: bool,
}
