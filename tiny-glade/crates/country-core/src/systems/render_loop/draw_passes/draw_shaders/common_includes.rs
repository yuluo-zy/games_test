//! common_includes.rs — 跨 pass 共享的 `bind_*` 助手（`DrawBuilder` / `ComputePass` 两套）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/common_includes.rs`。机器码（O）：
//! - `impl$2::bind_water_object_push<DrawBuilder>` 0x141631f90（314 B）：`TexturePreset(0xf)`（名 0x035280b96dab13e6）
//!   → `read_storage_buffer<RippleSourceSsbo>`（名 0x68197c4966ba52c0）→ `TexturePreset(0x18)`（名 0xfb32aff2a6359615），
//!   纹理访问码均 0x1d。
//! - `impl$2::bind_water_object_push<ComputePass>` 0x14160e330（767 B）：同序，纹理访问码 0x19、SSBO 用
//!   `read_access_buffer<RippleSourceSsbo>`（访问码 0x1a）。
//! - `impl$3::bind_garden_exclusion<ComputePass>` 0x14160e630（538 B）：`TexturePreset(0xe/0x13/5/0x10)`，
//!   名 0x4ee098b38612d2b9 / 0x6ad63ccd18c8f041 / 0xed502fff6dc4021f / 0x0d7eab55ca05e34c，访问码 0x19。
//! - `impl$0::bind_terrain_heightmap`（line 13）与 `impl$1::bind_terrain_lighting`（line 30）没有独立过程（内联进各
//!   pass，PDB 只留名字）。**I**：几乎每个 draw pass 都以 `TexturePreset(9)`（名 0x61b08b5e0470c5f2）→
//!   `TexturePreset(10)`（名 0x297d1e4cdae8f657）开头，顺序与这两个方法的声明顺序一致，故按此建模；
//!   这两个预设的真实名字 U。
//!
//! `water_object_push.inc.hlsl` 被 `_lilypad` / `_reed` 等水植被 raster 嵌入（见 `water_planes_draw.rs`）。

use crate::render::pass_builder::draw_builder::{ACCESS_SAMPLED_IMAGE, BindTexture, DrawBuilder};
use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    ReadSsboExt, RippleSourceTag,
};

// ---- 绑定名哈希（O：movabs 立即数）----
pub const TERRAIN_HEIGHTMAP_PRESET: u8 = 9;
pub const TERRAIN_HEIGHTMAP_NAME: u64 = 0x61b0_8b5e_0470_c5f2;
pub const TERRAIN_LIGHTING_PRESET: u8 = 10;
pub const TERRAIN_LIGHTING_NAME: u64 = 0x297d_1e4c_dae8_f657;

pub const WATER_OBJECT_PUSH_PRESET_A: u8 = 0xf;
pub const WATER_OBJECT_PUSH_NAME_A: u64 = 0x0352_80b9_6dab_13e6;
pub const WATER_OBJECT_PUSH_RIPPLE_NAME: u64 = 0x6819_7c49_66ba_52c0;
pub const WATER_OBJECT_PUSH_PRESET_B: u8 = 0x18;
pub const WATER_OBJECT_PUSH_NAME_B: u64 = 0xfb32_aff2_a635_9615;

/// `bind_garden_exclusion` 的 (预设, 名字哈希)，按调用顺序（O）。
pub const GARDEN_EXCLUSION: [(u8, u64); 4] = [
    (0xe, 0x4ee0_98b3_8612_d2b9),
    (0x13, 0x6ad6_3ccd_18c8_f041),
    (0x5, 0xed50_2fff_6dc4_021f),
    (0x10, 0x0d7e_ab55_ca05_e34c),
];

/// `+0xf0` 字节（`bind_common` 写入 `CommonDrawParams +4`，`DrawBuilder::new` 置 0）。
///
/// **与 A4 的出入**：A4 把它建模成 `bool`（`common_flag`），但 `indirect_draw_bricks` 里
/// `cmp byte ptr [rcx+0xf0], 2; jae`、`0x1c >> v`、`0x201000000 >> (8*v)` 说明它是 0..=4 的小枚举。A4 把它截成 `bool`
/// 之前，这里只能取到 0/1；`bricks` 的判定表（见 `draw_bricks::plan`）已按完整字节写好并单独测试。
pub trait DrawModeExt {
    fn draw_mode(&self) -> u8;
}

impl DrawModeExt for DrawBuilder<'_> {
    fn draw_mode(&self) -> u8 {
        self.common_flag() as u8
    }
}

/// `bind_texture_to_draw_builder<TexturePreset>`：访问码 0x1d、无 view——所有 draw 绑定点的统一形态。
pub fn bind_preset<'p>(b: DrawBuilder<'p>, preset: u8, name_hash: u64) -> DrawBuilder<'p> {
    b.bind_texture_to_draw_builder(
        BindTexture::Preset(TexturePreset(preset)),
        name_hash,
        ACCESS_SAMPLED_IMAGE,
        None,
    )
}

