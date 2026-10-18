//! draw_bricks.rs — 墙砖 LOD0 / LOD1 间接绘制（`_wall_wall_brick_lod{0,1}.raster`）。
//!
//! PDB `systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs`（编译单元 487）。机器码（O）：
//! `indirect_draw_bricks<0>` 0x1408a53d0（1692 B）、`indirect_draw_bricks<1>` 0x1408a4ca0（1831 B）。
//! ABI：`rcx = &mut DrawBuilder`、`rdx = &EntityDrawParams`（`[rdx+8]` = 实体）、`r8 = &World`。
//!
//! ## 实体与代理（O）
//! 实体经 `Entities` 表校验（槽越界 / 代数不符 / 空位 → `Result::unwrap` panic，文案
//! "called `Result::unwrap()` on an `Err` value"，43 B）；再 `UnsafeEntityCell::get<WallBrickInstancedDrawProxy>`，
//! 缺失 → `Option::unwrap` panic。代理里用到三个字节：`b0 @+0`、`b1 @+1`、`b2 @+2`。
//!
//! ## 绘制模式字节（O）
//! `DrawBuilder +0xf0`（`bind_common` 写入 `CommonDrawParams +4`）是 0..=4 的小枚举，不是 bool：
//! - LOD0：`>= 2` → `discard`；
//! - LOD1：`r12b = 0x1c >> v`（bit0 = `v ∈ {2,3,4}`）、`r13b = (0x201000000 >> 8v) & 0xff`（= `[0,0,0,1,2][v & 7]`）。
//!
//! ## LOD0（O）
//! `mode >= 2` 或 `b1 == 1` → `discard`；否则 `packet.state` 第 0 字节 `(b & 0xf3) | 4`，然后公共段（见下）。
//! ## LOD1（O）
//! `b1 == 1`：需要 `bit && b2 == sel`，否则 `discard`；`b1 != 1`：需要 `!bit`，否则 `discard`。通过后 `state & 0xf3`，公共段。
//!
//! ## 公共段（O）
//! 1. 库里取 `BrickTransformSsbo` 的实例缓冲（`FrozenMap::get` → `assert_eq!(name, type_name)` → 槽 +0x18，缺失 →
//!    `Option::unwrap` panic）→ `bind_external_storage_buffer(0xac77ba3d359698bc, &buffer.handle)`
//! 2. `read_storage_buffer<BrickDrawListSsbo>`（名 0xca16df0f469c8dff）
//! 3. LOD1 且 `bit`（代理路径）→ 直接 `multi_draw_indexed_indirect(0x15 + sel, 1, 0)`（跳表 `[0x15, 0x16, 0x17]`）
//! 4. 否则（常规路径）：世界资源（TypeId 0x850dbd51eafef756/0x85782a4b39854c35）的 `u8 @+0x10` = `brick_mode`：
//!    - `brick_mode != 0`：`bricks_mode = (b0 & 1) ? brick_mode : 0`；
//!    - `brick_mode == 0`：`b0 != 0` → `discard`（此时已绑过前两项）；`bricks_mode = 0`；
//!    然后 `bind_brick_erase_highlight` → `bind_wall_color_customization` → 磁盘 0x7db9e7483fa24078（名 0xaa8f0490dba0d03c）→
//!    `read_storage_buffer<RoofColorIdSsbo>`（名 0xd56729333fc94f92）→ 常量块存在 → `write_uniform<i32>(0x052e7aa4e36b047b,
//!    bricks_mode)` → `multi_draw_indexed_indirect(base + 2 * (bricks_mode != 0), 1, 0)`，`base` = LOD0 `0x11`、LOD1 `0x12`。
//!
//! U：`brick_mode` 资源与代理字节 `b0/b1/b2` 的名字；`bind_brick_erase_highlight` 的四个世界资源类型（见 A4）。
//! shader URI（O）：`/wall/wall_brick_lod0.raster.hlsl`、`/wall/wall_brick_lod1.raster.hlsl`。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::bool_assert_comparison, clippy::doc_lazy_continuation)]
use crate::render::pass_builder::bind_resources::WallColorIdSsbo;
use crate::render::pass_builder::draw_builder::{BrickEraseHighlightInputs, DrawBuilder};
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes::{
    DrawModeExt, bind_disk,
};
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
    BrickDrawListTag, BrickTransformTag, ReadSsboExt, RoofColorIdTag, SsboInstanceView,
    storage_buffer,
};

