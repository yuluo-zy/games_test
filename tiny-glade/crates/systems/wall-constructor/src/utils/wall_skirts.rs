//! 墙裙边（utils/wall_skirts.rs）。
//!
//! `add_skirts`（0x1412221f0，construct_walls 实调；G05 caller 链）。
use glam::Vec3;

/// B `add_skirts`（0x1412221f0）：墙裙边砖（96B 记录）。
/// **未实现**：旧近似体（沿曲线均分插值）不是原二进制真实逻辑。
/// 真实逻辑（fresh decomp /tmp/wc-dec/0x1412221f0.c）含 max_y/is_rectangle
/// 分支（矩形 get_point_u×8，非矩形 [0,1]）、split_height_into_skirts、
/// ceilf((u1−u0)*h/0.69)+random_splits_into、abs 累计距离 inverse_lerp、
/// Aabb2::from_center_dims+holes_no_duplicates→resolve_hole_overlap、
/// WallSpaceCoord::to_world_space→96B（@0x54/58=(中点±半宽)*h）、
/// 裙阈值 wall_y+3.5。
pub fn add_skirts_new(points: &[Vec3], _out: &mut Vec<[u8; 96]>) {
    let _ = points;
    todo!("未实现: add_skirts 0x1412221f0")
}
