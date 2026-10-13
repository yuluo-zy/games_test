//! Typed gap：未恢复字段的统一载体（项目规则：未知字段 → typed gap，不编造）。

/// 未恢复类型的统一占位。`repr(transparent)` 保证与真实布局无关；集成 owner 用
/// 真实类型替换字段。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(transparent)]
pub struct Gap(pub [u8; 0]);
