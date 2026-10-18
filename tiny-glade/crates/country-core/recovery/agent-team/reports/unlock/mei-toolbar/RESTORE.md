# RESTORE — unlock/mei-toolbar（依赖线 #8+#10：mei DSL + render_taffy 13 参 + 共享 UI 类型 + taffy setter closures）

> restorer-unlock-mei-toolbar，2026-10-17。硬标准：每项 fresh 反编译 → 逐语句判等；
> 无法闭合标未实现并写分歧点，不落近似体。工具：ghidra `--worker shared`（只读）；
> 证据在本目录 `work/unlock-mei-toolbar/`（`evidence/`、`jobs/`）。
> 身份：B patched EXE（`cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`）。
> 红线遵守：只改 `crates/systems/toolbar/`（src + Cargo.toml）；未改 shared / 他模块。

## 0. 结论速览

| # | 条目 | verdict | 关键交付 / 分歧 |
|---|---|---|---|
| 1 | `mei_pre_update` 0x1406842e0 接口 | **判等（9 参签名）**；主体**未实现** | `mei/systems.rs`；输入→handler 状态机/双缓冲交换 U |
| 2 | `mei_post_update` 0x1406884c0 接口 | **判等（9 参签名）**；主体**未实现** | 同上；update_nav/taffy 最小尺寸/render 两遍/z-order U |
| 3 | `render_taffy_ui_to_widgets` 0x142338990 | **判等（13 参接线）**；主体**未实现** | `ui_interaction.rs`；core-type/API gap（Ui2dExtent/Ui2dImage/DragState）|
| 4 | `UiNav` 布局 | **判等**（0x40 B / 4×16B link） | `resouces_and_components/mod.rs`；槽语义名 U |
| 5 | `HistoryButton` 布局 | **判等**（1 B `{kind}`） | 同上；0/1 语义映射 I |
| 6 | `link_up_nav` 0x141e4d880 payload 写面 | **重建**（I：tag=3 语义 U） | `UiNav::{horizontal,vertical}` + `link_up_nav(ids, horizontal)` |
| 7 | 20× `register_required_components` 组件定义 | **判等（尺寸/布局）** | 20 个 `ComponentDescriptor` size 立即数逐项；胶水不重建 |
| 8 | `UiInteractionState`/`UiMouseInteractable` 尺寸 | **判等**（3 B / 4 B） | 修正 wave3 的 4B/16B 推断 |
| 9 | 8× taffy setter closure（已实例化） | **判等** | closure_env 0/2/10/11/12/13 + undo_redo 0/1 字段写面 |
| 10 | 其余 6× alignment closure（1/3..9） | **未实现（无单态）** | B 未实例化 `taffy_mut_style`；不伪造 |
| 11 | `toolbar_alignment` case 序列 0..3/5/7/9/10/0xb | **判等（跳表+序列）**；case 4/6**未实现** | `apply_alignment_setters` |

**计数（行级）**：判等 **9** / 重建 **1** / 未实现 **4 组（6 个 closure 编号 + case 4/6 + 3 个函数主体）**。
**编译**：`cargo check -p toolbar` **0 错误**（toolbar 侧 0 warning；country-core 的 14 条既有
warning 非本批）；`cargo test -p toolbar --lib` **6/6 通过**（含本批新增 3 测试）。

## 1. mei DSL 使能面（依赖线 #8）

### 1.1 系统签名（判等，O）

`procs.tsv` 的 `run_unsafe<FunctionSystem<...>>` mangled 名逐参核对 + 本批 fresh 反编译：

`mei_pre_update`（0x1406842e0，7382 B）9 参：
```
ResMut<MeiContext>, EventReader<bevy_input::mouse::MouseWheel>, ResMut<UiNavState>,
Res<WindowSize>, Res<Time>, Res<ActionSetStates>, Option<ResMut<UiState>>,
Option<Res<CursorPositionSS>>, EventWriter<PlayMenuUiSound>
```

`mei_post_update`（0x1406884c0，14583 B）9 参：
```
ResMut<MeiContext>, ResMut<tg_text::TextRenderer>, ResMut<UiNavState>,
ResMut<AssetTextureLibrary>, Res<GraphicsDeviceResource>, Res<WindowSize>,
Option<Res<SteamClientResource>>, Option<Res<SteamInput>>, Res<ActiveInputModeTracker>
```

