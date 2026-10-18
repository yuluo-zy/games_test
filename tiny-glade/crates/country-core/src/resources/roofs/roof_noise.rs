//! B `resources/roofs/roof_noise.rs` — 屋顶顶点噪声位移（章节 O；8 函数）。
//!
//! `gpunoise` 是 shader 风格 2D value-noise 的 CPU 复刻（floor + 4 角 Wang 型哈希 +
//! smoothstep 插值）；`apply_roof_noise` 0x1412348f0 把噪声按强度叠加到屋顶顶点 y。
//! 常量（机器码更正）：采样尺度 **0.5**、振幅 **0.3**、smoothstep **(3−2s)·s²**。
//! 消费方：roof_visual_shape 的 `get_height_01_unclamped_from_ws`（0x1408e4790）与
//! 屋顶网格生成。

/// 角种子（机器码 O）：simd 角 = 角种子表 @0x142ac7e10 元素 0/1 = 0x0ae135d3
/// （0x140a152c0 SIMD `paddd xmm7, [@0x142ac7e10]`；表 0x18/0x1c = 0 未用）。
const CORNER_SEED: u32 = 0x0ae135d3;
/// scalar 角种子 = CORNER_SEED + 0x19660d² mod 2³²（机器码 scalar 角 = `hash_combine2(hash1(z), x)`
/// = `combine_partial(hash1(z), x) + 0xae135d3 + 0x19660d²`，0x140a152c0 逐指令 0x140a15364/6a/6d/74/7b）。
const SCALAR_CORNER_SEED: u32 = 0x2219_927c; // = 0x0ae135d3 + 0x17385ca9(0x19660d²) mod 2³²

/// `gpunoise::hash1`（内联，行 24）：单 u32 哈希（`(v·0x401>>6 ^ v·0x401)·9`）。
pub fn hash1(v: u32) -> u32 {
    let a = v.wrapping_mul(0x401);
    let h = (a >> 6) ^ a;
    h.wrapping_mul(9)
}

/// `gpunoise::hash_combine2`（内联，行 34）：双 u32 哈希合并
/// （`(h>>0xb ^ h)·0x8001 → (y·0x19660d + h + 0x19660d)·0x19660d + 0xae135d3`）。
pub fn hash_combine2(h: u32, y: u32) -> u32 {
    let h = (h >> 0xb ^ h).wrapping_mul(0x8001);
    let h = y.wrapping_mul(0x19660d).wrapping_add(h).wrapping_add(0x19660d);
    h.wrapping_mul(0x19660d).wrapping_add(0xae135d3)
}

/// `gpunoise::hash_combine2` 的 partial（不含 final 常量）：`(h>>11^h)*0x8001 → (h + x*0x19660d)*0x19660d`
/// （机器码 0x140a1534f/59/60/64：r9d = (h>>11^h)*0x8001、r8d = x*0x19660d、eax = (r9+r8)*0x19660d）。
fn combine_partial(h: u32, x: u32) -> u32 {
    let h = (h >> 0xb ^ h).wrapping_mul(0x8001);
    h.wrapping_add(x.wrapping_mul(0x19660d)).wrapping_mul(0x19660d)
}

/// `gpunoise::hash2` 的 finalizer：`(h>>11^h) → (h<<7 & 0x9d2c5680)^h → (h<<15 & 0xefc60000)^h →
/// (h&0x7fffff)^(h>>18)`（掩码 @0x142ac7e20/30/40 机器码实测）。
fn finalize_hash(h: u32) -> u32 {
    let h = (h >> 0xb) ^ h;
    let h = (h << 7 & 0x9d2c5680) ^ h;
    let h = (h << 0xf & 0xefc60000) ^ h;
    (h & 0x7fffff) ^ (h >> 0x12)
}

/// 2D 坐标哈希（机器码 0x140a152c0 内联）：`finalizer( combine_partial(hash1(z), x) + seed )`。
/// 机器码无 `x^(y<<8)` 前置异或（旧实现编造），无 combine 前角种子异或。
pub fn hash2(x: u32, z: u32, seed: u32) -> u32 {
    finalize_hash(combine_partial(hash1(z), x).wrapping_add(seed))
}

/// `uint_to_u01_float`（内联，行 54）：哈希 → [0,1) 浮点。原 `(bits|0x3f800000)−1.0`
/// 依赖低位 mantissa：**先掩掉指数位**（`&0x7fffff`），否则高指数位使 OR 后成 NaN。
pub fn uint_to_u01_float(bits: u32) -> f32 {
    f32::from_bits((bits & 0x007f_ffff) | 0x3f80_0000) - 1.0
}

