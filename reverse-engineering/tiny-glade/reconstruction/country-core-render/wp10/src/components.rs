//! WP10 消费的游戏组件（B 骨架交叉引用）：`SittableForBirds` / `TreeAnimation` /
//! `TreeMarkedForDeletion`。字段 U（typed gap），名称 O（PDB）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 可栖坐组件（O：`system_critters::wren::update_roof_bird_spots` 查询
/// `Query<(ref SittableForBirds, ref Transform)>`，来自 `tree_storage.rs.structures` 交叉引用）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SittableForBirds(pub Gap);

/// 树动画组件（O：`despawn_trees` 读 `f32 @+4 < 0.01` 门控；其余字段 U）。
#[derive(Component, Clone, Debug)]
pub struct TreeAnimation {
    /// 动画进度/淡出值（`+4`，O；<0.01 触发销毁）。
    pub despawn_progress: f32,
    /// `+0` 相位及 `+8..` 字段 U。
    pub _gap: Gap,
}

impl Default for TreeAnimation {
    fn default() -> Self {
        TreeAnimation {
            despawn_progress: 1.0,
            _gap: Gap,
        }
    }
}

/// 标记待删除组件（O 名；`process_tree_cmds` Delete 分支经 Commands 推入，疑似零尺寸标记）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TreeMarkedForDeletion(pub Gap);
