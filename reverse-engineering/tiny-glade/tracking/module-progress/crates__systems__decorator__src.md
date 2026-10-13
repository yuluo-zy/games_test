# crates/systems/decorator/src

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

墙面四族asset/subtype/position→raw holes→wall brick；Info/storage与部分stairs候选。

## 接线与验证

接线：墙面子集真实World已接。

placement1668/subtype1326等限定报告；新增Info/storage不冒称当前整批通过。

## 已恢复的函数和类型范围

- load_decorators/WallSpaceCoord定位
- derive_decorator_subtype、四族raw-hole producer
- apply_decorator_op/archivist/storage部分候选

## 明确缺口

- 全部decorator_visual家族：dormer/chimney/corner/flag/clothline/lantern等
- 真实anchor/拖动与roof宿主形变、跨对象恢复

下一闭包：全部decorator_visual家族：dormer/chimney/corner/flag/clothline/lantern等

## 证据

- [evidence/building-placement/B-placement-verification.json](../../evidence/building-placement/B-placement-verification.json)
- [evidence/building-placement/B-subtype-verification.json](../../evidence/building-placement/B-subtype-verification.json)
- [evidence/building-integrate/findings.md](../../evidence/building-integrate/findings.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/decorator/src/apply_decorator_op.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/audio/mod.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/audio/op_confirm_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/audio/op_preview_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/audio/op_stairs_audio.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/backup.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/blueprint.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/cache_decorator_transforms.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/chimney_smoke_anim_thickness.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/collision_detection.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/cull_oob_decorators.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_on_freehand_wall_edited.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_on_move_shape.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_on_rectangle_edited.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_on_wall_height_changed.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_resources.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/arrow_slit.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/chimney_assemble_bricks.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/chimney_roof.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/chimney_wall.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/cottage_balcony_door.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/cottage_corner_window.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/cottage_dormers/collision_geo.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/cottage_dormers/maintain_dormer_tiles_draw.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/cottage_dormers/mantain_dormer_tiles_draw.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/cottage_dormers/util_assemble_gothic_dormer.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/cottage_wall_window.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/deco_differential_state.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/decorator_clutter.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/door_stairs_assemble_bricks.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/flag.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/flag_connections.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/generate_clothline.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/gothic_balcony_door.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/gothic_dormer.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/gothic_wall_window.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/lantern.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/maintain_visual_entities.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/mantain_visual_entities.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/calculate_distortion_from_structural_graph.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/create_structural_colliders_and_initial_graph.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/foliage_exclusion.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/mod.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/playermade_stairs_assemble_visuals.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/stairs_graph_and_requests.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/stairs_lazy_update.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/stairs_wall_holes.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_assemble_wood_misc.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_railing_shared.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_rope_bridge.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_stone_railings_assemble.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/playermade_stairs/utils/stairs_wooden_railings_assemble.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/stairs.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/stairs_assemble.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual/trap_doors.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/decorator_visual_state.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/deferred_decorator_history.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/derive_decorator_subtype.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/dormer_holes.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/handle_decorator_deletion_cmds.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/handle_external_decorator_cmds.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/load_decorators.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/merge_proposal_roof_decorators.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/merge_proposal_wall_decorators.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/move_decorators_following_anchors.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/remove_ivy_under_windows.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/replay_decorator.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/restore_archived_decorators.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/signifiers.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/stone_foundation_cutouts.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/calculate_decorator_dst.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/calculate_decorator_grab_offset.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/move_ops.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/ui_change_chimney_height.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/ui_change_stair_height.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/ui_decorator_affordance_dispatch.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/ui_focus_stairs.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/ui_inactive.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/ui_move_decorator.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/ui_place_decorator.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/ui_rotate_lantern.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/ui_stairs_misc.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/ui_systems/ui_stairs_viz.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/utils/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/utils/visual_entity_builder.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/decorator/src/window_lights.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

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
### reconstruction/building/decorator_placement.rs

[reconstruction/building/decorator_placement.rs](../../reconstruction/building/decorator_placement.rs)

SHA256：`9f1b56e53e73f8cf26ec2ef51aa8cbf818dd5fb514874b314a95d9abef93e345`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `metrics` | 23 | 未由扫描证明；以证据限定范围为准 |
| `wall_decorator_key` | 57 | 未由扫描证明；以证据限定范围为准 |
| `b_wall_collision_asset_name` | 75 | 未由扫描证明；以证据限定范围为准 |
| `mesh_collision_bbx` | 108 | 未由扫描证明；以证据限定范围为准 |
| `validate_curve` | 141 | 未由扫描证明；以证据限定范围为准 |
| `wall_coord_from_world` | 160 | 未由扫描证明；以证据限定范围为准 |
| `wall_coord_to_world` | 205 | 未由扫描证明；以证据限定范围为准 |
| `place_wall_decorator` | 245 | 未由扫描证明；以证据限定范围为准 |
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
### reconstruction/building/opening_rules.rs

[reconstruction/building/opening_rules.rs](../../reconstruction/building/opening_rules.rs)

SHA256：`d8db51ae36fee5c45464014a650eb597b1ebd52bd345961389668e10a8002056`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `generate_wall_opening` | 40 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/building/wall_decorator_archivist.rs

[reconstruction/building/wall_decorator_archivist.rs](../../reconstruction/building/wall_decorator_archivist.rs)

SHA256：`e384b89ff5b307495d3b383f1e10a6352e287cd2cd03d50de5a6362911b317eb`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `restore_one` | 32 | 未由扫描证明；以证据限定范围为准 |
| `restore_archived_decorators` | 56 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/decorator_assets.rs

[reconstruction/crates/tg-building-runtime/src/decorator_assets.rs](../../reconstruction/crates/tg-building-runtime/src/decorator_assets.rs)

SHA256：`4c6671718f920e8b977ac7d61c1f33631f7030a08c3b866935867921e40464a3`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `decode_collision_mesh` | 66 | 未由扫描证明；以证据限定范围为准 |
| `load_b_wall_opening_assets` | 122 | 未由扫描证明；以证据限定范围为准 |
| `derive_and_place_wall` | 144 | 未由扫描证明；以证据限定范围为准 |
| `load_b_plain_wall_assets` | 209 | 未由扫描证明；以证据限定范围为准 |
| `load_bindings` | 217 | 未由扫描证明；以证据限定范围为准 |
| `place` | 237 | 未由扫描证明；以证据限定范围为准 |
| `b_plain_wall_bindings` | 252 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/decorator_info_binding.rs

[reconstruction/crates/tg-building-runtime/src/decorator_info_binding.rs](../../reconstruction/crates/tg-building-runtime/src/decorator_info_binding.rs)

SHA256：`eb9894ccee04322c08be5705cbf9b3e1c7e39616cacfa8140f1358cf70923aca`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `owns` | 15 | 未由扫描证明；以证据限定范围为准 |
| `owns_native` | 16 | 未由扫描证明；以证据限定范围为准 |
| `register` | 40 | 未由扫描证明；以证据限定范围为准 |
| `consume_ids` | 48 | 未由扫描证明；以证据限定范围为准 |
| `consume_places` | 52 | 未由扫描证明；以证据限定范围为准 |
| `derive_input` | 69 | 未由扫描证明；以证据限定范围为准 |
| `resolve` | 82 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/decorator_placement.rs

[reconstruction/crates/tg-building-runtime/src/decorator_placement.rs](../../reconstruction/crates/tg-building-runtime/src/decorator_placement.rs)

SHA256：`b00254586b5e91bd50fcff5831be5236f34218bc2ab79f768c101eb9622fc488`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 35 | 未由扫描证明；以证据限定范围为准 |
| `resolve` | 44 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/crates/tg-building-runtime/src/openings.rs

[reconstruction/crates/tg-building-runtime/src/openings.rs](../../reconstruction/crates/tg-building-runtime/src/openings.rs)

SHA256：`ed6651b2acad8b5e67461007b08d8090a401e0d83a1b3744c87d0ca6a495920c`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `register` | 46 | 未由扫描证明；以证据限定范围为准 |
| `consume_inputs` | 67 | 未由扫描证明；以证据限定范围为准 |
| `rebuild_openings` | 100 | 未由扫描证明；以证据限定范围为准 |
| `gather_and_finalize` | 156 | 未由扫描证明；以证据限定范围为准 |
