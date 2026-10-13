//! session.rs — 会话资源的水相关切片（WP7 恢复，仅水域表面选择所需）。
//!
//! 源（B skeleton）：`crates/country-core/src/resources/session.rs`。本模块只承载
//! 水/冰表面派发消费的 `GladeType` 判别方法 + `ActiveSession` 资源外观；其余会话域
//! （存档/设置/主题）超出 WP7 水域范围，字段保持 typed gap。
//!
//! O 证据：
//! - `GladeType::is_flowery`（0x14092e6a0）/ `is_autumn`（0x14092e6c0）/
//!   `is_olden`（0x140a9fa70）/ `is_winter`（0x140ae8320）为 7B 常量函数
//!   （public-symbols.json；与 `is_corrupted` 等共享同址 = identical code folding）。
//! - 会话资源**字节 @+0x58 == 4 选冬季**（O：`startup_water` 0x1408f1a20、
//!   `water_planes_draw` 0x1409d9bc0 `cVar1 == 4` 选雪面）。
//! - `render_world` W45 以 `GladeType::is_winter`（0x140ae8320）派发
//!   `water_surface_pass` ↔ `ice_surface_pass`（O，frame-wiring §3）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 当前会话资源（O：`Res<ActiveSession>`；`water_spawn_vegetation` 0x140a13970 经
/// type-id `0x74430dfcc6d8719b` 断言存在并读 `is_autumn`，I：该 type-id 归属本资源）。
/// 布局 U；水面变体字节 @+0x58（O）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ActiveSession(pub Gap);

/// 湖面/草地主题判别（O：四个 `is_*` 方法；判别值域仅 `4 = Winter` 被证实）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GladeType {
    /// 原始判别值载体（U：其余主题值域）。已证实：`Raw(4)` = Winter（O）。
    Raw(u8),
}

impl GladeType {
    /// 冬季判定（O：会话字节@+0x58 == 4 选冬季水面/雪面）。
    pub fn is_winter(self) -> bool {
        matches!(self, GladeType::Raw(4))
    }
    /// 秋季判定（O：方法存在；判别值 U，默认仅 Winter 有证据）。
    pub fn is_autumn(self) -> bool {
        // 判别值 U：按 PDB 方法存在性实现，仅承载域内选择；不臆造值。
        false
    }
    /// 老式主题判定（O：方法存在；判别值 U）。
    pub fn is_olden(self) -> bool {
        false
    }
    /// 繁花主题判定（O：方法存在；判别值 U）。
    pub fn is_flowery(self) -> bool {
        false
    }
}
