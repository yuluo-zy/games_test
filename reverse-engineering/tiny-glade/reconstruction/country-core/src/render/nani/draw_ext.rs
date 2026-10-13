//! draw_ext.rs — DrawBuilder 的 nani 扩展（B 阶段按机器码重写）。
//!
//! 机器码锚点（B patched EXE，cf048e27…）：
//! - `draw_ext::impl$0::nani_draw` 0x140904ae0（1428 B；PDB 过程表 size=1428）。
//!
//! 标注：**O**=机器码观测；**I**=推断；**U**=未知（typed gap）。
//!
//! ## `nani_draw(self: DrawBuilder, kind: GpuWorldSubsetKind) -> DrawBuilder` 逐步还原（O）
//! 1. 经 `self.context(+0xe0)->world(+0x1f0)` 取 ECS `World`，用 `Components::get_resource_id`
//!    （type-hash `0x662978458699a5ef/0x3ec057c162419b46`）取 `GpuWorld`；缺失 → `Option::unwrap` panic。
//! 2. 取 `GpuWorld.subsets[kind]`（槽 stride 0xc0）的 **读锁**（`RwLock::read`，慢路径
//!    `lock_shared_slow`）；槽为 `None`（哨兵 `0x8000000000000001`）→ `Option::unwrap` panic。
//! 3. subset 是 Owned/External 之一（`neg` 溢出判别）：取该变体里的 `u64`（`+0x8` 或 `+0xa8` 后 `+8`）
//!    作为 `draw_count`（O）；Owned 时另经 vtable `+0x38` 取实例间接缓冲的 RG 句柄
//!    （`Option`；External 为 `None`）。
//! 4. `list_index = kind*4 | PASS_VARIANT_TABLE[self.pass_variant(+0xf0)]`；
//!    `cmd_slot = GpuWorld.list_index_to_cmd_offset[list_index]`（越界 → `panic_bounds_check`）。
//! 5. 取 `AssetSsboLibrary`（type-hash `-0x7dd9338791986cab/0x65cfda6a55a841ac`），分别
//!    `get_rg_buffer::<NaniDrawListLengthSsbo>`（→ 计数缓冲）与
//!    `get_rg_buffer::<NaniDrawListSsbo>`（→ 间接参数缓冲）。
//! 6. `DrawBuilder` 链：`read_storage_buffer::<NaniInstanceIndirectionSsbo>()`；若 3 中句柄存在 →
//!    `read_rg_storage_buffer(NANI_INSTANCE_INDIRECTION_RG_HASH, handle)`。
//! 7. 间接参数偏移 `cmd_slot * 20`（`VkDrawIndexedIndirectCommand` 20B 步长，`r9d = 0x14`）。
//! 8. 分派：`count_path = FORCE_FINE_GRAINED_SORTING (bss 0x143634d9c) != 0
//!    || (kind <= 1 && FORCE_NO_FINE_GRAINED_SORTING (0x143634d9d) == 0)`。
//!    - count 路径：`draw_indexed_indirect_count(args_buf, offset, stride=0x14, count_buf,
//!      count_offset = list_index*4, max_draw_count = MAX_DRAWS_PER_LIST[kind])`。
//!    - raw 路径：`draw_indexed_indirect_raw(args_buf, offset, stride=0x14, draw_count)`。
//! 9. 释放读锁（`release_resource`×2 + `unlock_shared_slow` 条件）。
//!
//! ## 与 A4 `DrawBuilder<'p>` 的对齐（B_i_nani）
//! - `draw_*` 是**终结操作**（返回 `()`，取走 packet 压入 `RasterPass.draws`），故 `nani_draw` 也返回 `()`。
//! - `read_storage_buffer<T>(slot_name, binding_hash)`：槽名 = `"NaniInstanceIndirectionSsbo"`
//!   （`read_access_buffer<…>` 0x140972320 的 `rustc_entry<&str>` 键，0x142c8151d），binding 名 hash 为
//!   `r8 = -0x32063c9595637380`；`read_rg_storage_buffer(name_hash, buffer_resource)`。
//! - `draw_indexed_indirect_raw(res, arg_48 = r8, arg_58 = r9, arg_50 = 第 5 参数)`：O 调用点
//!   `r8 = cmd_slot*20`、`r9 = 0x14`、`[rsp+0x20] = draw_count`。
//! - `draw_indexed_indirect_count(draw_res, count_res, [a0..a3])`：写入 packet `+0x50/+0x58/+0x60/+0x68`；
//!   O 调用点 `r8 = cmd_slot*20`（+0x50）、`r9 = 0x14`（+0x58）、`[rsp+0x28] = list_index*4`（+0x60）、
//!   `[rsp+0x30] = MAX_DRAWS_PER_LIST[kind]`（+0x68）。
//! - 原版经 `RasterPass +0x1f0` 的 `World` 取 `GpuWorld`/`AssetSsboLibrary`；`PassHost` 没有这两个资源的
//!   访问器（A4 缺口），所以仍以显式参数传入。

