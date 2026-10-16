//! B `decorator_visual/stairs_railings_assemble.rs` — 楼梯扶手装配（D1）。
//!
//! 证据：fresh RE-MCP 反编译（4 业务 VA）+ 骨架 callees（O）：
//! Affine3Packed::from_cols（0x14089d500）、BrickSourceId::from_wall_id
//! （0x140858750）、swizzlers x0y、memmove 族、rapier 碰撞、blueprint 链。
//!
//! 结构恢复（O 业务链，U 主体）：assemble_railings 0x1407f98e0（扶手段装配，
//! 67 callees）、railings_tips 0x1407fcc80（12+ 参面，扶手端头，Affine3Packed
//! + BrickSourceId 消费）、construct_wooden_railing 0x1407fdf90（8 参面，木扶手
//! 几何，sqrt 弦长计算）、add_corner_stone_railing_piece 0x1407febb0（13+ 参面，
//! 角石扶手件）。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use glam::Vec3;

use crate::blueprint::DecoratorBlueprints;

/// 0x1407f98e0（O 结构）：扶手装配（逐段几何 + 蓝图）。
pub fn assemble_railings(
    _storage: Res<DecoratorStorage>,
    _blueprints: ResMut<DecoratorBlueprints>,
    _transient: ResMut<crate::decorator_visual_state::DecoratorTransientVisualState>,
) {
    // U：扶手段几何主体待取证。
}

/// 0x1407fcc80（O 结构，12+ 参面，float* 返回）：扶手端头（Affine3Packed +
/// BrickSourceId 消费）。
pub fn railings_tips(
    _points: &[Vec3],
    _brick: u32,
    _p3: u32,
    _p4: u64,
    _p5: u32,
    _p6: u32,
    _p7: u8,
    _p8: u64,
    _p9: u64,
    _p10: &[f32],
    _p11: u64,
    _p12: &[f32],
) -> Vec<f32> {
    Vec::new() // U。
}

/// 0x1407fdf90（O 结构，8 参面）：木扶手几何（sqrt 弦长 + Vec3 装配）。
pub fn construct_wooden_railing(
    _p1: &mut u64,
    _p2: u32,
    _p3: u64,
    _p4: f32,
    _p5: &[f32],
    _p6: u64,
    _p7: u64,
    _p8: u64,
) {
    // U。
}

/// 0x1407febb0（O 结构，13+ 参面）：角石扶手件。
pub fn add_corner_stone_railing_piece(
    _p1: u64,
    _p2: u32,
    _p3: u32,
    _p4: u64,
    _p5: u64,
    _p6: u64,
    _p7: u64,
    _p8: u64,
    _p9: u64,
    _p10: u64,
    _p11: u8,
    _p12: u8,
    _p13: f32,
) {
    // U。
}
