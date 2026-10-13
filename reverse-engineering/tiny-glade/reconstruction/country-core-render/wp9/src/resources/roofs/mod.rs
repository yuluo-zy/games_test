//! roofs/mod.rs — 屋顶资源族（WP9 屋顶内容域：roof_shape / roof_tile_sim / roof_tiles /
//! roof_animation / roof_ridge / roof_visual_shape / roof_trigger_volume / roof_noise /
//! roof_buffers / dormer_tiles / commands）。
//!
//! 来源：PDB 骨架 `crates/country-core/src/resources/roofs/*.rs`（B）。本模块镜像
//! `reconstruction/country-core` 的装配布局；WP6 已恢复 `roof_animation` / `roof_shape`
//! 的 nani 消费子集，此处按屋顶内容域扩展。
//!
//! 方法：业务逻辑（Ghidra 反编译 + 调用链 + PDB 骨架证据）→ 合同（contracts/*）→
//! 骨架实现；未知字段一律 typed gap（`crate::typed_gap::Gap`），不臆造。
//! 已机器码验证的数值核心复用 `reconstruction/roof/{surface,tiles,parameters,
//! roof_state,edit_rules}.rs`（本树内联等价实现并保留证据引用）。

#![allow(dead_code, unused_imports)]

pub mod commands;
pub mod dormer_tiles;
pub mod roof_animation;
pub mod roof_buffers;
pub mod roof_noise;
pub mod roof_ridge;
pub mod roof_shape;
pub mod roof_tile_sim;
pub mod roof_tiles;
pub mod roof_trigger_volume;
pub mod roof_visual_shape;
