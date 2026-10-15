# systems/toolbar 批次 3 业务逻辑复核报告

- 复核人：批次 3 业务逻辑复核 agent（toolbar 模块）
- 日期：2026-10-15
- 对象：`upper-modules-plan/systems/toolbar/ANALYSIS.md`（A mei 框架 + B 布局/系统/交互 + C 菜单/拍照/文本/许可/插件）
- 对照：`skeleton/B/crates/systems/toolbar/`（38 文件 / 509 本 ns compiled_procedure）
- 二进制：B patched EXE（sha256 `cf048e27…7036323`）
- 方式：符号清单全量比对 + 关键结论只读 MCP 抽查（job：`jobs/review-spotcheck.json` / `review-no-phys.json` / `review-newsatpos.json`，worker shared）

---

## 1. 复核范围与结论摘要

**范围**：只复核 toolbar 模块内部实现；不分析上下层调用关系。覆盖核对（509 发射函数 vs ANALYSIS 条目）、逐函数逻辑、跨部分一致性、算法与依赖标注、unknown 合理性、冲突登记。

**结论摘要**：

1. **覆盖度**：509 发射函数中 ANALYSIS 覆盖约 9 成；A/B/C 主函数基本齐全。**主要遗漏 1 个真实业务函数**：`taffy_update::taffy_finalize_element_size`（0x142340890，size 1152），B 段只写了 `taffy_update_drawables`，未提及前者。另有若干 NodeContext / NodeContent / tweaker / 菜单内容 trait 单态未逐条列出（多为分组覆盖或胶水，见 §2）。
2. **抽查坐实**：
   - `ui_sounds`（0x14233a7f0）尾部并入 `drop_ptr<StairsRemovedSupports/TreeStorage>` + 组件索引 jumptable（至 0x14234004d）——B11.4「反编译边界错并」**属实**，需 radare2/定点核对边界（size 2608 含尾块）。
   - **`new_at_pos<String>` 有物理函数**：0x142289f80 存在（268B，可正常反编译），结构完全吻合 A1.3 描述（tag=4、写 x/y/w/h、+0x14 Vec2、+0x30=id、stride 0x38）。A1.3 与已知卡点「无物理函数」**为 stale 断言**，应更新。
   - `NodeContext::mei`（0x885d0）确无物理函数（Ghidra Function not found）——A1.3 此半正确。
   - `taffy_update_drawables`（0x142340d20，size 819）反编译与 B3.1 描述逐条吻合（两套映射 +0x108、stride 0xa0/0x48、center=origin+size*0.5 + origin + u32 数组）。
3. **跨部分一致性**：GlobalId 派生常量 `hash^0xb1695320004fb443` 在 A/B/C 引用处一致；TaffyLayout/anchored_nodes 归属无冲突（mei 保留模式节点表 HashMap<GlobalId,Node> 与 taffy 实体两套 UI 并行，A 不涉及 anchored_nodes）。**不一致点**：Node 布局偏移 A/C 对不上（+0xa4 opacity vs +0xb0/+0xb8 alpha）；A0.3 NodeContent 虚表缺 `on_input` 槽；`tag`/`判别子` 术语三种含义混用；`0x28B`/`0x28` 步长笔误。
4. **算法/依赖**：三类算法清单均已点名关键算法（含 ipnsort/insertion_sort/driftsort、smoothstep、sinc、exp2f、inverse_lerp、NDC 逆投影等）；taffy / tg-text / ron / tiny_http 均正确标为「复用不逆向」。
5. **known 卡点核对**：`color_menu`/`show_color_options` 不在 toolbar 发射清单 ✓；`ui_sounds` 边界错并 ✓ 坐实；`toolbar_alignment`/`ui_nav_update` jumptable 破坏 ✓ 均已标 I 并列入 U；`new_at_pos<String>` 无物理函数 ✗（应关闭）。

---

## 2. 覆盖核对表

