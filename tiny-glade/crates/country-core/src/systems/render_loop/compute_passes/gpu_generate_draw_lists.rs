//! `gpu_generate_draw_lists.rs` — Nani GPU 驱动 draw-list 生成（W22/W28）+ 墙 draw-list（W21/W27）。
//!
//! # 来源（O）
//! `gpu_generate_draw_lists` 0x140987dd0（11539 B）、`generate_wall_draw_lists` 0x14098aaf0、`…_pass2` 0x14098b2a0、
//! `create_cull_and_bucket_pass<T>` 0x140a28630/28950/28c60/28f70、`cull_and_bucket_external<S,C>` 0x140a27b10/280a0、
//! `generate_draw_lists`/`group_draw_cmd_prefix_sum`/`prefix_sum_unsorted_instances_per_atlas_entry`/
//! `fill_sorted_instance_list`（均对 `RenderSublistsIter` 单态化，0x140a29280..0x140a29930）及其 dispatch 闭包
//! （0x1415f8d10..0x1415fbc50）。HLSL：`shader-source/shaders/{nani,wall}/*.cs.hlsl`；绑定名由 FNV-1a 反解。
//!
//! # 已恢复（O）
//! * 固定形状的 pass：`"Nani clear draw lists"`（`dispatch_1d(108)`）、`"Nani clear histograms"`
//!   （`dispatch_1d(0x1b000)`）、`"Draw -> dispatch args"`（`dispatch_1d(1)`，写 `args_buffer`，源 `draw_buffer`）、
//!   `"Nani histogram prefix"`（`dispatch_1d(0xd800)`）。`.rdata` 描述符 `{1,108,1,1}`/`{1,110592,1,1}`/`{1,1,1,1}`/`{1,55296,1,1}`。
//! * `"Nani cull & bucket"` 的 11 步绑定序（见 [`create_cull_and_bucket_pass`]）与 dispatch 闭包（[`cull_and_bucket_dispatch`]：
//!   对 `Vec<(PushConstants[20 B], u32)>` 逐项 `push_constants(20 B)` + `dispatch_3d(count, 1, 1)`；O 0x1415fa2f0）。
//! * `RenderSublistsIter` 之后的 4 个 pass 的绑定序与 RG 缓冲名（[`generate_draw_lists_pass`] 等），dispatch 的计数来自
//!   GpuWorld 的各子集实例数（原版 `RenderSublistsIter`，这里作为参数）。
//! * 墙 draw list：W21 `"Generate wall draw lists"`（`cs_pass1`）、W27 `"Wall pass2 reset inst counts"` +
//!   `"Generate wall draw lists OC/2"`（`cs_pass2`）的绑定序。
//! * 主函数 RG 资源：`"Nani list sorting histogram"`（`0x6c000` B）、`"Nani filtered instance meta"`、
//!   `"Dispatch args"`（`0x10 * n`）等名字与创建顺序（[`GPU_GENERATE_RG_RESOURCES`]）。
//!
//! # 未恢复 / typed gap（U）
//! * `GpuWorldSubsetKind` 的 27 个变体与每子集的 `PushConstants` 5 个 u32 的含义、`gpu_world_subset` 的跳表目的地
//!   （`0x14156b970` 等 `FnOnce` 闭包）——依赖 `render::nani::gpu_world`（他人所有）。
//! * `RenderSublistsIter` 内部的 sublist 迭代与 `dispatch_indirect_with` 偏移（由 `IndirectArgsType::buffer_offset` 决定，U）。
//! * 墙实例经 `UnsafeWorldCell::get_entity` 的收集与 `SparseInstanceBuffer::{hide,show}`（属 `geometry::instanced_wall`）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::redundant_closure, clippy::too_many_arguments)]
#![allow(dead_code, unused_imports)]

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::compute_pass_builder::{
    ComputeDispatchApi, DispatchArgs, RecordedComputePass, RgBuffer,
};
use crate::render::pass_builder::pass_resource_access::RgTexture;
use crate::render::pass_builder::shader_uri::AsComputeShaderSource;
use crate::typed_gap::Gap;

use super::pass_kit::{PassEnv, binding_hash, imm};

// ---------------------------------------------------------------- 常量

/// RG 直方图缓冲大小（O：`BufferDesc::new_gpu_only(0x6c000, 0)`）。
pub const LIST_SORTING_HISTOGRAM_BYTES: u64 = 0x6c000;
pub const CLEAR_DRAW_LISTS_THREADS: u32 = 108;
pub const CLEAR_HISTOGRAM_THREADS: u32 = 110_592;
pub const HISTOGRAM_PREFIX_THREADS: u32 = 55_296;
pub const DRAW_TO_DISPATCH_THREADS: u32 = 1;
/// `cull_and_bucket` 的每子集 push constants（O：20 B = 5 个 u32，`push_constants(.., 0x14)`）。
pub const PUSH_CONSTANTS_BYTES: usize = 20;
/// 间接参数槽编号（O：`draw_indexed_indirect_to_dispatch` 前的立即数 `0x400000000b` / `0x400000000a`：低 32 位）。
pub const ARGS_SLOT_WOODEN_PLANK: u32 = 0xb;
pub const ARGS_SLOT_ROOF_TILE: u32 = 0xa;
/// 全局"每类使能"字节（O：`cmp byte ptr [0x143634d9c], 0`，`generate_draw_lists` 跳表分支）。
pub const KIND_ENABLE_FLAG_ADDR: u64 = 0x1_4363_4d9c;
/// W27/W28 的门控（O：`*0x14362c0d8 == 0`）。
pub const ROUND_GATE_ADDR: u64 = 0x1_4362_c0d8;

pub const GPU_GENERATE_RG_RESOURCES: [&str; 5] = [
    "Nani list sorting histogram",
    "Nani filtered instance meta",
    "Nani draw count per thread group",
    "Dispatch args",
    "Nani sorted instances",
];

// ---------------------------------------------------------------- 绑定名

pub const H_INSTANCE_LIST_OFFSET: u64 = binding_hash("instance_list_offset");
pub const H_DRAW_LIST_LENGTH_BUFFER: u64 = binding_hash("draw_list_length_buffer");
pub const H_INSTANCE_INDIRECTION_ALLOCATOR: u64 =
    binding_hash("instance_indirection_allocator_buffer");
