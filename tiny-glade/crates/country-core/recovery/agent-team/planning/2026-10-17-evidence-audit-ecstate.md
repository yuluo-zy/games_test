# 上层 eco/state 9 模块证据等级审计（2026-10-17）

> 角色：auditor-ecstate（只读汇总；未改 src、未反编译）。
> 范围：glade-border、forest、tree-tool、critters、water、timberframe、roof、color、decorator。
> 来源：各模块 `reports/**/RESTORE.md`、`REVIEW.md` 与 `reports/{wave2,wave2b,wave3,wave4,gap,consume,remaining,unlock,integration,tests}/**`；
> 速查与台账：`planning/2026-10-17-upper-final-progress.md`、`2026-10-17-domain-quickref.md`、`2026-10-17-unimplemented-ledger.md`。

## 0. 口径与判定规则

证据等级（逐函数取**最高正向证据**，后续 wave 复核覆盖早期 verdict）：

1. **未验证**：无实现或仅空壳。
2. **restorer 自判**：只有 RESTORE（或 restorer 轮次报告）声明判等/重建，无独立 REVIEW 对该实现给出正向 verdict。
3. **独立复核判等**：REVIEW 明确给出 `判等` / `fix_verified` / “数据流判等” 等正向结论。
4. **数值断言**：`reports/tests/numeric-assertions/REPORT.md` 或 crate 内单测直接断言该函数行为。
5. **差分案例**：早期模块的原指令/RNE 差分案例（本 9 模块范围内未发现可归入此级的逐函数案例）。

分类规则：

- **A. 判等但无独立复核**：当前“判等”主张只有 restorer 来源，独立 REVIEW 缺失（或仅对旧实现给过否定 verdict，修复后未复验）。
- **B. 重建但未判等**：有实现体（含近似/部分实现），但独立复核为 `未实现-需重做`/`需修`/`重建`，或修复后未再复核。
- **C. 有独立复核且判等**：REVIEW 明确正向 verdict（含 fix_verified、数据流判等）；附登记缺口不改变主判定。
- **D. 未实现**：RESTORE/REVIEW 明确未实现；按“空壳/仅签名/空循环”与“有部分行为”区分，凡有部分行为者归 B。
- **glue**：Bevy 插件注册/SSBO 上传/访问器等非业务，不计入 A-D。
- **shared-utils**：跨模块 utils（rotation_from_normal/orientation_mat3/split_height_into_skirts）单列，不计入 9 模块业务计数。

> 重要口径：`reports/consume`、`reports/remaining`、`reports/unlock` 在 planning 旧文中曾记“不存在”，实际已落盘；本审计以实际文件为准。
> 规划速查表存在滞后（例如 U1 closure$1 标“重建”、U3 行级计数、U5 仅列 draw_water 注册），本审计以逐函数 RESTORE/REVIEW 为准。

## 1. 统计

### 1.1 按模块（仅 9 模块业务函数；不含 glue/shared-utils）

| 模块 | 业务函数 | A | B | C | D | C 占比 |
|---|---:|---:|---:|---:|---:|---:|
| glade-border | 4 | 0 | 2 | 2 | 0 | 50.0% |
| forest | 5 | 0 | 0 | 4 | 1 | 80.0% |
| tree-tool | 13 | 0 | 8 | 5 | 0 | 38.5% |
| critters | 44 | 1 | 23 | 18 | 2 | 40.9% |
| water | 4 | 0 | 0 | 3 | 1 | 75.0% |
| timberframe | 3 | 0 | 2 | 1 | 0 | 33.3% |
| roof | 22 | 0 | 3 | 10 | 9 | 45.5% |
| color | 23 | 0 | 11 | 3 | 9 | 13.0% |
| decorator | 23 | 0 | 8 | 13 | 2 | 56.5% |
| **合计** | **141** | **1** | **57** | **59** | **24** | **41.8%** |

含 decorator `~146` 业务函数族估算：总函数面 **287**；A **1**（0.3%）/ B **57**（19.9%）/ C **59**（20.6%）/ D **170**（59.2%）。

shared-utils 另计：A 1（split_height_into_skirts）、C 2（rotation_from_normal、orientation_mat3，均为 level 4）。

### 1.2 关键结论

- 独立复核判等（C）共 **59** 个显式业务函数，集中在 roof（10）、critters（18）、decorator（13）。
- restorer 重建但未判等（B）共 **57** 个；其中 **8 个**是“复核给出 FIX 后 restorer 已改但无 post-fix 独立复验”，必须补复核才能升 C。
- A 仅 **1** 个（module 内），另有 shared-utils 1 个；A 面很小，说明模块级复核覆盖较完整。
- D 显式 **24** 个；decorator `~146` 族是最大未实现面，且本轮未逐函数 fresh 复核。

## 2. A/B/C/D 清单

### A. 判等但无独立复核（最高正向证据=restorer 自判）

| 模块 | 函数 | VA | 声明 verdict | 证据等级 | 复核缺口 |
|---|---|---|---|---:|---|
| critters | `FishSpawnLocations::new` | 0x140f909f0 | 判等（purelogic restorer 自判） | 2 | 无独立复核；原 module REVIEW 判未实现（行列转置），post-fix 无 REVIEW；used 字节 DIVERGE |
| shared-utils | `split_height_into_skirts` | 0x140b34f80 | 判等（restorer 自判） | 2 | 仅 RESTORE 声明；REVIEW 裁决表未列；无独立复核/数值断言 |

### B. 重建但未判等

