//! `restir_di.rs` — ReSTIR 直接光照蓄水池时间复用（W41，`restir_sample_direct_lights` 0x140a6fec0，2583 B）。
//!
//! 机器码（O）：入参 `(recorder, world, half_depth, half_normal, reprojection, grid: &LightGrid)`，
//! `LightGrid` = `+0x00` `light_indirection_grid` 缓冲、`+0x18` `grid_light_indices` 缓冲。
//! 1. world 资源 `AssetSsboLibrary`（缺失 → `Option::unwrap`）；`FrozenMap::get(SsboId)` 取 `LanternSsbo` 的实例缓冲
//!    （未注册 → `Option::unwrap`；类型名核对 `"country_core::resources::render::lantern::LanternSsbo"`（0x35 字节）不符 → `assert_failed`；
//!    `downcast` 失败 → `Result::unwrap`）。`lantern_count = instance_count`。
//! 2. `lantern_count == 0`：**不声明任何 pass**，只建占位纹理 `"RDI reservoirs placeholder"`（名长 0x1a）：`new_2d((1,1), 0x65)`，返回它。
//! 3. 否则：`get_or_create_temporal_pair("RDI reservoirs")`（名长 0xe）：`new_2d(extent_2d(half_depth.desc), 0x65).sampled().storage()`
//!    （`Err` → `Result::unwrap` panic）。
//!    **"RDI temporal"** `/rdi/temporal.cs.hlsl`（名长 0xc、URI 长 0x15）：`blue_noise_texture`（磁盘）→ `reprojection_tex`、`half_depth_tex`、
//!    `half_view_normal_tex`、`rdi_temporal_history_tex`（读 0x19）→ `read_storage_buffer<LanternSsbo>(lantern_buf)` →
//!    `read_rg_storage_buffer(light_indirection_grid, grid[0])`、`read_rg_storage_buffer(grid_light_indices, grid[1])` →
//!    `rdi_temporal_out_tex`（写 0x32）→ uniform i32 `lantern_count` → uniform bool `history_valid = flag & 1` → `dispatch_2d(half)`。
//! 4. 返回本帧蓄水池句柄。
//!
//! `LanternSsbo`（`resources::render::lantern`，别的模块）作泛型参数；SsboId 立即数 0x8b2da1ef1bff9619 = xxh3_64("LanternSsbo")。

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::pass_builder::compute_pass_builder::{RecordedComputePass, RgBuffer};
use crate::render::pass_builder::pass_resource_access::{RgTexture, WriteTextureUnchecked};
use crate::rhapsody_types::TextureDesc;
use crate::systems::render_loop::compute_passes::pass_kit::{
    BLUE_NOISE, PassEnv, binding, binding_hash, extent_of,
};

pub const PLACEHOLDER_NAME: &str = "RDI reservoirs placeholder";
pub const RESERVOIRS_NAME: &str = "RDI reservoirs";
pub const PASS_NAME: &str = "RDI temporal";
pub const URI: &str = "/rdi/temporal.cs.hlsl";
pub const RESERVOIR_FORMAT: u32 = 0x65;
pub const LANTERN_BUF: u64 = binding_hash("lantern_buf");
pub const LIGHT_INDIRECTION_GRID: u64 = binding_hash("light_indirection_grid");
pub const GRID_LIGHT_INDICES: u64 = binding_hash("grid_light_indices");
pub const LANTERN_COUNT: u64 = binding_hash("lantern_count");
pub const RDI_TEMPORAL_HISTORY_TEX: u64 = binding_hash("rdi_temporal_history_tex");
pub const RDI_TEMPORAL_OUT_TEX: u64 = binding_hash("rdi_temporal_out_tex");
/// 原版核对的类型名（O：0x35 字节）。
pub const LANTERN_TYPE_NAME: &str = "country_core::resources::render::lantern::LanternSsbo";

/// 光照格网的两个缓冲（O：`+0x00`、`+0x18`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LightGrid {
    pub light_indirection_grid: RgBuffer,
    pub grid_light_indices: RgBuffer,
}

pub struct RestirDi {
    pub reservoir: RgTexture,
    /// `lantern_count == 0` 时为 `None`（只返回占位纹理）。
    pub pass: Option<RecordedComputePass>,
}

