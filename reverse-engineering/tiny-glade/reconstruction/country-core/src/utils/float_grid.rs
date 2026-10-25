//! utils/float_grid.rs — `utils::float_grid::impl$0::bilinear_sample_at_world_xz<G>`。
//!
//! O（机器码）：
//!  * `RasterBufferRef<512>` 实例 0x140a14df0（835 B，被 8 个 rlib 共用）；
//!  * `TerrainHeightFloatGrid` 实例 0x140257ba0（798 B，同一算法，格点读取走
//!    `terrain_height::impl$7::index`，常量 130.0 被内联）。
//!
//! 调用约定（Win64）：`rcx = &grid`、`xmm1 = x`、`xmm2 = z`、`xmm3 = extent_x`、`[rsp+0x20] = extent_z`。
//! preplaced_flower 的 5 个调用点都是 `(grid, x, z, 130.0, 130.0)`（`0x43020000` 同时在 xmm3 与栈上），
//! 即“第 4 个参数 130.0”其实是成对的世界范围（x 范围、z 范围），而不是半宽：`u = x/130 + 0.5`。
//!
//! 算法（O）：`p = (z/ez, x/ex) + 0.5`（SSE `divps`，lane0=z、lane1=x），`idx = (row << 9) | col`，
//! 栅格边长固定 512，格点 = `u8 / 255.0`，结果 `lerp(lerp(A, C, fz), lerp(B, D, fz), fx)`
//! 其中 A=(row0,col0)、B=(row0,col1)、C=(row1,col0)、D=(row1,col1)，`lerp(a,b,t)=a+(b-a)*t`（`utils::lerp`）。
//! 入口处有一个全局字节开关（`*[0x14362c308]`，U：写入者未找到，.bss 初值 0）选择两条路径：
//!  * 开关 != 0（“居中”路径，fall-through）：`t = min(p,1)*512 - 0.5`，`t = max(t,0)`，
//!    `i = sat_u32(t)`，`i0 = min(i,511)`，`i1 = min(i+1,511)`，`frac = t - i`；
//!  * 开关 == 0（“向量”路径）：`t = clamp(p,0,1)*512`（NaN→0），`i = min(sat_u32(t), 510)`，
//!    `i1 = i+1`，`frac = t - i`（t==512 时 frac=2，原样保留）。
//! 两路径的格点顺序与 lerp 顺序相同。

use core::sync::atomic::{AtomicBool, Ordering};

/// 栅格边长（O：`shl 9` / `0x1ff`）。
pub const GRID_SIDE: usize = 512;
/// preplaced_flower 传入的世界范围（O：`0x43020000`）。
pub const DEFAULT_WORLD_EXTENT: f32 = 130.0;

/// 原 EXE 里 `*[0x14362c308]` 字节开关的镜像（U：来源未知；.bss 初值 0 → 向量路径）。
pub static CENTERED_PATH: AtomicBool = AtomicBool::new(false);

/// 浮点网格：按线性格点下标（`row*512 + col`）取 0..1 的值（O：`u8/255.0` 或地形高度读取）。
pub trait FloatGridCells {
    fn cell(&self, index: usize) -> f32;
}

/// `utils::lerp`（O 0x140c904d0：`a + (b-a)*t`）。
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// `cvttss2si` + 下溢/上溢比较的饱和 `f32 -> u32`（O；NaN → 0 与 Rust `as` 一致）。
#[inline]
fn sat_u32(v: f32) -> u32 {
    v as u32
}

/// O：`minss`（NaN 时返回第二操作数）等价于 `a.min(b)`；`maxss` 同理。
#[inline]
fn clamp01_to_cells(c: f32, extent: f32) -> f32 {
    // 向量路径：maxps(0) → cmpord 屏蔽 NaN → minps(1) → cmpunord 屏蔽；NaN 结果 0（O）。
    let p = c / extent + 0.5;
    if p.is_nan() {
        return 0.0;
    }
    p.max(0.0).min(1.0) * GRID_SIDE as f32
}