| 模块 | 函数 | VA | 声明 verdict | 证据等级 | 最后独立 verdict / 来源 | 缺口 |
|---|---|---|---|---:|---|---|
| glade-border | `display_glade_border` | 0x141be0ad0 | 未实现-需重做（wave4：perimeter 写路径判等，DebugLine::show 缺） | 2 | reports/eco/glade-border/REVIEW.md; reports/wave3/reviews/tg-tr/REVIEW.md §5; reports/wave4/reviews/misc/REVIEW.md §6 | DebugLine::show 0x140b04f70 未落库，折线不落 append-only 流 |
| glade-border | `display_glade_border::closure$1` | 0x141be1080 | 重建（wave3/wave4 链与 2×insert 判等） | 2 | reports/wave3/reviews/tg-tr/REVIEW.md §6; reports/wave4/reviews/misc/REVIEW.md §6 | 唤醒 vtable 0x142efa150 未接线（dead_code） |
| tree-tool | `process_tree_cmds` | 0x142267430 | 重建（wave3 Move/Delete 判等；wave4 G4 修；Insert T2） | 2 | reports/wave3/reviews/tg-tr/REVIEW.md §1; reports/wave4/reviews/geometry/REVIEW.md §2.4; reports/consume/types/RESTORE.md §1 | Insert 0x48 分支、SpawnPrefabCmd 0x50、is_corrupted/drag_started 资源 |
| tree-tool | `replay_tree` | 0x141bc34f0 | 判等→重建（wave4 G2 降级） | 2 | reports/wave4/reviews/geometry/REVIEW.md §2.1; reports/consume/types/RESTORE.md §2 | Move undo 旧位置（edit+0x14/+0x1c）整条事件缺失 |
| tree-tool | `tree_move_bend` | 0x1422689c0 | 未实现-需重做（bend 数学判等） | 2 | reports/eco/tree-tool/REVIEW.md §6 | PrefabInstanceData +0x24/+0x28 step 写面缺失 |
| tree-tool | `hide_trees` | 0x14226a260 | 未实现（core-type-gap） | 2 | reports/remaining/misc/RESTORE.md §3; reports/wave4/reviews/misc/REVIEW.md §9 | shape bit/gate byte/可见性数组/Decorators 候选源 |
| tree-tool | `ui_place_tree` | 0x142269170 | 未实现-需重做（主体判等） | 2 | reports/eco/tree-tool/REVIEW.md §9 | raycast+geo_src_to_edit_subject 早退分支缺失 |
| tree-tool | `place_tree` | 0x142269970 | 未实现-需重做（splitmix/scale/History 判等） | 2 | reports/eco/tree-tool/REVIEW.md §10; reports/integration/coregap-1/REVIEW.md | InsertTreeCmd 0x48 全字段；会话门控硬编码 |
| tree-tool | `ui_move_tree` | 0x14226b1a0 | 重建（wave4 G5/G6 修） | 2 | reports/consume/types/RESTORE.md §3; reports/wave4/reviews/geometry/REVIEW.md §2.5 | 候选源 gap9；History Move 旧位 T3 |
| tree-tool | `ui_delete_tree` | 0x141bd2510 | 未实现-需重做 | 2 | reports/eco/tree-tool/REVIEW.md §12 | PathDrawEvent flag@+0x18 / bVar30 门控 |
| critters | `BoidFlock::remove` | 0x141dc2340/0x141dc2540 | 重建+未实现-需重做（FIX⑥ 部分验证） | 2 | reports/eco/critters/REVIEW.md §2/§9 | boid 越界 break vs 二进制 panic（swap_remove assert）；正常态等价 |
| critters | `move_critters` | 0x142318fa0 | fix_verified+残留→wave4 新 FIX（已改未复验） | 2 | reports/eco/critters/REVIEW.md §9; reports/wave4/reviews/misc/REVIEW.md §2.1; reports/remaining/misc/RESTORE.md §6 | water2 组件 Sheep 已改；纯墙 relate 路径；spherecast 半径参数；PublicWalls panic |
| critters | `critters_avoid_collisions` | 0x1423179a0 | fix_verified（有残留）→wave4 新 FIX（已改未复验） | 2 | reports/eco/critters/REVIEW.md §9; reports/wave4/reviews/misc/REVIEW.md §2.2; reports/remaining/misc/RESTORE.md §6 | water8 两位置 4+4 已改；relate 形状 segment-vs-segment 近似 |
| critters | `DuckSpawnLocations::new` | 0x140f8ee90 | 实现（精确网格+timer=25.0） | 2 | reports/integration/purelogic/RESTORE.md §A | 仅 restorer 实现；无独立 REVIEW（原 module REVIEW 判未实现） |
| critters | `spawn_ducks` | 0x140f8f210 | 重建 | 2 | reports/eco/critters/RESTORE.md（duck+fish 子 agent A） | sorted_by comparator U；demo/corrupted 门未接线 |
| critters | `despawn_ducks` | 0x140f8f8b0 | 重建 | 2 | reports/eco/critters/RESTORE.md | 离场计划=1 点+circle，现 3 点外扩 |
| critters | `despawn_fish` | 0x140f90c10 | 未实现（crate 以实体清理建模） | 2 | reports/eco/critters/REVIEW.md §3; reports/integration/purelogic/RESTORE.md §A | 真实=资源重置（FishBoids Resource + OnExit<AppLoadingState>） |
| critters | `plan_duck_trajectories` | 0x14230ff00 | 未实现（简化实现） | 2 | reports/eco/critters/REVIEW.md §3; reports/integration/purelogic/RESTORE.md §A | legacy_raycast/Frenet/spawn 10 组件 bundle 未重建 |
| critters | `move_ducks_along_trajectories` | 0x142312200 | 未实现（部分） | 2 | reports/eco/critters/REVIEW.md §3 | Frenet 帧简化；轨迹状态 {0,2} vs 现 {1,3} |
| critters | `duck_splash_particle` | 0x142322650 | 重建 | 2 | reports/eco/critters/RESTORE.md | 触发查询 vs 事件语义 |
| critters | `ducks_react_to_being_inside_buildings` | 0x14231b140 | 未实现-需重做（FIX⑧ 改判） | 2 | reports/eco/critters/REVIEW.md §2/§9 | 墙组查询结构 U；2D 射线围合近似；GladeBorder 未接线 |
| critters | `update_butterfly_states` | 0x141d84d40 | 部分判等→wave4 需修（已改未复验） | 2 | reports/wave2/reviews/critters/REVIEW.md §4; reports/wave4/reviews/misc/REVIEW.md §3.2; reports/remaining/misc/RESTORE.md §4 | dispatch flag 基线已改；state4-7/audio u32 写值 U |
| critters | `sim_wren_boids` | 0x142326a80 | 重建（近似） | 2 | reports/eco/critters/REVIEW.md §5 | 出生门精确条件/viable 探针/FreeSpot 认领近似 |
| critters | `update_wrens` | 0x14231e390 | 部分判等→wave4 需修（已改未复验） | 2 | reports/wave2/reviews/critters/REVIEW.md §5; reports/wave4/reviews/misc/REVIEW.md §1.2; reports/remaining/misc/RESTORE.md §5 | h0 模式门/阈值已改；h1 音频槽/实体映射/PlacementDataSsbo |
| critters | `update_sheep_goals` | 0x142315050 | 重建（主流程一致） | 2 | reports/eco/critters/REVIEW.md §5 | XMM7/8 寄存器残留、flag_31 死分支 |
| critters | `sheep_unstuck` | 0x1423246b0 | 重建（DIVERGE） | 2 | reports/eco/critters/REVIEW.md §5 | spherecast ABI/rock 逐点采样/is_corrupted |
| critters | `react_to_being_inside_buildings` | 0x14231f610 | 未实现（部分重建） | 2 | reports/eco/critters/REVIEW.md §5 | emoji/UI spawn glue 之外主体不全 |
| critters | `sheep_reaction_emoji` | 0x14231ff70 | 重建 | 2 | reports/eco/critters/REVIEW.md §5 | EasingFunction 变体未取证 |
| critters | `sheep_floaties` | 0x142320cf0 | 重建（DIVERGE） | 2 | reports/eco/critters/REVIEW.md §5 | spawn bundle glue；DIVERGE |
| critters | `sheep_umbrellas` | 0x142321980 | 重建（DIVERGE） | 2 | reports/eco/critters/REVIEW.md §5 | 0x1a 掩码/翻向音频 U |
| critters | `sheep_input_record_replay` | 0x14231b720 | 重建（DIVERGE） | 2 | reports/eco/critters/REVIEW.md §5 | 会话资源 +0x40 身份 U |
| critters | `ui_pet_sheep` | 0x142324f80 | 重建核心+未实现（UI） | 2 | reports/eco/critters/RESTORE.md/REVIEW.md §5 | UiWidgetContext/SignifierEntityBuilder/slider3d API 缺 |
| critters | `ui_pet_ducks` | 0x142322e30 | 重建核心+未实现（UI） | 2 | reports/eco/critters/RESTORE.md/REVIEW.md §5 | UI gate/eye 距离/signifier/audio 未接线 |
| timberframe | `construct_timberframe_rectangle` | 0x141a41ba0 | 重建+写路径（相位/计数判等） | 2 | reports/state/timberframe/REVIEW.md §1; reports/wave3/reviews/tg-tr/REVIEW.md §8 | RenderInstance 字段序 I；bit0 代理；ResMut 库面；rotation_from_normal→beam_rotation I |
| timberframe | `construct_timberframe_circle` | 0x1421b67a0 | 重建+写路径（同上） | 2 | reports/state/timberframe/REVIEW.md §2; reports/wave3/reviews/tg-tr/REVIEW.md §9 | 主环 Quat::IDENTITY 占位 GAP；不 clear 忠实 |
| roof | `generate_roof_stone_floor_and_roof_bottom` | 0x1421a6d40 | 重建（CPU 控制流判等；命令执行体未实现） | 2 | reports/state/roof/REVIEW.md §1.8 | NaniBundle<StoneFloorInstanceData> insert 执行体未实现 |
| roof | `highlight_roof（iter 单态）` | 0x141a284b0 | 未实现→重建（数据流判等） | 2 | reports/wave3/reviews/tg-tr/REVIEW.md §11; reports/state/roof/REVIEW.md §3.2 | 8B 写覆盖 pad 不可表示；iter/store 读出偏移不同 |
| roof | `highlight_roof（store 单态）` | 0x141a31280 | 未实现→重建（数据流判等） | 2 | reports/wave3/reviews/tg-tr/REVIEW.md §11; reports/state/roof/REVIEW.md §3.2 | 同上 |
| color | `init_color_ids_buffer` | 0x142290750 | 未实现-需重做（SSBO 段阻塞；insert 面判等） | 2 | reports/state/color/REVIEW.md 总表 | SSBO add_new×4 接线；coregap-2 已补类型但函数未复验 |
| color | `set_wall_color_id` | 0x142290820 | 未实现-需重做（SSBO 段阻塞；CPU 面判等） | 2 | reports/state/color/REVIEW.md 总表 | SSBO get/update_buffer_bytes；selector/value U |
| color | `set_gate_color_id` | 0x142299840 | 未实现-需重做（SSBO 段阻塞；CPU 面判等） | 2 | reports/state/color/REVIEW.md 总表 | SSBO 段；custom==1 已修 |
| color | `set_halftimber_color_id` | 0x142299250 | 未实现-需重做（SSBO 段阻塞；CPU 面判等） | 2 | reports/state/color/REVIEW.md 总表 | SSBO 段；custom==1 已修 |
| color | `set_decorator_color_id` | 0x1422af0c0 | 未实现-需重做（记录 value U 面） | 2 | reports/state/color/REVIEW.md 总表 | 5 层分派/写面判等；记录 value 非 sub==4 为栈值 U |
| color | `set_flag_connection_and_chimney_smoke` | 0x142298970 | 未实现-需重做（烟值 +4 阻塞） | 2 | reports/state/color/REVIEW.md 总表 | flag 面判等；ChimneyExtra +4 写面 coregap-2 已补未复验 |
| color | `set_roof_customization` | 0x142299dd0 | 未实现（密集表阻塞；外环近似） | 2 | reports/state/color/REVIEW.md 总表; reports/unlock/publicwalls-tables/RESTORE.md | PublicWalls 稀疏位集/密集表；外环第二写遗漏 |
| color | `show_color_options` | 0x14229a7f0 | 重建（结构+case0/1 载荷；其余 GAP） | 2 | reports/wave2/reviews/color/REVIEW.md; reports/wave2/color/RESTORE.md | case0 载荷修正后无独立复验；case2..0x26/demo/事件写面 GAP |
| color | `hide_color_menu` | 0x1422adcf0 | 未实现（事件门控已修） | 2 | reports/state/color/REVIEW.md 总表（FIX-2） | just_pressed 动作名/图标目录 typed-gap |
| color | `autoclose_color_menu` | 0x142298d30 | 未实现（stub 非判等） | 2 | reports/state/color/REVIEW.md 总表（FIX-3） | menu.center 近似、省略光标 2D 跳过、kind 直闭 |
| color | `ColorRadialMenu::create` | 0x1425f27c0 | 未实现（仅数据组装） | 2 | reports/state/color/REVIEW.md 总表 | UI 实体 spawn typed-gap |
| decorator | `maintain_dormer_tiles` | 0x1407ce920 | 未实现→判等（G3 已修未复验） | 2 | reports/state/decorator/REVIEW.md §一-9; reports/wave4/reviews/geometry/REVIEW.md §2.3; reports/consume/types/RESTORE.md §5 | 游标写序已修；id 单槽 store 已修；rhapsody 注册表/RoofTileSsbo drop 未接线 |
| decorator | `apply_decorator_op` | 0x141276e30 | 未实现-需重做（控制流判等；叶近似） | 2 | reports/state/decorator/REVIEW.md §二-1 | archive_decorator_state/DecoratorLocator::new/build_op_event 0x88B 叶未还原 |
| decorator | `create_or_restore_decorator_backup` | 0x1408238a0 | 未实现-需重做（状态机判等；恢复叶近似） | 2 | reports/state/decorator/REVIEW.md §二-2 | 0x30B 三 map 反向拷贝未接线；桶模型近似 |
| decorator | `decorator_record_history` | 0x140817850 | 未实现-需重做（循环/清空判等；载荷映射错） | 2 | reports/state/decorator/REVIEW.md §二-3 | 48B {1,1,kind,f1,f2,f3} kind/f2/f3 丢弃 |
| decorator | `window_lights` | 0x1407b5980 | 未实现-需重做（常量/灯态/帧末判等；收集主循环缺失） | 2 | reports/state/decorator/REVIEW.md §二-4; reports/wave3/reviews/toolbar-decorator/REVIEW.md §1 | PublicWalls 密集窗桶收集主循环；rng 种子/retain 谓词/lerp 实参 U |
| decorator | `chimney_smoke_anim_thickness` | 0x140818df0 | 未实现-需重做（窗口/目标/更新公式判等；收集缺失） | 2 | reports/state/decorator/REVIEW.md §二-5; reports/wave3/reviews/toolbar-decorator/REVIEW.md §1 | spot 收集主循环；rng 种子/retain 谓词 U |
| decorator | `replay_decorator` | 0x14080c8d0 | 未实现（结构判等；core-gap 诚实壳） | 2 | reports/wave3/reviews/toolbar-decorator/REVIEW.md §4; reports/unlock/replay-decorator/RESTORE.md | ReplayCommand Decorator 变体/ReplayBucket cull 表未落库 |
| decorator | `calc_gothic_half_dormer_profile_ws` | 0x1407eb160 | 重建+部分未实现 | 2 | reports/wave2/decorator-forest/RESTORE.md §1; reports/wave2/reviews/geometry/REVIEW.md §3.2 | wall-space A/B/C/D lerp/法线/0..2 索引语义未实现 |

