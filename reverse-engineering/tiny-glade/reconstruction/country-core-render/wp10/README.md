# WP10 — vegetation + critters CPU content domain（B 版本）

版本 **B**（`tiny-glade.exe` cf048e27…，PDB 2d9686b8…）。恢复 country-core **植被 + 小动物**
CPU 域：树/植被实例存储、小动物 CPU 状态、garden/grass/植被 CPU 数据（帧 pass 消费的生产者）
与植被/小动物系统。方法按项目流程：业务逻辑分析（PDB enriched/structures + Ghidra 反编译 +
调用链）→ 整理契约（Observed/Inferred/Unknown + typed gap）→ 骨架对齐 Rust 实现。
机器码差分仅作裁决（map 值类型冲突、关键数值），不构成实现手段；不建立 oracle。

## 交付

| 路径 | 内容 |
|---|---|
| `contracts/tree-storage.md` | TreeStorage 布局/方法、命令、map 值类型冲突、O/I/U |
| `contracts/critters.md` | critters 资源 + sheep + systems/critters 系统逐条业务逻辑 |
| `contracts/vegetation-systems.md` | garden/grass/植被 CPU 数据 + tree-tool/forest/garden 系统 |
| `src/resources/tree_storage.rs` | TreeStorage{map,next_index} + TreeStorageEntry + ForestTree/BirdHaven + 命令 |
| `src/resources/critters.rs` | CritterControl/CritterGoal/CritterVelocityTarget/Multiplier/Velocity/Firefly/Butterfly/ButterflyAudio/Duck/DuckState/Wren/BirdSpots/BirdSpotManager/BirdSpotRecycler/Fish |
| `src/resources/critters/sheep.rs` + `sheep/control.rs` | Sheep 族 + SheepControlSnapshot/SheepSnapshot |
| `src/resources/vegetation.rs` | TerrainHeightsData/GardenMask/WaterExcludeClutterMask/leaf particle/grass SSBO |
| `src/resources/render_loop_critters.rs` | FireflySsbo/ButterflySsbo/FireflyVisibilitySsbo/SheepAnimationSsbo |
| `src/systems/critters.rs` | shared/firefly/butterfly/wren/wren_boids/boids/duck/fish/spawn_ducks + 6 测试 |
| `src/systems/tree_tool.rs` | process_tree_cmds/despawn_trees/hide_trees/tree_move_bend + UI 命令 |
| `src/systems/forest.rs` | handplaced/forest_tree_animation/manage_tree_prefabs |
| `src/systems/garden.rs` | draw_garden/record_garden_history + 植被 CPU 数据接线 |
| `evidence.json` | 反编译地址、bundle SHA、候选复用裁决、关键数值 |
| `evidence/g{1,2,3}-{job,result}.json` | Ghidra 原始返回（tree/critters 资源 + 系统） |

## 本轮恢复要点

- **TreeStorage**（O）：hashbrown map + `next_index`；`insert` 做 `next_index = max(next_index, index+1)`；
  `get` 先查 bucket_mask；`next_index` 自增。map 值类型冲突（8B `{prefab_index,entity}` vs
  enriched `ref_<AutoClutterObject>`）→ **typed gap**，不单边定论。
- **critters 资源**（O）：`CritterControl.control_enabled`（+4 门控）、`BirdSpotManager`（gen_id +8
  自增 / create_spots 池复用）、`Duck::new`（FMOD Quack/TakeOff/Land）、`ButterflyAudio::set_position`。
- **GPU 上限**（O）：Firefly 96、Butterfly 24（`from_iter` panic 断言）。
- **spawn 网格**（O）：Duck 9×4（x lerp -52..+52，z∈{-52,-40.22,-28.9,-18.66}）、Fish 25×25（625）。
- **spawn_ducks 门控**（O）：demo 模式关 且 存档未损坏。
- **候选复用**：`reconstruction/critters`（firefly/shared/duck 核心）与
  `reconstruction/tree-tool`（process_tree_cmds/despawn_trees）按 B 骨架名核对后集成，
  verdict 见 `evidence.json`。

## 验证

- `cargo check --offline`：**0 错误**
- `cargo clippy --all-targets --offline`：**0 错误 / 0 警告**
- `cargo test --offline`：**6 通过**（smooth_velocity/move_critter/duck goal/duck velocity/
  duck spawn grid/fish spawn grid）

全部 `implemented_unverified`：无 runtime/GPU 验收；合同是当前可审阅依据，不因编译通过声称与原作等价。

## 剩余缺口（typed gaps，不臆造）

- `TreeStorageEntry` 精确布局与 map 值类型（`ref_<AutoClutterObject>` vs 8B 记录冲突）。
- 各组件（Critter/Firefly/Butterfly/Duck/DuckState/Wren/Fish/BirdSpot/BirdSpots/Sheep*）字段布局。
- `BirdSpotManager`/`BirdSpotRecycler` 池布局与 `BirdSpot` 字段。
- GPU SSBO（FireflySsbo/ButterflySsbo/FireflyVisibilitySsbo/SheepAnimationSsbo）实例布局。
- boids/wren_boids 规则权重、`update_butterfly_states`/`update_duck_ripples` 算法、
  `handplaced/forest_tree_animation` 相位数值、`tree_move_bend` 公式、`draw_garden` 画笔规则。
- `TerrainHeightsData` 内部布局与 rhapsody 回读面；leaf particle/grass SSBO 布局。
- sheep 各系统（sheep_pet_animation/sheep_walk_animation/sheep_umbrella/sheep_floatie/
  sheep_unstuck/sheep_terrain/ui_pet_sheep/sheep_input_record_replay）逐体重建（本包登记名与边界）。

## 参考（只读，未修改）

- `skeleton/B/crates/country-core/src/resources/{tree_storage,critters}.rs…`
- `skeleton/B/crates/systems/{critters,tree-tool,forest}/…`
- `reconstruction/critters/`、`reconstruction/tree-tool/`（早前恢复候选）
- `evidence/render-passes/INDEX.md`、`cpu-frame/sequence.md`（帧 pass 清单）
