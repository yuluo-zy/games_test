# 第 11 轮 TB 组（toolbar）复核报告

> 复核对象：`evidence/round11/contracts/tb-toolbar-CONTRACT.md`、`evidence/round11/deliveries/tb-toolbar-DELIVERY.md`、
> 当前工作树 `D:/game/tiny-glade/crates/systems/toolbar/**`（含未提交改动）。
> 复核方式：只读源码/证据；契约 82 项逐条对账；MEI 帧顺序与接线核对；独立 target 编译/测试。
> 复核时间：2026-10-19（第 11 轮 TB③；初版 16:2x，增量复评 17:4x 收口版，
> 18:4x 按用户要求更正宿主接线状态）。
> 增量复评：TB② 17:2x-17:43 补入 `toolbar_plugin.rs`/`configure_app`、`deserialize_optional`
> 接线、`taffy_mut_style` 消费、Signifier 真实入流与 `report_bug::try_remove_file`；
> 本报告已按新快照更新分类与结论。**宿主 `run_game.rs` 的插件接入按用户指示已撤回**。

## 0. 结论

**有条件通过（crate 侧面；宿主接线按用户指示不接）。**

判定依据（详见后文）：

1. 契约 82 项中，**真实实现 11 项**（其中 #73/#79 为 crate 侧入口、宿主未接；其余经
   `mei_host` set6/set44 等既有路径消费）、**改名/合并等价且底层被消费 12 项**、
   **部分实现（测试覆盖但无生产消费者 / 主体未闭合 / primitives-only）22 项**、**仍缺 37 项**。
   剩余缺口合计 **59 项**（37 仍缺 + 22 部分）。
2. 收口续跑补上了关键接线：新增 `tiny-glade/src/toolbar_plugin.rs`（set43 chain 8 系统）、
   `toolbar::configure_app`（资源/事件/24 组件）、`deserialize_optional` 接 `LabelConfig`、
   `taffy_mut_style` 接 `TaffyLayout::style_mut`、Signifier 0x20B 载荷真实入流。
   此前的主要“孤儿/吞输出”问题已闭合。
3. **宿主接线按用户指示不接**：`run_game.rs` 当前**不含** `ToolbarPlugin`（18:01 曾短暂加入，
   18:45 已按用户要求撤回）；`crates/tiny-glade/src/lib.rs:56` 保留 `pub mod toolbar_plugin;`，
   `ToolbarPlugin` 作为 **crate 侧入口**保留。因此 set43 的 8 个系统在运行时 App 中不生效，
   这是用户明确要求的边界，**不计 TB 实现缺陷**；结论按 crate 侧面判定。
4. MEI 帧顺序（S0/S1/S7-S13）与契约一致，未出现“绘制后交互”的颠倒。
5. 未发现伪造 VA/字段；`0x14233c300`、`0x142341fe0` 两份 fresh 反编译与实现公式/载荷判等成立。
6. 独立 target 验证：`cargo check -p toolbar` **exit 0**、`cargo check -p tiny-glade` **exit 0**；
   `cargo test -p toolbar --lib` 的 **36 passed / 0 failed** 来自 TB 记录，本报告独立复跑
   被并发构建与 600s 超时截断，未独立证实（详见 §6）。
7. 过程问题：`crates/systems/toolbar/**` 本轮被多个写者并发修改（`Node::used`、`PillButtonContent`、
   `back_button`、`lut_gallery`、`load_ui`、`menu_button` 等）；交付已标注非本 agent 来源。
   复核按当前工作树整体判定，但这些项不计入 TB 交付声明。

## 1. 水分计数

| 类别 | 计数 | 明细 |
|---|---:|---|
| 空函数/只 log 冒充业务 | 0 | 4 个空壳（`ui_slider`/`ui_sounds`/`kind_handler`/`toolbar_auto_hide`）均显式标“未实现”，未冒充完成 |
| 吞输出（`let _ =` 业务输出） | **0（已修复）** | 原 `render_taffy_ui_to_widgets` Signifier 载荷丢弃已改为 `SignifierFactory::new + write_factory` 真实入流（`ui_interaction.rs:452-470`） |
| 伪造成“已完成” | 0 | 未闭合项均标 U/部分；primitives-only API 已在交付 §6.2 降级 |
| 伪造 VA/字段/API | 0 | 抽查 VA 与契约一致；`0x14233c300`/`0x142341fe0` 反编译核对通过 |
| 无证据常量/实现 | **1 组（非 TB 作者）** | `PillButtonContent` 绘制常量（圆角 10、`0xffffff40/0x20`）无交付/反编译引用；交付已声明该项非本 agent 写入 |
| 孤儿导出（有实现、零生产消费者） | **15** | 见下；`deserialize_optional`、`taffy_mut_style` 已接真实消费者，移出 |
| 交付与工作树 scope 偏差 | **0（已修正）** | 交付 17:43 版新增 §5.1/§6.4，明确并发写者与不认领项 |