### C. 有独立复核且判等（合规）

| 模块 | 函数 | VA | 证据等级 | 复核来源 | 已登记缺口 |
|---|---|---|---:|---|---|
| glade-border | `display_glade_border::closure$0` | inline（宿主 from_iter 0x141be6110） | 3 | reports/eco/glade-border/REVIEW.md §2 | — |
| glade-border | `feedback_input_out_of_border` | inline（宿主 0x141be2280） | 3 | reports/wave3/reviews/tg-tr/REVIEW.md §7; reports/gap/review/REVIEW.md #1 | — |
| forest | `manage_tree_prefabs` | 0x141bf16c0 | 3 | reports/eco/forest/REVIEW.md §2/§9; reports/wave4/reviews/misc/REVIEW.md §4 | 记录 0x50B/实体位/+0x44、SpawnPrefabCmd 0x50 未承载 |
| forest | `handplaced_tree_animation` | 0x142279500 | 3 | reports/eco/forest/REVIEW.md §4/§9 | 输出指针写未建模；盒扩展 SIMD lane 近似；TreeAnimation[+4] 以 Local 承载 |
| forest | `forest_tree_animation` | 0x14227a0d0 | 3 | reports/eco/forest/REVIEW.md §5 | +0x44 清标志、DespawnPrefabCmd 载荷为 core 类型缺口 |
| forest | `is_too_close_to_shape` | inline（宿主 0x141bf1d6a 区） | 3 | reports/eco/forest/REVIEW.md §3/§9 | 矩形 closest_point_on_perimeter 分解；数值逐指令等价未证 |
| tree-tool | `despawn_trees` | 0x1422686d0 | 4 | reports/eco/tree-tool/REVIEW.md §2; reports/tests/numeric-assertions/REPORT.md | TreeAnimation α@+4 布局分歧 5 |
| tree-tool | `convert_preplaced_trees` | 0x14226be70 | 3 | reports/wave4/reviews/misc/REVIEW.md §5 | position 源/scale-rotation 记录/候选位图/rock buffer 登记 |
| tree-tool | `recalc_tree_position_on_terrain_height_change` | 0x142268dc0 | 4 | reports/eco/tree-tool/REVIEW.md §5; reports/tests/numeric-assertions/REPORT.md | — |
| tree-tool | `random_tree_prefab` | 0x14226a170 | 4 | reports/eco/tree-tool/REVIEW.md §8; reports/tests/numeric-assertions/REPORT.md | — |
| tree-tool | `reset_tree_tool` | inline（PDB inlinee） | 3 | reports/eco/tree-tool/REVIEW.md §13 | — |
| critters | `Boid::random_vel_change` | 0x141dc2170 | 3 | reports/eco/critters/REVIEW.md §1 | RAY_DIRS 32 方向字面量未提取 |
| critters | `Boid::frame_update<fish>` | 0x141dc2740 | 3 | reports/eco/critters/REVIEW.md §1/§9 | RAY_DIRS 字面量；query 环境值写未建模 |
| critters | `Boid::frame_update<wren>` | 0x141dc3510 | 3 | reports/eco/critters/REVIEW.md §1/§9 | RAY_DIRS 字面量；viable 探针近似 |
| critters | `apply_fish_motion` | helper（非独立二进制函数） | 3 | reports/eco/critters/REVIEW.md §1 | — |
| critters | `smooth_critter_velocities` | 0x142318c10 | 3 | reports/eco/critters/REVIEW.md §1 | powf(0,dt<0) QNaN 符号位差已登记 |
| critters | `update_firefly_goals` | 0x142316940 | 3 | reports/eco/critters/REVIEW.md §1 | query 环境值写未建模 |
| critters | `set_firefly_velocities_towards_goals` | 0x142317040 | 3 | reports/eco/critters/REVIEW.md §1 | query 环境值写未建模 |
| critters | `fish` | 0x140f90cd0 | 3 | reports/eco/critters/REVIEW.md §9 | FishBoids Component 非 Resource；光标/相机逃避未做 |
| critters | `update_duck_goals` | 0x142313b30 | 3 | reports/eco/critters/REVIEW.md §9 | aux 恒 0；within_glade dev 门 false；旋转精确四元数 U |
| critters | `set_duck_velocities_towards_goals` | 0x142314470 | 3 | reports/eco/critters/REVIEW.md §1 | query 环境值写未建模 |
| critters | `update_butterfly_goals` | 0x141d86060 | 3 | reports/eco/critters/REVIEW.md §1 | per-entity 4B 查询环境写未建模 |
| critters | `set_butterfly_velocities_towards_goals` | 0x141d867b0 | 3 | reports/wave2/reviews/critters/REVIEW.md §4.3 | 动画跳表 handler DAT_142f68018 U |
| critters | `update_butterfly_audio` | 0x141d86eb0 | 3 | reports/eco/critters/REVIEW.md §1 | — |
| critters | `set_sheep_velocities_towards_goals` | 0x142315e40 | 3 | reports/eco/critters/REVIEW.md §1 | query 环境值写未建模 |
| critters | `update_sheep_audio` | 0x142316490 | 3 | reports/eco/critters/REVIEW.md §1 | 音频记录 +0x34+=dt 未建模（DIVERGE） |
| critters | `sheep_walk_on_terrain` | 0x14231bd80 | 3 | reports/eco/critters/REVIEW.md §1 | case4 lerp rate 符号 U |
| critters | `sheep_walk_animation` | 0x14231a630 | 4 | reports/wave2/reviews/critters/REVIEW.md §2; reports/tests/numeric-assertions/REPORT.md | 音频槽写未建模 |
| critters | `sheep_pet_animation` | 0x14231ac20 | 3 | reports/wave2/reviews/critters/REVIEW.md §3 | 音频槽写未建模 |
| water | `draw_water` | 0x141ba9130 | 3 | reports/state/water/REVIEW.md §1 | 橡皮 signifier name 布局疑点；RasterStrokeCmd.strength 建模补充 |
| water | `update_water_audio_grid` | 0x141baefe0 | 3 | reports/state/water/REVIEW.md §4 | intersects_any max=1.0 语义差；WaterAudioGrid 语义投影 |
| water | `record_water_history` | inline（宿主 0x140e92a80） | 3 | reports/state/water/REVIEW.md §5 | — |
| timberframe | `timberframe_beam_count_change_audio` | 0x1421b6610 | 4 | reports/state/timberframe/REVIEW.md §3; reports/tests/numeric-assertions/REPORT.md | get_building_size_01 size/id 缺；GameClock +0x2c 布局 GAP |
| roof | `simulate_roof_tile_for_audio` | 0x1421a7090 | 3 | reports/state/roof/REVIEW.md §1.5; reports/wave3/reviews/tg-tr/REVIEW.md §10 | RoofTileSimulation 单记录 vs 0x18 数组；RoofSound 载荷；GameClock.delta |
| roof | `wall_height_dt` | 0x1421a2b60 | 3 | reports/state/roof/REVIEW.md §1.1 | — |
| roof | `roof_animation` | 0x1421a2da0 | 4 | reports/state/roof/REVIEW.md §1.2; reports/tests/numeric-assertions/REPORT.md | — |
| roof | `roof_record_history` | 0x141a37fd0 | 3 | reports/state/roof/REVIEW.md §1.3 | 生产端 UI 未接线 |
| roof | `replay_roof` | 0x141a37ad0 | 3 | reports/state/roof/REVIEW.md §1.4 | 完成位 byte[21] 映射 I |
| roof | `replay_roof::impl$0::process` | 0x1421b48e0 | 3 | reports/state/roof/REVIEW.md §1.4 | previous_direction I；非法 tag Err 路径 |
| roof | `switch_roof_ridge_direction` | 0x141a3a730 | 3 | reports/state/roof/REVIEW.md §1.6 | RoofSound 载荷布局 core 面 |
| roof | `record_switch_roof_ridge_history` | 0x141a3ac20 | 3 | reports/state/roof/REVIEW.md §1.7 | — |
| roof | `switch_roof_type` | 0x1421ac4f0 | 3 | reports/state/roof/REVIEW.md §2.1（非 mesh 判等）; reports/wave4/reviews/visual/REVIEW.md §2（mesh 5/5） | 整函数无单一 REVIEW 判等；由两次独立复核拼合 |
| roof | `gpu_flush_buffers` | 0x140de50a0 | 3 | reports/wave4/reviews/geometry/REVIEW.md §2.2 | GraphicsDeviceResource +0x188 管线缓存槽 GAP |
| color | `IconInfo::new_separator` | 0x1425f2680 | 3 | reports/state/color/REVIEW.md 总表 | — |
| color | `IconInfo::new_flat_roof_rim_ty` | 0x1425f26f0 | 3 | reports/state/color/REVIEW.md 总表 | ClutterVersion::is_initial 输入源 U |
| color | `set_window_light` | 0x142298260 | 3 | reports/state/color/REVIEW.md 总表 | 表头 verdict 与正文矛盾；按正文“整体判等”取 C |
| decorator | `DecoratorBackup::clear` | 0x140823660 | 3 | reports/state/decorator/REVIEW.md §一-1 | — |
| decorator | `clear_out_wall_holes` | 0x1407f98c0 | 3 | reports/state/decorator/REVIEW.md §一-2 | — |
| decorator | `ui_decorator_inactive` | 0x140791400 | 3 | reports/state/decorator/REVIEW.md §一-3 | reset_interaction 额外置 active=true（I 级模型 artifact） |
| decorator | `stairs_reset_for_new_frame` | 0x14079c350 | 3 | reports/state/decorator/REVIEW.md §一-4 | unset 落点 [内层+0x10]=3 以 mark_invalid 承载（I） |
| decorator | `seal_persistent_entities` | 0x140811e80 | 3 | reports/state/decorator/REVIEW.md §一-5 | — |
| decorator | `add_decorator_visual_entity impl$4` | 0x1408119e0 | 3 | reports/state/decorator/REVIEW.md §一-6 | cap=8 预分配差异 minor |
| decorator | `DecoratorPersistentVisualEntities::entities` | 0x1408119d0 | 3 | reports/state/decorator/REVIEW.md §一-7 | — |
| decorator | `DecoratorPersistentVisualState::entities` | 0x14067dd80 | 3 | reports/state/decorator/REVIEW.md §一-7 | — |
| decorator | `clear_out_visual_entities` | 0x1407f9260 | 3 | reports/state/decorator/REVIEW.md §七 | param_2 身份 U 登记 |
| decorator | `add_decorator_visual_entity impl$5` | 0x140811c30 | 3 | reports/wave4/reviews/visual/REVIEW.md §1.4; reports/consume/nani-decorator/RESTORE.md §2.4 | wave3 autoclutter 表结论被机器码否证；类型 stand-in |
| decorator | `WindowVisualEntityBuilder::build<Transient>` | 0x1412ab270 | 3 | reports/consume/nani-decorator/RESTORE.md §0/§2; reports/wave4/reviews/visual/REVIEW.md §1 | 12 个条件组件以 stand-in 承载；builder 字段名 U |
| decorator | `WindowVisualEntityBuilder::build<Persistent>` | 0x1412a9b30 | 3 | reports/consume/nani-decorator/RESTORE.md §0/§2; reports/wave4/reviews/visual/REVIEW.md §1 | 同上 |
| decorator | `gothic_half_dormer_radius` | 0x1412ac9b0（call_once 内联） | 3 | reports/wave2/decorator-forest/RESTORE.md §1; reports/wave2/reviews/geometry/REVIEW.md §3.1 | rank∉1..3 panic vs 默认 0.38（低影响登记） |

