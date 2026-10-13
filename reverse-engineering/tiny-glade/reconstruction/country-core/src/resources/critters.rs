//! critters.rs — 小动物资源族（WP6 恢复；只含 nani systems 消费的 `Wren`）。
//!
//! 源 SHA256（B skeleton）`resources/critters.rs`；PDB 命名实体 19 个
//! （BirdSpot/Firefly/Duck/Sheep/Wren 等）。`Wren` 是 `nani_update_wren` 读的鹪鹩状态，
//! 在 ECS 查询中作**组件**（`Query<...&Wren...>`，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 鹪鹩状态（O：ECS `&Wren`；`nani_update_wren` 读——prev/current 变换 + 参数）。
/// 布局 U（typed gap；`resources::critters::Wren` 命名实体，PDB 尺寸未知）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Wren(pub Gap);

// B 阶段 i_infra：WP18 的 `ButterflySsbo` / `FireflySsbo` / `FireflyVisibilitySsbo` `(Gap)` 占位已删——PDB 里
// `read_storage_buffer<…>` / `new_storage_buffer<…>` 的类型路径是 `systems::render_loop::resources::critters::*`，
// 真实元素类型与库侧 `SsboType` 在 `systems/render_loop/resources/critters.rs`（全 crate 已无对旧占位的引用）。