pub const BRICK_TRANSFORM_BINDING: u64 = 0xac77_ba3d_3596_98bc;
pub const BRICK_DRAW_LIST_BINDING: u64 = 0xca16_df0f_469c_8dff;
pub const BRICK_TEXTURE: (u64, u64) = (0x7db9_e748_3fa2_4078, 0xaa8f_0490_dba0_d03c);
pub const BRICK_ROOF_COLOR_BINDING: u64 = 0xd567_2933_3fc9_4f92;
pub const BRICK_MODE_UNIFORM: u64 = 0x052e_7aa4_e36b_047b;
/// 常规路径的网格下标基值（O：LOD0 `lea edi,[2*rax+0x11]`、LOD1 `lea esi,[2*rax+0x12]`）。
pub const BRICK_BASE_INDEX: [u32; 2] = [0x11, 0x12];
/// 代理路径的网格下标（O：LOD1 跳表 0x1408a4f17 / 0x1408a5134 / 0x1408a512d）。
pub const BRICK_PROXY_INDEX: [u32; 3] = [0x15, 0x16, 0x17];
/// LOD1 判定常量：`0x1c >> v` 与 `0x201000000 >> 8v`。
const LOD1_BIT_MASK: u8 = 0x1c;
const LOD1_SEL_TABLE: u64 = 0x2_0100_0000;

/// 代理组件里被读取的三个字节。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrickProxy {
    pub b0: u8,
    pub b1: u8,
    pub b2: u8,
}

/// 实体查找结果（O：`Entities` 校验 + `UnsafeEntityCell::get`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrickEntity {
    /// 槽越界 / 代数不符 / 空位 → `Result::unwrap` panic。
    Missing,
    /// 实体存在但没有 `WallBrickInstancedDrawProxy` → `Option::unwrap` panic。
    NoProxy,
    Proxy(BrickProxy),
}

/// LOD1 的两个由模式字节导出的量（O：`shr r12b, cl` / `shr r13, cl`，移位计数按硬件规则取模）。
pub fn lod1_mode_bits(v: u8) -> (bool, u8) {
    let count8 = (v & 0x1f) as u32; // 8 位 `shr r12b, cl`：计数取低 5 位
    let bit = if count8 >= 8 {
        0
    } else {
        (LOD1_BIT_MASK >> count8) & 1
    };
    let shift = (((v as u32) << 3) & 0xff) & 0x3f; // `shl cl, 3`（8 位）后 64 位 `shr` 取低 6 位
    (bit != 0, ((LOD1_SEL_TABLE >> shift) & 0xff) as u8)
}

/// 前置判定的结果。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrickGate {
    Discard,
    /// 通过；`proxy_index` 为 LOD1 代理路径的下标（`None` = 常规路径）；`cull_or`：`state` 的 `| 4` 位。
    Pass {
        proxy_index: Option<u32>,
        cull_or: u32,
    },
}

/// LOD0 / LOD1 的前置判定（纯函数）。
pub fn gate<const LOD: u8>(mode: u8, p: BrickProxy) -> BrickGate {
    if LOD == 0 {
        if mode >= 2 || p.b1 == 1 {
            return BrickGate::Discard;
        }
        return BrickGate::Pass {
            proxy_index: None,
            cull_or: 4,
        };
    }
    let (bit, sel) = lod1_mode_bits(mode);
    if p.b1 == 1 {
        if !bit || p.b2 != sel {
            return BrickGate::Discard;
        }
        // 跳表只有三项（sel ∈ {0,1,2}）；第 4 项目标 0x1408a524e 是不可达分支。
        let idx = BRICK_PROXY_INDEX.get(sel as usize).copied();
        BrickGate::Pass {
            proxy_index: idx,
            cull_or: 0,
        }
    } else if bit {
        BrickGate::Discard
    } else {
        BrickGate::Pass {
            proxy_index: None,
            cull_or: 0,
        }
    }
}

