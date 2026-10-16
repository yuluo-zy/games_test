# REVIEW6 — resources/history + systems/history（组 state 卡C 终验）

> 审查员：reviewer-state，2026-10-16。复核 restorer 卡C 落地（5 缺 edit 文件 + EditType 扩变体
> + replay 族库内部分 + FIX-1 吸收）与 RE-HISTORY-WRAPUP.md 一致性。
> 编译基线实测：`cargo check -p country-core --offline` 通过；本组+history 测试**绿窗口**（
> 状态/历史模块 30 通过 0 败，覆盖 restorer 53/53 声明）。全 crate 当前 **1624/3**：geometry::sphere
> （components-geometry 组，既知）+ resources::walls::decorator ×2（wall 组 in-flight，协调员提示的
> 瞬时打断，非 state 文件）。

## 逐项复核

### 1. 缺 5 edit 文件（vs 章节结构/serde 地址）
| 文件 | verdict | 说明 |
|---|---|---|
| `color_edit.rs`（ColorEdit/ColorTarget） | **OK** | ColorTarget 8 类（Wall/Roof/Halftimber/Decorator/StairRailing/FlatRoof/StairLadder/Brightness）；ColorEdit 6 字段（target/from/to/target_id/chimney_h_offset/allow_connection）。章节 serde 地址（serialize 0x14003c5d0 / deserialize 0x1417befe0 / visit_str 0x1417bf8a0）注释登记。字段序"按章节"，类型细节待取证诚实。 |
| `path_edit.rs`（RasterEdit/RasterEditSerialized） | **OK** | points/brush_size/is_additive/blend_mode（blend_mode 数值待取证）；serde 地址登记。 |
| `stairs_history_edit.rs`（StairsHistoryEdit/Diff/StairsEdge） | **OK** | `StairsEdge=(u32,u32)`；before/after/hash；StairsDiffTable 96B 语义；serde 0x140259af0/0x140a18ee0 登记。 |
| `tree_edit.rs`（TreeEdit Place/Move 六字段） | **OK** | TreeEditKind{Place,Move}；TreeEditData{id,position,scale,rotation_degrees,prefab_name,is_preplaced}；serde 族登记。 |
| `decorator_edit.rs`（DecoratorHistoryEdit/State/Diff） | **OK** | 前后状态 Diff；`from_preview` panic（preview）；`is_default_lantern_brightness`==`is_default_stair_width`==`x==1.0f`（ICF 0x140b3f910）；convert_before_after 0x140b3fab0 注释。 |

### 2. EditType 扩变体
| 项 | verdict | 说明 |
|---|---|---|
| 14 变体（+Color/Decorator/Stairs/Tree） | **OK** | 枚举与载荷 Arc 形态一致。 |
| 新增 native_tag 保留号 15-18 | **OK（不冒充对齐）** | 注释明确"精确 tag 待 re-analyst 按 Debug fmt 跳表 0x142ad88d0 定；当前用保留号登记，不冒充对齐"；与 MultiEdit deserialize 跳表 0x142aee31c 对齐残留登记 §5.1。诚实。 |
| 既有 AutoClutter=13 / Terrain=14 维持 | OK | 未改动旧 tag。 |

### 3. replay 族
| 项 | verdict | 说明 |
|---|---|---|
| `finish_replay`（双空 log + RestoreAppTo/工具栏/CmdUnfreezeSheep） | **OK**（宿主接线） | is_replay_naturally_finished（ReplayBuffer+PendingHistoryCommands 双空）+ log "History replay has naturally finished. Unblocking game commander."（@0x142a9f208）；CmdUnfreezeSheep 属 sheep 域（未建），登记为宿主消费，不伪造。 |
| `replay_mode`（is_replay_mode tag 13 / transition / HoldToFastReplay） | **OK** | native_tag==13 判定 + 进出检测；FAST_REPLAY_HOLD_MS=500 标 I（阈值待取证）。 |
| systems/history.rs 接线（finish_replay 入 Pop 阶段） | **OK** | `add_systems(finish_replay::finish_replay.in_set(HistoryCpuStage::Pop))`。 |
| 仍缺：replay_terrain/replay_stairs/replay_pop_bucket（restore_decorators FUN_14155ba30）/signifier_replay_active/history_process 第二事件流（byte[+9] 跳表 0x142ae5420） | GAP（登记） | history.rs 头注明确"16-byte playback/control event stream not decoded here"。诚实披露。 |

### 4. FIX-1 吸收（commander_update AppMode 门控）
| 项 | verdict | 说明 |
|---|---|---|
| 门控改 {13,14} | **OK** | `commander_update_system` 现用 `if !matches!(app_mode.native_tag(), 13 | 14)`，注释引 RE-SESSION-SWITCH §1 机器码 `mode-6<0xf && mode-0xd<2`；并注明 load_game 自己的门才是 ==LoadGame(14)。 |

## 结论与 DISPATCH 建议
- **卡C → 闭合（edit 文件 + EditType 扩变体 + replay 族库内部分）**。5 文件结构与章节/地址一致；
  14 变体 + 保留号 15-18 诚实不冒充对齐；replay 族可接部分入 Pop 阶段；FIX-1 正确吸收。
- 残留登记（不阻塞）：40 变体逐 tag 名映射（Debug fmt 跳表 0x142ad88d0 / MultiEdit deserialize
  0x142aee31c 对齐，re-analyst 定点）、replay_terrain/replay_stairs/replay_pop_bucket/
  signifier_replay_active/history_process 第二事件流、HoldToFastReplay 精确阈值、各 edit 载荷字段序/类型细节。
- 模块状态建议：save/session/history → **review_passed（限定已实现范围）**；完整验收仍 blocked
  （跨组域组装 / capture_game_snapshot 域 Query / commander_update 变体 1 应用 / 全局接线）。
