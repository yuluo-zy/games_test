# WP10 契约 3/3 — vegetation 系统（garden / grass / foliage exclusion / tree-tool / forest）

版本 **B**。骨架证据：`systems/tree-tool/*`、`systems/forest/*`、`country-core/src/systems/garden/*`、
`country-core/src/startup/{spawn_preplaced_trees,clearing,startup_grass,startup_meadow,startup_critters,startup_sheep}.rs`、
`country-core/src/resources/{garden_mask,terrain_height,water_exclude_clutter_mask}.rs`、compute_passes（WP5 消费侧，本包只记录 CPU 生产者/数据）。
反编译：Ghidra `0x142267430`(process_tree_cmds)/`0x1422686d0`(despawn_trees)/`0x14226a260`(hide_trees)/`0x1422689c0`(tree_move_bend)/`0x142279500`(handplaced_tree_animation)/`0x14227a0d0`(forest_tree_animation)/`0x141bf16c0`(manage_tree_prefabs)。
标注：**O**=反编译/PDB 直接支持；**I**=调用链/名称组合；**U**=未恢复。

## 1. 植被 CPU 数据（帧 pass 消费的生产者，O/I）

| 数据 | 来源模块 | 消费 pass（WP5） | 状态 |
|---|---|---|---|
| `TerrainHeightsData<T>`（`sample_at_world_xz` / `heightfield_intersect_ray` / `copy_from_gpu` / `subscribe`） | `resources/terrain_height.rs`（O 方法名） | grass_compute(W29)、garden_spawn(W16)、spawn_vegetation_around_buildings(W14)、path_pebbles、water_spawn(W13)、rasterize_buildings | CPU 高度图；`bind_terrain_heightmap` 经 SSBO 上 GPU |
| `GardenMaskSsbo` | `resources/garden_mask.rs` | garden_rasterize_from_cpu(W12) 写、garden_spawn_vegetation(W16) 读 | 花园 512 掩码 |
| `WaterExcludeClutterMaskSsbo` | `resources/water_exclude_clutter_mask.rs` | foliage exclusion / water spawn（I） | 水面排除灌木掩码 |
| `GardenRaster`（CPU raster） | `resources/world_raster.rs`（WP6 已恢复） | W12/W16 | 上传/程序化生成 |
| `BushSsbo/FlowerSsbo/Flower03Ssbo/GardenStoneSsbo` | compute_garden（nani 实例数据） | garden_spawn_vegetation(W16) | 布局 U（wp3 PlantInstanceData 域，引用不重定义） |
| `BushCpuSsbo` | read_bush_positions_for_cpu.rs | read_bush_positions_for_cpu(W15) | CPU 回读灌木位置 |
| `LeafParticleSsbo/ParticleEmitterSsbo/LoadedParticleEmitterCount` | leaf_particles.rs + startup | leaf_particles_compute(W18) | CPU 资源；`LoadedParticleEmitterCount` 定义在 startup/mod.rs（O） |
| `GrassBladeDataSsbo/ObjectGrassPush` | grass.rs + draw_shaders | grass_compute(W29)、object_grass_push(W9) | 布局 U（WP5 §5/§8） |
| `startup_grass::startup_grass` | startup | grass 资源初始化 | 函数 O，数据 U |
| `startup_meadow::init_meadow`（`PlacementConfigEntry`） | startup | 草地放置配置 | 名 O |

## 2. tree-tool 系统（`systems/tree-tool`，函数名 O）

