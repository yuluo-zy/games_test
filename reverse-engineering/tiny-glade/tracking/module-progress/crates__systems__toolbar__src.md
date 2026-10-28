# crates/systems/toolbar/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

索引/零散候选；完整系统范围尚未核对。

## 接线与验证

接线：not_established。

未登记完整模块验收。

## 已恢复的函数和类型范围

原函数全集尚未核对。下面的Rust定义清单仅登记实现符号。

## 明确缺口

- 完整文件/函数、状态/资源和生产者消费者尚未恢复

下一闭包：完整文件/函数、状态/资源和生产者消费者尚未恢复

## 证据

尚未登记模块级证明。

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/toolbar/src/decoclutter_picker.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/demo_stuff/demo_show_full_glade_size.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/demo_stuff/demo_show_max_elevation.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/mei/audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/mei/config.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/mei/core.rs` | A,B,C | [skeleton/task-index/work/TG-70a753eeccd8998d55/abc-tools-52/crates/systems/toolbar/src/mei/core.rs](../../skeleton/task-index/work/TG-70a753eeccd8998d55/abc-tools-52/crates/systems/toolbar/src/mei/core.rs)：explicit_candidate_delivery_mapping | toolbar::mei::core::impl$2::as_id（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/systems/toolbar/src/mei/layout.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/mei/widgets.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/mei/widgets/decorate.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/mei/widgets/tweaker.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/corrupted_save_file.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/credits.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/daily_welcome.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/import.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/load.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/missing_mods.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/mod_checklist.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/mod_row.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/mods.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/mods_glade_tab.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/mods_mine_tab.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/mods_workshop_tab.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/new_session.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/report_bug.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/save_error.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/splash.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/thumbnail_scroll.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/menu/welcome.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/photomode/lut_gallery.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/photomode/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/photomode/photomode_buttons.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/photomode/setup_photomode_toolbar.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/photomode/ui_slider.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/quick_tool_palette.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/resouces_and_components/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/resouces_and_components/taffy_utils.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/resouces_and_components/undo_redo_layout.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/taffy_update.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/text.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/third_party_licenses_server.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/toolbar_button.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/toolbar_startup.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/toolbar_systems.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/toolbar_update.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/toolbar/src/ui_interaction.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### skeleton/task-index/work/TG-70a753eeccd8998d55/abc-tools-52/crates/systems/toolbar/src/mei/core.rs

[skeleton/task-index/work/TG-70a753eeccd8998d55/abc-tools-52/crates/systems/toolbar/src/mei/core.rs](../../skeleton/task-index/work/TG-70a753eeccd8998d55/abc-tools-52/crates/systems/toolbar/src/mei/core.rs)

SHA256：`7c13c0e681493bda984b7dcea9e97838874ea3fd6d8ff4017740bf96066bdbfb`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `as_id` | 15 | 未由扫描证明；以证据限定范围为准 |