### D. 未实现

| 模块 | 函数 | VA | 证据等级 | 登记来源 | 解锁条件摘要 |
|---|---|---|---:|---|---|
| forest | `get_gothic_hat_bones` | 0x1407eb500 | 1 | reports/eco/forest/RESTORE.md; planning quickref U2 | 骨架主体待取证 |
| critters | `update_decorator_bird_spots` | 0x14231c7d0 | 1 | reports/eco/critters/REVIEW.md §3 | create_spots 布局/数量、双列表滑动窗口 |
| critters | `update_roof_bird_spots` | 0x14231d6c0 | 1 | reports/eco/critters/REVIEW.md §3 | random_splits 分段语义/停靠分布 |
| water | `draw_water::closure$0` | 0x141ba9700 | 1 | reports/state/water/REVIEW.md §2 | SignifierEntityBuilder 链 + brush_circle_scribble 未落库 |
| roof | `calculate_roof_trigger_volumes` | 0x1421a3220 | 1 | reports/state/roof/REVIEW.md §3.1 | 曲面三角化/剖面/檐口/UV/噪声未重建 |
| roof | `ui_edit_roof` | 0x141a18dd0 | 1 | reports/state/roof/REVIEW.md §3.4 | 三角拾取/WidgetParams/HighlightRoofCmd |
| roof | `do_ui_edit_roof` | 0x141a19a90 | 1 | reports/state/roof/REVIEW.md §3.4 | 三 case 历史/拖拽量 |
| roof | `ui_edit_roof_ridge_dir` | 0x141a386b0 | 1 | reports/state/roof/REVIEW.md §3.5 | iter_removed_ids/to_world_space/box_trigger_volume |
| roof | `do_ui_edit_roof_ridge_dir` | 0x141a39490 | 1 | reports/state/roof/REVIEW.md §3.5/§7 | 方向位 ^1 / SwitchRoofRidgeDirCmd |
| roof | `ui_edit_roof_ridge_length` | 0x1421a8210 | 1 | reports/state/roof/REVIEW.md §3.5 | TweakerView/project_cursor_onto_ray |
| roof | `do_ui_edit_roof_ridge_length` | 0x1421a8c50 | 1 | reports/state/roof/REVIEW.md §3.5/§7 | inverse_ridge_01_param 投影链 |
| roof | `create_ui_widget` | 0x1421aa170 | 1 | reports/state/roof/REVIEW.md §3.5 | widget 描述 0x18/分发骨架 |
| roof | `ui_edit_roof_ridge_and_height_focused` | 0x1421ad440 | 1 | reports/state/roof/REVIEW.md §3.5/§7 | CameraMatrices/project_cursor_onto_ray/inverse_lerp |
| color | `write_roof_color_id_to_gpu` | 0x142298520 | 1 | reports/state/color/REVIEW.md 总表 | 双源逐元素 update_buffer_bytes；RoofColorIdSsbo 已补但函数未落 |
| color | `color_menu` | 0x142291230 | 1 | reports/state/color/REVIEW.md 总表; reports/wave2/reviews/color/REVIEW.md | CustomizationIcon 0xa0/Events 类型/12 参 ABI typed-gap |
| color | `color_button_animations` | 0x1422979b0 | 1 | reports/state/color/REVIEW.md 总表 | CustomizationIcon/Ui2dExtent typed-gap |
| color | `delete_color_icon_entities` | 0x142297ff0 | 1 | reports/state/color/REVIEW.md 总表 | progress<=0 门控 typed-gap |
| color | `ui_color_wall_and_roof` | 0x141c9ab70 | 1 | reports/state/color/REVIEW.md 总表 | WidgetParams/raycast +0x1d8/closure |
| color | `color_cursor_icon` | 0x141c9b510 | 1 | reports/state/color/REVIEW.md 总表 | SignifierEntityBuilder/UiInactive/Ui2d* |
| color | `ui_color_gates` | 0x1422afe70 | 1 | reports/state/color/REVIEW.md 总表 | 稀疏墙位集/mesh 三角形/signifier |
| color | `ui_color_windows_and_flags` | 0x1422b0e00 | 1 | reports/state/color/REVIEW.md 总表 | legacy_raycast/StairsState graph/outline |
| color | `replay_color` | 0x141c7f270 | 1 | reports/state/color/REVIEW.md 总表; reports/unlock/replay-decorator/RESTORE.md §5.4 | ReplayBuffer 原始字节/13 写者派发 |
| decorator | `get_gothic_hat_bones` | 0x1407eb500 | 1 | reports/state/decorator/RESTORE.md; planning quickref U9 | 骨架主体待取证 |
| decorator | `~146 业务函数（blueprint/cull_oob/merge/visual 族/stairs/ui_systems）` | 族（未逐一列 VA） | 1 | reports/state/decorator/RESTORE.md 判定总表末行; planning 2026-10-17-unimplemented-ledger.md | 族估算 ~146；含 DecoratorBackup/visual 壳（chimney/door_stairs_assemble_bricks）；未逐函数审计 |
| decorator | `~146 业务函数族`（blueprint/cull_oob/merge/visual 族/stairs/ui_systems） | 族（未逐一列 VA） | 1 | decorator RESTORE 末行；unimplemented-ledger | 按族分批 fresh 后重建；族估算 ~146 |

## 3. 全量逐函数表（模块 | 函数 | VA | 声明 verdict | 证据等级 | 分类 | 复核来源 | 缺口）

