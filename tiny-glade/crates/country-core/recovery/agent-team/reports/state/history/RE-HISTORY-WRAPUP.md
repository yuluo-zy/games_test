# RE-HISTORY-WRAPUP — 卡C history 收尾证据包（9 edit 类型 + replay 族）

分析师: reanalyst-state
日期: 2026-10-15
目标: 卡2 完成后 restorer-state 接 history 收尾（README order 5）。给出 edit 类型现状、
缺 edit 的结构证据、replay 族接线、与 save/session 衔接。**已挖透的指认章节，不重挖**。

---

## 1. EditType 9 变体现状（inventory ↔ src ↔ 章节）

### 1.1 原版 EditType tag 空间（O 证据）

- `EditType Debug::fmt 0x140a842c0` 跳表 @0x142ad88d0 **≥40 项**（RNE 原字节 0x142ad88d0 起，
  每项 i32 偏移；第 0-14 项全异、15-24 部分复用、25-39 高度复用）——**原版 EditType 变体
  远多于 src 的 10 个**。src 注释「tag 空间 0..13」过窄。
- 各变体构造函数（章节已 O）：Water 0x1416bb7f0、Garden 0x1416960d0、Path/Cobblestone1/2/3
  （行 496-499）、`EditType::new_height_changed`（0x140a83fb0，写 niche 0x8000000000000002）。
- **注意**：new_height_changed 判别 0x8000000000000002 ≠ src 的 Cobblestone1=tag 2——墙子编辑
  的变体编号与顶层 EditType 编号**不共用**（Wall 是 EditType 一个变体，内部 WallHistoryEdit
  有 ~40 操作变体；niche 0x8000000000000002 是 WallHistoryEdit 变体 2 或 EditType 的墙相关
  判别，需按 Debug fmt 跳表逐项定 tag，登记为残留 §5）。

### 1.2 inventory 文件 ↔ src ↔ 章节 状态表

| inventory 文件 | 原函数 | src 现状 | 章节 | 结论 |
|---|---|---|---|---|
| resources/history/mod.rs | 107 | **EXISTS**（71 行，EditType 10 变体 + HistoryIdentity + History<EditType> 别名） | mod.rs.md O | **部分实现**：EditType 变体缺 ~30（§1.1）；HistoryTransactions 委托 recovered_building |
| resources/history/auto_clutter_edit.rs | 9 | **EXISTS**（27 行） | 章 O | ✓（AutoClutterEdit{host wall_id, decorator_ids}） |
| resources/history/replay_cmd_buffer.rs | — | **EXISTS**（156 行） | 章 O | ✓（ReplayBuffer/ReplayBucket/ReplayCommand） |
| resources/history/color_edit.rs | 48 | **MISSING**（ColorEdit 全 src 无定义） | 章 O | **缺**：ColorEdit{目标 Wall/Roof/Halftimber/Decorator/StairRailing/FlatRoof/StairLadder/Brightness + from/to + 目标 id, chimney_h_offset, allow_connection}；serde 0x14003c5d0/0x1417befe0/0x1417bf8a0 |
| resources/history/decorator_edit.rs | 45 | **MISSING**（DecoratorHistoryEdit 无定义） | 章 O | **缺**：DecoratorHistoryEdit/Diff/State、DecoratorHistoryState::From（preview panic）、is_default_*（==1.0f ICF）、convert_before_after_states_to_changes 0x140b3fab0、send_state_diff_events 0x1417bdae0 |
| resources/history/path_edit.rs | 13 | **MISSING**（RasterEdit 无定义） | 章 O | **缺**：RasterEdit{points, brush_size, is_additive, blend_mode} + RasterEditSerialized；serde 0x1404896f0/0x1416e8340/0x1416e8510；From 0x140a976b0 |
| resources/history/roof_edit.rs | 22 | **部分**（RoofEdit 在 resources/roofs/commands.rs / recovered_building/building_history_transactions.rs） | 章 O | **半**：RoofEdit 操作枚举（FlatRoofRimStyleChange/SwitchRoofRidgeDir…）；serde 0x140259e30/0x140b38b80/visit_str 0x140b392d0 族 |
| resources/history/stairs_history_edit.rs | 12 | **MISSING**（StairsHistoryEdit 无定义） | 章 O | **缺**：StairsHistoryEdit/Diff{边对 before/after + hash}、运行时 StairsDiffTable 96B=两 Vec<u32>；serde 0x140259af0/0x140a18ee0 |
| resources/history/terrain_edit.rs | 24 | **部分**（TerrainEdit 在 resources/terrain_editing.rs） | 章 O | **半**：TerrainEdit（判别 0/1/2 New/Edit/Move stroke；duration clamp(Σ点距·0.01,0.4,3.0)）；serde 0x1404f41b0/0x1417c1590/0x1417c1ac0/0x1417c23e0 |
| resources/history/tree_edit.rs | 25 | **MISSING**（TreeEdit 无定义） | 章 O | **缺**：TreeEdit{Place/Move + tree{id,position,scale,rotation_degrees,prefab_name,is_preplaced}}、missing_prefab 0x1409e6210、TreeCanopyMaxCullDistance；serde 0x14003d4e0/0x1417c2770 族 |
| resources/history/wall_compound_edit.rs | 79 | **部分**（WallCompoundHistoryEdit 在 recovered_building/wall_history_edit.rs） | 章 O | **半**：WallHistoryEdit 操作枚举（~40 变体，ChangeFreehandHeightType/PlasterPeelAmount/MorphPremadeShape…）、CompoundEditBuilder（new_edit_sequence/next/build 0x140a82d50）、WallHistoryCompoundEdit 0x108B；serde 0x141401960 族 |
| resources/history/wall_history.rs | 3 | **MISSING**（ev_new_wall/ev_height_changed 在 recovered_building/wall_history_edit.rs 域） | 章 O | **半**：ev_new_wall 0x140a83300（History::add tag 7）、ev_height_changed 0x140a83f80→new_height_changed 0x140a83fb0 |

