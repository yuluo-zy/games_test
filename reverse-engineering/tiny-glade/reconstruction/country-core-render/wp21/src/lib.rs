//! `country_core` WP21 — 杂项资源余量（Tiny Glade 版本 B）骨架对齐 Rust。
//!
//! 覆盖 skeleton/B/crates/country-core/src/ 下：walls 余量 9、stairs 余量 7、
//! ivy/rt 余量（骨架子路径粒度）、misc resources 26、render/nani.rs、
//! systems/save/save_backup_metronome、systems/garden/{draw_garden,record_garden_history}。
//! 全部 `implemented_unverified`；合同见 `contracts/*.md`。未知字段一律
//! `crate::typed_gap::Gap`（不臆造 ABI/字段）。机器码差分仅作数值/边界裁决。

#![allow(dead_code, unused_imports)]

pub mod typed_gap;

pub mod boundary;
pub mod resources;
pub mod render;
pub mod systems;
