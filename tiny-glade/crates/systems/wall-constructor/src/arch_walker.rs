//! 拱/檐梁/支架共用的沿曲线砖步进器（arch_walker.rs）。
//!
//! `ArchWalker::new`（0x14074c2d0，17B 手解）：{+0x00 u32（new 实参，调用点
//! 传 0）、+0x04 f32、+0x08 brick_len f32、+0x0c = 0.1（常量尾砖系数）}。
//!
//! `walk`（4 实例同构，re-analyst-C O 机器码差分）：0x141232bb0（arches）、
//! 0x141233380（lintels emit_support_beams）、0x141231ea0（stair brackets）、
//! 0x14048ab50（playermade stairs closure<false>）。
//! 算法：total_len = Σ‖p[i+1]−p[i]‖（两列取 min 计数）；n = max(1, floor(total/
//! brick_len + 0.5))；brick_len' = total/n；沿曲线逐段推进，`t` 越界 [0,1] 或
//! NaN → panic；收尾 `if brick_len'·0.1 < 剩余` → 再发一砖。
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
    /// B `ArchWalker::new`（0x14074c2d0）：+0x0c 恒 0.1。
    pub fn new(kind: u32, extra: f32, brick_len: f32) -> Self {
        Self {
            kind,
            extra,
            brick_len,
            tail_ratio: 0.1,
        }
    }

    /// B `walk`（4 实例同构）：沿两条点列（取 min 计数）每砖调一次 closure。
    /// t 越界/NaN 按原断言 panic（payload = f32 fmt）。
    pub fn walk(
        &self,
        points_a: &[Vec3],
        points_b: &[Vec3],
        mut closure: impl FnMut(Vec3, Vec3, f32),
    ) {
        let count = points_a.len().min(points_b.len());
        if count < 2 {
            return;
        }
        let mut total = 0.0f32;
        for i in 0..count - 1 {
            let seg = points_a[i + 1] - points_a[i];
            total += seg.length();
        }
        let n = (total / self.brick_len + 0.5).floor().max(1.0) as u32;
        let step = total / n as f32;
        let mut travelled = 0.0f32;
        let mut emitted = 0u32;
        let mut seg_start = 0.0f32;
        for i in 0..count - 1 {
            let a = points_a[i];
            let b = points_a[i + 1];
            let seg = (b - a).length();
            if seg == 0.0 {
                continue;
            }
            let mut t = (step - travelled).max(0.0) / seg;
            while step >= travelled + seg * t {
                // t ∈ [0,1] 且非 NaN（原断言）。
                assert!(
                    t.is_finite() && (0.0..=1.0).contains(&t),
                    "arch walk interpolation out of range"
                );
                let dir = (b - a).normalize();
                let at = a + dir * (t * seg);
                let at_b = if points_b.len() == points_a.len() {
                    points_b[i] + (points_b[i + 1] - points_b[i]).normalize_or_zero() * (t * seg)
                } else {
                    at
                };
                closure(at, at_b, t);
                emitted += 1;
                travelled = step * emitted as f32;
                if emitted >= n {
                    break;
                }
                t = (step - travelled).max(0.0) / seg;
            }
            seg_start += seg;
        }
        // 收尾：`if brick_len'·0.1 < 剩余` → 再发一砖。
        let remaining = total - travelled;
        if step * self.tail_ratio < remaining && emitted < n {
            let a = points_a[count - 2];
            let b = points_a[count - 1];
            let dir = (b - a).normalize_or_zero();
            let at = a + dir * remaining;
            closure(at, at, 1.0);
        }
        let _ = (seg_start, points_b);
    }
}
