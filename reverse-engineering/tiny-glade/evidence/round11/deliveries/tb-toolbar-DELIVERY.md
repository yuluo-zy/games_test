# TB② 交付：`crates/systems/toolbar`（第 11 轮实现）

> 契约：`evidence/round11/contracts/tb-toolbar-CONTRACT.md`。
> 工作目录：`D:/game/tiny-glade`。仅修改 `crates/systems/toolbar/**`。
> 证据优先级：既有 fresh 反编译（`upper-modules-plan/systems/toolbar/jobs/decomp/*.c`、
> `evidence/core-recovery-20261015/gui/0x142341fe0.c`、`0x14233c300.c`）> PDB 依赖注解
> （`skeleton/B/.../toolbar/src/**`）> 契约表。未取证处一律标 U，不填默认值。

## 0. 构建/测试状态

| 命令 | 结果 |
|---|---|
| `CARGO_TARGET_DIR=/d/game/tiny-glade/target cargo check -p toolbar` | 见 §5（本轮结束前运行；前一次基线 exit 0） |
| `cargo test -p toolbar` | 见 §5 |

## 1. 本轮已完成（真实输出 + 消费者）

### 1.1 MEI core（`src/mei/core.rs`）

| 契约 # | 函数 | 状态 | 位置 | 证据/输出 |
|---:|---|---|---|---|
| 15 | `MeiContext::new_root`（合并 safe_frame/new_at_pos/fullscreen 共用根创建） | 已完成 | `core.rs:552` | O PDB inline 85；写 `frame_events` + `reuse_or_create_node`，消费者 `perform_layout` |
| 10 | `MeiContext::fullscreen` / `fullscreen_without_block_input` | 已完成 | `core.rs:568/574` | O inline 12/4；`FrameKind::FullWindow`，opaque 决定 `FORCE_INTERACTIVE` |
| 9 | `find_node_path` | 已完成（合并进既有 handler） | `core.rs:47` | O inline 2（deps `HashMap::get`/`Vec::push`/`slice::reverse`）；`process_input_action_handlers` 直接消费 |
| 5 | `NodeContext::child_with_content<T>` | 已完成 | `core.rs:976` | I（deps = `child<&str>` + `get_content_mut_impl`）；构建器/模板可用 |
| 7 | `NodeContext::content_mut<T>` | 已完成 | `core.rs:961` | O inline 223 = `get_content_mut_impl(false)` |
| 11 | `NodeContext::get_content_impl<T>` | 已完成 | `core.rs:967` | O inline 12（`downcast_ref` + `Option::expect`）；只读 `&T` |
| 12 | `NodeContext::into_child` | 已完成（复用 `into_child_with_global_id`） | `core.rs:987` | I：参数取 `GlobalId`（原生 inline 4，具体 ID 来源 U） |
| 8 | `RenderContext::draw_rect` | 已完成（投影到 `NodeDrawContext`） | `core.rs:134` | I：`center - extent*0.5` + `into`；消费者改为 `FrameContent::draw`（`widgets.rs:74`） |

### 1.2 MEI layout（`src/mei/layout.rs`）

| 契约 # | 函数/类型 | 状态 | 位置 | 证据 |
|---:|---|---|---|---|
| 21 | `LayoutStyle::align_center` | 已完成 | `layout.rs:304` | O 0x14068e960：justify=1 |
| 24 | `LayoutStyle::space_between` | 已完成 | `layout.rs:299` | O 0x14068e930：justify=3 |
| 25 | `LayoutStyle::xalign_center` | 已完成 | `layout.rs:309` | O 0x14068e990：align=2 |
| 22 | `LengthPercentage::points` / `points_or_zero` | 已完成 | `layout.rs:33/38` | O inline 24/11 |
| 23 | `Rect::size_vec2`（及 `width`/`height`/`points*`） | 已完成 | `layout.rs:86..120` | O inline 4（两次 `add`） |
| — | `Size{width,height}` + `points*` | 已完成 | `layout.rs:60` | O：LayoutStyle `size()` 写 +0x28/+0x30 的 16B |
| — | `LayoutStyle::{horizontal,vertical,gap,padding,expand,size}` | 已完成 | `layout.rs:273..319` | O 0x68e880/0x68e8b0/0x68e8f0/0x68e9c0 + inline 2/1/2 |

