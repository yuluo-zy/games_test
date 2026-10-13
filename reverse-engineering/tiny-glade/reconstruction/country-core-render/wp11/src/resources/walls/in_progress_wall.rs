//! in_progress_wall.rs — 编辑中墙（WP11 形状/轮廓几何族）。
//!
//! 源骨架 `resources/walls/in_progress_wall.rs`（B）；PDB 命名实体：
//! `InProgressWall` / `InProgressWallResource`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 编辑中墙（组件/瞬态；布局 U）。
///
/// O：`InProgressWall::new`（行~30）；`impl$3::fmt` RVA 0x140abceb0。
/// 语义 I：徒手墙绘制输入→预备轮廓→提交创建前置状态
/// （先验 `reconstruction/building/wall_freeform_cache.rs`）。
#[derive(Component, Clone, Debug, Default)]
pub struct InProgressWall {
    /// 编辑几何（typed gap）。
    pub gap: Gap,
}

impl InProgressWall {
    /// O：`new`。构造参数 U。
    pub fn new() -> Self {
        Self::default()
    }
}

/// 编辑中墙资源（O：命名实体；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct InProgressWallResource(pub Gap);
