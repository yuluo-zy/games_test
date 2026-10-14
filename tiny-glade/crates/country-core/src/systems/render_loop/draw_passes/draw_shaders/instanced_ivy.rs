//! instanced_ivy.rs — 常春藤实例绘制（枝 / 叶 / 花）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs`。机器码（O）：
//! - `draw_instanced_ivy_branch_shader` 0x1408dbc50（1319 B）
//! - `ivy_leaf_shader` 0x1408dc1c0（2093 B，共用体）；`draw_instanced_ivy_leaf_shader` 0x1408dc180、
//!   `draw_instanced_ivy_flower_shader` 0x1408dc1a0 各 21 B：只是 `mov rdx, r8`（世界指针后移）+ 给出
//!   `(is_flower, texture_key)` 后 `jmp ivy_leaf_shader`：叶 = `(false, 0xe6d1a603f18a02b7)`、花 = `(true, 0x0c88938148e6dab3)`。
//!
//! ## 枝（O）
//! 世界资源（TypeId 0xdf356893cc4e98a8/0xee486fdcc00a9f4c）是个 `HashMap<_, 0x98 B>`；对每个桶取 `usize @桶+0x20`，
//! 累加 `saturating_sub(1)`（`cmovae`）得到实例数。随后：`read_storage_buffer<IvySegmentSsbo>`（槽 "Ivy Segments Buffer"，
//! 名 0x899ce6fa16080d30）→ 预设 9 / 10 → 预设 8（名 0x77160113ac8dca3f）→ 磁盘 0x579f39ab91a01865（名 0x815bf5036966978c）
//! → 预设 5（名 0xed502fff6dc4021f）→ 磁盘 0xedf425b62b653c03（名 0x2136b4ee6592f1ca）→ `draw_instanced_w_gl_draw_flag(sum)`。
//!
//! ## 叶 / 花（O）
//! 1. 相机资源（TypeId 0xbba451ee6a08a3ff/0x8d008d15febcf29d）`CameraMatrices::eye_position()` →
//!    常量块存在 → `write_uniform<Vec3>(0x4bbdb30b873d3990, eye)`
//! 2. 预设 9 / 10 / 5
//! 3. `TextureFromDisk(texture_key)`（名 0x1a51099ba93c5ba6）
//! 4. 常量块存在 → `write_uniform<bool>(0xf70ab63d7b8fd23d, is_flower)`
//! 5. `read_storage_buffer<IvyFlowerSsbo|IvyLeafSsbo>`（同一个绑定名 0x3dce10f97920242d）
//! 6. 库里取该 SSBO 的实例缓冲（`FrozenMap::get(SsboId)` → `assert_eq!(name, type_name)` → 虚表槽 +0x18），实例数
//!    缺省 0 → `draw_instanced_w_gl_draw_flag(count)`

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::identity_op)]
use glam::Vec3;

use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    bind_disk, bind_preset, bind_terrain_heightmap, bind_terrain_lighting,
};
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    IvyFlowerTag, IvyLeafTag, IvySegmentTag, ReadSsboExt, SsboInstanceView, instance_count,
};

pub const IVY_PRESET_5: (u8, u64) = (5, 0xed50_2fff_6dc4_021f);
pub const IVY_PRESET_8: (u8, u64) = (8, 0x7716_0113_ac8d_ca3f);
pub const IVY_SEGMENT_BINDING: u64 = 0x899c_e6fa_1608_0d30;
pub const IVY_BRANCH_TEXTURE_A: (u64, u64) = (0x579f_39ab_91a0_1865, 0x815b_f503_6966_978c);
pub const IVY_BRANCH_TEXTURE_B: (u64, u64) = (0xedf4_25b6_2b65_3c03, 0x2136_b4ee_6592_f1ca);
pub const IVY_EYE_UNIFORM: u64 = 0x4bbd_b30b_873d_3990;
pub const IVY_LEAF_TEXTURE_NAME: u64 = 0x1a51_099b_a93c_5ba6;
pub const IVY_IS_FLOWER_UNIFORM: u64 = 0xf70a_b63d_7b8f_d23d;
pub const IVY_LEAF_SSBO_BINDING: u64 = 0x3dce_10f9_7920_242d;
/// 薄壳传给 `ivy_leaf_shader` 的磁盘纹理文件键。
pub const IVY_LEAF_TEXTURE_KEY: u64 = 0xe6d1_a603_f18a_02b7;
pub const IVY_FLOWER_TEXTURE_KEY: u64 = 0x0c88_9381_48e6_dab3;

