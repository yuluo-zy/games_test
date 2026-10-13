//! cull_shaders.rs — nani 剔除/分桶 compute pass 结构体（WP3）。
//!
//! 本模块在 PDB 无独立 native procedure（泛型/内联）。契约按
//! `gpu_generate_draw_lists`（0x140987dd0）的 `cull_and_bucket` 闭包接线登记，
//! 以及 `ComputePass::dispatch_with<cull_and_bucket::closure_env$1>`（0x14160a2e0）/
//! `dispatch_indirect_with<group_draw_cmd_prefix_sum::closure_env$0>`（0x14160b850）。
//! 完整契约见 contracts/cull_shaders.md。

#![allow(dead_code, unused_imports)]

use crate::render::nani::gpu_world::GpuWorldSubsetKind;

/// `AsComputeShaderSource` 实现：把 URI 映射到 shader 管线（trait 属 WP1 shader_uri）。
/// URI 见 contracts/cull_shaders.md §1。

/// 通用主实例 cull/bucket（`/nani/cull_and_bucket.cs.hlsl:cs_main`）。
pub struct NaniCullAndBucketMain;

/// 灌木（`..._bush...`）。
pub struct NaniCullAndBucketBush;

/// 弯曲半木（`..._curved_halftimber...`）。
pub struct NaniCullAndBucketCurvedHalftimber;

/// 花（`..._flowers...`）。
pub struct NaniCullAndBucketFlowers;

/// 石膏（`..._plaster...`）。
pub struct NaniCullAndBucketPlaster;

/// 屋顶瓦（`..._roof_tile...`；瓦变体读 `instance_buffer`）。
pub struct NaniCullAndBucketRoofTile;

/// 烟（`..._smoke...`）。
pub struct NaniCullAndBucketSmoke;

/// 楼梯地板（`..._stairs_floor...`）。
pub struct NaniCullAndBucketStairsFloor;

/// 木板（`..._wooden_plank...`）。
pub struct NaniCullAndBucketWoodenPlank;

/// kind → cull/bucket pass 构造（I：由 `gpu_generate_draw_lists` 分派）。
pub fn cull_and_bucket_for_kind(_kind: GpuWorldSubsetKind) {
    todo!("recover cull_and_bucket closure; see contracts/cull_shaders.md §2")
}

/// 分组 draw-cmd 前缀和（`dispatch_indirect_with<group_draw_cmd_prefix_sum>`）。
pub fn group_draw_cmd_prefix_sum() {
    todo!()
}
