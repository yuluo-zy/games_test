# 第 10 轮 G2② 交付 — 回放消费者与读档残留（实现）

日期：2026-10-19
范围：`crates/country-core`（G2 文件权内）+ 1 处跨文件最小字段适配
契约：`evidence/round10/contracts/g2-replay-consumers-CONTRACT.md`（已逐条对齐 C1–C4、G1–G2）

---

## 1. 颜色回放（C1，契约 §2 / 清单 1–2）

节奏按原版 0x141c7f270 重写 `crates/country-core/src/systems/history/replay_color.rs`：

- 只在 `elapsed == 0` 派发一次（`replay_color.rs:42`）；删除 `cubic_in_out` 与每帧写事件。
- instant：派发后立即完成；timed：`required = max(duration/speed, 0.1)`（`replay_color.rs:39`），
  `required < elapsed` 完成；随后无条件 `elapsed += GameTime.delta`（不乘 speed）。
- 完成索引 `len > 1` 先反转再逐个移除。
- 事件改为 `ColorHistoryReplayApply { edit, undo }`（`replay_color.rs:34`，无 `progress`）。

真实写回 consumer `apply_color_replay`（`replay_color.rs:100`）：

| ColorEdit | 目标 | 行 |
|---|---|---|
| Wall / Gate / Roof / Halftimber | `WallColorIds` / `GateColorIds` / `RoofColorIds` / `HalftimberColorIds`（索引 = id，断言 < 0x5000） | :100–160 |
| Decorator | `DecoratorStorage` `DecoratorInfo.u28` | :163 |
| WindowLight | `WindowExtra.0`（经 `set_extra`） | :178 |
| Brightness | `LanternExtra.0`（一次取 `undo?from:to`，无插值） | :196 |
| FlagBunting / ChimneySmoke | `FlagExtra.0` / `ChimneyExtra.smoke` | :216 / :232 |
| StairLadder / MagicColor / StairRailing | `StairExtra` 偏移 8 / 12 / 0x10 | :248 / :256 / :264 |
| FlatRoof | 发 `SetFlatRoofStyleCmd` 给屋顶 customization owner（消费端在 system_roof，不在 core 调度） | :171 |
| StairSupport | 原版 switch 无 case → 不派发 | :281 |

装饰附加簇写入全部改为**无 panic** 的 `match`：变体不匹配时 `log::warn` 并跳过
（Window/Flag/Chimney；Lantern/Stair 原本即安全匹配）。

`register_history_events` 初始化四张颜色表（`history.rs`，与 `system_color` 的 `init_resource` 幂等）。
partial 登记：`set_wall_color_id` 的 plaster 分类、`PlayWallUiSound kind=6`、SSBO dirty 未在 core 复刻
（本组文件权不含 color/渲染域），不得视为完整等价的 `set_*` 链。

## 2. 装饰回放（C2，契约 §3 / 清单 3–4）

`replay_decorator.rs` 重写为原版 0x14080c8d0 职责，**不再写 `DecoratorStorage`**：

- `ReplayBucketAux` 新增 `cull_allow: Option<Arc<[DecoratorId]>>`（`resources/history/replay_cmd_buffer.rs:128`），
  take 语义：`replay_decorator.rs:35-47` 注入 `DecoratorArchivist.culling.allowed_ids`；None 不清空。
- 只处理 tag-7：instant 立即完成；timed `required=max(duration/speed, 0.05)`，`required < elapsed` 完成；
  `elapsed += delta`（不乘 speed）；完成后逆序移除（`replay_decorator.rs:50-76`）。
- 删除 `DecoratorHistoryReplayApply` 事件与存储写回 consumer；`register_history_events` 与
  `add_history_cpu_systems` 均已移除。
- 存储恢复唯一入口：Pop 的 `restore_decorators(reverse=undo)`（见 §3）。
- 生产 producer（history_process 的 cull 快照）未恢复 → `advance_history_cursor` 显式写 `cull_allow: None`
  （`history.rs:283-288`），typed gap。

## 3. Pop 生产路径（G1，契约 §1.2 / 清单 8–9）

- `native-physics`：只注册 `replay_pop_bucket_system`（`history.rs:411`），`finish_replay` 在其后；
  另一构建只注册 `replay_pop_bucket_cpu_system`（`history.rs:519`）。**任何构建只有一条 Pop 路径**，
  `pop_empty_replay_bucket` 仅保留为兼容/测试辅助，不再进生产 Schedule。
- 两个 ECS Pop 都把 host 资源改为 `Option`；**所有 aux 需要的 owner 都在弹桶前校验**
  （decorator diff → storage+id_gen；stairs diff → StairsState），失败路径不弹桶、不丢 diff。
  collider owner 缺失时用 no-op remover 并 warn（非静默 no-op，decorator 状态仍写回）；
  archivist/raycast 缺失时跳过对应收尾。
