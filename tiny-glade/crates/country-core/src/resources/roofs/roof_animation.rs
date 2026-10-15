//! B 0x1408dbc40 / 0x1421a2da0: two scalar animation channels.
//! The consuming query strides by eight bytes; two zeroed XMM return registers
//! do not imply a sixteen-byte component (the historical chapter did so).
use bevy_ecs::component::Component;

#[derive(Component, Clone, Debug, Default)]
pub struct RoofAnimation(pub(crate) crate::recovered_building::roof_components::RoofAnimation);

impl RoofAnimation {
    pub fn is_being_animated(&self) -> bool {
        self.0.is_being_animated()
    }
    pub fn values(&self) -> [f32; 2] {
        let bytes = self.0.bytes();
        [
            f32::from_le_bytes(bytes[..4].try_into().unwrap()),
            f32::from_le_bytes(bytes[4..].try_into().unwrap()),
        ]
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct RoofMarkedForDeletion;
