# RESTORE — wave3 toolbar 布局/系统/交互族（taffy setter 目标 + 按钮族 + 动画状态）

> restorer-wave3-toolbar-layout，2026-10-17。任务：把总账 §toolbar 的布局/系统/
> 交互族未实现函数落库（逐语句判等，硬标准；还原不了标未实现+分歧点）。
> 方法：ghidra `--worker shared` fresh 反编译（10 函数 signature/callers/callees +
> 4 taffy_mut_style 单态 + render_taffy_ui_to_widgets 全量）+ caller mangled 名
> `run_unsafe<FunctionSystem<...>>` SystemParam 签名逐参核对 + bevy 0.16 Query/
> EventInstance/EventId 布局对回（Query {world@0,state@8,last_run@0x10,this_run@0x14}、
> EventInstance {event_id u64@0, event@8}）。证据：`wave3/toolbar/`（本目录 jobs/evidence）。
> 身份：B patched EXE（ghidra shared 核对通过，`cf048e27...`）。

## 0. 结论速览

| 函数 | VA | 判定 | 交付 |
|---|---|---|---|
| `toolbar_alignment` | 0x142344560 | **重建（前置+分派结构）+ 未实现（setter 目标）** | 前置清位判等（FIX-A `&3` 补显式）；分派 case 结构登记；`taffy_mut_style` 骨架（判等） |
| `toolbar_button_keep_focus` | 0x1423414b0 | **判等（四步；FIX-T1 组件同构+接 schedule）** | 冷却 0.3/重置/any_keep/keep 写面；`interaction.keep` |
| `switch_app_mode_button` | 0x142341fe0 | **未实现（FIX-B 降级）** | 二进制核心描述；Rust 诚实壳 |
| `switch_toolbar_category_button` | 0x1423427f0 | **判等（门/载荷/选中；FIX-T1+接 schedule）** | ModeJustChanged 存在门+未选中门；`interaction.selected` |
| `tod_v2_button` | 0x142342f30 | **判等/重建（FIX-T1+T2）** | click→toggle→ModeJustChanged 首 tag≠0xd 清→写 selected；命中门代理登记 |
| `ui_animation_state_update` | 0x14233b3f0 | **公式判等（FIX-T1+T2 state 写面）** | 四分支/state 0/1/2/appear sign=`interaction.state` |
| `toolbar_auto_hide` | 0x142343a10 | **未实现** | 公式/常量已取证；系统参数接线 U |
| `ui_sounds` | 0x14233a7f0 | **未实现** | 9 SystemParam 签名已证；case 体截断 |
| `ui_slider` | 0x1423486d0 | **未实现** | callees 已证；case 体截断 |
| `render_taffy_ui_to_widgets` | 0x142338990 | **未实现** | Query init 面 + WidgetParams 链已证；接线 U |

**类型落库/校正 10**：`UiAnimationState`（16B 布局**判等**校正）、`UiInteractionState`
（4B 校正）、`ToolbarButtonCooldown`/`UiLocked`/`SwitchAppModeButton`/
`SwitchToolbarCategoryButton`/`UiSlider`/`PhotomodeButton`/`TimeOfDayButtonV2`/
`UiPlaySelectAnimation` + 事件 `UiClickEvent`/`UiDragEvent`/`UiJustBecameInFocusEvent`/
`UiToolbarButtonResetAllCooldowns`。

**编译**：`cargo check -p toolbar` **0 错误 / 0 警告**（toolbar 侧；country-core 依赖侧
9 既有警告非本批）；`cargo test -p toolbar --lib` **1/1 通过**（LogSlider）。注：
workspace 共享 target 下并行 agent 的 `cargo test` 曾使 rhapsody 产物失效（ash
`cmd_pipeline_barrier2` 假错误），rhapsody 独立 fresh build 后工具栏链路编译恢复
（**非本批引入**）。

## 1. 关键机理（wave3 新增）