- CPU Pop 走 `restore_decorators` + `restore_backup/apply_diff/make_backup` + `extract_diff` + flush。
- `history_clear_previous_state` 在 Cursor 阶段 `after(history_process_system)`、`run_if` archivist 存在
  （`history.rs:694-702`）。
- Schedule 顺序：Cursor（命令展开 + 清 allowed）→ Replay（各域 replay + 三个真实 consumer）→ Pop →
  finish_replay → Record（`history.rs:668+`）。

## 4. 墙颜色 / 墙音频（C3，契约 §4 / 清单 5–6）

- `apply_wall_replay_color`（`replay_wall.rs:50`）按 `observed_special_owner`（source+65）分流：
  false → `WallColorIds`；true → `GateColorIds`；两边都做 `< MAX_NUMBER_OF_WALLS` 边界检查。
  partial 登记同 §1（plaster/sound/SSBO）。
- `apply_wall_replay_audio`（`replay_wall.rs:79`）→ `PlayWallAudio { kind, position=anchor, flag,
  value=0.0, volume=1.0 }`；value/volume 为 I（载荷无对应字段，契约 §4.3 同结论）。
- 测试覆盖 special/normal 两向与 kind 0/1/2 × flag 0/1 映射。

## 5. 楼梯音频（C4，契约 §5 / 清单 7）

`replay_stairs.rs`：

- kind 双路径：instant `0x12 - undo`，animated `0x11 + undo`（`replay_stairs.rs:84-89`）。
- position：`CachedDecoratorTransforms::try_position(edge1)`，未命中 0。
- **消除伪字段**：`stairs_audio_system` 只读 kind/position/values（证据
  `multi-baseline/audio-events-analysis.md` §5b：瞬发路径 set_position + stair_width + size），
  不读 `+0x28/+0x30`。因此不再把 `u30` 写 `tail`、不新增 `flag` 字段；`PlayStairsAudioCmd`
  维持原字段集，producer 侧保留 DecoratorStorage 过滤查询作为原生语义接线点并登记 U。
- 其余 `values/tail/bytes` 来源未解码，保持零并登记 U。
- 测试：instant×redo/undo、animated×redo/undo 四组合 kind + position。

## 6. 读档残留（契约 §6 / 清单 10–12）

### 6.1 RoofAssociations（`snapshot.rs:1500-1660`）— 延后重建

- Roofs spawn 时记录 `(Entity, Roof::wall_id)`；用 `PublicWalls.get(wall).building_projection()` 生成
  `PublicWallBuildingState` 行，`public_wall_order = PublicWalls::wall_ids()`，`phase = 0`，
  `marked = false`（capture 已 `Without<RoofMarkedForDeletion>`）。
- **读档顺序修正**：`Walls` 段只写 `InnerWalls`，`PublicWalls` 要等发布阶段才有行。apply 期行不全时
  不再记“永久 gap”，而是把行写入 `PendingRoofAssociationRebuild`（`snapshot.rs:1500+`），由新增系统
  `rebuild_pending_roof_associations`（`snapshot.rs:1531`）在 `WallCpuStage::Publish` 之后每帧重试，
  集齐后调用一次 `rebuild`、消费 `AssociationChanges` 并清空 pending。行齐全时仍在 apply 内立即重建。
- `AssociationChanges` 被真实消费：`invalid_roof_requests → DeleteRoofCmd`，
  `wall_presence_changes → WallRoofPresenceChanged`（立即路径与延后路径均实现）。
- `SnapshotApplyReport` 新增 `roof_associations: usize`（`snapshot.rs:1485`）。
- 系统注册：`add_history_cpu_systems` 中 `.after(WallCpuStage::Publish)`；pending 资源与
  `RoofAssociations` 在 `register_history_events` 初始化。
- 测试：`pending_roof_associations_wait_for_public_wall_rows`（无墙行时保持 pending）。
- 未做：36 样本端到端关联数校验（样本 load 后需运行墙发布阶段；现有解析测试仍绿）。

### 6.2 树 prefab（契约 §6.2）— 真实可解析路径

- 空 `prefab_name` 时优先发真实 `InsertTreeCmd`（`tree_edit`，消费端 =
  tree-tool `process_tree_cmds`，其 Insert 分支对 `MISSING_TREE_PREFAB_NAME` 调用
  `random_tree_prefab(id&1, deny_load, is_halloween, is_corrupted, false)`）；实体 / TreeStorage /
  prefab 实例都由该 consumer 建立，snapshot 不再为这些条目手工 spawn，避免双实体。
  sentinel 来自 `resources/history/tree_edit.rs::missing_prefab`。
- `Events<InsertTreeCmd>` 缺失时退回“手工 spawn + `SpawnPrefabCmd` sentinel”，并在 gap 中区分
  dispatched/fallback 数量（不静默）。
