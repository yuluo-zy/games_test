# history 域 第三轮（章节化填充）总结（agent tg_a1，2026-10-14）

## 完成章数：27 / 27

`evidence/country-core-deep/chapters/` 下全部 history 域章节已就地更新
（12 resources/history + 15 systems/history）。每章含：文件作用（章首）、
业务函数表（作用=有依据推断、逆向结论 O/I/U、证据链接）、内联函数清单
（名称推断，逻辑并入调用点）。填充脚本：
`evidence/country-core-deep/round2/fill-chapters.py`（可重跑；知识基 =
DIGEST/CLOSURE/decomp 证据）。

| 文件 | 业务 | 内联 | 未覆盖 |
|---|---|---|---|
| resources/history/mod.rs | 34 | 73 | 0 |
| resources/history/wall_compound_edit.rs | 11 | 68 | 0 |
| resources/history/decorator_edit.rs | 10 | 35 | 0 |
| resources/history/terrain_edit.rs | 7 | 17 | 0 |
| resources/history/roof_edit.rs | 6 | 16 | 0 |
| resources/history/tree_edit.rs | 5 | 20 | 0 |
| resources/history/replay_cmd_buffer.rs | 4 | 15 | 0 |
| resources/history/color_edit.rs | 3 | 45 | 0 |
| resources/history/path_edit.rs | 3 | 10 | 0 |
| resources/history/wall_history.rs | 3 | 0 | 0 |
| resources/history/auto_clutter_edit.rs | 2 | 7 | 0 |
| resources/history/stairs_history_edit.rs | 2 | 10 | 0 |
| systems/history/replay_raster.rs | 9 | 13 | 0 |
| systems/history/history_process.rs | 8 | 9 | 0 |
| systems/history/replay_wall.rs | 4 | 8 | 0 |
| systems/history/mod.rs | 3 | 0 | 0 |
| systems/history/replay_terrain.rs | 3 | 2 | 0 |
| systems/history/replay_path.rs | 2 | 3 | 0 |
| systems/history/replay_water.rs | 2 | 3 | 0 |
| systems/history/signifier_replay_active.rs | 2 | 1 | 0 |
| 其余 7 个系统章（finish_replay/history_recording/replay_auto_clutter/replay_garden/replay_mode/replay_pop_bucket/replay_stairs） | 各 1 | 各 1-4 | 0 |
| **合计** | **127** | **~370** | **0** |

## 新反编译数：14（本轮新增落盘）

0x1409e6210（tree_edit::missing_prefab）、0x1417bdae0（send_state_diff_events，
107KB 巨函数）、0x140abfaa0（replay_wall::closure$0）、0x141447500/0x1414475a0/
0x141447640（garden/path/water 生成闭包）、0x140a01560（water closure$2）、
0x140b363e0（HistoryChainSnapshots::deserialize）、0x140a87970（impl$5::fmt）、
0x140b30a10（impl$25::fmt）、0x140b3b630、0x1417c0890、0x1417c2960（serde
visit_str ×3）。2 个 NotFound（0x140b39400/0x140b394f0 roof_edit visit_str，
位于合并区内，已按 serde 胶水处理）。
落盘至 history/decomp/（总计 85 个 decomp 文件）。

本轮新 O 事实（已写入对应章节）：
- missing_prefab 0x1409e6210 返回 24B {0x142ac4708, 0x142ac4710, "FAB_NAME"}，
  引用串 MISSING_TREE_PREFAB_NAME / TREE_PREFAB_NAME（tree_edit.rs）。
- impl$5::fmt（wall_compound_edit）= Display "WallHistoryCompoundEdit"；
  impl$25::fmt（decorator_edit）= Display "DecoratorStorageAddr…"（装饰存储键）。
- SOURCE_MAP 新名归位：TerrainEdit::update_terrain_curve = 0x140859740
  （纠正 sysres-v1 的 "reapply::closure$0" 名）、replay_raster::
  duration_based_on_stroke_length / get_stroke_from_0_to_t（内联）、
  History::closest_snapshot / pending_undo_count（内联）、
  WallHistoryEdit::gather_wall_ids（内联）。

## 仍空白函数数：0

（全部 27 章的业务+内联函数均有名称推断或 O/I/U 结论；无"待推断"残留。）
