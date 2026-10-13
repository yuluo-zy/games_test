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

// ---------------------------------------------------------------------------
// WP18: insect/animal GPU SSBO carriers (instanced_critters draw reads these
// through `read_storage_buffer<T>`; O names, fields U).
// ---------------------------------------------------------------------------

/// 蝴蝶实例 SSBO（O：`read_storage_buffer<ButterflySsbo>`）。
#[derive(Clone, Debug, Default)]
pub struct ButterflySsbo(pub Gap);

/// 萤火虫实例 SSBO（O：`read_storage_buffer<FireflySsbo>`）。
#[derive(Clone, Debug, Default)]
pub struct FireflySsbo(pub Gap);

/// 萤火虫可见性查询 SSBO（O：`read_storage_buffer<FireflyVisibilitySsbo>`；
/// 对应用户 `_critters_firefly_vis_query.cs.hlsl` 可见性查询 compute）。
#[derive(Clone, Debug, Default)]
pub struct FireflyVisibilitySsbo(pub Gap);