fn bilinear_vector_path<G: FloatGridCells + ?Sized>(
    grid: &G,
    x: f32,
    z: f32,
    ex: f32,
    ez: f32,
) -> f32 {
    let (tz, tx) = (clamp01_to_cells(z, ez), clamp01_to_cells(x, ex));
    let (row, col) = (sat_u32(tz).min(0x1fe), sat_u32(tx).min(0x1fe));
    let (fz, fx) = (tz - row as f32, tx - col as f32);
    let base = ((row as usize) << 9) | col as usize;
    let a = grid.cell(base);
    let b = grid.cell(base + 1);
    let c = grid.cell(base + GRID_SIDE);
    let d = grid.cell(base + GRID_SIDE + 1);
    lerp(lerp(a, c, fz), lerp(b, d, fz), fx)
}

fn bilinear_centered_path<G: FloatGridCells + ?Sized>(
    grid: &G,
    x: f32,
    z: f32,
    ex: f32,
    ez: f32,
) -> f32 {
    let t = |c: f32, e: f32| -> f32 {
        // minss(p, 1.0) (NaN → 1.0), *512, -0.5, maxss(.,0) (NaN → 0)
        let p = c / e + 0.5;
        let p = if p.is_nan() { 1.0 } else { p.min(1.0) };
        let v = p * GRID_SIDE as f32 - 0.5;
        if v.is_nan() { 0.0 } else { v.max(0.0) }
    };
    let (tz, tx) = (t(z, ez), t(x, ex));
    let (iz, ix) = (sat_u32(tz), sat_u32(tx));
    let (fz, fx) = (tz - iz as f32, tx - ix as f32);
    let (r0, c0) = (iz.min(0x1ff) as usize, ix.min(0x1ff) as usize);
    let (r1, c1) = (
        iz.wrapping_add(1).min(0x1ff) as usize,
        ix.wrapping_add(1).min(0x1ff) as usize,
    );
    let a = grid.cell((r0 << 9) | c0);
    let b = grid.cell((r0 << 9) | c1);
    let c = grid.cell((r1 << 9) | c0);
    let d = grid.cell((r1 << 9) | c1);
    lerp(lerp(a, c, fz), lerp(b, d, fz), fx)
}

/// `bilinear_sample_at_world_xz`：世界 XZ 双线性采样。`extent_x/extent_z` = 世界范围（调用点恒为 130.0）。
pub fn bilinear_sample_at_world_xz<G: FloatGridCells + ?Sized>(
    grid: &G,
    x: f32,
    z: f32,
    extent_x: f32,
    extent_z: f32,
) -> f32 {
    if CENTERED_PATH.load(Ordering::Relaxed) {
        bilinear_centered_path(grid, x, z, extent_x, extent_z)
    } else {
        bilinear_vector_path(grid, x, z, extent_x, extent_z)
    }
}

