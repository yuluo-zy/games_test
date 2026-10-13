//! components/mod.rs — WP6 恢复的游戏组件集（systems.rs 消费的集合）。
//!
//! 源 SHA256（B skeleton）`components/mod.rs`；PDB 命名实体 45 个。
//! 本模块只恢复 nani systems 消费的组件；其余为 typed gap（绝不臆造字段）。

#![allow(dead_code, unused_imports)]

pub mod drawable;

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 半透明标记组件（O：ECS `Option<&SemiTransparent>`；零尺寸 marker，I）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SemiTransparent;

/// 平直光照标记组件（O：ECS `Option<&FlatLighting>`；零尺寸 marker，I）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FlatLighting;

/// 砖色组件（O：ECS `&BrickColor`；nani_update_solid_vertex_color_bricks / gothic 读）。
/// 布局 U（typed gap；色索引/SSBO 句柄）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct BrickColor(pub Gap);

/// 哥特窗砖变形（O：ECS `&GothicWindowBrickDeformation`；nani_update_gothic_window_bricks 读）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct GothicWindowBrickDeformation(pub Gap);

/// 晾衣绳布料（O：ECS `&ClothlineCloth`；nani_update_cloth 读；布局 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ClothlineCloth(pub Gap);

/// 法线贴图（O：ECS `&NormalMap`；nani_update_cloth 读）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NormalMap(pub Gap);

/// 灯笼光照参数（O：ECS `&LanternLightParams`；nani_update_lantern_emissive 读）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LanternLightParams(pub Gap);

/// 烟（O：ECS `&Smoke`；nani_update_smoke 读；布局 U）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Smoke(pub Gap);
