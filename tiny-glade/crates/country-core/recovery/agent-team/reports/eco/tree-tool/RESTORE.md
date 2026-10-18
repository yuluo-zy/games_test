# restorer-eco-treetool — system-tree RESTORE 报告

日期：2026-10-17（含 reviewer-eco-treetool FIX 响应轮）
状态：in_progress（FIX 已处理，待 reviewer 复核）
涉及文件：`crates/systems/tree-tool/src/*.rs`（package `system-tree`）
证据包：`reverse-engineering/tiny-glade/upper-modules-plan/systems/tree-tool/.work/eco-restorer-fresh/`
（`dec/*.c` fresh 反编译 + `result*.json` 反汇编/内存直读，`--worker shared`）
复核证据：`.work/eco-reviewer-fresh/`（reviewer 独立 fresh）

## 〇、总述

本轮对 **13 个编译实例 + reset_tree_tool（内联）** 全部 fresh 反编译/反汇编，
重建真实控制流。reviewer-eco-treetool 复核后修正 verdict：凡存在未还原控制流
分支/未实现写面的函数一律归 **未实现-需重做**（不因"部分子逻辑已重建"通过）。

**最终 verdict 统计：判等 4 / 未实现-需重做 9 / 胶水 1。**

`cargo check -p system-tree`：本批 FIX 改动后**已通过**（tree-tool 0 警告；
country-core 13 个既有警告非本组范围）。**注意**：复核期间并发 audio 组对
`country-core/src/resources/audio_resources.rs` 的 in-flight 改动（02:03/02:05，
Studio::Error/EventInstance::Error 缺 Debug 的 4 个 E0277 + meta_commander 级联）
暂时阻断整个 country-core 编译——**非 tree-tool 改动所致**，本组未改该文件，
待 audio 组闭合后可复验。阻塞项：country-core ivy_pruner 曾因 `point_inside_wall_
terrain_samples` private 编译失败，做了 1 字可见性修复 `pub(crate)`
（ivy_spawner.rs:203，与 ivy 组协作登记）。

## 一、函数 → Rust 位置 → verdict → 证据

| 原函数 | 地址 | Rust 位置 | verdict | fresh 证据 |
|---|---|---|---|---|
| process_tree_cmds | 0x142267430 | process_tree_cmds.rs:40 | **未实现-需重做**(分歧 1/2/18) | dec/0x142267430_process_tree_cmds.c |
| despawn_trees | 0x1422686d0 | process_tree_cmds.rs:88 | **判等** | dec/0x1422686d0_despawn_trees.c |
| convert_preplaced_trees | 0x14226be70 | convert_preplaced_trees.rs:50 | **未实现-需重做**(分歧 12-15) | dec/0x14226be70_convert_preplaced_trees.c |
| replay_tree | 0x141bc34f0 | replay_tree.rs:20 | **未实现-需重做**(分歧 1/2/8) | dec/0x141bc34f0_replay_tree.c |
| recalc_tree_position_on_terrain_height_change | 0x142268dc0 | recalc_tree_position_on_terrain_height_change.rs:18 | **判等** | dec/0x142268dc0_...c |
| tree_move_bend | 0x1422689c0 | tree_move_bend.rs:34 | **未实现-需重做**(分歧 7；bend 数学判等) | dec/0x1422689c0_tree_move_bend.c + result7.json |
| hide_trees | 0x14226a260 | hide_trees.rs:42 | **未实现-需重做**(分歧 9-11) | dec/0x14226a260_hide_trees.c |
| random_tree_prefab | 0x14226a170 | ui_place_tree.rs:77 | **判等** | dec/0x14226a170_...c + result9.json + result10.json |
| ui_place_tree | 0x142269170 | ui_place_tree.rs:128 | **未实现-需重做**(分歧 3；主体判等) | dec/0x142269170_ui_place_tree.c |
| place_tree | 0x142269970 | ui_place_tree.rs:178 | **未实现-需重做**(分歧 2/3/4；splitmix/scale/History 判等) | dec/0x142269970_place_tree.c |
| ui_move_tree | 0x14226b1a0 | ui_move_tree.rs:41 | **未实现-需重做**(分歧 1/9/15/16) | dec/0x14226b1a0_ui_move_tree.c |
| ui_delete_tree | 0x141bd2510 | ui_delete_tree.rs:28 | **未实现-需重做**(分歧 9/15/17) | dec/0x141bd2510_ui_delete_tree.c |
| reset_tree_tool | 内联 | startup_tree_tool.rs:9 | **判等** | PDB inlinee + ANALYSIS |
| TreePlugin::build | 0x140650520 | lib.rs:83 | **胶水-非业务** | dec/0x140650520_build.c |

## 二、判等说明（业务语句逐条对应）

1. **despawn_trees**：α **严格小于** 0.01（`<=0.01 && !=0.01`，0.01/NaN 均排除）；
   `DespawnPrefabCmd{entity:Some(entity)}` payload@+8 正确。真实 TreeAnimation
   浮点在组件 +4 vs 共享型 α@+0——**country-core 布局分歧 5**（非本 body 错误，
   修复前编译产物读 +0，字节级不等；判等限 body 逻辑）。