孤儿导出（测试不算生产消费者；交付 §6.2 已标 primitives-only U）：

1. `mei/core.rs:968` `get_content_impl`
2. `mei/core.rs:988` `into_child`
3. `mei/core.rs:977` `child_with_content`（底层 `get_content_mut_impl` 有消费，此封装仅测试）
4. `mei/core.rs:571` `fullscreen_without_block_input`（`fullscreen` 仅被未注册的 lut_gallery 用）
5. `mei/layout.rs:304` `align_center`
6. `mei/layout.rs:299` `space_between`
7. `mei/layout.rs:309` `xalign_center`
8. `mei/layout.rs:273-319` `horizontal/vertical/gap/padding/expand/size` 构建器族（仅测试）
9. `resouces_and_components/mod.rs:607` `value_to_param`
10. `taffy_layout.rs:165` `TaffyNodeBuilder::root`
11. `taffy_layout.rs:186` `TaffyNodeBuilder::anchored_to`
12. `taffy_layout.rs:206` `TaffyNodeBuilder::child_of`
13. `taffy_layout.rs:58/:63` `roots_mut`/`root_mut`
14. `toolbar_systems.rs:101` `apply_alignment_setters`（仅测试；`toolbar_alignment` 不调用）
15. `photomode.rs:26` `photomode_camera_buttons`（已注册 set8，但无生产者 spawn `PhotomodeButton`）

> 另 `try_remove_file` 有真实 I/O 但生产调用者（`report_bug::ui`）未落，计入“部分”而非孤儿。

另：`photomode/lut_gallery.rs:19 ui` 与 `mei/widgets.rs:284 back_button` 有实现且彼此消费，但
`lut_gallery::ui` 未加入任何 schedule，链条整体不可达（计为“部分”，不单列为孤儿）。

## 2. 契约 82 项（75 + 7）逐项对账

分类：**真实实现** = 主体按证据落库且生产路径可达；**改名等价** = 原符号已由现有等价符号承载且底层被生产消费；
**部分** = 已落主体但缺消费者 / 缺分支 / 仅测试；**仍缺** = 无同名也无可接受等价实现。
“消费者”列只认生产 schedule/真实资源，测试不算。

### 2.1 demo_stuff（2）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 1 | `demo_show_full_glade_size` | 仍缺 | 无符号 | 无 |
| 2 | `demo_show_max_elevation` | 仍缺 | 无符号 | 无 |

### 2.2 mei::config（2）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 3 | `deserialize_optional` | 真实实现 | `config.rs:233`；`LabelConfig` 6 个 `Option` 字段 `deserialize_with`（`config.rs:41-52`） | 经 `read_node_config_from_file` 反序列化真实消费 |
| 4 | `localize_ui_text_from_config` | 改名等价 | `config.rs:227` → 既有 `localize_label`（`config.rs:286` 生产消费） | 生产仍直接调 `localize_label`，公开名仅测试 |

### 2.3 mei::core（16）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 5 | `child_with_content` | 部分 | `core.rs:977` | 仅测试；底层 `child`+`get_content_mut_impl` 有生产消费 |
| 6 | `clear_state` | 改名等价 | → `frame_begin` `core.rs:855` | `mei_pre_update` `systems.rs:91` 消费 |
| 7 | `content_mut` | 改名等价 | `core.rs:962` → `get_content_mut_impl(false)` | 底层被 `config.rs:273`、`widgets.rs:292` 消费 |
| 8 | `RenderContext::draw_rect` | 真实实现 | `core.rs:131`；`center-extent*0.5` | `FrameContent::draw`（`widgets.rs:167`）、`PillButtonContent::draw` |
| 9 | `find_node_path` | 真实实现 | `core.rs:47` | `process_input_action_handlers` `core.rs:822` 消费 |
| 10 | `fullscreen_without_block_input` | 部分 | `core.rs:571` | 仅测试；`fullscreen` 只被未注册的 lut_gallery 消费 |
| 11 | `get_content_impl` | 部分 | `core.rs:968` | 仅测试 |
| 12 | `into_child` | 部分 | `core.rs:988` | 仅测试；底层 `into_child_with_global_id` 存在 |
| 13 | `load_fonts` | 改名等价 | → `load_font_table` `core.rs:771` / `config.rs:203` | `tiny-glade/src/loading_graph.rs:122` 走 `mei_host::load_text_fonts`（等价路径，非本函数） |
| 14 | `lose_focus_on_invalid_nav` | 仍缺 | 无符号 | 无 |
| 15 | `new_root` | 真实实现 | `core.rs:550`（safe_frame/fullscreen/new_at_pos 共用） | `safe_frame` 被 `menu/splash.rs`、`menu/load.rs` 消费 |
| 16 | `prepare_content` | 改名等价 | `systems.rs:381-411` 内联循环 | `mei_post_update` set44 已接线 |
| 17 | `reset_keeping_content` | 改名等价 | → `recycle_frame_nodes` `core.rs:614` | `mei_pre_update`/`mei_post_update` 消费 |
| 18 | `steal_focus` | 仍缺 | 无符号 | 无 |
| 19 | `update_parent_mapping` | 改名等价 | → `build_parent_links` `core.rs:862` | `mei_pre_update` `systems.rs:96` 消费 |
| 20 | `with_layout` | 改名等价 | → `NodeContext::layout` `core.rs:1046` | `menu/splash.rs`、`config.rs:274` 消费 |

