//! `mouse_depth_value.rs` — 鼠标处深度回读（W33，`mouse_depth_compute` 0x140a01620，1742 B）。
//!
//! 机器码（O）：三个 world 资源，缺失任一 → `Option::unwrap`：
//! * `0x4cab10763b3fdb80 / 0x01a422ef7534b54d`：前两个字 `(a, b)` 原样（位模式）作 `Vec2` uniform `mouse_position_screenspace`
//!   （光标屏幕坐标）。
//! * `0x3d7b56e787d47c40 / 0x67b0b10a8d43bfc7`：读 `f32 @+0x110`，夹到 [0,1] 后写 uniform `filter_box_y_frac`
//!   （`0.0 <= v ? v : 0.0` 再 `v <= 1.0 ? v : 1.0`，故 NaN → 0）。资源名 U（相机相关，I）。
//! * `0x864ddca98c2312a6 / 0x27b0e9369e9a484e`：`WindowSize {u32 w, u32 h}`，`(w as f32, h as f32)` 写 uniform `window_size`。
//!
//! `ComputePass::new("Mouse Depth Write to SSBO", "/read_mouse_depth_value.cs.hlsl")`（名长 0x19、URI 长 0x1f）。顺序：
//! `write_to_cpu_mapped_storage_buffer<MouseDepthValueSsbo>(out_buffer)` → `depth_map`（`&Handle`，读 0x19）→
//! uniform `mouse_position_screenspace` → `window_size` → `filter_box_y_frac` → `dispatch_1d(1)`（`{1,1,1,1}`）。
//!
//! `MouseDepthValueSsbo`（`resources::compute_mouse_depth_value`，别的模块）：SsboId 0x75bd131453883cb7 = xxh3_64("MouseDepthValue")，
//! 作泛型参数。

use glam::Vec2;

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding, binding_hash};

pub const NAME: &str = "Mouse Depth Write to SSBO";
pub const URI: &str = "/read_mouse_depth_value.cs.hlsl";
pub const DEPTH_MAP: u64 = binding_hash("depth_map");
pub const MOUSE_POSITION_SCREENSPACE: u64 = binding_hash("mouse_position_screenspace");
pub const WINDOW_SIZE: u64 = binding_hash("window_size");
pub const FILTER_BOX_Y_FRAC: u64 = binding_hash("filter_box_y_frac");

/// 光标位置资源 TypeId（O）。
pub const CURSOR_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0x4cab_1076_3b3f_db80,
    hi: 0x01a4_22ef_7534_b54d,
};
/// `f32 @+0x110` 所在资源 TypeId（O；名 U）。
pub const FOCUS_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0x3d7b_56e7_87d4_7c40,
    hi: 0x67b0_b10a_8d43_bfc7,
};
/// `WindowSize` TypeId（O）。
pub const WINDOW_SIZE_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0x864d_dca9_8c23_12a6,
    hi: 0x27b0_e936_9e9a_484e,
};
/// 上面第二个资源里被读取的字段偏移（O）。
pub const FOCUS_FIELD_OFFSET: usize = 0x110;

/// `filter_box_y_frac`：夹到 [0,1]，NaN → 0（O：两次 `comiss` 的写法）。
pub fn filter_box_y_frac(value: f32) -> f32 {
    let lower = if 0.0 <= value { value } else { 0.0 };
    if lower <= 1.0 { lower } else { 1.0 }
}

/// W33：鼠标深度 compute pass。`cursor` = 光标屏幕坐标；`focus_value` = 资源 `FOCUS_TYPE_ID` 的 `f32 @+0x110`；
/// `window` = `(w, h)`。
pub fn mouse_depth_compute<Out: SsboType>(
    env: &mut PassEnv<'_>,
    cursor: Vec2,
    focus_value: f32,
    window: (u32, u32),
    depth: &RgTexture,
) -> RecordedComputePass {
    env.pass(NAME, URI)
        .write_to_cpu_mapped_storage_buffer::<Out>(binding::OUT_BUFFER)
        .bind_texture(DEPTH_MAP, &depth, 0x19)
        .uniform(MOUSE_POSITION_SCREENSPACE, cursor)
        .uniform(WINDOW_SIZE, Vec2::new(window.0 as f32, window.1 as f32))
        .uniform(FILTER_BOX_Y_FRAC, filter_box_y_frac(focus_value))
        .dispatch_1d(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        accesses, env_with, rgtex, test_carrier, threads,
    };

    test_carrier!(MouseDepth, "MouseDepthValue");

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(DEPTH_MAP, 0x6fce_509d_259d_aacf);
        assert_eq!(MOUSE_POSITION_SCREENSPACE, 0x3b8f_ce71_6c29_ac83);
        assert_eq!(WINDOW_SIZE, 0x69fd_ab9b_991c_4787);
        assert_eq!(FILTER_BOX_Y_FRAC, 0xc10b_fe30_e41e_d046);
        assert_eq!(binding::OUT_BUFFER, 0xa868_b696_44bd_36ba);
    }

    #[test]
    fn frac_is_clamped_and_nan_maps_to_zero() {
        assert_eq!(filter_box_y_frac(-3.0), 0.0);
        assert_eq!(filter_box_y_frac(0.25), 0.25);
        assert_eq!(filter_box_y_frac(7.0), 1.0);
        assert_eq!(filter_box_y_frac(f32::NAN), 0.0);
        assert_eq!(filter_box_y_frac(f32::INFINITY), 1.0);
    }

    #[test]
    fn declares_uniforms_in_exe_order_and_dispatches_one_thread() {
        let mut e = env_with(
            &[binding::OUT_BUFFER, DEPTH_MAP],
            &[MOUSE_POSITION_SCREENSPACE, WINDOW_SIZE, FILTER_BOX_Y_FRAC],
            |lib, dev| {
                lib.add_new::<MouseDepth>(dev);
            },
        );
        let depth = rgtex(9, 100, 50);
        let pass = e.run(|env| {
            mouse_depth_compute::<MouseDepth>(env, Vec2::new(12.5, 7.0), 1.75, (1920, 1080), &depth)
        });
        assert_eq!(pass.name, NAME);
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec![format!("{URI}:cs_main")]
        );
        assert_eq!(threads(&pass), (1, 1, 1));
        let m = e.uniform(&pass, MOUSE_POSITION_SCREENSPACE, 8);
        assert_eq!(f32::from_le_bytes(m[..4].try_into().unwrap()), 12.5);
        assert_eq!(f32::from_le_bytes(m[4..].try_into().unwrap()), 7.0);
        let w = e.uniform(&pass, WINDOW_SIZE, 8);
        assert_eq!(f32::from_le_bytes(w[..4].try_into().unwrap()), 1920.0);
        assert_eq!(f32::from_le_bytes(w[4..].try_into().unwrap()), 1080.0);
        assert_eq!(
            e.uniform_f32(&pass, FILTER_BOX_Y_FRAC),
            1.0,
            "1.75 clamps to 1"
        );
        let a = accesses(&pass);
        assert_eq!(a.iter().map(|x| x.1).collect::<Vec<_>>(), [0x32, 0x19]);
        assert_eq!(a[1].0, 9);
    }
}
