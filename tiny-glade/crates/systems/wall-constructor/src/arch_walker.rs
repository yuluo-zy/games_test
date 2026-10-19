//! 拱/檐梁/支架共用的沿曲线砖步进器（arch_walker.rs）。
//!
//! 原 B 源：`crates/systems/wall-constructor/src/arch_walker.rs`。
//!
//! `ArchWalker::new`（0x14074c2d0，17B 手解）：{+0x00 u32（new 实参，调用点
//! 传 0）、+0x04 f32、+0x08 brick_len f32、+0x0c = 0.1（常量尾砖系数）}。
//!
//! `walk`（4 实例同构：0x141232bb0 arches / 0x141233380 lintels emit_support_beams /
//! 0x141231ea0 stair brackets / 0x14048ab50 playermade stairs closure<false>）。
//! 参数面（由 0x141232bb0 原生汇编入口与 0x141232f13+ 循环逐指令核对）：
//!
//! ```text
//! walk(&self,
//!      heights: &[f32],   // RDX=ptr, R8=len；逐点高度
//!      points:  &[Vec3],  // R9=ptr, [rsp+0x28]=len；逐点平面曲线（world xz+y 基高）
//!      us:      &[f32],   // [rsp+0x30]=ptr, [rsp+0x38]=len；逐点 CurveU
//!      closure: impl FnMut(Vec3, f32)) // (采样点, 插值 u)
//! ```
//!
//! 采样点 = `Vec3::new(p.x, p.y + heights[i], p.z)`（原 0x140c9cde0
//! `xvy(p.xz(), p.y + h)`，汇编 0x141232f3c..0x141232f6f 逐字）。
//!
//! 算法（0x141232bb0）：
//! 1. `total_len = Σ‖P(i+1)−P(i)‖`，`P(i)=(x, y+h[i], z)`；
//!    循环上界 `min(points.len, heights.len)`。
//! 2. `n = max(1, floor(total_len / brick_len + 0.5))`（0x3f000000=0.5，
//!    上限 1.0 由 `fVar5 <= 1.0 → 1.0`；float→u64 饱和转换守卫
//!    0x5f000000/0x5f7fffff）；`step = total_len / n`。
//! 3. 步进上界再取 `min(…, us.len)`。逐段：
//!    `remaining = residual + seg_len`；`if step < remaining` 时循环：
//!    `d = max(step − residual, 0)`；`traversed += d`；
//!    `fraction = traversed / seg_len`；
//!    `u = project_clamp(lerp(us[i], us[i+1], fraction))`（原
//!    `wall_space::subtract` + `project_clamp`；超出 [0,1] 或 NaN 触发
//!    panic_fmt/panic，见 0x1412332cf/0x141233333）；
//!    `residual = max(residual − step, 0)`；`closure(lerp(Pi,Pi+1,fraction), u)`；
//!    `remaining = (seg_len − traversed) + residual`。
//!    循环退出后 `remaining < 0` panic（NaN 路径），`residual = remaining`。
//! 4. 尾砖：`if step * tail_ratio(0.1) < residual` → 以最后一点与
//!    `us[last]` 再发一砖（0x1412331ae..；bounds check `us.len > points.len−1`）。
use glam::Vec3;

/// B `ArchWalker`（0x14074c2d0）。
#[derive(Clone, Copy, Debug)]
pub struct ArchWalker {
    /// +0x00（new 实参；调用点 0）。
    pub kind: u32,
    /// +0x04（new 实参；walk 未用）。
    pub extra: f32,
    /// +0x08 砖长。
    pub brick_len: f32,
    /// +0x0c = 0.1 尾砖阈值系数（常量，非参数）。
    pub tail_ratio: f32,
}

impl ArchWalker {
    /// B `ArchWalker::new`（0x14074c2d0，17B 手解）：
    /// `+0=r8d, +4=xmm3, +8=xmm1, +0xc=0.1`。
    pub fn new(kind: u32, extra: f32, brick_len: f32) -> Self {
        Self {
            kind,
            extra,
            brick_len,
            tail_ratio: 0.1,
        }
    }

