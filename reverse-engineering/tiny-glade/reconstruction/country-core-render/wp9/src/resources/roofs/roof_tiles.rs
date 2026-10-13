//! roof_tiles.rs — RoofTiles 屋瓦实例资源（WP9）。
//!
//! PDB 骨架 `resources/roofs/roof_tiles.rs`：源文件 8 行；命名实体 `RoofTiles`。
//! `impl$2::register_required_components`（0x140903f40）与 `clone_behavior`（0x140002370，
//! Component derive 胶水，全游戏 183 处复用）→ **ECS 组件**。
//! 消费：`roof_tiles_gravity_compute` 的 `QueryState<(&RoofTiles, &RoofTileSimulation,
//! &Roof, &RoofAnimation)>` 逐项 flat_map `Option<SparseInstanceBufferAllocation<
//! RoofTileSsbo>>` → `InstancedDraw` → `PushConstants`（见 contracts/roof-tiles.md）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;

use crate::resources::roof_tile_ssbo::RoofTileSsbo;
use crate::resources::roofs::roof_animation::RoofAnimation;
use crate::resources::roofs::roof_shape::Roof;
use crate::resources::roofs::roof_tile_sim::RoofTileSimulation;
use crate::render::sparse_instance_buffer::SparseInstanceBufferAllocation;
use crate::typed_gap::Gap;

/// 屋顶的屋瓦实例分配句柄。`create_roof` spawn 时默认空；`assemble_roof_tiles`
/// （0x141a36700，prior work `reconstruction/roof/tiles.rs` 数值核心）填充 64B
/// RoofShingle 记录。
///
/// 字段形态 I：单 `Option<SparseInstanceBufferAllocation<RoofTileSsbo>>`（gravity
/// 闭包内 `.into_iter().flatten()`）；具体 `#[require(...)]` 成员 U。
#[derive(Component, Clone, Debug, Default)]
pub struct RoofTiles {
    /// 已恢复语义 I：该屋顶在共享 RoofTileSsbo 稀疏缓冲中的分配。U：字段名/确切形态。
    pub allocation: Gap,
    /// 布局占位（8 行源文件：类型极可能为 `Option<SparseInstanceBufferAllocation<
    /// RoofTileSsbo>>` 的新类型包装；保留引用防丢失）。
    pub _gpu: Option<SparseInstanceBufferAllocation<RoofTileSsbo>>,
}

impl RoofTiles {
    /// `#[require(...)]` 目标（PDB `register_required_components` 单次注册；具体成员 U）。
    /// 保持与 create_roof spawn 集合一致的类型引用，供集成期对齐。
    #[allow(dead_code)]
    pub fn requires() -> (Roof, RoofTileSimulation, RoofAnimation) {
        (Roof([0u8; 88]), RoofTileSimulation::new(0.0), RoofAnimation::new())
    }
}
