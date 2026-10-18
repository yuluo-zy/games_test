//! 墙角砖（utils/wall_corners.rs）。
//!
//! `add_wall_corners`（0x141215540，construct_walls 实调；G05 caller 链）。
use glam::Vec3;

/// B `add_wall_corners`（0x141215540）：墙角砖（96B 记录）。
/// **未实现**：旧近似体（首尾端点各一块转角砖）不是原二进制真实逻辑。
/// 真实逻辑（fresh decomp /tmp/wc-dec/0x141215540.c，976 行）含
/// 夹角检测（−0.1736482/0.17364822 阈值，±10°）、Triangle 链、
/// 随机砖宽 0.92±0.16 / 末砖 0.76、aabb 合并、resolve_hole_overlap、
/// 96B BrickTransformSsbo 发射（seed/索引/@0x5c 标志）。
pub fn add_wall_corners_new(points: &[Vec3], _out: &mut Vec<[u8; 96]>) {
    let _ = points;
    todo!("未实现: add_wall_corners 0x141215540")
}
