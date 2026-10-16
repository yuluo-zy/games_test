//! 玩家楼梯（construct_playermade_stairs/mod.rs）。
//!
//! `construct_playermade_stairs_arches_n_walls<true>`（0x140496800）/`<false>`
//! （0x14048ab50 = ArchWalker::walk<closure<false>> 宿主）。
//! 候选参数面（skeleton 富化，arch_walker.rs 项）：
//! `(ResMut<StairsArchSupportRequests>, Res<UiSignifierStream>,
//!  Res<TerrainHeightsData>, ResMut<StairsPillarProposals>, …)`。
//!
//! `preprocess_curve`（0x140780e20）：曲线预处理（building-production）。
//! ArchWalker::new 两调用点均在此族（0x14049877e/0x14049eb1c）。
pub mod brackets;
pub mod magic_support;
pub mod pillars_process_proposals;
pub mod pillars_visual;

use country_core::resources::{
    stairs_walls::{StairsArchSupportRequests, StairsPillarProposals},
    terrain_height::TerrainHeightsData,
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use bevy_ecs::prelude::*;
use glam::Vec3;

/// B `preprocess_curve`（0x140780e20）：曲线预处理（重采样/方向归一）。
pub fn preprocess_curve(points: &[Vec3]) -> Vec<Vec3> {
    if points.len() < 2 {
        return points.to_vec();
    }
    // 观测：沿曲线按弦长重采样（间距 = 平均弦长；building-production）。
    let total: f32 = points.windows(2).map(|w| (w[1] - w[0]).length()).sum();
    let n = ((total / 0.5).ceil() as usize).clamp(2, 256);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let span = (points.len() - 1) as f32;
        let u = t * span;
        let idx = (u as usize).min(points.len() - 2);
        let frac = u - idx as f32;
        out.push(points[idx] + (points[idx + 1] - points[idx]) * frac);
    }
    out
}

/// B 0x140496800 / 0x14048ab50：玩家楼梯拱与墙（双模板）。
pub fn construct_playermade_stairs_arches_n_walls<const WALLS: bool>(
    mut support_requests: ResMut<StairsArchSupportRequests>,
    _signifiers: Res<UiSignifierStream>,
    _terrain: Res<TerrainHeightsData>,
    mut proposals: ResMut<StairsPillarProposals>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    // 冻结链（building-production B-variant/B-other-kind）：沿曲线 walk
    // （brick_len = total/整数砖数）→ 拱砖/墙砖 96B + 支柱提议。
    for (&wall, record) in support_requests.requests.iter() {
        let start = glam::Vec3::new(
            f32::from_le_bytes(record[0..4].try_into().unwrap_or([0; 4])),
            0.0,
            f32::from_le_bytes(record[4..8].try_into().unwrap_or([0; 4])),
        );
        let end = glam::Vec3::new(
            f32::from_le_bytes(record[8..12].try_into().unwrap_or([0; 4])),
            0.0,
            f32::from_le_bytes(record[12..16].try_into().unwrap_or([0; 4])),
        );
        // ArchWalker::walk（0x14048ab50 closure<false>）沿支撑请求段发砖。
        let walker = crate::arch_walker::ArchWalker::new(0, 0.0, 1.2);
        let points = [start, end];
        walker.walk(&points, &points, |at, _b, _t| {
            let mut brick = [0u8; 96];
            brick[..12].copy_from_slice(&at.to_array().map(f32::to_le_bytes).concat());
            output.brick_instances.push(brick);
        });
        // 支柱提议（44B）落盘。
        let mut proposal = [0u8; 44];
        proposal[..4].copy_from_slice(&start.x.to_le_bytes());
        proposal[4..8].copy_from_slice(&start.z.to_le_bytes());
        proposals.add(wall, proposal);
    }
}
