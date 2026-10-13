//! window_lights.rs — 窗光资源族（WP6 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/window_lights.rs`；PDB 命名实体 4 个：
//! `WindowLight` / `GlobalWindowLightsControl` / `LightInfo` / `GlobalSmokeControl`。
//! 本模块恢复 nani systems 消费的 `WindowLight` 与 `GlobalWindowLightsControl`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 窗内灯光（O：ECS `&WindowLight`；`nani_update_solid_vertex_color_window_glass` 读）。
/// 布局 U（typed gap；灯功率/色调参数）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WindowLight(pub Gap);

/// 全局窗光开关资源（O：`Res<GlobalWindowLightsControl>`；glass 系统读）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GlobalWindowLightsControl(pub Gap);

/// 灯信息（O：命名实体；本组系统未直接消费）。
#[derive(Clone, Copy, Debug, Default)]
pub struct LightInfo(pub Gap);

/// 全局烟开关（O：命名实体；本组系统未直接消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GlobalSmokeControl(pub Gap);
