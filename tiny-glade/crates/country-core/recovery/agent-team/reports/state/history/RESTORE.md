# RESTORE — resources/history + systems/history（卡C，组 state）

> restorer-state，2026-10-16。按 RE-HISTORY-WRAPUP.md 落地缺 edit 文件 + EditType 扩变体
> + replay 族可接部分；save/load serde tag 对齐残留登记。

## 本批新增实现

| 项 | Rust 位置 | 证据 |
|---|---|---|
| `ColorEdit`/`ColorTarget` | 新 `resources/history/color_edit.rs` | 章节 O（serialize 0x14003c5d0 / deserialize 0x1417befe0 / visit_str 0x1417bf8a0；目标枚举 8 类 + from/to + 目标 id + chimney_h_offset/allow_connection） |
| `RasterEdit`/`RasterEditSerialized` | 新 `resources/history/path_edit.rs` | 章节 O（points/brush_size/is_additive/blend_mode；serde 0x1404896f0/0x1416e8340/0x1416e8510） |
| `StairsHistoryEdit`/`StairsDiff`（`StairsEdge = (u32,u32)`） | 新 `resources/history/stairs_history_edit.rs` | 章节 O（边对 before/after + hash；StairsDiffTable 96B；serde 0x140259af0/0x140a18ee0） |
| `TreeEdit`/`TreeEditKind`/`TreeEditData` | 新 `resources/history/tree_edit.rs` | 章节 O（Place/Move + tree 六字段；serde 0x14003d4e0/0x1417c2770 族） |
| `DecoratorHistoryEdit`/`DecoratorHistoryState`/`DecoratorHistoryDiff` | 新 `resources/history/decorator_edit.rs` | 章节 O（前后状态 Diff；preview panic；is_default_* == 1.0f；convert_before_after 0x140b3fab0） |
| **EditType 扩变体** | `resources/history/mod.rs`：+Color/Decorator/Stairs/Tree（14 变体） | RE-HISTORY-WRAPUP §1（原版 Debug fmt 跳表 0x142ad88d0 ≥40 项；新增变体精确 tag 待取证） |
| `finish_replay` | 新 `systems/history/finish_replay.rs`（双空 → log "…naturally finished…"；RestoreAppTo/工具栏/CmdUnfreezeSheep 宿主接线） | RE-HISTORY-WRAPUP §3（@0x142a9f208） |
| `replay_mode` | 新 `systems/history/replay_mode.rs`（is_replay_mode tag 13 / transition / HoldToFastReplay 阈值） | RE-HISTORY-WRAPUP §3（0xd 进出检测） |
| FIX-1（reviewer 卡2）：commander_update AppMode 门控 {13,14} | `meta_commander.rs::commander_update_system` | REVIEW5（机器码 `mode-6<0xf && mode-0xd<2`） |

## save/load 对齐（关键，部分落地）

- `History::add` 载荷 24B {tag, records, count} 与 `MultiEdit::serialize`
  （FUN_140b39b80）共用 EditType 判别空间；新增 4 变体 native_tag 用保留号
  （15/16/17/18），**精确 tag 需与 MultiEdit deserialize 跳表 0x142aee31c 对齐**——
  re-analyst 按 Debug fmt 跳表 0x142ad88d0 逐项定 tag 后修正（残留 §5.1）。
- `chain_index`/`history_start`/`snapshot` 四节载荷：HistoryChainIndex（base-100 itoa
  0x1408aa7a0）、history_start=u64、HistoryChainSnapshots（0x140b38810）——后者 schema
  待取证，登记。

## replay 族现状

- 已有：`process_history_cursor_commands`（history_process 核心）、`pop_empty_replay_bucket`、
  replay_wall/roof/auto_clutter/raster（RasterDomain 六实例含 Path/C1/C2/C3/Water/Garden）、
  recording。
- 新增：finish_replay、replay_mode。
- **仍缺/阻塞（宿主资源）**：replay_terrain/replay_stairs/replay_pop_bucket（装饰+楼梯
  diff 应用链 restore_decorators FUN_14155ba30→1408ff210→…）/signifier_replay_active
  （Transform 弹性抖动）/history_process 第二事件流（16B 记录 byte[+9] 跳表 0x142ae5420）。

## 卡R3（wall_compound_edit 完整族）追加

