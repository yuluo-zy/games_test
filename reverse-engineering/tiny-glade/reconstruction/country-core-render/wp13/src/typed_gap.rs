//! 类型化缺口标记：布局/语义未从原机器码恢复的字段。绝不臆造。

/// 未恢复字段/类型的占位（零尺寸，Send+Sync，serde 直通）。
#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Gap;

/// 对齐 `country_core::typed_gap::Gap` 的别名模块。
pub mod alias {
    pub use super::Gap;
}
