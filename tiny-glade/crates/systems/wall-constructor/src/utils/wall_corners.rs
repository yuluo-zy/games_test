//! 墙角砖（utils/wall_corners.rs）。
//!
//! `add_wall_corners`（0x141215540，construct_walls 实调；G05 caller 链）。
use glam::{Vec2, Vec3};

/// 原 0x141215540 的夹角窗口（rine-read .rdata）：
/// 0x142c1c4e0 = −0.1736482、0x142c1c4e4 = 0.17364822（±10° 的余弦窗口）。
pub const CORNER_ANGLE_MIN: f32 = f32::from_bits(0xbe31d0d5);
pub const CORNER_ANGLE_MAX: f32 = f32::from_bits(0x3e31d0d7);

/// 原 inlinee `check_angle`（add_wall_corners 内联体，0x141215540 前段）：
/// 计算 `normalize(prev−cur)` 与 `normalize(next−cur)` 的夹角余弦，返回是否
/// 落在 `[−0.1736482, 0.17364822)` 且非 NaN。原件先以
/// `|dot| > f32::MAX 位型` 过滤非有限值。
pub fn check_angle(previous: Vec2, current: Vec2, next: Vec2) -> bool {
    let a = previous - current;
    let b = next - current;
    let dot = a.dot(b) / (a.length() * b.length());
    if !dot.is_finite() {
        return false;
    }
    CORNER_ANGLE_MIN <= dot && dot < CORNER_ANGLE_MAX
}

/// 原 inlinee `find_corners`（add_wall_corners 内联体）：以三相邻点窗口贪心
/// 扫描角点。原件以索引步进（命中时前进 1、未命中前进 2）收集角点索引。
pub fn find_corners(points: &[Vec2]) -> Vec<usize> {
    let mut out = Vec::new();
    if points.len() < 3 {
        return out;
    }
    let mut index = 1usize;
    while index + 1 < points.len() {
        if check_angle(points[index - 1], points[index], points[index + 1]) {
            out.push(index);
            index += 1;
        } else {
            index += 2;
        }
    }
    out
}

/// B `add_wall_corners`（0x141215540）：墙角砖（96B 记录）。
/// **未实现（U）**：角点检测阈值/步进已按机器码恢复（见 [`check_angle`] /
/// [`find_corners`]）；但转角处的 Triangle 链、随机砖宽 0.92±0.16 / 末砖
/// 0.76 的逐砖坐标、aabb 合并与 `@0x5c` 标志枚举未逐语句闭合，不得以
/// 近似体冒充。
pub fn add_wall_corners_new(points: &[Vec3], _out: &mut Vec<[u8; 96]>) {
    let _ = points;
    todo!("未实现: add_wall_corners 0x141215540（角点检测已恢复，转角砖几何/发射未闭合）")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_angle_detects_right_angle_window() {
        assert!(check_angle(
            Vec2::new(-1.0, 0.0),
            Vec2::ZERO,
            Vec2::new(0.0, 1.0)
        ));
        // 45° 转角不在 ±10° 窗口内。
        assert!(!check_angle(
            Vec2::new(-1.0, 0.0),
            Vec2::ZERO,
            Vec2::new(1.0, 1.0)
        ));
    }

    #[test]
    fn find_corners_greedy_scan() {
        // 直角折线：角点位于索引 1。
        let points = [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
        ];
        assert_eq!(find_corners(&points), vec![1]);
        // 直线无角点。
        let line = [Vec2::ZERO, Vec2::new(1.0, 0.0), Vec2::new(2.0, 0.0)];
        assert!(find_corners(&line).is_empty());
    }
}