### 2.4 mei::layout（5）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 21 | `LayoutStyle::align_center` | 部分 | `layout.rs:304`（justify=1） | 仅测试；菜单/工具栏生产者未落 |
| 22 | `points_or_zero`（及变体） | 真实实现 | `layout.rs:38` | `place_childen` 内部路径（`core.rs:745`）消费 |
| 23 | `Rect::size_vec2` | 真实实现 | `layout.rs:100` | `size_vec2` 被布局内部消费（`layout.rs`） |
| 24 | `LayoutStyle::space_between` | 部分 | `layout.rs:299`（justify=3） | 仅测试 |
| 25 | `LayoutStyle::xalign_center` | 部分 | `layout.rs:309`（align=2） | 仅测试 |

### 2.5 mei::widgets（8）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 26 | `back_button` | 部分 | `widgets.rs:284`（并发写者新增；交付 §5.1 已声明非本 agent） | 仅 `lut_gallery::ui`（未注册）；绘制常量无引用证据 |
| 27 | `ImageView::blur_extend` | 仍缺 | 无符号 | 无 |
| 28 | 共享 `content_mut` trait 默认 | 改名等价 | → `NodeContext::content_mut/get_content_mut_impl` | 底层被 config/widgets 消费 |
| 29 | `MenuButtonView::force_highlight` | 仍缺 | 无符号 | 无 |
| 30 | `MenuButtonView/PillButtonView::hotkey` | 仍缺 | 无符号 | 无 |
| 31 | `PillButtonView::inactive_opacity` | 仍缺 | 无符号 | 无 |
| 32 | `make_hotkey_tooltip` | 仍缺 | 无符号 | 无 |
| 33 | `PillButtonView::mute_click` | 仍缺 | 无符号 | 无 |

### 2.6 menu::credits（3）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 34 | `ScrollView::content_mut` | 仍缺 | 无 `menu/credits.rs` | 无 |
| 35 | `ScrollView::max_scroll_pos` | 仍缺 | 无 | 无 |
| 36 | `spawn_third_party_license_http_server` | 仍缺 | 无 | 无 |

### 2.7 menu::load（8）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 37 | `ThumbnailScrollView::content_mut` | 改名等价 | `menu/load.rs:304` 走共享 `get_content_mut_impl` | `thumbnail_scroll`/lut_gallery 消费（后者未注册） |
| 38 | `display_save_file_load_error`（`display_error`/`draw_error`） | 改名等价 | `menu/load_ui.rs:397/419`：`menu/backup_load_error` 模板 header/error 两 slot | 仅 `tests/original_load_menu.rs` 调度；生产 App 未接线 |
| 39 | `hovered_item_pos` | 部分 | 逻辑内联于 `ThumbnailScrollContent::update_metrics`（`load.rs:244-258`） | 未以命名 helper 拆出；无 load UI 生产者 |
| 40 | `initial_focused_item` | 部分 | 内联于 `thumbnail_scroll` 默认初始化 | 无 load UI 生产者 |
| 41 | `min_scroll_pos` | 部分 | 内联于 `update_metrics`（`load.rs:175-196`） | 同上 |
| 42 | `nav_scroll_speed` | 部分 | 内联（`load.rs:212` `self.navigation_speed`） | 同上 |
| 43 | `run_count` | 部分 | 内联（`load.rs:183-186` 行数计算） | 同上 |
| 44 | `show_contextual_buttons`（`contextual`） | 改名等价 | `menu/load_ui.rs:295`：`position+(0,250)`、`Some(200,30)`、anchor `(0,-1)` 与契约 O 载荷一致 | 仅测试调度；生产 App 未接线 |

### 2.8 menu::new_session（1）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 45 | `resolve_new_glade` | 仍缺 | 无 | 无 |