| 模块 | 函数 | VA | 声明 verdict | 等级 | 类 | 复核来源 | 缺口 |
|---|---|---|---|---:|---|---|---|
| glade-border | `display_glade_border` | 0x141be0ad0 | 未实现-需重做（wave4：perimeter 写路径判等，DebugLine::show 缺） | 2 | B | reports/eco/glade-border/REVIEW.md; reports/wave3/reviews/tg-tr/REVIEW.md §5; reports/wave4/reviews/misc/REVIEW.md §6 | DebugLine::show 0x140b04f70 未落库，折线不落 append-only 流 |
| glade-border | `display_glade_border::closure$0` | inline（宿主 from_iter 0x141be6110） | 判等 | 3 | C | reports/eco/glade-border/REVIEW.md §2 | — |
| glade-border | `display_glade_border::closure$1` | 0x141be1080 | 重建（wave3/wave4 链与 2×insert 判等） | 2 | B | reports/wave3/reviews/tg-tr/REVIEW.md §6; reports/wave4/reviews/misc/REVIEW.md §6 | 唤醒 vtable 0x142efa150 未接线（dead_code） |
| glade-border | `feedback_input_out_of_border` | inline（宿主 0x141be2280） | 判等 | 3 | C | reports/wave3/reviews/tg-tr/REVIEW.md §7; reports/gap/review/REVIEW.md #1 | — |
| forest | `manage_tree_prefabs` | 0x141bf16c0 | 重建+fix_verified；构造形状判定判等 | 3 | C | reports/eco/forest/REVIEW.md §2/§9; reports/wave4/reviews/misc/REVIEW.md §4 | 记录 0x50B/实体位/+0x44、SpawnPrefabCmd 0x50 未承载 |
| forest | `handplaced_tree_animation` | 0x142279500 | 重建+fix_verified | 3 | C | reports/eco/forest/REVIEW.md §4/§9 | 输出指针写未建模；盒扩展 SIMD lane 近似；TreeAnimation[+4] 以 Local 承载 |
| forest | `forest_tree_animation` | 0x14227a0d0 | 判等（算法控制流） | 3 | C | reports/eco/forest/REVIEW.md §5 | +0x44 清标志、DespawnPrefabCmd 载荷为 core 类型缺口 |
| forest | `is_too_close_to_shape` | inline（宿主 0x141bf1d6a 区） | 重建+fix_verified（已接线） | 3 | C | reports/eco/forest/REVIEW.md §3/§9 | 矩形 closest_point_on_perimeter 分解；数值逐指令等价未证 |
| forest | `get_gothic_hat_bones` | 0x1407eb500 | 未实现 | 1 | D | reports/eco/forest/RESTORE.md; planning quickref U2 | 骨架主体待取证 |
| tree-tool | `process_tree_cmds` | 0x142267430 | 重建（wave3 Move/Delete 判等；wave4 G4 修；Insert T2） | 2 | B | reports/wave3/reviews/tg-tr/REVIEW.md §1; reports/wave4/reviews/geometry/REVIEW.md §2.4; reports/consume/types/RESTORE.md §1 | Insert 0x48 分支、SpawnPrefabCmd 0x50、is_corrupted/drag_started 资源 |
| tree-tool | `despawn_trees` | 0x1422686d0 | 判等 | 4 | C | reports/eco/tree-tool/REVIEW.md §2; reports/tests/numeric-assertions/REPORT.md | TreeAnimation α@+4 布局分歧 5 |
| tree-tool | `convert_preplaced_trees` | 0x14226be70 | 判等（wave4-misc） | 3 | C | reports/wave4/reviews/misc/REVIEW.md §5 | position 源/scale-rotation 记录/候选位图/rock buffer 登记 |
| tree-tool | `replay_tree` | 0x141bc34f0 | 判等→重建（wave4 G2 降级） | 2 | B | reports/wave4/reviews/geometry/REVIEW.md §2.1; reports/consume/types/RESTORE.md §2 | Move undo 旧位置（edit+0x14/+0x1c）整条事件缺失 |
| tree-tool | `recalc_tree_position_on_terrain_height_change` | 0x142268dc0 | 判等 | 4 | C | reports/eco/tree-tool/REVIEW.md §5; reports/tests/numeric-assertions/REPORT.md | — |
| tree-tool | `tree_move_bend` | 0x1422689c0 | 未实现-需重做（bend 数学判等） | 2 | B | reports/eco/tree-tool/REVIEW.md §6 | PrefabInstanceData +0x24/+0x28 step 写面缺失 |
| tree-tool | `hide_trees` | 0x14226a260 | 未实现（core-type-gap） | 2 | B | reports/remaining/misc/RESTORE.md §3; reports/wave4/reviews/misc/REVIEW.md §9 | shape bit/gate byte/可见性数组/Decorators 候选源 |
| tree-tool | `random_tree_prefab` | 0x14226a170 | 判等 | 4 | C | reports/eco/tree-tool/REVIEW.md §8; reports/tests/numeric-assertions/REPORT.md | — |
| tree-tool | `ui_place_tree` | 0x142269170 | 未实现-需重做（主体判等） | 2 | B | reports/eco/tree-tool/REVIEW.md §9 | raycast+geo_src_to_edit_subject 早退分支缺失 |
| tree-tool | `place_tree` | 0x142269970 | 未实现-需重做（splitmix/scale/History 判等） | 2 | B | reports/eco/tree-tool/REVIEW.md §10; reports/integration/coregap-1/REVIEW.md | InsertTreeCmd 0x48 全字段；会话门控硬编码 |
| tree-tool | `ui_move_tree` | 0x14226b1a0 | 重建（wave4 G5/G6 修） | 2 | B | reports/consume/types/RESTORE.md §3; reports/wave4/reviews/geometry/REVIEW.md §2.5 | 候选源 gap9；History Move 旧位 T3 |
| tree-tool | `ui_delete_tree` | 0x141bd2510 | 未实现-需重做 | 2 | B | reports/eco/tree-tool/REVIEW.md §12 | PathDrawEvent flag@+0x18 / bVar30 门控 |
| tree-tool | `reset_tree_tool` | inline（PDB inlinee） | 判等 | 3 | C | reports/eco/tree-tool/REVIEW.md §13 | — |
| critters | `Boid::random_vel_change` | 0x141dc2170 | 判等（控制流） | 3 | C | reports/eco/critters/REVIEW.md §1 | RAY_DIRS 32 方向字面量未提取 |
| critters | `Boid::frame_update<fish>` | 0x141dc2740 | fix_verified（FIX⑦） | 3 | C | reports/eco/critters/REVIEW.md §1/§9 | RAY_DIRS 字面量；query 环境值写未建模 |
| critters | `Boid::frame_update<wren>` | 0x141dc3510 | fix_verified（FIX⑦） | 3 | C | reports/eco/critters/REVIEW.md §1/§9 | RAY_DIRS 字面量；viable 探针近似 |
| critters | `apply_fish_motion` | helper（非独立二进制函数） | 判等（兼容辅助） | 3 | C | reports/eco/critters/REVIEW.md §1 | — |
| critters | `smooth_critter_velocities` | 0x142318c10 | 判等 | 3 | C | reports/eco/critters/REVIEW.md §1 | powf(0,dt<0) QNaN 符号位差已登记 |
| critters | `BoidFlock::remove` | 0x141dc2340/0x141dc2540 | 重建+未实现-需重做（FIX⑥ 部分验证） | 2 | B | reports/eco/critters/REVIEW.md §2/§9 | boid 越界 break vs 二进制 panic（swap_remove assert）；正常态等价 |
| critters | `move_critters` | 0x142318fa0 | fix_verified+残留→wave4 新 FIX（已改未复验） | 2 | B | reports/eco/critters/REVIEW.md §9; reports/wave4/reviews/misc/REVIEW.md §2.1; reports/remaining/misc/RESTORE.md §6 | water2 组件 Sheep 已改；纯墙 relate 路径；spherecast 半径参数；PublicWalls panic |
| critters | `critters_avoid_collisions` | 0x1423179a0 | fix_verified（有残留）→wave4 新 FIX（已改未复验） | 2 | B | reports/eco/critters/REVIEW.md §9; reports/wave4/reviews/misc/REVIEW.md §2.2; reports/remaining/misc/RESTORE.md §6 | water8 两位置 4+4 已改；relate 形状 segment-vs-segment 近似 |
| critters | `update_firefly_goals` | 0x142316940 | 判等 | 3 | C | reports/eco/critters/REVIEW.md §1 | query 环境值写未建模 |
| critters | `set_firefly_velocities_towards_goals` | 0x142317040 | 判等 | 3 | C | reports/eco/critters/REVIEW.md §1 | query 环境值写未建模 |
| critters | `DuckSpawnLocations::new` | 0x140f8ee90 | 实现（精确网格+timer=25.0） | 2 | B | reports/integration/purelogic/RESTORE.md §A | 仅 restorer 实现；无独立 REVIEW（原 module REVIEW 判未实现） |
| critters | `spawn_ducks` | 0x140f8f210 | 重建 | 2 | B | reports/eco/critters/RESTORE.md（duck+fish 子 agent A） | sorted_by comparator U；demo/corrupted 门未接线 |
| critters | `despawn_ducks` | 0x140f8f8b0 | 重建 | 2 | B | reports/eco/critters/RESTORE.md | 离场计划=1 点+circle，现 3 点外扩 |
| critters | `FishSpawnLocations::new` | 0x140f909f0 | 判等（purelogic restorer 自判） | 2 | A | reports/integration/purelogic/RESTORE.md §A | 无独立复核；原 module REVIEW 判未实现（行列转置），post-fix 无 REVIEW；used 字节 DIVERGE |
| critters | `despawn_fish` | 0x140f90c10 | 未实现（crate 以实体清理建模） | 2 | B | reports/eco/critters/REVIEW.md §3; reports/integration/purelogic/RESTORE.md §A | 真实=资源重置（FishBoids Resource + OnExit<AppLoadingState>） |
| critters | `fish` | 0x140f90cd0 | fix_verified（FIX⑤） | 3 | C | reports/eco/critters/REVIEW.md §9 | FishBoids Component 非 Resource；光标/相机逃避未做 |
| critters | `plan_duck_trajectories` | 0x14230ff00 | 未实现（简化实现） | 2 | B | reports/eco/critters/REVIEW.md §3; reports/integration/purelogic/RESTORE.md §A | legacy_raycast/Frenet/spawn 10 组件 bundle 未重建 |
| critters | `move_ducks_along_trajectories` | 0x142312200 | 未实现（部分） | 2 | B | reports/eco/critters/REVIEW.md §3 | Frenet 帧简化；轨迹状态 {0,2} vs 现 {1,3} |
| critters | `update_duck_goals` | 0x142313b30 | fix_verified（FIX②） | 3 | C | reports/eco/critters/REVIEW.md §9 | aux 恒 0；within_glade dev 门 false；旋转精确四元数 U |
| critters | `set_duck_velocities_towards_goals` | 0x142314470 | 判等 | 3 | C | reports/eco/critters/REVIEW.md §1 | query 环境值写未建模 |
| critters | `duck_splash_particle` | 0x142322650 | 重建 | 2 | B | reports/eco/critters/RESTORE.md | 触发查询 vs 事件语义 |
| critters | `ducks_react_to_being_inside_buildings` | 0x14231b140 | 未实现-需重做（FIX⑧ 改判） | 2 | B | reports/eco/critters/REVIEW.md §2/§9 | 墙组查询结构 U；2D 射线围合近似；GladeBorder 未接线 |
| critters | `update_butterfly_states` | 0x141d84d40 | 部分判等→wave4 需修（已改未复验） | 2 | B | reports/wave2/reviews/critters/REVIEW.md §4; reports/wave4/reviews/misc/REVIEW.md §3.2; reports/remaining/misc/RESTORE.md §4 | dispatch flag 基线已改；state4-7/audio u32 写值 U |
| critters | `update_butterfly_goals` | 0x141d86060 | 判等（核心） | 3 | C | reports/eco/critters/REVIEW.md §1 | per-entity 4B 查询环境写未建模 |
| critters | `set_butterfly_velocities_towards_goals` | 0x141d867b0 | 判等（vel 表 4 handler） | 3 | C | reports/wave2/reviews/critters/REVIEW.md §4.3 | 动画跳表 handler DAT_142f68018 U |
| critters | `update_butterfly_audio` | 0x141d86eb0 | 判等 | 3 | C | reports/eco/critters/REVIEW.md §1 | — |
| critters | `sim_wren_boids` | 0x142326a80 | 重建（近似） | 2 | B | reports/eco/critters/REVIEW.md §5 | 出生门精确条件/viable 探针/FreeSpot 认领近似 |
| critters | `update_decorator_bird_spots` | 0x14231c7d0 | 未实现（Rust 空 body） | 1 | D | reports/eco/critters/REVIEW.md §3 | create_spots 布局/数量、双列表滑动窗口 |
| critters | `update_roof_bird_spots` | 0x14231d6c0 | 未实现（Rust 空 body） | 1 | D | reports/eco/critters/REVIEW.md §3 | random_splits 分段语义/停靠分布 |
| critters | `update_wrens` | 0x14231e390 | 部分判等→wave4 需修（已改未复验） | 2 | B | reports/wave2/reviews/critters/REVIEW.md §5; reports/wave4/reviews/misc/REVIEW.md §1.2; reports/remaining/misc/RESTORE.md §5 | h0 模式门/阈值已改；h1 音频槽/实体映射/PlacementDataSsbo |
| critters | `update_sheep_goals` | 0x142315050 | 重建（主流程一致） | 2 | B | reports/eco/critters/REVIEW.md §5 | XMM7/8 寄存器残留、flag_31 死分支 |
| critters | `set_sheep_velocities_towards_goals` | 0x142315e40 | 判等 | 3 | C | reports/eco/critters/REVIEW.md §1 | query 环境值写未建模 |
| critters | `update_sheep_audio` | 0x142316490 | 判等（核心） | 3 | C | reports/eco/critters/REVIEW.md §1 | 音频记录 +0x34+=dt 未建模（DIVERGE） |
| critters | `sheep_walk_on_terrain` | 0x14231bd80 | 判等 | 3 | C | reports/eco/critters/REVIEW.md §1 | case4 lerp rate 符号 U |
| critters | `sheep_unstuck` | 0x1423246b0 | 重建（DIVERGE） | 2 | B | reports/eco/critters/REVIEW.md §5 | spherecast ABI/rock 逐点采样/is_corrupted |
| critters | `react_to_being_inside_buildings` | 0x14231f610 | 未实现（部分重建） | 2 | B | reports/eco/critters/REVIEW.md §5 | emoji/UI spawn glue 之外主体不全 |
| critters | `sheep_reaction_emoji` | 0x14231ff70 | 重建 | 2 | B | reports/eco/critters/REVIEW.md §5 | EasingFunction 变体未取证 |
| critters | `sheep_floaties` | 0x142320cf0 | 重建（DIVERGE） | 2 | B | reports/eco/critters/REVIEW.md §5 | spawn bundle glue；DIVERGE |
| critters | `sheep_umbrellas` | 0x142321980 | 重建（DIVERGE） | 2 | B | reports/eco/critters/REVIEW.md §5 | 0x1a 掩码/翻向音频 U |
| critters | `sheep_walk_animation` | 0x14231a630 | 判等（主循环+2 handler） | 4 | C | reports/wave2/reviews/critters/REVIEW.md §2; reports/tests/numeric-assertions/REPORT.md | 音频槽写未建模 |
| critters | `sheep_pet_animation` | 0x14231ac20 | 判等（5 handler） | 3 | C | reports/wave2/reviews/critters/REVIEW.md §3 | 音频槽写未建模 |
| critters | `sheep_input_record_replay` | 0x14231b720 | 重建（DIVERGE） | 2 | B | reports/eco/critters/REVIEW.md §5 | 会话资源 +0x40 身份 U |
| critters | `ui_pet_sheep` | 0x142324f80 | 重建核心+未实现（UI） | 2 | B | reports/eco/critters/RESTORE.md/REVIEW.md §5 | UiWidgetContext/SignifierEntityBuilder/slider3d API 缺 |
| critters | `ui_pet_ducks` | 0x142322e30 | 重建核心+未实现（UI） | 2 | B | reports/eco/critters/RESTORE.md/REVIEW.md §5 | UI gate/eye 距离/signifier/audio 未接线 |
| water | `draw_water` | 0x141ba9130 | 判等（fix_verified） | 3 | C | reports/state/water/REVIEW.md §1 | 橡皮 signifier name 布局疑点；RasterStrokeCmd.strength 建模补充 |
| water | `draw_water::closure$0` | 0x141ba9700 | 未落库-依赖 core signifier 域 | 1 | D | reports/state/water/REVIEW.md §2 | SignifierEntityBuilder 链 + brush_circle_scribble 未落库 |
| water | `update_water_audio_grid` | 0x141baefe0 | 判等（含 3 项已登记契约差） | 3 | C | reports/state/water/REVIEW.md §4 | intersects_any max=1.0 语义差；WaterAudioGrid 语义投影 |
| water | `record_water_history` | inline（宿主 0x140e92a80） | 判等 | 3 | C | reports/state/water/REVIEW.md §5 | — |
| timberframe | `construct_timberframe_rectangle` | 0x141a41ba0 | 重建+写路径（相位/计数判等） | 2 | B | reports/state/timberframe/REVIEW.md §1; reports/wave3/reviews/tg-tr/REVIEW.md §8 | RenderInstance 字段序 I；bit0 代理；ResMut 库面；rotation_from_normal→beam_rotation I |
| timberframe | `construct_timberframe_circle` | 0x1421b67a0 | 重建+写路径（同上） | 2 | B | reports/state/timberframe/REVIEW.md §2; reports/wave3/reviews/tg-tr/REVIEW.md §9 | 主环 Quat::IDENTITY 占位 GAP；不 clear 忠实 |
| timberframe | `timberframe_beam_count_change_audio` | 0x1421b6610 | 判等（主体） | 4 | C | reports/state/timberframe/REVIEW.md §3; reports/tests/numeric-assertions/REPORT.md | get_building_size_01 size/id 缺；GameClock +0x2c 布局 GAP |
| roof | `simulate_roof_tile_for_audio` | 0x1421a7090 | 判等（每记录控制流） | 3 | C | reports/state/roof/REVIEW.md §1.5; reports/wave3/reviews/tg-tr/REVIEW.md §10 | RoofTileSimulation 单记录 vs 0x18 数组；RoofSound 载荷；GameClock.delta |
| roof | `wall_height_dt` | 0x1421a2b60 | 判等 | 3 | C | reports/state/roof/REVIEW.md §1.1 | — |
| roof | `roof_animation` | 0x1421a2da0 | 判等（经 core 委托） | 4 | C | reports/state/roof/REVIEW.md §1.2; reports/tests/numeric-assertions/REPORT.md | — |
| roof | `roof_record_history` | 0x141a37fd0 | 判等（经 core 委托） | 3 | C | reports/state/roof/REVIEW.md §1.3 | 生产端 UI 未接线 |
| roof | `replay_roof` | 0x141a37ad0 | 判等（经 core 委托） | 3 | C | reports/state/roof/REVIEW.md §1.4 | 完成位 byte[21] 映射 I |
| roof | `replay_roof::impl$0::process` | 0x1421b48e0 | 判等（经 core 委托） | 3 | C | reports/state/roof/REVIEW.md §1.4 | previous_direction I；非法 tag Err 路径 |
| roof | `switch_roof_ridge_direction` | 0x141a3a730 | 判等 | 3 | C | reports/state/roof/REVIEW.md §1.6 | RoofSound 载荷布局 core 面 |
| roof | `record_switch_roof_ridge_history` | 0x141a3ac20 | 判等 | 3 | C | reports/state/roof/REVIEW.md §1.7 | — |
| roof | `generate_roof_stone_floor_and_roof_bottom` | 0x1421a6d40 | 重建（CPU 控制流判等；命令执行体未实现） | 2 | B | reports/state/roof/REVIEW.md §1.8 | NaniBundle<StoneFloorInstanceData> insert 执行体未实现 |
| roof | `switch_roof_type` | 0x1421ac4f0 | 部分重建→mesh 5/5 判等 | 3 | C | reports/state/roof/REVIEW.md §2.1（非 mesh 判等）; reports/wave4/reviews/visual/REVIEW.md §2（mesh 5/5） | 整函数无单一 REVIEW 判等；由两次独立复核拼合 |
| roof | `calculate_roof_trigger_volumes` | 0x1421a3220 | 未实现（诚实骨架） | 1 | D | reports/state/roof/REVIEW.md §3.1 | 曲面三角化/剖面/檐口/UV/噪声未重建 |
| roof | `highlight_roof（iter 单态）` | 0x141a284b0 | 未实现→重建（数据流判等） | 2 | B | reports/wave3/reviews/tg-tr/REVIEW.md §11; reports/state/roof/REVIEW.md §3.2 | 8B 写覆盖 pad 不可表示；iter/store 读出偏移不同 |
| roof | `highlight_roof（store 单态）` | 0x141a31280 | 未实现→重建（数据流判等） | 2 | B | reports/wave3/reviews/tg-tr/REVIEW.md §11; reports/state/roof/REVIEW.md §3.2 | 同上 |
| roof | `gpu_flush_buffers` | 0x140de50a0 | 判等 | 3 | C | reports/wave4/reviews/geometry/REVIEW.md §2.2 | GraphicsDeviceResource +0x188 管线缓存槽 GAP |
| roof | `ui_edit_roof` | 0x141a18dd0 | 未实现 | 1 | D | reports/state/roof/REVIEW.md §3.4 | 三角拾取/WidgetParams/HighlightRoofCmd |
| roof | `do_ui_edit_roof` | 0x141a19a90 | 未实现 | 1 | D | reports/state/roof/REVIEW.md §3.4 | 三 case 历史/拖拽量 |
| roof | `ui_edit_roof_ridge_dir` | 0x141a386b0 | 未实现 | 1 | D | reports/state/roof/REVIEW.md §3.5 | iter_removed_ids/to_world_space/box_trigger_volume |
| roof | `do_ui_edit_roof_ridge_dir` | 0x141a39490 | 未实现（近似体已清退） | 1 | D | reports/state/roof/REVIEW.md §3.5/§7 | 方向位 ^1 / SwitchRoofRidgeDirCmd |
| roof | `ui_edit_roof_ridge_length` | 0x1421a8210 | 未实现 | 1 | D | reports/state/roof/REVIEW.md §3.5 | TweakerView/project_cursor_onto_ray |
| roof | `do_ui_edit_roof_ridge_length` | 0x1421a8c50 | 未实现（近似体已清退） | 1 | D | reports/state/roof/REVIEW.md §3.5/§7 | inverse_ridge_01_param 投影链 |
| roof | `create_ui_widget` | 0x1421aa170 | 未实现 | 1 | D | reports/state/roof/REVIEW.md §3.5 | widget 描述 0x18/分发骨架 |
| roof | `ui_edit_roof_ridge_and_height_focused` | 0x1421ad440 | 未实现（近似体已清退） | 1 | D | reports/state/roof/REVIEW.md §3.5/§7 | CameraMatrices/project_cursor_onto_ray/inverse_lerp |
| color | `IconInfo::new_separator` | 0x1425f2680 | 判等 | 3 | C | reports/state/color/REVIEW.md 总表 | — |
| color | `IconInfo::new_flat_roof_rim_ty` | 0x1425f26f0 | 判等（输入源 U 登记） | 3 | C | reports/state/color/REVIEW.md 总表 | ClutterVersion::is_initial 输入源 U |
| color | `init_color_ids_buffer` | 0x142290750 | 未实现-需重做（SSBO 段阻塞；insert 面判等） | 2 | B | reports/state/color/REVIEW.md 总表 | SSBO add_new×4 接线；coregap-2 已补类型但函数未复验 |
| color | `set_wall_color_id` | 0x142290820 | 未实现-需重做（SSBO 段阻塞；CPU 面判等） | 2 | B | reports/state/color/REVIEW.md 总表 | SSBO get/update_buffer_bytes；selector/value U |
| color | `set_gate_color_id` | 0x142299840 | 未实现-需重做（SSBO 段阻塞；CPU 面判等） | 2 | B | reports/state/color/REVIEW.md 总表 | SSBO 段；custom==1 已修 |
| color | `set_halftimber_color_id` | 0x142299250 | 未实现-需重做（SSBO 段阻塞；CPU 面判等） | 2 | B | reports/state/color/REVIEW.md 总表 | SSBO 段；custom==1 已修 |
| color | `write_roof_color_id_to_gpu` | 0x142298520 | 未实现（签名面+空循环） | 1 | D | reports/state/color/REVIEW.md 总表 | 双源逐元素 update_buffer_bytes；RoofColorIdSsbo 已补但函数未落 |
| color | `set_decorator_color_id` | 0x1422af0c0 | 未实现-需重做（记录 value U 面） | 2 | B | reports/state/color/REVIEW.md 总表 | 5 层分派/写面判等；记录 value 非 sub==4 为栈值 U |
| color | `set_flag_connection_and_chimney_smoke` | 0x142298970 | 未实现-需重做（烟值 +4 阻塞） | 2 | B | reports/state/color/REVIEW.md 总表 | flag 面判等；ChimneyExtra +4 写面 coregap-2 已补未复验 |
| color | `set_window_light` | 0x142298260 | 整体判等（REVIEW 表头仍写未实现-需重做） | 3 | C | reports/state/color/REVIEW.md 总表 | 表头 verdict 与正文矛盾；按正文“整体判等”取 C |
| color | `set_roof_customization` | 0x142299dd0 | 未实现（密集表阻塞；外环近似） | 2 | B | reports/state/color/REVIEW.md 总表; reports/unlock/publicwalls-tables/RESTORE.md | PublicWalls 稀疏位集/密集表；外环第二写遗漏 |
| color | `color_menu` | 0x142291230 | 未实现（读取面 stub） | 1 | D | reports/state/color/REVIEW.md 总表; reports/wave2/reviews/color/REVIEW.md | CustomizationIcon 0xa0/Events 类型/12 参 ABI typed-gap |
| color | `color_button_animations` | 0x1422979b0 | 未实现（空循环） | 1 | D | reports/state/color/REVIEW.md 总表 | CustomizationIcon/Ui2dExtent typed-gap |
| color | `delete_color_icon_entities` | 0x142297ff0 | 未实现（循环体留空） | 1 | D | reports/state/color/REVIEW.md 总表 | progress<=0 门控 typed-gap |
| color | `show_color_options` | 0x14229a7f0 | 重建（结构+case0/1 载荷；其余 GAP） | 2 | B | reports/wave2/reviews/color/REVIEW.md; reports/wave2/color/RESTORE.md | case0 载荷修正后无独立复验；case2..0x26/demo/事件写面 GAP |
| color | `hide_color_menu` | 0x1422adcf0 | 未实现（事件门控已修） | 2 | B | reports/state/color/REVIEW.md 总表（FIX-2） | just_pressed 动作名/图标目录 typed-gap |
| color | `autoclose_color_menu` | 0x142298d30 | 未实现（stub 非判等） | 2 | B | reports/state/color/REVIEW.md 总表（FIX-3） | menu.center 近似、省略光标 2D 跳过、kind 直闭 |
| color | `ColorRadialMenu::create` | 0x1425f27c0 | 未实现（仅数据组装） | 2 | B | reports/state/color/REVIEW.md 总表 | UI 实体 spawn typed-gap |
| color | `ui_color_wall_and_roof` | 0x141c9ab70 | 未实现（仅模式守卫） | 1 | D | reports/state/color/REVIEW.md 总表 | WidgetParams/raycast +0x1d8/closure |
| color | `color_cursor_icon` | 0x141c9b510 | 未实现（空体） | 1 | D | reports/state/color/REVIEW.md 总表 | SignifierEntityBuilder/UiInactive/Ui2d* |
| color | `ui_color_gates` | 0x1422afe70 | 未实现（仅模式守卫） | 1 | D | reports/state/color/REVIEW.md 总表 | 稀疏墙位集/mesh 三角形/signifier |
| color | `ui_color_windows_and_flags` | 0x1422b0e00 | 未实现（仅模式守卫） | 1 | D | reports/state/color/REVIEW.md 总表 | legacy_raycast/StairsState graph/outline |
| color | `replay_color` | 0x141c7f270 | 未实现（空循环+登记） | 1 | D | reports/state/color/REVIEW.md 总表; reports/unlock/replay-decorator/RESTORE.md §5.4 | ReplayBuffer 原始字节/13 写者派发 |
| decorator | `DecoratorBackup::clear` | 0x140823660 | 判等 | 3 | C | reports/state/decorator/REVIEW.md §一-1 | — |
| decorator | `clear_out_wall_holes` | 0x1407f98c0 | 判等 | 3 | C | reports/state/decorator/REVIEW.md §一-2 | — |
| decorator | `ui_decorator_inactive` | 0x140791400 | 判等 | 3 | C | reports/state/decorator/REVIEW.md §一-3 | reset_interaction 额外置 active=true（I 级模型 artifact） |
| decorator | `stairs_reset_for_new_frame` | 0x14079c350 | 判等（门控） | 3 | C | reports/state/decorator/REVIEW.md §一-4 | unset 落点 [内层+0x10]=3 以 mark_invalid 承载（I） |
| decorator | `seal_persistent_entities` | 0x140811e80 | 判等 | 3 | C | reports/state/decorator/REVIEW.md §一-5 | — |
| decorator | `add_decorator_visual_entity impl$4` | 0x1408119e0 | 判等 | 3 | C | reports/state/decorator/REVIEW.md §一-6 | cap=8 预分配差异 minor |
| decorator | `DecoratorPersistentVisualEntities::entities` | 0x1408119d0 | 判等（退化访问器） | 3 | C | reports/state/decorator/REVIEW.md §一-7 | — |
| decorator | `DecoratorPersistentVisualState::entities` | 0x14067dd80 | 判等（退化访问器/胶水级） | 3 | C | reports/state/decorator/REVIEW.md §一-7 | — |
| decorator | `clear_out_visual_entities` | 0x1407f9260 | 判等（FIX-1 后） | 3 | C | reports/state/decorator/REVIEW.md §七 | param_2 身份 U 登记 |
| decorator | `maintain_dormer_tiles` | 0x1407ce920 | 未实现→判等（G3 已修未复验） | 2 | B | reports/state/decorator/REVIEW.md §一-9; reports/wave4/reviews/geometry/REVIEW.md §2.3; reports/consume/types/RESTORE.md §5 | 游标写序已修；id 单槽 store 已修；rhapsody 注册表/RoofTileSsbo drop 未接线 |
| decorator | `add_decorator_visual_entity impl$5` | 0x140811c30 | 判等（瞬态实体表 push） | 3 | C | reports/wave4/reviews/visual/REVIEW.md §1.4; reports/consume/nani-decorator/RESTORE.md §2.4 | wave3 autoclutter 表结论被机器码否证；类型 stand-in |
| decorator | `apply_decorator_op` | 0x141276e30 | 未实现-需重做（控制流判等；叶近似） | 2 | B | reports/state/decorator/REVIEW.md §二-1 | archive_decorator_state/DecoratorLocator::new/build_op_event 0x88B 叶未还原 |
| decorator | `create_or_restore_decorator_backup` | 0x1408238a0 | 未实现-需重做（状态机判等；恢复叶近似） | 2 | B | reports/state/decorator/REVIEW.md §二-2 | 0x30B 三 map 反向拷贝未接线；桶模型近似 |
| decorator | `decorator_record_history` | 0x140817850 | 未实现-需重做（循环/清空判等；载荷映射错） | 2 | B | reports/state/decorator/REVIEW.md §二-3 | 48B {1,1,kind,f1,f2,f3} kind/f2/f3 丢弃 |
| decorator | `window_lights` | 0x1407b5980 | 未实现-需重做（常量/灯态/帧末判等；收集主循环缺失） | 2 | B | reports/state/decorator/REVIEW.md §二-4; reports/wave3/reviews/toolbar-decorator/REVIEW.md §1 | PublicWalls 密集窗桶收集主循环；rng 种子/retain 谓词/lerp 实参 U |
| decorator | `chimney_smoke_anim_thickness` | 0x140818df0 | 未实现-需重做（窗口/目标/更新公式判等；收集缺失） | 2 | B | reports/state/decorator/REVIEW.md §二-5; reports/wave3/reviews/toolbar-decorator/REVIEW.md §1 | spot 收集主循环；rng 种子/retain 谓词 U |
| decorator | `replay_decorator` | 0x14080c8d0 | 未实现（结构判等；core-gap 诚实壳） | 2 | B | reports/wave3/reviews/toolbar-decorator/REVIEW.md §4; reports/unlock/replay-decorator/RESTORE.md | ReplayCommand Decorator 变体/ReplayBucket cull 表未落库 |
| decorator | `WindowVisualEntityBuilder::build<Transient>` | 0x1412ab270 | 判等（12 类分派+14 位链+impl$5） | 3 | C | reports/consume/nani-decorator/RESTORE.md §0/§2; reports/wave4/reviews/visual/REVIEW.md §1 | 12 个条件组件以 stand-in 承载；builder 字段名 U |
| decorator | `WindowVisualEntityBuilder::build<Persistent>` | 0x1412a9b30 | 判等（同体；尾部 impl$4） | 3 | C | reports/consume/nani-decorator/RESTORE.md §0/§2; reports/wave4/reviews/visual/REVIEW.md §1 | 同上 |
| decorator | `calc_gothic_half_dormer_profile_ws` | 0x1407eb160 | 重建+部分未实现 | 2 | B | reports/wave2/decorator-forest/RESTORE.md §1; reports/wave2/reviews/geometry/REVIEW.md §3.2 | wall-space A/B/C/D lerp/法线/0..2 索引语义未实现 |
| decorator | `gothic_half_dormer_radius` | 0x1412ac9b0（call_once 内联） | 判等 | 3 | C | reports/wave2/decorator-forest/RESTORE.md §1; reports/wave2/reviews/geometry/REVIEW.md §3.1 | rank∉1..3 panic vs 默认 0.38（低影响登记） |
| decorator | `get_gothic_hat_bones` | 0x1407eb500 | 未实现 | 1 | D | reports/state/decorator/RESTORE.md; planning quickref U9 | 骨架主体待取证 |
| decorator | `~146 业务函数（blueprint/cull_oob/merge/visual 族/stairs/ui_systems）` | 族（未逐一列 VA） | 未实现（本轮未 fresh 复核） | 1 | D | reports/state/decorator/RESTORE.md 判定总表末行; planning 2026-10-17-unimplemented-ledger.md | 族估算 ~146；含 DecoratorBackup/visual 壳（chimney/door_stairs_assemble_bricks）；未逐函数审计 |
| shared-utils（跨模块） | `utils::rotation_from_normal` | 0x140c90a40 | 判等（4 case 半 sqrt；掩码 FIX-2 已修） | 4 | C | reports/gap/review/REVIEW.md #7; reports/gap/utils-toolbar/RESTORE.md §9; reports/tests/numeric-assertions/REPORT.md（utils 4 tests） | c 归一化平方和顺序 I（极端输入位级差）；overflow 掩码测试已加 |
| shared-utils（跨模块） | `Rectangle2d::orientation_mat3` | 0x140c99570 | 判等 | 4 | C | reports/gap/review/REVIEW.md #8; reports/tests/numeric-assertions/REPORT.md | — |
| shared-utils（跨模块） | `split_height_into_skirts` | 0x140b34f80 | 判等（restorer 自判） | 2 | A | reports/integration/coregap-1/RESTORE.md（附带实现） | 仅 RESTORE 声明；REVIEW 裁决表未列；无独立复核/数值断言 |

