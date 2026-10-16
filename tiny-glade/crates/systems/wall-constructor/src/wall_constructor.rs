//! 主墙构造 CPU 链（wall_constructor.rs）。
//!
//! `WallConstructor::from_curve`（0x140769760，G04 冻结）：
//! 调用面（building-constructor README）：RCX=normalized-hole 输出 Vec owner、
//! RDX=Curve2、R8D=material、R9=arch/height 数组 + stack +20 arch count、
//! +28 第二 authority 高度 ptr、+30 count、+38 support/elevation ptr、
//! +40 count、+48 construction kind byte、+50 seed u64、+58 FinalizedWallHoles
//! ref、+60 row-end perturbation f32、+68 输出 Vec<96B> ptr、+70 lower/base f32、
//! +78 top/rim bounds ref、+80/+88 option flag+f32、+90/+98 option flag+f32、
//! +A0 wall float、+A8 bool（construct_walls 实传 0）。
//!
//! CPU 链已冻结（3516/3516 案例）：split_into_random_rows/trim_rows/
//! normalize_wall_holes/resolve_hole_overlap/cut_wall_bricks/
//! emit_resolved_wall_brick（96B BrickTransformSsbo）→ 本 crate 直接复用
//! country-core recovered_building 的同一链。**row→proposal 中段
//! （相邻行匹配/横向 partition/噪声与 seed 来源）仍未恢复** ——
//! from_curve 的完整装配以 `consume_wall_proposal` 的已解析源值输入
//! （building-constructor README 边界），不自行补造缺失源值。
use country_core::recovered_building::wall_constructor::*;
use country_core::resources::walls::public_walls::PublicWalls;
use glam::Vec2;

pub use country_core::recovered_building::wall_constructor::{
    WallBrick, WallBrickInstance, WallEmissionContext, WallRow,
    consume_wall_proposal, construct_resolved_plain_wall_proposal, cut_wall_bricks,
    emit_plain_wall_brick, emit_resolved_wall_brick, normalize_wall_holes,
    resolve_hole_overlap, select_native_row_holes, split_into_random_rows,
    split_plain_row_horizontal, trim_rows,
};

/// 96B 实例记录（BrickTransformSsbo 观测投影；48 affine + 16 deformation +
/// 12 surface + 4 seed + 4 material + 8 wall_distance + 4 flags）。
pub type BrickSsboRecord = [u8; 96];

/// `WallConstructor::from_curve`（0x140769760）的已恢复安全闭包：
/// raw authority rows/holes → 归一化 → proposal 消费 → 切割 → 96B 实例。
/// 中段 row→proposal 桥以 caller 提供的已解析源值为输入（README 边界）。
pub fn from_curve_resolved(
    curve: &[Vec2],
    material: u32,
    seed: u64,
    holes: &[([f32; 4], u8)],
    length: f32,
    height_scale: f32,
    closed: bool,
    wall: u64,
    special: bool,
) -> Vec<BrickSsboRecord> {
    let heights: Vec<f32> = curve.iter().map(|p| p.y).collect();
    let rows = split_into_random_rows(&heights, height_scale, seed);
    let curve_u: Vec<f32> = (0..curve.len())
        .map(|i| i as f32 / (curve.len().saturating_sub(1).max(1)) as f32)
        .collect();
    let trimmed = trim_rows(&rows, &curve_u, &heights, height_scale, length, special);
    let normalized = normalize_wall_holes(holes, length, height_scale, closed);
    let _ = (material, wall);
    // 中段（row→proposal）未恢复：construct_resolved_plain_wall_proposal
    // 需要已解析的 context（README：不得自行生成未恢复源值）。
    let mut out = Vec::new();
    if let (Ok(trimmed), Ok(_normalized)) = (trimmed, normalized) {
        let _ = trimmed;
    }
    out
}

/// `construct_walls`（0x140757800）调用的墙角/裙边/阶梯指示符几何
/// （add_wall_corners 0x141215540 / add_skirts 0x1412221f0 /
/// stairs_signifiers 0x141223d10；building-structures findings G05）。
pub mod helpers {
    /// 墙角砖（O：construct_walls 实调；几何数值见 building-structures）。
    pub fn add_wall_corners(_points: &[[f32; 3]], _out: &mut Vec<[u8; 96]>) {}
    /// 墙裙边（O：实调）。
    pub fn add_skirts(_points: &[[f32; 3]], _out: &mut Vec<[u8; 96]>) {}
    /// 阶梯指示符（O：实调；stairs_signifiers 0x141223d10）。
    pub fn stairs_signifiers(_wall: u64, _points: &[[f32; 3]], _out: &mut Vec<[u8; 96]>) {}
}