依据：38 个 `*.rs` 骨架文件全量提取的 509 个 `toolbar::` compiled_procedure。列为「发射→覆盖」。

| 骨架文件 | 发射数 | 覆盖 | 遗漏/备注 |
|---|---:|---|---|
| demo_stuff/demo_show_full_glade_size.rs | 2 | demo_show_full_glade_size、::closure$0 | B12.1/12.2 ✓ |
| demo_stuff/demo_show_max_elevation.rs | 1 | demo_show_max_elevation | B12.3 ✓ |
| lib.rs | 5 | impl$0::build、impl$1::build、report_bug::impl$6::process_input_action、welcome::ui、ui_nav_update（插件实例） | C18 + C2/B11.16 ✓ |
| taffy_update.rs | 2 | taffy_update_drawables | **taffy_finalize_element_size（0x2340890）完全未提及（B3 只写 drawables）** |
| text.rs | 5 | text_startup_system / update_text_renderer_scale / prepare_text_fragments / render_text_fragments / flush_text_render | C16 全部 ✓ |
| third_party_licenses_server.rs | 2 | SingleFileServer::http / ::run | C17 ✓ |
| toolbar_button.rs | 4 | toolbar_button_keep_focus / switch_app_mode_button / switch_toolbar_category_button / tod_v2_button | B9 ✓ |
| toolbar_startup.rs | 1 | toolbar_setup | B7.1 ✓ |
| toolbar_systems.rs | 5 | toolbar_alignment / undo_redo_alignment / undo_redo_auto_hide / toolbar_auto_hide / toolbar_steal_mouse_input | B8 ✓ |
| toolbar_update.rs | 1 | toolbar_button_size_and_padding_update | B10.1 ✓ |
| ui_interaction.rs | 18 | 18 个全部 | B11 ✓（含 ui_nav_update::closure$0、iter_subtree） |
| resouces_and_components/mod.rs | 21 | impl$2..27::register_required_components ×20、LogSlider::param_to_value | B6 ✓（register 计为 Bevy 胶水） |
| resouces_and_components/taffy_layout.rs | 3 | TaffyLayout::style / perform_layout / make_layout_absolute | B1 ✓ |
| resouces_and_components/taffy_utils.rs | 10 | taffy_mut_style ×10 单态 | B2.1 以地址区间分组 ✓ |
| resouces_and_components/toolbar_layout.rs | 6 | setup_screen_and_toolbar_nodes / add_toolbar_item / add_contextual_menu / link_up_nav / add_category_button ×2 | B4 + C13 ✓ |
| resouces_and_components/undo_redo_layout.rs | 1 | UndoRedoLayout::setup | B5.1 ✓ |
| mei/audio.rs | 1 | impl$0::default | A7 ✓（内联宿主定位） |
| mei/config.rs | 101 | TemplateView::with_slot 约 94 单态、template/template_with_id×2、into_slot/slot、node_mut、instantiate_ui_template_impl、MeiContext::{load_configs,load_font_table}、load_font_table、read_node_config_from_file | A2 ✓；deserialize visit_* 胶水未逐条（低） |
| mei/core.rs | 139 | 主函数（reuse_or_create_node/safe_frame/new_at_pos×2/mei_pre_update/mei_post_update/on_key_event/on_clipboard_paste/process_input_action_handlers/impl$23::build/Node 方法/as_id×3）及 NodeContext 系列大部分 | **遗漏：NodeContext::{sounds, hover, interaction, node, id, with_layout, content<…>×6, content_mut<FrameContent>} 未列；NodeContent::on_input<…>×8 未列（虚表描述缺 on_input 槽）；impl$21::{as_any,as_any_mut,type_name}×13、impl$20::default 胶水未逐条；mei/core iter_subtree×2（update_nav）算法仅在 B11.18 描述一次** |
| mei/layout.rs | 19 | compute_min_size_with_children / horizontal / LayoutStyle::{gap,expand,space_between,align_center,xalign_center,size,place_childen,place_childen_impl×2} / ComputedLayout::center / impl$16::default | A3 ✓；deserialize 胶水未逐条（低） |
| mei/widgets.rs | 65 | 构建器（label/label_with_id/menu_button/menu_button_no_nav/menu_button_with_id/settings_button/image×2/tab_switcher/space/action_hint/pill_button/back_button/frame）与视图方法 | **NodeContext::sounds（在 widgets.rs）未列；内容 trait 单态 impl$2/3/6/8/9/12/14/15/22/24/25、deref×2 未逐条（FrameView::draw=impl$9::render 已单独列，其余分组）** |
| mei/widgets/decorate.rs | 1 | decorate_hovered | A6 ✓ |
| mei/widgets/tweaker.rs | 37 | tweaker_impl、tweaker<TweakableBool>、tweaker<FloatWithStep>、impl$4/6::{decrease,increase,format}、impl$7::format<DisplayMode>、impl$8::format<Resolution>、TweakerView::delta、HintTracker::text、MaybeHint::confirm | **遗漏：tweaker<TweakableEnum<…>>×6、tweaker<TweakableList<…>>×6、tweaker_w_font_size<GuiKeyBindingsCategory>、impl$8::format<ImageQualityPreset/FpsCap/String/RefreshRateMillihertz>、impl$12::from 未列** |
| menu/corrupted_save_file.rs | 1 | ui | C7 ✓ |
| menu/credits.rs | 6 | ui / spawn_third_party_license_http_server / impl$3::scroll<u64> | **impl$2::{on_input,prepare_layout,render}（ScrollContent 内容 trait）未逐条** |
| menu/daily_welcome.rs | 3 | ui | ui::closure$1/closure$3 未逐条（C4 仅以「is_weekly 分流」示意） |
| menu/load.rs | 13 | ui / thumbnail_scroll / display_save_file_load_error / load_session_meta / show_contextual_buttons | **impl$3::{render,on_input,prepare_layout,process_input_action}（ThumbnailScrollContent trait）未逐条；ui::closure$7/8 已提及** |
| menu/mod.rs | 3 | impl$0::build / impl$1::build / advance_menu_state | C1 ✓ |
| menu/new_session.rs | 2 | ui | ui::closure$0 未逐条（C3 以 closure_env_0 示意） |
| menu/report_bug.rs | 16 | ui / try_remove_file / text_edit / toggle_icon | **impl$5::drop、impl$6::{prepare_layout,render,on_input,process_input_action}（TextEditContent trait）、impl$8::{prepare_layout,render}、serialize<serde_json> 未逐条** |
| menu/save_error.rs | 4 | ui | closure$2/3/4 已提及 ✓ |
| menu/splash.rs | 1 | ui | C5 ✓ |
| menu/welcome.rs | 3 | ui | closure$1/closure$2 已提及 ✓ |
| photomode/lut_gallery.rs | 1 | lut_gallery::ui | C15 ✓ |
| photomode/mod.rs | 1 | do_photomode | C11 ✓ |
| photomode/photomode_buttons.rs | 1 | photomode_buttons | C12 ✓ |
| photomode/setup_photomode_toolbar.rs | 1 | setup_photomode_toolbar | C13 ✓ |
| photomode/ui_slider.rs | 3 | ui_slider / ui_slider_reset / slider_val_01 | C14 ✓ |
| **合计** | **509** | | 见 §2.1 遗漏汇总 |