/// `bind_texture_to_draw_builder<TextureFromDisk>`：访问码 0x1d、无 view。
pub fn bind_disk<'p>(b: DrawBuilder<'p>, file_key: u64, name_hash: u64) -> DrawBuilder<'p> {
    b.bind_texture_to_draw_builder(
        BindTexture::FromDisk(TextureFromDisk(file_key)),
        name_hash,
        ACCESS_SAMPLED_IMAGE,
        None,
    )
}

/// `bind_terrain_heightmap`（impl$0，I：预设 9）。
pub fn bind_terrain_heightmap<'p>(b: DrawBuilder<'p>) -> DrawBuilder<'p> {
    bind_preset(b, TERRAIN_HEIGHTMAP_PRESET, TERRAIN_HEIGHTMAP_NAME)
}

/// `bind_terrain_lighting`（impl$1，I：预设 10）。
pub fn bind_terrain_lighting<'p>(b: DrawBuilder<'p>) -> DrawBuilder<'p> {
    bind_preset(b, TERRAIN_LIGHTING_PRESET, TERRAIN_LIGHTING_NAME)
}

/// `bind_water_object_push<DrawBuilder>`（O 0x141631f90）。
pub fn bind_water_object_push<'p>(b: DrawBuilder<'p>) -> DrawBuilder<'p> {
    let b = bind_preset(b, WATER_OBJECT_PUSH_PRESET_A, WATER_OBJECT_PUSH_NAME_A);
    let b = b.read_ssbo::<RippleSourceTag>(WATER_OBJECT_PUSH_RIPPLE_NAME);
    bind_preset(b, WATER_OBJECT_PUSH_PRESET_B, WATER_OBJECT_PUSH_NAME_B)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{Call, Rig, rg};

    fn names() -> Vec<u64> {
        vec![
            TERRAIN_HEIGHTMAP_NAME,
            TERRAIN_LIGHTING_NAME,
            WATER_OBJECT_PUSH_NAME_A,
            WATER_OBJECT_PUSH_RIPPLE_NAME,
            WATER_OBJECT_PUSH_NAME_B,
        ]
    }

    #[test]
    fn water_object_push_binds_two_presets_around_the_ripple_buffer() {
        let mut rig = Rig::new(&names(), &[]);
        let b = bind_water_object_push(rig.builder());
        assert_eq!(
            b.packet().accesses.len(),
            3,
            "preset 0xf、RippleSource、preset 0x18 都命中反射"
        );
        // 反射序号：A=2、ripple=3、B=4，且 packet 里的顺序就是调用顺序。
        let order: Vec<u32> = b
            .packet()
            .accesses
            .iter()
            .map(|a| a.binding_index)
            .collect();
        assert_eq!(order, vec![2, 3, 4]);
        b.draw();
        assert_eq!(
            rig.calls(),
            vec![
                Call::Preset(0xf),
                rg::<RippleSourceTag>(),
                Call::Preset(0x18)
            ]
        );
        rig.finish();
    }

    #[test]
    fn terrain_helpers_use_presets_9_and_10_in_that_order() {
        let mut rig = Rig::new(&names(), &[]);
        let b = bind_terrain_lighting(bind_terrain_heightmap(rig.builder()));
        let order: Vec<u32> = b
            .packet()
            .accesses
            .iter()
            .map(|a| a.binding_index)
            .collect();
        assert_eq!(order, vec![0, 1]);
        b.draw();
        assert_eq!(rig.calls(), vec![Call::Preset(9), Call::Preset(10)]);
        rig.finish();
    }

    #[test]
    fn disk_helper_goes_through_get_by_name_with_the_sampled_image_access() {
        let mut rig = Rig::new(&[0x77], &[]);
        let b = bind_disk(rig.builder(), 0xabcdef, 0x77);
        b.draw();
        assert_eq!(rig.calls(), vec![Call::Disk(0xabcdef)]);
        let d = rig.finish();
        assert_eq!(
            d[0].bindings.len(),
            1,
            "磁盘纹理走原生描述符绑定（{{0, view, 5}}）"
        );
    }

    #[test]
    fn observed_constants() {
        assert_eq!(GARDEN_EXCLUSION.map(|(p, _)| p), [0xe, 0x13, 5, 0x10]);
        assert_eq!(
            GARDEN_EXCLUSION[2].1, 0xed502fff6dc4021f,
            "= -0x12afd000923bfde1"
        );
        assert_eq!(WATER_OBJECT_PUSH_NAME_B, 0xfb32aff2a6359615);
    }
}
