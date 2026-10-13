//! systems/roof/utils/components.rs — RoofSpire / RoofSupportBeams（WP9）。
//!
//! PDB 骨架 `systems/roof/src/utils/components.rs`。命名实体 `RoofSpire` /
//! `RoofSupportBeams`（create_roof spawn bundle 成员）。impl$0/impl$1
//! register_required_components（rva 39375344/39377120 → VA 0x14258xxx，bevy 胶水）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 屋顶尖顶（spire）实例集合组件（place_roof_spires 写，RoofSpireSsbo 生产者）。
#[derive(Component, Clone, Debug, Default)]
pub struct RoofSpire(pub Gap);

/// 屋顶支撑梁实例集合组件（place_roof_support_beams 写，RoofSupportBeamSsbo 生产者）。
#[derive(Component, Clone, Debug, Default)]
pub struct RoofSupportBeams(pub Gap);