#![allow(dead_code, unused_imports)]

use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::render::nani::buffers::{
    NaniDrawListLengthSsbo, NaniDrawListSsbo, NaniInstanceIndirectionSsbo,
};
use crate::render::nani::gpu_world::{GpuWorld, GpuWorldSubset, GpuWorldSubsetKind};
use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::rhapsody_types::RenderGraph;

/// `VkDrawIndexedIndirectCommand` 步长（O：`mov r9d, 0x14`）。
pub const DRAW_INDEXED_INDIRECT_STRIDE: u32 = 0x14;

/// pass 变体 → draw-list 变体位（O：立即数 `0x302010000` 右移 `8*pass_variant` 取低字节）。
/// 字节序列（低→高）：`[0, 0, 1, 2, 3]`。
pub const PASS_VARIANT_TABLE: [u8; 5] = [0, 0, 1, 2, 3];

/// 每 kind 每 draw-list 最大 draw 数（O：`.rdata 0x142aab288`，27×u64，实读）。
pub const MAX_DRAWS_PER_LIST: [u64; 27] = [
    8192, 8192, 4096, 32768, 8192, 8192, 8192, 8192, 262144, 40960, 1024, 53120, 9600, 131072, 512,
    8192, 4096, 1024, 32768, 65536, 2048, 1024, 2048, 131072, 81920, 256, 32768,
];

/// `NANI_FORCE_FINE_GRAINED_SORTING`（O：bss `0x143634d9c` 的运行期 bool 静态量，默认 false）。
pub const NANI_FORCE_FINE_GRAINED_SORTING: bool = false;
/// `NANI_FORCE_NO_FINE_GRAINED_SORTING`（O：bss `0x143634d9d`，默认 false）。
pub const NANI_FORCE_NO_FINE_GRAINED_SORTING: bool = false;

/// `read_storage_buffer<NaniInstanceIndirectionSsbo>` 的槽名（O：0x142c8151d）。
pub const NANI_INSTANCE_INDIRECTION_SLOT_NAME: &str = "NaniInstanceIndirectionSsbo";
/// 实例间接缓冲 RG 句柄哈希（O：`read_rg_storage_buffer` 的 `r8` 立即数 `0x2106412d631790c6`）。
pub const NANI_INSTANCE_INDIRECTION_RG_HASH: u64 = 0x2106412d631790c6;
/// `read_storage_buffer::<NaniInstanceIndirectionSsbo>` 的类型哈希立即数（O：`-0x32063c9595637380`）。
pub const NANI_INSTANCE_INDIRECTION_SSBO_HASH: u64 = 0xcdf9c36a6a9c8c80;

