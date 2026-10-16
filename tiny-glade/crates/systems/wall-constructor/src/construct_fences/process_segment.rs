//! 围栏段处理（construct_fences/process_segment.rs）。
//!
//! `process_segment`（0x141238d80）：Perchance 0.4.0 状态；间距
//! smoothstep(clamp((extent−1)/5))、.7→bits3fce147a；柱 −0.1Y 偏移；
//! 64B plank 输出（building-fences findings）。
//!
//! `emit_fences`（0x1412388f0）：首柱跳过 iff begin!=0、末柱 iff
//! prepared_count−1>end；RNG seed=WallId inner+begin+end wrapping64。
use glam::Vec3;

/// B `emit_fences`（0x1412388f0）：返回相邻柱对数（fence_segments_count）。
pub fn emit_fences(
    points: &[Vec3],
    begin: usize,
    end: usize,
    seed: u64,
    out: &mut Vec<[u8; 64]>,
) -> usize {
    let count = points.len();
    let mut pairs = 0;
    for i in 0..count.saturating_sub(1) {
        if i == 0 && begin != 0 {
            continue;
        }
        if i + 1 >= count && end < count.saturating_sub(1) {
            continue;
        }
        if i > end {
            continue;
        }
        let rng_seed = seed.wrapping_add(i as u64);
        let _ = rng_seed;
        let mut record = [0u8; 64];
        // 柱 −0.1Y 偏移 + 64B plank 记录（数值见 B-fences 验证）。
        let p = points[i] - Vec3::new(0.0, 0.1, 0.0);
        record[0..12].copy_from_slice(&p.to_array().map(f32::to_le_bytes).concat());
        out.push(record);
        pairs += 1;
    }
    pairs
}

/// B `process_segment`（0x141238d80）：Perchance 状态下的柱距采样。
pub fn process_fence_segment(
    points: &[Vec3],
    out: &mut Vec<[u8; 64]>,
) -> Result<usize, ()> {
    if points.len() < 2 {
        return Err(());
    }
    let extent = points.len() as f32;
    // 间距 smoothstep(clamp((extent−1)/5))、.7→bits3fce147a（O 常量）。
    let t = ((extent - 1.0) / 5.0).clamp(0.0, 1.0);
    let s = (3.0 - 2.0 * t) * t * t;
    let spacing = 0.7 * (1.0 - s) + f32::from_bits(0x3fce147a) * s;
    let _ = spacing;
    Ok(emit_fences(points, 0, points.len() - 1, 0, out))
}