### 2.9 menu::report_bug（4）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 46 | `TextEditView::content_mut` | 仍缺 | 无 | 无 |
| 47 | `TextEditView::disable_input` | 仍缺 | 无 | 无 |
| 48 | `TextEditView::input_allowed` | 仍缺 | 无 | 无 |
| 49 | `try_remove_file` | 部分 | `menu/report_bug.rs:12`：`is_file`→`remove_file`，失败 log io::Error；单测覆盖 | 生产调用者 `report_bug::ui` 未实现 |

### 2.10 photomode（4）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 50 | `photomode_buttons` | 部分 | `photomode.rs:26` `photomode_camera_buttons` 仅覆盖 tag 0..3/5，且无交付证据引用 | 已注册 set8，但 `PhotomodeButton` 无 spawn 生产者；动作表 4..0x11 未覆盖 |
| 51 | `setup_photomode_toolbar` | 仍缺 | 无 | 无 |
| 52 | `slider_range` | 仍缺 | 无 | 无 |
| 53 | `slider_widget_size` | 仍缺 | 无 | 无 |

### 2.11 resouces_and_components（9）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 54 | `LogSlider::value_to_param` | 部分 | `mod.rs:607` | 仅测试；`ui_slider_reset` 未实现 |
| 55 | `TaffyNodeBuilder::anchored_to` | 部分 | `taffy_layout.rs:186` | 仅测试 |
| 56 | `TaffyNodeBuilder::child_of` | 部分 | `taffy_layout.rs:206` | 仅测试 |
| 57 | `roots_mut` | 部分 | `taffy_layout.rs:58` | 仅测试 |
| 58 | `root_mut` | 部分 | `taffy_layout.rs:63` | 仅测试 |
| 59 | `ToolbarLayout::add_contextual_menu` | 仍缺 | 无 | 无 |
| 60 | `contextual_menus` | 仍缺 | 无 | 无 |
| 61 | `link_up_cyclic_list_nav` | 仍缺 | 无 | 无 |
| 62 | `setup_screen_and_toolbar_nodes` | 仍缺 | 无 | 无 |

### 2.12 toolbar_startup（2）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 63 | `setup_toolbar` | 仍缺 | 无 | 无 |
| 64 | `toolbar_setup` | 仍缺 | 无 | 无 |

### 2.13 toolbar_update（1）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 65 | `toolbar_size_mult_update` | 仍缺 | `toolbar_update.rs` 仅 `ToolbarSizeMult` 资源 | 无 |

### 2.14 ui_interaction（10）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 66 | `sinc` | 真实实现 | `ui_interaction.rs:200` | `ui_locked_animation` 消费 |
| 67 | `sort_contextual_buttons_draw_order` | 仍缺 | 无 | 无 |
| 68 | `ui_appear_animation` | 仍缺 | 无 | 无 |
| 69 | `ui_focus_animation` | 仍缺 | 无 | 无 |
| 70 | `ui_focus_parent_if_self_in_focus` | 仍缺 | 无 | 无 |
| 71 | `ui_locked_animation` | 部分 | `ui_interaction.rs:242`；公式 O，命中插入 select 尾段 U | `toolbar_plugin.rs` set43 chain（crate 侧入口；宿主按用户指示未接） |
| 72 | `ui_notify_hints_if_toolbar_in_focus` | 仍缺 | 无 | 无 |
| 73 | `ui_select_animation` | 真实实现 | `ui_interaction.rs:217`；fresh `0x14233c300` 公式逐项成立 | `toolbar_plugin.rs:35-40` set43 chain（crate 侧入口；宿主按用户指示未接） |
| 74 | `ui_send_focus_begin_events` | 仍缺 | 无 | 无 |
| 75 | `ui_show_if_parent_in_focus` | 仍缺 | 无 | 无 |

### 2.15 同节 7 个空/平凡函数（82 的其余 7 个）

| # | 函数 | 分类 | 证据/位置 | 消费者/缺口 |
|---:|---|---|---|---|
| 76 | `NodeContent::is_empty` | 真实实现 | `core.rs:124` | `mei_post_update` 使用 |
| 77 | `ui_slider` | 仍缺 | `photomode.rs:20` 空壳（诚实标注） | 无 |
| 78 | `kind_handler` | 仍缺 | `toolbar_layout.rs:195` 空壳（诚实标注） | 无 |
| 79 | `switch_app_mode_button` | 真实实现 | `toolbar_button.rs:103`；fresh `0x142341fe0` 载荷/选中/动画判等成立 | `toolbar_plugin.rs:41` set43 chain（crate 侧入口；宿主按用户指示未接） |
| 80 | `taffy_mut_style` | 真实实现 | `toolbar_systems.rs:186`；被 `TaffyLayout::style_mut`（`taffy_layout.rs:91`）调用 | `perform_layout`/`taffy_finalize_element_size` 每帧消费 |
| 81 | `toolbar_alignment` | 部分 | `toolbar_systems.rs:222` 仅前置清位；`apply_alignment_setters` 未被调用；签名仍 helper | 无生产调用；case 4/6 U |
| 82 | `toolbar_auto_hide` | 仍缺 | `toolbar_systems.rs:259` 空壳（诚实标注） | 无 |

