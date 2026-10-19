//! 拱剖面（construct_arches/profile.rs）。
//!
//! `create_stone_arch_profile`（0x1407645f0，O）：输入墙顶 Vec3 点列 +
//! Option<&Vec3> + wall_state（+600 计数、+0xe0/+0xe8 切片）；`if len > 0.1875`
//! （0x3e400000）才进：`trim = min(0.95f, 0.1875/len)`（0x3f733333/0x3e400000），
//! 部分范围时 `slice_into(1−trim)`；重采样 n 点 `t = i/(n−1)`，**断言 t∈[0,1]
//! 且 !NaN**（Curve::get_coord_at_u 前）；x/z = lerp(p[u], p[u+1], u)，
//! y = max_y 偏移。常量：0.1875 最小长度、0.95 最大修剪、−0.0 累加器初值。
//!
//! `ArchFunction::remap_t`（0x140765300）：t 重映射函数（拱高曲线）。
use glam::Vec3;

/// B `create_stone_arch_profile`（0x1407645f0）。输出为重采样后的轮廓点列。
/// `wall_state` 为墙 0x2a0 投影（+600 段计数、+0xe0/+0xe8 预备切片）。
pub fn create_stone_arch_profile(
    points: &[Vec3],
    _highest: Option<&Vec3>,
    wall_state: &[u8],
    out: &mut Vec<Vec3>,
) {
    const MIN_LEN: f32 = f32::from_bits(0x3e400000); // 0.1875
    const MAX_TRIM: f32 = f32::from_bits(0x3f733333); // 0.95
    let segment_count = wall_state
        .get(0x258..0x260)
        .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
        .unwrap_or(0);
    let total: f32 = points.windows(2).map(|w| (w[1] - w[0]).length()).sum();
    if total <= MIN_LEN {
        return;
    }
    let trim = (MIN_LEN / total).min(MAX_TRIM);
    let keep = ((1.0 - trim) * points.len() as f32).max(2.0) as usize;
    let usable = &points[..keep];
    let n = (segment_count as usize).clamp(2, 64);
    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        assert!(
            t.is_finite() && (0.0..=1.0).contains(&t),
            "profile u out of range"
        );
        let span = (usable.len() - 1).max(1) as f32;
        let u = t * span;
        let idx = (u as usize).min(usable.len() - 2);
        let frac = u - idx as f32;
        let p = usable[idx] + (usable[idx + 1] - usable[idx]) * frac;
        out.push(p);
    }
}

/// B `ArchFunction`（PDB 命名实体；字段由 0x140765300 访问偏移反推）：
/// `byte0` 模式、`byte1` 镜像、`blend@+4`。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArchFunction {
    pub mode: u8,
    pub mirror: u8,
    pub blend: f32,
}

impl ArchFunction {
    /// B `ArchFunction::remap_t`（0x140765300，逐语句重建）。
    ///
    /// 返回原件 `CONCAT44(fVar4, fVar2)` 的两浮点 `(fVar2, fVar4)`：
    /// - `mode == 0`：`a = 2*|t − 0.5|`；返回
    ///   `(1 − (1 − strength)·a³, 0)`。
    /// - 否则：
    ///   `t2 = 2*|t − 0.5|`；`sq = if mirror { (1−t)² } else { t² }`；
    ///   `k = min(1, max(0, inverse_lerp(2, 1, blend)))`（0x140c904a0，
    ///   常量 2.0/1.0）；`strength' = 1 − strength`；
    ///   `inner = (1−k)·(1 − strength'·t2³)`；
    ///   `outer = inner + k·(1 − strength'·sq)`。
    ///   返回 `(outer, inner)`。
    pub fn remap_t(&self, t: f32, strength: f32) -> (f32, f32) {
        if self.mode == 0 {
            let a = ((t + f32::from_bits(0xbf000000)).abs()) * 2.0; // −0.5
            let value = 1.0 - (1.0 - strength) * a * a * a;
            return (value, 0.0);
        }
        let sq = if self.mirror == 0 {
            t * t
        } else {
            (1.0 - t) * (1.0 - t)
        };
        let strength = 1.0 - strength;
        let t2 = ((t + f32::from_bits(0xbf000000)).abs()) * 2.0;
        // inverse_lerp(2.0, 1.0, blend)。
        let inverse = (self.blend - 2.0) / (1.0 - 2.0);
        let clamped = if inverse >= 0.0 { inverse } else { 0.0 };
        let k = clamped.min(1.0);
        let inner = (1.0 - k) * (1.0 - strength * t2 * t2 * t2);
        let outer = inner + k * (1.0 - strength * sq);
        (outer, inner)
    }
}

/// 兼容旧调用点的单值重映射：`ArchFunction { mode:0, mirror:0, blend:0 }`
/// 的 `(outer, _)`。**旧签名不是原件 ABI**；新代码应使用
/// [`ArchFunction::remap_t`]。
pub fn remap_t(t: f32) -> f32 {
    ArchFunction {
        mode: 0,
        mirror: 0,
        blend: 0.0,
    }
    .remap_t(t, 0.0)
    .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_zero_is_mirrored_cubic() {
        let f = ArchFunction {
            mode: 0,
            mirror: 0,
            blend: 0.0,
        };
        assert_eq!(f.remap_t(0.5, 1.0), (1.0, 0.0));
        assert_eq!(f.remap_t(0.0, 0.5), (0.5, 0.0));
        let (v, _) = f.remap_t(0.25, 0.5);
        // a=0.5 → 1 − 0.5*0.125 = 0.9375
        assert!((v - 0.9375).abs() < 1e-6);
    }
}
