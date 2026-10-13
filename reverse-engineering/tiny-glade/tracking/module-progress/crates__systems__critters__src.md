# crates/systems/critters/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

duck/shared/sheep terrain/wren boids/fish/butterfly候选；firefly和sheep pet独立局部。

## 接线与验证

接线：not_established。

新harness记录22测试；firefly原码报告有失败，不记全通过；其他物种新源码未等价验收。

## 已恢复的函数和类型范围

- duck plan→move→goals→velocities→ripples
- shared避让/移动、sheep_walk_on_terrain
- sim_wren_boids、fish/butterfly更新
- firefly goal/velocity/smoothing与sheep pet局部

## 明确缺口

- CritterLocomotion物种映射、全startup/delete/history/建筑反应
- firefly2102案例中一处NaN符号差异保留
- 真实World/GPU与C物种全链

下一闭包：CritterLocomotion物种映射、全startup/delete/history/建筑反应

## 证据

- [evidence/multi-baseline/README.md](../../evidence/multi-baseline/README.md)
- [evidence/multi-baseline/critters-behavior-analysis.md](../../evidence/multi-baseline/critters-behavior-analysis.md)
- [evidence/multi-baseline/critters-extra-analysis.md](../../evidence/multi-baseline/critters-extra-analysis.md)
- [tracking/parallel-modules-stopped-snapshot.json](../parallel-modules-stopped-snapshot.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/critters/src/boids.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/butterfly.rs` | A,B,C | [reconstruction/critters/src/butterfly.rs](../../reconstruction/critters/src/butterfly.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/critters/src/duck.rs` | A,B,C | [reconstruction/critters/src/duck.rs](../../reconstruction/critters/src/duck.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/critters/src/duck_splash_particle.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/ducks_react_to_being_inside_buildings.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/firefly.rs` | A,B,C | [reconstruction/critters/firefly.rs](../../reconstruction/critters/firefly.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/critters/src/fish.rs` | A,B,C | [reconstruction/critters/src/fish.rs](../../reconstruction/critters/src/fish.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/critters/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/shared.rs` | A,B,C | [reconstruction/critters/src/shared.rs](../../reconstruction/critters/src/shared.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/critters/src/sheep/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/sheep/sheep_behavior.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/sheep/sheep_floatie.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/sheep/sheep_input_record_replay.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/sheep/sheep_pet_animation.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/sheep/sheep_react_to_being_inside_buildings.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/sheep/sheep_terrain.rs` | A,B,C | [reconstruction/critters/src/sheep_terrain.rs](../../reconstruction/critters/src/sheep_terrain.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/critters/src/sheep/sheep_umbrella.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/sheep/sheep_unstuck.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/sheep/sheep_walk_animation.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/sheep/ui_pet_sheep.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/spawn_ducks.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/ui_pet_ducks.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/wren.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/critters/src/wren_boids.rs` | A,B,C | [reconstruction/critters/src/wren_boids.rs](../../reconstruction/critters/src/wren_boids.rs)：same_source_submodule_candidate | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/critters/firefly.rs

[reconstruction/critters/firefly.rs](../../reconstruction/critters/firefly.rs)

SHA256：`b3b83e622fefd2d61eb8f4fc02f1bc89307f7fce762850d105612f021990dba1`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `sample_sphere_game` | 31 | 未由扫描证明；以证据限定范围为准 |
| `update_goal` | 44 | 未由扫描证明；以证据限定范围为准 |
| `set_velocity_target` | 85 | 未由扫描证明；以证据限定范围为准 |
| `smooth_velocity` | 96 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/critters/sheep_pet.rs

[reconstruction/critters/sheep_pet.rs](../../reconstruction/critters/sheep_pet.rs)

SHA256：`dabf5ed7aa69a3a15d338e10b0ef78e66414ebb64b95f346c04fe98cc5dc7a6b`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_observed` | 17 | 未由扫描证明；以证据限定范围为准 |
| `target` | 18 | 未由扫描证明；以证据限定范围为准 |
| `phase` | 19 | 未由扫描证明；以证据限定范围为准 |
| `progress` | 20 | 未由扫描证明；以证据限定范围为准 |
| `consumer_state` | 21 | 未由扫描证明；以证据限定范围为准 |
| `f32_at` | 25 | 未由扫描证明；以证据限定范围为准 |
| `set_phase` | 26 | 未由扫描证明；以证据限定范围为准 |
| `set_f32` | 27 | 未由扫描证明；以证据限定范围为准 |
| `advance_sheep_pet` | 33 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L11：/// C source+60 stores are proven; its other consumer semantics are unknown.
### reconstruction/critters/sheep_pet_c.rs

[reconstruction/critters/sheep_pet_c.rs](../../reconstruction/critters/sheep_pet_c.rs)

SHA256：`091fa41b39b61a61693682360ee0321cd380fff563b75ba47cb9f2355fa0788e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/critters/sheep_pet_runtime.rs

[reconstruction/critters/sheep_pet_runtime.rs](../../reconstruction/critters/sheep_pet_runtime.rs)

SHA256：`92c8d5ee10e7f1ed34acb08c8e87bef582f24133a37957fee288bd31116dffa6`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 21 | 未由扫描证明；以证据限定范围为准 |
| `advance` | 25 | 未由扫描证明；以证据限定范围为准 |
| `consume_locomotion` | 39 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/critters/src/butterfly.rs

[reconstruction/critters/src/butterfly.rs](../../reconstruction/critters/src/butterfly.rs)

SHA256：`23d09fa914f1e7d4ab58691f58b21204e1fd8a19e7323b4009d573d759e10ddf`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `update_goal` | 36 | 未由扫描证明；以证据限定范围为准 |
| `set_velocity_towards_goal` | 87 | 未由扫描证明；以证据限定范围为准 |
| `step_state` | 111 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L58：// state==1 → 0x142f67d14=0x3f5a2e8c(0.8529) 配 (1.0)。两常量对 64 位打包 select，逐分量配对为推断（unknown）。
- L85：/// 证据：`dir = normalize(goal - pos) * 1.77` 写 VelocityTarget；`multiplier += 1.0`（每帧，语义 unknown）。
### reconstruction/critters/src/duck.rs

[reconstruction/critters/src/duck.rs](../../reconstruction/critters/src/duck.rs)

SHA256：`04396a58e3bf5a4cbc49071eb33a4712bdabeba68ade3a55f050295f94aaa312`；缺口注释标记数：6。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_bytes` | 31 | 未由扫描证明；以证据限定范围为准 |
| `update_duck_goal` | 80 | 未由扫描证明；以证据限定范围为准 |
| `set_velocity_towards_goal` | 132 | 未由扫描证明；以证据限定范围为准 |
| `advance_trajectory` | 141 | 未由扫描证明；以证据限定范围为准 |
| `goal_state_from_duck_record` | 146 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L56：/// Duck+0xe1：每帧归零的次标志（语义 unknown）。
- L63：/// 2/3 的确切语义（游水/水面）为推断，标注 unknown。
- L73：///   L685）→ moving=0，高半 `u*0.5+0.5`（低半为寄存器残留，标注 unknown）。
- L75：///   换目标（pick_new；选新目标时原实现另写 Duck+0xe8=0.1，语义 unknown）。
- L105：// 两半均耗尽：写 8B，高半 = u*0.5+0.5；低半为寄存器残留（Ghidra 变量复用，unknown）。
- L131：/// `multiplier` 各系统写查询闭包环境 u32（语义 unknown，见分析文档 §6）。
### reconstruction/critters/src/fish.rs

[reconstruction/critters/src/fish.rs](../../reconstruction/critters/src/fish.rs)

SHA256：`b4fb9dbacc426271be0529c999119f227d8b6f079e56f6e783a3daf9d3e2b8b4`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `frame_update` | 40 | 未由扫描证明；以证据限定范围为准 |
| `fish_spawn_grid_positions` | 102 | 未由扫描证明；以证据限定范围为准 |
| `spawn_boid` | 119 | 未由扫描证明；以证据限定范围为准 |
| `avoid_walls` | 136 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/critters/src/shared.rs

[reconstruction/critters/src/shared.rs](../../reconstruction/critters/src/shared.rs)

SHA256：`040c846ed7dc8294f71d30c7c7eb91fc8291dc8fb4b51f972a7ccd080eb355cb`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `smooth_velocity` | 42 | 未由扫描证明；以证据限定范围为准 |
| `move_critter_core` | 61 | 未由扫描证明；以证据限定范围为准 |
| `face_movement` | 69 | 未由扫描证明；以证据限定范围为准 |
| `decide_avoidance` | 107 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L23：/// CritterVelocityMultiplier 组件（4B f32 @+0）。写查询闭包环境 u32（语义 unknown，见分析 §6）。
- L28：/// `99U >> (v & 0x1f) & 1`（0x63 位集 = {0,1,5,6}）；枚举语义 unknown。
### reconstruction/critters/src/sheep_terrain.rs

[reconstruction/critters/src/sheep_terrain.rs](../../reconstruction/critters/src/sheep_terrain.rs)

SHA256：`f480285cce55c31e67051d22d6b5c97bb04bfcb16e2a5b9edd03a85a3652c02c`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `apply_terrain_follow` | 52 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L17：/// 羊地形状态记录（8B 槽）：`state u32 @ +0`、`f32 @ +4`（@+4 语义 unknown）。
- L21：/// 竖向速度/差记录（12B 槽）：仅 @+4 被读写（反汇编证据）；其余字节 unknown。
### reconstruction/critters/src/wren_boids.rs

[reconstruction/critters/src/wren_boids.rs](../../reconstruction/critters/src/wren_boids.rs)

SHA256：`c12fc7b2136cbc1039c02ace67cb19c6a283d4c0eca4459056852565bb6c0798`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `frame_update` | 49 | 未由扫描证明；以证据限定范围为准 |
| `spawn_position` | 110 | 未由扫描证明；以证据限定范围为准 |
| `shake_burst` | 120 | 未由扫描证明；以证据限定范围为准 |
| `flock_remove` | 127 | 未由扫描证明；以证据限定范围为准 |