- 跨 crate 接缝登记：core 不复制 `random_tree_prefab`；tree-tool 的
  `register_tree_tool_events` / `add_tree_tool_cpu_systems` 必须由上层装配（当前 tiny-glade
  未注册），接入后 sentinel 即被真实解析。

### 6.3 OnTerrainChange（契约 §6.3）— 跨模块缺口

真实 reader = `systems::prefab::update_prefabs` 0x140841d60（PDB 签名 O），当前 prefab 域未实现/注册；
该文件不在 G2 文件权内，故不作伪 reader。terrain writer 本身无需改。

### 6.4 Clutter `to_info`（契约 §6.4 / 清单 12）

`resources/history/decorator_edit.rs:124`：`ty < 0x38`（Clutter）与 `0x3e`/未知共享原生
`ty-0x38` 夹取后的 index 6 分支，保留 `u30/u38/u3c/u40` 原始槽、style 回读 `state.style`
（Clutter 为 0，不再默认 3）。新增 Clutter round-trip 测试；原 `to_info_type...` 测试对
`0x3e` 的旧期望（style 3）按契约修正为 0。逐 case 的剩余槽语义仍为 U（typed gap）。

## 7. 测试与编译

- `cargo check -p country-core` ✅
- `cargo check -p tiny-glade` ✅（native-physics 分支的 Pop/consumer 一并编译）
- `cargo test -p country-core --lib`：**2510 passed / 0 failed**（基线 2467 + 新增 43）
- `cargo test -p country-core --lib systems::save`：**50 passed / 0 failed**（第 9 轮 49 条基线 + 1 条延后重建测试）
- `cargo test -p country-core --lib systems::history`：**60 passed / 0 failed**
- 新增测试（每 consumer 至少一条）：
  - `replay_color`：redo/undo 表写入；timed 只派发一次 + delta 不乘 speed；0.1 floor；
    brightness 一次性；flag/window 附加簇；FlatRoof→`SetFlatRoofStyleCmd`。
  - `replay_decorator`：cull take 注入；floor 0.05 计时；回放不写存储。
  - `replay_wall`：create→墙色表；special→门色表；音频 kind/flag/anchor 映射表。
  - `replay_stairs`：instant/animated × undo 四组合 kind + position。
  - `decorator_edit`：Clutter/0x3e 扩展簇保留与 style 回读 0（旧 0x3e 期望已修正并复跑）。
  - `snapshot`：`PendingRoofAssociationRebuild` 在无墙行时保持 pending。
  - `history/tests.rs`（`cfg(native-physics)`）：NewReplayCommandEvent 清空旧 allowed 集合。

## 8. 仍缺项（明确登记）

1. `ColorHistoryReplayApply` 目前由 core 直接写 CPU 表；`system_color` 的 plaster/声音/SSBO 副作用未复刻
   （partial）。`SetRoofColorIdCmd` 定义在 `system_color`，属跨 crate 接缝。
2. decorator cull-allow producer（history_process 快照）未恢复，生产为 `None`；注入链路已就绪。
3. `replay_pop_bucket` 的 stairs 结构连接两张 map 仍未建模（原登记 U；非空 stairs diff 显式 Err）。
4. 非 native-physics Pop 的 collider 移除为 no-op（CPU remover 未提供，已 warn）。
5. 楼梯音频 `values/tail/bytes` 槽来源未解码（U）；原生 +0x28/+0x30 不被
   `stairs_audio_system` 读取（证据 §5b），故不写入事件；DecoratorStorage 过滤查询保留为接线点。
6. RoofAssociations：36 样本端到端关联数校验需在 load 后运行墙发布阶段；延后重建系统已接，
   `AssociationChanges` 事件消费者（roof 域）是否注册属上层。
7. 空 prefab：core 已发可被 `process_tree_cmds` 解析的 `InsertTreeCmd`(sentinel)；
   tree-tool 的系统注册仍在 host 侧待接（跨 crate 接缝）。
8. `OnTerrainChange` 真实 reader（`systems::prefab::update_prefabs`）待 prefab 域实现。
9. Clutter/未知 ty 的逐 case 槽映射仍为 U（保留原槽，不丢字段）。
10. FlatRoof 回放只发 `SetFlatRoofStyleCmd`；consumer（system_roof）在 core 调度外，
    与 `SetRoofColorIdCmd` 同属跨 crate 接缝。

## 9. 跨文件改动登记

- `crates/country-core/src/systems/save/snapshot.rs`：同 crate 内 `impl Resource for RoofAssociations`
  （`resources/roofs/associations.rs` 不在 G2 文件权，未直接改其文件）。
- G2③ 修复后 `systems/audio/mod.rs` 不再需要任何改动（伪 `flag` 字段已撤回）。
