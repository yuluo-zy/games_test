# crates/country-core/src/resources/history

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

A/B/C历史格式解析、Wall/Roof事务和游戏命令字段；其他命令保留未知值。

## 接线与验证

接线：建筑已验收owner接World，其余未建立。

历史静态样本与Rust解析有证明，建筑事务限定链通过；不等于完整存档。

## 已恢复的函数和类型范围

- History start/add/finish/cursor、分叉截redo
- Roof五类owner和Wall compound edit
- 原历史大整数ID/未知tag保留

## 明确缺口

- Path/Garden/Color/Stairs等完整owner和schema消费者
- 快照解码、22缺失快照引用与C迁移

下一闭包：Path/Garden/Color/Stairs等完整owner和schema消费者

## 证据

- [evidence/multi-version/history-rust-verification.json](../../evidence/multi-version/history-rust-verification.json)
- [evidence/building-history/B-transaction-verification.json](../../evidence/building-history/B-transaction-verification.json)
- [evidence/building-history-owners/B-owner-verification.json](../../evidence/building-history-owners/B-owner-verification.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/resources/history/auto_clutter_edit.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/history/color_edit.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/history/decorator_edit.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/history/mod.rs` | A,B,C | [reconstruction/history/transaction.rs](../../reconstruction/history/transaction.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-3846e898cf43c29d09/abc-tools-52/crates/country-core/src/resources/history/mod.rs](../../skeleton/task-index/work/TG-3846e898cf43c29d09/abc-tools-52/crates/country-core/src/resources/history/mod.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-3846e898cf43c29d09/abc-building-35/crates/country-core/src/resources/history/mod.rs](../../skeleton/task-index/work/TG-3846e898cf43c29d09/abc-building-35/crates/country-core/src/resources/history/mod.rs)：explicit_candidate_delivery_mapping | country_core::resources::history::is_decorator_history_diff_empty（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::history::history_chain::HistoryChainIndex::new（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/history/path_edit.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/history/plant_paint_edit.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/history/replay_cmd_buffer.rs` | A,B,C | [skeleton/task-index/work/TG-d63bc50864713c7344/abc-building-35/crates/country-core/src/resources/history/replay_cmd_buffer.rs](../../skeleton/task-index/work/TG-d63bc50864713c7344/abc-building-35/crates/country-core/src/resources/history/replay_cmd_buffer.rs)：explicit_candidate_delivery_mapping | country_core::resources::history::replay_cmd_buffer::ReplayBuffer::len（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/history/roof_edit.rs` | A,B,C | [reconstruction/roof/replay.rs](../../reconstruction/roof/replay.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/history/stairs_history_edit.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/history/terrain_edit.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/history/tree_edit.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/history/wall_compound_edit.rs` | A,B,C | [reconstruction/building/wall_history_edit.rs](../../reconstruction/building/wall_history_edit.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-3c7e1129dc1230a273/abc-building-35/crates/country-core/src/resources/history/wall_compound_edit.rs](../../skeleton/task-index/work/TG-3c7e1129dc1230a273/abc-building-35/crates/country-core/src/resources/history/wall_compound_edit.rs)：explicit_candidate_delivery_mapping | country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/history/wall_history.rs` | A,B,C | [reconstruction/building/wall_history_edit.rs](../../reconstruction/building/wall_history_edit.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/wall_history_edit.rs

[reconstruction/building/wall_history_edit.rs](../../reconstruction/building/wall_history_edit.rs)

SHA256：`2be910c7806f16e13988ff02e822d6cd514db07145c7451479fcf8346c31e58c`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `for_new_wall` | 20 | 未由扫描证明；以证据限定范围为准 |
| `record_deleted_wall` | 74 | 未由扫描证明；以证据限定范围为准 |
| `apply_instant_wall_edit` | 82 | 未由扫描证明；以证据限定范围为准 |
| `slice_source_curve` | 130 | 未由扫描证明；以证据限定范围为准 |
| `record_new_wall` | 146 | 未由扫描证明；以证据限定范围为准 |
| `interpolate_rectangle_enclosure` | 171 | 未由扫描证明；以证据限定范围为准 |
| `rectangle_from_enclosure` | 199 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-history/src/lib.rs

[reconstruction/crates/tg-history/src/lib.rs](../../reconstruction/crates/tg-history/src/lib.rs)

SHA256：`a16555c05df5405f0aab4a45f0163b95f9f42c4501c7a6289a684692565367d8`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `fmt` | 82 | 未由扫描证明；以证据限定范围为准 |
| `object` | 104 | 未由扫描证明；以证据限定范围为准 |
| `singleton` | 108 | 未由扫描证明；以证据限定范围为准 |
| `parse` | 118 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 126 | 未由扫描证明；以证据限定范围为准 |
| `to_json` | 130 | 未由扫描证明；以证据限定范围为准 |
| `entries` | 134 | 未由扫描证明；以证据限定范围为准 |
| `edits` | 141 | 未由扫描证明；以证据限定范围为准 |
| `roof_commands` | 149 | 未由扫描证明；以证据限定范围为准 |
| `summary` | 177 | 未由扫描证明；以证据限定范围为准 |
| `summarize_json` | 223 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L3：//! Keep the raw document so unknown commands, extensions and u64 IDs survive.
### reconstruction/history/building_transactions.rs

[reconstruction/history/building_transactions.rs](../../reconstruction/history/building_transactions.rs)

SHA256：`f87218f20f0667be8b6066f5b23e4dab9bf1bf4d865c9398ba3d6e2d0c17b7ff`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `write_observed_arc_fields` | 33 | 未由扫描证明；以证据限定范围为准 |
| `instant_request` | 43 | 未由扫描证明；以证据限定范围为准 |
| `write_observed_fields` | 52 | 未由扫描证明；以证据限定范围为准 |
| `from` | 73 | 未由扫描证明；以证据限定范围为准 |
| `from_observed` | 80 | 未由扫描证明；以证据限定范围为准 |
| `payload` | 86 | 未由扫描证明；以证据限定范围为准 |
| `tag` | 89 | 未由扫描证明；以证据限定范围为准 |
| `wall_id` | 92 | 未由扫描证明；以证据限定范围为准 |
| `create` | 101 | 未由扫描证明；以证据限定范围为准 |
| `delete` | 104 | 未由扫描证明；以证据限定范围为准 |
| `delete_from_roof` | 110 | 未由扫描证明；以证据限定范围为准 |
| `presence` | 118 | 未由扫描证明；以证据限定范围为准 |
| `direction` | 125 | 未由扫描证明；以证据限定范围为准 |
| `rim` | 131 | 未由扫描证明；以证据限定范围为准 |
| `visual` | 137 | 未由扫描证明；以证据限定范围为准 |
| `collect_last_visual_history` | 164 | 未由扫描证明；以证据限定范围为准 |
| `from_observed` | 189 | 未由扫描证明；以证据限定范围为准 |
| `from_before_after` | 194 | 未由扫描证明；以证据限定范围为准 |
| `record` | 203 | 未由扫描证明；以证据限定范围为准 |
| `edit` | 206 | 未由扫描证明；以证据限定范围为准 |
| `collect_direction_history` | 211 | 未由扫描证明；以证据限定范围为准 |
| `eq` | 232 | 未由扫描证明；以证据限定范围为准 |
| `from` | 246 | 未由扫描证明；以证据限定范围为准 |
| `expanded` | 263 | 未由扫描证明；以证据限定范围为准 |
| `payload` | 276 | 未由扫描证明；以证据限定范围为准 |
| `state` | 279 | 未由扫描证明；以证据限定范围为准 |
| `advance` | 284 | 未由扫描证明；以证据限定范围为准 |
| `into_replay_buckets` | 329 | 未由扫描证明；以证据限定范围为准 |
| `advance_roof_bucket` | 344 | 未由扫描证明；以证据限定范围为准 |
| `from_observed` | 361 | 未由扫描证明；以证据限定范围为准 |
| `ledger` | 366 | 未由扫描证明；以证据限定范围为准 |
| `pending` | 369 | 未由扫描证明；以证据限定范围为准 |
| `is_recording` | 372 | 未由扫描证明；以证据限定范围为准 |
| `start_input_recording` | 375 | 未由扫描证明；以证据限定范围为准 |
| `try_start_input_recording` | 380 | 未由扫描证明；以证据限定范围为准 |
| `add` | 383 | 未由扫描证明；以证据限定范围为准 |
| `add_with_nonce_provider` | 386 | 未由扫描证明；以证据限定范围为准 |
| `finish_input_recording` | 398 | 未由扫描证明；以证据限定范围为准 |
| `finish_with_nonce_provider` | 405 | 未由扫描证明；以证据限定范围为准 |
| `finish_potential_mutation` | 413 | 未由扫描证明；以证据限定范围为准 |
| `cursor_step` | 416 | 未由扫描证明；以证据限定范围为准 |
| `after_observed_future_clear` | 495 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L79：/// Padding and unknown presence fields remain bytes, not invented defaults.
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
### skeleton/task-index/work/TG-3846e898cf43c29d09/abc-building-35/crates/country-core/src/resources/history/mod.rs

[skeleton/task-index/work/TG-3846e898cf43c29d09/abc-building-35/crates/country-core/src/resources/history/mod.rs](../../skeleton/task-index/work/TG-3846e898cf43c29d09/abc-building-35/crates/country-core/src/resources/history/mod.rs)

SHA256：`c01c7f83ce611036a4f452c6fcfc7ae53a61b8fc96089ed3f12fb8adb09ae839`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 11 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-3846e898cf43c29d09/abc-tools-52/crates/country-core/src/resources/history/mod.rs

[skeleton/task-index/work/TG-3846e898cf43c29d09/abc-tools-52/crates/country-core/src/resources/history/mod.rs](../../skeleton/task-index/work/TG-3846e898cf43c29d09/abc-tools-52/crates/country-core/src/resources/history/mod.rs)

SHA256：`2b1aad4733aec852f44364e2aeeafad3987d410502b332be7fab99a550e51669`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_decorator_history_diff_empty` | 14 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-3c7e1129dc1230a273/abc-building-35/crates/country-core/src/resources/history/wall_compound_edit.rs

[skeleton/task-index/work/TG-3c7e1129dc1230a273/abc-building-35/crates/country-core/src/resources/history/wall_compound_edit.rs](../../skeleton/task-index/work/TG-3c7e1129dc1230a273/abc-building-35/crates/country-core/src/resources/history/wall_compound_edit.rs)

SHA256：`b55c96b78fd97dca55298e3dc8f5340a66d8531f7ed18083c93fd9a5e3e86184`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new_edit_sequence` | 12 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-d63bc50864713c7344/abc-building-35/crates/country-core/src/resources/history/replay_cmd_buffer.rs

[skeleton/task-index/work/TG-d63bc50864713c7344/abc-building-35/crates/country-core/src/resources/history/replay_cmd_buffer.rs](../../skeleton/task-index/work/TG-d63bc50864713c7344/abc-building-35/crates/country-core/src/resources/history/replay_cmd_buffer.rs)

SHA256：`29aab7d2fd9fb2a3d73da2d27a137af2d9ec90b7a4af3262c13917bf8287ad85`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `len` | 13 | 未由扫描证明；以证据限定范围为准 |