- **bevy 0.16 Query 布局对回**：`Query {world@+0, state@+8, last_run@+0x10,
  this_run@+0x14}`（`query.rs` `pub struct Query`）——反编译 `*in_RCX`=world、
  `in_RCX[1]`=QueryState、`+0x14` u32=this_run tick 全部吻合。反编译的
  `+0xf8/+0x100/+0x108/+0x110/+0x118/+0x1a8`（QueryState/World 内部表列遍历）与
  `窗口+0x50` 子列表（stride 0x10）、每子 stride-3 字节记录 = bevy Query 迭代 + 一个
  **未具名 per-widget 字节记录容器**（`+0`/`+1`/`+2` 三字节面，见 §分歧）。
- **EventInstance 布局**：`EventInstance<E> = {event_id: EventId u64@+0, event: E@+8}`，
  stride 0x10 事件记录实体@+8 与 UiClickEvent `{entity}` 吻合。
- **taffy_mut_style 单态**（0x142375b40/0x142375ce0/0x142376050/0x142376300 fresh 逐语句）：
  `let mut s = taffy.style(node).unwrap().clone(); f(&mut s); taffy.set_style(node, s).unwrap();`
  ——closure `f`（alignment closure_env_0..13）为 setter 目标（taffy Style 字段），
  **未取证**。

## 2. 类型校正（`resouces_and_components/mod.rs`）

| 类型 | 校正 | 依据 |
|---|---|---|
| `UiAnimationState` | 16B `{appear: f32@0, select: f32@4, _gap4, state: u8@0xc, _pad3}`（原 `{state u8@0,_gap u64}` 作废）| **判等**：ui_animation_state_update fresh 读面 `{f32@0,f32@4,byte@0xc}` |
| `UiInteractionState` | 4B `{state u8@0,_pad u8@1,flag u8@2,_pad u8@3}`（原 16B 作废）| **重建（I）**：button/动画系统 stride-4 u32 写面 |
| `ToolbarButtonCooldown` | f32@0 | 重建（I）：toolbar_button_keep_focus stride-4 f32 |
| `SwitchAppModeButton` | `{mode: AppMode, _gap}` | 重建（I）：持有目标 AppMode |
| `SwitchToolbarCategoryButton` | `{category: ToolbarCategory, _gap}` | 重建（I）：目标类别 |
| `UiPlaySelectAnimation` | 16B `{a f32, b f32, u32, byte, pad3}` | 重建（I）：命令载荷观测 `{0.1,0.2}`/`{0.2,0.2}` |
| `UiLocked`/`UiSlider`/`PhotomodeButton`/`TimeOfDayButtonV2` | 标志/判别组件 | 重建（I）：ui_sounds Query 目标 |

事件：`UiClickEvent {entity}`（EventInstance 实体@+8，**判等**读面）、`UiDragEvent`
/`UiJustBecameInFocusEvent {entity}`（I）、`UiToolbarButtonResetAllCooldowns`（单元）。

## 3. `toolbar_alignment`（0x142344560）· 重建（前置+分派结构）+ 未实现（setter 目标）

签名（caller mangled）：`Res<GameConfig>, Res<ToolbarLayout>, Res<PhotomodeToolbarLayout>,
ResMut<TaffyLayout>`。

- **前置清位判等**（已有）+ wave3 fresh 复核：stride 0x28 项 `+0x24` `!=4` 时按
  `0x1020300 >> (alignment&3 << 3)` 重写。
- **分派结构**（跳表 0x14304e6d8，20 项，wave3 fresh 逐 case）：case 0→
  `taffy_mut_style<closure_env_12/11/10/11>`、1→`<closure_env_0/undo_redo::1/2/13>`、
  2→`<undo_redo::0/1/2/11>`、3→`<12/13/10/13>`、5/7→`<closure_env_13>`、9/10/0xb→
  `style+set_style` 直设、0xc/0xe→`EasingFunction::apply`+写偏移、0xd→
  `EscMenuState::is_closed`+`Aabb2`+`DigitalActionState::pressed`+Easing+写进度、
  0xf..0x13→算术/跳表转发。
