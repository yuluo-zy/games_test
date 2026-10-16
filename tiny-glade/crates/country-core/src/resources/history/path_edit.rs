//! B `resources/history/path_edit.rs` — 路径/鹅卵石光栅编辑负载（章节 O；13 函数）。
//!
//! `RasterEdit`（points/brush_size/is_additive/blend_mode；ReplayCmd 判别 0/1/2 的
//! Arc 负载）+ `RasterEditSerialized` 手写 serde（serialize 0x1404896f0/0x1416e8340、
//! deserialize 0x1416e8510、From 0x140a976b0）。raster 编辑走
//! `systems::history::replay_raster` 回放（Path/Cobblestone1-3/Garden/Water 六实例）。
use glam::Vec2;

/// 光栅编辑（路径/鹅卵石/水/花园共用载荷；字段序按章节）。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RasterEdit {
    pub points: Vec<Vec2>,
    pub brush_size: f32,
    pub is_additive: bool,
    /// 混合模式（数值/变体待取证）。
    pub blend_mode: u8,
}

/// 存档序列化载荷（手写 serde 族；字段 schema 待取证，登记结构入口）。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RasterEditSerialized {
    pub edit: RasterEdit,
}