### 2.16 对账汇总

| 分类 | 数量 |
|---|---:|
| 真实实现（crate 侧可达 / 宿主未接） | 11 |
| 改名等价（底层生产消费 / 等价实现） | 12 |
| 部分（缺消费者/缺分支/仅测试/primitives-only） | 22 |
| 仍缺 | 37 |
| **合计** | **82** |

剩余缺口 = **59 项**（37 仍缺 + 22 部分）。其中契约 §2.15 的 7 个空/平凡函数中，
`ui_slider`/`kind_handler`/`toolbar_auto_hide` 仍缺，`toolbar_alignment` 仍部分。

> **快照口径**：本章计数锚定 TB 交付 17:43 版及 17:5x 的 `cargo check` 快照。复核期间
> （18:14-18:15）仍有并发写者修改 `mei/widgets.rs`、`mei/widgets/menu_button.rs`、
> `resouces_and_components/toolbar_layout.rs`（新增 `setup_screen_and_toolbar_nodes`/
> `add_contextual_menu`/`link_up_cyclic_list_nav` 等），这些**不计入本章 82 项计数**，
> 需在其作者交付后另行复核。

## 3. MEI 帧顺序核对

权威顺序（契约 §1.1/§1.2）与当前注册逐项对照：

| 契约阶段 | 契约内容 | 实际注册 | 结论 |
|---|---|---|---|
| S0 | Startup：`text_startup_system`、`toolbar_setup`（内含 `setup_photomode_toolbar`、`UndoRedoLayout::setup`） | **未实现**；`toolbar_setup`/`setup_photomode_toolbar` 均无符号 | 缺（契约已列） |
| S1 | set6 `mei_pre_update`（帧首交互，不是绘制后） | `tiny-glade/src/mei_host.rs:52-53` set6 | **一致** |
| S2 | set7 `update_text_renderer_scale` | `mei_host.rs:54-55` set7，`height/1000` | **一致** |
| S3 | set8..35 UI 生产者 | 仅 `photomode_camera_buttons` 在 `run_game.rs:395` set8；`menu::*::ui`、`toolbar`/`undo_redo` 布局生产者缺 | 部分；集合号未伪造 |
| S4 | set36 `splash::ui` | `game_loop.rs:342` set36 | **一致** |
| S5 | set8..43 交互/状态生产者 | `toolbar_plugin.rs:33-45`：set43 `.chain()` 8 系统（`render_taffy → animation_state → select → locked → keep_focus → switch_category → switch_app_mode → tod_v2`）；**宿主按用户指示不接**（`run_game.rs` 无该行，`lib.rs:56` 仅保留模块）。`toolbar_alignment`/`auto_hide`/`ui_sounds`/焦点/排序仍未注册 | 部分（crate 侧入口就绪；契约 §1.2 第 4 条仍缺） |
| S6 | set44 `prepare_text_fragments` | `mei_host.rs:56-63` 链首 | **一致** |
| S7 | set44 `taffy_finalize_element_size` | 同链第 2 | **一致** |
| S8 | set44 `taffy_perform_layout` | 同链第 3 | **一致** |
| S9 | set44 `taffy_update_drawables` | 同链第 4 | **一致** |
| S10 | set44 `debug_hide_ui` | 同链第 5 | **一致** |
| S11 | set44 `render_text_fragments` | 同链第 6 | **一致** |
| S12 | set44 `mei_post_update` | `mei_host.rs:64-67`，`.after(render_text_fragments)` | **一致** |
| S13 | `flush_mei_frame` | `mei_host.rs:66`，紧随 `mei_post_update` | **一致**（GPU 后置） |

结论：

1. **没有把“交互”放到“绘制之后”**。`mei_pre_update` 固定在 set6 帧首，`mei_post_update` 在 set44
   文本渲染之后；契约特别提醒的“与原引擎相反”的错误未出现。
2. S7-S11 的 `prepare_text_fragments → finalize → perform_layout → drawables → debug_hide → render_text`
   与契约 §1.1 的链式顺序逐项一致，`chain_ignore_deferred` 使用正确。