**缺文件（restorer 需新建）**：color_edit.rs(48)、decorator_edit.rs(45)、path_edit.rs(13)、
stairs_history_edit.rs(12)、tree_edit.rs(25)；**半实现**：roof_edit/terrain_edit/wall_compound/
wall_history（类型已迁至 recovered_building/resources，需按 inventory 原位对齐）。

---

## 2. 各缺 edit 的结构/schema/replay（restorer 可直接落实现）

| edit | 结构 | serde | replay 命令 |
|---|---|---|---|
| **ColorEdit** | 目标枚举（Wall/Roof/Halftimber/Decorator/StairRailing/FlatRoof/StairLadder/Brightness）→ 颜色目标 + from/to + 目标 id；附加 chimney_h_offset/allow_connection | 手写 visitor：serialize 0x14003c5d0 / deserialize 0x1417befe0 / visit_str 0x1417bf8a0（字段名在串证据） | color 编辑走 History::add 普通编辑（无独立 replay 命令；replay 时经 edit 数据恢复） |
| **DecoratorHistoryEdit/Diff** | 装饰前后状态 Diff；DecoratorHistoryState::From（preview → panic "Attempting to capture a DecoratorHistoryState of a preview"） | is_default_lantern_brightness==is_default_stair_width==`x==1.0f`（0x140b3f910 ICF）；convert_before_after_states_to_changes 0x140b3fab0（按 style 字节分变体比较浮点输出变更集） | **replay_pop_bucket 消费**：send_state_diff_events 0x1417bdae0（装饰 diff 应用）；restore_decorators FUN_14155ba30 |
| **RasterEdit**（path_edit） | {points, brush_size, is_additive, blend_mode}；ReplayCmd 判别 0/1/2 的 Arc 负载 | serialize 0x1404896f0/0x1416e8340、deserialize 0x1416e8510、From 0x140a976b0 | **replay_path**：replay_raster 4 实例（DirtPath/Cobblestone1-3）+ generate_path_replay_cmds（0x1416286d0，niche 0..3 过滤后尾调共享核心 0x1414423e0） |
| **StairsHistoryEdit/Diff** | 边对 before/after + hash；运行时 StairsDiffTable 96B = 两 Vec<u32> 边对（cap-first） | serialize 0x140259af0 / deserialize 0x140a18ee0 | **replay_stairs**：undo==0 插边集（StairsRemovedSupports），!=0 remove_entry；查附挂装饰 + PlayStairsAudioCmd |
| **TreeEdit** | {Place/Move} + tree{id, position, scale, rotation_degrees, prefab_name, is_preplaced}；missing_prefab 0x1409e6210；TreeCanopyMaxCullDistance | serialize 0x14003d4e0/0x1417c2770/0x1417c2600/0x1417c2a50、deserialize 0x1417c2f60 | 树编辑普通编辑（无独立 replay 文件；place/move 数据恢复） |

**History::add 载荷（O，mod.rs.md 行 128-136）**：`History::add` 0x140a84ec0（3 参，第 3 参=
track-caller 文件）：pending<0 panic "Tried writing…"；桶空且 cursor-len<-2 先 nuke；tag 7 墙族
校验（逐条目 niche 解码，变体 0 断言 f32 高度、变体 1/2 断言 +0x18>=0）；push 24B 记录
{tag, records, count} 进 pending.edits（元素 0x18）。add_decorator_diff 0x140a854a0：4 参、
楼梯 checksum 校验（跨域 0x14362c011 开关）、两 u32 边对循环 push tag 0xE/1、Arc 替换两表。

---

## 3. replay 族（systems/history 接线图，全部章节 O）

