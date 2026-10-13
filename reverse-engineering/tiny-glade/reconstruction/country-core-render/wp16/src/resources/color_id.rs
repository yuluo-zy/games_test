//! color_id.rs — 颜色索引资源族（WP16 恢复；扩展 wp6 占位）。
//!
//! 源 SHA256（B skeleton）`resources/color_id.rs`；PDB 命名实体 10 个：
//! `ColorId` / `WallColorIds` / `GateColorIds` / `HalftimberColorIds` /
//! `{Wall,Gate,Halftimber}ColorIdSsbo` / `Set{Gate,Wall,Halftimber}ColorIdCmd`。
//!
//! O：`ColorId` 为 u32 newtype（serde newtype 证据）。三个 `*ColorIds` 资源
//! 各为 `HashMap<墙/门/半木 id, ColorId>` 集合（`new` 0x140b19890/0x140b19900/
//! 0x140b19970 建空表；`iter_nondefault` 于 save/snapshot 内联，滤默认色）。
//! `default_value`（0x14099ca50）给出默认 `ColorId`（0）。
//! SSBO 载体：`WallColorIdSsbo` / `GateColorIdSsbo` / `HalftimberColorIdSsbo`
//! （O：AssetSsboLibrary 注册 + draw_builder `read_storage_buffer`）。

#![allow(dead_code)]

use std::collections::HashMap;

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 颜色索引（O：ECS `&ColorId`；u32 newtype，serde 证据）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct ColorId(pub u32);

impl ColorId {
    /// `default_value`（O 0x14099ca50）—— 默认色索引。
    pub fn default_value() -> Self {
        ColorId(0)
    }
    pub fn is_default(&self) -> bool {
        self.0 == 0
    }
}

/// 墙色索引集合（O：`Res<WallColorIds>`；`new`/`iter_nondefault`）。
/// 键为墙 id（I：u64；墙 id 类型墙域 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallColorIds(pub HashMap<u64, ColorId>);

impl WallColorIds {
    /// `new`（O 0x140b19890；`clear_canvas` 内联）。
    pub fn new() -> Self {
        WallColorIds(HashMap::new())
    }
    /// `iter_nondefault`（O：save/snapshot 内联）—— 滤默认色的全部 (id,color)。
    pub fn iter_nondefault(&self) -> impl Iterator<Item = (u64, ColorId)> + '_ {
        self.0.iter().filter(|(_, c)| !c.is_default()).map(|(k, c)| (*k, *c))
    }
}

/// 门色索引集合（O：`Res<GateColorIds>`；同族）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GateColorIds(pub HashMap<u64, ColorId>);

impl GateColorIds {
    /// `new`（O 0x140b19900）。
    pub fn new() -> Self {
        GateColorIds(HashMap::new())
    }
    pub fn iter_nondefault(&self) -> impl Iterator<Item = (u64, ColorId)> + '_ {
        self.0.iter().filter(|(_, c)| !c.is_default()).map(|(k, c)| (*k, *c))
    }
}

/// 半木色索引集合（O：`Res<HalftimberColorIds>`；`nani_update_curved_halftimber`
/// 读端帽颜色 —— wp6 已接线）。
#[derive(Resource, Clone, Debug, Default)]
pub struct HalftimberColorIds(pub HashMap<u64, ColorId>);

impl HalftimberColorIds {
    /// `new`（O 0x140b19970）。
    pub fn new() -> Self {
        HalftimberColorIds(HashMap::new())
    }
    pub fn iter_nondefault(&self) -> impl Iterator<Item = (u64, ColorId)> + '_ {
        self.0.iter().filter(|(_, c)| !c.is_default()).map(|(k, c)| (*k, *c))
    }
}

/// 墙色索引 SSBO 载体（O：AssetSsboLibrary 注册 / read_storage_buffer）。
pub struct WallColorIdSsbo(pub Vec<u32>);
/// 门色索引 SSBO 载体（O）。
pub struct GateColorIdSsbo(pub Vec<u32>);
/// 半木色索引 SSBO 载体（O）。
pub struct HalftimberColorIdSsbo(pub Vec<u32>);

/// 命令：设置门颜色（O：PDB 实体；apply 语义 U）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct SetGateColorIdCmd(pub u64, pub ColorId);
/// 命令：设置半木颜色（O）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct SetHalftimberColorIdCmd(pub u64, pub ColorId);
/// 命令：设置墙颜色（O）。
#[derive(bevy_ecs::event::Event, Clone, Debug)]
pub struct SetWallColorIdCmd(pub u64, pub ColorId);

/// 颜色组件注册标记（O：`impl$12::register_required_components` 0x140b1cf80；
/// 伴随组件清单 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorIdMarker(pub Gap);
