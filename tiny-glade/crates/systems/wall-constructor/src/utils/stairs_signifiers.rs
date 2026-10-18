//! 阶梯指示符（utils/stairs_signifiers.rs）。
//!
//! `stairs_signifiers`（0x141223d10，construct_walls 实调；G05 caller 链）。
use glam::Vec3;

/// B `stairs_signifiers_new`（0x141223d10，construct_walls 实调）：阶梯指示符几何。
/// **未实现**：旧近似体（只写首点）不是原二进制真实逻辑。
/// 真实逻辑（fresh decomp /tmp/wc-dec/0x141223d10.c，704 行）含非矩形
/// 墙占位 [0,end−1]、矩形墙 get_point×2+get_approx_coord_from_pos、tuple_windows(u0,u1)、
/// TerrainHeightsData::sample_at_world_xz（y−terrain<0.5 且 max_y−y≥3.0 判楼梯段）、
/// SubtractiveBoolean1D::new_range/subtract 减 hole（HoleType 0/1/3/4 过滤）、
/// 剩余范围 ceil(段长*h/0.69)+random_splits_into 踏板、每阶 Curve::get_coord_at_u
/// 双端+xz_rotation_bad→96B（@0x54 两行 u、@0x5c=4）、2 阶循环、常量 0.69、
/// max_y−0.79−0.1−0.3 顶高。
pub fn stairs_signifiers_new(wall: u64, points: &[Vec3], _out: &mut Vec<[u8; 96]>) {
    let _ = (wall, points);
    todo!("未实现: stairs_signifiers_new 0x141223d10")
}