## 4. 不计入业务计数的 glue

| 模块 | 符号 | VA |
|---|---|---|
| glade-border | `GladeBorderPlugin::build` | 0x140650ab0 |
| forest | `ForestPlugin::build` | 0x140650c50 |
| tree-tool | `TreePlugin::build` | 0x140650520 |
| critters | `fireflies_send_to_gpu` | 0x142316760 |
| critters | `butterflies_send_to_gpu` | 0x141d87140 |
| critters | `update_duck_ripples` | 0x142314b70 |
| critters | `CritterPlugin::build` | 0x14067d440 |
| critters | `sheep::Plugin::build` | 0x14067d810 |
| water | `draw_water::closure$1/2/3` | inline |
| water | `WaterPlugin::build` | 0x1406501d0 |
| timberframe | `add_timberframe` | 0x140630a20 |
| timberframe | `timberframe_on_glade_unload` | inline（body U） |
| roof | `register_required_components impl$0` | 0x14258d1f0 |
| roof | `register_required_components impl$1` | 0x14258d8e0 |
| color | `add_color` | 0x140651870 |
| color | `register_required_components` | 0x1425f3d60 |
| decorator | `Plugin::build / snapping_direction / try_merge_decorators` | inline |

## 5. 优先补复核建议（top 10）

