//! RasterBufferRef<512> consumer of shared B bilinear sampling policy.
//! 0x140a14df0: byte values /255, original lerp order and edge extrapolation.
//! Branch byte 0x143638fac (pointer slot 0x14362c308) is caller state. Its producer is
//! `load_game` (`version >= 0x1a`, 0x14146dc82..0x14146dc95); see
//! `resources::save_file::LoadVersionGates::apply_centered_path`.
pub use ::utils::float_grid::{
    CENTERED_PATH, DEFAULT_WORLD_EXTENT, FloatGridCells, GRID_SIDE, bilinear_sample_at_world_xz,
    bilinear_sample_at_world_xz_130, lerp,
};

#[cfg(test)]
mod tests {
    use super::*;
    fn bilinear_vector_path<G: FloatGridCells + ?Sized>(
        grid: &G,
        x: f32,
        z: f32,
        ex: f32,
        ez: f32,
    ) -> f32 {
        ::utils::float_grid::bilinear_sample_at_world_xz_with_mode(grid, x, z, ex, ez, false)
    }
    fn bilinear_centered_path<G: FloatGridCells + ?Sized>(
        grid: &G,
        x: f32,
        z: f32,
        ex: f32,
        ez: f32,
    ) -> f32 {
        ::utils::float_grid::bilinear_sample_at_world_xz_with_mode(grid, x, z, ex, ez, true)
    }

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
