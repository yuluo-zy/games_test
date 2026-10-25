//! window_lights.rs — 窗光/灯资源族（WP16 恢复；扩展 wp6 占位）。
//!
//! 源 SHA256（B skeleton）`resources/window_lights.rs`；PDB 命名实体 4 个：
//! `WindowLight` / `GlobalWindowLightsControl` / `LightInfo` / `GlobalSmokeControl`。
//!
//! 消费链（O）：decorator 系统给窗实体挂 `WindowLight`；glass 绘制
//! （`nani_update_solid_vertex_color_window_glass`）读 `&WindowLight`；
//! 延迟光照分箱 `bin_lights` 以窗灯为光源（deferred light_binning lantern
//! 源）。`register_required_components`（0x14086cac0）注册所需组件。

#![allow(dead_code)]

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 窗内灯光组件（O：ECS `&WindowLight`；glass 系统读、bin_lights 光源）。
/// 布局 U（typed gap；灯颜色/功率/范围参数未从机器码恢复）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WindowLight(pub Gap);

impl WindowLight {
    /// `register_required_components`（O 0x14086cac0）—— 注册伴随组件集合；
    /// 伴随组件清单 U。
    pub fn register_required_components(_world: &mut bevy_ecs::world::World) {}
}

/// 全局窗光开关资源（O：`Res<GlobalWindowLightsControl>`；glass 系统读）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GlobalWindowLightsControl(pub Gap);

/// 灯信息（O：PDB 命名实体；bin_lights 光源描述，字段 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct LightInfo(pub Gap);

/// 全局烟开关（O：PDB 命名实体；烟系统读）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GlobalSmokeControl(pub Gap);