落库：`mei/systems.rs`（含 `GameConfig` 别名）。为 `MouseWheel` 在 toolbar
`Cargo.toml` 增加 `bevy_input = "=0.16.0"`（与 `tiny-glade` 同写法；`default-features=false`
会让 `bevy_math` 缺 `libm/std`，已纠正）。

### 1.2 主体（未实现，分歧点）

- `mei_pre_update`：fresh callees 证 `process_input_action_handlers` 0x140687eb0、
  `MeiInputEvent` drop、`ActionSetAction` retain closure_env$2、`UiWidgetContext::clicked`、
  `WidgetParams::{new,no_history_recording,occlude_by_world,sounds,allow_negative_distance}`
  （0x1409a8e10/8f10/8ee0/8e80/8f40）、`UiState::add<String>`、`grow_one<EventInstance<
  WindowFocusEvent>>`、`slider3d::into_drag_state`/`is_started`/`is_finished`/`is_some`、
  `Time::elapsed`。**分歧**：输入 0x70 载荷 tag 0xc3/0xc4/0xc5/0xc6 的完整 handler case
  体未逐段取证；双缓冲交换/`previous_nodes` GC 写序 U；`PlayMenuUiSound` 槽映射 U。
- `mei_post_update`：fresh callees 证 `update_nav` 两 `iter_subtree`（0x140f9e3e0 /
  0x140f9e610）、`impl$17::perform_layout` + `compute_min_size_with_children`（taffy 最小
  尺寸）、`compute_z_order_and_toposort`（`sort_unstable_by` + `insertion_sort_shift_left`/
  `ipnsort`）、`tg_text::ShapeStream`/`TextRenderer`、`encode_no_images`/`fill_color`/
  `corner_radius`/`rect`（绘制两遍）。**分歧**：两遍顺序与阈值、z-order 比较器
  `impl$17::post_update::closure_env$0`、StackEntry 布局、最小尺寸映射闭包 U。
- 解锁条件：`impl$17` 三方法与 `update_nav` 闭包体逐语句取证（当前 Ghidra 内联碎片）。

## 2. `render_taffy_ui_to_widgets` 13 参接线（依赖线 #10）

### 2.1 13 参（判等，O）

来源：`procs.tsv` 的 `FunctionSystemState<tuple<...>>`（0x1405d2600 等）与本批 fresh
0x142338990。逐参：

1. `Query<(Entity, &Ui2dExtent, &UiMouseInteractable, &TaffyNode, &mut UiInteractionState,
   Option<&HistoryButton>, Option<&UiNav>)>`
2. `Res<UiState>`
3. `ResMut<UiNavState>`
4. `EventWriter<UiClickEvent>`
5. `EventWriter<UiDragEvent>`
6. `Res<AppMode>`
7. `Res<UiSignifierStream>`
8. `Res<CursorPositionSS>`
9. `Res<GameConfig>`
10. `Res<ActionSetStates>`
11. `EventWriter<UiHistoryButtonPressed>`
12. `Commands`
13. `Res<ToolbarSizeMult>`

落库：`ui_interaction.rs` 的完整签名（`ToolbarSizeMult` 新建于 `toolbar_update.rs`，
单 f32；fresh 中作为 `*local_208` f32 参与 rect 扩张）。

### 2.2 已解主体结构（保存为分歧说明，不落近似体）

`QueryIterationCursor::init` → 逐 widget `format_inner`（`"toolbar_button_"` 拼接）→
`WidgetParams::new` + `no_history_recording` + `occlude_by_world` + `allow_negative_distance`
+ `sounds(empty)` → `UiState::add<String>`；`HistoryButton` 分支 `slider3d::into_drag_state()`
（is_some → `WindowFocusEvent`；==3 → `UiPlaySelectAnimation{0.2,0.2}`）；无 HistoryButton
分支 `!=0xd && is_active` → `SignifierFactory::new` + `UiSignifierStream::write`；
`clicked()` → `WindowFocusEvent`；`into_drag_state()!=2` → `(str, SettingsMenuState)` 事件写。

