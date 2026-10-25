//! B `resources/history/path_edit.rs` — 路径/鹅卵石光栅编辑负载（章节 O；13 函数）。
//!
//! `RasterEdit`（points/brush_size/is_additive/blend_mode；ReplayCmd 判别 0/1/2 的
//! Arc 负载）+ `RasterEditSerialized` 手写 serde（serialize 0x1404896f0/0x1416e8340、
//! deserialize 0x1416e8510、From 0x140a976b0）。raster 编辑走
//! `systems::history::replay_raster` 回放（Path/Cobblestone1-3/Garden/Water 六实例）。
//!
//! ## 重建（2026-10-17，fresh decomp 0x1404896f0/0x1416e8510/0x140a976b0）
//!
//! `serialize` 写单 map 四 entry（**字段序 points → is_additive → brush_size →
//! blend_mode**，与旧 src 平面序/类型不符）：
//! `{points: Vec<glam::Vec3>, is_additive: bool, brush_size: f32,
//! blend_mode: StrokeBlendMode}`。点为 **3D Vec3**（旧 src 为 Vec2）。
//! `From`（0x140a976b0）源结构除 Arc 切片外还读 `+0x00/+0x08/+0x30/+0x38` 四字段
//! （源 = 原生 RasterStroke 载荷，布局未对齐，登记 U）。
use glam::Vec3;
use serde::ser::SerializeMap;
use serde::{Deserializer, Serializer};
use std::sync::Arc;

use crate::resources::world_raster::StrokeBlendMode;

/// 光栅编辑（路径/鹅卵石/水/花园共用载荷；字段序按 0x1404896f0 重建）。
/// serde 由下方手写 `Serialize`/`Deserialize` 提供（原版手写 visitor 族）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RasterEdit {
    pub points: Vec<Vec3>,
    /// `is_additive` 在 brush_size 之前（serialize 第二 entry）。
    pub is_additive: bool,
    pub brush_size: f32,
    /// 混合模式（原生 `country_core::resources::world_raster::StrokeBlendMode`）。
    pub blend_mode: StrokeBlendMode,
}

/// 存档序列化载荷（手写 serde 族；`RasterEditSerialized` 为外层包装登记结构入口）。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RasterEditSerialized {
    pub edit: RasterEdit,
}

impl RasterEdit {
    /// `From<Arc<[Vec3]>>`（0x140a976b0）：复制点集；源结构其余四字段
    /// （+0x00/+0x08/+0x30/+0x38）未对齐，按默认填充登记 U。
    pub fn from_points(points: Arc<[Vec3]>) -> Self {
        Self {
            points: points.to_vec(),
            is_additive: false,
            brush_size: 0.0,
            blend_mode: StrokeBlendMode::Max,
        }
    }
}

impl From<Arc<[Vec3]>> for RasterEdit {
    /// B `path_edit::impl$1::from` 0x140a976b0（`From<Arc<[Vec3]>>`）。
    fn from(points: Arc<[Vec3]>) -> Self {
        RasterEdit::from_points(points)
    }
}

/// 手写 `Serialize`（0x1404896f0）：单 map，四 entry 序
/// `points / is_additive / brush_size / blend_mode`。
impl serde::Serialize for RasterEdit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("points", &self.points)?;
        map.serialize_entry("is_additive", &self.is_additive)?;
        map.serialize_entry("brush_size", &self.brush_size)?;
        map.serialize_entry("blend_mode", &self.blend_mode)?;
        map.end()
    }
}

/// 手写 visitor（0x1416e8510 `deserialize_struct<__Visitor>` + Arc 建载荷）。
struct RasterEditVisitor;
impl<'de> serde::de::Visitor<'de> for RasterEditVisitor {
    type Value = RasterEdit;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a raster history edit")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut points = None;
        let mut is_additive = None;
        let mut brush_size = None;
        let mut blend_mode = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "points" => points = Some(map.next_value()?),
                "is_additive" => is_additive = Some(map.next_value()?),
                "brush_size" => brush_size = Some(map.next_value()?),
                "blend_mode" => blend_mode = Some(map.next_value()?),
                _ => {
                    let _: serde_json::Value = map.next_value()?;
                }
            }
        }
        Ok(RasterEdit {
            points: points.unwrap_or_default(),
            is_additive: is_additive.unwrap_or(false),
            brush_size: brush_size.unwrap_or(0.0),
            blend_mode: blend_mode.unwrap_or(StrokeBlendMode::Max),
        })
    }
}

/// 手写 `Deserialize`（0x1416e8510）。
impl<'de> serde::Deserialize<'de> for RasterEdit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RasterEditVisitor)
    }
}
