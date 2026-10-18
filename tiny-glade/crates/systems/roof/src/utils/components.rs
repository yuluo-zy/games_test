//! system-roof 自有的组件/注册面（utils/components.rs）。
//!
//! PDB 源路径 `crates/systems/roof/src/utils/components.rs`。含：
//! - `RoofSpire`：尖顶标记组件（place_roof_spires 的 With 过滤器）。
//! - `RoofSupportBeams`：支撑梁标记组件。
//! - `register_required_components`（0x14258d1f0 impl$0 / 0x14258d8e0 impl$1）：
//!   Bevy 0.16 `#[derive(Component)]` 生成的 required-components 注册胶水
//!   （impl0 类型名长 0x29=41 = `system_roof::utils::components::RoofSpire`；
//!   impl1 类型名长 0x30=48 = `system_roof::utils::components::RoofSupportBeams`，
//!   decomp O）→ **glue-legitimate**（复用 derive，不重建）。
//!
//! 真逻辑（decomp 观测，Bevy derive 产物）：`enforce_no_required_components_recursion`
//! → 查 hashbrown HashMap<TypeId,ComponentId> → `ComponentInfo::new`（TypeId 常量
//! impl0=0x992a85940237df75/0x11f52152d45c2706）→ 注册组件族 → 递归注册 required
//! → `HashMap::insert`。derive(Component) 自动生成等价物。
use bevy_ecs::prelude::*;

/// B `system_roof::utils::components::RoofSpire`（尖顶实体标记；impl0
/// register_required_components 的宿主组件，glue-legitimate）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofSpire;

/// B `system_roof::utils::components::RoofSupportBeams`（支撑梁标记；impl1
/// register_required_components 的宿主组件，glue-legitimate）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct RoofSupportBeams;

/// 原 `impl$0::register_required_components`（0x14258d1f0）：`RoofSpire` 的
/// Bevy derive 注册胶水 —— glue-legitimate（derive(Component) 已生成等价物）。
pub fn register_required_components(_registry: &mut bevy_ecs::component::Components) {}

/// 原 `impl$1::register_required_components`（0x14258d8e0）：`RoofSupportBeams`
/// 的 Bevy derive 注册胶水 —— glue-legitimate。
pub fn register_required_components_second(_registry: &mut bevy_ecs::component::Components) {}