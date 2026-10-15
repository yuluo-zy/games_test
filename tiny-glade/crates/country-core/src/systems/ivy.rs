//! `ivy.rs` — 常春藤几何：`closest_segment_index`（B 0x14094c800，666 B，反编译 + 汇编逐条核对）。
//!
//! 纠错（相对此前文档，文档的过滤方向与距离度量都与机器码相反/不同）：
//! - 过滤 1：**仅当两端点高度都 `> max_height` 才跳过**（`ucomiss h0,max; jbe → 处理; ucomiss h1,max; ja → 跳过`），
//!   文档写成"任一端点 `<= max` 则跳过"；
//! - 过滤 2：插值高度 `h1*t + h0*(1-t) > max_height` 则跳过（NaN 不跳过）；
//! - `t = clamp(dot(P-P0, D) / max(|D|², 1e-20), 0, 1)`，`maxss`/`minss` 的 NaN 语义按 SSE；
//! - 距离度量是点到**直线**的垂直距离 `|(-Dy/len)*(x0-px) + (Dx/len)*(y0-py)|`，不是沿段投影；另加一项恒为 0 的
//!   `|t - t| * len * 100`（NaN 时该项为 NaN，使"处理"分支仍成立）；
//! - 初始最佳距离是 **100.0**（0x14292c76c），超出 100 的段永远选不中；
//! - 附带值是 `lerp(seg.f10, seg.f14, t)`（0x140c904d0：`a + (b - a) * t`），不是打包整数；
//! - 断言：命中时 `t` 必须在 `[0, 1]`（NaN 也 panic，串 0x142aa53ed，长 0x2a）。
//!
//! 段步长 0x38 B：`p0.xy @0x00`、`p1.xy @0x08`、`extra_a @0x10`、`extra_b @0x14`、`h0 @0x18`、`h1 @0x1c`，
//! 其后 24 B 未恢复。未找到时原函数不写有效的 index/extra，这里返回 `None`。
use glam::Vec2;

/// 初始最佳距离（O：0x14292c76c = 100.0）。
pub const INITIAL_BEST_DISTANCE: f32 = 100.0;
/// `|D|²` 的下限（O：0x142a7baa0 ≈ 1e-20）。
const MIN_LENGTH_SQ: f32 = f32::from_bits(0x1e3c_e508);

/// 墙段（只含 `closest_segment_index` 读取的字段）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallSegment {
    pub p0: Vec2,
    pub p1: Vec2,
    pub extra_a: f32,
    pub extra_b: f32,
    pub h0: f32,
    pub h1: f32,
}

/// 命中结果。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClosestSegment {
    pub index: usize,
    pub distance: f32,
    /// `lerp(extra_a, extra_b, t)`。
    pub interpolated: f32,
}

/// 命中后 `t` 越界（原版 panic）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterOutOfRange {
    pub t: f32,
}

impl core::fmt::Display for ParameterOutOfRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "closest_segment_index: t = {} is outside [0, 1]", self.t)
    }
}

impl std::error::Error for ParameterOutOfRange {}

/// SSE `maxss dst, src`。
fn sse_max(dst: f32, src: f32) -> f32 {
    if dst > src { dst } else { src }
}

/// SSE `minss dst, src`。
fn sse_min(dst: f32, src: f32) -> f32 {
    if dst < src { dst } else { src }
}