3. S5 在 **crate 侧**已就绪：`toolbar_plugin.rs` 在 set43 以 `.chain()` 固定
   `render_taffy → animation/button` 顺序，满足契约 §1.2 第 1-3 条；宿主按用户指示**不接**
   （`run_game.rs` 无 `add_plugins`，仅 `lib.rs:56` 保留模块），因此这些系统当前运行时不可达，
   属预期边界，不计实现缺陷。`toolbar_alignment → perform_layout`（第 4 条）仍未接线。

## 4. 关键结构核对

### 4.1 `taffy_finalize_element_size` / `taffy_update_drawables`

- `taffy_update.rs:18` `taffy_finalize_element_size`：仍是“单 Query + `TaffyNode._tail` 读 base/once”
  结构；契约 §4.4 判定原作遍历**两套映射**（`+0x108` 分支、stride 0xa0/0x48）并经 `taffy_mut_style`
   写样式，要求重验/重写。**本轮未改动、未补证**，仍为 I。
- `taffy_update.rs:63` `taffy_update_drawables`：仍是单 Query，`pos = location + size/2`；
  契约要求核对“两套映射 + `[center,origin]+u32`”。当前实现只满足“center=origin+size/2”这一条，
  **结构差异未闭合**。
- 二者都已被 `mei_host.rs:58-60` 接线（消费面真实），但算法结构不符合契约，不能计“真实实现”。

### 4.2 `toolbar::configure` / `register_local_components` 调用者

- `toolbar/src/lib.rs:77` `configure`：`grep -rn "toolbar::configure" crates` **仍零调用者**；
  新增的 `configure_app`（`lib.rs:113`）承担 App 注册面，并由 `toolbar_plugin::build` 调用。
- `resouces_and_components/mod.rs:662` `register_local_components`：**已由 `configure_app` 调用**
  （`lib.rs:141`），24 个本地组件进入注册面。
- `toolbar_plugin.rs:24-31` 的 `Plugin::build` → `toolbar::configure_app` + set43 8 系统；
  `crates/tiny-glade/src/lib.rs:56` 保留 `pub mod toolbar_plugin;`，但 `run_game.rs` **按用户指示
  不添加** `add_plugins(ToolbarPlugin)`。因此这是“crate 侧入口保留、宿主未接”的既定状态。
- 交付 §6.1 已如实说明“中央需要插入的一行”，口径正确。

### 4.3 本轮新增的关键类型

- `TaffyNode::default` 把 `+0x10` 的动画倍率初始化为 1.0（`mod.rs:41-52`），并新增
  `scale/set_scale`（`mod.rs:73-80`）；与 `ui_select_animation` 的乘法消费一致，属必要的 I 级修正。
- `UiPlaySelectAnimation`/`UiPlayLockedAnimation` 的 `+8` 由 `_pad_u32` 改为 `progress: f32`，
  与 fresh `0x14233c300` 的 `pfVar16[2]` 读写成对，**判等成立**。
- `PillButtonContent`/`back_button`/`lut_gallery::ui` 已在工作树落地但**未进入交付台账**；
  `lut_gallery::ui` 无 schedule 注册，`back_button` 的绘制常量无证据引用。

## 5. 跨 crate 与 T13

### 5.1 契约 §3.2 的 9 项中央报备缺口

| # | 契约缺口 | 交付是否登记 | 复核结论 |
|---:|---|---|---|
| 1 | `CurrentDaily`/`DailyThemeInfo` | **未登记** | 仍缺；daily_welcome 未实现 |
| 2 | `WelcomeScreenState`/`DailyWelcomeScreenState`/`FeedbackReportResources`/`ThirdPartyLicenseServer`/`LoadMenuThumbnailMetrics` | 仅 §4.1 提 `TaffyLayout.roots`，**未逐项登记** | `LoadMenuThumbnailMetrics` 已在 core（`menu/load.rs:12`），其余仍缺 |
| 3 | `SwitchAppModeButton` 16B 冲突 | 已登记并闭合（§1.5） | 与 fresh `0x142341fe0` 的 16B payload + 0x18 stride 一致 |
| 4 | `UiState::add` 键格式 / `WidgetParams` 字段身份 | **未登记**（`render_taffy_ui_to_widgets` 注释仍标 I） | 仍为 I |
| 5 | `UiNavState+0x10=0` 写面 | **未登记** | `render_taffy` 以 `focus.tag/_pad` 清零表达，判等 U |
| 6 | `EscapeMenuJustClosed` 发出点 | **未登记** | 未复核 |
| 7 | `PlayUiSound` 载荷映射 | §3.3 登记“截断/U” | 诚实但未推进 |
| 8 | `TaffyNode` 尾区 / `Ui2dExtent` 16B 归属 | §3.12 部分登记（scale O，其余 8B U） | 仍 U |
| 9 | `RequestAppModeChange` 冲突 | 已登记并裁定为 16B `AppMode`（§1.5） | core `mode_manager.rs:19-26` 注释一致；判等成立 |