pub const H_FILTERED_INSTANCE_ALLOCATOR: u64 = binding_hash("filtered_instance_allocator_buffer");
pub const H_HISTOGRAM: u64 = binding_hash("histogram");
pub const H_ATLAS_PART_INFO: u64 = binding_hash("atlas_part_info");
pub const H_DRAW_LIST_BUFFER: u64 = binding_hash("draw_list_buffer");
pub const H_DRAW_BUFFER: u64 = binding_hash("draw_buffer");
pub const H_ARGS_BUFFER: u64 = binding_hash("args_buffer");
pub const H_FILTERED_INSTANCE_BUFFER: u64 = binding_hash("filtered_instance_buffer");
pub const H_FILTERED_INSTANCE_META_BUFFER: u64 = binding_hash("filtered_instance_meta_buffer");
pub const H_OUTPUT_HISTOGRAM: u64 = binding_hash("output_histogram");
pub const H_DRAW_LIST_INFO: u64 = binding_hash("draw_list_info");
pub const H_PREV_VISIBILITY: u64 = binding_hash("prev_visibility_bitmask");
pub const H_NEXT_VISIBILITY: u64 = binding_hash("next_visibility_bitmask");
pub const H_HZB_TEX: u64 = binding_hash("hzb_tex");
pub const H_INOUT_BUF: u64 = binding_hash("inout_buf");
pub const H_SUBTOTALS_BUF: u64 = binding_hash("subtotals_buf");
pub const H_SORTED_INSTANCES_BUFFER: u64 = binding_hash("sorted_instances_buffer");
pub const H_INSTANCE_INDIRECTION_BUFFER: u64 = binding_hash("instance_indirection_buffer");
pub const H_THREAD_GROUP_DRAW_CMD_HISTOGRAM: u64 = binding_hash("thread_group_draw_cmd_histogram");
pub const H_THREAD_GROUP_DRAW_CMD_SUBTOTALS: u64 = binding_hash("thread_group_draw_cmd_subtotals");
pub const H_THREAD_COUNT_BUFFER: u64 = binding_hash("thread_count_buffer");
pub const H_UNSORTED_ATLAS_PREFIX: u64 =
    binding_hash("unsorted_instance_atlas_entry_prefix_buffer");
pub const H_INSTANCE_BUFFER: u64 = binding_hash("instance_buffer");
pub const H_BRICK_INSTANCE_DATA: u64 = binding_hash("brick_instance_data");
pub const H_BRICK_DRAW_LIST: u64 = binding_hash("brick_draw_list");
pub const H_DRAW_COMMANDS: u64 = binding_hash("draw_commands");
pub const H_DRAW_INDIRECT_BUFFER_OFFSET: u64 = binding_hash("draw_indirect_buffer_offset");
/// `hzb_tex` 之后的 uniform（O：`write_uniform<[u32;2]>`，键 `0xe8d23613960341d3`，名字 U；`hzb_tex` 维度字段）。
pub const H_HZB_DIMS: u64 = 0xe8d2_3613_9603_41d3;

pub const ACCESS_READ: u8 = 0x19;
/// `write_rg_storage_buffer_concurrent` 等并发写的同步组（O：`r9d` 由调用点给；`cull_and_bucket` 里取 1，I）。
pub const CULL_WRITE_GROUP: u32 = 1;

// ---------------------------------------------------------------- 类型

/// 间接 args 槽（O：`IndirectArgsType::buffer_offset`；变体 U，只固化已观测的两个槽号）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndirectArgsType {
    /// `0x400000000b`：WoodenPlank 外部 cull 后的 `draw_indexed_indirect_to_dispatch`。
    WoodenPlank,
    /// `0x400000000a`：RoofTile。
    RoofTile,
}

impl IndirectArgsType {
    /// 槽号（O 立即数低 32 位）。
    pub fn slot(self) -> u32 {
        match self {
            IndirectArgsType::WoodenPlank => ARGS_SLOT_WOODEN_PLANK,
            IndirectArgsType::RoofTile => ARGS_SLOT_ROOF_TILE,
        }
    }
    /// 槽的字节偏移：每槽 16 B（`DispatchIndirectCommand`，I）。
    pub fn buffer_offset(self) -> u64 {
        self.slot() as u64 * 16
    }
}

/// 每子集 push constants（O：20 B = 5 个 u32；字段含义 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PushConstants(pub [u32; 5]);

impl PushConstants {
    pub fn to_bytes(&self) -> [u8; PUSH_CONSTANTS_BYTES] {
        let mut out = [0u8; PUSH_CONSTANTS_BYTES];
        for (dst, v) in out.chunks_exact_mut(4).zip(self.0) {
            dst.copy_from_slice(&v.to_le_bytes());
        }
        out
    }
}

/// Nani 的 SSBO 类型集合（O：`read/write_storage_buffer<T>` 的 T；真实类型在 `render::nani::buffers`）。
pub trait NaniSsbos {
    /// `NaniDrawListOffsetsSsbo`（`instance_list_offset`，只读）。
    type DrawListOffsets: LibrarySsbo;
    /// `NaniDrawListLengthSsbo`（`draw_list_length_buffer`）。
    type DrawListLength: LibrarySsbo;
    /// `NaniInstanceIndirectionAllocatorSsbo`。
    type InstanceIndirectionAllocator: LibrarySsbo;
    /// `NaniFilteredInstanceAllocatorSsbo`。
    type FilteredInstanceAllocator: LibrarySsbo;
    /// `NaniDrawListSsbo`（`draw_list_buffer`）。
    type DrawList: LibrarySsbo;
    /// `NaniInstanceIndirectionSsbo`。
    type InstanceIndirection: LibrarySsbo;
    /// `NaniFilteredInstanceSsbo`（`filtered_instance_buffer`）。
    type FilteredInstance: LibrarySsbo;
    /// `NaniPrevVisibilityBitmaskSsbo`。
    type PrevVisibility: LibrarySsbo;
    /// `NaniNextVisibilityBitmaskSsbo`。
    type NextVisibility: LibrarySsbo;
    /// `indirect_params::DrawIndirectBuffer`（`draw_buffer`，只读）。
    type DrawBuffer: LibrarySsbo;
}

// ---------------------------------------------------------------- 固定形状 pass

