//! water_planes_draw.rs — 水面网格与水生植被 draw（`_water_plane` / `_lilypad` / `_lilypad_flower` / `_reed`）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs`。机器码（O）：
//! `water_planes_draw` 0x1409d9bc0（2487 B）、`lilypad_draw` 0x1409da580、`lilypad_flower_draw` 0x1409da760、
//! `reed_draw` 0x1409da940。
//!
//! ## water_planes_draw
//! ABI：`rcx = &mut DrawBuilder`、`r8 = &World`。
//! 0. 进程级布尔（`.data` 0x143634d90）非 0 → `discard`、返回（O；原文档称 demo 守卫，布尔的来源 U）
//! 1. 世界资源 `ActiveSession`（TypeId 0x74430dfcc6d8719b/0xca5591f7e464509c）取两次：`variant = byte @+0x58`；
//!    `+0x08` 指向的字节序列（长度 `+0x10`）若**没有任何 0 字节**（`iter().all(|b| b != 0)`）→ `all_set = true`
//! 2. `packet.state` 第 3 字节 `&= 0x9f`
//! 3. 绑定序（名字哈希见常量）：预设 9 / 10 / 0x13 → `RippleSourceSsbo` → 预设 0x18 → 磁盘 0x579f39ab91a01865 →
//!    `DofAutofocusSsbo` → 预设 9 / 10 / 8 → 磁盘 0x579f39ab91a01865（同一个名字再绑一次）
//! 4. 常量块存在 → `write_uniform<Vec3>(0x6f7108765d902970, water_color(all_set))`
//! 5. `bind_ray_tracing`（`bind_resources` 0x1416289d0；`DrawBuilder::bind_ray_tracing::<Rt…>()`）
//! 6. `variant == 4`（冬季，I）→ 磁盘 0xc555853f6bd247ac（名 0x706067c859557777）
//! 7. `draw()`
//!
//! ## 水生植被（三个函数同形，只差 SSBO、磁盘纹理与网格下标）
//! `read_storage_buffer<T>`（名 0x6d9256d4ac7887ad）→ 预设 9 / 10 → `bind_water_object_push` → 磁盘纹理 →
//! `multi_draw_indexed_indirect(idx, arg_50 = 1, arg_58 = 0)`：lilypad idx 8（磁盘 0x4d64746dbda554d2）、
//! lilypad_flower idx 0xf（0x9bbd5c16bda1314c）、reed idx 9（无磁盘纹理）。
//!
//! 注意：`multi_draw_indexed_indirect` 的静态表（`draw_builder::INDEXED_INDIRECT_INDEX_COUNTS`）A4 只核对了前 10 项，
//! lilypad_flower 的 idx 0xf 超出该表，见 `B_d_shaders.md`。

use crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer;
use glam::Vec3;

use crate::render::pass_builder::bind_resources::{
    RtBvhSsbo, RtTriangleMetaSsbo, RtTrianglesSsbo, WallColorIdSsbo,
};
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    bind_disk, bind_preset, bind_terrain_heightmap, bind_terrain_lighting, bind_water_object_push,
};
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    DofAutofocusTag, LilypadFlowerTag, LilypadTag, ReadSsboExt, ReedTag, RippleSourceTag,
};

// ---- water_planes_draw 常量 ----
pub const WATER_PRESET_MASK: (u8, u64) = (0x13, 0x6ad6_3ccd_18c8_f041);
pub const WATER_RIPPLE_BINDING: u64 = 0x6819_7c49_66ba_52c0;
pub const WATER_PRESET_PUSH: (u8, u64) = (0x18, 0xfb32_aff2_a635_9615);
pub const WATER_NOISE_TEXTURE: (u64, u64) = (0x579f_39ab_91a0_1865, 0x815b_f503_6966_978c);
pub const WATER_DOF_BINDING: u64 = 0xc265_54c6_de4b_08b0;
pub const WATER_PRESET_EXTRA: (u8, u64) = (8, 0x7716_0113_ac8d_ca3f);
pub const WATER_COLOR_UNIFORM: u64 = 0x6f71_0876_5d90_2970;
pub const WATER_SNOW_TEXTURE: (u64, u64) = (0xc555_853f_6bd2_47ac, 0x7060_67c8_5955_7777);
/// 冬季变体（O：`cmp r14b, 4`）。
pub const WATER_WINTER_VARIANT: u8 = 4;
/// `ActiveSession` 里变体字节的偏移（O：`mov r14b, [rax+0x58]`）。
pub const ACTIVE_SESSION_VARIANT_OFFSET: usize = 0x58;

