# RESTORE — in-module-pure-logic + integration-registration（restorer-purelogic-integration，2026-10-17）

> 角色：restorer-purelogic-integration。范围：`2026-10-17-unimplemented-ledger.md` §5
> in-module-pure-logic（21 条）与 §6 integration-registration（5 条）跨 13 模块。
> 硬标准：fresh 反编译 → 重建真实控制流 → Rust body 逐语句判等；依赖未满足的 core 类型
> 改登记 core-type-gap，不硬造。
>
> fresh 证据（Ghidra shared worker）：`crates/country-core/recovery/agent-team/work/purelogic-integration/`
> - `results/01-critters.json` → `evidence/0x140f8ee90_new.c`（DuckSpawnLocations::new）、
>   `0x140f909f0_new.c`（FishSpawnLocations::new）、`0x14230ff00_plan_duck_trajectories.c`、
>   `0x142312200_move_ducks_along_trajectories.c`、`0x14231c7d0_update_decorator_bird_spots.c`、
>   `0x14231d6c0_update_roof_bird_spots.c`；
> - `results/02-hidetrees-windowlights.json` → `evidence/0x14226a260_hide_trees.c`、
>   `evidence/0x1407b5980_window_lights.c`；
> - `results/00-test.json` → `evidence/0x140f90c10`（despawn_fish）。

## 判定口径

- **实现**：本会话按 fresh 反编译落代码 / 接线，body 逐语句对应。
- **判等**：现有 Rust 体与 fresh 反编译逐语句等价（复核确认）。
- **转 core-gap**：依赖未满足 country-core 类型 → 总账 §1 登记，不硬造。
- **未实现(分歧点)**：body 有 U 语义/结构差异，需更多反编译取证（re-analysis）。

---

## §A in-module-pure-logic（21 条逐条判定）

