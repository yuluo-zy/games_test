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
    let total: f32 = points
        .windows(2)
        .map(|w| (w[1] - w[0]).length())
        .sum();
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

/// B `ArchFunction::remap_t`（0x140765300）。
pub fn remap_t(t: f32) -> f32 {
    // 观测：t 的拱高重映射（1−t 镜像 + 平滑；精确系数见 decomp-0x140765300）。
    let u = if t < 0.0 { 0.0 } else if t > 1.0 { 1.0 } else { t };
    1.0 - (1.0 - u) * (1.0 - u)
}
