# 下一步分派建议(2026-10-15,coordinator)

> 供主会话把结果派给 workers。本批 9 模块按组 eco/state/interaction 已开工(in_progress 依
> 主会话写入);本清单给出**依赖顺序、组内分工与验收卡点**,不重复 DISPATCH.md 的原函数清单
> (以 DISPATCH.md 细化门槛为准)。

## 基线事实(本日实测)

- `cargo check -p country-core --all-targets` 通过(exit 0,1m31s);inventory `compiled_files`
  快照(00:12)早于当前 src(20:48),现场 check 为准。
- reports/ 三组目录已建、空;planning/ 本文件为第一条。

## 分派建议(模块 → 组/角色 → 理由 → 依赖)

### 立即执行(restorer 已在岗)

| 模块 | 组/角色 | 理由 | 依赖顺序 |
|---|---|---|---|
| `systems/ivy` 的 ivy_spawner + mod.rs 登记(eco/restorer) | 补 `ivy_spawner`(5 函数:ivy_spawner/try_random_ivy_root/try_ivy_root_next_to_cursor/is_point_inside_wall_segment)与 add_ivy_systems/insert_ivy_resources/add_ivy_events;这是 ivy 生产链的根 | 依赖 ivy_storage/ivy_grower 已在位;先闭合生产者再让 grower/leaf_spawner 的"全局生产/消费链"验收 | 依赖:ivy_storage(在位)。供:ivy_grower/leaf_animation/GPU |
| `systems/garden` 的 systems/garden.rs impl$0::build(eco/restorer) | 原 3 文件只缺 build 注册;当前 mod.rs 有事件与 replay 路由,补 build 后可与 world_raster::draw_raster<GardenRasterStagingSsbo> 对账 | garden 四链(draw/history/replay/注册)才能闭合 | 依赖:draw_garden/record_garden_history(在位) |
| `resources/critters/sheep` 验收口径确认(eco/reviewer) | sheep.rs 28 函数多属 serde/register_required_components 胶水,完整 Sheep owner 按 README 不复制;需 reviewer 先定"捕获/恢复 + Sheep 节接入"为验收边界,避免把 28 函数逐条当业务 | 避免对 derive/胶水逐函数计业务 | 依赖:control.rs 捕获/恢复(在位) |
| `systems/save` 步骤 1+2(save 纯逻辑层)(state/restorer) | 按跨组裁决先闭合 shadow_file/save_thread(不含 on_load 回放)/save_backup_* /snapshot_gc/save_file_identity/load_completion_checker 与 save_game 捕获+各节 to_json_string | 不依赖 meta_commander,可先行 | 依赖:save_file.rs(在位)。供:load_game/会话切换 |
| `resources/session` 数据结构部分(state/restorer) | NextSession(load/create/ignore_snapshot)、SavedSession 列表/删除、refresh/load_saved_session_list、SessionFlowPlugin build 不依赖 meta_commander,先落地 | append_on_load<MetaCommand> 留最后 | 依赖:SessionId/SessionMeta(在位) |
| `systems/backend/input` 缺失 5 文件(interaction/restorer) | translate_input/update_cursor_position/reset_input_state/input_record_replay/input_replay_for_trailers;现有 calculate_input_delta+translate_input_to_cursor_moved 已闭合下半链,补上半链后全链验收 | 上链生产者→下链翻译→录制回放 | 依赖:ActionSetValues/MainActionSet(在位) |
| `systems/terrain` 缺 5 文件(interaction/restorer) | deferred_terrain_history/terrain_hover_idx/ui_draw_terrain/ui_edit_terrain/ui_terrain_focus_mode;编辑/延迟回读已接,补工具 UI+历史生产 | 依赖 edit/draw/delayed(在位) | 依赖:AppMode/ToolUI 接线 |

### 定点逆向(re-analyst,为上述/后续供证)

| 目标 | 组/角色 | 产出 | 依赖 |
|---|---|---|---|
| `meta_commander.rs`(commander_update 0x14091f180/MetaCommand 0x38B 元素 tag 0..0x13/MetaCommandFence 0x14091ef90/peek 0x14091f0c0/consume 0x14091f100) | state/re-analyst | RE-ANALYSIS.md:命令枚举+载荷 schema+fence 语义,供 session append_on_load 与 load_game on_load 回放 | 独立;是会话切换链的前置证据 |
| save_game 的 GameSaveData 组装(0x140a3f470 区域,~9 资源 +0x28..+0x100) | state/re-analyst | GameSaveData 结构/字段顺序/默认值 → 共享类型 | 供 save_game 同步路径与 load apply |
| session 的 append_on_load<MetaCommand,3>(0x1403b5150)+<MetaCommandFence>(0x141328ac0)载荷 U | state/re-analyst | 3 条 on_load 命令内容与 fence 终止条件 | 依赖 meta_commander 证据 |
| ivy GPU 消费(0x14? ivy_send_to_gpu_leaf/segments)与 spawner VA | eco/re-analyst | 缺文件 3 个的算法与地址;若未深挖先补章节 | 供 ivy 生产者→GPU 验收 |
| terrain 工具 UI(ui_edit_terrain/ui_terrain_focus_mode VA)与输入源(AppMode/ToolUI) | interaction/re-analyst | 工具状态机/掩码/脏格证据 | 供 terrain 工具链验收 |

### 评审轮(reviewer,按 restorer 落盘推进)

- 每模块 RESTORE.md 落盘后 reviewer 复核"实现层面 ≠ 验收层面":核对 RESTORE 是否枚举原函数
  清单 + fresh 反编译证据;揪出空壳/假数据/依赖胶水冒充业务(如 session 未伪造加载成功、
  sheep 未复制空壳、ivy GPU 未假装消费)。
- 首批建议先评:save 步骤 1+2、session 数据结构部分、input 上链 —— 三者可独立形成
  review_passed 小块,不阻塞他人。

## 依赖顺序总览

```
[不依赖 meta_commander,可并行]                     [依赖 meta_commander,最后一条链]
save 纯逻辑层 ─────────────────┐
session 数据结构部分 ──────────┼──→ 会话切换链验收:session append_on_load
input 上链(独立) ─────────────┤      + load_game on_load 回放 + load_game_session
terrain 工具 UI(独立) ────────┤
garden build(独立) ───────────┤
ivy spawner→GPU(独立) ────────┘
```

## 卡点与回传

- meta_commander 定点挖出前,session/save 的"完整切换/异步捕获"不得标完成;load_game_session
  保持现状(README 一致,不伪造加载成功)。
- 若 state/re-analyst 挖 meta_commander 受阻(如 0 原函数索引致章节证据不足),回主会话裁决:
  派给 interaction/reanalyst 或单开会话切换链任务。
