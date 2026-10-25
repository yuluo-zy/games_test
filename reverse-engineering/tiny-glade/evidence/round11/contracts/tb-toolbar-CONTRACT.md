# TB 组 ② 可执行契约：`crates/systems/toolbar`

> 第 11 轮（2026-10-19）逆向取证产出。只读源码/证据，不改源码；本文件是唯一交付。
> 目标 crate：`D:/game/tiny-glade/crates/systems/toolbar`（当前 18 文件 / 5,325 行）。
> 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/systems/toolbar/src/**`（PDB 元数据：`// 函数符号` / `// 源码行` / `编译实例 RVAs` / 依赖注解）。
> 二进制：B patched EXE，sha256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`；VA = `0x140000000 + RVA`。
> 既有证据：`upper-modules-plan/systems/toolbar/ANALYSIS.md`（A/B/C）、`upper-modules-plan/BATCH3-LOGIC-REVIEW-toolbar.md`、`upper-modules-plan/STATUS-2026-10-19.md`、`reports/{wave2,wave2b,wave3,mei}/*`。
> 已完成交叉核对：`tiny-glade/src/mei_host.rs:39-68`（共享 MEI 帧集合序）、`tiny-glade/src/game_loop.rs:336`（splash set36）。
> T13 反馈捕获按 `AGENTS.md` 明确排除：本契约不列 capture/state 生产者，也不把其缺口计入验收。

## 0. 口径、计数与置信度

- **O**：已有反汇编/反编译逐指令或字节证据，可直接按表实现。
- **I**：控制流/常量已定位，但字段语义、闭包目标或 case 体未逐语句闭合。
- **U**：仅知 ABI/布局/上位调用，业务语义未知；实现必须显式登记 unknown，不得填默认值。
- **PHYS**：B 有物理 `compiled_procedure`（有 RVA），可 `get_functions` 定点反编译。
- **INLINE**：B 仅 `inline_instance, inlinee_declaration`（无物理函数），来源行由 PDB `源码行` 保留；按普通 Rust 实现，深挖其宿主物理函数。
- **改名/合并**：原作函数在现有 crate 已有等价符号或已被拆成两个函数；需判等后补回原名/等价封装。
- **真缺失**：当前 crate 无同名、无等价实现。

### 0.1 计数对账（重要）

`STATUS-2026-10-19.md:20` 表列 `systems/toolbar` “完全没迹象 **82**”，但同文件 §`systems/toolbar` 的明细清单实际枚举 **75** 个符号：

`2 (demo) + 2 (config) + 16 (core) + 5 (layout) + 8 (widgets) + 3 (credits) + 8 (load) + 1 (new_session) + 4 (report_bug) + 1 (photomode_buttons) + 1 (setup_photomode_toolbar) + 2 (ui_slider) + 1 (LogSlider) + 4 (taffy_layout) + 4 (toolbar_layout) + 2 (toolbar_startup) + 1 (toolbar_update) + 10 (ui_interaction) = 75`。

**82 = 75 个“无迹象” + 同节 7 个“空/平凡函数体”**（`is_empty`、`ui_slider`、`kind_handler`、`switch_app_mode_button`、`taffy_mut_style`、`toolbar_alignment`、`toolbar_auto_hide`）。本契约 §2 逐条列出全部 **82**（75 + 7），不把 7 个空壳当作已覆盖。

### 0.2 文件面差异（② 必须先补齐模块骨架）

当前 crate 只有 18 文件；骨架 38 文件。**完全缺失的源文件**（需按 `skeleton/B` 新建）：

- `mei/audio.rs`、`mei/widgets/decorate.rs`、`mei/widgets/tweaker.rs`
- `menu/{corrupted_save_file,credits,daily_welcome,load,new_session,report_bug,save_error,welcome}.rs`
- `photomode/{lut_gallery,photomode_buttons,setup_photomode_toolbar,ui_slider}.rs`
- `resouces_and_components/{taffy_utils,undo_redo_layout}.rs`
- `text.rs`、`third_party_licenses_server.rs`、`toolbar_startup.rs`、`demo_stuff/*.rs`

当前合并/改名的文件：`mei/systems.rs` 承载原作 `mei_pre_update`/`mei_post_update`；`photomode.rs` 仅 19 行占位。

**接线现状（2026-10-19 复核）**：

- `toolbar::configure`（`lib.rs:77`）在 workspace 内**无调用者**；其中 4 个系统与事件/组件注册实际未进入 App。
- `resouces_and_components::register_local_components`（`mod.rs:630`）**无调用者**；24 个本地组件未注册。
- 唯一真实接线是 `tiny-glade/src/mei_host.rs:43-68` 的 `install_mei_frame_systems`（set6/7/44）与 `game_loop.rs:336` 的 `splash::ui`（set36）。
- 因此 §2 的绝大多数函数即使实现，也必须由 §6 第 17 步接入 `ToolbarPlugin::build`/`Ui2dPlugin::build` 的等价入口，否则无消费者，不通过验收。

### 0.3 已证伪/已修正的旧断言

- `new_at_pos<String>`（`0x142289f80`，268B）**有物理函数**，与 `<&str>` 同构；仅 `NodeContext::mei`（0x885d0）无物理函数（BATCH3 §4.1）。
- `ui_sounds`（`0x14233a7f0`）尾部并入 `drop_ptr<...>`/组件索引尾块，反编译边界错并（BATCH3 §2/4.7）。
- `toolbar_update.rs` 模块头“`taffy_update_drawables`/`taffy_finalize_element_size` 仍未实现”是 **stale**：二者已在 `taffy_update.rs:18-75` 落库并由 `mei_host.rs:58-60` 接线；真实性另行按 §4 判等。
- `STATUS` 的 `taffy_mut_style` 位置写 `toolbar_systems.rs:186`；骨架归属为 `resouces_and_components/taffy_utils.rs:23`（`0x2375b40..0x2376300` 十单态）。

### 0.4 现场字节核对（2026-10-19，`scripts/rne-read.py --hex`）

以下 PHYS 地址均返回合法函数序言（非数据/空洞），可作 ② 的 fresh 反编译起点：

`0x142345890`（toolbar_setup）、`0x14234c230`（photomode_buttons）、`0x1426246e0`（setup_photomode_toolbar）、`0x142340890`（taffy_finalize_element_size）、`0x1423486d0`（ui_slider）、`0x14233ce00`（ui_show_if_parent_in_focus）、`0x14233a170`、`0x14233c8b0`、`0x14233c300`、`0x14233ba40`、`0x14233e760`、`0x141e4d1e0`、`0x141e4ccb0`、`0x141e23b90`、`0x141e23d20`、`0x141e1e5f0`、`0x141dfb410`、`0x141eac540`、`0x14234a410`、`0x142343a10`、`0x142344560`；核心宿主 `0x140f99230`（safe_frame）、`0x140f992e0`（new_at_pos<&str>）、`0x140683ec0`（reuse_or_create_node）、`0x142289f80`（new_at_pos<String>）。

未在本次现场复核、仅沿用 ANALYSIS/BATCH3 证据的项（② 第一步须补）：`0x1406884c0`（mei_post_update）、`0x1406842e0`（mei_pre_update）、`0x141e4dc40`（add_toolbar_item）、`0x141eaec50`（toolbar_button_size_and_padding_update）、`0x142338990`（render_taffy_ui_to_widgets）、`0x14233a7f0`（ui_sounds）、`0x142347f90`（ui_slider_reset）、`0x142349de0`（slider_val_01）、`0x14234b010`（do_photomode）、`0x142383730`（param_to_value）。

---

## 1. MEI / toolbar 帧顺序契约（布局 → 绘制 → 交互 → 状态 → 文本/拍照）

### 1.1 权威调度顺序（O：`tiny-glade/src/mei_host.rs:39-68`）

`GameSystemSet(n)` 按 n 升序串行（`run_game.rs:258`）。共享 MEI 帧：

| 序 | Set | 系统 | 输入（权威） | 输出（真实消费者） | 级别 |
|---:|---:|---|---|---|---|
| S0 | Startup / `OnEnter(AppLoadingState)` | `text_startup_system`（0x141e9ce50）、`toolbar_setup`（0x142345890；其内部依次调 `setup_photomode_toolbar` 0x1426246e0 与 `UndoRedoLayout::setup` 0x140fad870） | `GameConfig`、字体目录、静态分类表、`Taffy` | `TextRenderer` 字体表；`TaffyLayout`/`ToolbarLayout`/`PhotomodeToolbarLayout`/`UndoRedoLayout`；taffy 节点树 + `AnchoredNode` 表 + spawn 的按钮实体 + `UiNav` 链接 | O/I |
| S1 | **6** | `mei_pre_update`（0x1406842e0） | 上一帧节点表、`MouseWheel`、`WindowSize`、`Time`、`ActionSetStates`、`UiNavState`、`Option<UiState>`（ResMut，注册面）、`CursorPositionSS` | 节点 `state_bits`（hover/drag/click/focus）、下一帧 `MeiInputEvent`、`UiState::add` 条目、`PlayMenuUiSound`、节点表双缓冲交换/GC | O（主体） |
| S2 | **7** | `update_text_renderer_scale`（0x141e9d960） | `WindowSize` | `TextRenderer.pixels_per_point = height/1000` | O |
| S3 | 8..35（原集合号 U） | UI 生产者：各 `menu::*::ui`、`photomode::*`、`toolbar`/`undo_redo` 布局、按钮/滑块 | 菜单/拍照/会话状态、事件、`MeiContext`、资源 | `safe_frame`/`new_at_pos`/`child` 节点、`TextFragment`、taffy 实体与组件 | 部分 O，集合号 U |
| S4 | 36 | `splash::ui`（0x141ec1390） | `LogoAnimation`、`GameClock` | splash 节点 + iced Event | O（接线），主体 I |
| S5 | 8..43（U） | 交互/状态生产者：`render_taffy_ui_to_widgets`（0x142338990）、`ui_sounds`（0x14233a7f0）、`ui_animation_state_update`（0x14233b3f0）、4×`ui_*_animation`、焦点传播 4 系统、`sort_contextual_buttons_draw_order`、`toolbar_button_*`、`toolbar_alignment`、`toolbar_auto_hide`、`undo_redo_*`、`toolbar_steal_mouse_input` | `Query<...TaffyNode...>`、`UiState`、`UiNavState`、`ActionSetStates`、`CursorPositionSS`、`GameConfig`、`ToolbarSizeMult` | `UiState` 注册（下一帧 S1 消费）、`UiClickEvent`/`UiDragEvent`/`UiHistoryButtonPressed`、`UiPlaySelectAnimation`/`UiPlayLockedAnimation`、`UiAnimationState`/`UiInteractionState`、taffy Style 改写的重排、`WindowFocusEvent`、`WriteSettingsFileCmd`、`PlayUiSound` | 部分 O/I |
| S6 | **44**（chain_ignore_deferred） | `prepare_text_fragments`（0x141e9dd20） | `TextFragment` 组件、`TextRenderer` | `TextRenderer` 字形/atlas 脏缓冲 | O |
| S7 | 44 | `taffy_finalize_element_size`（0x142340890） | `TaffyNode` 尾区、`UiScaleWithUserSetting`、`TextFragment::bounds`、`GameConfig.ui.icon_size_mult`、`ToolbarSizeMult` | 写 taffy `Style.size`；一帧倍率复位 1.0 | I |
| S8 | 44 | `taffy_perform_layout`（原内联于 0x141e9eb40；`TaffyLayout::perform_layout` 0x141e4c6c0） | `TaffyLayout.anchored`、`WindowSize`、`GameConfig`、`ToolbarSizeMult` | taffy 计算布局 + `make_layout_absolute` 绝对化 | O |
| S9 | 44 | `taffy_update_drawables`（0x142340d20） | taffy layout | `Ui2dExtent{pos=origin+size/2,size}` → `ui_2d_pass` | I |
| S10 | 44 | `debug_hide_ui`（0x14233e960） | `DebugGui.show_2d_ui`、`UiMouseInteractable` | `Ui2dExtent` 置零 → `ui_2d_pass` | O |
| S11 | 44 | `render_text_fragments`（0x141e9df60） | `TextFragment` + `Ui2dExtent` | `TextRenderer` 的 TextPrim/ShapeStream 顶点流 | O |
| S12 | 44 | `mei_post_update`（0x1406884c0） | `MeiContext` 节点表、`TextRenderer`、`UiNavState`、`AssetTextureLibrary`、`WindowSize`、输入模式 | 每节点 `content.prepare_layout`（vtable+0x60，写 intrinsic size）→ MEI 盒模型 `perform_layout` → 焦点位/`update_nav` → 按 `Node+0xac` 排序 → `content.draw`（vtable+0x68，写 ShapeStream）→ 焦点矩形 → 清帧事件/状态 | O（主体），焦点矩形 I |
| S13 | 44 | `flush_mei_frame`（`MeiHost`，对应 0x141e9e1f0） | TextPrim/ShapeStream | GPU buffer `TextPrimSsbo`/`ShapeStreamSsbo` → `ui_2d_pass` | **GPU 后置** |

### 1.2 请求的五类阶段 → 实际引擎映射（② 不要颠倒）

| 需求阶段 | 实际所在 | 权威输入 | 权威输出 | 消费者 |
|---|---|---|---|---|
| 布局（taffy） | S0 建树；S7-S9 定稿/布局/桥接 | `ToolbarItem` 树、`AnchoredNode`、`GameConfig`、`TextFragment` | taffy `Style`/layout、`Ui2dExtent`、`TaffyLayout.anchored` | S5 对齐/自动隐藏、`ui_2d_pass` |
| 绘制（draw） | S12（MEI `content.draw`）；S11 文本顶点；S13 上传 | 节点 `pos/size/opacity/z`、`TextFragment` | `ShapeStream`/`TextPrim` 命令流 | `ui_2d_pass`（GPU 后置） |
| 交互（hover/click/drag） | **S1（帧首）**，不是绘制之后 | `UiState` 注册面（上帧条目由其它系统消费）+ 本帧光标/动作 | 节点 state bits、`UiState::add`、`PlayMenuUiSound`、`MeiInputEvent` | 本帧 UI 内容 `on_input`/`process_input_action`、S5 按钮/动画系统 |
| 工具栏/菜单状态更新 | S3 生产者 + S5 系统 | `EscMenuState`/`AppMode`/`ModeJustChanged`/点击事件 | 菜单节点、`RequestAppModeChange`/`RequestToolbarCategoryChange`、动画/焦点事件、`WriteSettingsFileCmd` | `country-core` mode_manager、settings、history、audio |
| 文本/拍照 UI | S2/S6/S11 文本；S3 `photomode::*` + S5 `ui_slider` | 字体、`TextFragment`、`PhotomodeState`/`ImageFormationConfigs`/`BrushSizes` | 文本顶点、拍照节点/滑块实体、相机参数写入 | `ui_2d_pass`、`MainCamera`、渲染配置 |

**关键顺序约束（O/U）**：

1. `render_taffy_ui_to_widgets`（UiState 生产者）必须在 `mei_pre_update` 消费之前完成上一帧注册；原集合号 U（未在 `mei_host` 注册）。数据依赖要求：taffy layout（S8）之后、下一帧 S1 之前。
2. `ui_animation_state_update` 读 `UiInteractionState+1`（keep），该字节由 `render_taffy_ui_to_widgets` 本帧写；二者同帧时需 `render_taffy → animation` 顺序（实际未接线）。
3. `toolbar_button_*` 读 `UiClickEvent`，该事件由 `render_taffy_ui_to_widgets` 尾段写；同帧顺序同 2。
4. `toolbar_alignment`/`toolbar_auto_hide` 改 taffy Style 后必须重跑 S8；实际未接线。
5. `prepare_text_fragments` 在 `taffy_finalize_element_size` 前（文本度量供定稿）；`render_text_fragments` 在 `mei_post_update` 前（原生 set44 chain）。

### 1.3 帧数据流总链

```
S0 toolbar_setup/setup_photomode_toolbar/UndoRedoLayout::setup/text_startup
  └─ TaffyLayout{taffy, anchored} + ToolbarLayout/PhotomodeToolbarLayout/UndoRedoLayout
  └─ spawn(TaffyNode, UiScaleWithUserSetting, UiInteractionState, UiAnimationState,
           UiMouseInteractable, Ui2dExtent, Ui2dImage, UiDrawOrder)
  └─ link_up_nav: UiNav{prev,next} 环形；HistoryButton{undo/redo}
S1 mei_pre_update:
  MouseWheel+ActionSetStates+UiNavState+UiState(注册面) → handler 派发 → pre_update
  → hover/click/drag 状态位 → UiState::add(WidgetParams) → PlayMenuUiSound
  → MeiInputEvent 队列（下一帧）→ 双缓冲换表/GC
S2/S6 text scale + prepare
S7-S9 TextFragment::bounds → taffy Style size → compute_layout → absolute → Ui2dExtent
S10 debug hide
S11 text render → TextPrim/ShapeStream
S12 mei_post_update:
  content.prepare_layout → MEI root/child 盒模型 → update_nav/焦点 → 按 z 排序
  → content.draw → 焦点矩形 → 清帧事件/state
S13 flush → ui_2d_pass(GPU)
```

---

## 2. 82 个无迹象/空壳函数逐个契约

> `骨架` 列 = `skeleton/B/crates/systems/toolbar/src/<file>:<PDB 源码行>`。
> `地址/形式` 列：`0x…` 为 B 物理 RVA（VA=+0x140000000）；`INLINE` 表示无物理函数。
> `判定`：内联 / 物理 / 改名合并 / 真缺失。
> `深挖入口`：PHYS 用 `re_mcp_ghidra get_functions` 定点反编译；INLINE 列宿主 VA（由 PDB 依赖注解/ANALYSIS 导出）。
> 验收统一口径：**真实输出必须进入表内“消费者”列的真实系统/资源**，不得 `let _ =`、空体、恒 0 或伪造事件。

### 2.1 demo_stuff（2）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 1 | `demo_stuff/demo_show_full_glade_size.rs:35` `demo_show_full_glade_size` | `0x1eac540` PHYS | 真缺失 | I | 周界虚线 Signifier 实体 + MEI 半径标签节点（137.88582m、y-30/-50）→ `SignifierEntityBuilder`/`ui_2d_pass` | `get_functions 0x1401eac540`；ANALYSIS B12.1；closure `0x1ead870` |
| 2 | `demo_stuff/demo_show_max_elevation.rs:19` `demo_show_max_elevation` | `0x234a410` PHYS | 真缺失 | I | 3 条 DebugLine 十字 + 最高点 MEI 标签 → `DebugLine`/`ui_2d_pass` | `get_functions 0x14234a410`；ANALYSIS B12.3 |

### 2.2 mei::config（2）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 3 | `mei/config.rs:152` `deserialize_optional` | INLINE（19 实例） | 内联/真缺失 | O（依赖注解） | 反序列化 `Option<T>` 字段；输出 `NodeConfig`/`UiTemplate` 可选字段 → `read_node_config_from_file`（`mei/config.rs:196`） | 宿主 `read_node_config_from_file 0x14068fe40`、`instantiate_ui_template_impl 0x14068eb50` |
| 4 | `mei/config.rs:352` `localize_ui_text_from_config` | INLINE（2） | 内联/真缺失 | I（`format!`+`str::find/index`+`push_str`） | 本地化文本 `String` → `LabelContent.set_text`；消费者 `instantiate_ui_template_impl`（`mei/config.rs:242`）/菜单模板 | 宿主 `instantiate_ui_template_impl 0x14068eb50` |

### 2.3 mei::core（16）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 5 | `mei/core.rs:1302` `NodeContext::child_with_content` | INLINE（119） | 合并候选（`child_with_global_id` 905 + `get_content_mut_impl` 882） | O | 子节点 `NodeContext` + 已初始化内容 → 所有 widget builder / `instantiate_ui_template_impl` | 宿主 label 0x68bf00 / menu_button 0x68bfc0 / template 0x14068eb50 |
| 6 | `mei/core.rs:739` `MeiContext::clear_state` | INLINE（2） | 改名合并→`frame_begin`（`mei/core.rs:796`），需补 handler 表/状态清理 | I | 帧首干净状态 → `mei_pre_update` | 宿主 `mei_pre_update 0x1406842e0` |
| 7 | `mei/core.rs:1446` `NodeContext::content_mut` | INLINE（223）；物理单态 FrameContent `0x32d500` | 改名合并→`get_content_mut_impl`（882） | O | `&mut T` 内容 → 所有 View/content trait | 宿主 builders；`0x140032d500` |
| 8 | `mei/core.rs:1654` `RenderContext::draw_rect` | INLINE（10） | 真缺失 | I | 矩形 `ShapeStream` 命令（fill/radius/center/extent）→ `FrameContent::draw`（`mei/widgets.rs:75`）/`ui_2d_pass` | 宿主 `FrameView::draw 0x6803f0`、`mei_post_update 0x1406884c0` |
| 9 | `mei/core.rs:1752` `find_node_path` | INLINE（2） | 真缺失 | I | root→focus 路径 `Vec<GlobalId>` → `update_nav`/焦点传播 | 宿主 `ui_nav_update 0x14233eed0`、`mei_post_update 0x1406884c0` |
| 10 | `mei/core.rs:439` `MeiContext::fullscreen_without_block_input` | INLINE（4） | 真缺失 | I | 全屏 `safe_frame`（不阻塞输入）节点 → 菜单覆盖层 | 宿主 welcome 0x141ebe130 / splash 0x141ec1390 / load 0x141e1fe80 / corrupted 0x141ebfbf0 / save_error 0x141ec0ce0 |
| 11 | `mei/core.rs:1494` `NodeContext::get_content_impl` | INLINE（12） | 改名合并→`get_content_mut_impl`（882，`require_existing` 形参） | O | 已有/新建内容引用 → `content_mut`/View | 宿主 `content_mut`；`NodeContext` 系列 |
| 12 | `mei/core.rs:1338` `NodeContext::into_child` | INLINE（4） | 改名合并→`into_child_with_global_id`（922） | O | 子节点 + 父 `children` append → template/菜单 | 宿主 `child_with_global_id 0x1406861b0` |
| 13 | `mei/core.rs:1692` `load_fonts` | INLINE（5） | 合并→`load_font_table`（711）/`text_startup_system` | O | 字体枚举加载 → `TextRenderer` 字体表 | 宿主 `text_startup_system 0x141e9ce50`；`tiny-glade/src/mei_host.rs:96-115`（O） |
| 14 | `mei/core.rs:321` `Node::lose_focus_on_invalid_nav` | INLINE（5） | 真缺失 | I | 无效导航时清焦点位/焦点表 → `update_nav` | 宿主 `mei_post_update 0x1406884c0`/`ui_nav_update 0x14233eed0` |
| 15 | `mei/core.rs:494` `MeiContext::new_root` | INLINE（85） | 改名合并→`safe_frame`（504） | O | 根节点 `NodeContext` + tag1 帧事件 → UI 生产者 | 宿主 `safe_frame 0x140f99230` |
| 16 | `mei/core.rs:836` `MeiContext::prepare_content` | INLINE（2） | 合并→`mei_post_update` 内联循环（`mei/systems.rs:381-411`） | I | 每节点 `prepare_layout` 返回值写 intrinsic size → `perform_layout` | 宿主 `mei_post_update 0x1406884c0` |
| 17 | `mei/core.rs:281` `Node::reset_keeping_content` | INLINE（2） | 改名合并→`recycle_frame_nodes`（554） | O | 保留内容/几何、清状态 → 节点复用 | 宿主 `reuse_or_create_node 0x140683ec0` |
| 18 | `mei/core.rs:316` `Node::steal_focus` | INLINE（4） | 真缺失 | I | 写 `UiNavState.focus = Mei(self)` → 导航/点击 | 宿主 `mei_post_update 0x1406884c0`（update_nav） |
| 19 | `mei/core.rs:1764` `update_parent_mapping` | INLINE（2） | 改名合并→`build_parent_links`（803） | O | `HashMap<GlobalId,GlobalId>` → `process_input_action_handlers`/焦点上溯 | 宿主 `mei_pre_update 0x1406842e0` |
| 20 | `mei/core.rs:1406` `NodeContext::with_layout` | INLINE（41）；物理单态 `0xf9c3b0` | 改名合并→`layout`（955） | O | 写 `Node+0x18..+0x60` 样式并返回 context → builders | 宿主 builders；`0x1400f9c3b0`（report_bug size closure） |

### 2.4 mei::layout（5）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 21 | `mei/layout.rs:336` `LayoutStyle::align_center` | `0x68e960` PHYS | 真缺失 | O | `justify=1` 的 LayoutStyle → `place_childen` 分派 | `get_functions 0x140068e960` |
| 22 | `mei/layout.rs:386` `LengthPercentage::points_or_zero`（及 Size/Rect 变体） | INLINE（11/6/2） | 真缺失 | O | 百分比/点值→f32（非点值 0）→ `compute_min_size_with_children`/`place_childen` | 宿主 `0x140fe0e10`/`0x140fe12a0`/`0x140fe19a0` |
| 23 | `mei/layout.rs:116` `Rect::size_vec2` | INLINE（4） | 真缺失 | O | `Vec2(width,height)` → `ComputedLayout` | 宿主 `place_childen`/`Node` |
| 24 | `mei/layout.rs:326` `LayoutStyle::space_between` | `0x68e930` PHYS | 真缺失 | O | `justify=3` → `place_childen` space_between 分支 | `get_functions 0x140068e930` |
| 25 | `mei/layout.rs:353` `LayoutStyle::xalign_center` | `0x68e990` PHYS | 真缺失 | O | `align=2` → 子项交叉轴居中 | `get_functions 0x140068e990` |

### 2.5 mei::widgets（8）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 26 | `mei/widgets.rs:992` `back_button` | `0x6811c0` PHYS | 真缺失 | I | `pill_button` 节点 + `block_input/ignore_input` 标志 → 菜单返回按钮 | `get_functions 0x14006811c0` |
| 27 | `mei/widgets.rs:517` `ImageView::blur_extend` | `0x67eaf0` PHYS | 真缺失 | O | `ImageContent+0x44=true` → 图像 blur 绘制 | `get_functions 0x140067eaf0` |
| 28 | `mei/widgets.rs:210` `content_mut`（Frame/Image/Label/MenuButton/Pill/Scroll/Thumbnail/TextEdit 共享 trait 默认） | INLINE（各 2..133） | 改名合并→`NodeContext::content_mut`/`get_content_mut_impl` | O | `&mut T` 内容 → 各 View 设置器 | 宿主 `NodeContext::content_mut 0x140068?`/`0x32d500` |
| 29 | `mei/widgets.rs:379` `MenuButtonView::force_highlight` | `0x67e9e0` PHYS | 真缺失 | O | `MenuButtonContent` 高亮标志 → 菜单按钮绘制 | `get_functions 0x140067e9e0` |
| 30 | `mei/widgets.rs:333` `MenuButtonView::hotkey` / `:1032` `PillButtonView::hotkey` | `0x67e6d0` / `0x6816c0` PHYS | 真缺失 | O | 热键字节（MenuButton `+0x25d`、Pill `+0x44`）→ tooltip/提示 | `get_functions 0x140067e6d0,0x14006816c0` |
| 31 | `mei/widgets.rs:1013` `PillButtonView::inactive_opacity` | `0x6814f0` PHYS | 真缺失 | O | `PillButtonContent+0x40` f32 → pill 绘制透明度 | `get_functions 0x14006814f0` |
| 32 | `mei/widgets.rs:386` `make_hotkey_tooltip` | INLINE（3） | 真缺失 | I | `new_relative` + `with_node_mut` 子 tooltip 节点 → menu_button | 宿主 `menu_button 0x68bfc0`/`0x68c260` |
| 33 | `mei/widgets.rs:1006` `PillButtonView::mute_click` | INLINE（2） | 真缺失 | I | Pill 点击静音标志 → `pill_button` 构建 | 宿主 `pill_button 0x32e5d0` |

### 2.6 menu::credits（3）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 34 | `mei/widgets.rs:210`（`credits::ScrollView::content_mut`） | INLINE（3） | 合并→共享 `content_mut` | O | `ScrollContent` 引用 → credits 滚动视图 | 宿主 `credits::ui 0x141dfa070`/`scroll 0x140fa04f0` |
| 35 | `menu/credits.rs:309` `ScrollView::max_scroll_pos` | INLINE（2） | 真缺失 | I | 滚动上限 `(max-min)-450.0`（ANALYSIS C9）→ `Local<f32>`/滚动 | 宿主 `credits::ui 0x141dfa070` |
| 36 | `menu/credits.rs:238` `spawn_third_party_license_http_server` | `0x1dfb410` PHYS | 真缺失 | O | 绑定 127.0.0.1:8123、spawn `SingleFileServer::run` 线程、`Local<Option<Server>>`/错误串 | `get_functions 0x141dfb410` |

### 2.7 menu::load（8）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 37 | `mei/widgets.rs:210`（`ThumbnailScrollView::content_mut`） | INLINE（12） | 合并→共享 `content_mut` | O | `ThumbnailScrollContent` 引用 → load UI | 宿主 `load::ui 0x141e1fe80` |
| 38 | `menu/load.rs:1080` `display_save_file_load_error` | `0x1e23b90` PHYS | 真缺失 | O | 错误模板节点（标题/详情 slot）→ load 菜单 | `get_functions 0x141e23b90` |
| 39 | `menu/load.rs:133` `hovered_item_pos` | INLINE（2） | 真缺失 | I | 当前 hovered 缩略图索引 `Option<usize>` → 缩略图滚动/上下文按钮 | 宿主 `load::ui 0x141e1fe80` |
| 40 | `menu/load.rs:145` `initial_focused_item` | INLINE（4） | 真缺失 | I | 初始焦点项索引 → `ThumbnailScrollContent` | 宿主 `thumbnail_scroll 0x140fa06a0` |
| 41 | `menu/load.rs:191` `min_scroll_pos` | INLINE（2） | 真缺失 | I | 最小滚动位 → 滚动范围 | 宿主 `load::ui 0x141e1fe80`/`thumbnail_scroll` |
| 42 | `menu/load.rs:197` `nav_scroll_speed` | INLINE（4） | 真缺失 | I | 导航滚动速度 → 滚动动画 | 宿主 `thumbnail_scroll 0x140fa06a0` |
| 43 | `menu/load.rs:165` `run_count` | INLINE（2） | 真缺失 | I | 连续存档项计数 → 分页/滚动 | 宿主 `thumbnail_scroll` |
| 44 | `menu/load.rs:1102` `show_contextual_buttons` | `0x1e23d20` PHYS | 真缺失 | O | iced Event 判别子 4（x,y+250,0,-1；200×30）+ 模板 slot（会话名/日期/元数据）→ load 上下文按钮 | `get_functions 0x141e23d20` |

### 2.8 menu::new_session（1）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 45 | `menu/new_session.rs:526` `resolve_new_glade` | INLINE（3） | 真缺失 | I（Path clone/`Result::as_ref`/Vec clone） | 解析新谷路径/会话（`Option<PathBuf>` 语义 I）→ `new_session::ui` | 宿主 `new_session::ui 0x141eb9690` |

### 2.9 menu::report_bug（4）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 46 | `mei/widgets.rs:210`（`TextEditView::content_mut`） | INLINE（4） | 合并→共享 `content_mut` | O | `TextEditContent` 引用 → 文本框 | 宿主 `report_bug::ui 0x141e1b8e0`/`text_edit 0x140fa0bd0` |
| 47 | `menu/report_bug.rs:504` `TextEditView::disable_input` | INLINE（2） | 真缺失 | I | 置输入禁用字节 → `TextEditContent` | 宿主 `report_bug::ui 0x141e1b8e0` |
| 48 | `menu/report_bug.rs:500` `TextEditView::input_allowed` | INLINE（2） | 真缺失 | I | 返回 `input_allowed` bool → 提交/输入门控 | 宿主 `report_bug::ui 0x141e1b8e0` |
| 49 | `menu/report_bug.rs:425` `try_remove_file` | `0x1e1e5f0` PHYS | 真缺失 | O | `is_file`→Windows unlink；失败 `log` io::Error | `get_functions 0x141e1e5f0` |

### 2.10 photomode（4）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 50 | `photomode/photomode_buttons.rs:0` `photomode_buttons` | `0x234c230` PHYS | 真缺失 | I | 拍照按钮节点/主按钮 hotkey；动作跳表 4..0x11 → 文件夹/相机模式/菜单/Vermeer/滑块/焦点/设置 | `get_functions 0x14234c230`；ANALYSIS C12 |
| 51 | `photomode/setup_photomode_toolbar.rs:0` `setup_photomode_toolbar` | `0x26246e0` PHYS | 真缺失 | I | `ToolbarItem` 组（label/icon/类别/索引/kind）+ category button + contextual menu + nav → `PhotomodeToolbarLayout` | `get_functions 0x1426246e0`；ANALYSIS C13 |
| 52 | `photomode/ui_slider.rs:495` `slider_range` | INLINE（3） | 真缺失 | I | 滑块 `(min,max)`（表 `0x14304ed8c/0x14304edc0`）→ `ui_slider`/`ui_slider_reset` | 宿主 `ui_slider 0x1423486d0`、`ui_slider_reset 0x142347f90` |
| 53 | `photomode/ui_slider.rs:491` `slider_widget_size` | INLINE（3） | 真缺失 | I | `Vec2 * 常量` 的滑块尺寸 → 滑块实体 | 宿主 `ui_slider 0x1423486d0` |

### 2.11 resouces_and_components（9）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 54 | `resouces_and_components/mod.rs:212` `LogSlider::value_to_param` | INLINE（2） | 真缺失 | **O（依赖注解）** | `log2(value)`（`.clamp(min,max)`）→ 参数；与 `param_to_value=exp2(clamp+base)` 互逆 → `ui_slider_reset` case7 | 宿主 `ui_slider_reset 0x142347f90` |
| 55 | `resouces_and_components/taffy_layout.rs:178` `TaffyNodeBuilder::anchored_to` | INLINE（2） | 真缺失 | O（`Vec::push`） | 追加 `AnchoredNode{order,offset,node,relative_to,gap,side}` → `perform_layout` | 宿主 `toolbar_setup 0x142345890`/`setup_screen_and_toolbar_nodes 0x141e4ccb0` |
| 56 | `resouces_and_components/taffy_layout.rs:188` `TaffyNodeBuilder::child_of` | INLINE（12） | 真缺失 | O（`add_child().unwrap()`） | taffy 父子边 → 布局 | 宿主 `add_toolbar_item 0x141e4dc40` |
| 57 | `resouces_and_components/taffy_layout.rs:76` `roots_mut` | INLINE（2） | 真缺失 | I | 根切片 → `toolbar_alignment`/`perform_layout` | 宿主 `perform_layout 0x141e4c6c0` |
| 58 | `resouces_and_components/taffy_layout.rs:80` `root_mut` | INLINE（3） | 真缺失 | I（`iter_mut().find` + `slotmap` 比较） | 按节点 id 找根 → alignment | 宿主 `toolbar_alignment 0x142344560` |
| 59 | `resouces_and_components/toolbar_layout.rs:225` `ToolbarLayout::add_contextual_menu` | `0x1e4d1e0` PHYS | 真缺失 | I | 菜单根 leaf + `anchored{tag=0,weight=10.0}` + 递归 `add_toolbar_item`（`tag!=2`） | `get_functions 0x141e4d1e0` |
| 60 | `resouces_and_components/toolbar_layout.rs:281` `contextual_menus` | INLINE（5） | 真缺失 | I | 菜单切片迭代 → alignment/auto_hide | 宿主 `toolbar_setup 0x142345890` |
| 61 | `resouces_and_components/toolbar_layout.rs:288` `link_up_cyclic_list_nav` | INLINE（2） | 真缺失 | O（`EntityCommands::insert` + enumerate） | 每个按钮 `UiNav{prev=ids[(i+n-1)%n], next=ids[(i+1)%n]}`（水平/垂直） | 宿主 `toolbar_setup 0x142345890`/`setup_photomode_toolbar 0x1426246e0` |
| 62 | `resouces_and_components/toolbar_layout.rs:0` `setup_screen_and_toolbar_nodes` | `0x1e4ccb0` PHYS | 真缺失 | O | screen root + toolbar leaf + `anchored{order 0/1, tag4}` + roots[0..4] | `get_functions 0x141e4ccb0` |

### 2.12 toolbar_startup（2）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 63 | `toolbar_startup.rs:57` `setup_toolbar` | INLINE（2） | 真缺失 | I（`ToolbarItem::row` + Vec 分配） | 分类/按钮行 `Vec<(name,ToolbarCategory,Vec<ToolbarItem>)>` → `toolbar_setup` | 宿主 `toolbar_setup 0x142345890` |
| 64 | `toolbar_startup.rs:37` `toolbar_setup` | `0x2345890` PHYS | 真缺失 | I | Taffy + 静态 `0xb0B` 分类表 + demo/正式两套按钮组 + `add_category_button` + `add_contextual_menu` + `link_up_nav` + photomode/undo_redo setup + 3 资源 insert | `get_functions 0x142345890`；ANALYSIS B7.1 |

### 2.13 toolbar_update（1）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 65 | `toolbar_update.rs:28` `toolbar_size_mult_update` | INLINE（5） | 真缺失 | **O（依赖注解）** | `mult = min(mult, input)` + `set_changed` → `taffy_finalize_element_size`/`render_taffy`/`perform_layout` | 宿主 `toolbar_button_size_and_padding_update 0x141eaec50`、`render_taffy_ui_to_widgets 0x142338990` |

### 2.14 ui_interaction（10）

| # | 骨架 | 地址/形式 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 66 | `ui_interaction.rs:450` `sinc` | INLINE（2） | 真缺失 | O（`sin`；x→0 取 1） | `sin(x)/x` 缩放 → `ui_locked_animation` | 宿主 `ui_locked_animation 0x14233c8b0` |
| 67 | `ui_interaction.rs:552` `sort_contextual_buttons_draw_order` | `0x233e760` PHYS | 真缺失 | I | 写 u32 draw order + u64 可见性排序键 → `ui_2d_pass` | `get_functions 0x14233e760` |
| 68 | `ui_interaction.rs:367` `ui_appear_animation` | `0x233c020` PHYS | 真缺失 | I | `appear *= ease` + draw order → `Ui2dExtent`/`UiDrawOrder` | `get_functions 0x14233c020` |
| 69 | `ui_interaction.rs:335` `ui_focus_animation` | `0x233ba40` PHYS | 真缺失 | I | 目标 scale（1.5 空 / 1.2 / `UiCustomSize`）+ easing 写 `Ui2dExtent.size` | `get_functions 0x14233ba40` |
| 70 | `ui_interaction.rs:499` `ui_focus_parent_if_self_in_focus` | `0x233dcb0` PHYS | 真缺失 | I | 聚焦 id 去重后沿父链写 focus flag → `ui_show_if_parent_in_focus` | `get_functions 0x14233dcb0` |
| 71 | `ui_interaction.rs:414` `ui_locked_animation` | `0x233c8b0` PHYS | 真缺失 | I | `sinc` 抖动 scale；完成后 remove `UiPlayLockedAnimation`；命中插 `UiPlaySelectAnimation` | `get_functions 0x14233c8b0` |
| 72 | `ui_interaction.rs:483` `ui_notify_hints_if_toolbar_in_focus` | `0x233d680` PHYS | 真缺失 | I | 首个聚焦节点存在 → `WriteSettingsFileCmd` | `get_functions 0x14233d680` |
| 73 | `ui_interaction.rs:375` `ui_select_animation` | `0x233c300` PHYS | 真缺失 | I | `p=clamp01(p+dt/dur)`；`((1-p)*0.1+p)*4*(1-p)` 二次脉冲；完成 remove | `get_functions 0x14233c300` |
| 74 | `ui_interaction.rs:212` `ui_send_focus_begin_events` | `0x233a170` PHYS | 真缺失 | I | 进入聚焦的 entity 去重 → `WindowFocusEvent` | `get_functions 0x14233a170` |
| 75 | `ui_interaction.rs:456` `ui_show_if_parent_in_focus` | `0x233ce00` PHYS | 真缺失 | I | 父聚焦位 → 自身 visible/flag（父聚焦时 `visible=(c==0)||(state-1<2)`） | `get_functions 0x14233ce00` |

（2.14 为 10 项；全表累计 75 项。）

### 2.15 同节 7 个空/平凡函数（82 的其余 7 个）

| # | 当前源码 | 原地址 | 判定 | 级别 | 真实输出 → 消费者 | 深挖入口 |
|---:|---|---|---|---|---|---|
| 76 | `mei/core.rs:124` `NodeContent::is_empty` | 无独立符号（trait 默认） | 已实现 | O | 空内容判别（`EmptyContent`/默认）→ `mei_pre_update`/`mei_post_update` | 现有 `mei/core.rs:124,824` |
| 77 | `photomode.rs:17` `ui_slider` | `0x23486d0` PHYS（4152B） | 真缺失 | I/U | 滑块实体 spawn（`Ui2dExtent/Ui2dImage/UiDrawOrder`，纹理 `toolbar/slider_widget`）+ 鼠标拖拽 → 按 widget kind 写相机/笔刷参数 | `get_functions 0x1423486d0`；ANALYSIS C14；`ui_slider_reset 0x142347f90`、`slider_val_01 0x142349de0` |
| 78 | `resouces_and_components/toolbar_layout.rs:195` `kind_handler` | 跳表 `0x142f8f774`（数据） | 真缺失 | U | kind0 `0x60B` 命令记录（fn `0x140fdf9a0`，vtable+0x18）/kind7 `0x20B`（fn `0x140fdf500`）/kind2 三张 f32 朝向表（`0x142f8f6d8/…70c/…740`，fn `0x140c904a0`）/kind1/3/6 | 反汇编 `add_toolbar_item 0x141e4dc40` kind 跳表；`kind_handler` 现状仅占位 |
| 79 | `toolbar_button.rs:101` `switch_app_mode_button` | `0x2341fe0` PHYS | 空壳 | I（载荷 U） | 点击命中 → `RequestAppModeChange`；逐项比较 `AppMode`/`ModeJustChanged` 变体；命中插 `UiPlaySelectAnimation{0.2,0.2}` | `get_functions 0x142341fe0`；ANALYSIS B9.2 |
| 80 | `toolbar_systems.rs:186` `taffy_mut_style<T>` | 十单态 `0x2375b40..0x2376300` PHYS | 已实现（骨架） | O（骨架）/I（closure 目标） | `style→clone→f→set_style`（`TaffyLayout::style_mut` 等价） | `get_functions 0x142375b40..0x142376300`；`toolbar_systems.rs:25-90` 已记录 8 个写面 |
| 81 | `toolbar_systems.rs:222` `toolbar_alignment` | `0x2344560` PHYS | 部分 | I | 清非默认对齐位 `0x1020300>>((align&3)<<3)`；20 项跳表 `0x14304e6d8` 分派 taffy setter 组；case0xd 进度/缓动写 anchored | `get_functions 0x142344560`；`toolbar_systems.rs:101-240`；case4/6 二跳 U |
| 82 | `toolbar_systems.rs:259` `toolbar_auto_hide` | `0x2343a10` PHYS | 真缺失 | I | 光标/Esc/动作驱动的隐藏进度 `clamp01` 写 `+0x2c`；`EasingFunction` 写 anchored 位置 | `get_functions 0x142343a10`；ANALYSIS B8.1 |

---

## 3. 跨 crate 契约（只列需求，不改 country-core）

### 3.1 现有可直接消费（O，均已落库）

| 类型/API | 位置 | 用途 |
|---|---|---|
| `WidgetParams`（`new(volume,extra_78)`/`sounds`/`occlude_by_world`/`allow_negative_distance`/`no_history_recording`）、`WidgetVolumeShape` | `country-core/src/systems/ui/resources/widget_params.rs:240,272,284,292,298,305,311,317,179` | `mei_pre_update`、`render_taffy_ui_to_widgets`、`toolbar_steal_mouse_input` |
| `UiState`（`add(&str,&WidgetParams)`/`add_with_id`/`widget_context`/`hovered/clicked/dragged/is_active`）、`DragState` | `.../ui_state.rs:105,174,180,192,240,247,257,271,30` | UI 输入注册/查询 |
| `UiFocus`（`is_none/is_toolbar/is_mei/as_mei`）、`UiNavState`（`focus`/`staged_focus`/`hold`/`timer`） | `widget_params.rs:131,86,99` | 焦点与导航 |
| `UiSounds::empty`、`PlayUiSound`、`PlayMenuUiSound` | `widget_params.rs:205`、`systems/audio/mod.rs:115` | 声音事件 |
| `WindowFocusEvent` | `resources/window.rs:51` | 焦点/类别事件载荷 |
| `WriteSettingsFileCmd`、`EscapeMenuJustClosed` | `resources/events.rs:86`、`resources/gui.rs` | 设置写盘/菜单关闭 |
| `EscMenuState`/`NextEscMenuState`/`PrevEscMenuState`、`advance_menu_state` | `resources/gui.rs:59,123,133`（系统适配在 `gui.rs:180+`） | 菜单状态机 |
| `Toolbar`/`ToolbarCategory`/`ToolbarCategoryModes` | `resources/toolbar.rs:9,66,87` | 工具栏类别/当前 mode |
| `AppMode`、`ModeJustChanged`、`RequestAppModeChange`、`RequestToolbarCategoryChange` | `systems/mode_manager/types.rs:266`、`resources/events.rs:46`、`systems/mode_manager.rs:26,32` | 按钮切换 |
| `PhotoModeCameraMode`、`PhotomodeState`、`ImageFormationConfigs`、`BrushSizes` | `resources/photo_mode.rs:6`、`startup/init_photomode_state.rs:6`、`resources/image_formation.rs:209`、`resources/brush_sizes.rs:36` | 拍照系统 |
| `SessionMeta`/`ActiveSession`、`AutoSaveState`、`DisplayCorruptedSaveFileWarning` | `resources/session.rs:147,206`、`systems/save/save_game.rs:603`、`systems/save/load_completion_checker.rs:81` | 菜单/存档 |
| `GameConfig`、`GameProfile`、`GameClock`、`WindowSize`、`VermeerState`、`MainCamera`、`CursorPositionSS`、`SteamClientResource`、`UiSignifierStream`、`MetaCommander`、`DebugGui`、`TimeOfDayScreen`、`LogoAnimation`、`GraphicsDeviceResource`、`AssetTextureLibrary` | 各自文件（`game_config.rs:34`、`game_profile.rs:491`、`resources/game_clock.rs:92`、`resources/window.rs:68`、`systems/render_loop/vermeer.rs:162`、`systems/camera_rig/main_camera.rs:155`、`resources/mouse_terrain_raycast.rs:7`、`systems/steam/mod.rs:49`、`systems/ui/signifiers/signifier_create.rs:96`、`meta_commander.rs:94`、`resources/gui.rs`、`resources/gui.rs:728`、`resources/logo_animation.rs:45`、`render/device.rs`、`asset_libraries/texture_library.rs`） | 帧/菜单/拍照 |
| `Ui2dExtent`/`Ui2dImage{name,layer}`/`UiDrawOrder` | `components/ui.rs:31` 等 | 渲染实体 |
| `ShapeStream`/`TextPrim`/`Ui2dExtent` 消费者 | `systems/render_loop/draw_passes/ui_2d_pass.rs:6,204` | 绘制（GPU 后置） |
| 文本库 `TextRenderer`/`TextFragment`/`FontTable` | `tg-text` 依赖（不逆向） | 文本链 |

### 3.2 需要中央报备的缺口（② 不得本地伪造）

1. `CurrentDaily` / `DailyThemeInfo`：`tiny-glade/src/theme_fetcher_plugin.rs:23` 只有 `DailyThemeInfo`；每日欢迎所需的 `CurrentDaily` 资源未在 core/toolbar 出现。需中央定位原资源归属。
2. `WelcomeScreenState`、`DailyWelcomeScreenState`、`FeedbackReportResources`、`ThirdPartyLicenseServer`、`LoadMenuThumbnailMetrics`（credits/load/report_bug 的 Local/资源）在 core 未发现；属于 toolbar 私有或 tiny-glade 资源，需在 ② 落地时登记归属。
3. `SwitchAppModeButton`：toolbar 组件 16B 现承载 `AppMode`（`resouces_and_components/mod.rs:232`）；`toolbar_button.rs:93-108` 记录 B 读面为 `{ptr,len}` `&str`。二者冲突为 **U**，需 `get_functions 0x142341fe0` + `impl$5 0x142384c40` 复核后再定，不得改 core 猜测。
4. `UiState::add` 的 `String` 键格式（`format!("toolbar_button_{:?}", taffy_node)`）与 `WidgetParams` 的 `volume/extra_78` 字段身份：现有 API 可承载，但 `WidgetVolumeShape{tag=3,layer,size}` 投影是 I；需在 ② 判等。
5. `UiNavState+0x10=0` 写面（`tod_v2_button`）：现有 `UiNavState.staged_focus` 可表达，但类型/偏移判等 U。
6. `EscapeMenuJustClosed` 发出点：核心 `advance_menu_state` 已实现在关闭时发；`ReportBug`/`Photomode(false)` 区分需按原 `NextEscMenuState::set()` 库外语义复核（ANALYSIS C1）。
7. `PlayUiSound` 载荷（`{kind,payload,id}`）与 `ui_sounds` 9 参 Query 的映射 U；`ui_sounds` case 体被截断。
8. `TaffyNode` 尾区 12B 字段语义 U；`Ui2dExtent` 的 16B 常量归属（原 `UiScaleWithUserSetting` 0B 后改判）U。
9. `RequestAppModeChange` 的 `(str, SettingsMenuState)` 与 `AppMode` 冲突：`systems/mode_manager.rs:22-26` 注释称 16B AppMode，`toolbar_button.rs` 注释称 `tuple<ref<str>,SettingsMenuState>` 为 ICF 折叠；需中央裁定（不影响 `switch_toolbar_category_button` 的 `WindowFocusEvent` 面）。

### 3.3 依赖（按 `build-info/Cargo.lock`，不逆向）

`taffy 0.3.13`、`tg-text`、`ron 0.8.1`、`walkdir`、`relative-path`、`lang-lib`、`bumpalo`、`hashbrown`（12/14/15）、`iced_core`（事件名污染）、`bevy 0.16`、`tiny_http 0.12`、`ttf-parser`（经 tg-text）、`steamworks`（PounceLight rev）、`image 0.24/0.25`、`png 0.17.6`。

---

## 4. 验收门（“真实输出 + 消费者”判据）

### 4.1 全局门

1. `cargo check -p toolbar`（workspace 包名以 `crates/systems/toolbar/Cargo.toml` 为准）通过；每个恢复函数至少 1 个单测覆盖数值/布局/事件载荷。
2. 所有 PHYS 函数须有 fresh 反编译证据（`get_functions` 输出落 `evidence/` 或 `reports/`），并与骨架 `源码行`/RVA 对齐；INLINE 函数须给出宿主 RVA 与 PDB 依赖注解。
3. 禁止：`let _ = output` 吞输出、空体、`todo!` 长期留存、恒 0/恒 1 占位、伪造事件/字段；U 项在代码注释与本契约同时登记。
4. 每个输出必须落到真实消费者（见 §2 列）；无消费者时先报中央，不落“孤儿输出”。
5. 不复制 core 已恢复逻辑：MEI 优先调用 `safe_frame`/`new_at_pos`/`get_content_mut_impl`/`build_parent_links`；taffy 优先 `TaffyLayout::style_mut`/`perform_layout`；发现不等价先登记差异。

### 4.2 分阶段门（按帧顺序）

| 阶段 | 必须成立 |
|---|---|
| S0 建树 | `toolbar_setup` 输出真实 `ToolbarLayout`/`PhotomodeToolbarLayout`/`UndoRedoLayout` 资源；`TaffyLayout.anchored` 有 4+ 条；`link_up_nav` 的 `(i±1)%n` 环可断言；demo/正式两套按钮组分流可断言 |
| S1 交互 | `mei_pre_update` 的 hover/click/drag 位、`UiState::add` 调用、`PlayMenuUiSound` 写入、双缓冲换表均有单测；`process_input_action_handlers` 的 handled 压实可断言 |
| S7-S9 布局 | `taffy_finalize_element_size` 的 `TextFragment::bounds` 参与 size、倍率复位 1.0 可断言；`perform_layout` 排序/绝对化结果可对照；`taffy_update_drawables` 的 center=`origin+size/2` 可断言 |
| S11-S12 绘制 | `content.draw` 写 ShapeStream 的 center/extent/opacity 正确；文本顶点输入 `Ui2dExtent`；`mei_post_update` 的 z 排序键 `Node+0xac` 与 `total_cmp` 升序可断言 |
| S5 状态 | `toolbar_alignment` 的 20 项跳表各 case 的 taffy 字段写面（8 个已 O，其余 U 单列）；`toolbar_auto_hide` 进度 `clamp01`/缓动；`ui_*_animation` 自移除/插入；焦点上推/显隐；`sort_contextual_buttons_draw_order` 写序 |
| 菜单 | 每个 `menu::*::ui` 的 `EscMenuState` 门控、模板 slot、事件写面可断言；`advance_menu_state` 复用 core，不重写 |
| 拍照 | `photomode_buttons` 动作字节跳表每一 case、`setup_photomode_toolbar` 的 item 组、`ui_slider` 的 0..0x1e case 写目标都有真实输出；被截断 case 标 U |
| 文本 | `prepare_text_fragments`/`render_text_fragments` 使用真实 `TextFragment` 与 `Ui2dExtent`；`flush` 只消费真实 TextPrim/ShapeStream |

### 4.3 GPU/渲染后置清单（CPU 先真实，GPU 允许留边界）

| 项 | 处理 |
|---|---|
| `flush_mei_frame` / `flush_text_render`（TextPrim/ShapeStream → GPU buffer） | CPU 顶点/命令流先真实；上传与 `ui_2d_pass` 像素后置。宿主已有 `tiny-glade/src/mei_host.rs:196+` 的 library 消费边界 |
| `report_bug::ui` 的 GPU 截图（`MetaCommand`+rhapsody 分配器） | 表单/路径/删除 CPU 先真实；截图队列后置 |
| `ui_slider` 的 `Ui2dImage` 纹理 `toolbar/slider_widget` | 实体/组件/参数写入先真实；纹理上传后置 |
| `render_taffy_ui_to_widgets` 的 Signifier 绘制 | `WidgetParams`/事件/焦点先真实；Signifier 闭包/GPU 后置 |
| `demo_show_full_glade_size`/`max_elevation` 的 DebugLine/虚线 | 世界坐标/屏幕投影/节点先真实；GPU 线段后置 |
| `lut_gallery` 缩略图、`credits` 滚动纹理 | 资源/索引/滚动先真实；纹理 GPU 后置 |
| `photomode` 准星/围栏提示节点 | 节点/位置先真实；绘制后置 |

### 4.4 当前 crate 内已知近似/未闭合（不得计“已完成”）

- `taffy_update.rs:18` `taffy_finalize_element_size`：现按 `TaffyNode._tail + Query` 实现；BATCH3 证实原函数遍历**两套映射**（`+0x108` 分支、stride 0xa0/0x48）、逐节点 `TextFragment::bounds`、经 `taffy_mut_style` 写样式。**结构不一致，必须重验/重写**（登记为 I，不计 O）。
- `taffy_update.rs:63` `taffy_update_drawables`：现 Query 版与 ANALYSIS 的“两套映射 + `[center,origin]+u32`”不一致，需重验。
- `toolbar_systems.rs:222` `toolbar_alignment`：分派 case 体未落；case4/6 二跳跳表语义 U。
- `toolbar_systems.rs:259` `toolbar_auto_hide`：空体。
- `toolbar_button.rs:101` `switch_app_mode_button`：空壳；`tod_v2_button` 的 visible 偏移与 `UiNavState+0x10` 写面 U。
- `photomode.rs:17` `ui_slider`：空体；`ui_interaction.rs:168` `ui_sounds`：空体；`resouces_and_components/toolbar_layout.rs:195` `kind_handler`：空体。
- `ui_interaction.rs` `render_taffy_ui_to_widgets` 的 Signifier 闭包与焦点矩形链未落。

---

## 5. T13 反馈捕获（排除）

按 `AGENTS.md`：**不推进** T13 捕获/状态生产者；本契约不把 `FeedbackReportResources` 的 capture/state 缺口、GPU 截图回读、反馈提交网络链路计入 toolbar 验收。`report_bug::ui` 只验表单/文本编辑/临时文件删除的 CPU 面。

---

## 6. ② 实现清单（≤18 步，按帧顺序/依赖）

1. **证据复核（只读）**：对 §2 全部 PHYS 地址跑 `rne-read.py --hex` 确认函数头；对 INLINE 记录宿主；把 `taffy_finalize_element_size`/`taffy_update_drawables` 的“两套映射”差异写成待判等条目，不改代码。
2. **MEI core 叶子**：`child_with_content`、`content_mut`/`get_content_impl`、`with_layout`、`into_child`、`reset_keeping_content`、`clear_state`、`prepare_content`、`new_root`、`update_parent_mapping`、`find_node_path`、`steal_focus`、`lose_focus_on_invalid_nav`、`fullscreen_without_block_input`、`draw_rect`、`load_fonts`；逐一接到 `mei_pre_update`/`mei_post_update`/builders。
3. **config**：`deserialize_optional`、`localize_ui_text_from_config`，接 `read_node_config_from_file`/`instantiate_ui_template_impl`。
4. **layout 构建器**：`align_center`、`space_between`、`xalign_center`、`points_or_zero`（Length/Size/Rect）、`size_vec2`，接 `place_childen`/`compute_min_size_with_children`。
5. **widgets 视图**：`content_mut`（共享 trait）、`back_button`、`blur_extend`、`force_highlight`、`hotkey`（Menu/Pill）、`inactive_opacity`、`make_hotkey_tooltip`、`mute_click`。
6. **taffy 构建/访问器**：`anchored_to`、`child_of`、`root_mut`、`roots_mut`、`contextual_menus`、`link_up_cyclic_list_nav`。
7. **工具栏层级**：`setup_screen_and_toolbar_nodes`、`add_contextual_menu`、`setup_toolbar`、`toolbar_setup`（含 demo/正式分流、静态分类表、资源 insert、photomode/undo_redo 调用）。
8. **对齐/隐藏**：`taffy_mut_style` 全单态；`toolbar_alignment` 20 case（已 O 的 8 个 setter + case4/6 U）；`toolbar_auto_hide`。
9. **toolbar_update**：`toolbar_size_mult_update`、`toolbar_button_size_and_padding_update` 的 `closure_env$1` 样式目标。
10. **按钮系统**：`switch_app_mode_button` 载荷/变体判定、`tod_v2_button` 的 visible 偏移与 `UiNavState+0x10` 写面、对齐签名改回 SystemParam。
11. **动画**：`ui_focus_animation`、`ui_appear_animation`、`ui_select_animation`、`ui_locked_animation`、`sinc`。
12. **焦点/提示/排序**：`ui_send_focus_begin_events`、`ui_show_if_parent_in_focus`、`ui_focus_parent_if_self_in_focus`、`ui_notify_hints_if_toolbar_in_focus`、`sort_contextual_buttons_draw_order`。
13. **菜单 credits/load**：`ScrollView::content_mut/max_scroll_pos`、`spawn_third_party_license_http_server`、`ThumbnailScrollView::{content_mut,hovered_item_pos,initial_focused_item,min_scroll_pos,nav_scroll_speed,run_count}`、`display_save_file_load_error`、`show_contextual_buttons`。
14. **菜单 new_session/report_bug**：`resolve_new_glade`、`TextEditView::{content_mut,input_allowed,disable_input}`、`try_remove_file`。
15. **拍照**：`ui_slider`（含 `slider_range`/`slider_widget_size`/`value_to_param`）+ `ui_slider_reset`/`slider_val_01`（辅助）+ `photomode_buttons` + `setup_photomode_toolbar`。
16. **demo**：`demo_show_full_glade_size`、`demo_show_max_elevation`（CPU 标记/投影先真实）。
17. **调度接线**：`ToolbarPlugin::build`/`Ui2dPlugin::build` 的 9 事件 + 组件注册 + Startup/Update 集合；把 S3/S5 系统按 §1.1 顺序加入 `GameSystemSet`（`render_taffy → animation/button`、`alignment→perform_layout` 等约束），不把未知集合号伪造成 O。
18. **验收**：分阶段单测（§4.2）+ `cargo check`/`cargo test`；登记 U 与 GPU 后置；对比原字节/反编译证据；更新模块进度（不宣称完整验收，直到全部消费者闭合）。

> 每步完成即跑 `cargo check -p toolbar` + 该步单测；不满足“真实输出 + 真实消费者”不得标记完成。

---

## 7. 取证入口速查

- 骨架符号/RVA/源码行：`skeleton/B/crates/systems/toolbar/src/<file>.rs` 的 `// 函数符号` / `// 源码行` / `// 编译实例 RVAs` / `// 函数体依赖注解`。
- 定点反编译：`node scripts/mcp-job.mjs re_mcp_ghidra <job.json> <out.json>`，job 步骤 `decompile_function{address:"0x14..."}` / `batch`；共享只读，不写库。
- 原字节：`python scripts/rne-read.py --hex <VA> <n>`（VA 含 `0x140000000`）。
- 既有证据：`upper-modules-plan/systems/toolbar/ANALYSIS.md`（A1-A7/B1-B12/C1-C18）、`BATCH3-LOGIC-REVIEW-toolbar.md`、`evidence/toolbar-origin-2026-10-16/`、`reports/{wave2,wave2b,wave3,mei}/toolbar/`。
- 调度事实：`tiny-glade/src/mei_host.rs:39-68`（set6/7/44）、`tiny-glade/src/game_loop.rs:336`（set36）。
