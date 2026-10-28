# crates/systems/roof/src

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

create/edit/delete/spatial/replay和类型/动画已接World；CPU铺瓦完整核心。

## 接线与验证

接线：真实Bevy建筑runtime已接验收子集；GPU/完整系统未验收。

矩形33输入19024记录/圆形37输入11467记录历史报告；编辑/生命周期/World链各自范围独立。

## 已恢复的函数和类型范围

- create_roof、edit_roof、delete_roof、update_roof_spatial
- replay_roof/五类owner
- assemble_circular_roof/assemble_rectangular_roof
- switch_roof_type和屋顶动画/移除

## 明确缺口

- roof_trigger_volume实际几何、place_spires/place_support_beams、snow与瓦片音频模拟完整消费者
- UI输入与完整视觉所有权
- ARM矩形case14一处1ULP历史失败

下一闭包：roof_trigger_volume实际几何、place_spires/place_support_beams、snow与瓦片音频模拟完整消费者

## 证据

- [evidence/roof-pipeline-verification.json](../../evidence/roof-pipeline-verification.json)
- [evidence/roof-tiles/circular-verification.json](../../evidence/roof-tiles/circular-verification.json)
- [evidence/source-edits/verification.json](../../evidence/source-edits/verification.json)
- [evidence/building-integrate/stage-reviews/2026-10-11-stage4-frozen.md](../../evidence/building-integrate/stage-reviews/2026-10-11-stage4-frozen.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/roof/src/create_roof.rs` | A,B,C | [reconstruction/building/roof_creation.rs](../../reconstruction/building/roof_creation.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/roof/src/deferred_record_roof_history.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/delete_roof.rs` | A,B,C | [reconstruction/building/roof_deletion.rs](../../reconstruction/building/roof_deletion.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/roof/src/edit_roof.rs` | A,B,C | [reconstruction/roof/edit_rules.rs](../../reconstruction/roof/edit_rules.rs)：reviewed_partial_module_mapping<br>[reconstruction/roof/edit_rules.rs](../../reconstruction/roof/edit_rules.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；4514已解析编辑后段案例，不含完整原ECS前段：[evidence/source-edits/verification.json](../../evidence/source-edits/verification.json) |
| `crates/systems/roof/src/highlight_roof.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/replay_roof.rs` | A,B,C | [reconstruction/roof/replay.rs](../../reconstruction/roof/replay.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/roof/src/roof_trigger_volume.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/simulate_roof_tile_for_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/ui_edit_roof.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/ui_edit_roof_ridge_dir.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/ui_edit_roof_ridge_length.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/ui_edit_roof_tip.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/ui_focused/ui_edit_roof_height_focused.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/ui_focused/ui_edit_roof_profile_focused.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/update_roof_spatial.rs` | A,B,C | [reconstruction/building/roof_spatial.rs](../../reconstruction/building/roof_spatial.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/roof/src/utils/assemble_circular_roof.rs` | A,B,C | [reconstruction/roof/tiles.rs](../../reconstruction/roof/tiles.rs)：reviewed_partial_module_mapping<br>[reconstruction/roof/tiles.rs](../../reconstruction/roof/tiles.rs)：reviewed_partial_module_mapping | system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>（original_located_source_candidate；业务等价以限定证据为准） | candidate_present；37输入，25prepared/12完整入口，限定CPU记录：[evidence/roof-tiles/circular-verification.json](../../evidence/roof-tiles/circular-verification.json) |
| `crates/systems/roof/src/utils/assemble_rectangular_roof.rs` | A,B,C | [reconstruction/roof/pipeline.rs](../../reconstruction/roof/pipeline.rs)：reviewed_partial_module_mapping<br>[reconstruction/roof/pipeline.rs](../../reconstruction/roof/pipeline.rs)：reviewed_partial_module_mapping | system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>（original_located_source_candidate；业务等价以限定证据为准） | candidate_present；历史Windows33完整CPU输入；ARM1ULP失败单列：[evidence/roof-pipeline-verification.json](../../evidence/roof-pipeline-verification.json) |
| `crates/systems/roof/src/utils/components.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/visual/assemble_roof_tiles.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | system_roof::visual::assemble_roof_tiles::assemble_roof_tiles（original_located_source_candidate；业务等价以限定证据为准）<br>system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0（original_located_source_candidate；业务等价以限定证据为准）<br>system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0（original_located_source_candidate；业务等价以限定证据为准） | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/visual/place_spires.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/visual/place_support_beams.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/visual/roof_animation.rs` | A,B,C | [reconstruction/building/roof_components.rs](../../reconstruction/building/roof_components.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/roof/src/visual/roof_snow.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/roof/src/visual/switch_roof_type.rs` | A,B,C | [reconstruction/crates/tg-building-runtime/src/roof_edits.rs](../../reconstruction/crates/tg-building-runtime/src/roof_edits.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/roof/src/wall_height_dt.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/roof_associations.rs

[reconstruction/building/roof_associations.rs](../../reconstruction/building/roof_associations.rs)

SHA256：`bfeb3d778ad114d6fe3960af27b6e73b7874c54cd12e881c5490e512d0d3a9fe`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `association_action` | 21 | 未由扫描证明；以证据限定范围为准 |
| `entity` | 58 | 未由扫描证明；以证据限定范围为准 |
| `has_roof` | 61 | 未由扫描证明；以证据限定范围为准 |
| `rebuild` | 66 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/roof_components.rs

[reconstruction/building/roof_components.rs](../../reconstruction/building/roof_components.rs)

SHA256：`18f23980d35689b61cf219e26e0f812f813564edf791d09b7e6ae73d4ddcf6df`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_observed` | 13 | 未由扫描证明；以证据限定范围为准 |
| `initialize` | 17 | 未由扫描证明；以证据限定范围为准 |
| `bytes` | 22 | 未由扫描证明；以证据限定范围为准 |
| `default` | 35 | 未由扫描证明；以证据限定范围为准 |
| `from_observed` | 40 | 未由扫描证明；以证据限定范围为准 |
| `bytes` | 43 | 未由扫描证明；以证据限定范围为准 |
| `is_being_animated` | 46 | 未由扫描证明；以证据限定范围为准 |
| `deletion_finished` | 49 | 未由扫描证明；以证据限定范围为准 |
| `value` | 52 | 未由扫描证明；以证据限定范围为准 |
| `advance` | 57 | 未由扫描证明；以证据限定范围为准 |
| `invalid` | 95 | 未由扫描证明；以证据限定范围为准 |
| `add` | 102 | 未由扫描证明；以证据限定范围为准 |
| `mul` | 105 | 未由扫描证明；以证据限定范围为准 |
| `prepare_resolved_roof_components` | 119 | 未由扫描证明；以证据限定范围为准 |
| `triangles` | 146 | 未由扫描证明；以证据限定范围为准 |
| `normals` | 149 | 未由扫描证明；以证据限定范围为准 |
| `uvs` | 152 | 未由扫描证明；以证据限定范围为准 |
| `set_triangles` | 155 | 未由扫描证明；以证据限定范围为准 |
| `set_normals` | 158 | 未由扫描证明；以证据限定范围为准 |
| `set_uvs` | 166 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/roof_creation.rs

[reconstruction/building/roof_creation.rs](../../reconstruction/building/roof_creation.rs)

SHA256：`46e7029d434d88044b7d669b21bcc07918cc51ad43f27285bb16fc5772c716bc`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_observed` | 15 | 未由扫描证明；以证据限定范围为准 |
| `from_replay_event` | 18 | 未由扫描证明；以证据限定范围为准 |
| `wall_id` | 26 | 未由扫描证明；以证据限定范围为准 |
| `record` | 29 | 未由扫描证明；以证据限定范围为准 |
| `initialize_resolved_roof` | 50 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/roof_customization.rs

[reconstruction/building/roof_customization.rs](../../reconstruction/building/roof_customization.rs)

SHA256：`e420282196d213a573825bc2fc4bcfc1deba406f8b27c5b79c81d0407a7a09ef`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_observed` | 9 | 未由扫描证明；以证据限定范围为准 |
| `bytes` | 12 | 未由扫描证明；以证据限定范围为准 |
| `tag` | 15 | 未由扫描证明；以证据限定范围为准 |
| `notification_eq` | 20 | 未由扫描证明；以证据限定范围为准 |
| `from_event_record` | 39 | 未由扫描证明；以证据限定范围为准 |
| `from_event_record` | 58 | 未由扫描证明；以证据限定范围为准 |
| `pivot` | 90 | 未由扫描证明；以证据限定范围为准 |
| `apply_flat_style_resolved` | 100 | 未由扫描证明；以证据限定范围为准 |
| `apply_flat_rim_resolved` | 124 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/roof_deletion.rs

[reconstruction/building/roof_deletion.rs](../../reconstruction/building/roof_deletion.rs)

SHA256：`fbe6c26fd35d6103865913ecbc620dc1b13f6880213558ae8f5dec24caf5e933`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `match_deletion_request` | 14 | 未由扫描证明；以证据限定范围为准 |
| `should_despawn` | 35 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/roof_direction.rs

[reconstruction/building/roof_direction.rs](../../reconstruction/building/roof_direction.rs)

SHA256：`08eeae209fc03b99b3a6b856834b5fda604d4046b916b0c2b76d1efceb4a25e7`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `apply_resolved_direction` | 19 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/roof_invalid_notifications.rs

[reconstruction/building/roof_invalid_notifications.rs](../../reconstruction/building/roof_invalid_notifications.rs)

SHA256：`9f22fa037a0e7d59ad4aa587dbf1b5e1003f45caf6309d958122f16c2771dce2`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `process_invalid_roof_notifications` | 24 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/roof_lifecycle.rs

[reconstruction/building/roof_lifecycle.rs](../../reconstruction/building/roof_lifecycle.rs)

SHA256：`337da38a42cca25f9db24f8c7d1b0eeb3baa432af16e4e6d4d386a009d77ced0`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `prepare` | 19 | 未由扫描证明；以证据限定范围为准 |
| `consume_delete_requests` | 33 | 未由扫描证明；以证据限定范围为准 |
| `deletion_mark_committed` | 37 | 未由扫描证明；以证据限定范围为准 |
| `advance` | 42 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/roof_spatial.rs

[reconstruction/building/roof_spatial.rs](../../reconstruction/building/roof_spatial.rs)

SHA256：`5a880fca8562ed52ecebbd6d81112da5c131ae2fe1808b222553ccbc1e41d32f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `synchronize_resolved_roof` | 30 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/roof_edits.rs

[reconstruction/crates/tg-building-runtime/src/roof_edits.rs](../../reconstruction/crates/tg-building-runtime/src/roof_edits.rs)

SHA256：`77e7fa9e74979122c056cd8344467815287522f1b0b65c6e91da5aab37fecd0c`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 52 | 未由扫描证明；以证据限定范围为准 |
| `switch_type_cpu` | 62 | 未由扫描证明；以证据限定范围为准 |
| `consume` | 136 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/roof_invalid.rs

[reconstruction/crates/tg-building-runtime/src/roof_invalid.rs](../../reconstruction/crates/tg-building-runtime/src/roof_invalid.rs)

SHA256：`212f4dd3e2a893761cc15a7d6b7c6547302ff3703d31d9e366053f962f97bc87`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 23 | 未由扫描证明；以证据限定范围为准 |
| `consume` | 32 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/roof/edit_rules.rs

[reconstruction/roof/edit_rules.rs](../../reconstruction/roof/edit_rules.rs)

SHA256：`53865ab5877240dbee44ba526fb7ca3151d621f3e0bd2c585c09691b4b3b2e5e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_observed` | 28 | 未由扫描证明；以证据限定范围为准 |
| `get` | 100 | 未由扫描证明；以证据限定范围为准 |
| `put` | 103 | 未由扫描证明；以证据限定范围为准 |
| `maxss` | 106 | 未由扫描证明；以证据限定范围为准 |
| `minss` | 109 | 未由扫描证明；以证据限定范围为准 |
| `clamp_observed` | 112 | 未由扫描证明；以证据限定范围为准 |
| `invalid_result` | 118 | 未由扫描证明；以证据限定范围为准 |
| `add_observed` | 125 | 未由扫描证明；以证据限定范围为准 |
| `sub_observed` | 128 | 未由扫描证明；以证据限定范围为准 |
| `add_tip` | 131 | 未由扫描证明；以证据限定范围为准 |
| `sub_tip` | 134 | 未由扫描证明；以证据限定范围为准 |
| `unordered_ne_zero` | 137 | 未由扫描证明；以证据限定范围为准 |
| `gable` | 140 | 未由扫描证明；以证据限定范围为准 |
| `limited_tip` | 143 | 未由扫描证明；以证据限定范围为准 |
| `pivot` | 156 | 未由扫描证明；以证据限定范围为准 |
| `apply_edit` | 163 | 未由扫描证明；以证据限定范围为准 |
| `apply_resolved_roof_edit` | 310 | 未由扫描证明；以证据限定范围为准 |
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
### reconstruction/roof/pipeline.rs

[reconstruction/roof/pipeline.rs](../../reconstruction/roof/pipeline.rs)

SHA256：`4a02dfb3cce9ac0a1076ff3b6f7d9d7bdf36e7505a50b72ffe4f21a30e409f4b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `assemble_roof_tiles_observed` | 16 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/roof/rectangular_caps.rs

[reconstruction/roof/rectangular_caps.rs](../../reconstruction/roof/rectangular_caps.rs)

SHA256：`1781aba7992c0ad596ba284f91b0a1eb94bdddc480f5a4faa0cd09c685a3e0a3`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `cap_endpoints_from_observed` | 8 | 未由扫描证明；以证据限定范围为准 |
| `assemble_caps_from_observed` | 25 | 未由扫描证明；以证据限定范围为准 |
| `assemble_ridge_caps` | 42 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/roof/rectangular_edges.rs

[reconstruction/roof/rectangular_edges.rs](../../reconstruction/roof/rectangular_edges.rs)

SHA256：`e3d89d36f50d352465e7327ce997da908a82fdc90a233b8157bf014db7c6261d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `frames_from_observed` | 9 | 未由扫描证明；以证据限定范围为准 |
| `assemble_edges_from_observed` | 17 | 未由扫描证明；以证据限定范围为准 |
| `assemble_rectangular_edges` | 22 | 未由扫描证明；以证据限定范围为准 |
| `assemble_edge` | 35 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/roof/rectangular_faces.rs

[reconstruction/roof/rectangular_faces.rs](../../reconstruction/roof/rectangular_faces.rs)

SHA256：`824405be4f280f157ee4284a1292b903dbd11a4e5935847eeba0da53baba1c26`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `plane` | 29 | 未由扫描证明；以证据限定范围为准 |
| `prepare_rectangular_faces_observed` | 40 | 未由扫描证明；以证据限定范围为准 |
| `assemble_faces_from_observed` | 106 | 未由扫描证明；以证据限定范围为准 |
| `world` | 130 | 未由扫描证明；以证据限定范围为准 |
| `cross` | 136 | 未由扫描证明；以证据限定范围为准 |
| `packed` | 139 | 未由扫描证明；以证据限定范围为准 |
| `center_world` | 156 | 未由扫描证明；以证据限定范围为准 |
| `distance3` | 160 | 未由扫描证明；以证据限定范围为准 |
| `assemble_rectangular_face_tiles` | 167 | 未由扫描证明；以证据限定范围为准 |
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
### reconstruction/roof/tiles.rs

[reconstruction/roof/tiles.rs](../../reconstruction/roof/tiles.rs)

SHA256：`0b643d6d41ffd1d4c0adf9476ba412c64e999dbe99ec6bc931e64543b249c68b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `next_f32` | 10 | 未由扫描证明；以证据限定范围为准 |
| `random_splits` | 22 | 未由扫描证明；以证据限定范围为准 |
| `circular_row_plan` | 47 | 未由扫描证明；以证据限定范围为准 |
| `circular_row_coordinates` | 61 | 未由扫描证明；以证据限定范围为准 |
| `circular_column_count` | 67 | 未由扫描证明；以证据限定范围为准 |
| `circular_boundaries` | 81 | 未由扫描证明；以证据限定范围为准 |
| `half` | 106 | 未由扫描证明；以证据限定范围为准 |
| `record` | 110 | 未由扫描证明；以证据限定范围为准 |
| `curve_coord` | 122 | 未由扫描证明；以证据限定范围为准 |
| `circle_unit` | 130 | 未由扫描证明；以证据限定范围为准 |
| `rotate` | 136 | 未由扫描证明；以证据限定范围为准 |
| `matrix_quat` | 138 | 未由扫描证明；以证据限定范围为准 |
| `assemble_circular_tiles` | 146 | 未由扫描证明；以证据限定范围为准 |
| `assemble_circular_from_observed` | 194 | 未由扫描证明；以证据限定范围为准 |
| `expanded_bounds_observed` | 220 | 未由扫描证明；以证据限定范围为准 |
| `bytes` | 236 | 未由扫描证明；以证据限定范围为准 |
| `from_bytes` | 237 | 未由扫描证明；以证据限定范围为准 |
| `axis_aligned` | 238 | 未由扫描证明；以证据限定范围为准 |
| `basis` | 239 | 未由扫描证明；以证据限定范围为准 |
| `corners` | 241 | 未由扫描证明；以证据限定范围为准 |
| `rectangular_context_from_observed` | 261 | 未由扫描证明；以证据限定范围为准 |
