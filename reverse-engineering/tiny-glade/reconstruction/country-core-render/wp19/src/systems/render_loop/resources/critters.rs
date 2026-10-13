//! `render_loop/resources/critters.rs` — critter GPU SSBO type markers.
//!
//! Business logic (contracts/indirect-params.md §2): no standalone functions
//! (PDB functions=0); the 4 types are SSBO carriers used by the render-loop
//! critter systems (via `AssetSsboLibrary` + `pass_resource_access` monomorphs,
//! O procedure_occurrences). Distinct from `resources/critters.rs` (WP6, CPU
//! `Wren` component): this module is the GPU-side buffer markers only.

use crate::typed_gap::Gap;

/// 萤火虫实例数据 SSBO 载体（O 类型名；field 布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct FireflySsbo(pub Gap);

/// 蝴蝶实例数据 SSBO 载体（O 类型名；field 布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ButterflySsbo(pub Gap);

/// 萤火虫可见性查询结果 SSBO（`_critters_firefly_vis_query` compute 写，I）。
#[derive(Clone, Copy, Debug, Default)]
pub struct FireflyVisibilitySsbo(pub Gap);

/// 绵羊动画数据 SSBO 载体（O 类型名；field 布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct SheepAnimationSsbo(pub Gap);
