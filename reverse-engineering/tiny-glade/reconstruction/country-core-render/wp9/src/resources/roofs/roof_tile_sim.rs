//! roof_tile_sim.rs — RoofTileSimulation（WP9）。
//!
//! PDB 骨架 `resources/roofs/roof_tile_sim.rs`（约 14 行）。命名实体 `RoofTileSimulation`，
//! OBS 字段 off=4 f32。`new`（0x140939390）：`out[0..4]=0`、`out[4]=入参`、`out[8..16]=0`
//! → 16B **组件**。消费：`roof_tiles_gravity_compute` push `wall_height_delta`（I）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 屋顶铺瓦模拟参数（16B）。
///
/// OBS：off4 = f32（`new` 传入）。off0 f32（初 0）、off8 u64（初 0）。
/// 语义 I：off4 疑为 gravity 统一的 `wall_height_delta`；off0 疑为初始高度/标记。U。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofTileSimulation {
    pub field_0: f32,
    pub field_4: f32,
    pub field_8: u64,
}

impl RoofTileSimulation {
    /// RoofTileSimulation::new（O：0x140939390）。
    pub fn new(value: f32) -> Self {
        Self {
            field_0: 0.0,
            field_4: value,
            field_8: 0,
        }
    }

    /// `wall_height_delta`（I：gravity shader push 同名字段）。U：字段名。
    pub fn wall_height_delta(&self) -> f32 {
        self.field_4
    }

    /// 供 typed-gap 保留的占位（防误删消费引用）。
    #[allow(dead_code)]
    pub fn _gap(&self) -> Gap {
        Gap
    }
}
