//! 圆形支柱（construct_elevation_supports/circle_pillars.rs）。
//!
//! `construct_circle_pillars`（0x14124ba30，1016 行 fresh decomp）。
//! reviewer-C 轮 3 终验机器码（三方核对标准）：
//! - **无 0x3f99999a(1.2)/0x3e4ccccd(0.2) 常量**（旧实现不实，已弃）；
//! - `_UNK_142a7cd48` 实测 = **7.0f**、`_UNK_142934694` = **4.0f**：
//!   `count = round(clamp01(ratio)·7.0 + (1−clamp01(ratio))·4.0)`
//!   （ratio = func_0x000140c904a0 的 [0,1] 归一，钳 _UNK_14295a0e0 上界）；
//! - 角向均匀步进 `t = i/count`（断言 t∈[0,1] 非 NaN）；
//! - 每点：`FUN_140c975a0`（形状点采样）→
//!   `FUN_141246fb0`（TerrainHeightsData::sample_at_world_xz）→
//!   `terrain + max(0,ratio) <= wall_height(+0x280)` 门控 → 支柱；
//! - 支柱高度：`find_stone_column_start_y`（0x141250720）+ smoothstep
//!   公式（(h−y+_UNK_142a7b650)/5.0 → smoothstep → ·1.5 + 偏移族
//!   _UNK_142c204d0/4/8/c）；
//! - 支柱实体：wall 条目（stride 0x50，entity@+0x38）→ PublicWalls::get →
//!   get_wall_shape → contains_point 校验；
//! - `wall+0x298` 字节门控第二变体（半木构系；常量族 0.5×3
//!   _UNK_1429258e0/8e4/8ec/70 + `wall+0x280·_UNK_14292c76c` 计数）。
use glam::Vec3;

/// B `construct_circle_pillars`（0x14124ba30）：圆形墙支柱。
/// `ratio` = func_0x000140c904a0 输出（[0,1] 归一）；`wall_height` = wall+0x280；
/// `terrain_sample` = TerrainHeightsData::sample_at_world_xz（0x141246fb0）。
pub fn construct_circle_pillars(
    shape_point_at: impl Fn(f32) -> Vec3,
    ratio: f32,
    wall_height: f32,
    terrain_sample: impl Fn(Vec3) -> f32,
    contains_point: impl Fn(Vec3) -> bool,
    start_y: impl Fn(f32, f32, f32) -> f32,
    out: &mut Vec<Vec3>,
) {
    // count = round(lerp(4, 7, clamp01(ratio)))（7.0 = _UNK_142a7cd48、
    // 4.0 = _UNK_142934694；钳 _UNK_14295a0e0 上界）。
    let ratio_c = ratio.clamp(0.0, 1.0);
    let count = (ratio_c * 7.0 + (1.0 - ratio_c) * 4.0).round();
    let count = count.clamp(0.0, 4096.0) as u32;
    let max_ratio = ratio.max(0.0);
    for i in 0..count {
        let t = i as f32 / count.max(1) as f32;
        assert!(
            t.is_finite() && (0.0..=1.0).contains(&t),
            "circle pillar u out of range"
        );
        let point = shape_point_at(t);
        // terrain + max(0, ratio) <= wall_height 门控。
        let terrain = terrain_sample(point);
        if terrain + max_ratio > wall_height {
            continue;
        }
        if !contains_point(point) {
            continue;
        }
        // 支柱起始高度（find_stone_column_start_y 链）+ smoothstep 高度
        // （(h−y+_UNK_142a7b650)/5.0 → smoothstep → ·1.5；偏移族登记）。
        let start = start_y(point.y, terrain, wall_height);
        let h = ((wall_height - start + f32::from_bits(0x3f4ccccd)) / 5.0).clamp(0.0, 1.0);
        let smooth = (3.0 - 2.0 * h) * h * h;
        let y = start + smooth * 1.5;
        out.push(Vec3::new(point.x, y, point.z));
    }
}
