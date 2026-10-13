//! render/nani.rs — nani GPU-driven 渲染模块根（WP3）。
//!
//! 源 SHA256 `abf304771afe0ae305af45e5f05d879cd9be470def1994d8e8b068f5b522d7e4`；
//! PDB 0 procedure_instances（纯模块声明 + 常量 + 类型别名）。
//! 子模块与 skeleton 树一一对应；各子模块业务契约见 contracts/。

#![allow(dead_code, unused_imports)]

pub mod buffers;
pub mod cull_shaders;
pub mod draw_ext;
pub mod gpu_world;
pub mod instance_data;
pub mod mesh_atlas;
// `systems.rs` (WP6): the 20 `nani_update_*` + `nani_maintain_instances` +
// `nani_flush_instance_updates` ECS systems. Declared by WP6 together with the
// consumed component/resource graph (`crate::components::*`, `crate::resources::*`,
// `crate::utils::components::transform::Transform`, `crate::systems::paths::draw_path`).
// See wp6 contracts/systems.md.
pub mod systems;
pub mod trimesh;

/// 编译期开关：强制细粒度排序（public-symbols `NANI_FORCE_NO_FINE_GRAINED_SORTING`，O）。
pub const NANI_FORCE_FINE_GRAINED_SORTING: bool = false;
/// 编译期开关：禁止细粒度排序（O）。
pub const NANI_FORCE_NO_FINE_GRAINED_SORTING: bool = false;

/// `render::nani` 对外重导出（contracts 提及的共享类型）。
pub use gpu_world::{
    GpuWorld, GpuWorldSubsetKind, InstanceHandle, InstanceKey, NaniInstanceData, OwnedGpuWorldSubset,
};
pub use mesh_atlas::{MeshAtlas, MeshAtlasPart, NaniMeshAtlasPartInfo};

// WP21: nani-mod glue surface（骨架 `render/nani.rs`）。
/// `GpuDrivenDrawable`（WP21；O：GPU 驱动可绘制组件）。
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuDrivenDrawable(pub Gap);

use crate::typed_gap::Gap;
