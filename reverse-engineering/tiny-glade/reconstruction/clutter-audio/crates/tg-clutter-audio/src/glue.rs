//! 跨模块 glue 类型：bevy 系统签名需要、但原草稿未定义/未引入的部分。

use bevy_ecs::prelude::*;

/// country_core 实体变换（bevy_transform 边界；本 crate 不还原矩阵布局）。
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Transform;
