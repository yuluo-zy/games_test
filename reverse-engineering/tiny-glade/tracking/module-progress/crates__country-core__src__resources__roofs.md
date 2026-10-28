# crates/country-core/src/resources/roofs

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

Roof88、参数/形状、曲面/Curve2、脊线、方向与Flat/Rim部分消费者已恢复。

## 接线与验证

接线：部分由真实Bevy建筑runtime消费。

限定原码报告：surface23810、ridge14776；参数与状态有独立对照。不是全部11文件验收。

## 已恢复的函数和类型范围

- Roof::new/ty/is_gable
- profile/截面/Curve2构造、采样、导数和切线
- RoofRidgeDims、roof_tip_offset_xz、世界点/线段脊线
- Width/Length参数映射与檐口计算

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| Roof::new | [reconstruction/roof/roof_state.rs](../../reconstruction/roof/roof_state.rs)：`new_observed` | B：256构造器输入的原字节写入；未写padding保留，不证明完整Rust ABI | reported_verified_local：[evidence/native/roof-state-verification.json](../../evidence/native/roof-state-verification.json) |
| Roof::ty | [reconstruction/roof/roof_state.rs](../../reconstruction/roof/roof_state.rs)：`ty_observed` | B：1348观察输入的布尔返回；不证明完整Roof类型 | reported_verified_local：[evidence/native/roof-state-verification.json](../../evidence/native/roof-state-verification.json) |
| Roof::is_gable | [reconstruction/roof/roof_state.rs](../../reconstruction/roof/roof_state.rs)：`is_gable_observed` | B：1348观察输入的布尔返回；NaN比较遵原指令范围 | reported_verified_local：[evidence/native/roof-state-verification.json](../../evidence/native/roof-state-verification.json) |

## 明确缺口

- roof_noise/buffers/trigger_volume/tile_sim等全部字段与消费者未闭合
- A/C布局及完整ABI单独核实
- ARM矩形case14 filler Y仍有1ULP历史失败

下一闭包：roof_noise/buffers/trigger_volume/tile_sim等全部字段与消费者未闭合

## 证据

