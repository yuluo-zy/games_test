# crates/systems/wall-constructor/src

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

construct_walls/WallConstructor与noise/seed/authority walker→96B砖输出已恢复并接World。

## 接线与验证

接线：真实raw/prepared/cache→brick World链。

普通85/其他304限定入口及连续World链；冻结后新增源码不继承旧报告。

## 已恢复的函数和类型范围

- WallConstructor::from_curve
- construct_walls输入策略、proposal与96B实例输出
- noise/seed/height/开口裁切与顺序

## 明确缺口

- 所有结构family和完整原资源消费者
- 原ECS/fork排程及C迁移

下一闭包：所有结构family和完整原资源消费者

## 证据

- [evidence/building-constructor/B-wall-constructor-verification.json](../../evidence/building-constructor/B-wall-constructor-verification.json)
- [evidence/building-production-authority/findings.md](../../evidence/building-production-authority/findings.md)
- [evidence/building-integrate/stage-reviews/2026-10-11-stage4-frozen.md](../../evidence/building-integrate/stage-reviews/2026-10-11-stage4-frozen.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/wall-constructor/src/arch_walker.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_floors.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/construct_walls.rs` | A,B,C | [reconstruction/building/wall_production.rs](../../reconstruction/building/wall_production.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/init_wall_height_and_arch_segment_height.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/place_pillars.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/plaster_systems.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/stone_foundation_surface_systems.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/ui_arch_controls.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/ui_lintel_controls.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/wall-constructor/src/wall_constructor.rs` | A,B,C | [reconstruction/building/wall_constructor.rs](../../reconstruction/building/wall_constructor.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/wall-constructor/src/wooden_arch_frame_systems.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/wall_construction_sources.rs

[reconstruction/building/wall_construction_sources.rs](../../reconstruction/building/wall_construction_sources.rs)

SHA256：`0fcd77cf2f110e2c334da92e11df33e1956a08892c960374cc7ac2f488ff582c`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `construction_kind` | 18 | 未由扫描证明；以证据限定范围为准 |
| `source_seed` | 28 | 未由扫描证明；以证据限定范围为准 |
| `brick_source_id` | 36 | 未由扫描证明；以证据限定范围为准 |
| `source_max` | 48 | 未由扫描证明；以证据限定范围为准 |
| `source_min` | 51 | 未由扫描证明；以证据限定范围为准 |
| `clamp_unit` | 54 | 未由扫描证明；以证据限定范围为准 |
| `row_end_perturbation` | 59 | 未由扫描证明；以证据限定范围为准 |
| `circle_construction_options` | 78 | 未由扫描证明；以证据限定范围为准 |
| `constructor_arch_heights` | 136 | 未由扫描证明；以证据限定范围为准 |
| `construction_policy` | 152 | 未由扫描证明；以证据限定范围为准 |
| `prepared_construction_arrays` | 226 | 未由扫描证明；以证据限定范围为准 |
| `initialize_segment_height_array` | 251 | 未由扫描证明；以证据限定范围为准 |
| `plain_construction_policy` | 266 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L17：/// not create a default ClutterVersion for a raw wall whose field is unknown.
### reconstruction/building/wall_constructor.rs

[reconstruction/building/wall_constructor.rs](../../reconstruction/building/wall_constructor.rs)

SHA256：`22a44d9360049aecebf14b984811495257fdf8674dd9c1a55a164bacb7d8b320`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `observed_bytes` | 13 | 未由扫描证明；以证据限定范围为准 |
| `dims` | 24 | 未由扫描证明；以证据限定范围为准 |
| `center` | 27 | 未由扫描证明；以证据限定范围为准 |
| `from_center` | 30 | 未由扫描证明；以证据限定范围为准 |
| `contains` | 34 | 未由扫描证明；以证据限定范围为准 |
| `clamp01` | 37 | 未由扫描证明；以证据限定范围为准 |
| `resolve_hole_overlap` | 53 | 未由扫描证明；以证据限定范围为准 |
| `cut_wall_bricks` | 198 | 未由扫描证明；以证据限定范围为准 |
| `random_splits_into` | 210 | 未由扫描证明；以证据限定范围为准 |
| `split_into_random_rows` | 225 | 未由扫描证明；以证据限定范围为准 |
| `observed_bytes` | 255 | 未由扫描证明；以证据限定范围为准 |
| `trim_rows` | 268 | 未由扫描证明；以证据限定范围为准 |
| `observed_bytes` | 373 | 未由扫描证明；以证据限定范围为准 |
| `emit_resolved_wall_brick` | 411 | 未由扫描证明；以证据限定范围为准 |
| `normalize_wall_holes` | 456 | 未由扫描证明；以证据限定范围为准 |
| `intersects` | 502 | 未由扫描证明；以证据限定范围为准 |
| `consume_wall_proposal` | 509 | 未由扫描证明；以证据限定范围为准 |
| `emit_plain_wall_brick` | 577 | 未由扫描证明；以证据限定范围为准 |
| `construct_resolved_plain_wall_proposal` | 593 | 未由扫描证明；以证据限定范围为准 |
| `select_native_row_holes` | 626 | 未由扫描证明；以证据限定范围为准 |
| `split_plain_row_horizontal` | 669 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_path_preparation.rs

[reconstruction/building/wall_path_preparation.rs](../../reconstruction/building/wall_path_preparation.rs)

SHA256：`85793179f54a2a415f6a7b8007026059a685b844a43133f6ae0b00157288e417`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_encoded` | 23 | 未由扫描证明；以证据限定范围为准 |
| `encoded` | 52 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 55 | 未由扫描证明；以证据限定范围为准 |
| `sample_sum` | 58 | 未由扫描证明；以证据限定范围为准 |
| `prepare_wall_path` | 84 | 未由扫描证明；以证据限定范围为准 |
| `clamp_unit` | 148 | 未由扫描证明；以证据限定范围为准 |
| `distance` | 152 | 未由扫描证明；以证据限定范围为准 |
| `interpolate` | 155 | 未由扫描证明；以证据限定范围为准 |
| `midpoint` | 158 | 未由扫描证明；以证据限定范围为准 |
| `append` | 161 | 未由扫描证明；以证据限定范围为准 |
| `prepare_nonempty` | 167 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_production.rs

[reconstruction/building/wall_production.rs](../../reconstruction/building/wall_production.rs)

SHA256：`66b6d4551631d82329b2add6125cb0270024c4a8089800a35c636103b754996f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from` | 32 | 未由扫描证明；以证据限定范围为准 |
| `start_coordinate` | 41 | 未由扫描证明；以证据限定范围为准 |
| `advance_coordinate` | 59 | 未由扫描证明；以证据限定范围为准 |
| `sample` | 77 | 未由扫描证明；以证据限定范围为准 |
| `hash_raw` | 80 | 未由扫描证明；以证据限定范围为准 |
| `hash_phase` | 84 | 未由扫描证明；以证据限定范围为准 |
| `noise` | 88 | 未由扫描证明；以证据限定范围为准 |
| `contains_u` | 92 | 未由扫描证明；以证据限定范围为准 |
| `perturb` | 95 | 未由扫描证明；以证据限定范围为准 |
| `produce_plain_wall` | 135 | 未由扫描证明；以证据限定范围为准 |
| `produce_wall_kind` | 142 | 未由扫描证明；以证据限定范围为准 |
| `produce_wall` | 156 | 未由扫描证明；以证据限定范围为准 |
| `last_row_edges` | 499 | 未由扫描证明；以证据限定范围为准 |
| `bounded_row_edges` | 522 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/wall_bricks.rs

[reconstruction/crates/tg-building-runtime/src/wall_bricks.rs](../../reconstruction/crates/tg-building-runtime/src/wall_bricks.rs)

SHA256：`dc6617c83305f2e71481b751468738615805f99b5c2e790ecd37d9dc1b9d0846`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 44 | 未由扫描证明；以证据限定范围为准 |
| `withdraw` | 54 | 未由扫描证明；以证据限定范围为准 |
| `construct` | 67 | 未由扫描证明；以证据限定范围为准 |
