//! `dof_autofocus.rs` — DoF autofocus（W32，O：`calculate_dof_autofocus` 0x140a17880，2714 B）。
//!
//! 一个 compute pass：`"Mouse Depth Write to SSBO"`（pass 名，O 字符串，25 B；沿用复制粘贴的名字）+
//! 源 `/dof/autofocus.cs.hlsl`。HLSL 来源见 `reconstruction/shader-source/shaders/dof/autofocus.cs.hlsl`
//! （`$Globals` 字段名即下方 uniform 键的原名；名字→哈希关系见 `pass_kit::binding_hash`）。
//!
//! # 恢复内容（O）
//! 1. 取 5 个 ECS 资源（缺一 `Option::unwrap` panic，O：5 处 `unwrap_failed`）：
//!    `view_kind: u32`（`== 0x13` 参与门控）、`WindowSize`、`MainCamera`（焦点参数表，按 `+0x4d0` 的相机下标选行）、
//!    `lock_request`（仅 `mode < 2 && view_kind == 0x13` 才取，否则 `0xffff_ffff`）、
//!    `mouse_pos`（仅 `mode == 2` 才取）。
//! 2. `point_uv`：
//!    * `mode == 2`：`(mouse + (0.5, 0.5)) / window`（O：`addps [0.5,0.5]` → `cvtsi2ss`/`divps`；`use_dual_focus` 槽写 `true`）；
//!    * `mode == 3`：`(0.5 + cam[+8] / aspect * 0.5, 0.5 - cam[+4] * 0.5)`；
//!    * 其余：`(0.5 + cam[+0x14] / aspect * 0.5, 0.5 - cam[+0x10] * 0.5)`；`aspect = w / h`（`cvtsi2ss`，h=0 得 inf，不 panic）。
//! 3. 7 个 uniform（顺序 O）：`point_uv`(Vec2)、`use_dual_focus`(bool)、`focus_box_height`(f32 = `cam[+0x18]`)、
//!    `focus_lock_request`(i32)、`focus_is_locked`(bool = `mode < 2 && view_kind == 0x13`)、
//!    `use_tilt_shift`(bool = `camx[+0x128]`)、`aperture_size`(f32 = `camx[+0x118]`)。
//! 4. `write_storage_buffer<DofAutofocusSsbo>` + `write_storage_buffer<DofFocusReadbackSsbo>`，
//!    `bind_texture(depth_tex, 0x19)`，`dispatch_2d(8, 8)`（O：闭包环境里读自 `.rdata 0x142ac7e70` 的 16 B
//!    = `{mode 1, x 8, y 8, z 1}`；反汇编注释把它显示成 `0.0` 向量是 f32 非规格化数的假象）。
//!
//! # 偏离
//! `DofAutofocusSsbo` / `DofFocusReadbackSsbo` 的真实载体在 `resources` 里是 typed gap，不满足 `LibrarySsbo`；
//! 这里对两个 SSBO 类型做泛型参数（`A`/`R`），调用方传真实载体。

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::RgTexture;

use super::pass_kit::{PassEnv, binding_hash};

/// pass 名（O 字符串 `"Mouse Depth Write to SSBO"`，len 0x19）。
pub const PASS_NAME: &str = "Mouse Depth Write to SSBO";
/// 着色器 URI（O，len 0x16）。
pub const SHADER_URI: &str = "/dof/autofocus.cs.hlsl";

pub const H_POINT_UV: u64 = binding_hash("point_uv");
pub const H_USE_DUAL_FOCUS: u64 = binding_hash("use_dual_focus");
pub const H_FOCUS_BOX_HEIGHT: u64 = binding_hash("focus_box_height");
pub const H_FOCUS_LOCK_REQUEST: u64 = binding_hash("focus_lock_request");
pub const H_FOCUS_IS_LOCKED: u64 = binding_hash("focus_is_locked");
pub const H_USE_TILT_SHIFT: u64 = binding_hash("use_tilt_shift");
pub const H_APERTURE_SIZE: u64 = binding_hash("aperture_size");
pub const H_AUTOFOCUS_BUF: u64 = binding_hash("autofocus_buf");
pub const H_FOCUS_READBACK_BUF: u64 = binding_hash("focus_readback_buf");
pub const H_DEPTH_TEX: u64 = binding_hash("depth_tex");