/// 枝的实例数：对每条藤的段数取 `saturating_sub(1)` 求和（O 0x1408dbd10 起的 SIMD 桶扫描）。
pub fn branch_instance_count(segment_counts: impl IntoIterator<Item = usize>) -> usize {
    segment_counts
        .into_iter()
        .map(|n| n.saturating_sub(1))
        .sum()
}

/// `draw_instanced_ivy_branch_shader`（O 0x1408dbc50）。
pub fn draw_instanced_ivy_branch_shader(
    b: DrawBuilder<'_>,
    segment_counts: impl IntoIterator<Item = usize>,
) {
    let count = branch_instance_count(segment_counts);
    let b = b.read_ssbo::<IvySegmentTag>(IVY_SEGMENT_BINDING);
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = bind_preset(b, IVY_PRESET_8.0, IVY_PRESET_8.1);
    let b = bind_disk(b, IVY_BRANCH_TEXTURE_A.0, IVY_BRANCH_TEXTURE_A.1);
    let b = bind_preset(b, IVY_PRESET_5.0, IVY_PRESET_5.1);
    let b = bind_disk(b, IVY_BRANCH_TEXTURE_B.0, IVY_BRANCH_TEXTURE_B.1);
    b.draw_instanced_w_gl_draw_flag(count as u32);
}

/// `ivy_leaf_shader`（O 0x1408dc1c0）。`eye` = `CameraMatrices::eye_position()`。
pub fn ivy_leaf_shader(
    b: DrawBuilder<'_>,
    is_flower: bool,
    eye: Vec3,
    texture_key: u64,
    lib: &dyn SsboInstanceView,
) {
    let b = b.uniform(IVY_EYE_UNIFORM, eye);
    let b = bind_terrain_heightmap(b);
    let b = bind_terrain_lighting(b);
    let b = bind_preset(b, IVY_PRESET_5.0, IVY_PRESET_5.1);
    let b = bind_disk(b, texture_key, IVY_LEAF_TEXTURE_NAME);
    let b = b.uniform(IVY_IS_FLOWER_UNIFORM, is_flower);
    let (b, count) = if is_flower {
        let b = b.read_ssbo::<IvyFlowerTag>(IVY_LEAF_SSBO_BINDING);
        (b, instance_count::<IvyFlowerTag>(lib))
    } else {
        let b = b.read_ssbo::<IvyLeafTag>(IVY_LEAF_SSBO_BINDING);
        (b, instance_count::<IvyLeafTag>(lib))
    };
    b.draw_instanced_w_gl_draw_flag(count as u32);
}

/// `draw_instanced_ivy_leaf_shader`（O 0x1408dc180）。
pub fn draw_instanced_ivy_leaf_shader(b: DrawBuilder<'_>, eye: Vec3, lib: &dyn SsboInstanceView) {
    ivy_leaf_shader(b, false, eye, IVY_LEAF_TEXTURE_KEY, lib);
}