### 2.3 未实现（分歧）

- **core-type-gap**：`country_core::components::Ui2dExtent` 声明 8B（原生 **0x10**）、
  `Ui2dImage` 声明 8B（原生 **0x20**）——rect/图像载荷不可构造（本批用 fresh
  `register_component` 尺寸证据登记；core 团队可直接领）。
- **core-api-missing**：`slider3d::{DragState, impl$1::into_drag_state, is_started,
  is_finished, is_some}` 未落库（`slider3d.rs` 仅占位）。
- **载荷 U**：`WidgetParams::new` 实参（volume/extra_78）、`format_inner` 实体格式、
  `UiSignifierStream` 40B 记录、`(str, SettingsMenuState)` 的 str 来源、`UiNavState` 命中门。
- PDB 路径为 `country_core::components::{UiNav,HistoryButton,UiInteractionState,
  UiMouseInteractable}`；这些类型当前仅 toolbar 本地（core 只有 registry 条目），
  移库属 core 团队任务。本批 Query 使用本地同布局类型。

## 3. UiNav / HistoryButton 布局（U→实现）

### 3.1 UiNav（0x40 B，判等）

- **尺寸证据**：`register_component<UiNav>` 0x141e36510，`ComponentDescriptor` size
  立即数 `0x40`（`disasm-comp-0x141e36510.txt`）。
- **写面证据**：`ToolbarLayout::link_up_nav` 0x141e4d880（fresh，`decomp/0x141e4d880-link_up_nav.c`）。
  逐指令得到四槽 16B `{tag: u64@+0, value: u64@+8}`：
  - `in_DL==0`（横向）：slot0/slot1 = 0；slot2 = `{3, next}`；slot3 = `{3, prev}`；
  - `in_DL!=0`（纵向）：slot0 = `{3, prev}`；slot1 = `{3, next}`；slot2/slot3 = 0。
- **落库**：`UiNavLink` + `UiNav { links: [UiNavLink;4] }`（size assert 0x40）、
  `UiNav::horizontal/vertical`、`link_up_nav(ids, horizontal)`（prev=(i+n-1)%n、
  next=(i+1)%n；命令队列插入由消费接线负责）。
- **U**：tag=3 的枚举语义（3 变体 enum 的 niche `None` vs 具体变体）与槽语义名
  （up/down/left/right 顺序）未取证；按索引/原值落库，不猜名称。
- **I 级残差**：未连接槽在原版只显式写 tag qword=0（横向 slot0/1、纵向 slot2/3），
  value qword 未显式写（栈初值 U）；本实现取 0（`Default`），登记为重建选择。

### 3.2 HistoryButton（1 B，判等）

- **尺寸证据**：`register_component<HistoryButton>` 0x141e34b90，size 立即数 `0x1`。
- **写面证据**：`UndoRedoLayout::setup` 0x140fad870（fresh）：静态表
  `("icon/undoicon/redo", flag=0)` / `("icon/redo", flag=1)` → insert 记录
  `{fn@0, handler@8, entity@0x10, payload@0x18}` 写 `byte = flag & 1`。
- **落库**：`HistoryButton { kind: u8 }` + `UNDO=0` / `REDO=1`（语义映射 I）。
- 同函数还证明 `UndoRedoLayout::setup` 的 add_child 链与 8 组件 spawn，可作后续重建输入。

## 4. 26× register_required_components 组件定义

B 侧口径：`toolbar::resouces_and_components::{impl$2..impl$27}` 共 **20** 个
`register_required_components` 单态（procs.tsv 逐地址；impl 号 2/3/4/5/6/13..27），
加 6 个共享 UI 类型 = **26**。`register_required_components` 本体是 bevy
`#[derive(Component)]` 胶水（AGENTS.md glue-legitimate），不逐函数重建；组件
**类型/尺寸**按 fresh `ComponentDescriptor` size 立即数落库。

