//! water_exclude_clutter_mask.rs — 水域排除 clutter 的 shoreline mask（WP7 恢复）。
//!
//! 源（B skeleton）：`crates/country-core/src/resources/water_exclude_clutter_mask.rs`
//! （metadata-only）。PDB 命名实体 `WaterExcludeClutterMaskSsbo` `name_and_relations_only`。
//! 语义（I）：`read_water_mask.cs.hlsl` 打包的 clutter 位（`.y > 0.2`），水域内不摆
//! clutter/装饰物（`systems/clutter/add_lantern_water_setdresssing.rs` 相关）。
//! 生命周期（O，泛型胶水）：`AssetSsboLibrary::add_new/get_rg_buffer`、
//! `ComputePass::write_storage_buffer`、`pass_resource_access::write_access_buffer`
//! （0x140974e20）。

#![allow(dead_code, unused_imports)]

use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::typed_gap::Gap;

/// 水排除 clutter 回读缓冲（O 名，布局 U）。
pub struct WaterExcludeClutterMaskSsbo(pub Gap);

impl SsboType for WaterExcludeClutterMaskSsbo {
    /// typed gap：原值 = `utils::name_hash("…WaterExcludeClutterMaskSsbo")`。
    fn rg_buffer_hash() -> u64 {
        0
    }
}