**新增跨 crate 依赖未报备**：`resouces_and_components/mod.rs:186` 改为
`pub use country_core::components::UiInteractionState;`，即 toolbar 现在**依赖 core 新增的 3B 组件**
（`crates/country-core/src/components/ui.rs:71`，工作树已改）。交付 §5 仍称“未改 core、不改变依赖面”，
与事实不符；按 COORDINATION，这类跨界需求应先报中央。

### 5.2 T13

- `toolbar` 内无 `FeedbackReportResources` 捕获/状态生产者、无反馈网络链路、无 GPU 截图回读实现；
  交付 §5 未把 T13 计入验收。**排除执行正确**。

## 6. 编译与测试

运行环境：`CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-tb-review`（独立 target，不写源码；
按协调要求每个 agent 单独编译）。

| 命令 | 结果 | 说明 |
|---|---|---|
| `cargo check -p toolbar`（17:43 快照） | **exit 0**，`Finished dev profile ... in 5m 51s` | 覆盖 TB 收口版（`toolbar_plugin`/`configure_app`/`deserialize_optional`/`taffy_mut_style`/Signifier）；此前 16:19 的失败是 country-core（WC 组在途 `WallId`）阻塞，已由对方修复 |
| `cargo check -p tiny-glade`（含 `toolbar_plugin.rs`） | **exit 0**，`Finished dev profile ... in 3m 44s` | 未回退。该次复跑时 `run_game.rs` 曾含插件一行（18:01-18:45）；该行现已按用户要求撤回，`lib.rs:56` 的模块声明与 `toolbar_plugin.rs` 保留，编译面不变；未在撤回后重跑（按协调指示不再重跑全部测试） |
| `cargo test -p toolbar --lib` | **36 passed / 0 failed**（TB §6.5，target-agent-tb）；**本报告独立复跑未完成** | 独立复跑两次被 600s 前台超时截断在 toolbar 测试编译阶段（rustc 被终止），随后按协调指示“不再重跑全部测试”停止后台任务；因此 36 passed 采信 TB 记录，未被本报告独立证实 |

补充：

- 16:19 首次独立复跑时 `cargo check -p toolbar` 失败在
  `crates/country-core/src/resources/walls/public_wall_state.rs:271,294`（E0308，两个同名 `WallId`），
  属 WC 组并发编辑，不是 toolbar 代码错误；17:5x 复跑已 **exit 0**。
- `toolbar` 当前 `#[test]` 数量为 **42**（含 17:43 后并发写者新增的 `menu/credits.rs` 等），
  因此 36 passed 对应 17:43 快照；最新快照的测试数量会更多，需在其作者收口后重跑。
- `crates/systems/toolbar/tests/` 下另有集成测试（`original_menu_widget.rs` 等）需要
  `TG_INSTALL_ROOT` 字体资产，不属于 `--lib` 范围。

## 7. 缺口继承与交付诚实性

| 项 | 交付声明 | 工作树事实 | 判定 |
|---|---|---|---|
| `configure` 无调用者 | §1.8 明说；§6.1 改为新增 `ToolbarPlugin` + 宿主一行（按用户指示不接） | `configure_app` 由 `toolbar_plugin::build` 调用；`configure` 仍无调用者；`run_game.rs` 按用户指示不含插件 | 诚实（crate 侧入口保留，宿主不接为既定边界） |
| `ui_slider`/`ui_sounds`/`kind_handler`/`toolbar_auto_hide` 未实现 | 明说（§3） | 属实（空壳均带注释） | 诚实 |
| `toolbar_alignment`/`toolbar_auto_hide` 未完成 | 明说（§3.5） | 属实；`apply_alignment_setters` 未被系统调用 | 诚实（“已完成分派结构”偏乐观） |
| widgets 视图族（§3.8） | 声称 `PillButtonContent` 等“当前 crate 不存在” | 工作树已存在，但 §5.1/§6.4 明确标为并发写者、不认领 | **已澄清**；§3.8 旧文需同步删除 |
| `lut_gallery`/`load_ui`/`menu_button` | §6.4 标为并发写者、不认领 | 属实（仅有测试/未注册） | 诚实 |
| `photomode_camera_buttons` | §3.6 称“上一轮” | 在未提交 diff 中；无生产者 spawn `PhotomodeButton` | 归属待确认（不影响判定） |
| `deserialize_optional` 已完成 | §1.6 + §6.2 | 已接 `LabelConfig` 6 字段，真实反序列化消费 | **口径成立** |
| `taffy_mut_style` | §6.2 | 已接 `TaffyLayout::style_mut`，`perform_layout` 每帧消费 | **口径成立** |
| Signifier 0x20B | §6.3 | `SignifierFactory::new + write_factory` 真实入流 | **口径成立** |
| `value_to_param`/Taffy builder/`roots_mut` | §6.2 标 primitives-only U | 仅测试消费 | 诚实（不计完成） |
| 跨 crate | §5 称未改 core | toolbar 仍 `pub use country_core::components::UiInteractionState`（`mod.rs:186`） | **需补报依赖面** |