/// 水色 Vec3（O：`0x142ac1940..0x142ac1960` 的常量表 + `pand/pandn` 选择）。
/// `all_set`：会话字节序列里没有 0（`r15 == r12`）。
pub fn water_color(all_set: bool) -> Vec3 {
    if all_set {
        // (0.595, 0.374) | z = 表[1]
        Vec3::new(
            f32::from_bits(0x3f1851ec),
            f32::from_bits(0x3ebf7cee),
            f32::from_bits(0x3e9cac09),
        )
    } else {
        // (0.525, 0.375) | z = 表[0]
        Vec3::new(
            f32::from_bits(0x3f066666),
            f32::from_bits(0x3ec00000),
            f32::from_bits(0x3eb0a3d7),
        )
    }
}

/// 会话字节序列里没有 0 字节（O 0x1409d9d20 的扫描循环：遇到首个 0 停下，`sete` 比较停点是否等于长度）。
pub fn all_nonzero(bytes: &[u8]) -> bool {
    bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len()) == bytes.len()
}

/// `packet.state` 第 3 字节 `&= 0x9f`（O：`and byte ptr [rdx+0xab], -0x61`）。
pub fn water_state(state: u32) -> u32 {
    state & !0x6000_0000
}

/// `water_planes_draw` 的世界输入。
#[derive(Clone, Copy, Debug)]
pub struct WaterPlanesInputs {
    /// 进程级布尔（`.data` 0x143634d90）。
    pub skip: bool,
    /// `ActiveSession +0x58`。
    pub variant: u8,
    /// 见 [`all_nonzero`]。
    pub all_set: bool,
}

/// `water_planes_draw`（O 0x1409d9bc0）。第 5 步 `bind_ray_tracing` 由调用方给（兼容入口：`draw_callbacks_b` 仍按此签名调用）；
/// 真实实现用 [`water_planes_draw_rt`]。
pub fn water_planes_draw<'p>(
    mut b: DrawBuilder<'p>,
    inputs: WaterPlanesInputs,
    bind_ray_tracing: impl FnOnce(DrawBuilder<'p>) -> DrawBuilder<'p>,
) {
    if inputs.skip {
        b.discard();
        return;
    }
    let p = b.packet_mut();
    p.state.0 = water_state(p.state.0);
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = bind_preset(b, WATER_PRESET_MASK.0, WATER_PRESET_MASK.1);
    let b = b.read_ssbo::<RippleSourceTag>(WATER_RIPPLE_BINDING);
    let b = bind_preset(b, WATER_PRESET_PUSH.0, WATER_PRESET_PUSH.1);
    let b = bind_disk(b, WATER_NOISE_TEXTURE.0, WATER_NOISE_TEXTURE.1);
    let b = b.read_ssbo::<DofAutofocusTag>(WATER_DOF_BINDING);
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = bind_preset(b, WATER_PRESET_EXTRA.0, WATER_PRESET_EXTRA.1);
    let b = bind_disk(b, WATER_NOISE_TEXTURE.0, WATER_NOISE_TEXTURE.1);
    let b = b.uniform(WATER_COLOR_UNIFORM, water_color(inputs.all_set));
    let mut b = bind_ray_tracing(b);
    if inputs.variant == WATER_WINTER_VARIANT {
        b = bind_disk(b, WATER_SNOW_TEXTURE.0, WATER_SNOW_TEXTURE.1);
    }
    b.draw();
}

/// `water_planes_draw` 的真实形态：第 5 步 = A4 的 `DrawBuilder::bind_ray_tracing::<Rt…>()`
/// （`bind_resources::bind_ray_tracing<DrawBuilder>` 0x1416289d0，4167 B；B 阶段 i_infra 接线，闭包不再需要）。
pub fn water_planes_draw_rt(b: DrawBuilder<'_>, inputs: WaterPlanesInputs) {
    water_planes_draw(b, inputs, |b| {
        b.bind_ray_tracing::<RtTrianglesSsbo, RtTriangleMetaSsbo, RtBvhSsbo, WallColorIdSsbo>()
    });
}

// ---- 水生植被 ----
pub const WATER_OBJECT_SSBO_BINDING: u64 = 0x6d92_56d4_ac78_87ad;
pub const LILYPAD_MESH_INDEX: u32 = 8;
pub const LILYPAD_TEXTURE: (u64, u64) = (0x4d64_746d_bda5_54d2, 0x6ca6_e320_25d6_e831);
pub const LILYPAD_FLOWER_MESH_INDEX: u32 = 0xf;
pub const LILYPAD_FLOWER_TEXTURE: (u64, u64) = (0x9bbd_5c16_bda1_314c, 0x1660_00eb_afac_ae04);
pub const REED_MESH_INDEX: u32 = 9;