/// 间接绘制分派选择（O，步骤 8）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NaniIndirectMode {
    /// `draw_indexed_indirect_count`（mode 8；细粒度排序 kind）。
    Count,
    /// `draw_indexed_indirect_raw`（mode 7）。
    Raw,
}

/// `nani_draw` 的纯决策结果（不依赖 ECS/GPU，便于断言机器码常量）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaniDrawPlan {
    /// `kind*4 | PASS_VARIANT_TABLE[pass_variant]`（O）。
    pub list_index: u32,
    /// 计数缓冲内字节偏移 = `list_index * 4`（O：`shl ebx,2`）。
    pub count_offset_bytes: u64,
    /// 间接参数缓冲内字节偏移 = `cmd_slot * 20`（O）。
    pub args_offset_bytes: u64,
    pub mode: NaniIndirectMode,
    /// count 路径的 `max_draw_count`（O：`MAX_DRAWS_PER_LIST[kind]`）；raw 路径为 `None`。
    pub max_draw_count: Option<u64>,
}

/// 是否走 count 路径（O，见模块注释步骤 8）。
pub fn uses_indirect_count(
    kind: GpuWorldSubsetKind,
    force_fine: bool,
    force_no_fine: bool,
) -> bool {
    force_fine || ((kind as u8) <= 1 && !force_no_fine)
}

/// `list_index` 计算（O：`lea eax,[4*rsi]; or al,bl; movzx ebx,al`——结果截断为 u8）。
/// `pass_variant` 越过 `PASS_VARIANT_TABLE`（>=5）时机器码 `shr` 取到 0（高位字节为 0）。
pub fn nani_list_index(kind: GpuWorldSubsetKind, pass_variant: u8) -> u32 {
    let variant_bits = if (pass_variant as usize) < PASS_VARIANT_TABLE.len() {
        PASS_VARIANT_TABLE[pass_variant as usize]
    } else {
        0
    };
    (((kind as u8 as u32) << 2) | variant_bits as u32) & 0xff
}

/// 纯决策：由 `cmd_slot`（`list_index_to_cmd_offset[list_index]`）得出偏移与分派（O）。
pub fn nani_draw_plan(
    kind: GpuWorldSubsetKind,
    pass_variant: u8,
    cmd_slot: u32,
    force_fine: bool,
    force_no_fine: bool,
) -> NaniDrawPlan {
    let list_index = nani_list_index(kind, pass_variant);
    let count = uses_indirect_count(kind, force_fine, force_no_fine);
    NaniDrawPlan {
        list_index,
        count_offset_bytes: (list_index as u64) << 2,
        args_offset_bytes: cmd_slot as u64 * DRAW_INDEXED_INDIRECT_STRIDE as u64,
        mode: if count {
            NaniIndirectMode::Count
        } else {
            NaniIndirectMode::Raw
        },
        max_draw_count: if count {
            Some(MAX_DRAWS_PER_LIST[kind as usize])
        } else {
            None
        },
    }
}

/// nani 绘制扩展（`impl$0::nani_draw`，O）。
pub trait NaniDrawExt {
    /// `nani_draw`（0x140904ae0）。终结操作（见模块注释）。
    fn nani_draw(
        self,
        kind: GpuWorldSubsetKind,
        gpu_world: &GpuWorld,
        ssbo: &AssetSsboLibrary,
        rg: &mut RenderGraph,
    );
}