/// `"Nani clear draw lists"`（O：`/nani/clear_draw_lists.cs.hlsl`）。
pub fn clear_draw_lists<S: NaniSsbos>(env: &mut PassEnv<'_>) -> RecordedComputePass {
    env.new_pass("Nani clear draw lists", &"/nani/clear_draw_lists.cs.hlsl")
        .read_storage_buffer::<S::DrawListOffsets>(H_INSTANCE_LIST_OFFSET)
        .write_storage_buffer::<S::DrawListLength>(H_DRAW_LIST_LENGTH_BUFFER)
        .write_storage_buffer::<S::InstanceIndirectionAllocator>(H_INSTANCE_INDIRECTION_ALLOCATOR)
        .write_storage_buffer::<S::FilteredInstanceAllocator>(H_FILTERED_INSTANCE_ALLOCATOR)
        .dispatch_1d(CLEAR_DRAW_LISTS_THREADS)
}

/// `"Nani clear histograms"`（O：`/nani/clear_histogram.cs.hlsl`；写 RG 缓冲 `histogram`）。
pub fn clear_histogram(env: &mut PassEnv<'_>, histogram: &RgBuffer) -> RecordedComputePass {
    env.new_pass("Nani clear histograms", &"/nani/clear_histogram.cs.hlsl")
        .write_rg_storage_buffer(H_HISTOGRAM, histogram)
        .dispatch_1d(CLEAR_HISTOGRAM_THREADS)
}

/// `"Draw -> dispatch args"`（O：`/nani/draw_indexed_indirect_to_dispatch.cs.hlsl`）：
/// 读 `draw_buffer`，写 RG `args_buffer`，push constants = `(槽号, 4)`（O：`0x400000000b` = {0xb, 4}），`dispatch_1d(1)`。
pub fn draw_indexed_indirect_to_dispatch<S: NaniSsbos>(
    env: &mut PassEnv<'_>,
    args: &RgBuffer,
    slot: IndirectArgsType,
) -> RecordedComputePass {
    let mut pc = [0u8; 8];
    pc[..4].copy_from_slice(&slot.slot().to_le_bytes());
    pc[4..].copy_from_slice(&4u32.to_le_bytes());
    env.new_pass(
        "Draw -> dispatch args",
        &"/nani/draw_indexed_indirect_to_dispatch.cs.hlsl",
    )
    .read_storage_buffer::<S::DrawBuffer>(H_DRAW_BUFFER)
    .write_rg_storage_buffer(H_ARGS_BUFFER, args)
    .push_constants(&pc)
    .dispatch_1d(DRAW_TO_DISPATCH_THREADS)
}

/// `"Nani histogram prefix"`（O：`/nani/histogram_prefix_sum.cs.hlsl`）。
pub fn histogram_prefix_sum(env: &mut PassEnv<'_>, inout: &RgBuffer) -> RecordedComputePass {
    env.new_pass(
        "Nani histogram prefix",
        &"/nani/histogram_prefix_sum.cs.hlsl",
    )
    .write_rg_storage_buffer(H_INOUT_BUF, inout)
    .dispatch_1d(HISTOGRAM_PREFIX_THREADS)
}

// ---------------------------------------------------------------- cull & bucket

/// `cull_and_bucket` 的 dispatch 闭包体（O 0x1415fa2f0）：逐项 `push_constants(20 B)` → `dispatch_3d(count,1,1)`。
pub fn cull_and_bucket_dispatch(api: &mut dyn ComputeDispatchApi, items: &[(PushConstants, u32)]) {
    for (pc, count) in items {
        api.push_constants(&pc.to_bytes());
        api.dispatch_3d(DispatchArgs {
            x: *count,
            y: 1,
            z: 1,
            indirect_index: None,
        });
    }
}

/// `create_cull_and_bucket_pass<T>`（O 0x140a28950 等）的外部输入。
pub struct CullBucketResources<'a> {
    pub filtered_instance_meta: &'a RgBuffer,
    pub output_histogram: &'a RgBuffer,
    pub atlas_part_info: &'a ExternalBufferHandle,
    pub draw_list_info: &'a ExternalBufferHandle,
    pub hzb: Option<&'a RgTexture>,
    /// `hzb_tex` 维度 uniform（HZB 变体写）。
    pub hzb_dims: [u32; 2],
}

/// 受管缓冲句柄（`bind_external_storage_buffer` 的参数；A4 的 `ManagedHandle`）。
pub type ExternalBufferHandle = crate::rhapsody_contract::ManagedHandle;

/// `create_cull_and_bucket_pass<T>`（O）：pass 名 `"Nani cull & bucket"`，绑定序：
/// `filtered_instance_allocator_buffer`(并发写) → `filtered_instance_buffer`(并发写) → `filtered_instance_meta_buffer`(RG 并发写)
/// → `output_histogram`(RG 并发写) → `atlas_part_info`(外部) → `draw_list_info`(外部) → `prev_visibility_bitmask`(读)
/// → `next_visibility_bitmask`(并发写) →（HZB 变体）`hzb_tex`(0x19) + `[u32;2]` 维度 uniform。
///
/// （I）反汇编里 `read_storage_buffer<Prev>` 与 `write_storage_buffer_concurrent<Next>` 出现两组（HZB / 非 HZB 变体各一份函数体，
/// 签名分别按 `NaniNextVisibility`/`NaniPrevVisibility` 单态化）；这里统一成一组，仅 `hzb` 为 `Some` 时多绑 `hzb_tex`。
pub fn create_cull_and_bucket_pass<T: AsComputeShaderSource, S: NaniSsbos>(
    env: &mut PassEnv<'_>,
    source: &T,
    res: &CullBucketResources<'_>,
) -> crate::render::pass_builder::compute_pass_builder::ComputePass {
    let mut pass = env
        .new_pass("Nani cull & bucket", source)
        .write_storage_buffer_concurrent::<S::FilteredInstanceAllocator>(
            H_FILTERED_INSTANCE_ALLOCATOR,
            CULL_WRITE_GROUP,
        )
        .write_storage_buffer_concurrent::<S::FilteredInstance>(
            H_FILTERED_INSTANCE_BUFFER,
            CULL_WRITE_GROUP,
        )
        .write_rg_storage_buffer_concurrent(
            H_FILTERED_INSTANCE_META_BUFFER,
            res.filtered_instance_meta,
            CULL_WRITE_GROUP,
        )
        .write_rg_storage_buffer_concurrent(
            H_OUTPUT_HISTOGRAM,
            res.output_histogram,
            CULL_WRITE_GROUP,
        )
        .bind_external_storage_buffer(H_ATLAS_PART_INFO, res.atlas_part_info)
        .bind_external_storage_buffer(H_DRAW_LIST_INFO, res.draw_list_info)
        .read_storage_buffer::<S::PrevVisibility>(H_PREV_VISIBILITY)
        .write_storage_buffer_concurrent::<S::NextVisibility>(H_NEXT_VISIBILITY, CULL_WRITE_GROUP);
    if let Some(hzb) = res.hzb {
        pass = pass
            .bind_texture(H_HZB_TEX, &hzb, ACCESS_READ)
            .uniform(H_HZB_DIMS, res.hzb_dims);
    }
    pass
}

