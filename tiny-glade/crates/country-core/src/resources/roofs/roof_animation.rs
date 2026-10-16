//! B 0x1408dbc40 / 0x1421a2da0: two scalar animation channels.
//! The consuming query strides by eight bytes; two zeroed XMM return registers
//! do not imply a sixteen-byte component (the historical chapter did so).
use bevy_ecs::component::Component;

#[derive(Component, Clone, Debug, Default)]
pub struct RoofAnimation(pub(crate) crate::recovered_building::roof_components::RoofAnimation);

impl RoofAnimation {
    /// 观测内层（create_roof 装配）。
    pub fn from_inner(inner: crate::recovered_building::roof_components::RoofAnimation) -> Self {
        Self(inner)
    }
    /// 观测内层引用（delete_roofs 完成判定）。
    pub fn inner(&self) -> &crate::recovered_building::roof_components::RoofAnimation {
        &self.0
    }
    /// 动画推进（system-roof `roof_animation` 消费；行为 = 内层 `advance`）。
    pub fn advance_observed(
        &mut self,
        roof: &[u8; 88],
        marked: bool,
        delta_time: f32,
        changed_tick: &mut u32,
        current_tick: u32,
    ) {
        self.0.advance(roof, marked, delta_time, changed_tick, current_tick);
    }
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