| 组件 | size | 证据地址 | 处置 |
|---|---|---|---|
| TaffyNode | 0x14 | 0x142383770 | 20B `{node: Node(8,align4), _tail:[u8;12]}` |
| UiScaleWithUserSetting | 0x0 | 0x142383e60 | ZST（原 16B 推断作废）|
| DebugToolbarActivationZone | 0x0 | 0x142384550 | ZST（新增）|
| SwitchAppModeButton | 0x10 | 0x142384c40 | `{mode: AppMode}` |
| SwitchToolbarCategoryButton | 0x1 | 0x142385330 | `{category: ToolbarCategory}` |
| UiAnimationState | 0x10 | 0x142385a20 | 既有 16B 布局判等 |
| UiShowIfParentIsInFocus | 0x8 | 0x142386110 | `{parent: u64}`（字段 U）|
| ShowOnlyInFpvAndFree | 0x0 | 0x142386800 | ZST |
| UiCustomSize | 0x8 | 0x142386ef0 | `{size: Vec2}`（新增）|
| ToolbarButtonCooldown | 0x4 | 0x1423875e0 | `{remaining: f32}` |
| UiPlaySelectAnimation | 0xC | 0x142387cd0 | `{a,b,u32}`（原 16B 作废）|
| UiLocked | 0x0 | 0x1423883c0 | ZST |
| UiPlayLockedAnimation | 0xC | 0x142388ab0 | `{a,b,u32}`（新增）|
| ToolbarExpand | 0x0 | 0x1423891a0 | ZST（新增）|
| TimeOfDayButtonV2 | 0x0 | 0x142389890 | ZST（原 16B 作废）|
| HideIfMenuIsOpen | 0x0 | 0x142389f80 | ZST（新增）|
| ShowOnlyInPhotomode | 0x0 | 0x14238a670 | ZST（新增）|
| PhotomodeButton | 0x1 | 0x14238ad60 | `{tag: u8}`（原 16B 作废）|
| UiSlider | 0x18 | 0x14238b450 | typed-gap 24B（原 16B 作废）|
| UiSliderReset | 0x1 | 0x14238bb40 | `{flag: u8}`（新增）|
| UiNav | 0x40 | 0x141e36510 | 见 §3.1 |
| HistoryButton | 0x1 | 0x141e34b90 | 见 §3.2 |
| UiInteractionState | 0x3 | 0x141e34530 | `{state,pad,flag}`（原 4B/16B 作废）|
| UiMouseInteractable | 0x4 | 0x141e35eb0 | `[u8;4]`（原 16B 作废）|
| Ui2dExtent | 0x10 | 0x141e35850 | core（typed-gap 8B 需修）|
| Ui2dImage | 0x20 | 0x141e33ed0 | core（typed-gap 8B 需修）|

落库：`SHARED_UI_COMPONENTS` 台账 + `register_local_components(world)`（24 本地项）。
测试 `shared_ui_component_sizes_match_fresh_descriptors` 逐项断言。

## 5. taffy_mut_style closure_env_0..13

### 5.1 已实例化 8 个（判等）

`procs.tsv` 全表：toolbar_alignment 只有 6 个 `taffy_mut_style` 单态
（closure_env 0/2/10/11/12/13），加 undo_redo 2 个（0/1）；共 8 个。每函数
`PUSH...` → `Taffy::style` 0x140fe7cb0 → `Style::clone` 0x142374fd0 →
`MOV byte [RSP+off], imm`（inline closure 写）→ `Taffy::set_style` 0x140fe7b10。
Style 偏移用 workspace taffy 0.3.13 探针实测（`evidence/taffy-style-offsets.txt`）：
`flex_direction@0x14b`、`align_items@0x14d`、`justify_content@0x152`；Option niche：
`Some(FlexStart)=2`、`Some(FlexEnd)=3`、`Some(Center)=4`。

| 单态 | 地址 | 写面 | 落库 setter |
|---|---|---|---|
| toolbar_env$0 | 0x142375c10 | `[0x1c2]=2` | `justify_content=Some(FlexStart)` |
| toolbar_env$2 | 0x142375eb0 | `[0x1bb]=1` | `flex_direction=Column` |
| toolbar_env$10 | 0x142375b40 | `[0x1bb]=0` | `flex_direction=Row` |
| toolbar_env$11 | 0x142375ce0 | `[0x1bd]=3` | `align_items=Some(FlexEnd)` |
| toolbar_env$12 | 0x142376050 | `[0x1c2]=4` | `justify_content=Some(Center)` |
| toolbar_env$13 | 0x142376300 | `[0x1bd]=2` | `align_items=Some(FlexStart)` |
| undo_redo_env$0 | 0x142376230 | `[0x1c2]=3` | `justify_content=Some(FlexEnd)` |
| undo_redo_env$1 | 0x142375f80 | `[0x1bd]=4` | `align_items=Some(Center)` |