- **分歧（未实现）**：closure_env_0..13 的 **setter 目标（taffy Style 字段）U**；
  alignment 0..19→锚点名 U；`+0xf0` 资源类型 U。本实现落 `taffy_mut_style` 骨架
  （**判等**）+ 分派结构登记，setter 目标不伪造。

## 4. `toolbar_button_keep_focus`（0x1423414b0）· 重建（近似）+ 分歧

签名：`Query<(&mut ToolbarButtonCooldown, &mut UiInteractionState)>,
EventReader<UiToolbarButtonResetAllCooldowns>, Res<Time>`。

**重建（逐语句，0x14234153d..0x142341dc5）**：① 逐项 keep 字节（per-widget `+1`，U）
≠0 → 冷却保持 0.3 默认、`any_keep` 置位；=0 → `冷却 -= dt`（clamp ≥0）。② 重置事件
非空 → 全部冷却置 0。③ `any_keep` 置位 → keep 字节=0 的项冷却置 0。④ 冷却>0 的项
keep 字节置 1。

**分歧（未实现）**：keep 字节记录容器类型 U；`uVar6`（this_run tick）写
UiInteractionState 值语义 U。本实现以 bevy Query 迭代表达实体遍历，冷却写入逐语句。

## 5. `switch_app_mode_button` / `switch_toolbar_category_button` / `tod_v2_button` · 未实现/重建（核心）+ 分歧

- **switch_app_mode_button**（0x142341fe0）：`Query<(Entity, &SwitchAppModeButton,
  &mut UiInteractionState)>, EventReader<UiClickEvent>, EventWriter<RequestAppModeChange>,
  Res<AppMode>, EventReader<ModeJustChanged>, Commands`。**未实现（FIX-B 降级）**：
  二进制核心（点击匹配 → 读按钮目标 AppMode，push `(ref str, SettingsMenuState)` 事件
  stride 0x18；逐项按钮目标与当前 AppMode/ModeJustChanged 判别比较 0..6 变体：
  0=判别等、1/4=字节[1]、3=u64[+2]、6=键盘字符归一；相等 → `UiPlaySelectAnimation
  {a:0.2,b:0.2}`）**均未落库**——`(str, SettingsMenuState)` push 面与签名
  `RequestAppModeChange` 载荷面不符 U、AppMode 判别字节归一逻辑 U。Rust 诚实壳。
- **switch_toolbar_category_button**（0x1423427f0）：`Query<(Entity, &SwitchToolbarCategoryButton,
  &mut UiInteractionState)>, EventReader<UiClickEvent>, EventWriter<RequestToolbarCategoryChange>,
  EventReader<ModeJustChanged>, Res<Toolbar>, Commands`。重建（核心）+ FIX-C 补 2 门：
  点击匹配 → push `EventInstance<WindowFocusEvent>`（stride 0x10，**未实现**）；逐项
  动画插入条件 = `(ModeJustChanged 事件存在) && (per-widget +2 != 1 未选中，代理) &&
  (toolbar.current == button.category)` → `UiPlaySelectAnimation{a:0.1,b:0.2}` +
  per-widget `+2` 选中标志。分歧：`WindowFocusEvent` push 面与签名
  `RequestToolbarCategoryChange` 面不符 U。
- **tod_v2_button**（0x142342f30）：`Query<(&TimeOfDayButtonV2, &mut UiInteractionState)>,
  EventReader<UiClickEvent>, ResMut<TimeOfDayScreen>, ResMut<UiNavState>,
  EventReader<ModeJustChanged>, Commands`。重建（核心）+ FIX-D 补门控：清
  `TimeOfDayScreen.visible`（O 布局 +0xa；country-core 简化 `{visible}`）条件 =
  **首事件判别 `!=0xd` && visible**（判别面 U，登记）；toggle `visible^=1` +
  `UiPlaySelectAnimation{a:0.1,b:0.2}` 按 **widget 命中门**（`Query<Entity,
  &TimeOfDayButtonV2>` 代理二进制 UiNavState hashbrown；数据源分歧登记，签名增补）。
  分歧：`visible` 偏移 +0xa vs +0；`!=0xd` 过滤未实现。

