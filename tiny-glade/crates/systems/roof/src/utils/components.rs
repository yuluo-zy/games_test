//! system-roof 自有的组件/注册面（utils/components.rs）。
//!
//! PDB 源路径 `crates/systems/roof/src/utils/components.rs`。含：
//! - `RoofSpire`：尖顶标记组件（place_roof_spires 的 With 过滤器）。
//! - `register_required_components`（0x14258d1f0 / 0x14258d8e0 ×2）：
//!   Bevy required-components 注册（胶水；原始两个 impl 覆盖的组件族
//!   见 decomp-0x14258d1f0/0x14258d8e0 —— Roof 实体缺省组件族）。
use bevy_ecs::prelude::*;

/// B `system_roof::utils::components::RoofSpire`（尖顶实体标记）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofSpire;

/// 原 `impl$0::register_required_components`（0x14258d1f0）：
/// 屋顶实体的 required-components 注册（create_roof 的 spawn 元组
/// {Roof, RoofAnimation, RoofTileSimulation, RoofTriggerVolume, Transform}
/// 证实该组合；Bevy 注册胶水不计业务）。
pub fn register_required_components(_registry: &mut bevy_ecs::component::Components) {}

/// 原 `impl$1::register_required_components`（0x14258d8e0）：
/// 另一组件族（动画/瓦片模拟系）的 required 注册。
pub fn register_required_components_second(_registry: &mut bevy_ecs::component::Components) {}
