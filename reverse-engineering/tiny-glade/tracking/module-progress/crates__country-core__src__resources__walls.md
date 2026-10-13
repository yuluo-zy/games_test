# crates/country-core/src/resources/walls

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

Public/InnerWall投影、开口存储、subtype、装饰字段与部分附着候选。

## 接线与验证

接线：已验收子集接真实World，新增字段另列未验证。

高度、hole、subtype有原码局部证明；Info和部分storage为新增候选，不能整体通过。

## 已恢复的函数和类型范围

- PublicWalls::get合法性读取
- max_y/max_y_gable/flat_roof_y
- WallHoleStorage周期副本/finalize/cache
- derive_decorator_subtype和Info72字段投影

## 明确缺口

- 未知published字段及PublicWall+296生产者
- 全部资源字段/布局和wall_attachment跨版ABI
- 新Info/storage绑定未统一验收

下一闭包：未知published字段及PublicWall+296生产者

## 证据

- [evidence/scope-priorities/wall-rules-verification.json](../../evidence/scope-priorities/wall-rules-verification.json)
- [evidence/building-holes/storage-B-verification.json](../../evidence/building-holes/storage-B-verification.json)
- [evidence/building-placement/B-subtype-verification.json](../../evidence/building-placement/B-subtype-verification.json)
- [evidence/building-decorator-info/findings.md](../../evidence/building-decorator-info/findings.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/resources/walls/arches.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/cached_decorator_transforms.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/curved_halftimber.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/decorator.rs` | A,B,C | [reconstruction/building/decorator_info.rs](../../reconstruction/building/decorator_info.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-43142338addb95ca2d/abc-tools-52/crates/country-core/src/resources/walls/decorator.rs](../../skeleton/task-index/work/TG-43142338addb95ca2d/abc-tools-52/crates/country-core/src/resources/walls/decorator.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-43142338addb95ca2d/abc-building-35/crates/country-core/src/resources/walls/decorator.rs](../../skeleton/task-index/work/TG-43142338addb95ca2d/abc-building-35/crates/country-core/src/resources/walls/decorator.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-43142338addb95ca2d/abc-building-35/crates/country-core/src/resources/walls/decorator.rs](../../skeleton/task-index/work/TG-43142338addb95ca2d/abc-building-35/crates/country-core/src/resources/walls/decorator.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-43142338addb95ca2d/abc-building-35/crates/country-core/src/resources/walls/decorator.rs](../../skeleton/task-index/work/TG-43142338addb95ca2d/abc-building-35/crates/country-core/src/resources/walls/decorator.rs)：explicit_candidate_delivery_mapping | country_core::resources::walls::decorator::ClutterVersion::is_initial（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::walls::decorator::TransientDecoratorIdGen::new（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::walls::decorator::DecoratorId::new（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::walls::decorator::DecoratorRank::new（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/walls/decorator_archivist.rs` | A,B,C | [reconstruction/building/wall_decorator_archivist.rs](../../reconstruction/building/wall_decorator_archivist.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-774ee58bc153366d1d/abc-building-35/crates/country-core/src/resources/walls/decorator_archivist.rs](../../skeleton/task-index/work/TG-774ee58bc153366d1d/abc-building-35/crates/country-core/src/resources/walls/decorator_archivist.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-774ee58bc153366d1d/abc-building-35/crates/country-core/src/resources/walls/decorator_archivist.rs](../../skeleton/task-index/work/TG-774ee58bc153366d1d/abc-building-35/crates/country-core/src/resources/walls/decorator_archivist.rs)：explicit_candidate_delivery_mapping | country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_colliders（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_interactions（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/walls/decorator_collision.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/decorator_outline.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/decorator_storage.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/decorator_subtype.rs` | A,B,C | [reconstruction/building/decorator_subtype.rs](../../reconstruction/building/decorator_subtype.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-60901c6c8508b1af76/abc-building-35/crates/country-core/src/resources/walls/decorator_subtype.rs](../../skeleton/task-index/work/TG-60901c6c8508b1af76/abc-building-35/crates/country-core/src/resources/walls/decorator_subtype.rs)：explicit_candidate_delivery_mapping | country_core::resources::walls::decorator_subtype::DecoratorLibKey::new（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/walls/dormers.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/elevation_support_stats.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/gates.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/in_progress_wall.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/inner_wall_state.rs` | A,B,C | [reconstruction/building/wall_height_authority.rs](../../reconstruction/building/wall_height_authority.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-3188a81d4989b20289/abc-tools-52/crates/country-core/src/resources/walls/inner_wall_state.rs](../../skeleton/task-index/work/TG-3188a81d4989b20289/abc-tools-52/crates/country-core/src/resources/walls/inner_wall_state.rs)：explicit_candidate_delivery_mapping | country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/walls/inner_walls.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/material_stats.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/mod.rs` | A,B,C | [skeleton/task-index/work/TG-4509462cb49323f18d/abc-tools-52/crates/country-core/src/resources/walls/mod.rs](../../skeleton/task-index/work/TG-4509462cb49323f18d/abc-tools-52/crates/country-core/src/resources/walls/mod.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-4509462cb49323f18d/abc-building-35/crates/country-core/src/resources/walls/mod.rs](../../skeleton/task-index/work/TG-4509462cb49323f18d/abc-building-35/crates/country-core/src/resources/walls/mod.rs)：explicit_candidate_delivery_mapping | country_core::resources::walls::WallId::is_valid（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::walls::WallId::get_inner（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/walls/plaster.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/public_wall_state.rs` | A,B,C | [reconstruction/building/wall_state.rs](../../reconstruction/building/wall_state.rs)：reviewed_partial_module_mapping<br>[reconstruction/wall/height_rules.rs](../../reconstruction/wall/height_rules.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；3个缓存叶函数及数值后段；非整个墙系统：[evidence/scope-priorities/wall-rules-verification.json](../../evidence/scope-priorities/wall-rules-verification.json) |
| `crates/country-core/src/resources/walls/public_walls.rs` | A,B,C | [reconstruction/building/wall_state.rs](../../reconstruction/building/wall_state.rs)：reviewed_partial_module_mapping<br>[reconstruction/wall/height_rules.rs](../../reconstruction/wall/height_rules.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；3个缓存叶函数及数值后段；非整个墙系统：[evidence/scope-priorities/wall-rules-verification.json](../../evidence/scope-priorities/wall-rules-verification.json) |
| `crates/country-core/src/resources/walls/stone_foundation_cutouts.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/stone_foundation_surface.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/wall_attachment.rs` | A,B,C | [skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-tools-52/crates/country-core/src/resources/walls/wall_attachment.rs](../../skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-tools-52/crates/country-core/src/resources/walls/wall_attachment.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-building-35/crates/country-core/src/resources/walls/wall_attachment.rs](../../skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-building-35/crates/country-core/src/resources/walls/wall_attachment.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-building-35/crates/country-core/src/resources/walls/wall_attachment.rs](../../skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-building-35/crates/country-core/src/resources/walls/wall_attachment.rs)：explicit_candidate_delivery_mapping | country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::walls::wall_attachment::impl$1::default（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/walls/wall_holes.rs` | A,B,C | [reconstruction/building/wall_holes.rs](../../reconstruction/building/wall_holes.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs](../../skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs](../../skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs](../../skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs](../../skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs)：explicit_candidate_delivery_mapping | country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::walls::wall_holes::FinalizedHoleStorage::inner（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::walls::wall_holes::InProgressHoleStorage::holes（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/walls/wall_path_segment.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/wall_render_slots.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/wall_stone_columns.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/wall_trigger_volume.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/window_cuts.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/wooden_arch_frame.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/wooden_gates.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/wooden_platform_stairs.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/walls/wooden_wall_boards.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/building/decorator_info.rs

[reconstruction/building/decorator_info.rs](../../reconstruction/building/decorator_info.rs)

SHA256：`0c9e795a65ba4fc08150c245281c26e74f4a7c024c1958dab10027c85695d726`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_observed` | 31 | 未由扫描证明；以证据限定范围为准 |
| `bytes` | 32 | 未由扫描证明；以证据限定范围为准 |
| `destination_tag` | 33 | 未由扫描证明；以证据限定范围为准 |
| `rank` | 34 | 未由扫描证明；以证据限定范围为准 |
| `decorator_type` | 35 | 未由扫描证明；以证据限定范围为准 |
| `decorator_id` | 36 | 未由扫描证明；以证据限定范围为准 |
| `set_wall_coordinate` | 37 | 未由扫描证明；以证据限定范围为准 |
| `try_dst_as_wall` | 44 | 未由扫描证明；以证据限定范围为准 |
| `dst_as_wall` | 47 | 未由扫描证明；以证据限定范围为准 |
| `wall_fields` | 48 | 未由扫描证明；以证据限定范围为准 |
| `wall_opening_policy` | 58 | 未由扫描证明；以证据限定范围为准 |
| `anchor_max` | 78 | 未由扫描证明；以证据限定范围为准 |
| `anchor_min` | 79 | 未由扫描证明；以证据限定范围为准 |
| `wall_attachment_height` | 83 | 未由扫描证明；以证据限定范围为准 |
| `project_wall_space_clamp` | 102 | 未由扫描证明；以证据限定范围为准 |
| `move_wall_decorator_following_anchor` | 114 | 未由扫描证明；以证据限定范围为准 |
| `decorator_source_from_place` | 139 | 未由扫描证明；以证据限定范围为准 |
| `insert_decorator_source` | 163 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L113：/// typed gap before commit instead of publishing an incomplete door position.
### reconstruction/building/decorator_subtype.rs

[reconstruction/building/decorator_subtype.rs](../../reconstruction/building/decorator_subtype.rs)

SHA256：`f9d393d67c90be2eb04a58a174448abe18c4b5f29eba19a42d598506b2a6dd27`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `wall_opening_family` | 35 | 未由扫描证明；以证据限定范围为准 |
| `b_wall_subtype_collision_asset_name` | 48 | 未由扫描证明；以证据限定范围为准 |
| `read` | 66 | 未由扫描证明；以证据限定范围为准 |
| `max` | 69 | 未由扫描证明；以证据限定范围为准 |
| `min` | 72 | 未由扫描证明；以证据限定范围为准 |
| `nearest_segment` | 77 | 未由扫描证明；以证据限定范围为准 |
| `closest_roof_edge` | 105 | 未由扫描证明；以证据限定范围为准 |
| `is_half_dormer` | 157 | 未由扫描证明；以证据限定范围为准 |
| `derive_wall_subtype` | 205 | 未由扫描证明；以证据限定范围为准 |
| `wall_attachment_subtype` | 223 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_decorator_archivist.rs

[reconstruction/building/wall_decorator_archivist.rs](../../reconstruction/building/wall_decorator_archivist.rs)

SHA256：`e384b89ff5b307495d3b383f1e10a6352e287cd2cd03d50de5a6362911b317eb`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `restore_one` | 32 | 未由扫描证明；以证据限定范围为准 |
| `restore_archived_decorators` | 56 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_height_authority.rs

[reconstruction/building/wall_height_authority.rs](../../reconstruction/building/wall_height_authority.rs)

SHA256：`7a6204c8c399872ef8fdfd56df8680e0ae2e6771bb57b38bc4da2b7bff5089fd`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `assign_resolved_height` | 19 | 未由扫描证明；以证据限定范围为准 |
| `write_resolved_source_height` | 35 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_holes.rs

[reconstruction/building/wall_holes.rs](../../reconstruction/building/wall_holes.rs)

SHA256：`2956c98235c406d306bfaf00a494a9b125fa8b4883cc19ad974ad51d7a6007d6`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `raw_hole_hash` | 45 | 未由扫描证明；以证据限定范围为准 |
| `add` | 55 | 未由扫描证明；以证据限定范围为准 |
| `subtract` | 63 | 未由扫描证明；以证据限定范围为准 |
| `default` | 83 | 未由扫描证明；以证据限定范围为准 |
| `new_in_progress` | 88 | 未由扫描证明；以证据限定范围为准 |
| `phase` | 99 | 未由扫描证明；以证据限定范围为准 |
| `fingerprint` | 102 | 未由扫描证明；以证据限定范围为准 |
| `previous_fingerprint` | 105 | 未由扫描证明；以证据限定范围为准 |
| `raw_holes` | 108 | 未由扫描证明；以证据限定范围为准 |
| `finalized_holes` | 111 | 未由扫描证明；以证据限定范围为准 |
| `wall_length` | 117 | 未由扫描证明；以证据限定范围为准 |
| `closed` | 123 | 未由扫描证明；以证据限定范围为准 |
| `clone_and_reopen` | 128 | 未由扫描证明；以证据限定范围为准 |
| `advance_frame` | 136 | 未由扫描证明；以证据限定范围为准 |
| `add` | 154 | 未由扫描证明；以证据限定范围为准 |
| `finalize` | 166 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_state.rs

[reconstruction/building/wall_state.rs](../../reconstruction/building/wall_state.rs)

SHA256：`3ac4555841032f8347e1d28da60be7260eead44ae7408c8de265169ce269cf4d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `validate_public_wall_read` | 17 | 未由扫描证明；以证据限定范围为准 |
| `from_observed` | 33 | 未由扫描证明；以证据限定范围为准 |
| `bytes` | 36 | 未由扫描证明；以证据限定范围为准 |
| `tag` | 39 | 未由扫描证明；以证据限定范围为准 |
| `synchronize_roof` | 46 | 未由扫描证明；以证据限定范围为准 |
| `from_observed` | 71 | 未由扫描证明；以证据限定范围为准 |
| `validate_read` | 74 | 未由扫描证明；以证据限定范围为准 |
| `source_point_count` | 80 | 未由扫描证明；以证据限定范围为准 |
| `source_curve_length` | 83 | 未由扫描证明；以证据限定范围为准 |
| `published_source_curve_length` | 86 | 未由扫描证明；以证据限定范围为准 |
| `max_y` | 89 | 未由扫描证明；以证据限定范围为准 |
| `max_y_gable` | 92 | 未由扫描证明；以证据限定范围为准 |
| `shape` | 95 | 未由扫描证明；以证据限定范围为准 |
| `is_halftimbered` | 99 | 未由扫描证明；以证据限定范围为准 |
| `float_at` | 102 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/wall/height_rules.rs

[reconstruction/wall/height_rules.rs](../../reconstruction/wall/height_rules.rs)

SHA256：`a518c1364af4473f41f528815f3a720e9f0ea647f86b678dbbd9449b128728a7`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `wall_height_bounds` | 13 | 未由扫描证明；以证据限定范围为准 |
| `clamp_resolved_wall_height` | 25 | 未由扫描证明；以证据限定范围为准 |
| `cached_wall_max_y` | 49 | 未由扫描证明；以证据限定范围为准 |
| `cached_gable_max_y` | 56 | 未由扫描证明；以证据限定范围为准 |
| `flat_roof_y` | 64 | 未由扫描证明；以证据限定范围为准 |
| `main` | 69 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L12：/// unknown; it is not labelled as a gameplay feature without producer proof.
- L94：_ => panic!("unknown mode"),
### skeleton/task-index/work/TG-3188a81d4989b20289/abc-tools-52/crates/country-core/src/resources/walls/inner_wall_state.rs

[skeleton/task-index/work/TG-3188a81d4989b20289/abc-tools-52/crates/country-core/src/resources/walls/inner_wall_state.rs](../../skeleton/task-index/work/TG-3188a81d4989b20289/abc-tools-52/crates/country-core/src/resources/walls/inner_wall_state.rs)

SHA256：`c3f84a083063b2359e456870151314b3b15e8645cdbec2f08b71cd78858df807`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `get_enclosure_params` | 17 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-43142338addb95ca2d/abc-building-35/crates/country-core/src/resources/walls/decorator.rs

[skeleton/task-index/work/TG-43142338addb95ca2d/abc-building-35/crates/country-core/src/resources/walls/decorator.rs](../../skeleton/task-index/work/TG-43142338addb95ca2d/abc-building-35/crates/country-core/src/resources/walls/decorator.rs)

SHA256：`8ba52dfdd1b9e3be4de1d40c81bb47f7b056dedb5f25f5c56e54fa0524ee077d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 14 | 未由扫描证明；以证据限定范围为准 |
| `new` | 29 | 未由扫描证明；以证据限定范围为准 |
| `new` | 39 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-43142338addb95ca2d/abc-tools-52/crates/country-core/src/resources/walls/decorator.rs

[skeleton/task-index/work/TG-43142338addb95ca2d/abc-tools-52/crates/country-core/src/resources/walls/decorator.rs](../../skeleton/task-index/work/TG-43142338addb95ca2d/abc-tools-52/crates/country-core/src/resources/walls/decorator.rs)

SHA256：`ee76dcd8ef32ec8d46294517a674852c7f3d2b7ba5441c9471520b4bcf0f170f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_initial` | 14 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-4509462cb49323f18d/abc-building-35/crates/country-core/src/resources/walls/mod.rs

[skeleton/task-index/work/TG-4509462cb49323f18d/abc-building-35/crates/country-core/src/resources/walls/mod.rs](../../skeleton/task-index/work/TG-4509462cb49323f18d/abc-building-35/crates/country-core/src/resources/walls/mod.rs)

SHA256：`458893fe16b340cd2761fdb11b6ee62e7d239368819620ee41676be473a7639a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `get_inner` | 10 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-4509462cb49323f18d/abc-tools-52/crates/country-core/src/resources/walls/mod.rs

[skeleton/task-index/work/TG-4509462cb49323f18d/abc-tools-52/crates/country-core/src/resources/walls/mod.rs](../../skeleton/task-index/work/TG-4509462cb49323f18d/abc-tools-52/crates/country-core/src/resources/walls/mod.rs)

SHA256：`7671948c78b1b7139cf423c0b8250bc20a4545446934e1601d4d2104436e4840`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_valid` | 12 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-60901c6c8508b1af76/abc-building-35/crates/country-core/src/resources/walls/decorator_subtype.rs

[skeleton/task-index/work/TG-60901c6c8508b1af76/abc-building-35/crates/country-core/src/resources/walls/decorator_subtype.rs](../../skeleton/task-index/work/TG-60901c6c8508b1af76/abc-building-35/crates/country-core/src/resources/walls/decorator_subtype.rs)

SHA256：`69033449ffb26e60229974781d8a5bd787b7404b4aedc062228f141a1f37717f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 16 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-774ee58bc153366d1d/abc-building-35/crates/country-core/src/resources/walls/decorator_archivist.rs

[skeleton/task-index/work/TG-774ee58bc153366d1d/abc-building-35/crates/country-core/src/resources/walls/decorator_archivist.rs](../../skeleton/task-index/work/TG-774ee58bc153366d1d/abc-building-35/crates/country-core/src/resources/walls/decorator_archivist.rs)

SHA256：`be2e21ce0f663e26141be9e2e91b19eb0a996f19f42cab5818ae8868d323fcbc`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `decorator_colliders` | 15 | 未由扫描证明；以证据限定范围为准 |
| `decorator_interactions` | 22 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs

[skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs](../../skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs)

SHA256：`4f940201c9c9d91e5035200ff6431f869cd53cba41d3d42d8e3b25e95cf71aeb`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `wall_length` | 21 | 未由扫描证明；以证据限定范围为准 |
| `holes_no_duplicates` | 30 | 未由扫描证明；以证据限定范围为准 |
| `holes` | 34 | 未由扫描证明；以证据限定范围为准 |
| `inner` | 46 | 未由扫描证明；以证据限定范围为准 |
| `holes` | 58 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-building-35/crates/country-core/src/resources/walls/wall_attachment.rs

[skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-building-35/crates/country-core/src/resources/walls/wall_attachment.rs](../../skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-building-35/crates/country-core/src/resources/walls/wall_attachment.rs)

SHA256：`8431804309c9a379580349d9a30632b4e5d2469442d5299ebb1326a4b016c756`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `default` | 10 | 未由扫描证明；以证据限定范围为准 |
| `to_rectangle_corner_id` | 20 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-tools-52/crates/country-core/src/resources/walls/wall_attachment.rs

[skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-tools-52/crates/country-core/src/resources/walls/wall_attachment.rs](../../skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-tools-52/crates/country-core/src/resources/walls/wall_attachment.rs)

SHA256：`9e9f32b92bcdb83670a4eeddf4e3e3d037bca8214c8f18e3324493383220002a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_attached` | 15 | 未由扫描证明；以证据限定范围为准 |