impl NaniDrawExt for DrawBuilder<'_> {
    fn nani_draw(
        self,
        kind: GpuWorldSubsetKind,
        gpu_world: &GpuWorld,
        ssbo: &AssetSsboLibrary,
        rg: &mut RenderGraph,
    ) {
        // 步骤 2：读锁 + `Option::unwrap`（槽空 → panic，O）。
        let guard = gpu_world.get_subset(kind);
        let subset = guard
            .as_ref()
            .expect("called `Option::unwrap()` on a `None` value");

        // 步骤 3：draw_count 与实例间接缓冲 RG 句柄（Owned 才有，O：vtable +0x38 → `rg_handle`）。
        let (draw_count, indirection_handle) = match subset {
            GpuWorldSubset::Owned(owned) => (
                owned.instance_count() as u64,
                Some(owned.data().rg_handle().resource as u32),
            ),
            // U：External 变体的 draw_count 字段（`+0xa8` 后 `+8`）在重建类型里尚无访问器。
            GpuWorldSubset::External(_) => (0, None),
        };

        // 步骤 4：list_index / cmd_slot。
        let pass_variant = self.variant();
        let list_index = nani_list_index(kind, pass_variant);
        let cmd_slot = gpu_world.list_index_to_cmd_offset(list_index as usize);

        // 步骤 5：两个 RG 缓冲（先 DrawListLength，再 DrawList，O 调用顺序）。
        let count_buf = ssbo.get_rg_buffer::<NaniDrawListLengthSsbo>(rg);
        let args_buf = ssbo.get_rg_buffer::<NaniDrawListSsbo>(rg);

        // 步骤 6：builder 链。
        let mut builder = self.read_storage_buffer::<NaniInstanceIndirectionSsbo>(
            NANI_INSTANCE_INDIRECTION_SLOT_NAME,
            NANI_INSTANCE_INDIRECTION_SSBO_HASH,
        );
        if let Some(handle) = indirection_handle {
            builder = builder.read_rg_storage_buffer(NANI_INSTANCE_INDIRECTION_RG_HASH, handle);
        }

        // 步骤 7/8：分派。
        let plan = nani_draw_plan(
            kind,
            pass_variant,
            cmd_slot,
            NANI_FORCE_FINE_GRAINED_SORTING,
            NANI_FORCE_NO_FINE_GRAINED_SORTING,
        );
        match plan.mode {
            NaniIndirectMode::Count => builder.draw_indexed_indirect_count(
                args_buf.resource as u32,
                count_buf.resource as u32,
                [
                    plan.args_offset_bytes,
                    DRAW_INDEXED_INDIRECT_STRIDE as u64,
                    plan.count_offset_bytes,
                    plan.max_draw_count
                        .expect("count path always carries max_draw_count"),
                ],
            ),
            NaniIndirectMode::Raw => builder.draw_indexed_indirect_raw(
                args_buf.resource as u32,
                plan.args_offset_bytes,
                DRAW_INDEXED_INDIRECT_STRIDE as u64,
                draw_count,
            ),
        }
    }
}

