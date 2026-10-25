//! Typed-gap 标记：未从原机器码恢复布局/语义的字段。不臆造。

use core::fmt;

/// 未恢复布局/语义的字段占位。
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Gap;

impl fmt::Debug for Gap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<gap>")
    }
}

impl Gap {
    /// 占位 f32（U 值，仅供调用点形状）。
    pub fn f32() -> f32 {
        0.0
    }
}

// serde 透传占位（仅保证含 Gap 字段的结构（如 AudioSettings）能派生/序列化）。
impl serde::Serialize for Gap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_unit_struct("Gap")
    }
}

impl<'de> serde::Deserialize<'de> for Gap {
    fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Ok(Gap)
    }
}

// bevy 边界透传：Gap 可作为 Resource / Event 占位（Res<Gap>、EventReader<Gap> 调用点形状）。
impl bevy_ecs::resource::Resource for Gap {}

/// bevy `Event` 派生（bevy 0.16 `Event` 需要 `Traversal` 关联类型，由派生宏生成）。
#[derive(bevy_ecs::event::Event)]
pub struct GapEventPlaceholder(pub Gap);

/// 便捷占位 f32（U）。
pub fn gap_f32() -> f32 {
    0.0
}