/// 常规路径里的 `bricks_mode`；`None` = `discard`。
pub fn bricks_mode(brick_mode: u8, b0: u8) -> Option<u8> {
    if brick_mode != 0 {
        Some(if b0 & 1 != 0 { brick_mode } else { 0 })
    } else if b0 != 0 {
        None
    } else {
        Some(0)
    }
}

/// `indirect_draw_bricks<LOD>`（O 0x1408a53d0 / 0x1408a4ca0）。
///
/// - `entity`：见 [`BrickEntity`]；
/// - `lib`：`AssetSsboLibrary` 查询边界；
/// - `brick_mode`：世界资源 `u8 @+0x10`（资源缺失 → 调用方 `Option::unwrap` panic）；
/// - `erase_highlight`：`DrawBuilder::bind_brick_erase_highlight`（0x14090be10）的四个世界资源值。
pub fn indirect_draw_bricks<'p, const LOD: u8>(
    b: DrawBuilder<'p>,
    entity: BrickEntity,
    lib: &dyn SsboInstanceView,
    brick_mode: u8,
    erase_highlight: &BrickEraseHighlightInputs,
) {
    let mode = b.draw_mode();
    indirect_draw_bricks_with_mode::<LOD>(b, mode, entity, lib, brick_mode, erase_highlight);
}