/// `view_kind` 门控值（O：`cmp esi, 0x13`）。
pub const VIEW_KIND_GATE: u32 = 0x13;
/// 贴图读访问码（O：`bind_texture_to_compute_pass` 的 `r9b = 0x19`）。
pub const ACCESS_SAMPLED_READ: u8 = 0x19;

/// `WindowSize`（O：两个 u32，`aspect_ratio` = `w / h` 以 f32 做）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {
    /// `WindowSize::aspect_ratio`：`cvtsi2ss` 两个 u32 后相除。
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

/// `MainCamera` 里一行焦点参数（O：`camera + table1[cam_index]`，表 `.rdata 0x142ac8948` 的 6 个偏移
/// `0x170,0x228,0x2a8,0x348,0x400,0x488`）。只列被本函数读到的字段。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FocusParams {
    /// `+0x00`：u32，焦点模式（`mode - 2 < 2` 的比较用）。
    pub mode: u32,
    /// `+0x04`（mode 3 的 y 项）。
    pub f04: f32,
    /// `+0x08`（mode 3 的 x 项）。
    pub f08: f32,
    /// `+0x10`（其余模式的 y 项）。
    pub f10: f32,
    /// `+0x14`（其余模式的 x 项）。
    pub f14: f32,
    /// `+0x18`：`focus_box_height`。
    pub box_height: f32,
}

/// `MainCamera` 的第二行（O：`camera + table2[cam_index]`，表 `.rdata 0x142ac8978`：
/// `0x8c,0x144,0x1c4,0x264,0x31c,0x3a4`）：`+0x128` u8 倾斜移位开关，`+0x118` f32 光圈。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CameraLens {
    pub use_tilt_shift: bool,
    pub aperture_size: f32,
}

/// `AutofocusInputs`：函数体里 5 个 `get_resource_id` 取到的数据。可选项对应"按需才取"的资源。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutofocusInputs {
    pub view_kind: u32,
    pub window: WindowSize,
    pub focus: FocusParams,
    pub lens: CameraLens,
    /// 资源 `+4` 的 u32；仅 `mode < 2 && view_kind == 0x13` 时被读。
    pub lock_request: Option<u32>,
    /// 鼠标像素坐标；仅 `mode == 2` 时被读。
    pub mouse_pos: Option<[f32; 2]>,
}

/// 计算后的 uniform 取值（O 顺序见文件头）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutofocusUniforms {
    pub point_uv: [f32; 2],
    pub use_dual_focus: bool,
    pub box_height: f32,
    pub focus_lock_request: i32,
    pub focus_is_locked: bool,
    pub use_tilt_shift: bool,
    pub aperture_size: f32,
}

const NONE_UNWRAP: &str = "called `Option::unwrap()` on a `None` value";

/// 函数体前半（纯计算部分）：O 分支逻辑。
pub fn autofocus_uniforms(i: &AutofocusInputs) -> AutofocusUniforms {
    let gate = i.focus.mode < 2 && i.view_kind == VIEW_KIND_GATE;
    // `bVar13 && bVar14` 为真时才取 lock_request 资源（缺失 → unwrap panic）。
    let focus_lock_request = if gate {
        i.lock_request.expect(NONE_UNWRAP) as i32
    } else {
        -1
    };
    // `mode - 2 < 2 ? mode - 2 : 2`（O：`add eax,-2; cmp eax,2; cmovb`）。
    let sel = match i.focus.mode.wrapping_sub(2) {
        v if v < 2 => v,
        _ => 2,
    };
    let (point_uv, use_dual_focus) = match sel {
        0 => {
            let m = i.mouse_pos.expect(NONE_UNWRAP);
            (
                [
                    (m[0] + 0.5) / i.window.width as f32,
                    (m[1] + 0.5) / i.window.height as f32,
                ],
                true,
            )
        }
        1 => (cam_point(i, i.focus.f08, i.focus.f04), false),
        _ => (cam_point(i, i.focus.f14, i.focus.f10), false),
    };
    AutofocusUniforms {
        point_uv,
        use_dual_focus,
        box_height: i.focus.box_height,
        focus_lock_request,
        focus_is_locked: gate,
        use_tilt_shift: i.lens.use_tilt_shift,
        aperture_size: i.lens.aperture_size,
    }
}