1. `move_critters` 0x142318fa0（B，wave4 water2=Sheep FIX 未复验；共享移动主路径）
2. `critters_avoid_collisions` 0x1423179a0（B，water8 两位置 4+4 FIX 未复验）
3. `update_wrens` 0x14231e390（B，h0 模式门/阈值 FIX 未复验）
4. `update_butterfly_states` 0x141d84d40（B，dispatch flag 基线 FIX 未复验）
5. `replay_tree` 0x141bc34f0（B，Move undo 旧位置整条事件缺失，历史回放高风险）
6. `process_tree_cmds` 0x142267430（B，Insert T2/T4/T5；树放置主链）
7. `maintain_dormer_tiles` 0x1407ce920（B，G3 单槽 store 已改未复验；GPU 退役）
8. `show_color_options` 0x14229a7f0（B，case 0 图标载荷已修未复验；用户可见 UI）
9. `window_lights` 0x1407b5980（B，收集主循环缺失；PublicWalls 表合同已出但未落库）
10. `chimney_smoke_anim_thickness` 0x140818df0（B，收集主循环缺失；公式已双证）

> A 面优先：`FishSpawnLocations::new` 0x140f909f0 与 `split_height_into_skirts` 0x140b34f80 需补一次独立复核（成本最低）。

## 6. 方法与残留限制

