//! Owns the B tile simulation state initialized by create_roof.
use bevy_ecs::component::Component;

#[derive(Component, Clone, Debug)]
pub struct RoofTileSimulation(
    pub(crate) crate::recovered_building::roof_components::RoofTileSimulation,
);

impl RoofTileSimulation {
    /// 观测内层（create_roof 装配）。
    pub fn from_inner(
        inner: crate::recovered_building::roof_components::RoofTileSimulation,
    ) -> Self {
        Self(inner)
    }
    pub fn new(floor_y: f32) -> Self {
        Self(
            crate::recovered_building::roof_components::RoofTileSimulation::initialize(
                [0; 24], floor_y,
            ),
        )
    }
    pub fn observed(&self) -> &[u8; 24] {
        self.0.bytes()
    }
    /// wall_height_dt records the change from the previous published wall top.
    pub fn update_wall_height(&mut self, floor_y: f32) {
        let mut bytes = *self.0.bytes();
        let previous = f32::from_le_bytes(bytes[4..8].try_into().unwrap());
        bytes[..4].copy_from_slice(&(floor_y - previous).to_le_bytes());
        bytes[4..8].copy_from_slice(&floor_y.to_le_bytes());
        self.0 =
            crate::recovered_building::roof_components::RoofTileSimulation::from_observed(bytes);
    }
    /// Observed-state write used by `simulate_roof_tile_for_audio`.
    pub fn replace_observed(&mut self, bytes: [u8; 24]) {
        self.0 = crate::recovered_building::roof_components::RoofTileSimulation::from_observed(bytes);
    }
}
