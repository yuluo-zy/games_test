//! instanced_roof.rs — 屋顶瓦 / 尖顶 / 支撑梁的实例绘制。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/instanced_roof.rs`（上一版本目录里没有这个文件，PDB 里有 3 个函数）。
//! 机器码（O）：`draw_instanced_roof_shader_nani` 0x1409d9070（1579 B）、`draw_instanced_roof_spire_shader` 0x1409d96a0
//! （498 B）、`draw_instanced_roof_support_beam_shader` 0x1409d98a0（788 B）。ABI：`rcx = &mut DrawBuilder`、`r8 = &World`。
//!
//! ## 屋顶瓦（nani）
//! 世界资源（TypeId 0x9311cf2688353878/0xfd54118f9a9ee3d4）：`u32 @+0`、`u8 @+8`（形如 `Option<u32>`：标签在 +8）。
//! `tag = byte @+8`；`value = tag != 0 ? u32 @+0 : 0`。
//! 0. `packet.state` 第 0 字节 `(b & 0xf3) | 4`
//! 1. 预设 9 / 10
//! 2. `read_storage_buffer<DormerHoleSsbo>`（槽 "Dormer Holes"，名 0x799f9d8c5d59d1d2）
//! 3. `read_storage_buffer<DormerHoleCountPerCellSsbo>`（槽 "Dormer Hole Count"，名 0x6e980b4acba4b95d）
//! 4. 磁盘 0x7db9e7483fa24078（名 0xaa8f0490dba0d03c；与 `draw_bricks` 同一文件）
//! 5. `read_storage_buffer<RoofColorIdSsbo>`（槽 "RoofColorIds"，名 0xd56729333fc94f92）
//! 6. 磁盘 0xe09955c0b15b3e04（名 0x82cb46c505c25a1f）、0xb35c3f60a2b76288（名 0xf28e4e718e1b468c）、
//!    0xa169bb74b48855e1（名 0x862435346213c22e）
//! 7. `read_storage_buffer<RoofTileSsbo>`（槽 "RoofTileSsbo"，名 0xc3483cd70105de2f）
//! 8. 常量块存在 → `write_uniform<i32>(0x5add70b0954f689d, value)`；`write_uniform<i32>(0x052e7aa4e36b047b, tag)`
//! 9. `nani_draw(RoofTile = 8)`
//!
//! ## 尖顶 / 支撑梁
//! 先查库里该 SSBO 的实例缓冲取实例数（缺省 0）；尖顶：`read_storage_buffer<RoofSpireSsbo>`（槽 "RoofSpearTransforms"，
//! 名 0xc3483cd70105de2f）→ `draw_instanced_w_gl_draw_flag(count)`；支撑梁：预设 9 / 10 →
//! `read_storage_buffer<RoofSupportBeamSsbo>`（槽 "RoofSupportBeams"，同名 0xc3483cd70105de2f）→ 同样绘制。

use crate::render::nani::gpu_world::GpuWorldSubsetKind;
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    bind_disk, bind_terrain_heightmap, bind_terrain_lighting,
};
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    DormerHoleCountTag, DormerHoleTag, ReadSsboExt, RoofColorIdTag, RoofSpireTag,
    RoofSupportBeamTag, RoofTileTag, SsboInstanceView, instance_count,
};

pub const DORMER_HOLE_BINDING: u64 = 0x799f_9d8c_5d59_d1d2;
pub const DORMER_COUNT_BINDING: u64 = 0x6e98_0b4a_cba4_b95d;
pub const ROOF_BRICK_TEXTURE: (u64, u64) = (0x7db9_e748_3fa2_4078, 0xaa8f_0490_dba0_d03c);
pub const ROOF_COLOR_BINDING: u64 = 0xd567_2933_3fc9_4f92;
/// 屋顶瓦的三张磁盘纹理（O：文件键, 名字哈希）。
pub const ROOF_TEXTURES: [(u64, u64); 3] = [
    (0xe099_55c0_b15b_3e04, 0x82cb_46c5_05c2_5a1f),
    (0xb35c_3f60_a2b7_6288, 0xf28e_4e71_8e1b_468c),
    (0xa169_bb74_b488_55e1, 0x8624_3534_6213_c22e),
];
/// 瓦 / 尖顶 / 支撑梁三者共用的 SSBO 绑定名。
pub const ROOF_TILE_BINDING: u64 = 0xc348_3cd7_0105_de2f;
pub const ROOF_VALUE_UNIFORM: u64 = 0x5add_70b0_954f_689d;
pub const ROOF_TAG_UNIFORM: u64 = 0x052e_7aa4_e36b_047b;