/// 同上，但 `(130.0, 130.0)`（O：preplaced_flower 的全部调用点 / 地形高度实例内联常量）。
pub fn bilinear_sample_at_world_xz_130<G: FloatGridCells + ?Sized>(
    grid: &G,
    x: f32,
    z: f32,
) -> f32 {
    bilinear_sample_at_world_xz(grid, x, z, DEFAULT_WORLD_EXTENT, DEFAULT_WORLD_EXTENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 以 `u8/255` 为格点的 512×512 平面：值 = col（低 8 位）。
    struct Ramp;
    impl FloatGridCells for Ramp {
        fn cell(&self, index: usize) -> f32 {
            assert!(
                index < GRID_SIDE * GRID_SIDE + GRID_SIDE + 2,
                "超出 512x512 + 余量"
            );
            ((index & 0x1ff) as u8) as f32 / 255.0
        }
    }
    struct Rows;
    impl FloatGridCells for Rows {
        fn cell(&self, index: usize) -> f32 {
            ((index >> 9) as u8) as f32 / 255.0
        }
    }

    #[test]
    fn lerp_is_a_plus_b_minus_a_times_t() {
        assert_eq!(lerp(2.0, 6.0, 0.25), 3.0);
    }

    #[test]
    fn vector_path_world_origin_is_cell_256() {
        // x=0 → 0.5*512 = 256，frac 0 → 正好格点 col 256
        let v = bilinear_vector_path(&Ramp, 0.0, 0.0, 130.0, 130.0);
        assert!((v - (256u32 as u8) as f32 / 255.0).abs() < 1e-6, "{v}");
    }

    #[test]
    fn vector_path_interpolates_between_columns() {
        // t = 0.5 + 0.1/130*... 取 x 使 tx = 256.5：x = 0.5/512*130
        let x = 0.5 / 512.0 * 130.0;
        let v = bilinear_vector_path(&Ramp, x, 0.0, 130.0, 130.0);
        let expect = (0.0 + 1.0 / 255.0 * 0.5) as f32; // col 256 → 0, col 257 → 1/255
        assert!((v - expect).abs() < 1e-5, "{v} vs {expect}");
    }

    #[test]
    fn z_selects_rows_and_x_selects_columns() {
        let z_only = bilinear_vector_path(&Rows, 0.0, 10.0, 130.0, 130.0);
        let x_only = bilinear_vector_path(&Rows, 10.0, 0.0, 130.0, 130.0);
        assert!(z_only > x_only, "行由 z 决定: {z_only} {x_only}");
    }

    #[test]
    fn vector_path_clamps_to_510_with_frac_two_at_the_edge() {
        // 超界 → t=512，row/col 取 510，frac=2：lerp 外推
        let v = bilinear_vector_path(&Ramp, 1e9, 0.0, 130.0, 130.0);
        let (a, b) = ((510u32 as u8) as f32 / 255.0, (511u32 as u8) as f32 / 255.0);
        // col510→254, col511→255 ; row 256 → 同值，lerp(a,b,2)
        let ca = (510u32 & 0x1ff) as u8 as f32 / 255.0;
        let cb = (511u32 & 0x1ff) as u8 as f32 / 255.0;
        let _ = (a, b);
        assert!((v - lerp(ca, cb, 2.0)).abs() < 1e-5);
    }

    #[test]
    fn vector_path_nan_maps_to_origin_cell() {
        let v = bilinear_vector_path(&Ramp, f32::NAN, f32::NAN, 130.0, 130.0);
        assert_eq!(v, 0.0);
    }

    #[test]
    fn centered_path_shifts_by_half_a_cell_and_clamps_to_511() {
        // x 恰在 tx = 256.5 + 0.5 → 取 col 257 frac 0
        let x = 1.0 / 512.0 * 130.0; // p*512 = 257 → t = 256.5 → i=256 frac .5
        let v = bilinear_centered_path(&Ramp, x, 0.0, 130.0, 130.0);
        let expect = (0.0 + 1.0 / 255.0 * 0.5) as f32;
        assert!((v - expect).abs() < 1e-5, "{v}");
        // 上界：t = 511.5 → i0 = 511, i1 = 511 (min 0x1ff)，frac 0.5 → 值 = col 511
        let v = bilinear_centered_path(&Ramp, 1e9, 0.0, 130.0, 130.0);
        assert!((v - (511u32 & 0xff) as f32 / 255.0).abs() < 1e-6);
        // 下界：负超界 → t=0
        let v = bilinear_centered_path(&Ramp, -1e9, 0.0, 130.0, 130.0);
        assert_eq!(v, 0.0);
    }

    #[test]
    fn public_entry_uses_switch_and_130_wrapper_matches_explicit_extent() {
        let a = bilinear_sample_at_world_xz_130(&Ramp, 3.0, 4.0);
        let b = bilinear_sample_at_world_xz(&Ramp, 3.0, 4.0, 130.0, 130.0);
        assert_eq!(a, b);
        // 范围参数确实参与：extent 减半 → 不同结果
        assert_ne!(a, bilinear_sample_at_world_xz(&Ramp, 3.0, 4.0, 65.0, 65.0));
    }
}