| 系统 | VA | 语义 |
|---|---|---|
| `impl$0::build` | — | 插件注册（`TreeStorage::default` 内联 :33） |
| `process_tree_cmds` | 0x142267430 | 消费 `MoveTreeCmd`/`InsertTreeCmd`/`DeleteTreeCmd` → TreeStorage get/insert/remove + `SpawnPrefabCmd`/`PlayTreeSoundCmd(kind 0/1/2)`（tree-tool 草稿：Move→Insert→Delete 三段） |
| `despawn_trees` | 0x1422686d0 | `Query<(Entity,&TreeAnimation,&TreeMarkedForDeletion)>`；`progress < 0.01` → `DespawnPrefabCmd` |
| `hide_trees` | 0x14226a260 | 消费 `HideTreesNextToWallCmd`/`HideTreesinAreaCmd`，隐藏/显示树（U 掩码细节） |
| `tree_move_bend` | 0x1422689c0 | 拖拽弯曲插值（动画状态，U 公式） |
| `ui_place_tree`/`ui_move_tree`/`ui_delete_tree` | — | UI 命令生产（名 O，行为 I） |
| `replay_tree`/`recalc_tree_position_on_terrain_height_change`/`convert_preplaced_trees`/`startup_tree_tool` | — | 回放/地形跟随/预置树转换 |

## 3. forest 系统（`systems/forest`，函数名 O）

| 系统 | VA | 语义 |
|---|---|---|
| `impl$0::build` | — | 插件注册（`TreeStorage` 默认 + ForestTree，:100） |
| `handplaced_tree_animation` | 0x142279500 | 手植树动画：跳表状态机（`uVar5-6 < 0xf` → 15 分支，O）；puffin scope |
| `forest_tree_animation` | 0x14227a0d0 | 森林树动画（风/摆动，U 公式） |
| `should_remove_tree_canopy_from_view` | — | 冠层视锥移除（名 O） |
| `manage_tree_prefabs` | 0x141bf16c0 | 管理树预制体实例；`is_too_close_to_shape`（防贴建筑） |

## 4. garden 系统（`country-core::systems/garden`，函数名 O）

| 系统 | 语义 |
|---|---|
| `draw_garden`（closure$0/1/2） | 花园画笔输入 → 更新 GardenRaster/掩码（I） |
| `record_garden_history` | 花园编辑历史记录 |

`startup_garden_brush`/`startup_meadow` 初始化 GardenMask/草地；`clearing::{init_clearing,uninit_clearing}` 维护 `InteractableForestPlacements`。

## 5. 帧 pass ↔ CPU 数据接线（I，WP5 已有消费侧）

- W29 `grass_compute`：`TerrainHeightsData<TerrainHeightSsbo>`（`bind_terrain_heightmap`）→ 间接草束。
- W16 `garden_spawn_vegetation`：`TerrainHeightsData` + `GardenMaskSsbo` → `/garden/spawn_{bushes,flowers,small_flowers}_garden`。
- W14 `spawn_vegetation_around_buildings`：`ActiveSession::is_autumn` + `TerrainHeightsData` → `/garden/spawn_{bushes,flowers}_buildings`。
- W18 `leaf_particles_compute`：`utils::get_res` + `LoadedParticleEmitterCount` → `/leaf_particles.cs.hlsl`。
- W7 `rasterize_foliage_exclusion_mask`：`WaterExcludeClutterMaskSsbo` 相关（I）。
- W15 `read_bush_positions_for_cpu`：`BushCpuSsbo` 回读 → CPU 灌木状态。

## 6. 失败分支 / 边界

- `spawn_ducks`：demo 模式或损坏存档 → 不生成（O）。
- `despawn_trees`：`f < 0.01` 严格小于才销毁（NaN 不销毁，O）。
- `process_tree_cmds` Move 未命中 → log error（O 字符串）。
- `fireflies/butterflies_send_to_gpu`：实例数不等于 96/24 → panic（O）。

## 7. typed gap 汇总

- 各动画系统（tree_move_bend、handplaced/forest_tree_animation、update_butterfly_states、sim_wren_boids）的数值公式与相位状态。
- `GardenMaskSsbo`/`WaterExcludeClutterMaskSsbo`/各 `*Ssbo` 字段布局与纹理尺寸。
- `LoadedParticleEmitterCount`/`ParticleEmitterSsbo` 布局。
- `draw_garden` 画笔形状/半径规则。
- boids（apply_rules）权重与 wren_boids 着陆判定阈值。