## 6. `ui_animation_state_update`（0x14233b3f0）· 公式判等；分派/符号未实现

签名：`Query<(&UiInteractionState, &mut UiAnimationState)>, Res<Time>`。

**公式判等（逐语句，code_r0x00014233b130 段）+ FIX-F state 写面补登记**：每工具栏项
`Query<(&UiInteractionState, &mut UiAnimationState)>`：
- keep 字节（per-widget 记录 `+1`，U）为 0 → 离开：`select += dt/-0.2`、state→1；
  否则进入：`select += dt/0.2`、state→0/2；
- **state 字节（+0xc）每迭代写** `uVar22`（0/1/2 转写）——`update_progress` 已含
  （进入：state!=1→`select+=dt/0.2`+state=0、否则 state=2；离开：state==0→2、
  否则 `select-=dt/0.2`+state=1）；
- `select = clamp01(select)`；`appear = clamp01(appear + 4·dt·sign)`，sign =
  字节记录 `+0`（U）为 0 → -1 否则 +1。

**分歧（未实现）**：stride-3 per-widget 字节记录容器类型 U（entering 判别与 appear
sign 源）；state 0/1/2 语义名 U；`uVar8`（this_run tick）写 UiInteractionState U。
本实现落 `{f32@0,f32@4,byte@0xc}` 布局与 select/appear/state 更新公式（`update_progress`），
字节记录源标 U；**分派/符号未实现**（本实现单路径 `entering=false`、`appear sign=+1`）。

## 7. 未实现（重建阻塞，wave3 fresh 复核后分歧登记）

- **`toolbar_auto_hide`**（0x142343a10）：公式/常量已取证（`110/10/0.3/-0.5/0.5/
  {80,40}/{40,80}/200`；`fVar55=(count-1)·10+count·110)·fVar56`、
  `fVar49=fVar56·110·0.3`、`±dt/0.3` 增量 + clamp01 写 `+0x2c`、Easing 缩放 +
  movmskpd 项查找写 `{dx,dy}`）；**未实现**：系统参数接线（`+0xf1/+0x1e` 资源类型 U）、
  switch 尾 case 截断区未取证。
- **`ui_sounds`**（0x14233a7f0）：9 SystemParam 签名已证（`EventReader<UiJustBecameInFocusEvent/
  UiClickEvent/UiDragEvent> + EventWriter<PlayUiSound> + Query<&UiLocked> +
  Query<&SwitchAppModeButton> + Query<&SwitchToolbarCategoryButton> + Query<&UiSlider> +
  Query<&PhotomodeButton>`）；**未实现**：~18 case 体截断（"Bad instruction data"）、
  0x14233fc6a/0x14233fe2d 跳表未恢复、声音槽映射 U。
- **`ui_slider`**（0x1423486d0）：callees 已证（`slider_val_01/inverse_lerp/
  just_pressed/pressed/released/contains/from_center_dims/despawn/
  spawn<Ui2dExtent,Ui2dImage,UiDrawOrder>`）；**未实现**：`switch(entry+0x10)` ~28 case
  截断、widget kind 枚举 U。
