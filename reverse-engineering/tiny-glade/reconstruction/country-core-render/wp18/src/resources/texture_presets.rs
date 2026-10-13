//! `resources::texture_presets` — 纹理预设绑定类型（O 名，字段 typed gap）。
//!
//! 原符号：`country_core::resources::texture_presets::{TexturePreset,
//! TextureFromDisk}`，用于 `bind_texture_to_draw_builder<T>` /
//! `bind_texture_to_compute_pass<T>` 泛型实现（wp1 draw_builder::impl$3 /
//! compute_pass_builder::impl$4，反编译 O）。
//!
//! - `TextureFromDisk(name_hash)`：按 64 位名字哈希取磁盘纹理资源。
//!   已知 O 值（见各模块 contract）：`0x579f39ab91a01865` =
//!   water_noise_texture；其余（clearing/floor/rocky/pebble/brick/leaf/
//!   critter/ivy 族）名字待资源库恢复（U）。
//! - `TexturePreset(preset_byte)`：数值预设槽。观测值 8/9/10/0xc/0xe/
//!   0x13/0x14/0x15/0x18/5/6/7/0xf 等（语义 U；water_planes_draw 记录
//!   preset 9/10/0x13/0x18/8 为水面纹理槽）。

/// 磁盘纹理绑定（64 位名字哈希，O：`TextureFromDisk`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureFromDisk(pub u64);

/// 数值纹理预设槽（O：`TexturePreset`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TexturePreset(pub u8);
