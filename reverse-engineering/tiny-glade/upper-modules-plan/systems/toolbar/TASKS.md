# systems/toolbar 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/toolbar` |
| package | `toolbar` |
| 规模 | 38 文件 / 509 本 ns 函数 / 129 闭包 / **331 业务函数**（上层函数数最多） |
| 难度 | 高（状态机面积大、证据几乎为零） |
| 批次 | 3（主线第三步） |
| 上游依赖 | `country-core`、`taffy`（布局）、`tg-text`、`theme-defs`、`lang-lib`、`tiny_http`、`image`、`ron`、`rhapsody`、`utils` |
| 复现工程现状 | MODULE_PROGRESS：**部分候选实现**（1 恢复文件）；仅索引 |

## 任务表（按 B src 层级）

### mei UI 框架（自研 UI 引擎）

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TOOL-01 | `src/mei/core.rs` | 恢复 mei 核心：`MeiContext::{load_configs, load_font_table, new_at_pos, reuse_or_create_node, mei_pre_update, mei_post_update, safe_frame, on_key_event, on_clipboard_paste}`、`Node::{set_sounds, ignore_input, force_interactive, with_nav, hover, has_focus, block_input, prev_computed_layout}`、`NodeContext::*`、`NodeContent::*` 多单态、`process_input_action_handlers`。 | taffy、tg-text | 无 | 原入口对照 + 节点复用/输入分派一致 | 未开始 |
| UM-TOOL-02 | `src/mei/config.rs` | 恢复 `read_node_config_from_file`、`instantiate_ui_template_impl`、`TemplateView::*`、`load_font_table`、`MeiContext::load_configs`。 | ron | 无 | 原入口对照 + 模板实例化一致 | 未开始 |
| UM-TOOL-03 | `src/mei/layout.rs` | 恢复 `horizontal`、`LayoutStyle::{gap, expand, space_between, align_center, xalign_center, size, place_childen}`、`ComputedLayout::center`。 | taffy | 无 | 原指令样例 + 布局参数一致 | 未开始 |
| UM-TOOL-04 | `src/mei/widgets.rs` | 恢复 widget 构建器/视图：`label*`、`menu_button*`、`settings_button`、`image`、`tab_switcher`、`space`、`action_hint`、`pill_button`、`back_button` 及 `*View::{content_mut, on_click, hotkey, weight, max_width, align, color, tint, ...}`。 | 布局 | 无 | 原指令样例 | 未开始 |
| UM-TOOL-05 | `src/mei/widgets/tweaker.rs` | 恢复 `tweaker<...>` 多单态（bool/enum/float/list/string）、`TweakerView::delta`、`format<...>`、`HintTracker`。 | 布局/settings | 无 | 原入口对照 + 各单态一致 | 未开始 |
| UM-TOOL-06 | `src/mei/widgets/decorate.rs` | 恢复 `decorate_hovered`。 | 布局 | 无 | 原指令样例 | 未开始 |
| UM-TOOL-07 | `src/mei/audio.rs` | 恢复 mei 音频（无独立符号，需定位内联宿主）。 | 音频胶水 | 无 | 原指令样例 | 未开始 |

### 布局与 taffy

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TOOL-08 | `src/resouces_and_components/taffy_layout.rs` | 恢复 `TaffyLayout::{style, perform_layout}`、`make_layout_absolute`。 | taffy | 无 | 原入口对照 | 未开始 |
| UM-TOOL-09 | `src/resouces_and_components/taffy_utils.rs` | 恢复 taffy 工具（无独立符号，需定位内联宿主）。 | taffy | 无 | 原指令样例 | 未开始 |
| UM-TOOL-10 | `src/taffy_update.rs` | 恢复 `taffy_update_drawables`。 | taffy | 无 | 原入口对照 | 未开始 |
| UM-TOOL-11 | `src/resouces_and_components/toolbar_layout.rs` | 恢复 `ToolbarLayout::{setup_screen_and_toolbar_nodes, add_contextual_menu, link_up_nav}`、`add_toolbar_item`。 | 布局 | 无 | 原入口对照 | 未开始 |
| UM-TOOL-12 | `src/resouces_and_components/undo_redo_layout.rs` | 恢复 `UndoRedoLayout::setup`。 | 布局 | 无 | 原指令样例 | 未开始 |
| UM-TOOL-13 | `src/resouces_and_components/mod.rs` | 恢复资源/组件定义、`LogSlider::param_to_value`、多个 `register_required_components`。 | — | 无 | 类型合同对照 | 未开始 |