- **`render_taffy_ui_to_widgets`**（0x142338990）：Query init 面已证（`(Entity,
  &Ui2dExtent, &UiMouseInteractable, &TaffyNode, &mut UiInteractionState,
  Option<&HistoryButton>, Option<&UiNav>)`）+ WidgetParams 链（`clicked/pressed/
  into_drag_state/is_active/is_toolbar/no_history_recording/occlude_by_world/sounds/
  write`）+ `grow_one<EventInstance<WindowFocusEvent>>` + `format_inner`（taffy layout
  rect → 串）；**未实现**：13 参接线与 WidgetParams→UiState 写入链 U（UiNav/
  HistoryButton/WindowFocusEvent 未落库，core-gap）。

## 8. 证据索引（`wave3/toolbar/`）

- `jobs/fresh-sigs.json` + `results/`：10 函数 signature/body/callers/callees
  （caller mangled 名 = SystemParam 签名源）。
- `jobs/decomp-buttons.json` + `evidence/fresh-0x1423414b0|0x142341fe0|0x1423427f0|
  0x142342f30|0x14233b3f0-*.c.txt`：按钮族 + 动画系统 fresh 反编译。
- `jobs/decomp-taffy.json` + `evidence/fresh-0x142375b40|0x142375ce0|0x142376050|
  0x142376300-*.c.txt`：taffy_mut_style 4 单态。
- `jobs/decomp-render.json` + `evidence/fresh-0x142338990-*.c.txt`：render_taffy_ui_to_widgets。
- 复用：`tool-usage/jobs/toolbar-b-analysis/all-b2.txt`（toolbar_systems 族全量反编译，
  本报告 §3/§7 分派 case 与 auto_hide 公式依据）。

## 9. 未实现总账更新建议（给 coordinator）

- toolbar §4：`toolbar_alignment` → **重建（分派结构）+ 未实现（closure_env_0..13 setter
  目标）**——下步定点反编译 4 taffy_mut_style 单态 closure 调用体（已取函数骨架）；
  `toolbar_auto_hide` → 仍未实现（系统参数接线 U）；`ui_sounds`/`ui_slider` → 仍
  未实现（截断 case 需 radare2 定点反汇编 case target）。
- toolbar §interaction：`toolbar_button` 族 4 函数 → **重建（核心）+ 分歧**（per-widget
  字节记录容器、事件载荷面、`(str, SettingsMenuState)`/`WindowFocusEvent` 与签名事件
  面不符；switch_app_mode_button 降级未实现）；`ui_animation_state_update` →
  **公式判等；分派/符号未实现**（state 写面已含）。
- **新增 core-gap**：per-widget stride-3 字节记录容器（focused-window 子树）、
  `UiNav`/`HistoryButton`/`WindowFocusEvent` 组件类型、`EventWriter` 载荷面
  （`RequestAppModeChange/RequestToolbarCategoryChange` 实写 `(str, SettingsMenuState)`/
  `WindowFocusEvent`）。
- **类型校正复核建议**：`UiInteractionState` 4B（原 16B 作废）与
  `UiAnimationState` 16B（原 `{state@0}` 作废）按 wave3 fresh 读面——reviewer
  wave3-toolbar-decorator §2 已独立复核判等成立。

## 10. reviewer-wave3-toolbar-decorator FIX 落实（2026-10-17）

> 复核报告：`reports/wave3/reviews/toolbar-decorator/REVIEW.md`。本 restorer 已按
> toolbar 侧 5 项修正 src + 更新本节；判等 12（类型 4 + 结构公式 8）复核成立。

