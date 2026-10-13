# crates/country-core/src/systems/history

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

Wall/Roof外层bucket与同Wall组barrier、计时和离散回放；terrain/tree/water/auto-clutter局部链。

## 接线与验证

接线：建筑World已接，独立terrain/tree/clutter不冒称统一游戏接线。

建筑冻结链通过；terrain Edit512原作↔Rust；其他新回放候选范围独立。

## 已恢复的函数和类型范围

- replay_wall/replay_pop_bucket限定范围
- Roof VisualEdit/create/delete/direction/rim
- replay_terrain Edit
- replay_tree/replay_water/replay_raster/replay_auto_clutter候选

## 明确缺口

- 全部命令owner统一调度、失败展开、snapshot联接
- Garden/Path/Stairs/Color完整消费者

下一闭包：全部命令owner统一调度、失败展开、snapshot联接

## 证据

- [evidence/building-integrate/stage-reviews/2026-10-11-stage4-frozen.md](../../evidence/building-integrate/stage-reviews/2026-10-11-stage4-frozen.md)
- [evidence/multi-version/B-replay-verification.json](../../evidence/multi-version/B-replay-verification.json)
- [evidence/terrain-editing/milestones/M1/status.md](../../evidence/terrain-editing/milestones/M1/status.md)
- [evidence/multi-baseline/README.md](../../evidence/multi-baseline/README.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/systems/history/finish_replay.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/history/history_process.rs` | A,B,C | [reconstruction/history/transaction.rs](../../reconstruction/history/transaction.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-c4cdc0aa5e62db9266/abc-building-35/crates/country-core/src/systems/history/history_process.rs](../../skeleton/task-index/work/TG-c4cdc0aa5e62db9266/abc-building-35/crates/country-core/src/systems/history/history_process.rs)：explicit_candidate_delivery_mapping | country_core::systems::history::history_process::HistoryCmd::interrupt（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/history/history_recording.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/history/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_auto_clutter.rs` | B,C | [reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_garden.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_mode.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_path.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_plant_paint.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_pop_bucket.rs` | A,B,C | [reconstruction/building/wall_history_replay.rs](../../reconstruction/building/wall_history_replay.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_raster.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_stairs.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_terrain.rs` | A,B,C | [reconstruction/terrain/crates/tg-terrain/src/replay.rs](../../reconstruction/terrain/crates/tg-terrain/src/replay.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_wall.rs` | A,B,C | [reconstruction/building/wall_history_replay.rs](../../reconstruction/building/wall_history_replay.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/history/replay_water.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/history/signifier_replay_active.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/wall_history_replay.rs

[reconstruction/building/wall_history_replay.rs](../../reconstruction/building/wall_history_replay.rs)

SHA256：`9d024a80ae4f429d7fdb24737a42d8908775147acf8b53efce0cb538173728e4`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `wall_id` | 40 | 未由扫描证明；以证据限定范围为准 |
| `advance_wall_groups` | 50 | 未由扫描证明；以证据限定范围为准 |
| `expand_wall_compound` | 74 | 未由扫描证明；以证据限定范围为准 |
| `cubic_in_out` | 88 | 未由扫描证明；以证据限定范围为准 |
| `divide` | 92 | 未由扫描证明；以证据限定范围为准 |
| `add` | 96 | 未由扫描证明；以证据限定范围为准 |
| `advance` | 104 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_history_transition.rs

[reconstruction/building/wall_history_transition.rs](../../reconstruction/building/wall_history_transition.rs)

SHA256：`03268d77edcc153b8d4f7ddd1b362fde975f3dda6469e2aa6434c392398c62de`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `add` | 8 | 未由扫描证明；以证据限定范围为准 |
| `progress` | 12 | 未由扫描证明；以证据限定范围为准 |
| `advance_wall_transition` | 25 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L24：/// to its existing implementation. No unknown operation is accepted.
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs)

SHA256：`f0de6bb3e439d91cec459fa201eea0be133dd9f5cb36c378de2507cc435cb00d`；缺口注释标记数：8。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `delete` | 52 | 未由扫描证明；以证据限定范围为准 |
| `history_record_auto_clutter` | 62 | 未由扫描证明；以证据限定范围为准 |
| `start_input_recording` | 75 | 未由扫描证明；以证据限定范围为准 |
| `add` | 76 | 未由扫描证明；以证据限定范围为准 |
| `handle_rmb_autoclutter` | 89 | 未由扫描证明；以证据限定范围为准 |
| `from` | 116 | 未由扫描证明；以证据限定范围为准 |
| `ui_convert_autoclutter_to_decoclutter` | 125 | 未由扫描证明；以证据限定范围为准 |
| `replay_auto_clutter` | 142 | 未由扫描证明；以证据限定范围为准 |
| `erase_decorator_pair` | 201 | 未由扫描证明；以证据限定范围为准 |
| `unknown` | 207 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L22：//! 保留 unknown。
- L101：unknown(); // 编辑记录（stride 0x48）写入 indexmap
- L103：unknown(); // Rapier raycast 拾取链
- L126：unknown();
- L190：/// +0x20/+0x28 unknown。
- L203：unknown();
- L207：fn unknown() -> ! {
- L208：unimplemented!("反汇编级证据：精确逻辑见 evidence/multi-baseline/clutter-extra-analysis.md")
### reconstruction/crates/tg-building-runtime/src/history.rs

[reconstruction/crates/tg-building-runtime/src/history.rs](../../reconstruction/crates/tg-building-runtime/src/history.rs)

SHA256：`3c873674973a57ecae358333bb8ade397c1cd434eddb1fc4fb4af5e012cf9726`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `default` | 44 | 未由扫描证明；以证据限定范围为准 |
| `register` | 75 | 未由扫描证明；以证据限定范围为准 |
| `prepare` | 87 | 未由扫描证明；以证据限定范围为准 |
| `record_and_finish` | 281 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/history/transaction.rs

[reconstruction/history/transaction.rs](../../reconstruction/history/transaction.rs)

SHA256：`9a96369ccfa607593b10675919d8f0c8ff00c8bf7e22b6eb71b99bd69f7bd1bd`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_edits` | 15 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 18 | 未由扫描证明；以证据限定范围为准 |
| `rim_events` | 63 | 未由扫描证明；以证据限定范围为准 |
| `from_observed` | 79 | 未由扫描证明；以证据限定范围为准 |
| `ledger` | 82 | 未由扫描证明；以证据限定范围为准 |
| `is_recording` | 83 | 未由扫描证明；以证据限定范围为准 |
| `pending` | 84 | 未由扫描证明；以证据限定范围为准 |
| `start_input_recording` | 85 | 未由扫描证明；以证据限定范围为准 |
| `try_start_input_recording` | 91 | 未由扫描证明；以证据限定范围为准 |
| `nuke_future_states` | 96 | 未由扫描证明；以证据限定范围为准 |
| `add` | 116 | 未由扫描证明；以证据限定范围为准 |
| `add_with_nonce_provider` | 120 | 未由扫描证明；以证据限定范围为准 |
| `finish_input_recording` | 129 | 未由扫描证明；以证据限定范围为准 |
| `finish_with_nonce_provider` | 132 | 未由扫描证明；以证据限定范围为准 |
| `finish_potential_mutation` | 145 | 未由扫描证明；以证据限定范围为准 |
| `finish_potential_with_nonce_provider` | 148 | 未由扫描证明；以证据限定范围为准 |
| `cursor_step` | 161 | 未由扫描证明；以证据限定范围为准 |
| `after_observed_future_clear` | 191 | 未由扫描证明；以证据限定范围为准 |
| `from_observed_active` | 205 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 208 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/roof/replay.rs

[reconstruction/roof/replay.rs](../../reconstruction/roof/replay.rs)

SHA256：`8d3eb0b1d7956a96681d19502cd647c900c4fe35e25b0f260f1066ec38b3f9df`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `write_observed_fields` | 33 | 未由扫描证明；以证据限定范围为准 |
| `observed_tag` | 68 | 未由扫描证明；以证据限定范围为准 |
| `step_visual_replay_resolved` | 80 | 未由扫描证明；以证据限定范围为准 |
| `from_observed` | 95 | 未由扫描证明；以证据限定范围为准 |
| `bytes` | 98 | 未由扫描证明；以证据限定范围为准 |
| `elapsed` | 101 | 未由扫描证明；以证据限定范围为准 |
| `finished` | 104 | 未由扫描证明；以证据限定范围为准 |
| `advance_discrete_observed` | 111 | 未由扫描证明；以证据限定范围为准 |
| `advance_direction` | 173 | 未由扫描证明；以证据限定范围为准 |
| `advance` | 192 | 未由扫描证明；以证据限定范围为准 |
| `get` | 251 | 未由扫描证明；以证据限定范围为准 |
| `invalid_result` | 254 | 未由扫描证明；以证据限定范围为准 |
| `visual_edit_from_observed` | 263 | 未由扫描证明；以证据限定范围为准 |
| `write_observed_event` | 284 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/terrain/crates/tg-terrain/src/replay.rs

[reconstruction/terrain/crates/tg-terrain/src/replay.rs](../../reconstruction/terrain/crates/tg-terrain/src/replay.rs)

SHA256：`8c40fb236f67e5b1b52c4a2fd2c8a545009752d50f10085393b12152176c34cc`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `command` | 18 | 未由扫描证明；以证据限定范围为准 |
| `observe_edit` | 29 | 未由扫描证明；以证据限定范围为准 |
| `edit_command` | 46 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/tree-tool/src/replay_tree.rs

[reconstruction/tree-tool/src/replay_tree.rs](../../reconstruction/tree-tool/src/replay_tree.rs)

SHA256：`401ee26e52c7ca887ed7432120affe8bacfc9397b61791e91fa78d037dae7fd1`；缺口注释标记数：14。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `front_mut_bucket` | 40 | 未由扫描证明；以证据限定范围为准 |
| `iter_mut` | 51 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 54 | 未由扫描证明；以证据限定范围为准 |
| `replay_tree` | 120 | 未由扫描证明；以证据限定范围为准 |
| `read_u64` | 193 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L19：//! unknown）。GameTime delta 证据：f32 @+0x28（两处独立调用点一致）。
- L30：/// [evidence] f32 @+0x28 = 延时除数（回放倍速，语义 unknown）。
- L32：/// 内部双缓冲桶（front/back；细节 unknown）。
- L34：/// [evidence] f32 @+0x28：ReplayCmd.delay 的除数（回放倍速，语义 unknown）。
- L73：/// byte @+0x1c：flag（选两种负载布局；语义 unknown）。
- L80：/// 1 = Move、2 = Insert、其余 = Delete。字段布局部分证据，其余 unknown。
- L88：pub unknown: [u8; 0x60],
- L113：pub unknown: [u8; 0x38],
- L148：let index = read_u64(&edit.unknown, 0x30);
- L151：position: Vec3::ZERO, // [evidence] position 字段偏移见 TreeEdit（unknown）
- L158：name: String::new(), // [evidence] 复制 TreeEdit 名字；字段偏移 unknown
- L160：unknown: [0; 0x38],
- L165：let index = read_u64(&edit.unknown, 0x18);
- L194：0 // 依赖布局 unknown，草稿占位
### skeleton/task-index/work/TG-c4cdc0aa5e62db9266/abc-building-35/crates/country-core/src/systems/history/history_process.rs

[skeleton/task-index/work/TG-c4cdc0aa5e62db9266/abc-building-35/crates/country-core/src/systems/history/history_process.rs](../../skeleton/task-index/work/TG-c4cdc0aa5e62db9266/abc-building-35/crates/country-core/src/systems/history/history_process.rs)

SHA256：`c378bf69e6f8d60bb2207be4835bd70ae60a2b4acc5f7eea83d3ed334570b10a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `interrupt` | 13 | 未由扫描证明；以证据限定范围为准 |