结论：17:43 版交付已修正此前的 scope 漏报与“无消费者也算完成”的高估，**总体诚实**。
剩余台账问题只有：§3.8 旧文与 §5.1/§6.4 自相矛盾（需删旧文）、跨 crate 依赖面未报。

## 8. 必须修复清单（按优先级）

### 前置（用户/中央决策，不计 TB 缺陷）

0. **宿主接线按用户指示不接**：`run_game.rs` 当前无 `ToolbarPlugin`；`lib.rs:56` 保留
   `pub mod toolbar_plugin;`，`ToolbarPlugin` 作为 crate 侧入口保留。后续若用户决定接入，
   须保持在 core 资源（`GameConfig`/`UiState`/`CursorPositionSS`）之后。

### P0（阻断“无条件通过”）

1. **`toolbar_alignment` 接回分派**：`toolbar_systems.rs:222` 现完全忽略 `_taffy` 且不调用
   `apply_alignment_setters`；至少在 helper 签名下调用已恢复的 0..3/5/7/9/10/11 case，
   并保留 4/6 U。当前该函数只做前置清位，不能计“部分以上”。
2. **台账自相矛盾**：交付 §3.8 仍称 `PillButtonContent`/`back_button` 不存在，与 §5.1/§6.4
   及工作树冲突；删除/改写旧段。
3. **跨 crate 报备**：登记 toolbar 对 `country_core::components::UiInteractionState` 的依赖
   （`mod.rs:186`），并按契约 §3.2 逐项补报 1/2/4/5/6/8。

### P1（结构正确性）

4. **重验两套映射**：`taffy_finalize_element_size`/`taffy_update_drawables` 按契约 §4.4
   重写或补 fresh 反编译证伪“两套映射”；当前单 Query 结构不能计 O。
5. **`toolbar_alignment` 签名**：改回原生 `Res<GameConfig>/Res<ToolbarLayout>/
   Res<PhotomodeToolbarLayout>/ResMut<TaffyLayout>` SystemParam。
6. **`PhotomodeButton` 生产者**：`photomode_camera_buttons` 已注册但无实体 spawn；
   在 `setup_photomode_toolbar` 落地前该链无输入。

### P2（证据）

7. `PillButtonContent` 的绘制常量（圆角 10、alpha `0xffffff40/0x20`、0x48 布局）补 fresh
   反编译引用，或降为 U 并移除未证实常量（非 TB 作者，需其作者处理）。
8. `photomode_camera_buttons` 的 tag→`PhotoModeCmd` 映射补 `0x14234c230` 跳表证据。
9. `back_button` 的 hash 常量与 style 写面补 PDB/反编译引用。

## 9. 给 TB① 的下一步方向

1. **宿主接线被用户撤销，crate 侧入口保留**：`ToolbarPlugin` 已在 `lib.rs` 暴露但未 `add_plugins`；
   若用户后续决定接入，先验 `render_taffy → UiClickEvent → 按钮系统` 的端到端回归。
2. **S0 建树仍是最高价值缺口**：`toolbar_setup`/`setup_toolbar`/`setup_screen_and_toolbar_nodes`/
   `add_contextual_menu`/`link_up_cyclic_list_nav` 未落，导致 `TaffyLayout.roots/anchored` 无生产者，
   `taffy_finalize_element_size`/`toolbar_alignment` 也失去输入。建议下一轮以 S0 为第一目标。
3. **剩余 37 项仍缺 + 22 项部分**：大头是 credits/welcome/new_session/report_bug 表单、
   `text_startup`、`setup_photomode_toolbar`、`ui_slider`、焦点/排序系统；
   先把 `menu/load_ui.rs` 的 `display_error`/`contextual` 与现有 `ThumbnailScrollContent`
   拆成契约 #38-#44 命名面并接入生产 schedule。
4. **primitives-only 项收敛**：`TaffyNodeBuilder` 全族、`roots/root_mut`、`value_to_param`、
   `get_content_impl`/`into_child` 在 S0/滑块落地时立即迁移真实调用点，避免长期孤儿。
5. **证据与台账同步**：每个函数登记“位置 / 证据 RVA / 生产消费者 / 注册点”；新增 API
   必须同时给出调用点或明确标“库面预留（不计完成）”。