/// 线性插值。
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// `gpunoise::noise` 0x140a152c0：2D value-noise（floor 格 + 4 角哈希 + smoothstep 双线性插值），
/// 返回 [0,1]。
///
/// 机器码 SIMD 结构（逐指令对照 0x140a152c0）：每 z 行两角——scalar 角 = `hash_combine2(hash1(z), x)`
/// （= `hash2(x, z, SCALAR_CORNER_SEED)`），simd 角 = `combine_partial(hash1(z), x) + CORNER_SEED`
/// （= `hash2(x, z, CORNER_SEED)`）；行内以 sx（x 插值）、行间以 sz（z 插值）双线性。
/// 机器码不递增 x0（无 x0+1），x 维由同一格的两类哈希经 sx 插值表达。
pub fn noise(x: f32, z: f32) -> f32 {
    let x0 = x.floor();
    let z0 = z.floor();
    let fx = x - x0;
    let fz = z - z0;
    let x0 = x0 as i32 as u32;
    let z0 = z0 as i32 as u32;
    let n00 = uint_to_u01_float(hash2(x0, z0, SCALAR_CORNER_SEED));
    let n10 = uint_to_u01_float(hash2(x0, z0, CORNER_SEED));
    let n01 = uint_to_u01_float(hash2(x0, z0.wrapping_add(1), SCALAR_CORNER_SEED));
    let n11 = uint_to_u01_float(hash2(x0, z0.wrapping_add(1), CORNER_SEED));
    // smoothstep `(3−2t)·t²` 双线性（机器序：行内 simd/scalar 以 sx、行间以 sz）。
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sz = fz * fz * (3.0 - 2.0 * fz);
    let nx = lerp(n10, n00, sx);
    let ny = lerp(n11, n01, sx);
    lerp(nx, ny, sz)
}

/// `apply_roof_noise` 0x1412348f0：`s = clamp(strength,0,1)`；
/// `n = noise(x·0.5, z·0.5)`；`vertex.y += n·0.3·(3−2s)·s²`。
pub fn apply_roof_noise(x: f32, z: f32, strength: f32, vertex_y: &mut f32) {
    let s = strength.clamp(0.0, 1.0);
    let n = noise(x * 0.5, z * 0.5);
    *vertex_y += n * 0.3 * (3.0 - 2.0 * s) * s * s;
}

/// `revert_roof_noise`（内联，行 12）：逆操作（从顶点 y 减噪声）。
pub fn revert_roof_noise(x: f32, z: f32, strength: f32, vertex_y: &mut f32) {
    let s = strength.clamp(0.0, 1.0);
    let n = noise(x * 0.5, z * 0.5);
    *vertex_y -= n * 0.3 * (3.0 - 2.0 * s) * s * s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_is_in_unit_range_and_deterministic() {
        for (x, z) in [(0.0, 0.0), (0.5, 0.5), (10.0, -3.0), (0.1, 0.2)] {
            let n = noise(x, z);
            assert!((0.0..=1.0).contains(&n), "noise({x},{z}) = {n}");
            assert_eq!(noise(x, z), n);
        }
    }

    #[test]
    fn noise_matches_machine_reference() {
        // 机器码数值（LOGIC §1：0x140a152c0 capstone + 同公式 f32 复刻；(0,0) 角 u01=0.5488）。
        for (x, z, expected) in [
            (0.0, 0.0, 0.5487866),
            (0.5, 0.5, 0.4374025),
            (0.0, 0.5, 0.5297968),
            (0.5, 0.0, 0.4771236),
            (1.0, 1.0, 0.2845558),
            (10.0, -3.0, 0.5514752),
        ] {
            let n = noise(x, z);
            assert!(
                (n - expected).abs() < 2e-6,
                "noise({x},{z}) = {n}, machine ~{expected}"
            );
        }
    }

    #[test]
    fn hash2_has_finalizer_and_no_pre_xor() {
        // 旧实现编造了 `hash1(x ^ (y << 8)) ^ corner` 前置异或且无 finalizer；
        // 机器码每角 = finalizer(combine_partial(hash1(z), x) + seed)。
        // 数值对照：hash2(0, 0, CORNER_SEED) 经 uint_to_u01 = 0.54879（机器 (0,0) 角）。
        let h = hash2(0, 0, CORNER_SEED);
        let u = uint_to_u01_float(h);
        assert!((u - 0.5487866).abs() < 2e-6, "hash2(0,0,seed) u01 = {u}");
        // scalar 角 = combine_partial + CORNER_SEED + 0x19660d²（= hash_combine2 内置常量折叠）。
        let scalar = finalize_hash(
            combine_partial(hash1(0), 0).wrapping_add(SCALAR_CORNER_SEED),
        );
        let combine2 = hash_combine2(hash1(0), 0);
        assert_eq!(scalar, finalize_hash(combine2));
    }

    #[test]
    fn apply_noise_scales_with_strength() {
        let mut y = 0.0;
        apply_roof_noise(1.0, 2.0, 0.0, &mut y);
        assert_eq!(y, 0.0); // s=0 → smoothstep 0
        let mut y1 = 0.0;
        apply_roof_noise(1.0, 2.0, 1.0, &mut y1);
        let mut y2 = 0.0;
        apply_roof_noise(1.0, 2.0, 0.5, &mut y2);
        // s=1 → 系数 0.3；s=0.5 → 0.3·(2·0.25)=0.15。
        assert!((y1 - 2.0 * y2).abs() < 1e-5);
        // revert 抵消 apply。
        let mut y = y1;
        revert_roof_noise(1.0, 2.0, 1.0, &mut y);
        assert_eq!(y, 0.0);
    }
}
