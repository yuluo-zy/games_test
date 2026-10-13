//! grow_grass.rs — 草地生长"仪式"系统（O 0x14172fe80）。
//!
//! 源 `systems/backend/grow_grass.rs`；实体 `GrowGrass` / `platform::Timer`。
//! `platform::grow_grass`（0x14172fe80）+ `platform::is_grown`（内联）：
//! 启动后按 Timer 延迟逐步显示草地（视觉装饰，非植被模拟）。

#![allow(dead_code)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 草地生长组件（O：`GrowGrass` 实体）。
#[derive(Clone, Copy, Debug, Default)]
pub struct GrowGrass(pub Gap);

/// 生长计时器（O：`platform::Timer`；`is_grown` 判定）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct GrassGrowthTimer(pub f64);

impl GrassGrowthTimer {
    pub fn new(_delay: std::time::Duration) -> Self {
        GrassGrowthTimer(0.0)
    }
    /// `is_grown`（O：`platform::is_grown` 内联）。
    pub fn is_grown(&self) -> bool {
        self.0 > 0.0
    }
}

/// `grow_grass`（O 0x14172fe80）：推进生长计时并处理"已生长"过渡。
pub fn grow_grass(_world: &mut bevy_ecs::world::World) {
    // U：具体视觉过渡（typed gap）。
}