| 模块 | 条目 | 判定 | 说明 |
|---|---|---|---|
| tree-tool | `hide_trees`（0x14226a260）| **转 core-gap** | fresh decomp：真实 = 两命令缓冲（Area stride 0x18 / NextToWall stride 0x30）滑动窗口 + `Decorators::iter_removed_ids()` 候选 + shape 字节 @+0 bit0 分派（`Circle2d::with_expanded_diameter`/`Rectangle2d::with_expanded_size`）+ @+0x24 门控 → DeleteTreeCmd + History 0x58B。共享 `HideTreesinAreaCmd/NextToWallCmd` 缺 shape/门控字节、系统签名无 Decorators 资源 → 总账 §1 登记（见 §B）。 |
| tree-tool | `ui_move_tree` slider3d case1/2/3 | **转 core-gap** | 依赖 `MoveTreeCmd` 全载荷（槽 0x20 {id,index,position}）落库后重建；共享型仍 `_gap`（总账 §1 已登记）。 |
| critters | `DuckSpawnLocations::new`（0x140f8ee90）| **实现** | components.rs 精确网格重写：行 `(i/9)*52-(1-i/9)*52`（逐 f32 = 十档常量 -52/-40.444443/-28.888891/-17.333334/-5.7777786/…，与 0xc2500000/0xc221c71c/0xc1e71c73/… 逐位一致）、行列序 row-major、timer=25.0（0x41c80000，旧 0 错）。新增 `duck_spawn_steps()` 纯函数。 |
| critters | `FishSpawnLocations::new`（0x140f909f0）| **判等** | fresh decomp 公式 `(k/24)*52-(1-k/24)*52` 与现实现 `((k/24)*2-1)*52` 恒等；行列序 row-major（row@+4→z、col@+8→x）一致。余缺：候选 `used` 字节未建模（DIVERGE 已登记 fish.rs）。 |
| critters | `despawn_fish`（0x140f90c10）| **转 core-gap** | fresh decomp：真实 = 资源重置（`ResMut<FishBoids>` + `ResMut<FishSpawnLocations>`，FishSpawnLocations::new() 重建）；crate `FishBoids` 是 Component 非 Resource + `OnExit<AppLoadingState>` bevy_state 缺 → 总账 §1 登记。 |
| critters | `plan_duck_trajectories`（0x14230ff00）| **未实现** | 36KB fresh decomp：dev 热键 A/B + CursorTerrainRaycast + RaycastWorld::legacy_raycast 两点插值（2.15/0.05）+ Frenet + Trajectory 4-f32 偏移 + spawn 10 组件 bundle + retain closure_env_7。现实现为简化（无 dev 路径）。大段重建 → 转 re-analysis。 |
| critters | `move_ducks_along_trajectories`（0x142312200）| **未实现** | 25KB fresh decomp：Frenet 帧 + 轨迹状态 ∈{0,2}（现 {1,3}）+ 音频链。转 re-analysis。 |
| critters | `update_decorator_bird_spots`（0x14231c7d0）/ `update_roof_bird_spots`（0x14231d6c0）| **未实现** | fresh decomp：双列表滑动窗口缓冲（cursor@+0 + 双 {ptr,len,count} + total@+0x40）+ 装饰物注册表查询 + RoofRidgeDims/calc_roof_tip_center_ls/Rectangle2d edge/gable/ClutterVersion 选边 + init_rng/random_splits + Roof::height + apply_roof_noise + BirdSpotManager::gen_id/create_spots。create_spots 布局/数量规则、random_splits 分段语义 U → 转 re-analysis。 |
| roof | `calculate_roof_trigger_volumes`（0x1421a3220）| **未实现** | 1580 行/12048B：曲面三角化（细分 `(is_flat^1)*5+4`、剖面采样+檐口 −0.56/+0.4、圆形 15 段、UV 2.0/2.2、apply_roof_noise）。逐语句重建需大段对线 → 转 re-analysis。 |
| decorator | `create_or_restore_decorator_backup`（0x1408238a0）| **部分（注册点已接）** | 状态机/门控判等（FIX-6 已复刻 +0x48 pending）；0x30B 三 map 反向拷贝 live 资源身份 U → body 待续（总账 in-module 保留）。调度注册点已接（§C）。 |
| decorator | `window_lights`（0x1407b5980）| **部分** | 常量（0.51/0.44/0.72 read_memory 核验）+ 灯态变化 + 帧末计时/开关 + time_continuous_lerp 已判等；墙/屋顶收集主循环（PublicWalls 双模式 +0xf8/+0xa0/+0x48/+0x110 索引、形状变化写）资源身份 U → 待续。 |
| decorator | ~146 业务函数 | **未实现** | 按族分批 fresh 后重建（blueprint/cull_oob/merge/visual 族/stairs/ui_systems）；不在本批。 |
| clutter | `manage_autoclutter_entities`（0x1422d04e0）| **未实现** | 8178B 宿主查询 +0xf8..+0x120 逐偏移、四元数、ColorId、update_shutter 未复现。转 re-analysis（结构差异）。 |
| clutter | `autoclutter_animation`（0x1422ccef0）| **未实现** | Easing 曲线/四元数 `(eased*dir*60-(1-eased)*dir*50)*0.017453292*0.5`/despawn 未复现。转 re-analysis。 |
| clutter | `add_autoclutter_around_gates/around_windows/add_preplaced/add_tree`（0x1422cc390/0x1422cefe0/0x141ce5900/0x1422d3800）| **未实现** | 门/窗键 tuple、偏移常量、变更检测、门控序列未逐一对。转 re-analysis。 |
| clutter | `request_inter_roof_merlon_audio`（0x1422d4230）| **未实现** | 真实 = `*RCX==0xd` 早退 + CursorTerrainRaycast::position + indexmap Bucket kind 8/9 + spec_extend；现为距离近似。转 re-analysis。 |
| clutter | `detect_inter_roof_merlons`/`detect_intra_shape_corners`/`add_hole_at_shape_intersection`/`spawn_stitches`（0x141ce3a70/0x141cd1a90/0x141cd3ce0/0x141ce65b0）| **部分** | `dirty_walls` 分离接线已完成（§C）；intersect_shapes/WallSpaceCoord/四元数逐语句重建待续。 |
| wall-constructor | `ArchWalker::walk` 族 4 单态（0x141232bb0/0x141233380/0x141231ea0/0x14048ab50）| **未实现** | fresh decomp 已解（travelled 推进/周期 u/尾砖条件）；crate `walk()` 为近似体，逐语句重建需 wall_space subtract+project_clamp。大段重建 → 转 re-analysis（总账已归 in-module，待专门批次）。 |
| wall-constructor | 支撑梁 5 函数 | **未实现** | place_support_beams 双 Grower/add_diagonal_support/emit_support_beams rdp/SupportBeamGrower::propose cost=len*(‖Δdir‖+.5)。大段重建。 |
| wall-constructor | segmentation 3 函数 | **未实现** | process_segment/add_splits_in_empty_range/score_split_candidate（>0.4 阈值/floor(len/span)/2 单位弦）。大段重建。 |
| wall-constructor | 立面支撑整族 + 门/栅栏/地板/楼梯族 + 半木/抹灰族 + utils 近似体族 | **未实现** | 各函数既有 fresh 证据（RESTORE.md 逐函数分歧点）；逐语句重建为专项批次。 |
| toolbar | `mei_pre_update`/`mei_post_update`（0x1406842e0/0x1406884c0）| **未实现** | 输入→handler 状态机 / render 两遍 + taffy 最小尺寸；宿主 mei 消费端 + taffy 依赖未落库。转 re-analysis/external-dep。 |

## §B 转 core-gap 明细（总账 §1 已登记）

1. **hide_trees**：`HideTreesinAreaCmd`/`HideTreesNextToWallCmd` 补 shape 字节（@+0 bit0）+ @+0x24 门控字节；树级 `Decorators` 候选源（`iter_removed_ids`）系统签名接线。证据 `0x14226a260_hide_trees.c`。
2. **despawn_fish**：`FishBoids(Resource)` 类型（现 Component）+ `OnExit<AppLoadingState>`（bevy_state，§3 external-dep）。证据 `0x140f90c10`。
3. **ui_move_tree**：`MoveTreeCmd` 槽 0x20 全载荷（既有登记）。

## §C integration-registration（5 条逐条判定）