```
history_process 0x140af5b30（42KB 主系统）：
  消费 HistoryCmd 事件流（undo/redo/interrupt）→ 移动 History 游标 → replay_edit 展开
  → generate_replay_cmds_from_edit（按 HistoryEdit.tag 分派 0x142c5f248 共享核心，10.4KB）
  → ReplayCmd 桶 push ReplayBuffer（undo 反转）
  + 第二事件流（16B 记录 byte[+9] 跳表 0x142ae5420）做命令校验/动画即时化/移除
  + 录制中仅记日志
finish_replay：ReplayBuffer+待处理双空 → RestoreAppTo 写回 AppMode → MetaCommander 哨兵弹栈
  → 恢复工具栏 → CmdUnfreezeSheep（"History replay has naturally finished. Unblocking game
  commander." 0x142a9f208）
replay_mode：AppMode 进出 0xd(Replay) 检测 + HoldToFastReplay 长按快进 → redo HistoryCmd
各 replay_*（O，均 前桶判别 tag + undo/redo 两路径）：
  replay_wall（4 命令，WallReplayEditCmd 0x18B process 0x140abfb30 自递归 8 参）
  replay_terrain（3 命令，TerrainReplayCmd duration clamp(Σ点距·0.01,0.4,3.0)，undo reapply/revert）
  replay_stairs（10 命令，边集插/删 + PlayStairsAudioCmd）
  replay_auto_clutter（9 命令，墙集 HashMap<WallId,()> 插/删）
  replay_raster<T,R,F>（6 实例 Path/C1/C2/C3/Garden/Water：undo 清缓冲重绘存储 stroke / redo 画 stroke）
  replay_garden / replay_water / replay_path = replay_raster 薄包装 + 命令生成闭包
  replay_pop_bucket：空桶 → pop_front + 装饰/楼梯 diff 应用（restore_decorators FUN_14155ba30 +
    restore_backup/apply_diff 链 1408ff210→140900ba0→14157cec0→140b50cc0→140b50d60→1408fefe0）
  signifier_replay_active：ReplayModeElapsedTime 驱动 Transform 弹性抖动
```

**src systems/history 现状**：EXISTS：recording.rs(81L,=history_recording)、replay_auto_clutter.rs(57L)、
replay_raster.rs(433L)、replay_roof.rs(50L)、replay_wall.rs(340L)、tests.rs。**缺**：history_process.rs、
finish_replay.rs、replay_mode.rs、replay_garden.rs、replay_terrain.rs、replay_water.rs、replay_path.rs、
replay_stairs.rs、replay_pop_bucket.rs、signifier_replay_active.rs、history_recording（src 名 recording.rs）、
mod.rs。

---

## 4. 与 save/session 衔接（edit 记录 → GameSaveData history 节，O）

```
History::add / add_decorator_diff → History.edit_chain（MultiEdit 记录 24B {tag, records, count}）
  ├─ ensure_current_snapshot 0x140a439b0（=add_history_edit_to_save_file）：
  │    MultiEdit::serialize（FUN_140b39b80，0x80B 缓冲）→ 追加历史 JSON + mark_dirty(start,end)
  └─ save_game 0x140a3f470 读取 ResMut<History>：
       Identity 节 = HistoryIdentity（u64，to_json_string 0x140b385e0）
       chain_index 节 = HistoryChainIndex（base-100 itoa，0x1408aa7a0）
       history_start 节 = u64
       snapshot 节 = HistoryChainSnapshots（0x140b38810，快照 Arc 列表）
       → 手工拼 JSON（save_game_to_file<Cursor/File>，"History":{...}，edit 间 ",\n" 0xa2c）
load_game 0x14146d030：parse_saved_game_data → MultiEdit deserialize（serde 巨函数 0x140b36460
  = <MultiEdit as Deserialize>，跳表 0x142aee31c 分派 EditType/HistoryEdit tag 0..=8+）
  → extract_max_wall_id_from_history（遍历 MultiEdit 变体 7/8/9 取最大 wall id）
  → init_game_state_from_snapshot
```

**衔接结论（restorer 落地）**：缺的 edit 类型（ColorEdit/DecoratorHistoryEdit/RasterEdit/
StairsHistoryEdit/TreeEdit）实现后，其 serde 必须与 `MultiEdit deserialize 0x140b36460` 的
tag 跳表对齐（0..=8+）才能被 load_game 正确反序列化；`History::add` 载荷 24B {tag, records,
count} 与 save_game_to_file 的 MultiEdit::serialize 共用同一 EditType 判别空间。

---

## 5. 未决疑问

1. EditType Debug::fmt 跳表 0x142ad88d0 的 40 项逐变体 tag/名映射（与 WallHistoryEdit 跳表
   0x142ad888c 的关系需厘清；src 10 变体 vs 原 ~40 变体）。
2. ColorEdit/DecoratorHistoryEdit/TreeEdit 的字段名串（serde visit_str 族串地址）——章节已
   指认函数地址，字段名表可对 serde visitor 逐串提取。
3. 缺 replay 文件（history_process/finish_replay/replay_mode/replay_{garden,terrain,water,
   path,stairs,pop_bucket}/signifier_replay_active）的 src 重建——章节全 O，可直接落实现。
