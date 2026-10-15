# systems/critters 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/critters` |
| package | `system-critters` |
| 规模 | 24 文件 / 56 本 ns 函数 / 9 闭包 / **42 业务函数** |
| 难度 | 中 |
| 批次 | 2 |
| 上游依赖 | `country-core`、`dolly`（相机）、`perchance`、`ron`、`fastrand`、`glam`、`utils` |
| 复现工程现状 | MODULE_PROGRESS：**部分候选实现**（10 恢复文件）；duck/shared/sheep terrain/wren boids/fish/butterfly 候选；firefly 与 sheep pet 独立局部；firefly 原码报告**有失败**，不记全通过；`evidence/parallel-critters/`、core 侧 `recovered_systems/critters.rs` |

## 任务表（按 B src 层级）

### 共享 boids / 运动

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-CRITTER-01 | `src/boids.rs` | 恢复 `Boid::random_vel_change`：boid 随机速度扰动（RNG 序列关键）。 | fastrand 1.8.0 | 候选 | 原指令样例 + RNG 序列逐位一致 | 部分 |
| UM-CRITTER-02 | `src/shared.rs` | 恢复 `critters_avoid_collisions`、`smooth_critter_velocities`、`move_critters`：通用避让/平滑/位移。 | 空间/地形 | 候选 | 原指令样例 | 部分 |

### 物种行为

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-CRITTER-03 | `src/duck.rs` | 恢复 `plan_duck_trajectories`、`move_ducks_along_trajectories`、`update_duck_goals`、`set_duck_velocities_towards_goals`、`update_duck_ripples`。 | 水面、共享 boids | 候选 | 原指令样例 + 轨迹/涟漪参数与原一致 | 部分 |
| UM-CRITTER-04 | `src/spawn_ducks.rs` | 恢复 `DuckSpawnLocations::new`、`spawn_ducks`、`despawn_ducks`（`DuckSpawnLocations` 10×10）。 | 水面/地形 | 候选 | 原指令样例 + 10×10 布局与原一致 | 部分 |
| UM-CRITTER-05 | `src/duck_splash_particle.rs` | 恢复 `duck_splash_particle`：鸭子入水粒子。 | GPU/粒子 | 无 | 原指令样例 | 未开始 |
| UM-CRITTER-06 | `src/fish.rs` | 恢复 `fish`、`FishSpawnLocations::new`、`despawn_fish`。 | 水面 | 候选 | 原指令样例 | 部分 |
| UM-CRITTER-07 | `src/butterfly.rs` | 恢复 `update_butterfly_states/goals`、`set_butterfly_velocities_towards_goals`、`update_butterfly_audio`、`butterflies_send_to_gpu`。 | 共享 boids、GPU | 候选 | 原指令样例；GPU 发送链登记 | 部分 |
| UM-CRITTER-08 | `src/firefly.rs` | 恢复 `fireflies_send_to_gpu`、`update_firefly_goals`、`set_firefly_velocities_towards_goals`。**注意：原码报告有失败**。 | 共享 boids、GPU | 有失败记录 | 修复失败项后 ≥ 原指令样例 | 部分 |
| UM-CRITTER-09 | `src/wren_boids.rs` | 恢复 `sim_wren_boids`：鹪鹩 boid 模拟。 | 共享 boids | 无 | 原指令样例 | 未开始 |
| UM-CRITTER-10 | `src/wren.rs` | 恢复 `update_wrens`、`update_decorator_bird_spots`、`update_roof_bird_spots`：鹪鹩行为 + 装饰/屋顶栖点。 | decorator/roof | 无 | 原指令样例 | 未开始 |

### 羊（sheep）

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-CRITTER-11 | `src/sheep/sheep_behavior.rs` | 恢复 `update_sheep_goals`、`set_sheep_velocities_towards_goals`、`update_sheep_audio`。 | 共享 boids、音频 | 候选 | 原指令样例 | 部分 |
| UM-CRITTER-12 | `src/sheep/sheep_terrain.rs` | 恢复 `sheep_walk_on_terrain`：羊在复杂地形上的行走。 | 地形高度 | 候选 | 原指令样例 | 部分 |
| UM-CRITTER-13 | `src/sheep/sheep_unstuck.rs` | 恢复 `sheep_unstuck`：卡住时的解卡逻辑。 | 地形/碰撞 | 无 | 原指令样例 | 未开始 |
| UM-CRITTER-14 | `src/sheep/sheep_react_to_being_inside_buildings.rs` | 恢复 `react_to_being_inside_buildings`、`sheep_reaction_emoji`。 | 建筑占用查询 | 无 | 原指令样例 | 未开始 |
| UM-CRITTER-15 | `src/ducks_react_to_being_inside_buildings.rs` | 恢复 `ducks_react_to_being_inside_buildings`。 | 建筑占用查询 | 无 | 原指令样例 | 未开始 |
| UM-CRITTER-16 | `src/sheep/sheep_floatie.rs` | 恢复 `sheep_floaties`：羊的漂浮物（水上）。 | 水面 | 无 | 原指令样例 | 未开始 |
| UM-CRITTER-17 | `src/sheep/sheep_umbrella.rs` | 恢复 `sheep_umbrellas`：羊的雨伞（下雨）。 | 天气 | 无 | 原指令样例 | 未开始 |
| UM-CRITTER-18 | `src/sheep/sheep_walk_animation.rs` | 恢复 `sheep_walk_animation`。 | — | 无 | 原指令样例 | 未开始 |
| UM-CRITTER-19 | `src/sheep/sheep_pet_animation.rs` | 恢复 `sheep_pet_animation`。 | 输入层 | 局部 | 原指令样例 | 部分 |
| UM-CRITTER-20 | `src/sheep/sheep_input_record_replay.rs` | 恢复 `sheep_input_record_replay`：羊互动的录制/回放。 | core 历史 | 无 | 原入口对照 | 未开始 |

### UI / 插件

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-CRITTER-21 | `src/ui_pet_sheep.rs` | 恢复 `ui_pet_sheep`。 | 输入层 | 局部 | 原指令样例 | 部分 |
| UM-CRITTER-22 | `src/ui_pet_ducks.rs` | 恢复 `ui_pet_ducks`。 | 输入层 | 无 | 原指令样例 | 未开始 |
| UM-CRITTER-23 | `src/lib.rs`、`src/sheep/mod.rs` | 恢复两处插件注册 `impl Plugin::build`。 | 全部上面 | 仅索引 | 与原注册顺序对照 | 未开始 |

## 卡点

- **CritterLocomotion 物种映射**未定（MODULE_PROGRESS 登记）；需先从原侧定位判别值。
- firefly 已有原码报告失败项，需先修复再计通过。
- GPU 发送链（butterflies/fireflies_send_to_gpu）依赖 rhapsody，GPU 消费后置。
- 音频跨切面（update_sheep_audio / update_butterfly_audio）。

## 验收标准

- 每函数 ≥ 原指令样例；boids RNG 序列逐位一致；物种映射有原侧证据。
