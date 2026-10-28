# crates/country-core/src/systems

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_source_candidate; ownership_requires_dependency_filter。

仅有源码清单或候选交付；完整模块行为尚未评估

## 接线与验证

接线：not_established。

未建立模块/文件级验收；候选交付不等于原作等价

## 已恢复的函数和类型范围

原函数全集尚未核对。下面的Rust定义清单仅登记实现符号。

## 明确缺口

- 原函数全集、类型/ABI、生产者/消费者和版本适配尚未核对

下一闭包：先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向

## 证据

尚未登记模块级证明。

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/systems/arrow_gizmo_animation.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/brush_preview.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/camera_rig.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/clear_canvas.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/collision.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/despawn.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/egui_input.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/garden.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/look_at_camera.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/main_camera.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/mesh_follow_terrain.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/mod_editor.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/mod_key.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/mode_manager.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/prefab.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/stone_foundation_offset.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/temp_eraser.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/time_of_day.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/time_update.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/timelapse_replay.rs` | A,B,C | [skeleton/task-index/work/TG-33dfed54eb285875e2/abc-tools-52/crates/country-core/src/systems/timelapse_replay.rs](../../skeleton/task-index/work/TG-33dfed54eb285875e2/abc-tools-52/crates/country-core/src/systems/timelapse_replay.rs)：explicit_candidate_delivery_mapping | country_core::systems::timelapse_replay::TimelapseState::is_active（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/systems/ui_2d_follow_mouse.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/ui_camera_mode_gizmos.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/ui_edit_focus.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/ui_edit_freehand_walls.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/ui_elevate_wall_group.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/ui_focus_mode_shape_height.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/ui_move_freehand_walls.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/ui_move_shapes.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/ui_move_wall_group.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/ui_raise_shape.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/ui_wall_transform_gizmos.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/utils.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/utils_intersect.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/weather_system.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/wooden_stair_support.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/world_raster.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### skeleton/task-index/work/TG-33dfed54eb285875e2/abc-tools-52/crates/country-core/src/systems/timelapse_replay.rs

[skeleton/task-index/work/TG-33dfed54eb285875e2/abc-tools-52/crates/country-core/src/systems/timelapse_replay.rs](../../skeleton/task-index/work/TG-33dfed54eb285875e2/abc-tools-52/crates/country-core/src/systems/timelapse_replay.rs)

SHA256：`b026c566719dbe8f6d0b23d433e07b1e7cdb77ab950d57304ce637c5bc9b3475`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_active` | 14 | 未由扫描证明；以证据限定范围为准 |