- 逐函数检索全部 RESTORE/REVIEW 的 verdict，取最高正向证据；对同一函数的多轮覆盖（模块 REVIEW → wave2/wave3/wave4 → consume/remaining → integration）按时间与证据强度取后者。
- `fix_verified` 只在 reviewer 对修复后的代码再次给出正向结论时计 C；restorer 单方声明“已修”不计 C（归 B）。
- decorator `~146` 为台账估算，未逐函数列名/VA；其 D 计数为估算值，不影响显式 23 个函数的逐条判定。
- 所有 shared-utils 与 glue 均单列，避免污染 9 模块业务函数的 A/B/C/D 占比。
- 本审计只读汇总，未修改任何 src，未调用反编译工具。

---

## 7. 复核回执 — reviewer-B-eco（2026-10-18）

> 只读独立复核；fresh Ghidra 反编译/反汇编/内存实读（`--worker shared`）；未改 src。
> 范围：B 清单中 **forest / tree-tool / critters / glade-border** 的 33 项
> （forest 0；glade-border 2；tree-tool 8；critters 23）。
> 报告与证据：`reports/reviews/B-eco/REVIEW.md`、`reports/reviews/B-eco/evidence/`。

### 7.1 三态结论：**判等 7 / 需修 22 / 未实现 4**

| 模块 | B 项 | 判等→C | 需修（仍 B） | 未实现（仍 B） |
|---|---:|---:|---:|---:|
| glade-border | 2 | 2 | 0 | 0 |
| forest | 0 | 0 | 0 | 0 |
| tree-tool | 8 | 1 | 4 | 3 |
| critters | 23 | 4 | 18 | 1 |
| **合计** | **33** | **7** | **22** | **4** |

- **升 C（7）**：`display_glade_border`、`display_glade_border::closure$1`、`ui_place_tree`、
  `despawn_fish`、`duck_splash_particle`、`sheep_floaties`、`sheep_umbrellas`。
- **需修（22）**：`process_tree_cmds`、`replay_tree`、`place_tree`、`ui_move_tree`、
  `ui_delete_tree`、`BoidFlock::remove`、`move_critters`、`critters_avoid_collisions`、
  `DuckSpawnLocations::new`、`spawn_ducks`、`despawn_ducks`、`plan_duck_trajectories`、
  `move_ducks_along_trajectories`、`ducks_react_to_being_inside_buildings`、
  `update_butterfly_states`、`sim_wren_boids`、`update_wrens`、`update_sheep_goals`、
  `sheep_unstuck`、`react_to_being_inside_buildings`、`sheep_reaction_emoji`、
  `sheep_input_record_replay`。
- **未实现（4）**：`tree_move_bend`、`hide_trees`、`ui_pet_sheep`、`ui_pet_ducks`。

### 7.2 账目影响

- 本四模块 B 面 33 → **C +7、B 仍 26**；9 模块总口径 B **57 → 50**、C **59 → 66**
  （其余模块本轮未动）。
- 时间线：`reviewer-B-ecstate` 的 8 项复验报告（23:32）之后，restorer 于 23:33–23:36
  又改了 `critters/shared.rs`、`butterfly.rs`、`tree-tool/replay_tree.rs`、
  `process_tree_cmds.rs`；本回执即这些修改的 post-fix 复验。旧 FIX 中已闭合：
  move_critters 纯墙路/spherecast 半径/PublicWalls panic、critters_avoid 半径/零方向/panic、
  butterfly state2 次态 3→1、replay_tree Move-undo 旧位。
- **新发现（本轮）**：
  1. `place_tree` 旋转下限常量错：原生 `r2*360+(1-r2)*0.0`（反汇编 lane1×0.0），
     Rust `0.492_200_02`。
  2. `DuckSpawnLocations::new` 列常量 j=2/3 与公式差 2 ULP（0xc1e71c73/0xc18aaaa9）。
  3. `update_butterfly_states` handler 的 t 源应为 `Butterfly+4 (state_t)`（派发浮点为死值），
     当前 Rust 仍把本地 `t_in` 传入。
  4. `sheep_unstuck` 跳过条件应为 `hitcode != 2`，当前 `!sphere.hit`。
  5. `spawn_ducks` 缺 `is_corrupted` 门，且未复刻 `sorted_by` + 从尾部遍历。
  6. `sheep_reaction_emoji` 缺 `WindowSize::as_vec2 + world_space_to_screen_space` 写面。
  7. `despawn_ducks` 原生**不直接 despawn**：只 push `key=鸭实体 id` 的 1 点离场计划；
     Rust 直接 despawn 且 `key=0`（会被 `plan_duck_trajectories` 当作出生计划）。
- **FIX 清单**：FIX-B1..B24（详见 `reports/reviews/B-eco/REVIEW.md` §5）。
- 其余共享类型缺口（InsertTreeCmd 0x48、MoveTreeCmd 0x20、PrefabInstanceData step、
  CritterGoal 布局、legacy_spherecast ABI、TreeAnimation α@+4）维持既有登记，
  由集成 owner 裁决后二次复核。