### 1.3 Taffy 构建/访问器（`src/resouces_and_components/taffy_layout.rs`）

| 契约 # | 函数 | 状态 | 位置 | 证据 |
|---:|---|---|---|---|
| 55 | `TaffyNodeBuilder::anchored_to` | 已完成 | `taffy_layout.rs:186` | O inline 2（`Vec::push`，写 `AnchoredNode{order,offset,node,relative_to,gap,side}`） |
| 56 | `TaffyNodeBuilder::child_of` | 已完成 | `taffy_layout.rs:206` | O inline 12（`add_child().unwrap()`） |
| — | `TaffyNodeBuilder::root` | 已完成 | `taffy_layout.rs:165` | O inline 4（两次 Vec::push → `roots`） |
| 57 | `TaffyLayout::roots_mut` | 已完成 | `taffy_layout.rs:58` | I inline 2；`roots()` 同步 |
| 58 | `TaffyLayout::root_mut` | 已完成 | `taffy_layout.rs:63` | I inline 3（`slice::iter::find` + slotmap eq） |
| — | `TaffyLayout::{add_node,layout,children}` | 已完成 | `taffy_layout.rs:47/78/84` | O inline 16/4/8（`new_leaf`/`layout`/`children` unwrap） |

### 1.4 交互/动画（`src/ui_interaction.rs`）

| 契约 # | 函数 | 状态 | 位置 | 证据 |
|---:|---|---|---|---|
| 73 | `ui_select_animation` | 已完成 | `ui_interaction.rs:217` | O fresh `0x14233c300.c`：`p+=dt/b` clamp，`pulse=((1-p)*0.1+p)*4(1-p)`，`scale=(pulse²·a+1)·scale`，`p==1` remove |
| 71 | `ui_locked_animation` | 已完成（核心脉冲） | `ui_interaction.rs:242` | I ANALYSIS-B §11.9：`x=(p*5-1)π`、`sinc`、自移除；命中插入分支 U |
| 66 | `sinc` | 已完成 | `ui_interaction.rs:200` | O inline 2（x→0 取 1） |
| — | `TaffyNode::scale/set_scale`（+0x10 f32） | 已完成 | `resouces_and_components/mod.rs:73` | O：0x14 stride 的 `*(f32*)(dense+idx*0x14+0x10)` 读写；字段名 U |

### 1.5 按钮（`src/toolbar_button.rs`）

| 契约 # | 函数 | 状态 | 位置 | 证据 |
|---:|---|---|---|---|
| 79 | `switch_app_mode_button` | 已完成 | `toolbar_button.rs:103` | O fresh `0x142341fe0.c`：点击→`RequestAppModeChange(button.mode)`（16B AppMode）；`ModeJustChanged` 未读时逐项 `AppMode` 相等→写 `interaction.selected` + 入队 `UiPlaySelectAnimation{0.2,0.2,0}`；reader 游标推进到末尾 |
| — | `switch_toolbar_category_button` 的 `ModeJustChanged` 读取 | 修正 | `toolbar_button.rs:167` | 由 `is_empty()` 改为遍历读取（drain），对齐原生游标移动 |
| — | `tod_v2_button` 首事件语义保留 + drain | 修正 | `toolbar_button.rs:191` | 只取首事件 `tool != 0xd`，其余仍被消费 |

### 1.6 config（`src/mei/config.rs`）

| 契约 # | 函数 | 状态 | 位置 | 证据 |
|---:|---|---|---|---|
| 3 | `deserialize_optional` | 已完成（helper 已公开） | `config.rs:233` | O inline 19（`Option<T>` seed） |
| 4 | `localize_ui_text_from_config` | 已完成（`localize_label` 公开入口） | `config.rs:227` | I inline 2（`t!(key)` 替换、`Invalid t! format` 诊断） |

### 1.7 其他叶子

| 契约 # | 函数 | 状态 | 位置 | 证据 |
|---:|---|---|---|---|
| 54 | `LogSlider::value_to_param` | 已完成 | `resouces_and_components/mod.rs:607` | O inline 2：`log2(value).clamp(min,max)`，与 `param_to_value` 互逆 |

### 1.8 调度接线

