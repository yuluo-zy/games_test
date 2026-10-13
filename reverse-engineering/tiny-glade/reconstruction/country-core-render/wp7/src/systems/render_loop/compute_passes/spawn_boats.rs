//! spawn_boats.rs — 船/水车 spawn（WP7 恢复）。
//!
//! skeleton bundle：`compute_passes/spawn_boats.rs` 仅 `BoatSsbo`（O 名，布局 U；
//! 0 个 procedure，5 条 metadata）。`/water/spawn_boats.cs.hlsl` URI 在 255 静态 URI 中
//! **缺失**（O）→ 运行时动态拼接或并入 `compute_water::water_spawn_vegetation`（I，wp5）。
//! `spawn_waterwheels.cs.hlsl` GPU stub 为空体（O，SPIR-V）→ **waterwheel spawn 在 CPU 侧**（I）。
//!
//! BoatSsbo 生命周期（O，泛型胶水）：`read_access_buffer`（0x14096fe20）、
//! `AssetSsboLibrary::add_new`（0x1412c23d0）、`get_rg_buffer`（0x1412d3af0）、
//! `DrawBuilder::read_storage_buffer`（0x141631690）。

#![allow(dead_code, unused_imports)]

use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::typed_gap::Gap;

/// 船实例 SSBO（O 名；布局 U）。
pub struct BoatSsbo(pub Gap);

impl SsboType for BoatSsbo {
    /// typed gap：原值 = `utils::name_hash("…BoatSsbo")`。
    fn rg_buffer_hash() -> u64 {
        0
    }
}