/// 世界资源 `Option<u32>`-形（`u32 @+0`、`tag @+8`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoofNaniParam {
    pub value: u32,
    pub tag: u8,
}

impl RoofNaniParam {
    /// O：`edi = tag != 0 ? [rax] : 0`；`esi = tag`。
    pub fn uniforms(&self) -> (i32, i32) {
        (
            if self.tag != 0 { self.value as i32 } else { 0 },
            self.tag as i32,
        )
    }
}

/// `packet.state` 第 0 字节 `(b & 0xf3) | 4`。
pub fn roof_state(state: u32) -> u32 {
    (state & !0x0c) | 0x04
}

/// `draw_instanced_roof_shader_nani`（O 0x1409d9070）。
pub fn draw_instanced_roof_shader_nani<'p>(
    mut b: DrawBuilder<'p>,
    param: RoofNaniParam,
    nani: impl FnOnce(DrawBuilder<'p>, GpuWorldSubsetKind),
) {
    let (value, tag) = param.uniforms();
    let p = b.packet_mut();
    p.state.0 = roof_state(p.state.0);
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = b.read_ssbo::<DormerHoleTag>(DORMER_HOLE_BINDING);
    let b = b.read_ssbo::<DormerHoleCountTag>(DORMER_COUNT_BINDING);
    let mut b = bind_disk(b, ROOF_BRICK_TEXTURE.0, ROOF_BRICK_TEXTURE.1);
    b = b.read_ssbo::<RoofColorIdTag>(ROOF_COLOR_BINDING);
    for (key, name) in ROOF_TEXTURES {
        b = bind_disk(b, key, name);
    }
    let b = b.read_ssbo::<RoofTileTag>(ROOF_TILE_BINDING);
    let b = b.uniform(ROOF_VALUE_UNIFORM, value);
    let b = b.uniform(ROOF_TAG_UNIFORM, tag);
    nani(b, GpuWorldSubsetKind::RoofTile);
}

/// `draw_instanced_roof_spire_shader`（O 0x1409d96a0）。
pub fn draw_instanced_roof_spire_shader(b: DrawBuilder<'_>, lib: &dyn SsboInstanceView) {
    let count = instance_count::<RoofSpireTag>(lib);
    let b = b.read_ssbo::<RoofSpireTag>(ROOF_TILE_BINDING);
    b.draw_instanced_w_gl_draw_flag(count as u32);
}

