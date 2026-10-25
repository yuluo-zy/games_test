//! WP18 — country-core `systems::render_loop::draw_passes::draw_shaders` 域
//! （14 个 per-shader raster draw 模块）骨架对齐还原。
//!
//! B 版本（EXE sha256 `cf048e27…`，PDB sha256 `2d9686b8…`）。范围：
//!   - `billboard_forest.rs` — 远景 billboard 森林 startup（mesh/shader 入库 + spawn）
//!   - `clearing_shaders.rs` — 林间空地 `tree_trunk` / `tree_canopy`（`_clearing_*.raster`）
//!   - `clouds.rs` / `stars.rs` — 云/星 startup（`_cloud.raster` / `_star.raster`）
//!   - `common_includes.rs` — 共享绑定 helper（terrain_heightmap / terrain_lighting /
//!     water_object_push / garden_exclusion）
//!   - `draw_bricks.rs` — 墙砖 LOD0/1 间接绘制（`_wall_wall_brick_lod{0,1}.raster`）
//!   - `floor_shader.rs` — 地形地面（`_terrain_editing_floor.raster`）
//!   - `grass_shader.rs` — 草地 LOD0/1（`_grass.raster` / `_grass_lod1.raster`）
//!   - `instanced_critters.rs` — 蝴蝶/萤火虫/萤光/鸭/鹪鹩（`_critters_*.raster`）
//!   - `instanced_ivy.rs` — 常春藤枝/叶/花（`_ivy_*.raster`）
//!   - `leaf_particle_draw.rs` — 落叶粒子（`_leaf_particle.raster`）
//!   - `path_pebble_shader.rs` — 路径石子（`_path_pebble.raster`）
//!   - `rocky_terrain_draw.rs` — 岩石地形（`_rocky_terrain_rocky_terrain.raster`）
//!   - `stairs_draw.rs` — 楼梯间接绘制（`_rocky_terrain_stairs_stairs_indirect.raster`）
//!
//! 业务逻辑合同见 `contracts/*.md`（Observed/Inferred/Unknown + typed gap）。
//! 本 crate 是独立离线 `cargo check` 门禁；**不 patch 共享 `country-core` crate**
//! （集成 owner 整合）。共享装配类型经 `country-core` path 依赖引用
//! （wp1 DrawBuilder/ShaderUri、wp2 buffers、wp3 GpuWorld、wp4 Asset*Library/
//! FrameConstants）；`resources::` 缺失数据以本束 typed-gap 载体补齐
//! （texture_presets / critters / color_id_roof）。

extern crate alloc;

/// 本束缺失的 `resources::` typed-gap 载体（texture_presets / critters /
/// color_id_roof）；共享 `country-core` 已恢复的资源类型直接经该 crate 引用。
pub mod resources;
pub mod systems;