/// `"Nani prepare unsorted draw lists"`（O：`/nani/prepare_unsorted_draw_lists.cs.hlsl`，`dispatch_with`）：
/// 外部绑 `atlas_part_info`（`GpuWorld::mesh_atlas_info_buf`），写 `draw_list_buffer`；每子集 `push_constants` + dispatch。
pub fn prepare_empty_unsorted_draw_lists<S: NaniSsbos>(
    env: &mut PassEnv<'_>,
    atlas_part_info: &ExternalBufferHandle,
    per_subset: Vec<(PushConstants, u32)>,
) -> RecordedComputePass {
    env.new_pass(
        "Nani prepare unsorted draw lists",
        &"/nani/prepare_unsorted_draw_lists.cs.hlsl",
    )
    .bind_external_storage_buffer(H_ATLAS_PART_INFO, atlas_part_info)
    .write_storage_buffer::<S::DrawList>(H_DRAW_LIST_BUFFER)
    .dispatch_with(move |api| cull_and_bucket_dispatch(api, &per_subset))
}

/// `cull_and_bucket_external<SsboT, CullT>`（O 0x140a27b10 / 0x140a280a0）：同 [`create_cull_and_bucket_pass`]
/// （带 `hzb_tex`），再多读一个外部实例缓冲 `instance_buffer`（`SsboT`：WoodenPlank / RoofTile），
/// `dispatch_with`（计数 = 该 SSBO 的实例数，经 `SparseInstanceBuffer::to_draw_multi_instanced`）。
/// 原函数末尾还有一个 `assert_eq!(&str, &str)`（`core::panicking::assert_failed<&str,&str>`，两侧字符串 U，未复刻）。
pub fn cull_and_bucket_external<C: AsComputeShaderSource, Inst: LibrarySsbo, S: NaniSsbos>(
    env: &mut PassEnv<'_>,
    source: &C,
    res: &CullBucketResources<'_>,
    batches: Vec<(PushConstants, u32)>,
) -> RecordedComputePass {
    create_cull_and_bucket_pass::<C, S>(env, source, res)
        .read_storage_buffer::<Inst>(H_INSTANCE_BUFFER)
        .dispatch_with(move |api| cull_and_bucket_dispatch(api, &batches))
}

/// 主函数 `gpu_generate_draw_lists`（O 0x140987dd0）里 pass 的**调用顺序**（O：按调用点；每个子集 / 变体展开处标 `*`）。
/// 其中 `clear_rg_buffer` ×2 清 prev/next 可见性位图。
pub const FRAME_PASS_ORDER: [&str; 13] = [
    "Nani clear draw lists",
    "Nani clear histograms",
    "clear_rg_buffer(NaniPrevVisibilityBitmask)",
    "clear_rg_buffer(NaniNextVisibilityBitmask)",
    "Nani prepare unsorted draw lists",
    "Nani cull & bucket*",
    "Draw -> dispatch args (WoodenPlank, slot 0xb)",
    "Nani cull & bucket (WoodenPlank, external)",
    "Draw -> dispatch args (RoofTile, slot 0xa)",
    "Nani cull & bucket (RoofTile, external)",
    "Nani histogram prefix",
    "Nani unsorted atlas idx prefix sum / fill / generate draws (RenderSublistsIter)",
    "Nani draw cmd histogram prefix / subtotal prefix",
];

// ---------------------------------------------------------------- RenderSublistsIter 之后的 4 个 pass

/// `"Nani generate draws"`（O 0x140a29930，`/nani/generate_draw_lists.cs.hlsl`，`dispatch_indirect_with`）。
pub fn generate_draw_lists_pass<S: NaniSsbos>(
    env: &mut PassEnv<'_>,
    atlas_part_info: &ExternalBufferHandle,
    sorted_instances: &RgBuffer,
    thread_group_histogram: &RgBuffer,
    thread_group_subtotals: &RgBuffer,
    thread_count: &RgBuffer,
    dispatch_args: &RgBuffer,
    args_offset: u64,
) -> RecordedComputePass {
    env.new_pass("Nani generate draws", &"/nani/generate_draw_lists.cs.hlsl")
        .bind_external_storage_buffer(H_ATLAS_PART_INFO, atlas_part_info)
        .read_rg_storage_buffer(H_SORTED_INSTANCES_BUFFER, sorted_instances)
        .read_storage_buffer::<S::DrawListOffsets>(H_INSTANCE_LIST_OFFSET)
        .write_storage_buffer_concurrent::<S::DrawList>(H_DRAW_LIST_BUFFER, CULL_WRITE_GROUP)
        .write_storage_buffer_concurrent::<S::InstanceIndirection>(
            H_INSTANCE_INDIRECTION_BUFFER,
            CULL_WRITE_GROUP,
        )
        .write_storage_buffer_concurrent::<S::InstanceIndirectionAllocator>(
            H_INSTANCE_INDIRECTION_ALLOCATOR,
            CULL_WRITE_GROUP,
        )
        .read_rg_storage_buffer(H_THREAD_GROUP_DRAW_CMD_HISTOGRAM, thread_group_histogram)
        .read_rg_storage_buffer(H_THREAD_GROUP_DRAW_CMD_SUBTOTALS, thread_group_subtotals)
        .read_rg_storage_buffer(H_THREAD_COUNT_BUFFER, thread_count)
        .dispatch_indirect_with(dispatch_args, move |_api| {
            let _ = args_offset;
        })
}

