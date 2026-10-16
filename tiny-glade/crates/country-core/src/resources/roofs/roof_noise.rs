//! B `resources/roofs/roof_noise.rs` — 屋顶顶点噪声位移（章节 O；8 函数）。
//!
//! `gpunoise` 是 shader 风格 2D value-noise 的 CPU 复刻（floor + 4 角 Wang 型哈希 +
//! smoothstep 插值）；`apply_roof_noise` 0x1412348f0 把噪声按强度叠加到屋顶顶点 y。
//! 常量（机器码更正）：采样尺度 **0.5**、振幅 **0.3**、smoothstep **(3−2s)·s²**。
//! 消费方：roof_visual_shape 的 `get_height_01_unclamped_from_ws`（0x1408e4790）与
//! 屋顶网格生成。

/// 角点种子表 @0x142ac7e10..e1c：e10/e14 = 0x0ae135d3、e18/e1c = 0（机器码 O）。
const CORNER_SEED_0: u32 = 0x0ae135d3;
const CORNER_SEED_1: u32 = 0x0ae135d3;
const CORNER_SEED_2: u32 = 0x0;
const CORNER_SEED_3: u32 = 0x0;

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

/// 2D 坐标哈希（角点种子；机器码角种子表 @0x142ac7e10）。
pub fn hash2(x: u32, y: u32, corner: u32) -> u32 {
    let h = hash1(x ^ (y << 8));
    hash_combine2(h ^ corner, y)
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

/// `gpunoise::noise` 0x140a152c0：2D value-noise（floor 格 + 4 角 Wang 型哈希 +
/// smoothstep 双线性插值），返回 [0,1]。
pub fn noise(x: f32, z: f32) -> f32 {
    let x0 = x.floor();
    let z0 = z.floor();
    let fx = x - x0;
    let fz = z - z0;
    let x0 = x0 as i32 as u32;
    let z0 = z0 as i32 as u32;
    let n00 = uint_to_u01_float(hash2(x0, z0, CORNER_SEED_0));
    let n10 = uint_to_u01_float(hash2(x0.wrapping_add(1), z0, CORNER_SEED_1));
    let n01 = uint_to_u01_float(hash2(x0, z0.wrapping_add(1), CORNER_SEED_2));
    let n11 = uint_to_u01_float(hash2(x0.wrapping_add(1), z0.wrapping_add(1), CORNER_SEED_3));
    // smoothstep `(3−2t)·t²` 双线性插值。
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sz = fz * fz * (3.0 - 2.0 * fz);
    let nx = lerp(n00, n10, sx);
    let ny = lerp(n01, n11, sx);
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