/// `(0.5 + x / aspect * 0.5, 0.5 - y * 0.5)`（O：`mulps [0.5,0.5]`、`subps` 自 `[0, 0.5]`）。
fn cam_point(i: &AutofocusInputs, x: f32, y: f32) -> [f32; 2] {
    let aspect = i.window.aspect_ratio();
    [0.5 + (x / aspect) * 0.5, 0.5 - y * 0.5]
}

/// W32：声明 autofocus pass（O 0x140a17880）。返回已声明完毕的 pass（原版直接交 `PassBuilder::render`）。
pub fn calculate_dof_autofocus<A: LibrarySsbo, R: LibrarySsbo>(
    env: &mut PassEnv<'_>,
    inputs: &AutofocusInputs,
    depth_tex: &RgTexture,
) -> RecordedComputePass {
    let u = autofocus_uniforms(inputs);
    let pass = env.new_pass(PASS_NAME_STATIC, &SHADER_URI);
    pass.uniform(H_POINT_UV, glam::Vec2::from(u.point_uv))
        .uniform(H_USE_DUAL_FOCUS, u.use_dual_focus)
        .uniform(H_FOCUS_BOX_HEIGHT, u.box_height)
        .uniform(H_FOCUS_LOCK_REQUEST, u.focus_lock_request)
        .uniform(H_FOCUS_IS_LOCKED, u.focus_is_locked)
        .uniform(H_USE_TILT_SHIFT, u.use_tilt_shift)
        .uniform(H_APERTURE_SIZE, u.aperture_size)
        .write_storage_buffer::<A>(H_AUTOFOCUS_BUF)
        .write_storage_buffer::<R>(H_FOCUS_READBACK_BUF)
        .bind_texture(H_DEPTH_TEX, &depth_tex, ACCESS_SAMPLED_READ)
        .dispatch_2d(8, 8)
}