/// 引用类型：仅用于名称对齐（GpuWorld 资源类型存在于 ECS）。
pub type GpuWorldResource = GpuWorld;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_variant_table_matches_immediate() {
        // 0x302010000 的字节 = 00 00 01 02 03
        let imm: u64 = 0x3_0201_0000;
        for (i, b) in PASS_VARIANT_TABLE.iter().enumerate() {
            assert_eq!(*b as u64, (imm >> (8 * i)) & 0xff);
        }
    }

    #[test]
    fn list_index_packs_kind_and_variant() {
        assert_eq!(nani_list_index(GpuWorldSubsetKind::Canopy, 0), 0);
        assert_eq!(nani_list_index(GpuWorldSubsetKind::Canopy, 1), 0);
        assert_eq!(nani_list_index(GpuWorldSubsetKind::Canopy, 2), 1);
        assert_eq!(nani_list_index(GpuWorldSubsetKind::TreeTrunk, 4), 4 | 3);
        assert_eq!(
            nani_list_index(GpuWorldSubsetKind::SolidVertexColor, 3),
            12 | 2
        );
        // 越界 pass_variant：移位后为 0
        assert_eq!(nani_list_index(GpuWorldSubsetKind::SolidVertexColor, 9), 12);
    }

    #[test]
    fn dispatch_branching() {
        // kind<=1 且未强制 no-fine → count；其余 raw。
        assert!(uses_indirect_count(
            GpuWorldSubsetKind::Canopy,
            false,
            false
        ));
        assert!(uses_indirect_count(
            GpuWorldSubsetKind::TreeTrunk,
            false,
            false
        ));
        assert!(!uses_indirect_count(
            GpuWorldSubsetKind::CurvedHalftimber,
            false,
            false
        ));
        assert!(!uses_indirect_count(
            GpuWorldSubsetKind::Canopy,
            false,
            true
        ));
        assert!(uses_indirect_count(
            GpuWorldSubsetKind::StairsFloor,
            true,
            false
        ));
        assert!(uses_indirect_count(GpuWorldSubsetKind::Canopy, true, true));
    }

    #[test]
    fn plan_offsets_and_max_count() {
        let p = nani_draw_plan(GpuWorldSubsetKind::TreeTrunk, 3, 7, false, false);
        assert_eq!(p.list_index, 4 | 2);
        assert_eq!(p.count_offset_bytes, 6 * 4);
        assert_eq!(p.args_offset_bytes, 7 * 20);
        assert_eq!(p.mode, NaniIndirectMode::Count);
        assert_eq!(p.max_draw_count, Some(8192));
        let q = nani_draw_plan(GpuWorldSubsetKind::WoodenPlank, 0, 1, false, false);
        assert_eq!(q.mode, NaniIndirectMode::Raw);
        assert_eq!(q.max_draw_count, None);
    }

    #[test]
    fn max_draws_table_is_27_entries_from_rdata() {
        assert_eq!(MAX_DRAWS_PER_LIST.len(), 27);
        assert_eq!(MAX_DRAWS_PER_LIST[8], 262144);
        assert_eq!(MAX_DRAWS_PER_LIST[25], 256);
    }

    // ---- 真实 `DrawBuilder<'p>` / `GpuWorld` / `AssetSsboLibrary` 的端到端测试 ----

    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::render::nani::gpu_world::{InstanceCullData, OwnedGpuWorldSubset};
    use crate::render::nani::instance_data::PlantInstanceData;
    use crate::render::pass_builder::draw_stream::test_host::MockHost;
    use crate::render::pass_builder::draw_stream::{
        DrawKind, DrawPacket, RasterPass, ShaderReflection,
    };
    use crate::render::pass_builder::pass_resource_access::SsboType;
    use crate::render::shader::RasterShaderProgram;
    use crate::render::ssbo::tests::MockDevice;
    use crate::rhapsody_contract::{Handle, ManagedHandle};
    use alloc::sync::Arc;
    use glam::Vec3;
    use hashbrown::HashMap;

    const BINDING_SLOT: u32 = 3;

    fn pass() -> RasterPass {
        let mut bindings = HashMap::new();
        bindings.insert(NANI_INSTANCE_INDIRECTION_SSBO_HASH, (0u32, BINDING_SLOT));
        bindings.insert(NANI_INSTANCE_INDIRECTION_RG_HASH, (0u32, BINDING_SLOT + 1));
        let refl = ShaderReflection {
            program_name: "nani_test",
            bindings,
            has_uniform_block: false,
            uniform_offsets: HashMap::new(),
            uniform_block_size: 0,
            uniform_block_binding: 0,
        };
        let (mut host, _log) = MockHost::new();
        host.shaders.insert(1, Arc::new(refl));
        host.rg_buffers
            .insert(NaniInstanceIndirectionSsbo::rg_buffer_hash(), 77);
        RasterPass::new("nani_test", &[], None, host.boxed())
    }

    fn builder(pass: &mut RasterPass, variant: u8) -> DrawBuilder<'_> {
        let st = pass.host.default_raster_state();
        let h = Handle::<RasterShaderProgram>(ManagedHandle(1), core::marker::PhantomData);
        DrawBuilder::new(pass, DrawPacket::new(h, [0, 1], st), variant)
    }

    fn world_and_library(
        kind: GpuWorldSubsetKind,
        instances: usize,
    ) -> (GpuWorld, AssetSsboLibrary, RenderGraph) {
        let dev = MockDevice::default();
        let mut gw = GpuWorld::new(&dev);
        let mut sub = OwnedGpuWorldSubset::new::<PlantInstanceData>(&dev, kind);
        let items = vec![InstanceCullData::new(Vec3::ZERO, 1.0, 0); instances];
        sub.extend_no_data(kind, &items, |_| {});
        gw.assign_subset(kind, GpuWorldSubset::Owned(sub));
        let mut rg = RenderGraph::default();
        gw.import_to_rg(&mut rg); // `rg_handle()` 只能在 import 之后调用（原版 expect 文案）。
        let lib = AssetSsboLibrary::new();
        lib.add_new::<NaniDrawListLengthSsbo>(&dev);
        lib.add_new::<NaniDrawListSsbo>(&dev);
        (gw, lib, rg)
    }

    #[test]
    fn nani_draw_raw_path_uses_draw_count_stride_and_rg_handle() {
        let kind = GpuWorldSubsetKind::WoodenPlank; // kind > 1 且未强制 → raw
        let (gw, lib, mut rg) = world_and_library(kind, 3);
        let mut p = pass();
        builder(&mut p, 0).nani_draw(kind, &gw, &lib, &mut rg);
        assert_eq!(
            p.draws.len(),
            1,
            "draw_* 是终结操作：packet 已压入 pass.draws"
        );
        let packet = &p.draws[0];
        let cmd_slot = gw.list_index_to_cmd_offset(nani_list_index(kind, 0) as usize);
        match &packet.kind {
            DrawKind::IndexedIndirect {
                arg_48,
                arg_50,
                arg_58,
                first_index_count,
                ..
            } => {
                assert_eq!(*arg_48, cmd_slot as u64 * 20, "r8 = cmd_slot*20");
                assert_eq!(*arg_58, 0x14, "r9d = 0x14");
                assert_eq!(*arg_50, 3, "[rsp+0x20] = 实例数");
                assert_eq!(*first_index_count, None);
            }
            other => panic!("expected IndexedIndirect, got {other:?}"),
        }
        // read_storage_buffer 登记 1 次访问 + read_rg_storage_buffer 登记 1 次 + 间接参数/计数缓冲。
        assert!(
            p.accesses.iter().any(|a| a.resource == 77),
            "间接寻址 SSBO 的 RG 资源"
        );
        p.rendered.0 = true;
    }

    #[test]
    fn nani_draw_count_path_packs_max_draw_count_and_count_offset() {
        let kind = GpuWorldSubsetKind::TreeTrunk; // kind <= 1 → count
        let (gw, lib, mut rg) = world_and_library(kind, 1);
        let mut p = pass();
        builder(&mut p, 3).nani_draw(kind, &gw, &lib, &mut rg);
        let list_index = nani_list_index(kind, 3);
        assert_eq!(list_index, (1 << 2) | 2);
        let cmd_slot = gw.list_index_to_cmd_offset(list_index as usize);
        match &p.draws[0].kind {
            DrawKind::IndexedIndirectCount { args, .. } => {
                assert_eq!(
                    *args,
                    [
                        cmd_slot as u64 * 20,
                        0x14,
                        list_index as u64 * 4,
                        MAX_DRAWS_PER_LIST[1]
                    ]
                );
            }
            other => panic!("expected IndexedIndirectCount, got {other:?}"),
        }
        p.rendered.0 = true;
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn nani_draw_with_empty_slot_panics() {
        let dev = MockDevice::default();
        let gw = GpuWorld::new(&dev);
        let lib = AssetSsboLibrary::new();
        let mut rg = RenderGraph::default();
        let mut p = pass();
        builder(&mut p, 0).nani_draw(GpuWorldSubsetKind::WoodenPlank, &gw, &lib, &mut rg);
    }
}