| FIX | 处置 | src |
|---|---|---|
| A `toolbar_alignment` 前置掩码缺显式 `& 3` | **已改**：`0x1020300u32 >> ((alignment & 3) << 3)`（release 等价、debug 防 `u32>>32` panic）；另登记 helper 签名 vs SystemParam 分歧 | toolbar_systems.rs |
| B `switch_app_mode_button` 判定过强 | **已改（降级未实现）**：Rust 诚实壳（全参 `_` 前缀），二进制核心（`(str,SettingsMenuState)` push、AppMode 0..6 判别比较、动画插入）描述 + 分歧登记 | toolbar_button.rs |
| C `switch_toolbar_category_button` 漏 2 门 | **已补**：动画插入条件 = `(ModeJustChanged 事件存在) && (per-widget +2 != 1 未选中，代理) && (toolbar.current==category)`；per-widget +2 写面 U 登记 | toolbar_button.rs |
| D `tod_v2_button` toggle/insert 未按 widget 命中门控 | **已补**：toggle+insert 按 `widget_entities: Query<Entity,&TimeOfDayButtonV2>` 命中门控（二进制经 UiNavState hashbrown，布局 U——数据源分歧登记；签名增补为重建决策）；`!=0xd` 事件判别过滤登记 | toolbar_button.rs |
| F `ui_animation_state_update` state 字节写面 | **已补**：`update_progress` 含 state 0/1/2 转写（进入：state!=1→`select+=dt/0.2`+state=0、否则 state=2；离开：state==0→2、否则 `select-=dt/0.2`+state=1）；标签改"公式判等；分派/符号未实现" | mod.rs + ui_interaction.rs |

## 11. 未实现总账更新建议（给 coordinator）

- toolbar §4：`toolbar_alignment` → **重建（分派结构）+ 未实现（closure_env_0..13 setter
  目标）**——下步定点反编译 4 taffy_mut_style 单态 closure 调用体（已取函数骨架）；
  `toolbar_auto_hide` → 仍未实现（系统参数接线 U）；`ui_sounds`/`ui_slider` → 仍
  未实现（截断 case 需 radare2 定点反汇编 case target）。
- toolbar §interaction：`toolbar_button` 族 4 函数 → **判等/重建（FIX-T1 后组件同构 +
  接 schedule）**；`ui_animation_state_update` → **公式判等（state 写面已含）**。
- **新增 core-gap 已消解**：per-widget 3B 记录 = `UiInteractionState`（非独立
  `WidgetByteState`，reviewer-A FIX-T1）；`UiNav`/`HistoryButton`/`WindowFocusEvent`
  已由 gap-utils-toolbar 落库。
- **类型校正复核**：`UiInteractionState` 3B `{state,keep,selected}`（wave3 4B 与 wave2b
  16B 均作废）——reviewer-A-toolbar FIX-T1 定案（descriptor size=3）；`UiAnimationState`
  16B 判等保留。

## 12. reviewer-A-toolbar FIX-T1/T2 落实（2026-10-17，高优先级）

> 复核报告：`reports/reviews/A-toolbar/REVIEW.md`。**T1（高）**：四系统原生 Query 仅 2
> 组件，3 字节记录 `+0 state/+1 keep/+2 selected` 就是 `UiInteractionState` 本身
> （descriptor size=3）；`WidgetByteState` 未注册/未挂载、四系统未接 schedule。

| FIX | 处置 | src |
|---|---|---|
| T1 `WidgetByteState` 拆分错误 | **已改**：`UiInteractionState` 归位 3B `{state@0, keep@1, selected@2}`；四系统 Query 去 `WidgetByteState`，直接读/写 `interaction.state/keep/selected`；**删除孤儿 `WidgetByteState`**（`PerWidgetButtonState` 改用 `Vec<UiInteractionState>`）；四系统接入 `configure` schedule + 注册事件/组件 | mod.rs / toolbar_button.rs / ui_interaction.rs / lib.rs |
| T2 tod_v2 清 visible 门错 | **已改**：改读 `ModeJustChanged` 首事件 `tool != 0xd`（click 之后），顺序改 click→clear→selected；`UiNavState+0x10=0` 写面登记（布局 U） | toolbar_button.rs |

**编译**：`cargo check -p toolbar` **Finished（0 错误 0 警告 toolbar 侧）**；
`cargo test -p toolbar --lib` **10/10 通过**（含 `ui_interaction_state_layout_and_accessors`
断言 3B 布局 state@0/keep@1/selected@2）。此前的 rhapsody 假错误系并行 agent 在途编辑，
已随其落定恢复。