### 2.1 遗漏汇总（按严重度）

- **高**：`taffy_finalize_element_size`（0x142340890）——B3 只覆盖 `taffy_update_drawables`。抽查确认其 size 1152、遍历同款「两套映射 +0x108」、逐节点调 `tg_text::TextFragment::bounds`、经 `taffy_mut_style<perform_layout::closure_env$1>` 写样式——是 drawables 的「定稿/文本包围盒」伴生函数，与 TaffyLayout 数据同簇，真实业务，必须补。
- **中**：tweaker 的 enum/list 单态族（`TweakableEnum<…>×6`、`TweakableList<…>×6`、`tweaker_w_font_size`、`impl$8::format` 另 4 个）；NodeContext::{sounds,hover,interaction,node,id,with_layout,content×6,content_mut<FrameContent>}；菜单内容 trait impl（credits impl$2、load impl$3、report_bug impl$6/impl$8/impl$5::drop/serialize）。这些在 ANALYSIS 里被「NodeContext 系列 / 内容结构 / 构建器同构」泛化覆盖，未逐条点地址；建议补一条「分组单态地址段」清单即可闭合。
- **低**：NodeContent::on_input<…>×8（见 §4 虚表缺槽）；mei/core iter_subtree×2（算法已在 B11.18 描述，归属未注）；NodeContent::interactive/pre_update/render/process_input_action 单态与 impl$21::{as_any,as_any_mut,type_name}×13、impl$20::default（多为空壳/胶水）；RON/serde deserialize visit_* 胶水；menu 各 ui::closure$n 多数已在正文以 closure_env 提及。

