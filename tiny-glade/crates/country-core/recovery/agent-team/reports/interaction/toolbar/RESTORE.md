# RESTORE — systems/toolbar（B 版函数体还原）

> restorer-interaction-toolbar，2026-10-17。fresh 反编译（Ghidra shared worker，
> `get_functions fields=signature,decompiled_code,callees`）逐函数核对。
> 证据：`reports/interaction/toolbar/evidence/`（results/batchA-core-leaf.json·txt、
> batchB-core-node.json·txt、batchC-layout-config.json·txt、batchD-verify.json·txt；
> jobs/*.json 可重放）。身份：B patched EXE tiny-glade.exe
> （sha256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323）。

## 现状与编译

- **crate 为本次从零建档**：`crates/systems/toolbar/`（workspace 原无此成员，任务描述
  「已注册」不实，已补齐）。模块集 = 本次可落 slice（mei core/layout/config、
  resouces_and_components::LogSlider）；其余 B 源文件按 ANALYSIS 三部分登记未实现，
  未纳入编译（见下）。
- `cargo check -p toolbar`：**0 错误 / 0 警告**（2026-10-17；未被其他组在途错误阻断，
  但 workspace 无 taffy/tiny_http/image 成员依赖，相关函数无法编译落库）。
- `cargo test -p toolbar`：**1/1 通过**（LogSlider::param_to_value 判等测试）。
- 依赖记录：taffy/tiny_http/image/open/webbrowser/feedback-reporter 等 B 锁 edge 未入
  workspace，本次不动共享 Cargo.lock（不代改他组）。

## 复核 FIX 落实（reviewer-interaction-toolbar，REVIEW.md FIX×8 + §4，2026-10-17 已闭合）

| FIX | 处置 |
|---|---|
| 1 `child_with_global_id`/`into_child_with_global_id` 父缺失静默创建 | **已改**：删 `reuse_or_create_node(self.id)`；父节点改为 `nodes.get_mut(&self.id).unwrap()`（探表+panic，unwrap_failed）。 |
| 2 `node()` expect 非 unwrap | **已改**：`.expect("NodeContext::node: node missing")`（原 expect_failed；expect 串 U）。 |
| 3 `NodeContext.mei` 应为 &MeiContext 指针 | **已改**：`pub mei: *mut MeiContext`（镜像原裸指针语义）；方法经 `unsafe { &mut *self.mei }` 重借；`new(mei: &mut MeiContext, id)`。 |
| 4 `calculate_hash` 疑零键 DefaultHasher | **已改**：`RandomState` → `DefaultHasher::new()`（零键 SipHasher13，与 `WidgetId::from_name` 0x14092df30 同源同值坐实；`<String>`/`<&str>` 同值消解）。 |
| 5 `new_at_pos<String>` 补 String 重载 src | **已加**：`MeiContext::new_at_pos_string`（0x142289f80；WidgetId::from_name 路径 + 帧末 drop String）。判等名实相符。 |
| 6 `load_configs` 存 self.configs | **已改**：`MeiContext` 增 `configs: HashMap<String, Arc<NodeConfig>>`，结果插入 self（不再返回 map）。 |
| 7 `Node::new` used_frame 初值 0 | **已改**：`used_frame: 0`（原新节点 +0x5d 区零）。 |
| 8 read_node_config Options 标志 | **已核**：ron 0.8.1 `Options::default()`={extensions empty, recursion_limit Some(128)}；原 recursion_limit 0x80=128 吻合；default_extensions（1 vs 0，疑 IMPLICIT_SOME）登记 I 级残留（控制流已判等）。 |
| §4 `process_input_action_handlers` 降级未实现-需重做 | **已改**：按 fresh 反编译重写——仅调 on_input(vtable[+0x50])（删 process_input_action）、正向首个 handled 记停点早停、反向自停点 re-apply 命中即返、**不修改 handler 表**（删 retain 副作用）。重建(I)（on_input 二/三参状态线程 local_70/local_64 U）。 |

## 硬标准说明

本 crate 509 本 ns 函数 / 331 业务，会话内按 ANALYSIS 三部分**串行推进**，块 A（mei 框架）
叶子切片完成，B/C 块及 A 的其余函数未及——未及者一律标未实现并登记分歧点，不以近似体
通过。**判等** 定义：fresh 反编译 → 逐语句控制流图 → Rust body 判等（hashbrown 探表以
`HashMap::get/contains_key/entry` 为合法翻译）。

## 判等函数（fresh 反编译逐语句，src 已落）

| 符号 | VA | 状态 | 证据 | src 位置 / 说明 |
|---|---|---|---|---|
| `Node::set_sounds` | 0x140683c20 | **判等** | batchB | 写 +0xb4/+0xb5 两声音 id。`mei/core.rs` |
| `Node::ignore_input` | 0x140683c30 | **判等** | batchB | `+0xb7|=2`。 |
| `Node::force_interactive` | 0x140683c40 | **判等** | batchB | `+0xb7|=1`。 |
| `Node::default_focus` | 0x140683c50 | **判等** | batchB | `+0xb7|=4`。 |
| `Node::with_nav` | 0x140683c60 | **判等** | batchB | `(b<<5)|(+0xb7&0xdf)`。 |
| `Node::block_input` | 0x140683cd0 | **判等** | batchD | `+0xb7=1`（赋值，非 OR）。 |
| `Node::used` | 0x140683c80 | **判等** | batchB | 物理 `return;` 空壳（used 语义由 safe_frame +0x5d=0 表达）。 |
| `NodeContext::hover` | 0x140683ce0 | **判等** | batchD | 探表确保存在（命中 return、缺失 panic；不写返回值）。 |
| `NodeContext::interaction` | 0x140683dd0 | **判等** | batchD | 同上。 |
| `MeiContext::safe_frame<&str>` | 0x140f99230 | **判等** | batchA | id=hash^mask；node+0x5d=0；opaque→+0xb7|=1；**(id&1)==0** 才 push 判别子 1 事件；返回{events,id,0}。 |
| `MeiContext::new_at_pos<&str>` | 0x140f992e0 | **判等** | batchA | 同上，判别子 4，写 x/y/w/h、+0x14 position、+0x1c extra、+0x30 id。 |
| `MeiContext::new_at_pos<String>` | 0x142289f80 | **判等** | batchD | **存在** 268B 物理函数（复核修正 unknown#1）；结构逐字段同 <&str>；id 经 `WidgetId::from_name`（=calculate_hash，零键 Sip13）^mask；末尾 dealloc String。**src 已落** `new_at_pos_string`（FIX 5）。 |
| `MeiContext::on_key_event` | 0x140685fc0 | **判等** | batchA | push MeiInputEvent（stride 0x70，cap@+0x18/data@+0x20/len@+0x28，满 grow_one）；载荷 10×u64+3×u64 + tag 字节 @+0x68。载荷字段语义 I。 |
| `MeiContext::on_clipboard_paste` | 0x1406860d0 | **判等** | batchA | push MeiInputEvent，tag 0xc5 @+0x58。 |
| `MeiContext::load_font_table` | 0x140683a20 | **判等** | batchC | `config::load_font_table().unwrap()` → `Arc::new` 存 +0x260（旧 Arc 递减/归零 drop_slow）。 |
| `NodeContext::node`（PDB widgets::impl$37::deref） | 0x140681d50 | **判等** | batchB | 探表返 &Node，缺失 **expect_failed**（.expect，FIX 2）。 |
| `NodeContext::node_mut`（deref_mut） | 0x140681e50 | **判等** | batchB | 探表返 &mut Node（.unwrap，unwrap_failed）。 |
| `NodeContext::child_with_global_id` | 0x1406861b0 | **判等** | batchB | reuse_or_create_node(子 id)；**父节点探表缺失 panic**（FIX 1，不静默创建）；命中 append 子 id 到父 children；返回{mei 指针,子 id}。 |
| `NodeContext::into_child_with_global_id` | 0x140686300 | **判等** | batchB | 同构。 |
| `NodeContext::has_global_id` | 0x140686450 | **判等** | batchB | 探表返 bool。 |
| `NodeContext::expect_child_with_global_id` | 0x140686530 | **判等** | batchB | 仅构造（断言调用侧）。 |
| `NodeContext::layout` | 0x140686550 | **判等** | batchB | 写本节点样式（72B 区）。 |
| `NodeContext::ignore_input` | 0x140686690 | **判等** | batchB | 透传 Node::ignore_input。 |
| `NodeContext::with_nav` | 0x140686790 | **判等** | batchB | 透传 Node::with_nav。 |
| `LayoutStyle::default` | 0x14068e850 | **判等** | batchC | 全零 + +0x44 u16=0（swizzle XY）+ +0x46 u8=4（justify）。 |
| `horizontal` | 0x14068e880 | **判等** | batchC | 与 default 同字节（+0x46=4）。 |
| `LayoutStyle::place_childen`（分派） | 0x14068ea20 | **判等** | batchC | 按 +0x44 swizzle 字节 0→SwizzleXY / 非 0→SwizzleYX。 |
| `LogSlider::param_to_value` | 0x142383730 | **判等** | batchC | `exp2f(param.clamp(min,max)+[3])`；min>max 触发 f32::clamp panic。含单测。 |
| `read_node_config_from_file` | 0x14068fe40 | **判等** | batchC | read_to_string→Ok→ron from_str_seed<NodeConfig>→返 Result；Err(io)→panic_fmt。NodeConfig RON 合同字段 U（空 serde 结构承载）。 |

## 重建函数（I 级：核心逻辑对应，字段/近似点登记）

| 符号 | VA | 状态 | 证据 | 分歧点 |
|---|---|---|---|---|
| `MeiContext::reuse_or_create_node` | 0x140683ec0 | **重建** | batchB | hashbrown 探表+插入以 `HashMap::get_mut/insert` 表达；**身份回收**（remove_entry + 旧 id/flags&0xf3/4×u32 载荷带入新节点）近似为新节点，未复刻。 |
| `process_input_action_handlers` | 0x140687eb0 | **重建** | batchA+review | reviewer §4 已改：仅 on_input(vtable+0x50)、正向首个 handled 记停点、反向 re-apply 命中即返、不修改 handler 表（删 retain）。残留：on_input 二/三参状态线程（local_70/local_64）U。 |
| `MeiContext::load_configs` | 0x140682e70 | **重建** | batchC | walkdir 扫描以 std::fs+递归等价；扩展名=="ron"、file_stem 键、log "Loaded UI config from {stem}"；**结果插入 self.configs**（FIX 6，不再返回 map）。 |
| `config::load_font_table` | 0x14068fc60 | **重建** | batchC | 同 read_node_config_from_file 解析 FontTable；ron Options 标志同 FIX 8 登记。 |
| `ComputedLayout::center` | 0x14068ea00 | **重建** | batchC | 物理 `return;`（getter 内联）；按 mei_post_update 用法 `pos+size*0.5`。 |
| `Node::hover` / `Node::has_focus` | 0x140683c90/0x140683ca0 | **重建** | batchB | 物理 `return;`；按 state bit0/bit6 getter（I）。 |

## glue-legitimate（分类，本次未重建/部分落库）

- **Bevy 注册胶水**：`ToolbarPlugin::build` 0x14068d610、`Ui2dPlugin::build` 0x14068dc70、
  `MeiPlugin(impl$23)::build` 0x1406868d0、`MenuFlowPlugin::build` 0x14068de50、
  `SplashScreenPlugin::build` 0x14068e570——`insert_resource`/`register_event`/`add_systems`
  胶水，本次仅落 `MeiContext` 资源（lib.rs `configure`），9 事件 + 系统集合因系统未恢复
  登记未实现（见下）。
- `resouces_and_components` 26× `impl$2..impl$27::register_required_components`
  （0x142383770..0x14238bb40）——Bevy 组件注册胶水，组件类型未落库，未实现。
- `GlobalId` Hash derive（impl$0::as_id 0x683c10，sip 写 16B）——derive 胶水。
- `NodeContent` 默认方法（0x3bb70/0x3bb80 空壳 return;）——trait 默认实现胶水。

## 未实现（分歧点登记；分析证据在 ANALYSIS.md / A / B / C 各节）

### A mei 框架（块 A 其余）
| 符号 | VA | 分歧点 |
|---|---|---|
| `MeiContext::mei_pre_update` | 0x1406842e0 | 37KB；输入→handler→hover/click/drag 状态机→事件派发→焦点→双缓冲交换+clear GC。子集合用途/0x78 阈值/tag0xc6/WindowFocusEvent 双条件 U。 |
| `MeiContext::mei_post_update` | 0x1406884c0 | 94KB；render 两遍+拓扑 z 序+taffy 最小尺寸+焦点/nav。render 返回值/+0xa8/+0xac/焦点判据/nav 动作名 U。 |
| widgets 构建器/视图（label/menu_button/settings_button/image/tab_switcher/space/action_hint/pill_button/back_button/frame + 数十 *View 方法 0x67e390..0x6816c0、0x32daf0/0x32d8b0/0x32e5d0 等） | 0x68bf00 等 | 需 Label/MenuButton/Image/Pill/Frame 内容类型落库；on_click 闭包槽位/metrics 字段 U。 |
| widgets/tweaker 族（0x32e920/0x330940/0x332010、impl$4/6/7/8 0x67da10..0x158770、TweakerView::delta/HintTracker/MaybeHint） | — | enum 版取模逻辑、TweakerContent 字段 U。 |
| widgets/decorate::decorate_hovered | 0x14067df20 | 悬停描边（tag3 事件+双 image 子节点）；tag3 消费/+0xa8 字段 U。 |
| mei/audio | 0x68e840 | 仅 default return;；声音接线在 ui_interaction。 |
| config `instantiate_ui_template_impl` / `TemplateView::*` / `load_configs`（config 侧） | 0x14068eb50 / 0x32e210..0x503950 | UiTemplate 字段布局/Frame 绘制参数/LabelContent 字段 U。 |
| layout `place_childen_impl` | 0x140fe12a0/0x140fe19a0 | 盒模型子布局；justify 全枚举+跳转表 0x142bc223c/0x142bc2260 坏分支未展开。 |

### B 布局/系统/交互
| 符号 | VA | 分歧点 |
|---|---|---|
| `TaffyLayout::style/perform_layout`、`make_layout_absolute` | 0x141e4c650/0x141e4c6c0/0x142374e10 | 依赖 `taffy` 0.3.13 未入 workspace；anchored 排序键/0x2fc/0x310/0x90/0x91 U。 |
| `taffy_mut_style` 10 单态 | 0x142375b40..0x142376300 | 依赖 taffy。 |
| `taffy_update_drawables` | 0x142340d20 | 依赖 taffy；两套映射/uVar4 U。 |
| `taffy_finalize_element_size` | 0x142340890 | **复核确认真实业务**（1152B；TextFragment::bounds + taffy_mut_style<closure_env_1>；B 段漏写）；依赖 taffy。 |
| `setup_screen_and_toolbar_nodes`/`add_contextual_menu`/`link_up_nav`/`add_toolbar_item` | 0x141e4ccb0/0x141e4d1e0/0x141e4d880/0x141e4dc40 | 依赖 taffy + country-core TaffyNode/UiNav 等未落库；add_toolbar_item kind 跳转表 DAT_142f8f774 U。 |
| `UndoRedoLayout::setup` | 0x140fad870 | 依赖 taffy + HistoryButton 组件。 |
| `toolbar_setup` | 0x142345890 | 静态分类表/DEMO 分流串文本 U；依赖 taffy。 |
| `toolbar_auto_hide`/`toolbar_alignment`/`undo_redo_alignment`/`undo_redo_auto_hide`/`toolbar_steal_mouse_input` | 0x142343a10/0x142344560/0x1423448a0/0x1423449f0/0x142344df0 | jumptable 破坏；对齐 case 偏移公式/资源偏移 +0xe8/+0xf1/+0xf7 U；依赖 taffy。 |
| `toolbar_button_keep_focus`/`switch_app_mode_button`/`switch_toolbar_category_button`/`tod_v2_button` | 0x1423414b0/0x142341fe0/0x1423427f0/0x142342f30 | 工具枚举映射/事件载荷 U；依赖 taffy/event。 |
| `toolbar_button_size_and_padding_update` | 0x141eaec50 | 依赖 taffy；closure_env$1 padding 数值 U。 |
| `render_taffy_ui_to_widgets`(+closure$0)/`ui_send_focus_begin_events`/`ui_sounds`/`ui_animation_state_update`/`ui_focus_animation`/`ui_appear_animation`/`ui_select_animation`/`ui_locked_animation`/`ui_show_if_parent_in_focus`/`ui_notify_hints_if_toolbar_in_focus`/`ui_focus_parent_if_self_in_focus`/`sort_contextual_buttons_draw_order`/`debug_hide_ui`/`hide_ui`/`ui_nav_update`(+closure$0)/`iter_subtree` | 0x142338990..0x14233eed0 | jumptable 破坏（ui_sounds 尾部边界错并已坐实）；依赖 taffy/UiNav/UiPlay*Animation 事件。 |
| `demo_show_full_glade_size`(+closure$0)/`demo_show_max_elevation` | 0x141eac540/0x141ead870/0x14234a410 | 依赖相机/地形/MEI 模板；常数 137.88582 语义 I。 |

### C 菜单/拍照/文本/许可/插件
| 符号 | VA | 分歧点 |
|---|---|---|
| `menu::advance_menu_state` | 0x14068e5b0 | 状态机推进（mem::replace 判别子技巧+变体 2 写文件）；EscMenuState 变体表不全。 |
| `welcome::ui`/`new_session::ui`/`daily_welcome::ui`/`splash::ui` | 0x141ebe130/0x141eb9690/0x141ebedd0/0x141ec1390 | 依赖 mei DSL/SteamClient/GameProfile 未落库；模板 id 串 U。 |
| `load::ui`(+closure)/`thumbnail_scroll<&str>`/`display_save_file_load_error`/`load_session_meta`/`show_contextual_buttons` | 0x141e1fe80/0x140fa06a0/0x141e23b90/0x141e23c00/0x141e23d20 | 依赖 core 存档链/mei DSL；会话目录名校验 U。 |
| `corrupted_save_file::ui`/`save_error::ui` | 0x141ebfbf0/0x141ec0ce0 | 依赖 mei DSL。 |
| `credits::ui`/`spawn_third_party_license_http_server`/`scroll<u64>` | 0x141dfa070/0x141dfb410/0x140fa04f0 | 依赖 tiny_http（未入 workspace）/mei DSL。 |
| `report_bug::ui`/`try_remove_file`/`text_edit<&str>`/`toggle_icon` | 0x141e1b8e0/0x141e1e5f0/0x140fa0bd0/0x14068cbf0 | 依赖 MetaCommander/FeedbackReportResources/cosmic_text 未落库。 |
| `photomode::do_photomode`/`photomode_buttons`/`setup_photomode_toolbar`/`ui_slider`/`ui_slider_reset`/`slider_val_01`/`lut_gallery::ui` | 0x14234b010/0x14234c230/0x1426246e0/0x1423486d0/0x142347f90/0x142349de0/0x141ec0270 | 依赖 MainCamera/ImageFormationConfigs/BrushSizes/mei DSL；滑块跳表截断分支 U。 |
| `text_startup_system`/`update_text_renderer_scale`/`prepare_text_fragments`/`render_text_fragments`/`flush_text_render` | 0x141e9ce50/0x141e9d960/0x141e9dd20/0x141e9df60/0x141e9e1f0 | 依赖 tg-text TextRenderer（已落库）接线+SSBO 名 U；update_text_renderer_scale 计算式 U。 |
| `SingleFileServer::http`/`run` | 0x14234d690/0x14286bf40 | 依赖 tiny_http（未入 workspace）；Content-Type/路径来源 U。 |
| `lib.rs` ToolbarPlugin/Ui2dPlugin 完整 build | 0x14068d610/0x14068dc70 | 9 事件注册+Startup/Update 系统集合（系统未恢复）；in_set 集合名 U。 |

## 共享基础缺口（未实现根因，与 onboarding 报告同域）

- `country_core::systems::ui::GlobalId` 类型未落库（本次暂驻 toolbar::mei::core，
  建议移入 country-core）；`UiNav/TaffyNode/UiAnimationState/UiPlaySelectAnimation/
  UiPlayLockedAnimation/WindowFocusEvent/WriteSettingsFileCmd/ToolbarSizeMult` 等未落库。
- `taffy 0.3.13`、`tiny_http 0.12.0`、`image 0.25.1`、`open/webbrowser/hotwatch` 等
  B 锁 edge 未入 workspace；feedback-reporter/build-id 无成员。
- `utils::calculate_hash` 属 utils crate 未落库（本 crate 内以 `calculate_hash` 镜像，
  恢复后应移）。

## 留给 reviewer 的核对点

1. `safe_frame`/`new_at_pos` 的 `(id&1)==0` push 门控与 0x140f99230/0x140f992e0
   `if ((uVar3 & 1) == 0)` 逐句一致（帧事件缓冲的线程化 Vec → MeiContext::frame_events
   累积表达是否可接受）。
2. `reuse_or_create_node` 的身份回收未复刻（remove_entry+旧 flags&0xf3/旧 id/4×u32 载荷），
   是否按 0x140683ec0 逐分支补齐。
3. `on_key_event`/`on_clipboard_paste` 的 0x70 载荷字段语义 I 级，事件 tag 偏移
   （key @+0x68 / clipboard @+0x58）需在 mei_pre_update 消费端复核。
4. `process_input_action_handlers` 残留：on_input 二/三参状态线程（local_70/local_64）、
   收集阶段的 handler 表链式探表（`goto LAB_140687f99` re-probe）是否与 0x140687eb0
   逐分支一致。
5. `NodeContext.mei` 用 `*mut MeiContext` 裸指针（FIX 3）在绘制链的 unsafe 重借边界
   是否可接受；`calculate_hash` 零键 `DefaultHasher`（FIX 4）已与 `WidgetId::from_name`
   同源同值坐实。
6. 未实现函数的共同根因（taffy/tiny_http/共享 UI 类型/mei DSL 消费端）落库后按本表
   证据地址逐函数重建。
