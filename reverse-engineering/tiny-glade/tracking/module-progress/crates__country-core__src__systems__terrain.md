# crates/country-core/src/systems/terrain

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

Stroke schema、点列/ID、已解析编辑与Edit回放；重算/读回事件链已分析。

## 接线与验证

接线：not_established。

M1报告Edit512原作↔Rust；Delta512是早期原片段↔公式，不能升格完整Rust入口。

## 已恢复的函数和类型范围

- stroke点列维护和ID上限
- Edit Delta/Absolute/Keep规则
- Edit正反replay
- delayed_terrain_changed_msg/readback聚合通知

## 明确缺口

- New/Move全入口与UI、pending history、真实ECS
- 非有限边界、GPU rasterize/readback同步、dirty消费者全链

下一闭包：New/Move全入口与UI、pending history、真实ECS

## 证据

- [evidence/terrain-editing/milestones/M1/status.md](../../evidence/terrain-editing/milestones/M1/status.md)
- [evidence/terrain-editing/second-batch-findings.md](../../evidence/terrain-editing/second-batch-findings.md)
- [evidence/terrain-editing/B-delta-edit-verification.json](../../evidence/terrain-editing/B-delta-edit-verification.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/systems/terrain/animate_terrain_delete.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/terrain/deferred_terrain_history.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/terrain/delayed_terrain_changed_msg.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/terrain/draw_terrain_stroke.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/terrain/edit_terrain_stroke.rs` | A,B,C | [reconstruction/terrain/crates/tg-terrain/src/edit.rs](../../reconstruction/terrain/crates/tg-terrain/src/edit.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/terrain/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/terrain/terrain_hover_idx.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/terrain/ui_draw_terrain.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/terrain/ui_edit_terrain.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/terrain/ui_terrain_focus_mode.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/terrain/crates/tg-terrain/src/edit.rs

[reconstruction/terrain/crates/tg-terrain/src/edit.rs](../../reconstruction/terrain/crates/tg-terrain/src/edit.rs)

SHA256：`d904978d89b2d1268ff6947a8470769d76ca02869ab4ecc34a17108179d6cc29`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `source_max` | 39 | 未由扫描证明；以证据限定范围为准 |
| `source_min` | 42 | 未由扫描证明；以证据限定范围为准 |
| `apply_resolved` | 48 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/terrain/crates/tg-terrain/src/readback.rs

[reconstruction/terrain/crates/tg-terrain/src/readback.rs](../../reconstruction/terrain/crates/tg-terrain/src/readback.rs)

SHA256：`ecd808d3467b9c4e5473274b0fe577dbf9a3beeaed8869bb18b93304500ee4d5`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 21 | 未由扫描证明；以证据限定范围为准 |
| `copy_from_gpu` | 29 | 未由扫描证明；以证据限定范围为准 |
| `cells` | 53 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/terrain/crates/tg-terrain/src/replay.rs

[reconstruction/terrain/crates/tg-terrain/src/replay.rs](../../reconstruction/terrain/crates/tg-terrain/src/replay.rs)

SHA256：`8c40fb236f67e5b1b52c4a2fd2c8a545009752d50f10085393b12152176c34cc`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `command` | 18 | 未由扫描证明；以证据限定范围为准 |
| `observe_edit` | 29 | 未由扫描证明；以证据限定范围为准 |
| `edit_command` | 46 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/terrain/crates/tg-terrain/src/schema.rs

[reconstruction/terrain/crates/tg-terrain/src/schema.rs](../../reconstruction/terrain/crates/tg-terrain/src/schema.rs)

SHA256：`dfc33b570017d6f6a65a093cd50746d80242f046e66c2219e10114e2c206ce27`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/terrain/crates/tg-terrain/src/stroke.rs

[reconstruction/terrain/crates/tg-terrain/src/stroke.rs](../../reconstruction/terrain/crates/tg-terrain/src/stroke.rs)

SHA256：`1f2fc447052acd866e4ed743fb72170a55987d383ca175786a764006c38e5994`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `distance` | 20 | 未由扫描证明；以证据限定范围为准 |
| `insert_or_morph_last_point` | 25 | 未由扫描证明；以证据限定范围为准 |
| `new_stroke` | 57 | 未由扫描证明；以证据限定范围为准 |
| `get_last_mut_stroke` | 65 | 未由扫描证明；以证据限定范围为准 |