### 工具栏系统 / 交互

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TOOL-14 | `src/ui_interaction.rs` | 恢复 `ui_nav_update`、`hide_ui`、`ui_animation_state_update`、`ui_sounds`、`render_taffy_ui_to_widgets`、`ui_*_animation`（focus/appear/select/locked）、`ui_show_if_parent_in_focus`、`ui_focus_parent_if_self_in_focus`、`sort_contextual_buttons_draw_order`、`debug_hide_ui`。 | mei | 无 | 原入口对照 + 焦点/导航规则一致 | 未开始 |
| UM-TOOL-15 | `src/toolbar_systems.rs` | 恢复 `toolbar_alignment`、`undo_redo_alignment`、`undo_redo_auto_hide`、`toolbar_auto_hide`、`toolbar_steal_mouse_input`。 | 布局/输入 | 无 | 原指令样例 | 未开始 |
| UM-TOOL-16 | `src/toolbar_button.rs` | 恢复 `toolbar_button_keep_focus`、`switch_app_mode_button`、`switch_toolbar_category_button`、`tod_v2_button`。 | 布局/输入 | 无 | 原指令样例 | 未开始 |
| UM-TOOL-17 | `src/toolbar_startup.rs` | 恢复 `toolbar_setup`。 | 全部 | 无 | 原入口对照 | 未开始 |
| UM-TOOL-18 | `src/toolbar_update.rs` | 恢复 `toolbar_button_size_and_padding_update`。 | 布局 | 无 | 原指令样例 | 未开始 |

### 菜单

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TOOL-19 | `src/menu/mod.rs` | 恢复 `advance_menu_state` + 插件注册。 | 菜单 | 无 | 原入口对照 | 未开始 |
| UM-TOOL-20 | `src/menu/{welcome,new_session,daily_welcome,splash}.rs` | 恢复 `welcome::ui`、`new_session::ui`、`daily_welcome::ui`、`splash::ui`。 | mei | 无 | 原指令样例 | 未开始 |
| UM-TOOL-21 | `src/menu/{load,corrupted_save_file,save_error}.rs` | 恢复 `load::ui`、`thumbnail_scroll`、`display_save_file_load_error`、`load_session_meta`、`show_contextual_buttons`、`corrupted_save_file::ui`、`save_error::ui`。 | core 存档 | 无 | 原入口对照 | 未开始 |
| UM-TOOL-22 | `src/menu/credits.rs` | 恢复 `credits::ui`、`spawn_third_party_license_http_server`、`scroll<u64>`。 | tiny_http | 无 | 原入口对照 | 未开始 |
| UM-TOOL-23 | `src/menu/report_bug.rs` | 恢复 `report_bug::ui`、`try_remove_file`、`text_edit`、`toggle_icon`、输入/渲染单态。 | 输入/tg-text | 无 | 原指令样例 | 未开始 |

### photomode

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TOOL-24 | `src/photomode/{mod,photomode_buttons,setup_photomode_toolbar}.rs` | 恢复 `do_photomode`、`photomode_buttons`、`setup_photomode_toolbar`。 | 相机/布局 | 无 | 原入口对照 | 未开始 |
| UM-TOOL-25 | `src/photomode/{ui_slider,lut_gallery}.rs` | 恢复 `ui_slider`、`slider_val_01`、`ui_slider_reset`、`lut_gallery::ui`。 | 渲染/LUT | 无 | 原指令样例 | 未开始 |

### 文本 / 许可服务 / demo / 插件

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TOOL-26 | `src/text.rs` | 恢复 `text_startup_system`、`update_text_renderer_scale`、`prepare_text_fragments`、`render_text_fragments`、`flush_text_render`。 | tg-text/rhapsody | 无 | 原入口对照 + 文本渲染链一致 | 未开始 |
| UM-TOOL-27 | `src/third_party_licenses_server.rs` | 恢复 `SingleFileServer::{http, run}`（tiny_http 单文件服务）。 | tiny_http | 无 | 原入口对照 | 未开始 |
| UM-TOOL-28 | `src/demo_stuff/*` | 恢复 `demo_show_full_glade_size`、`demo_show_max_elevation`。 | 世界尺寸 | 无 | 原指令样例 | 未开始 |
| UM-TOOL-29 | `src/lib.rs` | 恢复插件注册（含 `impl$0/impl$1::build`、`menu::report_bug::process_input_action`、`menu::welcome::ui`、`ui_nav_update`）。 | 全部上面 | 仅索引 | 与原注册顺序对照 | 未开始 |

## 卡点

- mei 是自研 UI 引擎（非 egui/taffy 直接暴露），`core.rs` 单文件含 ~130 函数，是最大单点；建议先做 core+config+layout+widgets 四文件，再铺菜单。
- `taffy` 为公开依赖，直接复用；恢复的是「游戏如何用 taffy」的策略。
- `text.rs` 依赖 tg-text（已内部收口）与 rhapsody。
- `menu/load`、`credits`、`report_bug` 依赖 core 存档/主题/反馈边界（部分 crate 已删或后置）。

## 验收标准

- mei 框架 ≥ 原入口对照；每菜单/按钮 ≥ 原指令样例；焦点/导航规则与原一致。
- 工具栏能驱动 wall/roof/decorator/color 工具（与批次 3 前两模块联动验收）。
