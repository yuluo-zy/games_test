//! common_includes_compute.rs — `common_includes` 里 `ComputePass` 实例的两个助手（拆出独立文件只是为了让
//! `DrawBuilder` 一侧的单元测试不必拖入 compute 全套；PDB 里它们同属 `draw_shaders::common_includes`）。
//!
//! - `impl$2::bind_water_object_push<ComputePass>` 0x14160e330（767 B）：纹理预设 0xf → `read_access_buffer<RippleSourceSsbo>`
//!   （访问 0x1a）→ 预设 0x18；纹理访问码 0x19。
//! - `impl$3::bind_garden_exclusion<ComputePass>` 0x14160e630（538 B）：预设 0xe / 0x13 / 5 / 0x10，访问码 0x19。
//!   四个名字哈希见 [`GARDEN_EXCLUSION`]。
//!
//! B 阶段 i_infra：这里是 ComputePass 版 `bind_*` 的**唯一**实现（原先 `c2_support.rs`、`spawn_vegetation_around_buildings.rs` 各有一份
//! 同形副本，已删；`compute_passes` 的调用方统一 `use` 本文件）。

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::compute_pass_builder::ComputePass;
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    GARDEN_EXCLUSION, WATER_OBJECT_PUSH_NAME_A, WATER_OBJECT_PUSH_NAME_B,
    WATER_OBJECT_PUSH_PRESET_A, WATER_OBJECT_PUSH_PRESET_B, WATER_OBJECT_PUSH_RIPPLE_NAME,
};

/// ComputePass 里 `bind_texture_to_compute_pass` 的读访问码（O：`mov r9b, 0x19`）。
pub const COMPUTE_TEXTURE_READ: u8 = 0x19;

/// `bind_water_object_push<ComputePass>`（O 0x14160e330）。`R` = `RippleSourceSsbo`（库侧 SSBO 类型）。
pub fn bind_water_object_push_compute<R: LibrarySsbo>(pass: ComputePass) -> ComputePass {
    pass.bind_texture(
        WATER_OBJECT_PUSH_NAME_A,
        &TexturePreset(WATER_OBJECT_PUSH_PRESET_A),
        COMPUTE_TEXTURE_READ,
    )
    .read_storage_buffer::<R>(WATER_OBJECT_PUSH_RIPPLE_NAME)
    .bind_texture(
        WATER_OBJECT_PUSH_NAME_B,
        &TexturePreset(WATER_OBJECT_PUSH_PRESET_B),
        COMPUTE_TEXTURE_READ,
    )
}

/// `bind_garden_exclusion<ComputePass>`（O 0x14160e630）。
pub fn bind_garden_exclusion_compute(mut pass: ComputePass) -> ComputePass {
    for (preset, name) in GARDEN_EXCLUSION {
        pass = pass.bind_texture(name, &TexturePreset(preset), COMPUTE_TEXTURE_READ);
    }
    pass
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        accesses, env, env_with,
    };
    use crate::systems::render_loop::compute_passes::pass_kit::{RippleSourceBuf, binding_hash};

    /// 绑定名哈希 = HLSL 名字的 `BindingNameHash`（O：movabs 立即数；与 DrawBuilder 版共用同一组常量）。
    #[test]
    fn constants_are_binding_name_hashes_of_the_hlsl_names() {
        assert_eq!(WATER_OBJECT_PUSH_NAME_A, binding_hash("grass_object_push"));
        assert_eq!(
            WATER_OBJECT_PUSH_RIPPLE_NAME,
            binding_hash("ripple_source_buffer")
        );
        assert_eq!(
            WATER_OBJECT_PUSH_NAME_B,
            binding_hash("water_noise_texture")
        );
        let names = [
            "rocky_terrain",
            "water_mask",
            "path_mask",
            "foliage_exclusion_mask",
        ];
        for ((_, h), n) in GARDEN_EXCLUSION.iter().zip(names) {
            assert_eq!(*h, binding_hash(n), "{n}");
        }
        assert_eq!(GARDEN_EXCLUSION.map(|(p, _)| p), [0xe, 0x13, 5, 0x10]);
    }

    /// O 0x14160e630：四个预设纹理按顺序绑定，访问码都是 0x19，不登记 SSBO。
    #[test]
    fn garden_exclusion_binds_four_presets_read_only() {
        let names: Vec<u64> = GARDEN_EXCLUSION.iter().map(|(_, h)| *h).collect();
        let mut e = env(&names, &[]);
        let pass =
            e.run(|env| bind_garden_exclusion_compute(env.pass("t", "/t.cs.hlsl")).dispatch_1d(1));
        let acc = accesses(&pass);
        assert_eq!(acc.len(), 4);
        assert!(acc.iter().all(|a| a.1 == COMPUTE_TEXTURE_READ));
        assert_eq!(
            pass.bindings.len(),
            1 + 4,
            "ctor 的 color library + 4 个预设纹理"
        );
    }

    /// O 0x14160e330：预设 0xf（0x19）→ `read_access_buffer<RippleSourceSsbo>`（0x1a）→ 预设 0x18（0x19）。
    #[test]
    fn water_object_push_is_texture_ripple_buffer_texture() {
        let mut e = env_with(
            &[
                WATER_OBJECT_PUSH_NAME_A,
                WATER_OBJECT_PUSH_RIPPLE_NAME,
                WATER_OBJECT_PUSH_NAME_B,
            ],
            &[],
            |lib, dev| lib.add_new::<RippleSourceBuf>(dev),
        );
        let pass = e.run(|env| {
            bind_water_object_push_compute::<RippleSourceBuf>(env.pass("t", "/t.cs.hlsl"))
                .dispatch_1d(1)
        });
        let acc: Vec<u8> = accesses(&pass).iter().map(|a| a.1).collect();
        assert_eq!(acc, [COMPUTE_TEXTURE_READ, 0x1a, COMPUTE_TEXTURE_READ]);
    }
}