/// [`indirect_draw_bricks`] 的核心：`mode` = `DrawBuilder +0xf0`（见模块注释；A4 的 `common_flag` 目前只能给 0/1，
/// 所以把它显式化，LOD1 的 2..=4 分支才能被测试）。
pub fn indirect_draw_bricks_with_mode<'p, const LOD: u8>(
    mut b: DrawBuilder<'p>,
    mode: u8,
    entity: BrickEntity,
    lib: &dyn SsboInstanceView,
    brick_mode: u8,
    erase_highlight: &BrickEraseHighlightInputs,
) {
    let proxy = match entity {
        BrickEntity::Missing => panic!("called `Result::unwrap()` on an `Err` value"),
        BrickEntity::NoProxy => panic!("called `Option::unwrap()` on a `None` value"),
        BrickEntity::Proxy(p) => p,
    };
    let (proxy_index, cull_or) = match gate::<LOD>(mode, proxy) {
        BrickGate::Discard => {
            b.discard();
            return;
        }
        BrickGate::Pass {
            proxy_index,
            cull_or,
        } => (proxy_index, cull_or),
    };
    let p = b.packet_mut();
    p.state.0 = (p.state.0 & !0x0c) | cull_or;

    let transform_handle = storage_buffer::<BrickTransformTag>(lib)
        .expect("called `Option::unwrap()` on a `None` value");
    let b = b.bind_external_storage_buffer(BRICK_TRANSFORM_BINDING, &transform_handle);
    let b = b.read_ssbo::<BrickDrawListTag>(BRICK_DRAW_LIST_BINDING);

    if let Some(idx) = proxy_index {
        b.multi_draw_indexed_indirect::<crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>(idx, 1, 0);
        return;
    }
    let Some(mode) = bricks_mode(brick_mode, proxy.b0) else {
        b.discard();
        return;
    };
    let b = b.bind_brick_erase_highlight(erase_highlight);
    let b = b.bind_wall_color_customization::<WallColorIdSsbo>();
    let b = bind_disk(b, BRICK_TEXTURE.0, BRICK_TEXTURE.1);
    let b = b.read_ssbo::<RoofColorIdTag>(BRICK_ROOF_COLOR_BINDING);
    let b = b.uniform(BRICK_MODE_UNIFORM, mode as i32);
    let idx = BRICK_BASE_INDEX[LOD as usize] + 2 * (mode != 0) as u32;
    b.multi_draw_indexed_indirect::<crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>(idx, 1, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::draw_builder::{
        BRICK_ERASE_ENABLED_UNIFORM, WALL_COLOR_TEXTURES,
    };
    use crate::render::pass_builder::draw_stream::DrawKind;
    use crate::render::pass_builder::pass_resource_access::SsboType;
    use crate::rhapsody_contract::ManagedHandle;
    use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::{
        DrawSsbo, InstanceBufferView,
    };
    use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{Call, Rig, rg};
    use crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer;

    fn p(b0: u8, b1: u8, b2: u8) -> BrickProxy {
        BrickProxy { b0, b1, b2 }
    }

    #[test]
    fn lod1_mode_bits_table() {
        let t: Vec<(bool, u8)> = (0..8).map(lod1_mode_bits).collect();
        assert_eq!(
            t,
            vec![
                (false, 0),
                (false, 0),
                (true, 0),
                (true, 1),
                (true, 2),
                (false, 0),
                (false, 0),
                (false, 0)
            ]
        );
        // 8 位移位计数取低 5 位：v = 8 → 0x1c >> 8 = 0；v = 32 → 计数 0 → 0x1c 的 bit0 = 0。
        assert_eq!(lod1_mode_bits(8).0, false);
        // `shl cl, 3` 在 8 位寄存器里回绕：v = 33 → cl = 0x08 → 64 位移位 8 → 字节 1 = 0。
        assert_eq!(lod1_mode_bits(11).1, 1, "v & 7 == 3 → sel = 1");
    }

    #[test]
    fn lod0_gate() {
        assert_eq!(
            gate::<0>(0, p(0, 0, 0)),
            BrickGate::Pass {
                proxy_index: None,
                cull_or: 4
            }
        );
        assert_eq!(
            gate::<0>(1, p(1, 0, 9)),
            BrickGate::Pass {
                proxy_index: None,
                cull_or: 4
            }
        );
        assert_eq!(gate::<0>(2, p(0, 0, 0)), BrickGate::Discard, "mode >= 2");
        assert_eq!(gate::<0>(0, p(0, 1, 0)), BrickGate::Discard, "b1 == 1");
    }

    #[test]
    fn lod1_gate_proxy_and_regular_paths() {
        // b1 != 1：只有 !bit（mode 0/1）才通过，走常规路径，cull 清零。
        assert_eq!(
            gate::<1>(0, p(0, 0, 0)),
            BrickGate::Pass {
                proxy_index: None,
                cull_or: 0
            }
        );
        assert_eq!(
            gate::<1>(1, p(0, 0, 0)),
            BrickGate::Pass {
                proxy_index: None,
                cull_or: 0
            }
        );
        assert_eq!(gate::<1>(2, p(0, 0, 0)), BrickGate::Discard);
        // b1 == 1：需要 bit 且 b2 == sel。
        assert_eq!(gate::<1>(0, p(0, 1, 0)), BrickGate::Discard);
        assert_eq!(
            gate::<1>(2, p(0, 1, 0)),
            BrickGate::Pass {
                proxy_index: Some(0x15),
                cull_or: 0
            }
        );
        assert_eq!(
            gate::<1>(3, p(0, 1, 1)),
            BrickGate::Pass {
                proxy_index: Some(0x16),
                cull_or: 0
            }
        );
        assert_eq!(
            gate::<1>(4, p(0, 1, 2)),
            BrickGate::Pass {
                proxy_index: Some(0x17),
                cull_or: 0
            }
        );
        assert_eq!(gate::<1>(4, p(0, 1, 1)), BrickGate::Discard, "b2 != sel");
    }

    #[test]
    fn bricks_mode_rules() {
        assert_eq!(bricks_mode(3, 1), Some(3));
        assert_eq!(bricks_mode(3, 2), Some(0), "只看 b0 的 bit0");
        assert_eq!(bricks_mode(0, 0), Some(0));
        assert_eq!(
            bricks_mode(0, 1),
            None,
            "brick_mode == 0 且 b0 != 0 → discard"
        );
        assert_eq!(bricks_mode(0, 2), None, "整字节比较（cmp byte, 0）");
    }

    struct Lib(Option<InstanceBufferView>);
    impl SsboInstanceView for Lib {
        fn lookup(&self, id: u64) -> Option<(&str, Option<InstanceBufferView>)> {
            (id == BrickTransformTag::rg_buffer_hash())
                .then(|| (BrickTransformTag::TYPE_NAME, self.0.clone()))
        }
    }
    fn lib() -> Lib {
        Lib(Some(InstanceBufferView {
            instance_count: 10,
            handle: ManagedHandle(77),
        }))
    }

    fn names() -> Vec<u64> {
        let mut v = vec![
            BRICK_TRANSFORM_BINDING,
            BRICK_DRAW_LIST_BINDING,
            BRICK_TEXTURE.1,
            BRICK_ROOF_COLOR_BINDING,
        ];
        v.extend(WALL_COLOR_TEXTURES.iter().map(|t| t.1));
        v
    }

    fn erase() -> BrickEraseHighlightInputs {
        BrickEraseHighlightInputs {
            enabled: true,
            cursor_position: None,
            radius: 1.0,
            suppress: false,
        }
    }

    fn run<const LOD: u8>(
        entity: BrickEntity,
        brick_mode: u8,
        lib: &dyn SsboInstanceView,
    ) -> (
        Rig,
        Vec<crate::render::pass_builder::draw_stream::DrawPacket>,
    ) {
        let mut rig = Rig::new(&names(), &[BRICK_MODE_UNIFORM, BRICK_ERASE_ENABLED_UNIFORM]);
        let b = rig.builder();
        indirect_draw_bricks::<LOD>(b, entity, lib, brick_mode, &erase());
        let d = rig.finish();
        (rig, d)
    }

    #[test]
    fn lod0_regular_path_sequence() {
        let (rig, d) = run::<0>(BrickEntity::Proxy(p(1, 0, 0)), 5, &lib());
        let mut want = vec![rg::<BrickDrawListTag>()];
        want.extend(WALL_COLOR_TEXTURES.iter().map(|t| Call::Disk(t.0)));
        want.push(rg::<
            crate::render::pass_builder::bind_resources::WallColorIdSsbo,
        >());
        want.push(Call::Disk(0x7db9e7483fa24078));
        want.push(rg::<RoofColorIdTag>());
        want.push(rg::<DrawIndexedIndirectBuffer>());
        assert_eq!(rig.calls(), want);
        let buf = rig.rg_id(DrawIndexedIndirectBuffer::rg_buffer_hash());
        // brick_mode 5 且 b0 bit0 = 1 → bricks_mode = 5 → idx 0x11 + 2 = 0x13；表里 [0x13] = 900。
        assert_eq!(d.len(), 1);
        match &d[0].kind {
            DrawKind::IndexedIndirect {
                buffer,
                arg_48,
                arg_50,
                arg_58,
                ..
            } => {
                assert_eq!(
                    (*buffer, *arg_48, *arg_50, *arg_58),
                    (buf, 0x13 * 0x14, 1, 0)
                );
            }
            k => panic!("{k:?}"),
        }
        assert_eq!(d[0].state.0 & 0x0c, 4, "LOD0 剔除位 = 4");
        assert_eq!(
            d[0].bindings.len(),
            1 + 4 + 1 + 1,
            "BrickTransform（外部缓冲）+ 4 张墙色磁盘纹理 + 砖纹理 + 常量块"
        );
    }

    #[test]
    fn lod0_binds_the_real_sparse_brick_library() {
        use crate::{
            asset_libraries::ssbo_library::AssetSsboLibrary,
            geometry::instanced_wall::BrickTransformSsbo, render::test_device::TestDevice,
            systems::render_loop::draw_passes::draw_callbacks_a::LibView,
        };
        let library = AssetSsboLibrary::new();
        library.add_new::<BrickTransformSsbo>(&TestDevice(1));
        let (_, packets) = run::<0>(BrickEntity::Proxy(p(0, 0, 0)), 0, &LibView(&library));
        assert_eq!(
            packets.len(),
            1,
            "真实稀疏缓冲可以进入砖绘制，不再因实例视图缺失而 panic"
        );
    }

    #[test]
    fn lod0_discards_when_the_mode_byte_or_proxy_says_so() {
        let (rig, d) = run::<0>(BrickEntity::Proxy(p(0, 1, 0)), 0, &lib());
        assert!(d.is_empty() && rig.calls().is_empty(), "判定在任何绑定之前");
        // brick_mode == 0 && b0 != 0：绑过两项后才 discard。
        let (rig, d) = run::<0>(BrickEntity::Proxy(p(1, 0, 0)), 0, &lib());
        assert!(d.is_empty());
        assert_eq!(rig.calls(), vec![rg::<BrickDrawListTag>()]);
    }

    #[test]
    fn lod1_proxy_path_draws_right_after_the_two_buffers() {
        for (mode, b2, idx) in [(2u8, 0u8, 0x15u32), (3, 1, 0x16), (4, 2, 0x17)] {
            let mut rig = Rig::new(&names(), &[BRICK_MODE_UNIFORM]);
            indirect_draw_bricks_with_mode::<1>(
                rig.builder(),
                mode,
                BrickEntity::Proxy(p(1, 1, b2)),
                &lib(),
                9,
                &erase(),
            );
            assert_eq!(
                rig.calls(),
                vec![rg::<BrickDrawListTag>(), rg::<DrawIndexedIndirectBuffer>()],
                "只有变换缓冲（外部）+ 绘制列表 + 间接缓冲"
            );
            let d = rig.finish();
            match &d[0].kind {
                DrawKind::IndexedIndirect {
                    arg_48,
                    arg_50,
                    arg_58,
                    ..
                } => {
                    assert_eq!(
                        (*arg_48, *arg_50, *arg_58),
                        (idx as u64 * 0x14, 1, 0),
                        "mode {mode}"
                    );
                }
                k => panic!("{k:?}"),
            }
            assert_eq!(d[0].state.0 & 0x0c, 0);
            assert_eq!(
                d[0].bindings.len(),
                2,
                "BrickTransform 外部缓冲 + 常量块（DrawBuilder::new 插入）"
            );
        }
    }

    #[test]
    fn lod1_discards_proxies_in_the_wrong_mode() {
        let mut rig = Rig::new(&names(), &[BRICK_MODE_UNIFORM]);
        indirect_draw_bricks_with_mode::<1>(
            rig.builder(),
            4,
            BrickEntity::Proxy(p(0, 0, 0)),
            &lib(),
            0,
            &erase(),
        );
        assert!(rig.finish().is_empty() && rig.calls().is_empty());
    }

    #[test]
    fn lod1_regular_path_uses_base_0x12_and_clears_the_cull_bits() {
        let (_, d) = run::<1>(BrickEntity::Proxy(p(0, 0, 0)), 0, &lib());
        match &d[0].kind {
            DrawKind::IndexedIndirect { arg_48, .. } => assert_eq!(*arg_48, 0x12 * 0x14),
            k => panic!("{k:?}"),
        }
        assert_eq!(d[0].state.0 & 0x0c, 0, "LOD1：state & 0xf3");
        let (_, d) = run::<1>(BrickEntity::Proxy(p(1, 0, 0)), 2, &lib());
        match &d[0].kind {
            DrawKind::IndexedIndirect { arg_48, .. } => {
                assert_eq!(*arg_48, 0x14 * 0x14, "bricks_mode != 0 → 0x14")
            }
            k => panic!("{k:?}"),
        }
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn missing_entity_panics_like_result_unwrap() {
        let _ = run::<0>(BrickEntity::Missing, 0, &lib());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_proxy_panics_like_option_unwrap() {
        let _ = run::<0>(BrickEntity::NoProxy, 0, &lib());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_transform_buffer_panics() {
        let _ = run::<0>(BrickEntity::Proxy(p(0, 0, 0)), 0, &Lib(None));
    }

    #[test]
    fn proxy_index_table_is_the_jump_table() {
        assert_eq!(BRICK_PROXY_INDEX, [0x15, 0x16, 0x17]);
        assert_eq!(BRICK_BASE_INDEX, [0x11, 0x12]);
        assert_eq!(
            BrickTransformTag::TYPE_NAME.len(),
            0x3a,
            "O：cmp qword ptr [r14+0x18], 0x3a"
        );
    }
}
