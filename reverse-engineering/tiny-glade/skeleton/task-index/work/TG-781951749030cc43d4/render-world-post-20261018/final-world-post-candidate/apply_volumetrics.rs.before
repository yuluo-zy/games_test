//! `apply_volumetrics.rs` — 体积光（W48，`apply_volumetrics` 0x1408a44c0，2009 B）。
//!
//! 机器码（O，0x1408a44c0 起）：
//! 1. 资源 `MainCamera`（TypeId `0xbba451ee6a08a3ff / 0x8d008d15febcf29d`，缺失 → `Option::unwrap`）；读相机模式字节 `[camera+0x4d0]`。
//! 2. `TextureDesc::extent_2d(color.desc)` → 之后 `dispatch_2d(w, h)`。
//! 3. 模式 ∈ {4, 1} → `(global_size_scale, global_density_scale) = (10.0, 0.075)`，否则两者都 1.0
//!    （`cmp eax,4; je; cmp eax,1; jne` 常量 0x41200000 / 0x3d99999a / 0x3f800000）。
//! 4. `ComputePass::new("Volumetrics", "/apply_volumetrics.cs.hlsl")`（名长 0xb、URI 长 0x1a）。
//! 5. 顺序：uniform `bool_render_shadows = true` → `shadow_map`（预设 0x11，读 0x19）→ `blue_noise_texture`
//!    （磁盘纹理 `blue_noise`，读 0x19）→ `color_tex`（写 0x32）→ `depth_tex`（读 0x19）→
//!    uniform `current_view_index = 0 (i32)` → `global_density_scale` → `global_size_scale` → `dispatch_2d(w, h)`。
//!
//! 偏差/U：相机模式 1/4 的枚举名未知（U，只能给数值）；`bind_texture` 的第 5 实参（值 2，I：视图种类）A4 未建模。

use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resource_access::{RgTexture, WriteTextureUnchecked};
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::pass_kit::{
    BLUE_NOISE, PassEnv, binding, dispatch_extent,
};

pub const NAME: &str = "Volumetrics";
pub const URI: &str = "/apply_volumetrics.cs.hlsl";
/// `MainCamera` 的 TypeId（O，`get_resource_id` 实参）。
pub const CAMERA_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0xbba4_51ee_6a08_a3ff,
    hi: 0x8d00_8d15_febc_f29d,
};
/// `shadow_map` 预设槽（`TexturePreset(0x11)`）。
pub const SHADOW_MAP_PRESET: u8 = 0x11;

/// `(global_density_scale, global_size_scale)`（O：相机模式 4 或 1 用大尺度/低密度）。
pub fn volumetric_scales(camera_mode: u8) -> (f32, f32) {
    if camera_mode == 4 || camera_mode == 1 {
        (0.075, 10.0)
    } else {
        (1.0, 1.0)
    }
}

/// W48：体积光 compute pass。`camera_mode` = `MainCamera+0x4d0`。
pub fn apply_volumetrics(
    env: &mut PassEnv<'_>,
    camera_mode: u8,
    color: &RgTexture,
    depth: &RgTexture,
) -> RecordedComputePass {
    let (density, size) = volumetric_scales(camera_mode);
    let pass = env
        .pass(NAME, URI)
        .uniform(binding::BOOL_RENDER_SHADOWS, true)
        .bind_texture(binding::SHADOW_MAP, &TexturePreset(SHADOW_MAP_PRESET), 0x19)
        .bind_texture(binding::BLUE_NOISE_TEXTURE, &BLUE_NOISE, 0x19)
        .bind_texture(binding::COLOR_TEX, &WriteTextureUnchecked(*color), 0x32)
        .bind_texture(binding::DEPTH_TEX, &depth, 0x19)
        .uniform(binding::CURRENT_VIEW_INDEX, 0i32)
        .uniform(binding::GLOBAL_DENSITY_SCALE, density)
        .uniform(binding::GLOBAL_SIZE_SCALE, size);
    dispatch_extent(pass, color)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{env, rgtex};

    #[test]
    fn scales_by_camera_mode() {
        assert_eq!(volumetric_scales(1), (0.075, 10.0));
        assert_eq!(volumetric_scales(4), (0.075, 10.0));
        for m in [0u8, 2, 3, 5, 255] {
            assert_eq!(volumetric_scales(m), (1.0, 1.0), "mode {m}");
        }
    }

    #[test]
    fn declares_one_pass_in_exe_order() {
        let mut e = env(
            &[
                binding::SHADOW_MAP,
                binding::BLUE_NOISE_TEXTURE,
                binding::COLOR_TEX,
                binding::DEPTH_TEX,
            ],
            &[
                binding::BOOL_RENDER_SHADOWS,
                binding::CURRENT_VIEW_INDEX,
                binding::GLOBAL_DENSITY_SCALE,
                binding::GLOBAL_SIZE_SCALE,
            ],
        );
        let (color, depth) = (rgtex(10, 1920, 1080), rgtex(11, 960, 540));
        let pass = e.run(|env| apply_volumetrics(env, 4, &color, &depth));
        assert_eq!(pass.name, NAME);
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec![format!("{URI}:cs_main")]
        );
        // dispatch_2d(color extent)
        match pass.dispatch {
            crate::render::pass_builder::compute_pass_builder::DispatchMode::Threads {
                x,
                y,
                z,
            } => {
                assert_eq!((x, y, z), (1920, 1080, 1))
            }
            _ => panic!("dispatch_2d expected"),
        }
        assert_eq!(e.uniform_u32(&pass, binding::BOOL_RENDER_SHADOWS), 1);
        assert_eq!(e.uniform_u32(&pass, binding::CURRENT_VIEW_INDEX), 0);
        assert_eq!(e.uniform_f32(&pass, binding::GLOBAL_DENSITY_SCALE), 0.075);
        assert_eq!(e.uniform_f32(&pass, binding::GLOBAL_SIZE_SCALE), 10.0);
        // 绑定顺序：color_library(0) + shadow_map、color_tex、depth_tex（RG 访问）；color_tex 为写、其余读。
        // blue_noise 是磁盘纹理（`TextureFromDisk`）：走外部描述符，不登记 RG 访问（O 0x1409056d0）。
        assert_eq!(pass.external_bindings.len(), 1);
        let writes: Vec<bool> = pass
            .builder
            .accesses
            .iter()
            .skip(1)
            .map(|a| crate::render::pass_builder::pass_resource_access::access_is_write(a.access))
            .collect();
        assert_eq!(writes, [false, true, false]);
        let res: Vec<u32> = pass
            .builder
            .accesses
            .iter()
            .skip(2)
            .map(|a| a.resource)
            .collect();
        assert_eq!(res, [10, 11]);
    }

    #[test]
    fn non_camera_modes_keep_unit_scales() {
        let mut e = env(
            &[],
            &[binding::GLOBAL_DENSITY_SCALE, binding::GLOBAL_SIZE_SCALE],
        );
        let t = rgtex(1, 8, 8);
        let pass = e.run(|env| apply_volumetrics(env, 0, &t, &t));
        assert_eq!(e.uniform_f32(&pass, binding::GLOBAL_DENSITY_SCALE), 1.0);
        assert_eq!(e.uniform_f32(&pass, binding::GLOBAL_SIZE_SCALE), 1.0);
    }
}