`lib.rs:101` 的 `configure` 新增 `ui_select_animation` / `ui_locked_animation`，其真实输出为
`TaffyNode+0x10` scale 与命令队列移除（不是孤儿函数）。`configure` 在 workspace 仍无
调用者（契约 §0.2 的已知接线缺口，属中央集成步骤 17），本轮不伪造外部调用。

## 2. 本轮新增测试

| 测试 | 文件 | 断言 |
|---|---|---|
| `find_node_path_reverses_parent_chain` | `mei/core.rs` | root→focus 顺序、单节点 |
| `fullscreen_variants_share_root_creation` | `mei/core.rs` | FullWindow、opaque 标志差异 |
| `child_with_content_initializes_typed_owner_once` | `mei/core.rs` | typed content 初始化/复用/只读访问 |
| `style_builders_write_native_tags_and_size` | `mei/layout.rs` | justify/align/gap/padding/expand/size 写面 |
| `points_accessors_resolve_or_zero` | `mei/layout.rs` | percent→0、Size/Rect 投影 |
| `builder_registers_roots_children_and_anchors` | `taffy_layout.rs` | root/child_of/anchored_to 真实 taffy 边 |
| `select_animation_pulses_scale_then_removes_component` | `ui_interaction.rs` | 脉冲数值 + p==1 移除 |
| `sinc_has_removable_singularity` | `ui_interaction.rs` | sinc(0)=1 |
| `click_writes_target_mode_and_mode_change_selects_matching_button` | `toolbar_button.rs` | RequestAppModeChange 载荷 + selected/animation |
| `value_to_param_inverts_and_clamps` | `resouces_and_components/mod.rs` | log2 互逆/夹取 |
| `localize_ui_text_substitutes_every_t_marker` | `mei/config.rs` | t! 替换与诊断 |

## 3. 显式 U / 未完成

1. `Node::steal_focus` / `Node::lose_focus_on_invalid_nav`（契约 #14/#18）：PDB 只有
   inline 记录与 bitflags 宿主（`load.rs` closures），无参数/字段证据。保留未实现，未猜签名。
2. `ui_appear_animation` / `ui_focus_animation`（#68/#69）：EasingFunction 的来源组件与
   focus 位图（+0x120/+0x128/+0x130）未取证，未落近似体。
3. `ui_sounds`（#67/`ui_interaction.rs:285`）：跳表 case 体被 Ghidra 截断；声音 id→音效映射 U。
4. `render_taffy_ui_to_widgets` 的 Signifier 闭包与焦点矩形链：闭包胖指针未展开（契约 §4.4）。
5. `toolbar_alignment`（#81）/`toolbar_auto_hide`（#82）：无 fresh 反编译；case 4/6 二跳 U。
6. `photomode::ui_slider`（#77）、`setup_photomode_toolbar`（#51）：4KB/超大反编译未在本轮
   重解；现有 `photomode_camera_buttons` 仅覆盖相机子集（上一轮）。
7. `ui_slider_reset` / `slider_val_01` / `param_to_value` 的滑块 case 写目标。
8. widgets 视图族（`back_button`/`blur_extend`/`force_highlight`/`hotkey`/`inactive_opacity`/
   `make_hotkey_tooltip`/`mute_click`）：反编译已确认字段写面（Image+0x44、MenuButton+0x25c/
   +0x25d、Pill+0x40/+0x44），但 `MenuButtonContent`/`PillButtonContent` 及其 builder 在
   当前 crate 不存在；先补类型需同时恢复 1000+ 行 widgets，本轮未落，登记跨模块缺口。
9. `kind_handler`（#78）命令记录载荷 U（0x60B/0x20B、fn@0x140fdf9a0/0x140fdf500）。
10. 菜单 `menu/{credits,load,new_session,report_bug,...}` 的多数函数与缺失源文件未实现；
    `menu/load.rs` 已有 `ThumbnailScrollContent` 主体，命名 helper（`hovered_item_pos`/
    `initial_focused_item`/`min_scroll_pos`/`nav_scroll_speed`/`run_count`）尚未拆出。
11. `UiPlayLockedAnimation` 的"命中交互插入 select 动画"尾段（0.3/0.1）未取证。
12. `TaffyNode` 尾区字段名：`+0x10` 已证实为动画乘数 f32，其余 8B 字段名 U；
    `TaffyNode::default` 的 scale=1.0 为 I（否则乘法恒 0，与动画消费矛盾）。