落库：`toolbar_systems::alignment_closures::*`（8 函数）；`toolbar_systems::tests` 逐字段断言。

### 5.2 未实现（分歧）

- `toolbar_alignment::closure_env_1/3/4/5/6/7/8/9` 在 B 中**没有**
  `taffy_mut_style` 单态（procs 全表仅 8 个 + `perform_layout` / `button_size_and_padding`
  两个），说明未实例化或已内联无独立符号。**不伪造 setter**。
- 跳表 0x14304e6d8 20 项已解码（`evidence/batch4-mem.json`）：case 0/1/2/3/5/7 为
  上述 setter 序列；9/10/0xb 为 `style+clone+set_style` 空 setter；4/6 指向共享二跳
  0x14234485d（未恢复）；8/0xc..0x13 为非 taffy 路径（写资源/缓动/算术）。
  落库 `apply_alignment_setters`（case 0/1/2/3/5/7/9/10/0xb 判等；4/6 未实现）。

## 6. 证据索引

`work/unlock-mei-toolbar/evidence/`：
- `batch1.json` + `decomp/0x141e4d880-link_up_nav.c`、`0x140fad870-setup.c`、
  `0x142344560-toolbar_alignment.c`、8× `0x142375*.c`（taffy 单态）
- `batch2-disasm.json` + `disasm-0x142375*.txt`（8 单态逐指令）
- `batch4-mem.json`（跳表 0x14304e6d8 原始 80B + 解码）
- `batch5-alignment-disasm.json` + `disasm-toolbar_alignment.txt`（函数边界 201B——
  Ghidra 边界止于 jmp rax 的旁证）
- `batch7-compmeta.json` + `disasm-comp-{0x141e36510,0x141e34b90,...}.txt`
  （UiNav/HistoryButton 注册/clone/write_target 面）
- `batch8-regcomp.json` + 20× `disasm-reg-0x14238*.txt`（size/align 立即数）
- `batch9-mei.json` + `decomp/0x1406842e0-mei_pre_update.c`、`0x1406884c0-mei_post_update.c`
- `batch10-coreui.json` + 4× `disasm-coreui-*.txt`（core UI 尺寸）
- `batch11-render.json` + `decomp/0x142338990-render_taffy_ui_to_widgets.c`（本批 fresh）
- `taffy-style-offsets.txt`（taffy 0.3.13 workspace 探针输出）
- 复用（跨目录）：`reports/wave3/toolbar/evidence/fresh-0x142338990-*.c.txt`（与
  batch11 fresh 交叉核对）；`reverse-engineering/tiny-glade/evidence/toolbar-origin-2026-10-16/procs.tsv`
  （函数/单态/签名总表）。

## 7. 未实现/分歧总账与解锁条件

1. `mei_pre_update` 主体：输入载荷 tag 分派 case 体、双缓冲 GC 写序、声音槽映射。
2. `mei_post_update` 主体：update_nav、taffy 最小尺寸、render 两遍、z-order 拓扑排序。
3. `render_taffy_ui_to_widgets` 主体：core `Ui2dExtent`(0x10)/`Ui2dImage`(0x20) 字段面、
   `slider3d::DragState`/`into_drag_state` 族、`WidgetParams::new` 实参与事件载荷。
4. taffy：closure_env 1/3..9 无单态；case 4/6 二跳 0x14234485d 未恢复；
   case 8/0xc..0x13 非 setter 路径（资源/缓动/算术）未在本批范围。
5. `UiNav` tag=3 语义、HistoryButton kind 0/1 命名映射（字节值已判等，名称 I/U）。

## 8. 编译/测试

```
cargo check -p toolbar      # 0 errors（toolbar 侧 0 warnings）
cargo test  -p toolbar --lib # 6 passed; 0 failed
```
（`country-core` 的 14 条 warning 为其既有项；本批未改 shared。）