/// `draw_instanced_roof_support_beam_shader`（O 0x1409d98a0）。
pub fn draw_instanced_roof_support_beam_shader(b: DrawBuilder<'_>, lib: &dyn SsboInstanceView) {
    let count = instance_count::<RoofSupportBeamTag>(lib);
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = b.read_ssbo::<RoofSupportBeamTag>(ROOF_TILE_BINDING);
    b.draw_instanced_w_gl_draw_flag(count as u32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::rhapsody_contract::ManagedHandle;
    use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
        TERRAIN_HEIGHTMAP_NAME, TERRAIN_LIGHTING_NAME,
    };
    use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
        DrawSsbo, InstanceBufferView,
    };
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{
        Call, Rig, as_i32, rg, uniform_at,
    };

    struct Lib(Vec<(u64, &'static str, Option<InstanceBufferView>)>);
    impl SsboInstanceView for Lib {
        fn lookup(&self, id: u64) -> Option<(&str, Option<InstanceBufferView>)> {
            self.0
                .iter()
                .find(|e| e.0 == id)
                .map(|e| (e.1, e.2.clone()))
        }
    }
    fn lib_with<T: DrawSsbo>(n: u64) -> Lib {
        Lib(vec![(
            T::rg_buffer_hash(),
            T::TYPE_NAME,
            Some(InstanceBufferView {
                instance_count: n,
                handle: ManagedHandle(1),
            }),
        )])
    }

    #[test]
    fn nani_param_uniforms_follow_the_tag() {
        assert_eq!(RoofNaniParam { value: 7, tag: 1 }.uniforms(), (7, 1));
        assert_eq!(
            RoofNaniParam { value: 7, tag: 0 }.uniforms(),
            (0, 0),
            "tag == 0 时值被置 0"
        );
    }

    #[test]
    fn nani_roof_order_uniforms_and_kind() {
        let mut names = vec![
            TERRAIN_HEIGHTMAP_NAME,
            TERRAIN_LIGHTING_NAME,
            DORMER_HOLE_BINDING,
            DORMER_COUNT_BINDING,
            ROOF_BRICK_TEXTURE.1,
            ROOF_COLOR_BINDING,
            ROOF_TILE_BINDING,
        ];
        names.extend(ROOF_TEXTURES.iter().map(|t| t.1));
        let mut rig = Rig::new(&names, &[ROOF_VALUE_UNIFORM, ROOF_TAG_UNIFORM]);
        let mut kind = None;
        let mut data = Vec::new();
        draw_instanced_roof_shader_nani(
            rig.builder(),
            RoofNaniParam { value: 9, tag: 1 },
            |b, k| {
                kind = Some(k);
                data = b.constants_data().unwrap().to_vec();
                b.draw();
            },
        );
        assert_eq!(kind, Some(GpuWorldSubsetKind::RoofTile));
        assert_eq!(GpuWorldSubsetKind::RoofTile as u8, 8);
        assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 9);
        assert_eq!(as_i32(&uniform_at(&data, 1, 4)), 1);
        assert_eq!(
            rig.calls(),
            vec![
                Call::Preset(9),
                Call::Preset(10),
                rg::<DormerHoleTag>(),
                rg::<DormerHoleCountTag>(),
                Call::Disk(0x7db9e7483fa24078),
                rg::<RoofColorIdTag>(),
                Call::Disk(0xe09955c0b15b3e04),
                Call::Disk(0xb35c3f60a2b76288),
                Call::Disk(0xa169bb74b48855e1),
                rg::<RoofTileTag>(),
            ]
        );
        let d = rig.finish();
        assert_eq!(d[0].state.0 & 0x0c, 0x04);
    }

    #[test]
    fn spire_and_beam_draw_the_library_instance_count() {
        let mut rig = Rig::new(
            &[
                ROOF_TILE_BINDING,
                TERRAIN_HEIGHTMAP_NAME,
                TERRAIN_LIGHTING_NAME,
            ],
            &[],
        );
        draw_instanced_roof_spire_shader(rig.builder(), &lib_with::<RoofSpireTag>(33));
        assert_eq!(rig.calls(), vec![rg::<RoofSpireTag>()]);
        assert_eq!(
            rig.finish()[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 33 }
        );

        let mut rig = Rig::new(
            &[
                ROOF_TILE_BINDING,
                TERRAIN_HEIGHTMAP_NAME,
                TERRAIN_LIGHTING_NAME,
            ],
            &[],
        );
        draw_instanced_roof_support_beam_shader(rig.builder(), &lib_with::<RoofSupportBeamTag>(5));
        assert_eq!(
            rig.calls(),
            vec![
                Call::Preset(9),
                Call::Preset(10),
                rg::<RoofSupportBeamTag>()
            ]
        );
        assert_eq!(
            rig.finish()[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 5 }
        );
    }

    #[test]
    fn no_registered_buffer_draws_zero() {
        let mut rig = Rig::new(&[], &[]);
        draw_instanced_roof_spire_shader(rig.builder(), &Lib(vec![]));
        assert_eq!(
            rig.finish()[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 0 }
        );
    }

    #[test]
    fn the_three_roof_shaders_share_one_ssbo_binding_name() {
        assert_eq!(
            ROOF_TILE_BINDING, 0xc3483cd70105de2f,
            "= -0x3cb7c328fefa21d1"
        );
        assert_eq!(RoofSpireTag::SLOT_NAME, "RoofSpearTransforms");
        assert_eq!(RoofSupportBeamTag::SLOT_NAME, "RoofSupportBeams");
    }
}