| 模块 | 条目 | 判定 | 说明 |
|---|---|---|---|
| decorator | `clear_out_visual_entities` / `clear_out_wall_holes` / `maintain_dormer_tiles` / `decorator_record_history` / `create_or_restore_decorator_backup` | **4/5 实现** | `lib.rs::add_decorator_systems` 改泛型 `<S,C>`，注册 `clear_out_wall_holes`（判等 0x1407f98c0）、`clear_out_visual_entities`（判等 0x1407f9260）、`decorator_record_history::<S>`（0x140817850）、`create_or_restore_decorator_backup::<C>`（0x1408238a0）；`add_decorator_events_and_resources` 幂等 init DecoratorVisualState/DeferredDecoratorEditHistory/DecoratorBackup。`maintain_dormer_tiles` 未注册：纯函数签名非 bevy 系统 + rhapsody 注册表缺口（core-gap）。 |
| clutter | `dirty_walls`（0x141cd16b0）| **实现** | `lib.rs::configure` 链前序注册 `dirty_walls`；`detect_inter_roof_merlons` 改读 `changes.dirty_walls`（移除自带三 EventReader，对齐二进制独立系统分离）。 |
| timberframe | `timberframe_on_glade_unload` | **仍阻断** | `OnExit<AppLoadingState>` bevy_state 未入 workspace（external-dep）；函数已暴露。 |
| water | `draw_water` 注册 + 追加自身事件 | **判等/已验证** | `add_water_cpu_systems` 已注册 `draw_water`（WaterCpuStage::Draw）；`register_water_events` 已注册事件（自足接线）。 |
| toolbar | `ToolbarPlugin`/`Ui2dPlugin` 完整 build | **仍阻断** | 系统/事件未落库（taffy/tiny_http/image 依赖缺）。 |

## §D cargo check 归属

- `cargo check -p system-critters -p system-clutter -p system-decorator`：**均通过（Finished，0 error）**。期间被 country-core 他组并发改动短暂阻断，归属记录：
  - `resources/onboarding.rs:92` E0665（`#[derive(Default)]` on enum TutorialOp 无 `#[default]`）——onboarding 组在途，已由该组闭合（07:05 复检消失）；
  - `systems/ui/signifiers/signifier_factory.rs` E0432——signifier 组已自行修复（06:49 后消失）；
  - `rhapsody_contract::GraphicsDevice` 缺 `Debug` E0277——render 组在途。
- **跨组适配 1 字**（color 组在途 `ChimneyExtra` 字段改 `{f, smoke}`）：本 crate `decorator_visual/chimney_roof.rs` / `chimney_wall.rs` 两处 `as_chimney().0` → `.f`（协作登记，2026-10-17）。
- 本批改动文件（critters/components.rs、decorator/lib.rs、decorator/…/chimney_{roof,wall}.rs、clutter/lib.rs、clutter/detect_inter_roof_merlons.rs）均已随上述通过。
- **tree-tool 现状（非本批，07:07 背景复检）**：`system-tree` 当前 2×E0560（`MoveTreeCmd` 无 `_gap` 字段）+ 3×E0308——core 团队在途扩展 `MoveTreeCmd`（正是 §B 中 ui_move_tree 的 core-gap 项），旧 `_gap` 用法待该组闭合后由 tree-tool restorer 同步，非本批改动所致。

## §E 交付差异摘要

- **实现（代码落库）**：DuckSpawnLocations::new（components.rs，精确网格+timer=25.0）、clutter `dirty_walls` 独立注册 + detect 分离、decorator `add_decorator_systems<S,C>` 接线 4 系统 + 资源 init。
- **判等（复核确认）**：FishSpawnLocations::new（公式/行列序一致）、water `draw_water` 注册。
- **转 core-gap**：hide_trees、despawn_fish、ui_move_tree（依赖未满足类型）。
- **未实现（分歧点/转 re-analysis）**：plan_duck_trajectories、move_ducks_along_trajectories、update_{decorator,roof}_bird_spots、calculate_roof_trigger_volumes（68KB decomp 已取）、clutter 放置/动画/城垛族、wall-constructor 各族、toolbar mei（U 语义/结构差异或大段重建）。
- **仍阻断**：timberframe_on_glade_unload、toolbar build、decorator maintain_dormer_tiles。

## §F 本次批次计数

- **in-module-pure-logic（21）**：实现 2（DuckSpawnLocations::new、FishSpawnLocations::new 判等）；转 core-gap 3（hide_trees / despawn_fish / ui_move_tree）；部分 2（create_or_restore_decorator_backup 注册点、window_lights 常量/帧末）；未实现/转 re-analysis 14（critters 4 + roof 1 + decorator ~146 + clutter 5 + wall-constructor 4 组 + toolbar mei 1）。
- **integration-registration（5）**：已接线 3（decorator 4/5、clutter dirty_walls、water draw_water 验证）；仍阻断 2（timberframe_on_glade_unload、toolbar build）。
- **cargo check**：`system-critters` / `system-clutter` / `system-decorator` **均通过（0 error）**；期间 country-core 他组并发阻断已由各归属组自行闭合（归属见 §D）。
