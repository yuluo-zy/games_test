//! `primary_rt_debug.rs` — 主光线（软件光追）调试视图（`primary_rt_debug` 0x1409beca0，1650 B）。
//!
//! 机器码（O，无现成 Ghidra 反编译，按反汇编核对）：入参 `(recorder, world, target: &mut Handle)`；`extent = extent_2d(target.desc)`
//! （`lea rcx,[r8+4]`）。`ComputePass::new("Primary RT", "/primary_rt_debug.cs.hlsl")`（名长 0xa、URI 长 0x19）。顺序：
//! `output_tex`（`&mut Handle`，写 0x32）→ `terrain_heightmap`（预设 9）→ `visual_terrain_heightmap`（预设 0xa）→
//! `terrain_lighting_texture`（预设 8）→ `blue_noise_texture`（磁盘 `blue_noise`，读 0x19）→ uniform bool `bool_render_shadows = true`
//! （`mov r9b,1`）→ `shadow_map`（预设 0x11）→ `bind_ray_tracing::<ComputePass>` → uniform i32 `current_view_index = 0`（`xor r9d,r9d`）→
//! `dispatch_2d(extent)`（闭包状态 `{1, w, h, 1}`）。无 world 资源读取、无分支。

use crate::render::pass_builder::bind_resources::BindGameResources;
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::{RgTexture, WriteTextureUnchecked};
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::pass_kit::{
    BLUE_NOISE, PassEnv, binding, dispatch_extent,
};

pub const NAME: &str = "Primary RT";
pub const URI: &str = "/primary_rt_debug.cs.hlsl";

/// 主光线调试 pass：把光追结果写入 `target`。
pub fn primary_rt_debug(env: &mut PassEnv<'_>, target: &RgTexture) -> RecordedComputePass {
    let mut pass = env
        .pass(NAME, URI)
        .bind_texture(binding::OUTPUT_TEX, &WriteTextureUnchecked(*target), 0x32)
        .bind_texture(binding::TERRAIN_HEIGHTMAP, &TexturePreset(9), 0x19)
        .bind_texture(binding::VISUAL_TERRAIN_HEIGHTMAP, &TexturePreset(0xa), 0x19)
        .bind_texture(binding::TERRAIN_LIGHTING_TEXTURE, &TexturePreset(8), 0x19)
        .bind_texture(binding::BLUE_NOISE_TEXTURE, &BLUE_NOISE, 0x19)
        .uniform(binding::BOOL_RENDER_SHADOWS, true)
        .bind_texture(binding::SHADOW_MAP, &TexturePreset(0x11), 0x19);
    BindGameResources::bind_ray_tracing(&mut pass);
    let pass = pass.uniform(binding::CURRENT_VIEW_INDEX, 0i32);
    dispatch_extent(pass, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        accesses, env, rgtex, threads,
    };

    #[test]
    fn declares_exe_sequence_and_dispatches_target_extent() {
        let mut e = env(
            &[
                binding::OUTPUT_TEX,
                binding::TERRAIN_HEIGHTMAP,
                binding::VISUAL_TERRAIN_HEIGHTMAP,
                binding::TERRAIN_LIGHTING_TEXTURE,
                binding::BLUE_NOISE_TEXTURE,
                binding::SHADOW_MAP,
            ],
            &[binding::BOOL_RENDER_SHADOWS, binding::CURRENT_VIEW_INDEX],
        );
        let t = rgtex(33, 1280, 720);
        let pass = e.run(|env| primary_rt_debug(env, &t));
        assert_eq!(pass.name, NAME);
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec![format!("{URI}:cs_main")]
        );
        assert_eq!(threads(&pass), (1280, 720, 1));
        assert_eq!(e.uniform_u32(&pass, binding::BOOL_RENDER_SHADOWS), 1);
        assert_eq!(e.uniform_u32(&pass, binding::CURRENT_VIEW_INDEX), 0);
        let a = accesses(&pass);
        assert_eq!(a[0], (33, 0x32, 0), "output_tex first, write");
        assert!(
            a[1..].iter().all(|x| x.1 == 0x19 || x.1 == 0x1a),
            "everything else is a read (ray-tracing buffers read 0x1a)"
        );
        // 4 个 RG 纹理访问 + 光追缓冲（Tri/Meta/Bvh/WallColorId）；blue_noise 是磁盘纹理（外部描述符，不登记 RG 访问）
        assert_eq!(a.len(), 1 + 4 + 4);
        assert_eq!(pass.external_bindings.len(), 1);
    }
}
