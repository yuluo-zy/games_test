//! walls/mod.rs — 墙资源族（WP6 恢复；nani systems 消费的子集）。

#![allow(dead_code, unused_imports)]

pub mod curved_halftimber;
pub mod plaster;
pub mod public_walls;

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 楼板（O：ECS `Option<&Floor>`；`nani_update_stone_floor` 读 +8 字节 bit0）。
/// 布局 I：≥0x28 字节（stone_floor 行 stride 0x28）；+0x8 bit0 控制屋檐切线路径。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Floor(pub Gap);
