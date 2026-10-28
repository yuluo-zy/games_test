# crates/country-core/src/systems/wall

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

raw Curve2→terrain/raster/prepared→cache/publication→Roof/openings/brick消费者；创建、高度、删除/bulldoze及部分矩形编辑。

## 接线与验证

接线：已验收链为真实Bevy World；新增链implemented_unverified。

阶段4冻结54测试/67限定证明；新增并行批不继承冻结验收。

## 已恢复的函数和类型范围

- publish_public_wall_state/采集与dirty更新
- circle/rectangle/freeform缓存和逐arch gable
- AdjustWallHeightCmd读取、clamp、authority/audio
- 创建/直接删除/animate_wall_bulldoze
- 矩形finish/cancel/enclosure及PendingWallChanged候选

## 明确缺口

- 完整UI placement/startup、snapping/merge/splitter未恢复
- 新矩形/Enclosure等整批尚未统一验证
- 原fork排程与跨writer顺序

下一闭包：完整UI placement/startup、snapping/merge/splitter未恢复

## 证据

- [evidence/building-integrate/stage-reviews/2026-10-11-stage4-frozen.md](../../evidence/building-integrate/stage-reviews/2026-10-11-stage4-frozen.md)
- [evidence/building-integrate/stage-reviews/stage4-frozen/summary.json](../../evidence/building-integrate/stage-reviews/stage4-frozen/summary.json)
- [evidence/building-wall-history/findings.md](../../evidence/building-wall-history/findings.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_height_impl.rs` | A,B,C | [reconstruction/building/wall_height_command.rs](../../reconstruction/building/wall_height_command.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/adjust_wall_height/signifier_animation.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height_mode.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/calc_wall_bend.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/deferred_wall_history.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/delete_wall.rs` | A,B,C | [reconstruction/building/wall_authoring.rs](../../reconstruction/building/wall_authoring.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/draw_wall.rs` | A,B,C | [reconstruction/building/wall_authoring.rs](../../reconstruction/building/wall_authoring.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs` | A,B,C | [reconstruction/building/wall_bulldoze.rs](../../reconstruction/building/wall_bulldoze.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/eraser/archivist.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/eraser/bulldoze_params.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/eraser/delete_focused.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/eraser/ui_bulldozer.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/eraser/wall_eraser.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/finish_wall.rs` | A,B,C | [reconstruction/building/wall_authoring.rs](../../reconstruction/building/wall_authoring.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/merge_walls.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/set_wall_style.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/calc_shape_height.rs` | A,B,C | [reconstruction/building/wall_freeform_cache.rs](../../reconstruction/building/wall_freeform_cache.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/circle/create_circle.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/circle/signifiers/edit_arrow.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/circle/signifiers/edit_line.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/circle/ui_create_circle.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/circle/ui_edit_circle.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/circle/ui_rotate_circle.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/rectangle/create_rectangle.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/rectangle/edit_rectangle.rs` | A,B,C | [reconstruction/building/wall_rectangle_edit.rs](../../reconstruction/building/wall_rectangle_edit.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/rectangle/finish_edit_rectangle.rs` | A,B,C | [reconstruction/building/wall_rectangle_edit.rs](../../reconstruction/building/wall_rectangle_edit.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/rectangle/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/rectangle/signifier_edit_dims_line.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/rectangle/ui_create_rectangle.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/rectangle/ui_edit_rectangle_orientation.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/shape_placement.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/shape_placement_signifiers.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/shapes/utils/shape_signifier_lerp.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/snapping/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/snapping/snapping_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/snapping/snapping_init.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/snapping/snapping_maintain.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/snapping/snapping_mantain.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/snapping/snapping_to_ends.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/snapping/snapping_to_segments.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/snapping/telegraph_walls_are_merged.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/structure_outline.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/ui_continue_wall.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/ui_draw_wall.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/upload_instanced_wall_ssbo.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/valid_enclosure/mod.rs` | A,B,C | [skeleton/task-index/work/TG-f2c3841f004fb1ad05/abc-building-35/crates/country-core/src/systems/wall/valid_enclosure/mod.rs](../../skeleton/task-index/work/TG-f2c3841f004fb1ad05/abc-building-35/crates/country-core/src/systems/wall/valid_enclosure/mod.rs)：explicit_candidate_delivery_mapping | country_core::systems::wall::valid_enclosure::ShapeParameters::center（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/wall_highlight.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/wall_outline.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/wall_splitter.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/wall_state/debug_viz_threshold_curve.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/wall_state/publish_public_wall_state.rs` | A,B,C | [reconstruction/building/wall_publication.rs](../../reconstruction/building/wall_publication.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/wall_state/spatial_hash_debug.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wall/wall_state/spatial_hash_update.rs` | A,B,C | [reconstruction/building/wall_spatial_index.rs](../../reconstruction/building/wall_spatial_index.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/wall_state/wall_roof_hashmap.rs` | A,B,C | [reconstruction/building/roof_associations.rs](../../reconstruction/building/roof_associations.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs` | A,B,C | [reconstruction/building/wall_spatial_index.rs](../../reconstruction/building/wall_spatial_index.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-56afc78f73b7ab7966/abc-building-35/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs](../../skeleton/task-index/work/TG-56afc78f73b7ab7966/abc-building-35/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-56afc78f73b7ab7966/abc-building-35/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs](../../skeleton/task-index/work/TG-56afc78f73b7ab7966/abc-building-35/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs)：explicit_candidate_delivery_mapping | country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index（implemented_unverified；业务等价以限定证据为准）<br>country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |

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
### reconstruction/building/wall_authoring.rs

[reconstruction/building/wall_authoring.rs](../../reconstruction/building/wall_authoring.rs)

SHA256：`2043afa2ba091e57d9368a5b2b88acc0803b510fd2d2ea8671e95b9714c7c2bb`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `select_creation_appearance` | 54 | 未由扫描证明；以证据限定范围为准 |
| `source_max` | 81 | 未由扫描证明；以证据限定范围为准 |
| `source_min` | 82 | 未由扫描证明；以证据限定范围为准 |
| `clamp01` | 83 | 未由扫描证明；以证据限定范围为准 |
| `smoothstep` | 84 | 未由扫描证明；以证据限定范围为准 |
| `sine_ease_in_out` | 85 | 未由扫描证明；以证据限定范围为准 |
| `creation_roof_parameters` | 92 | 未由扫描证明；以证据限定范围为准 |
| `edit_rectangle_source` | 174 | 未由扫描证明；以证据限定范围为准 |
| `create_building_from_placement` | 217 | 未由扫描证明；以证据限定范围为准 |
| `shape_snapshot` | 244 | 未由扫描证明；以证据限定范围为准 |
| `create_shape_source` | 262 | 未由扫描证明；以证据限定范围为准 |
| `request_from_shape_placement` | 295 | 未由扫描证明；以证据限定范围为准 |
| `active_create_event_bytes` | 312 | 未由扫描证明；以证据限定范围为准 |
| `mutate_observed_source` | 333 | 未由扫描证明；以证据限定范围为准 |
| `remove_source` | 353 | 未由扫描证明；以证据限定范围为准 |
| `clear_preview_for_inactive_mode` | 366 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_bulldoze.rs

[reconstruction/building/wall_bulldoze.rs](../../reconstruction/building/wall_bulldoze.rs)

SHA256：`09d218a1a95f753d611ac6964c9f227fc39613740820e7a21180f80875e2f72b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `record_wall_bulldoze` | 14 | 未由扫描证明；以证据限定范围为准 |
| `add` | 52 | 未由扫描证明；以证据限定范围为准 |
| `ease` | 53 | 未由扫描证明；以证据限定范围为准 |
| `advance` | 57 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_cache.rs

[reconstruction/building/wall_cache.rs](../../reconstruction/building/wall_cache.rs)

SHA256：`6bcaec9c4b112cee4ce126f7d30e233aa4b95317a4f5bf0969dcf2adfbe4b86e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `regular_wall_height_cache` | 18 | 未由扫描证明；以证据限定范围为准 |
| `add` | 46 | 未由扫描证明；以证据限定范围为准 |
| `write_projection` | 56 | 未由扫描证明；以证据限定范围为准 |
| `gable_maximum` | 67 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_collection.rs

[reconstruction/building/wall_collection.rs](../../reconstruction/building/wall_collection.rs)

SHA256：`b74c6aa577a9c012d4d34fcfb1851a8d33a84ee154872ece077d66e96ab198b6`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `sample_at_world_xz` | 21 | 未由扫描证明；以证据限定范围为准 |
| `collect_raw_wall_publication` | 114 | 未由扫描证明；以证据限定范围为准 |
| `terrain_requires_republication` | 201 | 未由扫描证明；以证据限定范围为准 |
| `dirty_tile_candidate` | 224 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L91：/// Original source flag copied by publication; its game name is unknown.
### reconstruction/building/wall_freeform_cache.rs

[reconstruction/building/wall_freeform_cache.rs](../../reconstruction/building/wall_freeform_cache.rs)

SHA256：`7d2a2870be3cfd73e4318ce4fb3d240b17f59ac2f3738d5504d11dc7c6b637a6`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `add` | 23 | 未由扫描证明；以证据限定范围为准 |
| `numeric_max` | 32 | 未由扫描证明；以证据限定范围为准 |
| `freeform_wall_height_cache` | 45 | 未由扫描证明；以证据限定范围为准 |
| `write_projection` | 86 | 未由扫描证明；以证据限定范围为准 |
| `float_at` | 143 | 未由扫描证明；以证据限定范围为准 |
| `write_float` | 146 | 未由扫描证明；以证据限定范围为准 |
| `source_max` | 149 | 未由扫描证明；以证据限定范围为准 |
| `source_min` | 152 | 未由扫描证明；以证据限定范围为准 |
| `rectangle_curve` | 156 | 未由扫描证明；以证据限定范围为准 |
| `circle_curve` | 175 | 未由扫描证明；以证据限定范围为准 |
| `original_cos_call` | 205 | 未由扫描证明；以证据限定范围为准 |
| `original_sin_call` | 209 | 未由扫描证明；以证据限定范围为准 |
| `preview_terrain_map` | 213 | 未由扫描证明；以证据限定范围为准 |
| `calculate_shape_height` | 269 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_gable.rs

[reconstruction/building/wall_gable.rs](../../reconstruction/building/wall_gable.rs)

SHA256：`4233b8d22a3055c413085fd0711cb312fee2a793caf84b30d4214d4c53bbd35f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `f` | 23 | 未由扫描证明；以证据限定范围为准 |
| `rectangle_point_u` | 27 | 未由扫描证明；以证据限定范围为准 |
| `max_source` | 44 | 未由扫描证明；以证据限定范围为准 |
| `min_source` | 47 | 未由扫描证明；以证据限定范围为准 |
| `apply_gable_arch_heights` | 54 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_height_command.rs

[reconstruction/building/wall_height_command.rs](../../reconstruction/building/wall_height_command.rs)

SHA256：`ca1b270bc785153720152a4a9d9e9ac4098615df0be991eacd7e9bfec2bd3e24`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_observed` | 22 | 未由扫描证明；以证据限定范围为准 |
| `native_divide` | 76 | 未由扫描证明；以证据限定范围为准 |
| `apply_resolved_height_command` | 85 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_hole_invalidation.rs

[reconstruction/building/wall_hole_invalidation.rs](../../reconstruction/building/wall_hole_invalidation.rs)

SHA256：`891d9f4c0194ad281f03da9334af7063136642b558520b633b5063b668b47531`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `forward_recalculation` | 4 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_pending_changed.rs

[reconstruction/building/wall_pending_changed.rs](../../reconstruction/building/wall_pending_changed.rs)

SHA256：`8db5e98df88e7cf6de945e4cb30f6ff1b9a8f5885431b7053f8d0b5fbe220cef`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `approximate_curve_coordinate` | 11 | 未由扫描证明；以证据限定范围为准 |
| `approximate_curve_u` | 25 | 未由扫描证明；以证据限定范围为准 |
| `try_pending_wall_changed` | 31 | 未由扫描证明；以证据限定范围为准 |
| `take_pending_wall_changed` | 42 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_publication.rs

[reconstruction/building/wall_publication.rs](../../reconstruction/building/wall_publication.rs)

SHA256：`681a44c56dfa67c0ee6380e84e77621ddc88d057909ed51a08f184302db4d3ab`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `min_fold` | 13 | 未由扫描证明；以证据限定范围为准 |
| `max_fold` | 16 | 未由扫描证明；以证据限定范围为准 |
| `publication_metrics` | 19 | 未由扫描证明；以证据限定范围为准 |
| `write_projection` | 56 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_rectangle_edit.rs

[reconstruction/building/wall_rectangle_edit.rs](../../reconstruction/building/wall_rectangle_edit.rs)

SHA256：`9b9858c29e1d9b105762c87040c9b7d6291c99056d470e0e1cd28d396df5d138`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `tag` | 24 | 未由扫描证明；以证据限定范围为准 |
| `wall` | 25 | 未由扫描证明；以证据限定范围为准 |
| `finish_rectangle_edit_decision` | 31 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_spatial_index.rs

[reconstruction/building/wall_spatial_index.rs](../../reconstruction/building/wall_spatial_index.rs)

SHA256：`91a3a4a6686b358a8e12dc40a642a696ad9ad4e142f55eb4c1efaa3c8a863f1a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `world_space_to_grid_coord_unclamped` | 10 | 未由扫描证明；以证据限定范围为准 |
| `grid_coord_to_cell_index` | 13 | 未由扫描证明；以证据限定范围为准 |
| `inside` | 16 | 未由扫描证明；以证据限定范围为准 |
| `segment_cells` | 22 | 未由扫描证明；以证据限定范围为准 |
| `default` | 89 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 97 | 未由扫描证明；以证据限定范围为准 |
| `replace_curve` | 106 | 未由扫描证明；以证据限定范围为准 |
| `wall_cells` | 120 | 未由扫描证明；以证据限定范围为准 |
| `candidates` | 125 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/wall_authoring.rs

[reconstruction/crates/tg-building-runtime/src/wall_authoring.rs](../../reconstruction/crates/tg-building-runtime/src/wall_authoring.rs)

SHA256：`37f9a566e6e9f67ca41c91835b150f041f7f48417d7bc41b83c65f71c39f2f23`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 57 | 未由扫描证明；以证据限定范围为准 |
| `cleanup_removed` | 79 | 未由扫描证明；以证据限定范围为准 |
| `consume` | 86 | 未由扫描证明；以证据限定范围为准 |
| `edit_rectangles` | 250 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/wall_bulldoze.rs

[reconstruction/crates/tg-building-runtime/src/wall_bulldoze.rs](../../reconstruction/crates/tg-building-runtime/src/wall_bulldoze.rs)

SHA256：`9f005a1afe400f39b428250d6c7bd69342ccc2da788704b1fe55922a4e21c9af`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 33 | 未由扫描证明；以证据限定范围为准 |
| `consume_and_animate` | 38 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/wall_height.rs

[reconstruction/crates/tg-building-runtime/src/wall_height.rs](../../reconstruction/crates/tg-building-runtime/src/wall_height.rs)

SHA256：`1c29dffdb0755c3b3291a03a4a963e48c80b1b604efc75adea28c66758cc127e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 36 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/wall_pending_changed.rs

[reconstruction/crates/tg-building-runtime/src/wall_pending_changed.rs](../../reconstruction/crates/tg-building-runtime/src/wall_pending_changed.rs)

SHA256：`f764894352431979b583134a40a8be7b2c047d2c20e81c41a744297c12cf8c01`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 16 | 未由扫描证明；以证据限定范围为准 |
| `consume` | 17 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/wall_rectangle_edit.rs

[reconstruction/crates/tg-building-runtime/src/wall_rectangle_edit.rs](../../reconstruction/crates/tg-building-runtime/src/wall_rectangle_edit.rs)

SHA256：`4ff75f33c0b3a52eaf484c68e7652ceba613dca39816aec330596a5f9e0cb1d8`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 27 | 未由扫描证明；以证据限定范围为准 |
| `consume_and_finish` | 31 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-56afc78f73b7ab7966/abc-building-35/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs

[skeleton/task-index/work/TG-56afc78f73b7ab7966/abc-building-35/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs](../../skeleton/task-index/work/TG-56afc78f73b7ab7966/abc-building-35/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs)

SHA256：`b4f7c7917ae832ebbc12f15832281b02b0ada2b5361cf87cc2901c47caf88d91`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `cell_size` | 16 | 未由扫描证明；以证据限定范围为准 |
| `grid_coord_to_cell_index` | 26 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-f2c3841f004fb1ad05/abc-building-35/crates/country-core/src/systems/wall/valid_enclosure/mod.rs

[skeleton/task-index/work/TG-f2c3841f004fb1ad05/abc-building-35/crates/country-core/src/systems/wall/valid_enclosure/mod.rs](../../skeleton/task-index/work/TG-f2c3841f004fb1ad05/abc-building-35/crates/country-core/src/systems/wall/valid_enclosure/mod.rs)

SHA256：`01dc4b5ceaf57d0bdd58d0381487334fdf37c8d78a8868a419946616969e0d4e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `center` | 15 | 未由扫描证明；以证据限定范围为准 |