/// `"Nani draw cmd histogram prefix"` + `"Nani draw cmd subtotal prefix"`（O 0x140a297b0）。
pub fn group_draw_cmd_prefix_sum<S: NaniSsbos>(
    env: &mut PassEnv<'_>,
    inout: &RgBuffer,
    subtotals: &RgBuffer,
    dispatch_args: &RgBuffer,
    histogram_args_offset: u64,
    subtotal_args_offset: u64,
) -> [RecordedComputePass; 2] {
    let histogram = env
        .new_pass(
            "Nani draw cmd histogram prefix",
            &"/nani/group_draw_cmd_prefix_sum.cs.hlsl",
        )
        .write_storage_buffer::<S::DrawListLength>(H_DRAW_LIST_LENGTH_BUFFER)
        .write_rg_storage_buffer(H_INOUT_BUF, inout)
        .write_rg_storage_buffer(H_SUBTOTALS_BUF, subtotals)
        .dispatch_indirect_with(dispatch_args, move |_| {
            let _ = histogram_args_offset;
        });
    let subtotal = env
        .new_pass(
            "Nani draw cmd subtotal prefix",
            &"/nani/subtotal_prefix_sum.cs.hlsl",
        )
        .write_rg_storage_buffer(H_INOUT_BUF, subtotals)
        .dispatch_indirect_with(dispatch_args, move |_| {
            let _ = subtotal_args_offset;
        });
    [histogram, subtotal]
}

/// `"Nani unsorted atlas idx prefix sum"`（O 0x140a29640）：创建 RG 缓冲 `"Nani unsorted atlas idx prefix"`，
/// `/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl`（`dispatch_with`，计数来自各子集）。
pub fn prefix_sum_unsorted_instances_per_atlas_entry<S: NaniSsbos>(
    env: &mut PassEnv<'_>,
    prefix_bytes: u32,
    per_subset: Vec<(PushConstants, u32)>,
) -> (RgBuffer, RecordedComputePass) {
    let prefix = env.create_buffer("Nani unsorted atlas idx prefix", prefix_bytes, 0);
    let rec = env
        .new_pass(
            "Nani unsorted atlas idx prefix sum",
            &"/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl",
        )
        .read_storage_buffer::<S::DrawListOffsets>(H_INSTANCE_LIST_OFFSET)
        .write_storage_buffer::<S::DrawList>(H_DRAW_LIST_BUFFER)
        .write_rg_storage_buffer(H_UNSORTED_ATLAS_PREFIX, &prefix)
        .dispatch_with(move |api| cull_and_bucket_dispatch(api, &per_subset));
    (prefix, rec)
}

/// `"Nani fill unsorted instance list"` + `"Nani fill sorted instance list"`（O 0x140a29280）：
/// 先建 `"Nani sorted instances"` RG 缓冲；两个 pass 的绑定序相同（`histogram`/`sorted_instances_buffer` 并发写），
/// unsorted 版额外并发写 `draw_list_buffer`。
pub fn fill_sorted_instance_list<S: NaniSsbos>(
    env: &mut PassEnv<'_>,
    sorted_bytes: u32,
    thread_count: &RgBuffer,
    filtered_meta: &RgBuffer,
    histogram: &RgBuffer,
    dispatch_args: &RgBuffer,
) -> (RgBuffer, [RecordedComputePass; 2]) {
    let sorted = env.create_buffer("Nani sorted instances", sorted_bytes, 0);
    let unsorted = env
        .new_pass(
            "Nani fill unsorted instance list",
            &"/nani/fill_unsorted_instance_list.cs.hlsl",
        )
        .read_storage_buffer::<S::DrawListOffsets>(H_INSTANCE_LIST_OFFSET)
        .read_rg_storage_buffer(H_THREAD_COUNT_BUFFER, thread_count)
        .read_storage_buffer::<S::FilteredInstance>(H_FILTERED_INSTANCE_BUFFER)
        .read_rg_storage_buffer(H_FILTERED_INSTANCE_META_BUFFER, filtered_meta)
        .write_rg_storage_buffer_concurrent(H_HISTOGRAM, histogram, CULL_WRITE_GROUP)
        .write_rg_storage_buffer_concurrent(H_SORTED_INSTANCES_BUFFER, &sorted, CULL_WRITE_GROUP)
        .write_storage_buffer_concurrent::<S::DrawList>(H_DRAW_LIST_BUFFER, CULL_WRITE_GROUP)
        .dispatch_indirect_with(dispatch_args, |_| {});
    let sorted_pass = env
        .new_pass(
            "Nani fill sorted instance list",
            &"/nani/fill_sorted_instance_list.cs.hlsl",
        )
        .read_storage_buffer::<S::DrawListOffsets>(H_INSTANCE_LIST_OFFSET)
        .read_rg_storage_buffer(H_THREAD_COUNT_BUFFER, thread_count)
        .read_storage_buffer::<S::FilteredInstance>(H_FILTERED_INSTANCE_BUFFER)
        .read_rg_storage_buffer(H_FILTERED_INSTANCE_META_BUFFER, filtered_meta)
        .write_rg_storage_buffer_concurrent(H_HISTOGRAM, histogram, CULL_WRITE_GROUP)
        .write_rg_storage_buffer_concurrent(H_SORTED_INSTANCES_BUFFER, &sorted, CULL_WRITE_GROUP)
        .dispatch_indirect_with(dispatch_args, |_| {});
    (sorted, [unsorted, sorted_pass])
}

// ---------------------------------------------------------------- 墙 draw list（W21 / W27）

/// 墙 SSBO 类型集合（`geometry::instanced_wall::*`）。
pub trait WallSsbos {
    type BrickTransform: LibrarySsbo;
    type BrickPrevVisibility: LibrarySsbo;
    type BrickNextVisibility: LibrarySsbo;
    type BrickDrawList: LibrarySsbo;
    /// `indirect_params::DrawIndexedIndirectBuffer`。
    type DrawCommands: LibrarySsbo;
}

/// W21 — `"Generate wall draw lists"`（O 0x14098aaf0，`/wall/generate_draw_lists.cs.hlsl:cs_pass1`）。
/// 之前：RG 清 `BrickNextVisibilityBitmask`；`SparseInstanceBuffer::{hide,show}`（他人域）。
pub fn generate_wall_draw_lists<W: WallSsbos>(
    env: &mut PassEnv<'_>,
    draw_indirect_offset: i32,
    batches: Vec<(PushConstants, u32)>,
) -> RecordedComputePass {
    env.new_pass(
        "Generate wall draw lists",
        &"/wall/generate_draw_lists.cs.hlsl:cs_pass1",
    )
    .read_storage_buffer::<W::BrickTransform>(H_BRICK_INSTANCE_DATA)
    .read_storage_buffer::<W::BrickPrevVisibility>(H_PREV_VISIBILITY)
    .write_storage_buffer::<W::BrickNextVisibility>(H_NEXT_VISIBILITY)
    .write_storage_buffer::<W::BrickDrawList>(H_BRICK_DRAW_LIST)
    .write_storage_buffer_concurrent::<W::DrawCommands>(H_DRAW_COMMANDS, 1)
    .uniform(H_DRAW_INDIRECT_BUFFER_OFFSET, draw_indirect_offset)
    .dispatch_with(move |api| cull_and_bucket_dispatch(api, &batches))
}