/// `closest_segment_index`（0x14094c800）。
pub fn closest_segment_index(
    point: Vec2,
    segments: &[WallSegment],
    max_height: f32,
) -> Result<Option<ClosestSegment>, ParameterOutOfRange> {
    let mut best = INITIAL_BEST_DISTANCE;
    let mut found = None;
    for (index, seg) in segments.iter().enumerate() {
        if seg.h0 > max_height && seg.h1 > max_height {
            continue;
        }
        let to_point = point - seg.p0;
        let dir = seg.p1 - seg.p0;
        let dot = to_point.y * dir.y + to_point.x * dir.x;
        let len_sq = dir.y * dir.y + dir.x * dir.x;
        let t = sse_min(1.0, sse_max(0.0, dot / sse_max(len_sq, MIN_LENGTH_SQ)));
        let height = seg.h1 * t + seg.h0 * (1.0 - t);
        if height > max_height {
            continue;
        }
        let len = len_sq.sqrt();
        // 原版算 `|t - t|`：有限 t 得 0，±inf/NaN 得 NaN。
        let t_minus_t = if t.is_finite() { 0.0 } else { f32::NAN };
        let along = t_minus_t * len * 100.0;
        if !along.is_nan() && along >= best {
            continue;
        }
        let normal = Vec2::new(-dir.y, dir.x) / len;
        let to_origin = seg.p0 - point;
        let perpendicular = (normal.y * to_origin.y + normal.x * to_origin.x).abs() + along;
        if perpendicular.is_nan() || perpendicular >= best {
            continue;
        }
        if t.is_nan() || !(0.0..=1.0).contains(&t) {
            return Err(ParameterOutOfRange { t });
        }
        found = Some(ClosestSegment {
            index,
            distance: perpendicular,
            interpolated: seg.extra_a + (seg.extra_b - seg.extra_a) * t,
        });
        best = perpendicular;
    }
    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(p0: (f32, f32), p1: (f32, f32), extra: (f32, f32), h: (f32, f32)) -> WallSegment {
        WallSegment {
            p0: Vec2::new(p0.0, p0.1),
            p1: Vec2::new(p1.0, p1.1),
            extra_a: extra.0,
            extra_b: extra.1,
            h0: h.0,
            h1: h.1,
        }
    }

    #[test]
    fn segments_with_both_ends_above_max_are_skipped() {
        let tall = seg((0.0, 0.0), (10.0, 0.0), (0.0, 1.0), (5.0, 5.0));
        assert_eq!(
            closest_segment_index(Vec2::new(5.0, 1.0), &[tall], 3.0),
            Ok(None)
        );
    }

    #[test]
    fn one_low_end_keeps_the_segment_but_the_interpolated_height_must_not_exceed_max() {
        let s = seg((0.0, 0.0), (10.0, 0.0), (0.0, 1.0), (0.0, 10.0));
        // t = 0.5 -> height 5 > 4: skipped. t = 0.5 with max 6: kept.
        assert_eq!(
            closest_segment_index(Vec2::new(5.0, 2.0), &[s], 4.0),
            Ok(None)
        );
        let hit = closest_segment_index(Vec2::new(5.0, 2.0), &[s], 6.0)
            .unwrap()
            .unwrap();
        assert_eq!(hit.index, 0);
        assert_eq!(hit.distance, 2.0);
        assert_eq!(hit.interpolated, 0.5);
    }

    #[test]
    fn picks_the_nearest_of_several_segments() {
        let low = (0.0, 0.0);
        let near = seg((0.0, 1.0), (10.0, 1.0), (0.0, 0.0), low);
        let far = seg((0.0, 4.0), (10.0, 4.0), (0.0, 0.0), low);
        let hit = closest_segment_index(Vec2::new(5.0, 0.0), &[far, near], 1.0)
            .unwrap()
            .unwrap();
        assert_eq!(hit.index, 1);
        assert_eq!(hit.distance, 1.0);
    }

    #[test]
    fn distance_is_to_the_infinite_line_not_the_clamped_segment() {
        let s = seg((0.0, 0.0), (1.0, 0.0), (0.0, 0.0), (0.0, 0.0));
        let hit = closest_segment_index(Vec2::new(50.0, 3.0), &[s], 1.0)
            .unwrap()
            .unwrap();
        assert_eq!(hit.distance, 3.0);
    }

    #[test]
    fn nothing_within_the_initial_radius_of_100_is_found() {
        let s = seg((0.0, 0.0), (10.0, 0.0), (0.0, 0.0), (0.0, 0.0));
        assert_eq!(
            closest_segment_index(Vec2::new(5.0, 100.0), &[s], 1.0),
            Ok(None)
        );
        assert!(
            closest_segment_index(Vec2::new(5.0, 99.0), &[s], 1.0)
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn interpolated_value_uses_the_clamped_parameter() {
        let s = seg((0.0, 0.0), (10.0, 0.0), (2.0, 4.0), (0.0, 0.0));
        let past_end = closest_segment_index(Vec2::new(20.0, 1.0), &[s], 1.0)
            .unwrap()
            .unwrap();
        assert_eq!(past_end.interpolated, 4.0);
        let before = closest_segment_index(Vec2::new(-5.0, 1.0), &[s], 1.0)
            .unwrap()
            .unwrap();
        assert_eq!(before.interpolated, 2.0);
    }

    #[test]
    fn empty_input_finds_nothing() {
        assert_eq!(closest_segment_index(Vec2::ZERO, &[], 1.0), Ok(None));
    }
}
