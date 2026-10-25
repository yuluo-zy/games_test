//! icc.rs — 窗口 ICC 色彩配置（O）。
//!
//! 源 `systems/backend/icc.rs`；实体 `ActiveIccProfile` / `IccProfileChanged` /
//! `IccProfilePlugin`。
//! `inner::update_window_icc_profile`（0x14093d830）：显示器 ICC 变更事件 →
//! 读文件字节 → `cmsOpenProfileFromMemTHR`（lcms2 复用）→ `generate_icc_lut`
//! 生成 LUT 上传 GPU。
//! `inner::impl$0::build`（0x140a58440）：注册系统与事件。
//! `impl$1::default`（0x940x2f80 内联于 insert_backend_resources）：默认 profile 资源。

#![allow(dead_code)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 活动 ICC profile（O：`ActiveIccProfile` 资源；文件字节 + lcms profile）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ActiveIccProfile(pub Gap);

/// ICC profile 变更事件（O：`IccProfileChanged`）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct IccProfileChanged(pub Gap);

/// ICC 插件（O：`IccProfilePlugin`；`impl$0::build` 0x140a58440）。
pub struct IccProfilePlugin;

impl IccProfilePlugin {
    pub fn build(&self, world: &mut bevy_ecs::world::World) {
        world.insert_resource(ActiveIccProfile(Gap));
    }

    /// `inner::update_window_icc_profile`（O 0x14093d830）。
    pub fn update_window_icc_profile(&self, _world: &mut bevy_ecs::world::World) {
        // U：lcms2 LUT 生成 + GPU 上传（typed gap）。
    }
}

/// ICC LUT 生成器（O：`inner::generate_icc_lut` 内联）。
pub struct GenerateIccLut(pub Gap);