/// W41：ReSTIR 直接光照时间复用。`lantern_count` = `InstanceBuffer<LanternSsbo>::instance_count`。
pub fn restir_sample_direct_lights<Lantern: SsboType>(
    env: &mut PassEnv<'_>,
    half_depth: &RgTexture,
    half_normal: &RgTexture,
    reprojection: &RgTexture,
    grid: &LightGrid,
    lantern_count: u32,
) -> RestirDi {
    if lantern_count == 0 {
        let reservoir = env.create_texture(
            PLACEHOLDER_NAME,
            TextureDesc::new_2d([1, 1], RESERVOIR_FORMAT),
        );
        return RestirDi {
            reservoir,
            pass: None,
        };
    }
    let half = extent_of(half_depth);
    let pair = env.temporal_pair(
        RESERVOIRS_NAME,
        TextureDesc::new_2d([half.width, half.height], RESERVOIR_FORMAT)
            .storage()
            .sampled(),
    );
    let pass = env
        .pass(PASS_NAME, URI)
        .bind_texture(binding::BLUE_NOISE_TEXTURE, &BLUE_NOISE, 0x19)
        .bind_texture(binding::REPROJECTION_TEX, &reprojection, 0x19)
        .bind_texture(binding::HALF_DEPTH_TEX, &half_depth, 0x19)
        .bind_texture(binding::HALF_VIEW_NORMAL_TEX, &half_normal, 0x19)
        .bind_texture(RDI_TEMPORAL_HISTORY_TEX, &&pair.previous, 0x19)
        .read_storage_buffer::<Lantern>(LANTERN_BUF)
        .read_rg_storage_buffer(LIGHT_INDIRECTION_GRID, &grid.light_indirection_grid)
        .read_rg_storage_buffer(GRID_LIGHT_INDICES, &grid.grid_light_indices)
        .bind_texture(
            RDI_TEMPORAL_OUT_TEX,
            &WriteTextureUnchecked(pair.current),
            0x32,
        )
        .uniform(LANTERN_COUNT, lantern_count as i32)
        .uniform(binding::HISTORY_VALID, pair.fresh)
        .dispatch_2d(half.width, half.height);
    RestirDi {
        reservoir: pair.current,
        pass: Some(pass),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        Env, accesses, env_with, rgtex, test_carrier, threads,
    };

    test_carrier!(Lantern, "LanternSsbo");

    fn names() -> Vec<u64> {
        vec![
            binding::BLUE_NOISE_TEXTURE,
            binding::REPROJECTION_TEX,
            binding::HALF_DEPTH_TEX,
            binding::HALF_VIEW_NORMAL_TEX,
            RDI_TEMPORAL_HISTORY_TEX,
            LANTERN_BUF,
            LIGHT_INDIRECTION_GRID,
            GRID_LIGHT_INDICES,
            RDI_TEMPORAL_OUT_TEX,
        ]
    }

    fn run(count: u32, valid: bool) -> (Env, RestirDi) {
        let mut e = env_with(
            &names(),
            &[LANTERN_COUNT, binding::HISTORY_VALID],
            |lib, dev| {
                lib.add_new::<Lantern>(dev);
            },
        );
        e.frame.pair_fresh = valid;
        let grid = LightGrid {
            light_indirection_grid: RgBuffer { resource: 50 },
            grid_light_indices: RgBuffer { resource: 51 },
        };
        let out = e.run(|env| {
            restir_sample_direct_lights::<Lantern>(
                env,
                &rgtex(1, 400, 200),
                &rgtex(2, 400, 200),
                &rgtex(3, 800, 400),
                &grid,
                count,
            )
        });
        (e, out)
    }

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(LANTERN_BUF, 0x0c6b_33eb_2099_2aad);
        assert_eq!(LIGHT_INDIRECTION_GRID, 0x2bd0_ac28_9087_c8e1);
        assert_eq!(GRID_LIGHT_INDICES, 0x129f_01ce_8d54_06a0);
        assert_eq!(LANTERN_COUNT, 0x9b88_329e_408f_f69f);
        assert_eq!(RDI_TEMPORAL_HISTORY_TEX, 0x4da9_887c_f3dc_26f4);
        assert_eq!(RDI_TEMPORAL_OUT_TEX, 0x8dae_eefe_7463_dee0);
        assert_eq!(LANTERN_TYPE_NAME.len(), 0x35);
    }

    #[test]
    fn no_lanterns_returns_a_1x1_placeholder_and_no_pass() {
        let (e, out) = run(0, true);
        assert!(out.pass.is_none());
        assert!(e.world.backend.paths.borrow().is_empty());
        assert!(e.frame.temporal_pairs.is_empty());
        let (n, s, t) = &e.frame.textures[0];
        assert_eq!(
            (n.as_str(), s.extent_2d(), s.format),
            (PLACEHOLDER_NAME, [1, 1], 0x65)
        );
        assert_eq!(out.reservoir, *t);
    }

    #[test]
    fn with_lanterns_declares_one_pass_at_half_res() {
        let (e, out) = run(7, true);
        let p = out.pass.as_ref().unwrap();
        assert_eq!(p.name, PASS_NAME);
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec![format!("{URI}:cs_main")]
        );
        assert_eq!(threads(p), (400, 200, 1));
        let (n, s) = &e.frame.temporal_pairs[0];
        assert_eq!(
            (n.as_str(), s.extent_2d(), s.format, s.storage, s.sampled),
            (RESERVOIRS_NAME, [400, 200], 0x65, true, true)
        );
        assert_eq!(e.uniform_u32(p, LANTERN_COUNT), 7);
        assert_eq!(e.uniform_u32(p, binding::HISTORY_VALID), 1);
        let a = accesses(p);
        // reprojection, half_depth, half_normal, history, lantern, grid0, grid1, out
        // （blue_noise 是磁盘纹理：外部描述符，不登记 RG 访问）
        assert_eq!(a.len(), 8);
        assert_eq!(p.external_bindings.len(), 1);
        assert_eq!((a[5], a[6]), ((50, 0x1a, 1), (51, 0x1a, 1)));
        assert_eq!((a[7].0, a[7].1), (out.reservoir.resource, 0x32));
    }

    #[test]
    fn history_valid_is_the_pair_flag() {
        let (e, out) = run(1, false);
        assert_eq!(
            e.uniform_u32(out.pass.as_ref().unwrap(), binding::HISTORY_VALID),
            0
        );
    }
}