    /// B `walk`（0x141232bb0，逐语句重建；见模块文档）。
    ///
    /// `heights[i]` 是该点的附加高度，`points[i]` 是平面/基高曲线点，
    /// `us[i]` 是该点的 CurveU。闭包收到采样点与周期投影后的 u。
    pub fn walk(
        &self,
        heights: &[f32],
        points: &[Vec3],
        us: &[f32],
        mut closure: impl FnMut(Vec3, f32),
    ) {
        // 原入口：`if points.len == 0 panic_bounds_check`、
        // `if heights.len == 0 panic_bounds_check`（0x141232bc3/0x141232c2c）。
        assert!(!points.is_empty(), "arch walk: empty points");
        assert!(!heights.is_empty(), "arch walk: empty heights");

        let point = |index: usize| -> Vec3 {
            Vec3::new(
                points[index].x,
                points[index].y + heights[index],
                points[index].z,
            )
        };

        // total_len：上界 min(points.len, heights.len)（0x141232c9b）。
        let length_bound = points.len().min(heights.len());
        let mut total_len = 0.0f32;
        let mut previous = point(0);
        for index in 1..length_bound {
            let current = point(index);
            let d = current - previous;
            total_len += (d.x * d.x + d.y * d.y + d.z * d.z).sqrt();
            previous = current;
        }

        // n = max(1, floor(total / brick_len + 0.5))，饱和转 u64（0x141232dec）。
        let mut n = (total_len / self.brick_len + 0.5).floor();
        if n <= 1.0 {
            n = 1.0;
        }
        let n = if n.is_nan() {
            u64::MAX
        } else if n <= 0.0 {
            0
        } else if n >= u64::MAX as f32 {
            u64::MAX
        } else {
            n as u64
        };
        let step = total_len / n as f32;

        // 步进上界：min(points, heights, us)（0x141232e71）。
        let walk_bound = length_bound.min(us.len());
        if walk_bound < 2 {
            return;
        }

        let mut residual = 0.0f32;
        let mut segment_start = point(0);
        for index in 1..walk_bound {
            let segment_end = point(index);
            let delta = segment_end - segment_start;
            let segment_len = (delta.x * delta.x + delta.y * delta.y + delta.z * delta.z).sqrt();
            let mut remaining = residual + segment_len;
            if step < remaining {
                let mut traversed = 0.0f32;
                loop {
                    let mut distance = step - residual;
                    if distance <= 0.0 {
                        distance = 0.0;
                    }
                    traversed += distance;
                    let fraction = traversed / segment_len;
                    // 原地形：`wall_space::impl$1::subtract()` +
                    // `project_clamp()` 的周期投影。
                    let u_start = us[index - 1];
                    let u_end = us[index];
                    let interpolated = u_start + (u_end - u_start) * fraction;
                    let u = if interpolated.is_finite() {
                        let wrapped = interpolated - interpolated.floor();
                        // project_clamp：闭区间 [0,1]；1.0 保留为 1.0。
                        if wrapped == 0.0 && interpolated > 0.0 {
                            1.0
                        } else {
                            wrapped.clamp(0.0, 1.0)
                        }
                    } else {
                        interpolated
                    };
                    assert!(u.is_finite(), "arch walk: non-finite curve coordinate");
                    assert!(
                        (0.0..=1.0).contains(&u),
                        "arch walk: curve coordinate out of range"
                    );
                    residual = (residual - step).max(0.0);
                    // 采样点 = (1−f)·起点 + f·终点（原 SUBPS/MULPS 序）。
                    let sample = segment_start * (1.0 - fraction) + segment_end * fraction;
                    closure(sample, u);
                    remaining = (segment_len - traversed) + residual;
                    if !(step < remaining) {
                        break;
                    }
                }
            }
            assert!(remaining >= 0.0, "arch walk: negative remaining distance");
            segment_start = segment_end;
            residual = remaining;
        }

        // 尾砖（0x1412331ae）：`step * 0.1 < residual` 时补最后一点。
        if step * self.tail_ratio < residual {
            let last = walk_bound - 1;
            assert!(
                last < us.len(),
                "arch walk: tail curve coordinate out of bounds"
            );
            closure(point(last), project_clamp_unit(us[last]));
        }
    }
}

/// `utils::wall_space::impl$1::project_clamp` 的非 NaN 路径：
/// 把值投影到 `[0,1]`，NaN 保持 NaN（由调用方 panic）。
fn project_clamp_unit(value: f32) -> f32 {
    if !value.is_finite() {
        return value;
    }
    let wrapped = value - value.floor();
    if wrapped == 0.0 && value > 0.0 {
        1.0
    } else {
        wrapped.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_has_constant_tail_ratio() {
        let walker = ArchWalker::new(0, 0.0, 0.56);
        assert_eq!(walker.tail_ratio.to_bits(), 0.1f32.to_bits());
        assert_eq!(walker.brick_len, 0.56);
    }

    #[test]
    fn walk_steps_evenly_along_straight_line() {
        // 10 m 直线，brick_len=2 → n=5, step=2；砖位于 2/4/6/8/10。
        let points = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0)];
        let heights = [0.0, 0.0];
        let us = [0.0, 1.0];
        let walker = ArchWalker::new(0, 0.0, 2.0);
        let mut emitted = Vec::new();
        walker.walk(&heights, &points, &us, |p, u| emitted.push((p.x, u)));
        let xs: Vec<f32> = emitted.iter().map(|(x, _)| *x).collect();
        assert_eq!(xs.len(), 5);
        for (i, x) in xs.iter().enumerate() {
            assert!((x - 2.0 * (i + 1) as f32).abs() < 1e-5, "x={x}");
        }
        assert!((emitted[0].1 - 0.2).abs() < 1e-5);
        assert!((emitted[4].1 - 1.0).abs() < 1e-5);
    }

    #[test]
    fn walk_supports_multiple_segments_and_range() {
        // 折线：0→3m→5m；brick_len=1 → 3D 总长 5m，n=5，step=1。
        let points = vec![
            Vec3::ZERO,
            Vec3::new(3.0, 0.0, 0.0),
            Vec3::new(3.0, 0.0, 2.0),
        ];
        let heights = [0.0, 0.0, 0.0];
        let us = [0.0, 0.6, 1.0];
        let walker = ArchWalker::new(0, 0.0, 1.0);
        let mut emitted = Vec::new();
        walker.walk(&heights, &points, &us, |p, u| emitted.push((p, u)));
        assert_eq!(emitted.len(), 5);
        for (_, u) in &emitted {
            assert!((0.0..=1.0).contains(u));
        }
        // 第 4 块跨入第二段，u 应在 0.6 与 1.0 之间。
        assert!(emitted[3].1 > 0.6);
        assert!((emitted[4].0 - Vec3::new(3.0, 0.0, 2.0)).length() < 1e-5);
    }

    #[test]
    fn walk_uses_heights_in_3d_length() {
        // 平面 3m、末端抬高 4m → 3D 总长 5m；brick_len=1 → 5 块。
        let points = vec![Vec3::ZERO, Vec3::new(3.0, 0.0, 0.0)];
        let heights = [0.0, 4.0];
        let us = [0.0, 1.0];
        let walker = ArchWalker::new(0, 0.0, 1.0);
        let mut count = 0;
        walker.walk(&heights, &points, &us, |_, _| count += 1);
        assert_eq!(count, 5);
    }
}