---

## 3. 逻辑总清单

逐函数逻辑清晰度评估（「讲清 / 部分 I / 含糊 / 缺环节」）：

**A mei 框架**
- `reuse_or_create_node`（0x683ec0）：讲清（hashbrown 探表→未中 reserve/remove_entry→身份回收→初始化）。U：+0x1b0、0xf3 位、两个栈字段——合理。
- `safe_frame`/`new_at_pos<&str>`：讲清；`new_at_pos<String>` 结构被 A1.3 描述并被抽查反编译**逐条验证**（tag=4、+4/+8/+0xc/+0x10 x/y/w/h、+0x14 Vec2、+0x1c、+0x30 id、stride 0x38、返回{events,id,0}）。
- `mei_pre_update`（0x6842e0）：流程讲清（输入合并→handler 派发→保存/恢复→pre_update→命中/hover/drag/click→事件派发→双缓冲交换）。U 项合理。
- `mei_post_update`（0x6884c0）：两遍 render+DFS 拓扑 z 序+taffy 最小尺寸+焦点/nav，流程讲清；render 返回值、+0xa8/+0xac、焦点判据、nav 动作名留 U 合理。
- Node 方法/NodeContext 系列/config/instantiate_ui_template_impl/widgets/tweaker/decorate_hovered：结构讲清；tweaker 枚举取模、内容字段布局留 U 合理。

**B 布局/系统/交互**
- `TaffyLayout::style/perform_layout`、`make_layout_absolute`：讲清（排序→compute→树形绝对化下推）。U：排序键、0x2fc/0x310、0x90/0x91 合理。
- `taffy_mut_style` 十单态：分组讲清（clone→改→set_style），closure 内联片段见各调用者。
- `toolbar_setup`/`setup_screen_and_toolbar_nodes`/`add_contextual_menu`/`add_toolbar_item`/`link_up_nav`/`UndoRedoLayout::setup`：讲清，导航环 (i±1)%n 明确。
- `toolbar_alignment`/`undo_redo_alignment`/`auto_hide` 族：标 I，jumptable 破坏下各 case 偏移公式留 U——诚实合理。
- `toolbar_button` 四函数、`toolbar_button_size_and_padding_update`：标 I，命中/焦点/动画事件逻辑讲清；枚举映射、padding 数值留 U。
- `ui_interaction` 18 函数：主线讲清（渲染桥接/hover/拖拽/点击事件、双通道动画状态机、4 个 ui_*_animation、焦点上推与父聚焦显隐、ui_nav_update+iter_subtree 方向最近搜索、hide_ui/debug_hide_ui/sort/notify）。`ui_sounds` 边界错并如实标注；`ui_nav_update` jumptable 破坏如实标注。
- `demo_stuff`：讲清；137.88582=glade 半径已明示为推断（非默认值猜测）。