2. **recalc_tree_position_on_terrain_height_change**：EventReader 空早退；
   `ABS(x)<=65 && ABS(z)<=65`（含边界，NaN 为假）；`y!=h`（f32 不等含 NaN）才写
   `y=(h<=0?0:h)`。
3. **random_tree_prefab**：门控 + 7 串表逐分支吻合（reviewer 升判等）：
   `corrupted&&bit==0→naked1_no_mushrooms`；`(deny||hal)&&bit==0→naked1`；
   else `started→autumn/branchy[bit]`、否则 `branchy[bit]`。可达性修正：
   `started&&bit==1→autumn/branchy1` 可达；仅 naked2 不可达（编译器保留）。
   分歧 4（deny/corrupted/drag 会话门控）在调用侧接线。
4. **reset_tree_tool**：`*storage = TreeStorage::default()`（hashbrown 头/随机
   KEYS/next_index=0 逐字段一致；KEYS 属 std 内部胶水）。

## 三、FIX 响应（reviewer-eco-treetool FIX 清单）

函数级已修（本批）：
- **convert_preplaced_trees**：事件方向**反转**——未转换（水/岩/花园/路径/墙任一
  命中）→ 发 DeleteTreeCmd；通过门禁 → History（未实现）。补 Wait 延迟帧门控
  （事件到达 count=1，无事件帧 count+1，>2 执行）。**History::insert_starting_edit
  仍未实现**（共享 History=HistoryTransactions 与 ObservedHistoryLedger 不兼容，
  无法调用 → 分歧 15）。
- **ui_delete_tree**：中心/半径改 PathDrawEvent（存在时 center=事件位置、
  radius=事件 radius×2；flag@+0x18 缺失以事件存在近似，分歧 17）。
- **lib.rs build**：移除 4 个非本 build 事件 init（PathDrawEvent/SpawnPrefabCmd/
  DespawnPrefabCmd/OnTerrainChange）——原 build 只注册 7 事件，余属其它插件域。
- **random_tree_prefab 注释**：更正"autumn/branchy1 不可达"（可达）。

函数级待类型接线（以下分支真实载荷/状态机依赖 country-core，已登记集成任务）：
- process_tree_cmds Move/Insert 分支（分歧 1/2）；Delete 分支 Entities::contains
  门控（分歧 18，以 Commands::entity 兜底）。
- replay_tree elapsed==0 门控 + Move/Insert 重建 + 时序记账（分歧 1/2/8）。
- tree_move_bend PrefabInstanceData step 写面（分歧 7）。
- hide_trees 形状分派/墙哈希/可见性写/门控字节（分歧 9-11）。
- place_tree InsertTreeCmd 全字段 + 会话门控接线（分歧 2/4）。
- ui_place_tree raycast+geo_src 早退分支（分歧 3，依赖 country-core
  geo_src_to_edit_subject）。
- ui_move_tree slider3d case1/2/3 状态机 + MoveTreeCmd 全载荷 + case2 界外双事件
  （分歧 1/15/16）。

## 四、country-core 集成任务（T 项，本组登记不改库）

| 集成任务 | 现状 | 阻塞 |
|---|---|---|
| MoveTreeCmd 扩为 {id, index u64@+8, position Vec3@+0x10}（槽 0x20） | 共享仅 `_gap` | process_tree_cmds Move / replay_tree Move / ui_move_tree 载荷 |
| InsertTreeCmd 扩为 0x48（name String@+0、id@+0x38、position/scale/rotation/variant/flags/height 中段） | 共享 {id,position,scale,prefab_name}（0x28） | process_tree_cmds Insert / place_tree / replay_tree 重建 |
| PrefabInstanceData 补 +0x24/+0x28 step 字段 | 布局只到 0x20 | tree_move_bend 变更写面（GPU 同步） |
| ReplayCommand/ReplayBucket 补 elapsed@+0x18/delay@+0xc/flags@+8/speed 时序 | 无 | replay_tree 时间推进/待移除集合 |
| TreeAnimation f32@+4 vs α@+0（分歧 5） | 共享型偏移不符 | despawn 字节级不等（判等限 body） |
| History 支持 insert_starting_edit（ObservedHistoryLedger vs HistoryTransactions） | 不兼容 | convert 转换链 |

## 五、关键纠偏（reviewer 复核通过 + 本批 FIX）

- **PlaceTreeCmd** = {position Vec3}（槽 0x18），非 {id,packed,extra}。
- **tree_move_bend** lerp rate = **5.0**（0x142925988，兼作位移上限）；3.22
  （0x14292598c）属其它调用点。
- **place_tree** scale=lerp(0.5,0.9)、rotation=r*360、变体=(u32)v*10 高 32 位
  （splitmix/拒绝采样/History Place 判等）。
- **recalc** 边界 `<=65`（含）；**despawn** 门控 **严格 `<0.01`**。
- **random_tree_prefab** 7 串表 0x1430351d0 区直读；可达性修正见 §二.3。
- **convert** 事件方向 = 未转换树发事件 / 通过者进 History（首批方向相反已纠）。

## 六、协作登记

- 修改 `country-core/src/systems/ivy/ivy_spawner.rs:203` `fn → pub(crate) fn`
  （解 ivy_pruner 编译阻塞；非 tree 域改动，1 字可见性）。
- 全部 fresh 反编译走 `--worker shared` 只读；无 ReVa 脚本/写库。
