//! `reprojection.rs` — 重投影图（W36，`calculate_reprojection_map` 0x140a0d190，2410 B）。
//!
//! 机器码（O）：入参 `(recorder, world, depth: &Handle, velocity: &Handle, prev: &PrevDepth)`，其中 `prev` 是 0x28 B：
//! 0x24 B 的 RG 句柄（上一帧深度，`+0..0x24`）+ `u8` 标志 `@+0x24`（上一帧深度有效）。
//! 1. 新建 RG 纹理 `"reprojection map"`（名长 0x10）：`TextureDesc::new_2d(extent_2d(depth.desc), 0x5c)`，不带 storage/sampled 链。
//! 2. `ComputePass::new("Reprojection map", "/calculate_reprojection_map.cs.hlsl")`（名长 0x10、URI 长 0x23）：
//!    uniform i32 `history_valid = prev.flag` → `depth_tex`（读 0x19）→ `prev_depth_tex`（`&prev` 句柄，读）→
//!    `velocity_tex`（读）→ `output_tex`（写 0x32，= 新图）→ `dispatch_2d(extent_2d(新图.desc))`。
//! 3. `ComputePass::new("Copy depth", "/copy_depth_to_r.cs.hlsl")`（名长 0xa、URI 长 0x18）：`input_tex`（= depth，读）→
//!    `output_tex`（写 0x32，= `prev` 句柄，即把本帧深度拷进历史）→ `dispatch_2d(extent_2d(prev.desc))`。
//! 4. 返回新图句柄（0x24 B 拷回 `param_1`）。

use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::{RgTexture, WriteTextureUnchecked};
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding, dispatch_extent, extent_of};

pub const MAP_NAME: &str = "reprojection map";
pub const PASS_NAME: &str = "Reprojection map";
pub const URI: &str = "/calculate_reprojection_map.cs.hlsl";
pub const COPY_NAME: &str = "Copy depth";
pub const COPY_URI: &str = "/copy_depth_to_r.cs.hlsl";
pub const PREV_DEPTH_TEX: u64 = crate::systems::render_loop::compute_passes::pass_kit::binding_hash("prev_depth_tex");
pub const VELOCITY_TEX: u64 = crate::systems::render_loop::compute_passes::pass_kit::binding_hash("velocity_tex");
/// `TextureDesc::new_2d` 的格式码（O：0x5c）。
pub const MAP_FORMAT: u32 = 0x5c;

/// 上一帧深度（RG 句柄）+ 有效标志（O：0x28 B 结构）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrevDepth {
    pub texture: RgTexture,
    pub valid: u8,
}

/// `calculate_reprojection_map` 的结果：新图句柄 + 两个已声明的 pass（图执行顺序）。
pub struct ReprojectionMap {
    pub map: RgTexture,
    pub map_pass: RecordedComputePass,
    pub copy_pass: RecordedComputePass,
}

/// W36：计算重投影图并把本帧深度拷进历史。
pub fn calculate_reprojection_map(
    env: &mut PassEnv<'_>,
    depth: &RgTexture,
    velocity: &RgTexture,
    prev: &PrevDepth,
) -> ReprojectionMap {
    let spec = TextureDesc::new_2d([extent_of(depth).width, extent_of(depth).height], MAP_FORMAT);
    let map = env.create_texture(MAP_NAME, spec);

    let pass = env
        .pass(PASS_NAME, URI)
        .uniform(binding::HISTORY_VALID, prev.valid as i32)
        .bind_texture(binding::DEPTH_TEX, &depth, 0x19)
        .bind_texture(PREV_DEPTH_TEX, &&prev.texture, 0x19)
        .bind_texture(VELOCITY_TEX, &velocity, 0x19)
        .bind_texture(binding::OUTPUT_TEX, &WriteTextureUnchecked(map), 0x32);
    let map_pass = dispatch_extent(pass, &map);

    let copy = env
        .pass(COPY_NAME, COPY_URI)
        .bind_texture(binding::INPUT_TEX, &depth, 0x19)
        .bind_texture(binding::OUTPUT_TEX, &WriteTextureUnchecked(prev.texture), 0x32);
    let copy_pass = dispatch_extent(copy, &prev.texture);

    ReprojectionMap {
        map,
        map_pass,
        copy_pass,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{accesses, env, rgtex, threads};

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(PREV_DEPTH_TEX, 0x647f_a570_442a_32f4);
        assert_eq!(VELOCITY_TEX, 0x97e5_140e_eba9_8244);
        assert_eq!(binding::INPUT_TEX, 0x11ba_5697_cc40_b3dd);
    }

    fn run(
        valid: u8,
    ) -> (
        crate::systems::render_loop::compute_passes::pass_kit::test_env::Env,
        ReprojectionMap,
    ) {
        let mut e = env(
            &[
                binding::DEPTH_TEX,
                PREV_DEPTH_TEX,
                VELOCITY_TEX,
                binding::OUTPUT_TEX,
                binding::INPUT_TEX,
            ],
            &[binding::HISTORY_VALID],
        );
        let (depth, vel) = (rgtex(10, 1280, 720), rgtex(11, 1280, 720));
        let prev = PrevDepth {
            texture: rgtex(12, 640, 360),
            valid: valid,
        };
        let out = e.run(|env| calculate_reprojection_map(env, &depth, &vel, &prev));
        (e, out)
    }

    #[test]
    fn creates_map_with_depth_extent_and_format_0x5c() {
        let (e, out) = run(1);
        let (name, spec, tex) = &e.frame.textures[0];
        assert_eq!(name.as_str(), MAP_NAME);
        assert_eq!(
            (spec.extent_2d(), spec.format, spec.storage, spec.sampled),
            ([1280, 720], 0x5c, false, false)
        );
        assert_eq!(out.map, *tex);
    }

    #[test]
    fn map_pass_reads_three_writes_map_and_dispatches_map_extent() {
        let (e, out) = run(1);
        assert_eq!(out.map_pass.name, PASS_NAME);
        assert_eq!(threads(&out.map_pass), (1280, 720, 1));
        assert_eq!(e.uniform_u32(&out.map_pass, binding::HISTORY_VALID), 1);
        let a = accesses(&out.map_pass);
        assert_eq!(
            a.iter().map(|x| x.0).collect::<Vec<_>>(),
            [10, 12, 11, out.map.resource]
        );
        assert_eq!(a.iter().map(|x| x.1).collect::<Vec<_>>(), [0x19, 0x19, 0x19, 0x32]);
    }

    #[test]
    fn copy_pass_writes_prev_history_with_its_own_extent() {
        let (_e, out) = run(0);
        assert_eq!(out.copy_pass.name, COPY_NAME);
        assert_eq!(threads(&out.copy_pass), (640, 360, 1));
        let a = accesses(&out.copy_pass);
        assert_eq!((a[0].0, a[0].1), (10, 0x19));
        assert_eq!((a[1].0, a[1].1), (12, 0x32));
    }

    #[test]
    fn history_valid_flag_passes_through_unmodified() {
        let (e, out) = run(0);
        assert_eq!(e.uniform_u32(&out.map_pass, binding::HISTORY_VALID), 0);
        let (e, out) = run(7);
        assert_eq!(
            e.uniform_u32(&out.map_pass, binding::HISTORY_VALID),
            7,
            "the byte is widened, not coerced to bool"
        );
    }
}