- **CompoundEditBuilder**（recovered_building/wall_history_edit.rs）：new_edit_sequence
  0x140a82b20 → next 0x140a82b40 → build 0x140a82d50（Option<WallHistoryCompoundEdit>
  0x108B）+ next_empty/with_concurrent；章节 O。
- **panic_deprecated_operation** 0x140a82e20（已弃用操作 panic；原文案待取证）。
- **WallHistoryEdit 操作变体族（~40）**：ChangeFreehandHeightType/PlasterPeelAmount/
  ChangePlasterPeelAmount/MorphPremadeShape/MorphFreehandWall…——**逐变体名/载荷由
  E1（re-analyst 40 变体映射）落地后吸收**（当前 WallHistoryEdit 4 结构变体
  Create/Delete/EnclosureChange/StateChange 已覆盖；操作变体未扩，不伪造载荷）。
- serde（deserialize 0x141401960 族 ~40 visitor / serialize 0x140314a00）与卡C EditType
  tag 对齐：待 E1 变体名落地 + MultiEdit deserialize 跳表 0x142aee31c 对齐（残留）。

## E1 吸收（RE-EDITTYPE.md，EditType/WallHistoryEdit 逐 tag→变体名）

- **EditType = 15 变体（tag 0..14，visit_str 0x140b3b630 权威）**：src 已按表修正——
  Tree=0/Path=1/Cobble1-3=2-4/Water=5/Garden=6/Wall=7/Roof=8/Color=9/DecoratorV5=10/
  Decorator=11/AutoClutter=13/Stairs=14；tag 12（4 字符名）未解登记。
- **Terrain 协调**：`EditType::Terrain`（deferred_terrain_history 0x140ac9770 O 使用）
  **原版 EditType 无此名**（"TerrainPath" 串非 visit_str 命中名，属 RasterEdit 族或他枚举）；
  地形历史记录机制待 E 补挖——Terrain 保留变体但 tag 定为保留号 15、明确标注待定位，不伪造。
- **WallHistoryEdit 17 操作变体**（tag 0..16，visit_str 0x1414022c0）：新增
  `WallHistoryOperationTag` 枚举登记名表（WallChanged0…ChangeSkirt16；tag15 名未解）；
  判别=首字段 niche 与顶层不共用。**待并入 WallHistoryEdit 结构**（并入会破坏 wall replay
  非穷尽匹配，E 补挖载荷后合并）。

## 库内可闭合缺口审计（逐项闭合）

1. **history 40-op 变体并入**：`WallHistoryOperationTag` 17 操作名表已登记（E1）。**并入
   WallHistoryEdit 结构交后续 restorer**（当前 4 结构变体为 wall 组部分解释；并入需 serde
   与 MultiEdit deserialize 跳表 0x142aee31c 对齐 + wall replay 通配核对——上下文将尽交接）。
2. **save extend_on_load tag=3**：卡2 已对齐（`OnLoadEntry::Batch` tag=3，RE-SESSION-SWITCH）✓。
3. **SaveBackupList 计数器**：已修（trailing_counter 对每个条目取 max(n+1)，不限于
   history-auto-；E/reviewer 0x140ae8470 机器码）✓。
4. **game_profile ron 字段序**：卡2 REVIEW4 已按原文 serialize 序重排，无回归 ✓。
5. **terrain 历史机制**：`EditType::Terrain`（deferred_terrain_history 0x140ac9770 O 使用）
   **原版 EditType 无此名**——地形历史记录机制（RasterEdit 族或独立编码）待 E 补挖登记。

## 编译/测试状态

- `cargo check -p country-core --offline`：我的文件零错误零警告。
- `cargo test --lib`（本组 + history）：**53/53 通过**（含 finish_replay/replay_mode 新增）。
- 全库测试目标编译曾被 wall 组 decorator.rs（in-flight）瞬时打断；wall 组修后已恢复。
  我的文件在测试构建零错误。

## 合同/状态建议

- inventory：history 模块 → `partial_implemented`（edit 类型补 5 文件 + EditType 14 变体；
  replay 族部分 + 域资源阻塞项登记）。
- 残留：EditType 40 变体逐 tag 名映射（0x142ad88d0 vs 0x142ad888c）、run_game_thread 载荷、
  save_game 本体 ECS 组装、capture_game_snapshot、全局接线。