/// `lilypad_draw`（O 0x1409da580）。
pub fn lilypad_draw(b: DrawBuilder<'_>) {
    let b = b.read_ssbo::<LilypadTag>(WATER_OBJECT_SSBO_BINDING);
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = bind_water_object_push(b);
    let b = bind_disk(b, LILYPAD_TEXTURE.0, LILYPAD_TEXTURE.1);
    b.multi_draw_indexed_indirect::<DrawIndexedIndirectBuffer>(LILYPAD_MESH_INDEX, 1, 0);
}

/// `lilypad_flower_draw`（O 0x1409da760）。
pub fn lilypad_flower_draw(b: DrawBuilder<'_>) {
    let b = b.read_ssbo::<LilypadFlowerTag>(WATER_OBJECT_SSBO_BINDING);
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = bind_water_object_push(b);
    let b = bind_disk(b, LILYPAD_FLOWER_TEXTURE.0, LILYPAD_FLOWER_TEXTURE.1);
    b.multi_draw_indexed_indirect::<DrawIndexedIndirectBuffer>(LILYPAD_FLOWER_MESH_INDEX, 1, 0);
}

/// `reed_draw`（O 0x1409da940）：没有磁盘纹理。
pub fn reed_draw(b: DrawBuilder<'_>) {
    let b = b.read_ssbo::<ReedTag>(WATER_OBJECT_SSBO_BINDING);
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = bind_water_object_push(b);
    b.multi_draw_indexed_indirect::<DrawIndexedIndirectBuffer>(REED_MESH_INDEX, 1, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::bind_resources::{
        RT_BVH_BINDING_HASH, RT_BVH_UNIFORM_HASH, RT_TRIANGLE_META_BINDING_HASH,
        RT_TRIANGLES_BINDING_HASH, RT_TRIANGLES_UNIFORM_HASH, WALL_COLOR_ID_BINDING_HASH,
    };
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::render::pass_builder::pass_resource_access::SsboType;
    use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
        TERRAIN_HEIGHTMAP_NAME, TERRAIN_LIGHTING_NAME, WATER_OBJECT_PUSH_NAME_A,
        WATER_OBJECT_PUSH_NAME_B, WATER_OBJECT_PUSH_RIPPLE_NAME,
    };
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{
        Call, RT_BVH_WORD, RT_TRIANGLES_WORD, Rig, as_f32, as_i32, rg, uniform_at,
    };

    fn plane_names() -> Vec<u64> {
        vec![
            TERRAIN_HEIGHTMAP_NAME,
            TERRAIN_LIGHTING_NAME,
            WATER_PRESET_MASK.1,
            WATER_RIPPLE_BINDING,
            WATER_PRESET_PUSH.1,
            WATER_NOISE_TEXTURE.1,
            WATER_DOF_BINDING,
            WATER_PRESET_EXTRA.1,
            WATER_SNOW_TEXTURE.1,
            RT_TRIANGLES_BINDING_HASH,
            RT_TRIANGLE_META_BINDING_HASH,
            RT_BVH_BINDING_HASH,
            WALL_COLOR_ID_BINDING_HASH,
        ]
    }

    fn plane_uniforms() -> Vec<u64> {
        vec![
            WATER_COLOR_UNIFORM,
            RT_TRIANGLES_UNIFORM_HASH,
            RT_BVH_UNIFORM_HASH,
        ]
    }

    fn inputs(variant: u8, all_set: bool) -> WaterPlanesInputs {
        WaterPlanesInputs {
            skip: false,
            variant,
            all_set,
        }
    }

    #[test]
    fn water_color_table() {
        let a = water_color(true);
        assert!(
            (a.x - 0.595).abs() < 1e-6 && (a.y - 0.374).abs() < 1e-6 && (a.z - 0.306).abs() < 1e-6
        );
        let b = water_color(false);
        assert!(
            (b.x - 0.525).abs() < 1e-6 && (b.y - 0.375).abs() < 1e-6 && (b.z - 0.345).abs() < 1e-6
        );
    }

    #[test]
    fn all_nonzero_scan_matches_the_loop() {
        assert!(all_nonzero(&[]), "空序列：r15 == r12 == 0");
        assert!(all_nonzero(&[1, 2, 3]));
        assert!(!all_nonzero(&[1, 0, 3]));
        assert!(!all_nonzero(&[0]));
    }

    #[test]
    fn state_clears_bits_5_and_6_of_byte_3() {
        assert_eq!(water_state(0xff00_00ff), 0x9f00_00ff);
    }

    #[test]
    fn skip_discards_the_builder() {
        let mut rig = Rig::new(&plane_names(), &plane_uniforms());
        water_planes_draw_rt(
            rig.builder(),
            WaterPlanesInputs {
                skip: true,
                variant: 0,
                all_set: false,
            },
        );
        assert!(rig.finish().is_empty(), "discard 不提交 draw");
        assert!(rig.calls().is_empty(), "守卫在任何绑定之前");
    }

    #[test]
    fn plane_bind_order_with_duplicated_helper_bindings() {
        let mut rig = Rig::new(&plane_names(), &plane_uniforms());
        water_planes_draw_rt(rig.builder(), inputs(0, false));
        assert_eq!(
            rig.calls(),
            vec![
                Call::Preset(9),
                Call::Preset(10),
                Call::Preset(0x13),
                rg::<RippleSourceTag>(),
                Call::Preset(0x18),
                Call::Disk(0x579f39ab91a01865),
                rg::<DofAutofocusTag>(),
                Call::Preset(9),
                Call::Preset(10),
                Call::Preset(8),
                Call::Disk(0x579f39ab91a01865),
                // `bind_ray_tracing`：Tri / Meta / Bvh / WallColorIds 四个槽（O 0x1416289d0）。
                rg::<RtTrianglesSsbo>(),
                rg::<RtTriangleMetaSsbo>(),
                rg::<RtBvhSsbo>(),
                rg::<WallColorIdSsbo>(),
            ]
        );
        let d = rig.finish();
        assert_eq!(d[0].kind, DrawKind::InstancedGlDrawFlag { flag: 1 });
        assert_eq!(d[0].state.0 & 0x6000_0000, 0);
    }

    #[test]
    fn snow_texture_only_for_variant_4() {
        for (variant, snow) in [(3u8, false), (4, true), (5, false)] {
            let mut rig = Rig::new(&plane_names(), &plane_uniforms());
            water_planes_draw_rt(rig.builder(), inputs(variant, true));
            let has = rig.calls().contains(&Call::Disk(0xc555853f6bd247ac));
            assert_eq!(has, snow, "variant {variant}");
            rig.finish();
        }
    }

    /// 第 5 步先于第 6 步：`bind_ray_tracing` 的四个槽在雪纹理（冬季变体）之前；两个 `write_uniform<u32>` 写入宿主给的缓冲字
    /// （Triangles / Bvh，O 0x1416289d0）。
    #[test]
    fn ray_tracing_is_bound_before_the_snow_texture_and_writes_the_buffer_words() {
        let mut rig = Rig::new(&plane_names(), &plane_uniforms());
        let b = rig.builder();
        // 与 `water_planes_draw` 内部相同的序列只到 ray tracing 为止，这里直接调用被测函数再读常量块不可行（draw 会消耗 builder），
        // 故用 builder 单独验证 uniform 偏移：Rig 里 uniforms 的序号 = 偏移 / UNIFORM_STRIDE。
        let b =
            b.bind_ray_tracing::<RtTrianglesSsbo, RtTriangleMetaSsbo, RtBvhSsbo, WallColorIdSsbo>();
        let data = b.constants_data().unwrap().to_vec();
        assert_eq!(as_i32(&uniform_at(&data, 1, 4)) as u32, RT_TRIANGLES_WORD);
        assert_eq!(as_i32(&uniform_at(&data, 2, 4)) as u32, RT_BVH_WORD);
        b.draw();
        rig.finish();
        // 冬季：雪纹理排在四个 Rt 槽之后。
        let mut rig = Rig::new(&plane_names(), &plane_uniforms());
        water_planes_draw_rt(rig.builder(), inputs(WATER_WINTER_VARIANT, true));
        let calls = rig.calls();
        let rt = calls
            .iter()
            .position(|c| *c == rg::<WallColorIdSsbo>())
            .unwrap();
        let snow = calls
            .iter()
            .position(|c| *c == Call::Disk(0xc555853f6bd247ac))
            .unwrap();
        assert!(rt < snow);
        rig.finish();
    }

    /// 兼容入口与 `_rt` 等价：闭包里调用同一个 `bind_ray_tracing` 序列。
    #[test]
    fn closure_entry_with_the_real_bind_equals_the_rt_entry() {
        let mut a = Rig::new(&plane_names(), &plane_uniforms());
        water_planes_draw(a.builder(), inputs(4, true), |b| {
            b.bind_ray_tracing::<RtTrianglesSsbo, RtTriangleMetaSsbo, RtBvhSsbo, WallColorIdSsbo>()
        });
        let mut r = Rig::new(&plane_names(), &plane_uniforms());
        water_planes_draw_rt(r.builder(), inputs(4, true));
        assert_eq!(a.calls(), r.calls());
        a.finish();
        r.finish();
    }

    #[test]
    fn water_color_uniform_is_written_as_three_floats() {
        let mut rig = Rig::new(&plane_names(), &plane_uniforms());
        let b = rig
            .builder()
            .uniform(WATER_COLOR_UNIFORM, water_color(true));
        let data = b.constants_data().unwrap().to_vec();
        let raw = uniform_at(&data, 0, 12);
        assert_eq!(as_f32(&raw[0..4]), water_color(true).x);
        assert_eq!(as_f32(&raw[8..12]), water_color(true).z);
        b.draw();
        rig.finish();
    }

    fn push_names() -> Vec<u64> {
        vec![
            WATER_OBJECT_SSBO_BINDING,
            TERRAIN_HEIGHTMAP_NAME,
            TERRAIN_LIGHTING_NAME,
            WATER_OBJECT_PUSH_NAME_A,
            WATER_OBJECT_PUSH_RIPPLE_NAME,
            WATER_OBJECT_PUSH_NAME_B,
            LILYPAD_TEXTURE.1,
            LILYPAD_FLOWER_TEXTURE.1,
        ]
    }

    #[test]
    fn lilypad_sequence_and_mesh_index() {
        let mut rig = Rig::new(&push_names(), &[]);
        lilypad_draw(rig.builder());
        assert_eq!(
            rig.calls(),
            vec![
                rg::<LilypadTag>(),
                Call::Preset(9),
                Call::Preset(10),
                Call::Preset(0xf),
                rg::<RippleSourceTag>(),
                Call::Preset(0x18),
                Call::Disk(0x4d64746dbda554d2),
                rg::<DrawIndexedIndirectBuffer>(),
            ]
        );
        let d = rig.finish();
        let buf = rig.rg_id(DrawIndexedIndirectBuffer::rg_buffer_hash());
        assert_eq!(
            d[0].kind,
            DrawKind::IndexedIndirect {
                buffer: buf,
                arg_48: 8 * 0x14,
                arg_50: 1,
                arg_58: 0,
                first_index_count: Some(0x30)
            }
        );
    }

    #[test]
    fn reed_has_no_disk_texture_and_uses_mesh_9() {
        let mut rig = Rig::new(&push_names(), &[]);
        reed_draw(rig.builder());
        assert_eq!(
            rig.calls().len(),
            7,
            "Reed SSBO + 预设 9/10 + (0xf, Ripple, 0x18) + 间接缓冲"
        );
        assert!(rig.calls().iter().all(|c| !matches!(c, Call::Disk(_))));
        let d = rig.finish();
        match &d[0].kind {
            DrawKind::IndexedIndirect {
                arg_48,
                first_index_count,
                ..
            } => {
                assert_eq!((*arg_48, *first_index_count), (9 * 0x14, Some(0xae)));
            }
            k => panic!("{k:?}"),
        }
    }

    #[test]
    fn lilypad_flower_uses_mesh_0xf_which_is_beyond_the_first_ten_table_entries() {
        let mut rig = Rig::new(&push_names(), &[]);
        lilypad_flower_draw(rig.builder());
        assert_eq!(
            rig.calls(),
            vec![
                rg::<LilypadFlowerTag>(),
                Call::Preset(9),
                Call::Preset(10),
                Call::Preset(0xf),
                rg::<RippleSourceTag>(),
                Call::Preset(0x18),
                Call::Disk(0x9bbd5c16bda1314c),
                rg::<DrawIndexedIndirectBuffer>(),
            ]
        );
        let d = rig.finish();
        match &d[0].kind {
            DrawKind::IndexedIndirect {
                arg_48,
                first_index_count,
                ..
            } => {
                assert_eq!((*arg_48, *first_index_count), (0xf * 0x14, Some(180)));
            }
            k => panic!("{k:?}"),
        }
    }

    #[test]
    fn lilypad_flower_texture_and_slot_names() {
        use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::DrawSsbo;
        assert_eq!(LILYPAD_FLOWER_MESH_INDEX, 0xf);
        assert_eq!(LILYPAD_FLOWER_TEXTURE.0, 0x9bbd5c16bda1314c);
        assert_eq!(LilypadFlowerTag::SLOT_NAME, "Lilypad Flowers");
        assert_eq!(LilypadTag::SLOT_NAME, "Lilypads");
        assert_eq!(ReedTag::SLOT_NAME, "Reeds");
    }
}