13. `SwitchAppModeButton` 点击事件的 EventInstance id 字段由 bevy 管理，未单独建模。

## 4. 跨 crate 缺口（未改 country-core/rhapsody）

1. `TaffyLayout.roots` 为本轮新增字段（原生 inline `root`/`roots`/`roots_mut` 的直接结论）；
   若中央后续确认原字段名不同，仅需重命名。
2. `MenuButtonContent`/`PillButtonContent`/`TextEditContent`/`ScrollContent` 等 widget 私有
   内容类型仍缺；它们的组件/系统在 toolbar 内，需要后续轮次补齐（非 core 缺口）。
3. `PhotomodeToolbarLayout`/`ToolbarLayout`/`UndoRedoLayout` 资源类型尚未在 crate 落地
   （契约步骤 6/7），后续需要与 `taffy_layout` 一起恢复。
4. `UiPlaySelectAnimation/UiPlayLockedAnimation` 第三字语义：本轮按反编译改为 `progress: f32`
   （原生以 f32 读写），已同步所有构造点；无 core 影响。

## 5. 构建结果

| 命令 | 结果 |
|---|---|
| `CARGO_TARGET_DIR=/d/game/tiny-glade/target cargo check -p toolbar` | **exit 0**，`Finished dev profile ... in 32m 51s`（共享 target；期间被其它 agent 的构建排队，最终通过） |
| `CARGO_TARGET_DIR=/d/game/tiny-glade/target-tb11 cargo check -p toolbar` | **exit 0**，`Finished dev profile ... in 2m 19s`（独立 target 复验） |
| `CARGO_TARGET_DIR=/d/game/tiny-glade/target-tb11 cargo test -p toolbar --lib` | 首轮因并发 core 编辑失败（非 toolbar）；续跑改用 `target-agent-tb` 后 **36 passed / 0 failed**（见 §6.5） |

`cargo check -p tiny-glade`：第一轮只修改 `crates/systems/toolbar/**`；续跑新增
`crates/tiny-glade/src/toolbar_plugin.rs` 与 `lib.rs` 的一行 `pub mod toolbar_plugin;`
（中央要求的宿主 Plugin 入口），未改 `run_game.rs`、country-core 或 rhapsody。
共享 target 的 `cargo check -p toolbar` 已通过；`-p tiny-glade` 的并发失败/成功取决于
其它 agent 对 core 的在途修改，与本 crate 无关。

### 5.1 并发环境说明

本轮有多个 agent 并行使用同一 `target/` 与同一 workspace：`cargo` 构建锁导致本组的 check
排队约 30 分钟，且 country-core 在测试时处于他人未完成编辑状态。交付中的编译结论以
`cargo check -p toolbar` 两次 exit 0 为准；测试重跑结果将追加在下方（若 core 仍不稳定，
以“测试被外部 core 编辑阻塞”记录，不伪称通过）。

**重要（并发写者）**：在本 agent 验证通过（共享 target 完成时间 16:08）之后，同一 crate
仍有其它写入发生（时间戳 16:05/16:18）：
- `mei/widgets.rs`：新增 `PillButtonContent` 与 `NodeContext::back_button`（非本 agent）；
- `mei/core.rs`：`Node::used` 由空壳改为 `&self -> bool`（bit4/5 判定，非本 agent）；
- `photomode.rs` + 新文件 `photomode/lut_gallery.rs`：LUT 浏览器（非本 agent）。
本交付对这些项**不作验收声明**；如它们导致最终 workspace 构建失败，应由其作者修复。
本 agent 的改动均仍存在，未被覆盖（已在 16:1x 逐项 grep 核对）。

## 6. 收口续跑（17:2x）

### 6.1 消费者接线（已完成）