/// `draw_instanced_ivy_flower_shader`（O 0x1408dc1a0）。
pub fn draw_instanced_ivy_flower_shader(b: DrawBuilder<'_>, eye: Vec3, lib: &dyn SsboInstanceView) {
    ivy_leaf_shader(b, true, eye, IVY_FLOWER_TEXTURE_KEY, lib);
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
        Call, Rig, as_f32, as_i32, rg, uniform_at,
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
    fn branch_count_is_a_saturating_sum() {
        assert_eq!(branch_instance_count([]), 0);
        assert_eq!(branch_instance_count([0, 1, 2, 5]), 0 + 0 + 1 + 4);
        assert_eq!(branch_instance_count([10]), 9);
    }

    #[test]
    fn branch_sequence_and_instance_count() {
        let names = [
            IVY_SEGMENT_BINDING,
            TERRAIN_HEIGHTMAP_NAME,
            TERRAIN_LIGHTING_NAME,
            IVY_PRESET_8.1,
            IVY_BRANCH_TEXTURE_A.1,
            IVY_PRESET_5.1,
            IVY_BRANCH_TEXTURE_B.1,
        ];
        let mut rig = Rig::new(&names, &[]);
        draw_instanced_ivy_branch_shader(rig.builder(), [3, 1, 0, 8]);
        assert_eq!(
            rig.calls(),
            vec![
                rg::<IvySegmentTag>(),
                Call::Preset(9),
                Call::Preset(10),
                Call::Preset(8),
                Call::Disk(0x579f39ab91a01865),
                Call::Preset(5),
                Call::Disk(0xedf425b62b653c03),
            ]
        );
        assert_eq!(
            rig.finish()[0].kind,
            DrawKind::InstancedGlDrawFlag {
                flag: 2 + 0 + 0 + 7
            }
        );
    }

    #[test]
    fn leaf_and_flower_thunks_differ_only_in_flag_and_texture_key() {
        assert_eq!(
            IVY_LEAF_TEXTURE_KEY, 0xe6d1a603f18a02b7,
            "= -0x192e59fc0e75fd49"
        );
        assert_eq!(IVY_FLOWER_TEXTURE_KEY, 0x0c88938148e6dab3);
        let names = [
            TERRAIN_HEIGHTMAP_NAME,
            TERRAIN_LIGHTING_NAME,
            IVY_PRESET_5.1,
            IVY_LEAF_TEXTURE_NAME,
            IVY_LEAF_SSBO_BINDING,
        ];
        for flower in [false, true] {
            let mut rig = Rig::new(&names, &[IVY_EYE_UNIFORM, IVY_IS_FLOWER_UNIFORM]);
            let eye = Vec3::new(1.0, 2.0, 3.0);
            if flower {
                draw_instanced_ivy_flower_shader(rig.builder(), eye, &lib_with::<IvyFlowerTag>(42));
            } else {
                draw_instanced_ivy_leaf_shader(rig.builder(), eye, &lib_with::<IvyLeafTag>(7));
            }
            let want_key = if flower {
                IVY_FLOWER_TEXTURE_KEY
            } else {
                IVY_LEAF_TEXTURE_KEY
            };
            let ssbo = if flower {
                rg::<IvyFlowerTag>()
            } else {
                rg::<IvyLeafTag>()
            };
            assert_eq!(
                rig.calls(),
                vec![
                    Call::Preset(9),
                    Call::Preset(10),
                    Call::Preset(5),
                    Call::Disk(want_key),
                    ssbo
                ]
            );
            assert_eq!(
                rig.finish()[0].kind,
                DrawKind::InstancedGlDrawFlag {
                    flag: if flower { 42 } else { 7 }
                }
            );
        }
    }

    #[test]
    fn eye_and_flower_uniforms_are_written_first_and_after_the_texture() {
        let names = [
            TERRAIN_HEIGHTMAP_NAME,
            TERRAIN_LIGHTING_NAME,
            IVY_PRESET_5.1,
            IVY_LEAF_TEXTURE_NAME,
            IVY_LEAF_SSBO_BINDING,
        ];
        let mut rig = Rig::new(&names, &[IVY_EYE_UNIFORM, IVY_IS_FLOWER_UNIFORM]);
        let b = rig
            .builder()
            .uniform(IVY_EYE_UNIFORM, Vec3::new(1.5, -2.0, 3.25))
            .uniform(IVY_IS_FLOWER_UNIFORM, true);
        let data = b.constants_data().unwrap().to_vec();
        let eye = uniform_at(&data, 0, 12);
        assert_eq!(
            [as_f32(&eye[0..4]), as_f32(&eye[4..8]), as_f32(&eye[8..12])],
            [1.5, -2.0, 3.25]
        );
        assert_eq!(as_i32(&uniform_at(&data, 1, 4)), 1);
        b.draw();
        rig.finish();
    }

    #[test]
    fn missing_library_entry_means_zero_instances() {
        let names = [IVY_LEAF_SSBO_BINDING];
        let mut rig = Rig::new(&names, &[]);
        draw_instanced_ivy_leaf_shader(rig.builder(), Vec3::ZERO, &Lib(vec![]));
        assert_eq!(
            rig.finish()[0].kind,
            DrawKind::InstancedGlDrawFlag { flag: 0 }
        );
    }
}