- [evidence/roof-surface/verification.json](../../evidence/roof-surface/verification.json)
- [evidence/roof-ridge/verification.json](../../evidence/roof-ridge/verification.json)
- [evidence/multi-version/B-parameters-verification.json](../../evidence/multi-version/B-parameters-verification.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/resources/roofs/commands.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/roofs/dormer_tiles.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/roofs/roof_animation.rs` | A,B,C | [skeleton/task-index/work/TG-3b800c682fa41d3683/abc-building-35/crates/country-core/src/resources/roofs/roof_animation.rs](../../skeleton/task-index/work/TG-3b800c682fa41d3683/abc-building-35/crates/country-core/src/resources/roofs/roof_animation.rs)：explicit_candidate_delivery_mapping | country_core::resources::roofs::roof_animation::RoofAnimation::new（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/roofs/roof_buffers.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/roofs/roof_noise.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/roofs/roof_ridge.rs` | A,B,C | [reconstruction/roof/ridge.rs](../../reconstruction/roof/ridge.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-e0787bc1be6b98fce2/abc-tools-52/crates/country-core/src/resources/roofs/roof_ridge.rs](../../skeleton/task-index/work/TG-e0787bc1be6b98fce2/abc-tools-52/crates/country-core/src/resources/roofs/roof_ridge.rs)：explicit_candidate_delivery_mapping | country_core::resources::roofs::roof_ridge::RoofRidgeDims::get_inner（implemented_unverified；业务等价以限定证据为准） | candidate_present；仅列出的状态/曲面/参数/脊线局部范围；非整个原文件：[evidence/roof-surface/verification.json](../../evidence/roof-surface/verification.json), [evidence/roof-ridge/verification.json](../../evidence/roof-ridge/verification.json), [evidence/multi-version/B-parameters-verification.json](../../evidence/multi-version/B-parameters-verification.json) |
| `crates/country-core/src/resources/roofs/roof_shape.rs` | A,B,C | [reconstruction/roof/roof_state.rs](../../reconstruction/roof/roof_state.rs)：reviewed_partial_module_mapping<br>[reconstruction/roof/surface.rs](../../reconstruction/roof/surface.rs)：reviewed_partial_module_mapping<br>[reconstruction/roof/parameters.rs](../../reconstruction/roof/parameters.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-4df0b3511f93a7ad7a/abc-tools-52/crates/country-core/src/resources/roofs/roof_shape.rs](../../skeleton/task-index/work/TG-4df0b3511f93a7ad7a/abc-tools-52/crates/country-core/src/resources/roofs/roof_shape.rs)：explicit_candidate_delivery_mapping | country_core::resources::roofs::roof_shape::GableRoofStyle::is_overhang（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::roofs::roof_shape::Roof::gable_sides（original_located_source_candidate；业务等价以限定证据为准）<br>country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz（original_located_source_candidate；业务等价以限定证据为准）<br>Roof::new（reported_verified_local；业务等价以限定证据为准）<br>Roof::ty（reported_verified_local；业务等价以限定证据为准）<br>Roof::is_gable（reported_verified_local；业务等价以限定证据为准） | candidate_present；仅列出的状态/曲面/参数/脊线局部范围；非整个原文件：[evidence/roof-surface/verification.json](../../evidence/roof-surface/verification.json), [evidence/roof-ridge/verification.json](../../evidence/roof-ridge/verification.json), [evidence/multi-version/B-parameters-verification.json](../../evidence/multi-version/B-parameters-verification.json)<br>仅列出的状态/曲面/参数/脊线局部范围；非整个原文件：[evidence/roof-surface/verification.json](../../evidence/roof-surface/verification.json), [evidence/roof-ridge/verification.json](../../evidence/roof-ridge/verification.json), [evidence/multi-version/B-parameters-verification.json](../../evidence/multi-version/B-parameters-verification.json)<br>仅列出的状态/曲面/参数/脊线局部范围；非整个原文件：[evidence/roof-surface/verification.json](../../evidence/roof-surface/verification.json), [evidence/roof-ridge/verification.json](../../evidence/roof-ridge/verification.json), [evidence/multi-version/B-parameters-verification.json](../../evidence/multi-version/B-parameters-verification.json)<br>Roof::new：256构造器输入的原字节写入；未写padding保留，不证明完整Rust ABI：[evidence/native/roof-state-verification.json](../../evidence/native/roof-state-verification.json)<br>Roof::ty：1348观察输入的布尔返回；不证明完整Roof类型：[evidence/native/roof-state-verification.json](../../evidence/native/roof-state-verification.json)<br>Roof::is_gable：1348观察输入的布尔返回；NaN比较遵原指令范围：[evidence/native/roof-state-verification.json](../../evidence/native/roof-state-verification.json) |
| `crates/country-core/src/resources/roofs/roof_tile_sim.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/roofs/roof_tiles.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/roofs/roof_trigger_volume.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/roofs/roof_visual_shape.rs` | A,B,C | [reconstruction/roof/surface.rs](../../reconstruction/roof/surface.rs)：reviewed_partial_module_mapping | country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom（original_located_source_candidate；业务等价以限定证据为准）<br>country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_circle（original_located_source_candidate；业务等价以限定证据为准） | candidate_present；仅列出的状态/曲面/参数/脊线局部范围；非整个原文件：[evidence/roof-surface/verification.json](../../evidence/roof-surface/verification.json), [evidence/roof-ridge/verification.json](../../evidence/roof-ridge/verification.json), [evidence/multi-version/B-parameters-verification.json](../../evidence/multi-version/B-parameters-verification.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/roof/parameters.rs

[reconstruction/roof/parameters.rs](../../reconstruction/roof/parameters.rs)

SHA256：`61bc696ffd9dc30eacdfd2739e5edff574c6db22b8ebb6356017d6d0eb371fcf`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_bytes` | 10 | 未由扫描证明；以证据限定范围为准 |
| `bytes` | 13 | 未由扫描证明；以证据限定范围为准 |
| `from_roof` | 16 | 未由扫描证明；以证据限定范围为准 |
| `write_to_roof` | 19 | 未由扫描证明；以证据限定范围为准 |
| `replace_serialized_fields` | 25 | 未由扫描证明；以证据限定范围为准 |
| `direction` | 42 | 未由扫描证明；以证据限定范围为准 |
| `profile_01` | 50 | 未由扫描证明；以证据限定范围为准 |
| `height_01` | 53 | 未由扫描证明；以证据限定范围为准 |
| `eave_length_01` | 56 | 未由扫描证明；以证据限定范围为准 |
| `float_at` | 59 | 未由扫描证明；以证据限定范围为准 |
| `eave_length_ws` | 65 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/roof/ridge.rs

[reconstruction/roof/ridge.rs](../../reconstruction/roof/ridge.rs)

SHA256：`cb3b6deb1396253c539b1ab789e04308c23a9c8933df6d890d9ae99ff08a13ad`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `at` | 22 | 未由扫描证明；以证据限定范围为准 |
| `tag` | 25 | 未由扫描证明；以证据限定范围为准 |
| `max_num` | 28 | 未由扫描证明；以证据限定范围为准 |
| `maxss` | 38 | 未由扫描证明；以证据限定范围为准 |
| `minss` | 41 | 未由扫描证明；以证据限定范围为准 |
| `neg_bits` | 44 | 未由扫描证明；以证据限定范围为准 |
| `rotation` | 47 | 未由扫描证明；以证据限定范围为准 |
| `ridge_dims_calc` | 55 | 未由扫描证明；以证据限定范围为准 |
| `ridge_dims_from_observed` | 63 | 未由扫描证明；以证据限定范围为准 |
| `calc_roof_tip_center_ls_observed` | 71 | 未由扫描证明；以证据限定范围为准 |
| `roof_tip_offset_xz_observed` | 82 | 未由扫描证明；以证据限定范围为准 |
| `inverse_ridge_01_param` | 94 | 未由扫描证明；以证据限定范围为准 |
| `roof_height` | 111 | 未由扫描证明；以证据限定范围为准 |
| `ridge_to_world_space_observed` | 140 | 未由扫描证明；以证据限定范围为准 |
| `decode` | 194 | 未由扫描证明；以证据限定范围为准 |
| `ff` | 198 | 未由扫描证明；以证据限定范围为准 |
| `bits` | 202 | 未由扫描证明；以证据限定范围为准 |
| `main` | 209 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L242：_ => panic!("unknown mode"),
### reconstruction/roof/roof_state.rs

[reconstruction/roof/roof_state.rs](../../reconstruction/roof/roof_state.rs)

SHA256：`68028044f413515724fd179b00326422c7e13cbbc942aecaef0b3f9ce82783df`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `ty_observed` | 5 | 未由扫描证明；以证据限定范围为准 |
| `is_gable_observed` | 11 | 未由扫描证明；以证据限定范围为准 |
| `new_observed` | 16 | 未由扫描证明；以证据限定范围为准 |
| `decode` | 30 | 未由扫描证明；以证据限定范围为准 |
| `main` | 36 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L51：_ => panic!("unknown test mode"),
### reconstruction/roof/surface.rs

[reconstruction/roof/surface.rs](../../reconstruction/roof/surface.rs)

SHA256：`132b17182082410908cae6a0d897638eeebb0e8095183d124767ec886428bbcd`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `f` | 7 | 未由扫描证明；以证据限定范围为准 |
| `put` | 8 | 未由扫描证明；以证据限定范围为准 |
| `write_observed` | 19 | 未由扫描证明；以证据限定范围为准 |
| `section` | 33 | 未由扫描证明；以证据限定范围为准 |
| `profile_exponent` | 40 | 未由扫描证明；以证据限定范围为准 |
| `get_roof_shape` | 46 | 未由扫描证明；以证据限定范围为准 |
| `eave_length_ws` | 76 | 未由扫描证明；以证据限定范围为准 |
| `sse_max` | 81 | 未由扫描证明；以证据限定范围为准 |
| `sse_min` | 82 | 未由扫描证明；以证据限定范围为准 |
| `height` | 86 | 未由扫描证明；以证据限定范围为准 |
| `profile_curve_normalized` | 105 | 未由扫描证明；以证据限定范围为准 |
| `profile_curve_ws_points` | 116 | 未由扫描证明；以证据限定范围为准 |
| `try_new` | 128 | 未由扫描证明；以证据限定范围为准 |
| `coord_at_u` | 148 | 未由扫描证明；以证据限定范围为准 |
| `pos_at_u` | 158 | 未由扫描证明；以证据限定范围为准 |
| `tangent_at_u` | 162 | 未由扫描证明；以证据限定范围为准 |
| `normalized_profile_curve` | 168 | 未由扫描证明；以证据限定范围为准 |
| `world_profile_curve` | 171 | 未由扫描证明；以证据限定范围为准 |
| `main` | 177 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L190：_=>panic!("unknown operation")
### skeleton/task-index/work/TG-3b800c682fa41d3683/abc-building-35/crates/country-core/src/resources/roofs/roof_animation.rs

[skeleton/task-index/work/TG-3b800c682fa41d3683/abc-building-35/crates/country-core/src/resources/roofs/roof_animation.rs](../../skeleton/task-index/work/TG-3b800c682fa41d3683/abc-building-35/crates/country-core/src/resources/roofs/roof_animation.rs)

SHA256：`5a197d8cb8995bd938d4b6e1146318656864131f6b9d1ab30d7895dc4b2393d4`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 14 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-4df0b3511f93a7ad7a/abc-tools-52/crates/country-core/src/resources/roofs/roof_shape.rs

[skeleton/task-index/work/TG-4df0b3511f93a7ad7a/abc-tools-52/crates/country-core/src/resources/roofs/roof_shape.rs](../../skeleton/task-index/work/TG-4df0b3511f93a7ad7a/abc-tools-52/crates/country-core/src/resources/roofs/roof_shape.rs)

SHA256：`d53dc0f34bcb9d4cf0fc02c0370909bd6654d2c8a3f1434ec382a6c0d1c8c379`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_overhang` | 14 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-e0787bc1be6b98fce2/abc-tools-52/crates/country-core/src/resources/roofs/roof_ridge.rs

[skeleton/task-index/work/TG-e0787bc1be6b98fce2/abc-tools-52/crates/country-core/src/resources/roofs/roof_ridge.rs](../../skeleton/task-index/work/TG-e0787bc1be6b98fce2/abc-tools-52/crates/country-core/src/resources/roofs/roof_ridge.rs)

SHA256：`c34495d6a660cd88960c385f1a04a406f1ead47200450b22d8a7e04d3e5ad7c0`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `get_inner` | 15 | 未由扫描证明；以证据限定范围为准 |