**C 菜单/拍照/文本/许可/插件**
- `advance_menu_state`：讲清（pending→current→prev、判别子改 0xb 避 Drop、变体 2 写设置文件），差异登记具体。
- welcome/daily_welcome/splash/corrupted/save_error/credits/load/report_bug 的 `ui`：均为状态门控编排，主流程讲清；welcome +0xe5/+0x206 归属、report_bug 提交格式、各模板 id 串留 U 合理。
- photomode `do_photomode`/`photomode_buttons`/`setup_photomode_toolbar`/`ui_slider`/`ui_slider_reset`/`lut_gallery`：编排讲清；动作字节跳表/滑块被截断分支留 U。
- text 链、SingleFileServer、lib.rs 插件：讲清；SSBO 名/字体目录/Content-Type 留 U。

**缺环节/含糊点**
1. A 帧事件（stride 0x38）与 MeiInputEvent（stride 0x70、tag 0xc3..0xc6）关系未讲清：A1.2 说 safe_frame 事件「stride 0x38」、A0.4 说 MeiInputEvent「stride 0x70」——两个结构在 core 里如何分工（一个给 UI 帧事件、一个给键盘/剪贴板）未明示；抽查确认新 at_pos 事件实为 iced_core::event::Event（grow_one<Event>），建议 A 明确标注。
2. A0.3 Node 字段图与 C 的 alpha 写偏移（+0xb0/+0xb8）未对齐（见 §4.2）。
3. B1.2/4.1「元素 0x28B」与 B8.2「anchored（0x28）」步长不一致（笔误）。

---

## 4. 冲突与需裁定项

**4.1 `new_at_pos<String>` 无物理函数（stale，建议关闭卡点）**
A1.3 及已知卡点称 `new_at_pos<String>`(RVA 0x2289f80) 与 `NodeContext::mei`(0x885d0) 均无物理函数。抽查：**`new_at_pos<String>` 在 0x142289f80 存在**（268B，name/size/body 均正常，反编译干净），其结构逐字段吻合 A1.3；仅 `NodeContext::mei` 确无函数。**裁定方向**：不改代码；更新 ANALYSIS A1.3 与卡点清单——`new_at_pos<String>` 可反编译，去掉「以 <&str> 单态为准」的降级，卡点仅保留 `NodeContext::mei`。

**4.2 Node 布局偏移 A/C 不一致**
A0.3 定义 Node：+0xa4 opacity=1.0、+0xb4/+0xb5 声音、+0xb6 状态、+0xb7 行为。C 多处写 alpha：welcome「写节点+0xb0」、daily_welcome「alpha 写+0xb8」、save_error「alpha -10（未标偏移）」；A1.5 用 +0xa8/+0xac 累积、A6 写 +0xa8=100.0。**+0xa4 与 +0xb0/+0xb8 关系未定义**。**裁定方向**：不改代码；A 的 Node 字段图需补 +0x5a/+0x5b/+0x5d（C 多处置位）+0xa8/+0xac（描边/累积）+0xb0/+0xb8（alpha 通道）并说明与 +0xa4 关系（可能不同通道或 C 偏移命名有误）。

**4.3 NodeContent 虚表缺 `on_input` 槽**
A0.3 虚表仅列 type_name/interactive/process_input_action/pre_update/render/draw 六槽；但发射清单含 `NodeContent::on_input<…>`×8 独立单态（TodCircle/Frame/Image/Label/PillButton/TabSwitcher/Tweaker/ToggleIcon）。**裁定方向**：不改代码；A 需补 on_input 槽或说明其与 process_input_action 的别名关系（on_input 单态地址可一次 get_functions 补出）。

**4.4 `tag`/`判别子` 术语冲突**
A0.4 用 tag∈{0xc3..0xc6}（MeiInputEvent）、A1.2/1.3/1.5/6 用 tag∈{0..6}（iced Event 判别子）、C 用「判别子」。同词三种含义。抽查确认帧事件为 iced_core::event::Event（判别子 1..6）。**裁定方向**：不改代码；A 统一术语（iced Event=判别子；MeiInputEvent=tag），避免实现时误用。