/// `ComputePass::new` 要 `&'static str`。
const PASS_NAME_STATIC: &str = PASS_NAME;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::render::pass_builder::pass_resource_access::PassBinding;
    use crate::rhapsody_types::TextureDesc;
    use crate::systems::render_loop::compute_passes::pass_kit::imm;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    fn inputs() -> AutofocusInputs {
        AutofocusInputs {
            view_kind: 0x13,
            window: WindowSize {
                width: 1600,
                height: 800,
            },
            focus: FocusParams {
                mode: 0,
                f04: 0.2,
                f08: 0.4,
                f10: 0.6,
                f14: 0.8,
                box_height: 0.25,
            },
            lens: CameraLens {
                use_tilt_shift: true,
                aperture_size: 2.8,
            },
            lock_request: Some(7),
            mouse_pos: Some([99.5, 49.5]),
        }
    }

    #[test]
    fn hashes_are_exe_immediates() {
        assert_eq!(H_POINT_UV, imm(-0x192809e5814464df));
        assert_eq!(H_USE_DUAL_FOCUS, imm(-0x2a4fc297357dd84));
        assert_eq!(H_FOCUS_BOX_HEIGHT, 0x2783b65eab1269bd);
        assert_eq!(H_FOCUS_LOCK_REQUEST, 0x78dfbf1f882dcc45);
        assert_eq!(H_FOCUS_IS_LOCKED, 0x77e48da9bec3aa07);
        assert_eq!(H_USE_TILT_SHIFT, 0x0d7fa4852b47fdd5);
        assert_eq!(H_APERTURE_SIZE, imm(-0x499a21330554d675));
        assert_eq!(H_AUTOFOCUS_BUF, imm(-0x3d9aab3921b4f750));
        assert_eq!(H_FOCUS_READBACK_BUF, imm(-0x2d220698bdd01d17));
        assert_eq!(H_DEPTH_TEX, 0x4861579da049fbfc);
        assert_eq!(PASS_NAME.len(), 0x19);
        assert_eq!(SHADER_URI.len(), 0x16);
    }

    #[test]
    fn mode_two_uses_mouse_over_window_and_flags_dual() {
        let mut i = inputs();
        i.focus.mode = 2;
        let u = autofocus_uniforms(&i);
        assert_eq!(u.point_uv, [100.0 / 1600.0, 50.0 / 800.0]);
        assert!(u.use_dual_focus);
        // mode == 2：不满足 `mode < 2`，不取 lock_request。
        assert_eq!((u.focus_lock_request, u.focus_is_locked), (-1, false));
    }

    #[test]
    fn mode_three_and_default_use_camera_offsets() {
        let mut i = inputs();
        i.mouse_pos = None;
        i.focus.mode = 3;
        let u = autofocus_uniforms(&i);
        assert_eq!(u.point_uv, [0.5 + (0.4 / 2.0) * 0.5, 0.5 - 0.2 * 0.5]);
        assert!(!u.use_dual_focus);
        for mode in [0u32, 1, 4, 99] {
            i.focus.mode = mode;
            i.lock_request = Some(3);
            let u = autofocus_uniforms(&i);
            assert_eq!(
                u.point_uv,
                [0.5 + (0.8 / 2.0) * 0.5, 0.5 - 0.6 * 0.5],
                "mode {mode}"
            );
        }
    }

    #[test]
    fn lock_gate_requires_low_mode_and_view_kind_0x13() {
        let mut i = inputs();
        i.focus.mode = 1;
        let u = autofocus_uniforms(&i);
        assert_eq!((u.focus_lock_request, u.focus_is_locked), (7, true));
        i.view_kind = 0x12;
        i.lock_request = None; // 门控不成立时不会被取
        let u = autofocus_uniforms(&i);
        assert_eq!((u.focus_lock_request, u.focus_is_locked), (-1, false));
    }

    #[test]
    #[should_panic(expected = "Option::unwrap")]
    fn missing_lock_resource_panics_like_unwrap_failed() {
        let mut i = inputs();
        i.focus.mode = 0;
        i.lock_request = None;
        autofocus_uniforms(&i);
    }

    #[test]
    fn declares_the_original_pass() {
        let uniforms = [
            H_POINT_UV,
            H_USE_DUAL_FOCUS,
            H_FOCUS_BOX_HEIGHT,
            H_FOCUS_LOCK_REQUEST,
            H_FOCUS_IS_LOCKED,
            H_USE_TILT_SHIFT,
            H_APERTURE_SIZE,
        ];
        let mut e = env_with(
            &[H_AUTOFOCUS_BUF, H_FOCUS_READBACK_BUF, H_DEPTH_TEX],
            &uniforms,
            |lib, dev| {
                lib.add_new::<SsboA>(dev);
                lib.add_new::<SsboB>(dev);
            },
        );
        let tex = RgTexture {
            resource: 55,
            desc: TextureDesc::new([32, 16, 1], 0, 0),
        };
        let mut i = inputs();
        i.focus.mode = 1;
        let rec = e.run(|penv| calculate_dof_autofocus::<SsboA, SsboB>(penv, &i, &tex));
        assert_eq!(rec.name, "Mouse Depth Write to SSBO");
        assert_eq!(
            e.world.backend.paths.borrow()[0],
            "/dof/autofocus.cs.hlsl:cs_main"
        );
        assert!(matches!(
            rec.dispatch,
            DispatchMode::Threads { x: 8, y: 8, z: 1 }
        ));
        // 绑定顺序：color library（ctor）→ 两个 SSBO → depth_tex 图像视图。
        assert!(matches!(
            rec.bindings.as_slice(),
            [
                PassBinding::BufferRead { .. },
                PassBinding::Buffer { .. },
                PassBinding::Buffer { .. },
                PassBinding::ImageView {
                    resource: 55,
                    write: false,
                    ..
                }
            ]
        ));
        // 贴图访问码 0x19、同步组 1（读）。
        let a = rec.builder.accesses.last().unwrap();
        assert_eq!((a.resource, a.access, a.group), (55, 0x19, 1));
        // uniform 实际落到块里。
        assert_eq!(e.uniform_f32(&rec, H_FOCUS_BOX_HEIGHT), 0.25);
        assert_eq!(e.uniform_u32(&rec, H_FOCUS_LOCK_REQUEST), 7);
        assert_eq!(e.uniform_u32(&rec, H_FOCUS_IS_LOCKED), 1);
        assert_eq!(e.uniform_u32(&rec, H_USE_DUAL_FOCUS), 0);
        assert_eq!(e.uniform_f32(&rec, H_APERTURE_SIZE), 2.8);
    }
}
