//! `half_res.rs` — 半分辨率深度/法线提取（W35，`extract_half_res_depth_normals` 0x1408f6180，1850 B）。
//!
//! 机器码（O）：入参 `(recorder, world, gbuffer: &GBuffer)`，`GBuffer` 里 `+0x00` = gbuffer 句柄、`+0x24` = 深度句柄
//! （各 0x24 B：`u32` 资源号 + 0x20 B `TextureDesc`）。
//! 1. `half = TextureDesc::div_up_extent(depth.desc, [2, 2, 1])`，`ext = extent_2d(half)`。
//! 2. `create_raw_resource("half-res depth")`（名长 0xe）：`new_2d(ext, 0x64)`；
//!    `create_raw_resource("half-res normals")`（名长 0x10）：`new_2d(ext, 0x26)`。
//! 3. `ComputePass::new("Half depth & normals", "/extract_half_res_depth_normals.cs.hlsl")`（名长 0x14、URI 长 0x27）：
//!    `depth_input_tex`（深度，读 0x19）→ `stencil_tex`（深度的模板视图，读 0x19）→ `depth_output_tex`（写 0x32）→
//!    `gbuffer_input_tex`（读）→ `normals_output_tex`（写）→ `dispatch_2d(ext)`。
//! 4. 返回 `(half depth, half normals)` 两个句柄。
//!
//! A4 缺口（U）：`stencil_tex` 原版用 `read_texture_view`（`TextureViewDesc` 取模板 aspect：`bd57a0`/`bd58a0(…, 4)`），
//! A4 的 `bind_texture` 不带视图描述，这里按同一深度句柄绑定、视图差异不建模；视图 aspect 值 4 记在 [`STENCIL_VIEW_ASPECT`]。

use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::{RgTexture, WriteTextureUnchecked};
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, binding, binding_hash, dispatch_extent, extent_of,
};

pub const DEPTH_NAME: &str = "half-res depth";
pub const NORMALS_NAME: &str = "half-res normals";
pub const PASS_NAME: &str = "Half depth & normals";
pub const URI: &str = "/extract_half_res_depth_normals.cs.hlsl";
pub const DEPTH_FORMAT: u32 = 0x64;
pub const NORMALS_FORMAT: u32 = 0x26;
pub const DEPTH_INPUT_TEX: u64 = binding_hash("depth_input_tex");
pub const STENCIL_TEX: u64 = binding_hash("stencil_tex");
pub const DEPTH_OUTPUT_TEX: u64 = binding_hash("depth_output_tex");
pub const GBUFFER_INPUT_TEX: u64 = binding_hash("gbuffer_input_tex");
pub const NORMALS_OUTPUT_TEX: u64 = binding_hash("normals_output_tex");
/// `read_texture_view` 里 `TextureViewDesc` 的参数（O：`bd58a0(.., 4)`；语义 I：模板 aspect）。
pub const STENCIL_VIEW_ASPECT: u32 = 4;

/// G-buffer 句柄组（O：`+0x00` gbuffer、`+0x24` 深度）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GBuffer {
    pub gbuffer: RgTexture,
    pub depth: RgTexture,
}

/// `TextureDesc::div_up_extent(extent, by)`：逐轴向上取整除（O：`0x140bd55e0`；这里只用 2D）。
pub fn div_up_extent(extent: [u32; 2], by: [u32; 2]) -> [u32; 2] {
    [extent[0].div_ceil(by[0]), extent[1].div_ceil(by[1])]
}

/// 半分辨率结果。
pub struct HalfRes {
    pub depth: RgTexture,
    pub normals: RgTexture,
    pub pass: RecordedComputePass,
}

/// W35：提取半分辨率深度 + 法线。
pub fn extract_half_res_depth_normals(env: &mut PassEnv<'_>, gbuffer: &GBuffer) -> HalfRes {
    let full = extent_of(&gbuffer.depth);
    let ext = div_up_extent([full.width, full.height], [2, 2]);
    let depth = env.create_texture(DEPTH_NAME, TextureDesc::new_2d(ext, DEPTH_FORMAT));
    let normals = env.create_texture(NORMALS_NAME, TextureDesc::new_2d(ext, NORMALS_FORMAT));
    let pass = env
        .pass(PASS_NAME, URI)
        .bind_texture(DEPTH_INPUT_TEX, &&gbuffer.depth, 0x19)
        .bind_texture(STENCIL_TEX, &&gbuffer.depth, 0x19)
        .bind_texture(DEPTH_OUTPUT_TEX, &WriteTextureUnchecked(depth), 0x32)
        .bind_texture(GBUFFER_INPUT_TEX, &&gbuffer.gbuffer, 0x19)
        .bind_texture(NORMALS_OUTPUT_TEX, &WriteTextureUnchecked(normals), 0x32);
    let pass = dispatch_extent(pass, &depth);
    let _ = binding::DEPTH_TEX;
    HalfRes {
        depth,
        normals,
        pass,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        accesses, env, rgtex, tex_key, threads,
    };

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(DEPTH_INPUT_TEX, 0xbd11_863f_f328_5ee3);
        assert_eq!(STENCIL_TEX, 0x31e4_b10c_dc9a_4455);
        assert_eq!(DEPTH_OUTPUT_TEX, 0x7021_fc7a_e6e0_ac34);
        assert_eq!(GBUFFER_INPUT_TEX, 0x6e82_49ad_194f_56c5);
        assert_eq!(NORMALS_OUTPUT_TEX, 0x2713_6cb8_3b34_cc67);
    }

    #[test]
    fn div_up_rounds_each_axis_up() {
        assert_eq!(div_up_extent([1920, 1080], [2, 2]), [960, 540]);
        assert_eq!(div_up_extent([1921, 1081], [2, 2]), [961, 541]);
        assert_eq!(div_up_extent([1, 1], [2, 2]), [1, 1]);
        assert_eq!(div_up_extent([0, 3], [2, 2]), [0, 2]);
    }

    #[test]
    fn creates_two_half_res_targets_and_one_dispatch() {
        let mut e = env(
            &[
                DEPTH_INPUT_TEX,
                STENCIL_TEX,
                DEPTH_OUTPUT_TEX,
                GBUFFER_INPUT_TEX,
                NORMALS_OUTPUT_TEX,
            ],
            &[],
        );
        let g = GBuffer {
            gbuffer: rgtex(1, 1921, 1081),
            depth: rgtex(2, 1921, 1081),
        };
        let out = e.run(|env| extract_half_res_depth_normals(env, &g));
        let t = &e.frame.textures;
        assert_eq!(tex_key(&t[0]), (DEPTH_NAME, [961, 541], 0x64));
        assert_eq!(tex_key(&t[1]), (NORMALS_NAME, [961, 541], 0x26));
        assert_eq!(out.pass.name, PASS_NAME);
        assert_eq!(threads(&out.pass), (961, 541, 1));
        let a = accesses(&out.pass);
        assert_eq!(
            a.iter().map(|x| x.1).collect::<Vec<_>>(),
            [0x19, 0x19, 0x32, 0x19, 0x32]
        );
        assert_eq!(
            a.iter().map(|x| x.0).collect::<Vec<_>>(),
            [2, 2, out.depth.resource, 1, out.normals.resource]
        );
    }
}