**4.5 两套 UI 归属无冲突（确认）**
TaffyLayout/anchored_nodes 属 B（工具栏实体 UI）；mei 用 HashMap<GlobalId,Node> 保留模式。A 不涉及 anchored_nodes；B 不涉及 mei 节点表。两套并行、共享 taffy 与 UiState 注册模式，不冲突。mei/core 的 iter_subtree×2（update_nav）与 B11.18 同一算法，归属需注明（低）。

**4.6 其他小笔误**
B1.2/4.1「元素 0x28B」应为 0x28；A0.1 子 id 种子写法（(u64,TypeId)/(u64,u64) 与 decorate_hovered 的 (u64,u32) 计数）表述需统一；C14 ui_slider_reset case7 `exp2(clamp(v,-2,2)-3)` 与 B6.1 `param_to_value=exp2f(p+self[3])` 是否互为逆（value_to_param）需核对。

**4.7 已知卡点核对汇总**
- `color_menu`/`show_color_options`：发射清单无此二符号 → 确属 system-color，不在本模块 ✓
- `ui_sounds` 尾部边界错并：**坐实**（并入 drop_ptr 组件注册尾块）✓
- `toolbar_alignment`/`ui_nav_update` jumptable 破坏：均已标 I 并入 U ✓
- `new_at_pos<String>` 无物理函数：**不成立**（有物理函数），见 4.1

---

## 5. 遗留 unknown 汇总

A（12 项）、B（12 项）、C（8 项）清单整体**合理**：多为字段语义/枚举映射/jumptable 分支/字符串常量，与 O/I/U 标注一致，未见「未标 unknown 却断言默认值」的情况（例外即 4.1 的 stale 断言）。需更新的仅：
- A1.3 的 `new_at_pos<String>`「无物理函数」条目（移除）；
- A0.3 虚表 + on_input 槽、Node +0xb0/+0xb8 字段（补查）；
- 建议把 A 未知 12 项中「NodeContent 默认方法真实返回值（地址别名）」标注为「已知空壳，保留 U」以免实现期反复回查。

其余 unknown（handler 条目结构、justify 全枚举、AnchoredNode 排序键、ToolbarAlignment 各 case 偏移、两套映射语义、EscMenuState 完整变体表、模板 id 串、滑块截断分支、SSBO 名等）均属合理待查，无过度断言。

---

## 6. 整体评价与建议

**评价**：ANALYSIS.md 三部分为本次复核所见最高质量分析之一——O/I/U 纪律严格、地址与字节常数具体、差异登记如实、未知不猜默认值。A/B/C 的 GlobalId 派生、状态门控、两套 UI 并行等主线索自洽，抽查的三条关键结论（ui_sounds 边界错并、new_at_pos 结构、taffy_update_drawables 描述）全部吻合真实反编译。

**建议（按优先级）**：
1. **补 `taffy_finalize_element_size`**（0x142340890）：与 `taffy_update_drawables` 同批反编译，补入 B3 与 TASKS UM-TOOL-10 任务内容。
2. **关闭/修订 `new_at_pos<String>` 卡点**：函数存在可反编译，把 `NodeContext::mei` 单独列为唯一无物理函数项。
3. **A 补 Node 字段图**（+0x5a/+0x5b/+0x5d/+0xa8/+0xac/+0xb0/+0xb8 与 +0xa4 关系）与 **NodeContent on_input 槽**。
4. **补分组单态清单**：tweaker enum/list 族、菜单内容 trait（credits impl$2 / load impl$3 / report_bug impl$6·impl$8）、NodeContext 剩余方法，一次 get_functions 收地址段即可闭合覆盖缺口。
5. 统一 `tag`/`判别子` 术语；修正 0x28B 笔误；核对 param_to_value/value_to_param 是否互为逆。

**复核证据**：`jobs/review-spotcheck.json`(-result)、`jobs/review-no-phys.json`(-result)、`jobs/review-newsatpos.json`(-result)。
