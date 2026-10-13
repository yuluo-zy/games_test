//! compute_passes/roof_tiles_gravity.rs — 屋顶瓦重力模拟 compute（W19）（WP9）。
//!
//! PDB 骨架 `systems/render_loop/compute_passes/roof_tiles_gravity.rs`（line 13；
//! PushConstants 命名实体；closure$0/closure$1）。机器码 `roof_tiles_gravity_compute`
//! 0x140a29b10。URI（O）：`/roof_tiles_gravity.cs.hlsl` → `_roof_tiles_gravity`
//! （+frame_constants_storage_buffer@1.1）。

#![allow(dead_code, unused_imports)]

use crate::consume::pass_builder::{AsComputeShaderSource, ComputePass, ComputeShaderSource, ShaderDefValue};
use crate::render::pass_builder::shader_uri::{ComputeShaderSourceHash, ShaderUri};
use crate::resources::roof_tile_ssbo::RoofTileSsbo;
use crate::typed_gap::Gap;

/// `/roof_tiles_gravity.cs.hlsl:cs_main` source（O）。
pub struct RoofTilesGravity;
impl AsComputeShaderSource for RoofTilesGravity {
    fn source_hash(&self) -> u64 {
        ComputeShaderSourceHash::compute("/roof_tiles_gravity.cs.hlsl:cs_main")
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse("/roof_tiles_gravity.cs.hlsl:cs_main"),
            defs: ShaderDefValue,
            _pd: core::marker::PhantomData,
        }
    }
}

/// RoofPushConstants（O：shader `roof/roof_tile.inc.hlsl`，20B）。
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct PushConstants {
    pub first_instance: u32,
    pub instance_count: u32,
    pub wall_height_delta: f32,
    pub roof_pivot_y: f32,
    pub animation_t: f32,
}

/// W19 — `roof_tiles_gravity_compute`（O：0x140a29b10）。
///
/// 1. `AssetSsboLibrary::get<RoofTileSsbo>` 取 BufferState（名称断言长度 53 字符串；
///    缺 → unwrap_failed）。
/// 2. `QueryState::new::<(&RoofTiles, &RoofTileSimulation, &Roof, &RoofAnimation)>`；
///    world 校验失败 → panic_mismatched。
/// 3. 每 query 项：RoofTiles 的 `Option<SparseInstanceBufferAllocation<RoofTileSsbo>>`
///    flat_map 各 `InstancedDraw` → 生成 `PushConstants`（first_instance/instance_count
///    来自 InstancedDraw；wall_height_delta 来自 RoofTileSimulation；roof_pivot_y 来自
///    Roof::pivot_3d；animation_t 来自 RoofAnimation），`Vec::from_iter`。
/// 4. `ComputePass::new<RoofTilesGravity>` + `read_storage_buffer<RoofTileSsbo>` +
///    `uniform<f32>`（closure$0 写 wall_height_delta）+ `dispatch_with(closure$1)`
///    （dispatch_1d，组数=PushConstants 数）。
///
/// 深 ECS/资源解析 U；数值语义见 contracts/roof-tiles.md。
pub fn roof_tiles_gravity_compute(
    _ssbo: &Gap,
    _world: &mut Gap,
    _push: &[PushConstants],
) {
    let _ = _ssbo;
    let _ = _world;
    let _ = _push;
}

/// 测试入口：构造一条 push（I）。
pub fn push_for_draw(first_instance: u32, instance_count: u32, delta: f32, pivot_y: f32, t: f32) -> PushConstants {
    PushConstants {
        first_instance,
        instance_count,
        wall_height_delta: delta,
        roof_pivot_y: pivot_y,
        animation_t: t,
    }
}