- 新增 `crates/tiny-glade/src/toolbar_plugin.rs`：
  - `pub struct ToolbarPlugin` 实现 `bevy_app::Plugin`；
  - `build(app)` 调 `toolbar::configure_app(app)`，并在 `GameSystemSet(43)` 用
    `.chain()` 注册 8 个真实系统：`render_taffy_ui_to_widgets` →
    `ui_animation_state_update` → `ui_select_animation` → `ui_locked_animation` →
    `toolbar_button_keep_focus` → `switch_toolbar_category_button` →
    `switch_app_mode_button` → `tod_v2_button`。
  - set43 的选择依据：契约 §1.1 的 S5 集合号 8..43 为 U；set43 在 set44（taffy
    finalize/layout）之前，使 `render_taffy` 消费上一帧布局、其 UiState 供下一帧
    set6 的 `mei_pre_update` 消费。
- `crates/systems/toolbar/src/lib.rs` 新增 `configure_app`：安装 `MeiContext`、
  `ToolbarSizeMult`、`UiNavState`、`UiSignifierStream`、8 个事件资源与 24 个本地组件。
- `crates/tiny-glade/src/lib.rs` 增加 `pub mod toolbar_plugin;`。
- **中央需要插入的一行**（`run_game.rs`，本 agent 未改）：
  ```rust
  app.add_plugins(tiny_glade::toolbar_plugin::ToolbarPlugin);
  ```
  依赖顺序：需在 `ToolbarPlugin` 之前完成 core 资源（`GameConfig`/`UiState`/
  `CursorPositionSS`/`ActionSetStates`/`AppMode`）注册；重复 `init_resource` 幂等。

### 6.2 孤儿 API 处置

| API | 处置 |
|---|---|
| `deserialize_optional` | **已接真实消费者**：`LabelConfig` 的 6 个 `Option` 字段均改为 `#[serde(default, deserialize_with = "deserialize_optional")]`，由 `read_node_config_from_file`/模板实例化实际反序列化 |
| `taffy_mut_style` | **已接真实消费者**：`TaffyLayout::style_mut` 的实现改为直接调用它（`perform_layout` 每帧经 `style_mut` 写窗口样式） |
| `TaffyNodeBuilder` 全族 | **primitives-only U**：`root/anchored_to/child_of` 的 Vec/add_child 语义已实现并有单测，但 `toolbar_setup`/`setup_screen_and_toolbar_nodes`/`add_toolbar_item` 尚未迁移到该 builder；不计“已完成业务” |
| `TaffyLayout::{add_node,roots,roots_mut,root_mut,layout,children}` | **primitives-only U**（`perform_layout` 目前直接读 `anchored`；roots 的消费者 alignment 未落） |
| `LogSlider::value_to_param` | **primitives-only U**：ui_slider_reset 未恢复，暂无调用点；公式 O + 单测 |

### 6.3 Signifier 载荷写回（已完成）

`render_taffy_ui_to_widgets` 的无 HistoryButton 分支不再丢弃 0x20B 载荷：
构造 `SignifierFactory::new(&payload, null, 0)` 后调用
`UiSignifierStream::write_factory` 真实入流。U：闭包 `inner/marker` 未取证，
以 null/0 记录（无闭包记录走 5-qword 原始视图），不伪造闭包。

### 6.4 剩余项推进

- 新增 `menu/report_bug.rs`：`try_remove_file`（O 0x141e1e5f0），含真实文件删除
  与 io::Error 日志；T13 捕获/截图按约束排除。
- 其余菜单/文本/拍照主体（credits、welcome、new_session、report_bug 表单、
  text_startup、setup_photomode_toolbar、ui_slider）仍缺证据或与并行写者重叠，
  按 §3 U 清单处理，不计完成。
- 已观测到其它 agent 在 `menu/load_ui.rs`、`mei/widgets/menu_button.rs`、
  `photomode/{lut_gallery,timelapse}.rs` 的并发实现，本交付不重复认领。

### 6.5 续跑编译

| 命令 | 结果 |
|---|---|
| `CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-tb cargo check -p toolbar` | **exit 0**（首次 1m37s，复核 0.91s fresh） |
| `CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-tb cargo check -p tiny-glade` | **exit 0**，1m36s；验证新增 `toolbar_plugin.rs` 与 `lib.rs` 接线 |
| `CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-tb cargo test -p toolbar --lib` | **36 passed / 0 failed**（含本 agent 新增 12 项与并发写者的测试；报告 `test result: ok`） |

测试结果：全部通过（17:43，target-agent-tb）。首次运行暴露的 `Time::default()` 类型推断问题已修正为 `Time::<()>::default()`。