/// W27 — `"Wall pass2 reset inst counts"`（`/wall/reset_pass2_inst_counts.cs.hlsl`，`dispatch_1d`，`.rdata 0x142ab8400`）+
/// `"Generate wall draw lists OC/2"`（`cs_pass2`，读 `hzb_tex`）。返回 `[reset, pass2]`。
pub fn generate_wall_draw_lists_pass2<W: WallSsbos>(
    env: &mut PassEnv<'_>,
    hzb: &RgTexture,
    hzb_dims: [u32; 2],
    reset_offset: i32,
    draw_indirect_offset: i32,
    batches: Vec<(PushConstants, u32)>,
) -> [RecordedComputePass; 2] {
    let reset = env
        .new_pass(
            "Wall pass2 reset inst counts",
            &"/wall/reset_pass2_inst_counts.cs.hlsl",
        )
        .write_storage_buffer::<W::DrawCommands>(H_DRAW_COMMANDS)
        .uniform(H_DRAW_INDIRECT_BUFFER_OFFSET, reset_offset)
        .dispatch_1d(1);
    let pass2 = env
        .new_pass(
            "Generate wall draw lists OC/2",
            &"/wall/generate_draw_lists.cs.hlsl:cs_pass2",
        )
        .read_storage_buffer::<W::BrickTransform>(H_BRICK_INSTANCE_DATA)
        .bind_texture(H_HZB_TEX, &hzb, ACCESS_READ)
        .uniform(H_HZB_DIMS, hzb_dims)
        .read_storage_buffer::<W::BrickPrevVisibility>(H_PREV_VISIBILITY)
        .write_storage_buffer::<W::BrickNextVisibility>(H_NEXT_VISIBILITY)
        .write_storage_buffer::<W::BrickDrawList>(H_BRICK_DRAW_LIST)
        .write_storage_buffer_concurrent::<W::DrawCommands>(H_DRAW_COMMANDS, 1)
        .uniform(H_DRAW_INDIRECT_BUFFER_OFFSET, draw_indirect_offset)
        .dispatch_with(move |api| cull_and_bucket_dispatch(api, &batches));
    [reset, pass2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::render::pass_builder::pass_resource_access::PassBinding;
    use crate::rhapsody_types::TextureDesc;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    macro_rules! carriers {
        ($($n:ident),*) => {$(
            #[derive(Clone, Copy, Debug, Default)]
            pub struct $n;
            impl crate::render::ssbo::StorageElement for $n {
                const NAME: &'static str = stringify!($n);
                const BUFFER_LEN: usize = 1;
                fn default_value() -> Option<Self> { Some($n) }
            }
            impl LibrarySsbo for $n {
                type Storage = crate::render::ssbo::TypedStorageBuffer<$n>;
                fn new_storage_buffer(
                    device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
                    initial: Option<&[u8]>,
                ) -> Self::Storage {
                    match initial {
                        None => crate::render::ssbo::TypedStorageBuffer::<$n>::new(device, None),
                        Some(b) => crate::render::ssbo::TypedStorageBuffer::<$n>::new_storage_buffer(device, Some(b)),
                    }
                }
            }
        )*};
    }
    carriers!(
        Offsets, Length, Alloc, FAlloc, DList, Indir, FInst, Prev, Next, DBuf, Brick, BPrev, BNext,
        BList, DCmd
    );

    struct N;
    impl NaniSsbos for N {
        type DrawListOffsets = Offsets;
        type DrawListLength = Length;
        type InstanceIndirectionAllocator = Alloc;
        type FilteredInstanceAllocator = FAlloc;
        type DrawList = DList;
        type InstanceIndirection = Indir;
        type FilteredInstance = FInst;
        type PrevVisibility = Prev;
        type NextVisibility = Next;
        type DrawBuffer = DBuf;
    }
    struct W;
    impl WallSsbos for W {
        type BrickTransform = Brick;
        type BrickPrevVisibility = BPrev;
        type BrickNextVisibility = BNext;
        type BrickDrawList = BList;
        type DrawCommands = DCmd;
    }

    fn make_env() -> Env {
        let names = [
            H_INSTANCE_LIST_OFFSET,
            H_DRAW_LIST_LENGTH_BUFFER,
            H_INSTANCE_INDIRECTION_ALLOCATOR,
            H_FILTERED_INSTANCE_ALLOCATOR,
            H_HISTOGRAM,
            H_ATLAS_PART_INFO,
            H_DRAW_LIST_BUFFER,
            H_DRAW_BUFFER,
            H_ARGS_BUFFER,
            H_FILTERED_INSTANCE_BUFFER,
            H_FILTERED_INSTANCE_META_BUFFER,
            H_OUTPUT_HISTOGRAM,
            H_DRAW_LIST_INFO,
            H_PREV_VISIBILITY,
            H_NEXT_VISIBILITY,
            H_HZB_TEX,
            H_INOUT_BUF,
            H_SUBTOTALS_BUF,
            H_SORTED_INSTANCES_BUFFER,
            H_INSTANCE_INDIRECTION_BUFFER,
            H_THREAD_GROUP_DRAW_CMD_HISTOGRAM,
            H_THREAD_GROUP_DRAW_CMD_SUBTOTALS,
            H_THREAD_COUNT_BUFFER,
            H_UNSORTED_ATLAS_PREFIX,
            H_BRICK_INSTANCE_DATA,
            H_BRICK_DRAW_LIST,
            H_DRAW_COMMANDS,
        ];
        env_with(
            &names,
            &[H_HZB_DIMS, H_DRAW_INDIRECT_BUFFER_OFFSET],
            |lib, dev| {
                lib.add_new::<Offsets>(dev);
                lib.add_new::<Length>(dev);
                lib.add_new::<Alloc>(dev);
                lib.add_new::<FAlloc>(dev);
                lib.add_new::<DList>(dev);
                lib.add_new::<Indir>(dev);
                lib.add_new::<FInst>(dev);
                lib.add_new::<Prev>(dev);
                lib.add_new::<Next>(dev);
                lib.add_new::<DBuf>(dev);
                lib.add_new::<Brick>(dev);
                lib.add_new::<BPrev>(dev);
                lib.add_new::<BNext>(dev);
                lib.add_new::<BList>(dev);
                lib.add_new::<DCmd>(dev);
            },
        )
    }

    #[test]
    fn hashes_are_exe_immediates() {
        assert_eq!(H_INSTANCE_LIST_OFFSET, imm(-0x7901623ae66f3a1d));
        assert_eq!(H_DRAW_LIST_LENGTH_BUFFER, imm(-0x799e57eecde3bb8c));
        assert_eq!(H_INSTANCE_INDIRECTION_ALLOCATOR, 0x2d8ee4b8ed072cba);
        assert_eq!(H_FILTERED_INSTANCE_ALLOCATOR, 0x686c38b1c905f34d);
        assert_eq!(H_HISTOGRAM, imm(-0x11c3b3628d30eff));
        assert_eq!(H_ATLAS_PART_INFO, 0x2a75aeb2d5bdccbd);
        assert_eq!(H_DRAW_LIST_BUFFER, imm(-0x2fdbfcd113417421));
        assert_eq!(H_DRAW_BUFFER, imm(-0x4976357cb4b70382));
        assert_eq!(H_ARGS_BUFFER, imm(-0x27492a0795df9f11));
        assert_eq!(H_FILTERED_INSTANCE_BUFFER, imm(-0x2ffa26203db61365));
        assert_eq!(H_FILTERED_INSTANCE_META_BUFFER, 0x77fada2ea41d423b);
        assert_eq!(H_OUTPUT_HISTOGRAM, 0x7ab593be137ba575);
        assert_eq!(H_DRAW_LIST_INFO, 0x668a7272bfe497f1);
        assert_eq!(H_PREV_VISIBILITY, 0x35ab36bc43c7b8ed);
        assert_eq!(H_NEXT_VISIBILITY, 0x3783a9903b7f67c5);
        assert_eq!(H_HZB_TEX, 0x62b6812f89a8a569);
        assert_eq!(H_INOUT_BUF, imm(-0x55479e2ce7528db8));
        assert_eq!(H_SUBTOTALS_BUF, imm(-0x3c98bbbf68e09670));
        assert_eq!(H_SORTED_INSTANCES_BUFFER, imm(-0x64a1866bc55ddcf2));
        assert_eq!(H_INSTANCE_INDIRECTION_BUFFER, imm(-0x32063c9595637380));
        assert_eq!(H_THREAD_GROUP_DRAW_CMD_HISTOGRAM, 0x4066895e2a040cae);
        assert_eq!(H_THREAD_GROUP_DRAW_CMD_SUBTOTALS, imm(-0x2054e818091adf0f));
        assert_eq!(H_THREAD_COUNT_BUFFER, 0x5337bbde238aac84);
        assert_eq!(H_UNSORTED_ATLAS_PREFIX, 0x5fc6561e3e33cde8);
        assert_eq!(H_BRICK_INSTANCE_DATA, imm(-0x5d0dc127d53b65b7));
        assert_eq!(H_BRICK_DRAW_LIST, 0x7d0cc6f7423d51b2);
        assert_eq!(H_DRAW_COMMANDS, imm(-0xe3f85d47fed89a6));
    }

    #[test]
    fn fixed_shape_passes_use_the_rdata_dispatch_sizes() {
        let mut e = make_env();
        let buf = RgBuffer { resource: 40 };
        let p = e.run(|penv| clear_draw_lists::<N>(penv));
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads { x: 108, y: 1, z: 1 }
        ));
        let p = e.run(|penv| clear_histogram(penv, &buf));
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads {
                x: 110_592,
                y: 1,
                z: 1
            }
        ));
        let p = e.run(|penv| histogram_prefix_sum(penv, &buf));
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads {
                x: 55_296,
                y: 1,
                z: 1
            }
        ));
        let p = e.run(|penv| {
            draw_indexed_indirect_to_dispatch::<N>(penv, &buf, IndirectArgsType::WoodenPlank)
        });
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads { x: 1, y: 1, z: 1 }
        ));
        assert_eq!(p.push_constants, [0xb, 0, 0, 0, 4, 0, 0, 0]);
        assert_eq!(
            e.world.backend.paths.borrow().as_slice(),
            [
                "/nani/clear_draw_lists.cs.hlsl:cs_main",
                "/nani/clear_histogram.cs.hlsl:cs_main",
                "/nani/histogram_prefix_sum.cs.hlsl:cs_main",
                "/nani/draw_indexed_indirect_to_dispatch.cs.hlsl:cs_main"
            ]
        );
        assert_eq!(LIST_SORTING_HISTOGRAM_BYTES, 0x6c000);
    }

    #[test]
    fn clear_draw_lists_binding_order() {
        let mut e = make_env();
        let p = e.run(|penv| clear_draw_lists::<N>(penv));
        // ctor 的 color library + 1 读 + 3 写。
        assert!(matches!(
            p.bindings.as_slice(),
            [
                PassBinding::BufferRead { .. },
                PassBinding::BufferRead { .. },
                PassBinding::Buffer { .. },
                PassBinding::Buffer { .. },
                PassBinding::Buffer { .. }
            ]
        ));
    }

    #[test]
    fn prepare_and_external_cull_declare_the_expected_extras() {
        let mut e = make_env();
        let h = crate::rhapsody_contract::ManagedHandle(5);
        let p = e.run(|penv| prepare_empty_unsorted_draw_lists::<N>(penv, &h, vec![]));
        assert_eq!(p.name, "Nani prepare unsorted draw lists");
        assert_eq!(p.external_bindings.len(), 1);
        assert!(matches!(p.dispatch, DispatchMode::With(_)));
        let meta = RgBuffer { resource: 41 };
        let hist = RgBuffer { resource: 42 };
        let res = CullBucketResources {
            filtered_instance_meta: &meta,
            output_histogram: &hist,
            atlas_part_info: &h,
            draw_list_info: &h,
            hzb: None,
            hzb_dims: [0, 0],
        };
        let before = e.run(|penv| penv_accesses(penv, &res));
        let ext = e.run(|penv| {
            cull_and_bucket_external::<_, DBuf, N>(
                penv,
                &"/nani/cull_and_bucket.cs.hlsl",
                &res,
                vec![],
            )
        });
        assert_eq!(
            ext.builder.accesses.len(),
            before + 1,
            "多读一个 instance_buffer"
        );
        assert!(matches!(ext.dispatch, DispatchMode::With(_)));
        assert_eq!(FRAME_PASS_ORDER[0], "Nani clear draw lists");
    }

    fn penv_accesses(penv: &mut PassEnv<'_>, res: &CullBucketResources<'_>) -> usize {
        create_cull_and_bucket_pass::<_, N>(penv, &"/nani/cull_and_bucket.cs.hlsl", res)
            .accesses()
            .len()
    }

    #[test]
    fn push_constants_are_twenty_bytes_le() {
        let pc = PushConstants([1, 2, 3, 4, 0xdead_beef]);
        let b = pc.to_bytes();
        assert_eq!(b.len(), 20);
        assert_eq!(&b[16..], &0xdead_beefu32.to_le_bytes());
    }

    #[derive(Default)]
    struct Api(Vec<String>);
    impl ComputeDispatchApi for Api {
        fn dispatch_1d(&mut self, x: u32) {
            self.0.push(format!("1d {x}"));
        }
        fn dispatch_2d(&mut self, x: u32, y: u32) {
            self.0.push(format!("2d {x} {y}"));
        }
        fn dispatch_3d(&mut self, a: DispatchArgs) {
            self.0.push(format!("3d {} {} {}", a.x, a.y, a.z));
        }
        fn push_constants(&mut self, b: &[u8]) {
            self.0.push(format!("pc {}", b.len()));
        }
        fn track_gpu_lifetime(&mut self) {}
    }

    #[test]
    fn cull_dispatch_closure_pushes_then_dispatches_each_subset() {
        let mut api = Api::default();
        cull_and_bucket_dispatch(
            &mut api,
            &[(PushConstants([1; 5]), 7), (PushConstants([2; 5]), 0)],
        );
        assert_eq!(api.0, ["pc 20", "3d 7 1 1", "pc 20", "3d 0 1 1"]);
    }

    #[test]
    fn cull_and_bucket_binding_order_and_hzb_variant() {
        let mut e = make_env();
        let meta = RgBuffer { resource: 41 };
        let hist = RgBuffer { resource: 42 };
        let h = crate::rhapsody_contract::ManagedHandle(5);
        let hzb = RgTexture {
            resource: 77,
            desc: TextureDesc::new([8, 8, 1], 0, 0),
        };
        let mk = |hzb| CullBucketResources {
            filtered_instance_meta: &meta,
            output_histogram: &hist,
            atlas_part_info: &h,
            draw_list_info: &h,
            hzb,
            hzb_dims: [8, 8],
        };
        let base = e.run(|penv| {
            create_cull_and_bucket_pass::<_, N>(penv, &"/nani/cull_and_bucket.cs.hlsl", &mk(None))
        });
        let with = e.run(|penv| {
            create_cull_and_bucket_pass::<_, N>(
                penv,
                &"/nani/cull_and_bucket.cs.hlsl",
                &mk(Some(&hzb)),
            )
        });
        assert_eq!(base.builder.name, "Nani cull & bucket");
        // 访问：ctor color library + 4 并发写(2 SSBO + 2 RG) + prev(读) + next(并发写)。
        let acc: Vec<_> = base
            .accesses()
            .iter()
            .map(|a| (a.access, a.group))
            .collect();
        assert_eq!(acc.len(), 1 + 4 + 1 + 1);
        assert_eq!(base.external_bindings().len(), 2);
        assert_eq!(with.accesses().len(), base.accesses().len() + 1);
        assert_eq!(
            with.accesses().last().map(|a| (a.resource, a.access)),
            Some((77, 0x19))
        );
    }

    #[test]
    fn wall_passes_declare_the_original_names_and_entries() {
        let mut e = make_env();
        let p1 = e.run(|penv| {
            generate_wall_draw_lists::<W>(penv, 3, vec![(PushConstants::default(), 4)])
        });
        assert_eq!(p1.name, "Generate wall draw lists");
        assert!(matches!(p1.dispatch, DispatchMode::With(_)));
        let hzb = RgTexture {
            resource: 9,
            desc: TextureDesc::new([4, 4, 1], 0, 0),
        };
        let [reset, p2] =
            e.run(|penv| generate_wall_draw_lists_pass2::<W>(penv, &hzb, [4, 4], 1, 2, vec![]));
        assert_eq!(
            (reset.name, p2.name),
            (
                "Wall pass2 reset inst counts",
                "Generate wall draw lists OC/2"
            )
        );
        assert!(matches!(
            reset.dispatch,
            DispatchMode::Threads { x: 1, y: 1, z: 1 }
        ));
        let uris = e.world.backend.paths.borrow().clone();
        assert_eq!(
            uris,
            [
                "/wall/generate_draw_lists.cs.hlsl:cs_pass1",
                "/wall/reset_pass2_inst_counts.cs.hlsl:cs_main",
                "/wall/generate_draw_lists.cs.hlsl:cs_pass2"
            ]
        );
    }

    #[test]
    fn rg_buffers_created_by_helpers() {
        let mut e = make_env();
        let (prefix, p) =
            e.run(|penv| prefix_sum_unsorted_instances_per_atlas_entry::<N>(penv, 1024, vec![]));
        assert_eq!(e.frame.buffers[0].0, "Nani unsorted atlas idx prefix");
        assert_eq!(e.frame.buffers[0].1.size, 1024);
        assert_eq!(prefix.resource, 101);
        assert_eq!(p.name, "Nani unsorted atlas idx prefix sum");
        let thread_count = RgBuffer { resource: 1 };
        let meta = RgBuffer { resource: 2 };
        let hist = RgBuffer { resource: 3 };
        let args = RgBuffer { resource: 4 };
        let (sorted, passes) = e.run(|penv| {
            fill_sorted_instance_list::<N>(penv, 4096, &thread_count, &meta, &hist, &args)
        });
        assert_eq!(e.frame.buffers[1].0, "Nani sorted instances");
        assert_eq!(e.frame.buffers[1].1.size, 4096);
        assert_eq!(sorted.resource, 102);
        assert_eq!(passes[0].name, "Nani fill unsorted instance list");
        assert_eq!(passes[1].name, "Nani fill sorted instance list");
        assert!(matches!(
            passes[0].dispatch,
            DispatchMode::IndirectWith { buffer: 4, .. }
        ));
    }
}
