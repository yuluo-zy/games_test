//! lib.rs — WP11 墙状态资源族证据束 crate 根（镜像 `reconstruction/country-core` 装配布局）。
//!
//! 本束只含 WP11 交付的 `resources::walls` **状态资源**类型（state types only；
//! 墙 system 属 WP12，装饰存储/归档属 WP13+）。完整可编译装配在
//! `reconstruction/country-core/src/lib.rs`（集成 owner 组装，wp1–wp10 + wp11）。
//! 骨架来源：`skeleton/B/crates/country-core/src/resources/walls/`。
//! 方法：业务逻辑（反编译 + 调用链 + PDB 骨架）→ 合同（contracts/*）→ 骨架实现；
//! 未知字段 → `typed_gap::Gap`。

extern crate alloc;

pub mod resources;

/// Typed-gap 标记（对齐装配 crate 的 `crate::typed_gap::Gap`）。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Gap;

pub mod typed_gap {
    pub use crate::Gap;
}
