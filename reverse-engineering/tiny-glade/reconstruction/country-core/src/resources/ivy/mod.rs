//! ivy/ — 常春藤生长状态族（WP16 恢复；WP21 拆分到骨架子路径粒度）。
//!
//! 源 SHA256（B skeleton）`resources/ivy/{ivy_storage,ivy_leaf_storage,
//! ivy_direction_proposer,wall_coord,mod}.rs`。PDB 命名实体：
//! `Ivy` / `IvyPoint` / `IvyStorage` / `IvyLeaf` / `IvyLeafRng` / `IvyLeafStorage` /
//! `IvyDirectionProposer` / `IvySpawnNoise` / `WallCoord`。
//!
//! 标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。
//! 消费方：`systems::ivy`（grower/spawner/pruner/leaf 等），
//! `render_loop::draw_passes::draw_shaders::instanced_ivy`（分支绘制）。
//!
//! WP16 恢复内容按 skeleton 子路径拆分到各子模块（ivy_storage /
//! ivy_leaf_storage / ivy_direction_proposer / wall_coord）；WP21 的
//! `IvyLeaf` 0x68B 布局（`ivy_leaf_storage.rs`）与 `PublicWallCurve`
//! 采样面（`wall_coord.rs`）为新增恢复。根路径重导出保持既有
//! `crate::resources::ivy::{Ivy, IvyStorage, …}` 消费面不变。

#![allow(dead_code)]

pub mod ivy_direction_proposer;
pub mod ivy_leaf_storage;
pub mod ivy_storage;
pub mod wall_coord;

// Root re-exports (existing consumers reference the root surface).
pub use ivy_direction_proposer::{IvyDirectionProposer, IvySpawnNoise};
pub use ivy_leaf_storage::{IvyLeaf, IvyLeafRng, IvyLeafStorage};
pub use ivy_storage::{Ivy, IvyPoint, IvyStorage};
pub use wall_coord::{PublicWallCurve, WallCoord};
