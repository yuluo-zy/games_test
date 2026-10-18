//! mei 自研 UI 框架（B 版恢复，restorer-interaction-toolbar）。
//!
//! 已恢复：core（节点引擎叶子）、layout（盒模型 LayoutStyle）、config（RON 字体表）。
//! systems（依赖线解锁 2026-10-17）：`mei_pre_update`/`mei_post_update` 的 **9 参
//! SystemParam 精确签名**（signature 判等；主体未实现，分歧见 `systems.rs`）。
//! 未实现（分歧见 RESTORE.md）：widgets、widgets/tweaker、widgets/decorate、audio、
//! mei_post_update 内部 update_nav/perform_layout/z-order、TemplateView 单态族、
//! place_childen_impl 尾区。

pub mod config;
pub mod core;
pub mod widgets;
pub mod layout;
pub mod systems;
