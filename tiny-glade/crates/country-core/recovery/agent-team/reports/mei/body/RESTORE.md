# RESTORE — mei body（mei_pre_update / mei_post_update / render_taffy_ui_to_widgets / taffy 残余）

> restorer-mei-body，2026-10-17（第二段：上次因 402 中断后续跑，上下文保留）。
> 身份：B patched EXE。硬标准：逐函数 fresh 反编译 → 逐语句判等；还原不了标
> **未实现** 并写明分歧点，不落近似体冒充判等。
> 证据目录：`work/restorer-mei-body/{jobs,results,evidence}/`（本批 fresh）。
> 红线：只改 `crates/systems/toolbar/src/`；未改 shared / 他模块。

## 0. 结论速览

| # | 条目 | VA | verdict | 关键交付 / 分歧 |
|---|---|---|---|---|
| 1 | `mei_pre_update` body | 0x1406842e0 | **重建**（12 条语句级判等 + 3 处 U） | `mei/systems.rs`；状态机/双缓冲/声音上升沿落地 |
| 2 | `mei_post_update` body | 0x1406884c0 | **重建**（8 条判等 + 4 处 U） | 同上；render 两遍/z 序拓扑/清位/绘制 |
| 3 | `render_taffy_ui_to_widgets` | 0x142338990 | **重建**（13 参判等 + 主循环）+ 3 处 U | `ui_interaction.rs`；WidgetParams→add→事件链 |
| 4 | `taffy_mut_style` closure_env 1/3..9 | — | **未实现（无单态）** | fresh 复核 procs 全表：B 无这些实例化 |
| 5 | `toolbar_alignment` case 4/6 二跳 | 0x14234485d | **未实现（分歧）** | 二级表 0x14304e6e8 与主表物理重叠；索引寄存器语义未闭合 |

**函数级计数**：判等 **0**（无完整函数可称逐语句全等）、重建 **3**、
未实现 **2 组**。
**语句级计数**（本批逐条对照）：判等 **20**、重建 **5**（含 I 级字段投影）、
未实现 **7 处**（逐条见 §4 总账）。

**编译/测试**：`cargo check -p toolbar` **0 error**（toolbar 侧 0 warning；
country-core 的 14 条 warning 为其既有项）；`cargo test -p toolbar --lib`
**10 passed / 0 failed**（本批新增 4 个测试）。

## 1. 证据与身份

- fresh 反编译（本批，`results/fresh-core.json`）：三函数
  `signature + decompiled_code + callees` 各一份，已落
  `evidence/0x1406842e0-mei_pre_update.c`（37 574 B）、
  `0x1406884c0-mei_post_update.c`（94 226 B）、
  `0x142338990-render_taffy_ui_to_widgets.c`（24 588 B）。
- 9 参签名（`procs.tsv` line 35018，`run_unsafe<FunctionSystem<...>>` mangled）：
  - pre：`ResMut<MeiContext>, EventReader<MouseWheel>, ResMut<UiNavState>,
    Res<WindowSize>, Res<Time>, Res<ActionSetStates>, Option<ResMut<UiState>>,
    Option<Res<CursorPositionSS>>, EventWriter<PlayMenuUiSound>`。
  - post：`ResMut<MeiContext>, ResMut<TextRenderer>, ResMut<UiNavState>,
    ResMut<AssetTextureLibrary>, Res<GraphicsDeviceResource>, Res<WindowSize>,
    Option<Res<SteamClientResource>>, Option<Res<SteamInput>>,
    Res<ActiveInputModeTracker>`。
- 13 参（render_taffy）：沿用 `unlock/mei-toolbar` 判等结果，本批 fresh 复核
  `QueryIterationCursor::init` 的 tuple 泛型逐型一致。
- taffy 跳表：`read_memory 0x14304e6d8 len=80`（主表）与
  `0x14304e6e8 len=96`（二级表）；`disassemble_function 0x142344560`
  （Ghidra 函数边界止于 `0x142344646 jmp rax`，case 体在函数体外，
  用 radare2 `px/pd` 线性核对）。

## 2. `mei_pre_update`（0x1406842e0）

### 2.1 已判等语句（逐条对 fresh 行号）

| 语句 | fresh 位置 | 实现 |
|---|---|---|
| `mei+0x270 = Time+0x58`（= `elapsed_secs_wrapped`） | L161 | `mei.elapsed_wrapped = time.elapsed_secs_wrapped()` |
| 输入 handler 表清空（hashbrown `clear` + 容量重写） | L163-175 | `MeiContext::frame_begin()` |
| 节点 children 快照集合 | L176-249 | `snapshot_child_ids()`（消费语义 U） |
| `scale = window_height / 1000.0` | L259/L402 | 同式 |
| 光标读取 + `cursor/scale` 写 `mei+0x268` | L254-258/L403-411 | `mei.cursor_units = cursor_ss / scale` |
| `EventReader<MouseWheel>` 累计 `(x,y)` | L259-314 | `scroll += Vec2::new(ev.x, ev.y)` |
| `deconstruct` → 逐 12B 记录 `process_input_action_handlers` → 未处理压实 | L400-445 | `records.retain(|r| !process(...))` |
| `reconstruct` → `memcpy(mei+0x30, set, 0x170)` | L472-475 | `mei.actions = CoreMainActionSet::reconstruct(&records)` |
| 每节点 `content.pre_update(&actions, {cursor, state_bits})`（vtable +0x58） | L476-539 | trait 扩参落地 |
| `interactive = !ignore_input && (force_interactive || content.interactive())` | L605-619 | 同式 |
| 矩形 = `pos*scale .. (pos+size)*scale`；4 界比较 | L620-639 | 同式 |
| hover 位写 `state & 1` | L640-649 | 同式 |
| `UiState` 缺失 → panic（unwrap/expect） | L656-675 | `.expect("...")` |
| `WidgetParams::{new,no_history_recording,occlude_by_world,allow_negative_distance,sounds(empty)}` → `UiState::add` | L687-706 | 链式落地 |
| `clicked()` → 状态 bit4 | L718-724 | `ctx.clicked()` |
| 焦点节点输入事件 → vtable +0x48；tag 0xc6 起丢弃 | L725-814 | `process_input_action` + break |
| `inside && scroll != 0` → `0xc3` 滚轮事件 | L829-853 | `MeiInputEvent::scroll` |
| 悬停上升沿 → `PlayMenuUiSound{hover}`（id != 0x0d） | L855-871 | 同式 |
| `(old & 0x30)==0 && (new & 0x30)!=0` → 点击声音 | L872-887 | 同式 |
| `z_order.len=0` + 6 qword 表交换 + `RawTable::clear` | L892-916 | `clear` + `swap` + `previous_nodes.clear()` |

### 2.2 未实现（分歧点，逐条）

1. **`Vec::retain<ActionSetAction, closure_env$2>`（L446-451）**：闭包读
   `UiNavState+0x30`（hold 字节）与 `+0x10`（焦点判别）。country-core 的
   `UiNavState` 仍为 `pub struct UiNavState(pub u64)` typed-gap（原生 0x38 B），
   读字段即越界 → **未实现（core-type-gap）**；本实现不叠加过滤。
2. **hover 的焦点项 `inside || (focus==Mei && focused==id && hold)`（L640-642）**：
   同上，`UiNavState+0x10/+0x18/+0x30` 不可读 → 只落 `inside` 项，
   **未实现（core-type-gap）**。
3. **drag 状态位 bit1/2/3（L707-717）**：`slider3d::DragState::{is_some,
   is_started,is_finished}` 与 `impl$1::into_drag_state` 未落库
   （`country-core/systems/ui/resources/slider3d.rs` 仅有 `project_cursor_onto_line`）
   → **未实现（core-api-missing）**，不对状态字节造假。
4. `WidgetParams::new` 的 `extra_78` 载荷（由 `{fmt 串, node+0xe}` 拼出）与
   widget 名字格式串：U，本实现以 `format!("{}", id.0)` + `[0;2]` 投影（I）。

## 3. `mei_post_update`（0x1406884c0）

### 3.1 已判等/重建语句

| 阶段 | fresh 位置 | verdict | 实现 |
|---|---|---|---|
| `WindowSize::as_vec2` + `scale=height/1000` | L300/L321 | 判等 | 同式 |
| 每节点 `content.render`（vtable +0x60），返回 !=2 写回 | L380-407 | 判等（调用点）/重建（返回布局 U） | `render` + `z_flags` 暂存 |
| z 序拓扑：StackEntry LIFO、父累加 `z=child_z+1.0`、flags `&2\|b`、子节点入栈 | L409-750 | **重建** | `StackEntry{variant,id,z,flags}` + `Vec`（bumpalo 语义） |
| 全体清焦点位 `state &= 0xbf` | L954-958 | 判等 | 同式 |
| UI 方向键公式 `dir={right-left, up-down}` | L1056-1095 | 判等（公式） | 只落公式闭包 |
| `UiFocus::as_mei` 命中 → `state |= 0x40` | L1424-1485 | 未实现 | core-type-gap |
| `Vec<&mut Node>` + `sort_unstable_by(closure_env$0)` | L1544-1568 | 重建 | 按 `Node.z` 升序（比较器 U，I） |
| 逐节点 `content.draw`（vtable +0x68，`node+0x80 != 0`） | L1571-1591 | 判等（调用点） | `draw(&ctx,pos,size)` |
| `pack_srgba_u32(1,1,1,0.15)` | L1597-1600 | 重建（utils 未落库） | 本地 sRGB 打包镜像 + 测试 |
| 清空输入事件 Vec | L1710-1739 | 判等 | `input_events.clear()` |
| 全体 `state &= 0xcf`（清 bit4/bit5） | L1765-1770 | 判等 | 同式 |

### 3.2 未实现（分歧点）

1. **`update_nav` 两遍 `iter_subtree`（closure_env_0 / closure_env_2）**
   （L1181-1519 / L1946-2093）：闭包体在 Ghidra 中为内联碎片；
   方向候选/边界比较语义未取证 → **未实现**。
2. **4 个 `DigitalActionState::just_pressed` 的载荷源**：9 参签名不含
   `Res<ActionSetStates>`；反编译从 `UiNavState` 相邻槽（+0x20..+0x30）读取
   → core-type-gap，**未实现**（仅登记 `dir` 公式）。
3. **`sort_unstable_by` 比较器 `impl_17::post_update::closure_env$0`**：
   比较键未取证；本实现按拓扑累计 `Node.z`（I 级重建选择）。
4. **taffy 最小尺寸 `compute_min_size_with_children<Map<Copied<Iter<GlobalId>>,
   perform_layout::closure_env$0>>`（L860-862）与 `impl$17::perform_layout`**：
   映射闭包体未取证 → **未实现**。

## 4. `render_taffy_ui_to_widgets`（0x142338990）

### 4.1 落地（13 参判等 + 主循环重建）

- Query 迭代 tuple 与 13 参签名逐型判等（fresh `QueryIterationCursor::init`
  泛型串与本文件签名一致）。
- 名字：`format!("toolbar_button_{:?}", taffy.node)`——前缀串在
  `0x14304dd04`（`ui_interaction.rstoolbar_button_`，`ps` 证据），槽格式为
  `slotmap::impl$23::fmt`（Debug）。
- 矩形扩张：`expand = game_config[+0xe8] * 10.0 * size_mult`；
  `rect = center ± (extent + expand)/2`（判等公式）。
- `WidgetParams::new(volume{tag=3,layer,size})` + `no_history_recording` +
  `occlude_by_world` + `allow_negative_distance` + `sounds(empty)` →
  `UiState::add::<String>`。
- `HistoryButton` 分支：拖拽态非 2 → `UiHistoryButtonPressed{kind}`
  （1B 载荷）；拖拽态 ==3 → `Commands::entity().insert(
  UiPlaySelectAnimation{0.2,0.2,0})`；`clicked()` → `UiClickEvent{entity}`。
- 无 `HistoryButton`：`AppMode != Photomode(0xd) && is_active` 的门控落地；
  载荷 `{1u64,1u64,cursor Vec2,f32=scale*60}` 构造落地。

### 4.2 未实现（分歧点）

1. **`Ui2dExtent` core-type-gap**：core 为 `{_gap: u64}`（8B），原生 0x10
   `{pos Vec2, size Vec2}` → rect/名字扩张输入不可读；本实现以零 extent 占位。
2. **`slider3d::DragState` core-api-missing**：以 `UiWidgetContext::dragged()`
   投影（I）替代 `into_drag_state()` 的 is_some/==3/!=2 判别。
3. **`UiNavState` 焦点字段**（+0x10 tag/+0x18 id/+0x30 hold）：
   前段 `UiFocus::is_toolbar()` 重置、keep/selected 字节写面 → 未实现。
4. **`SignifierFactory::new` 的 inner/marker**（闭包胖指针）未在反编译展开；
   载荷已判等但不伪造闭包 → 未实现（保留载荷构造）。
5. **`(str, SettingsMenuState)` 事件**（stride 0x18）：str 来源 U，且签名
   第 5 参 `EventWriter<UiDragEvent>` 载荷面不符 → 未实现。

## 5. taffy closure_env 1/3..9 与 case 4/6（fresh 复核）

### 5.1 closure_env 1/3..9：**未实现（无 B 单态）**

fresh 复核 `evidence/toolbar-origin-2026-10-16/procs.tsv` 全表：
`taffy_mut_style` 单态恰 **10 个**：

| 单态 | VA |
|---|---|
| toolbar_alignment::closure_env$0 | 0x142375c10 |
| toolbar_alignment::closure_env$2 | 0x142375eb0 |
| toolbar_alignment::closure_env$10 | 0x142375b40 |
| toolbar_alignment::closure_env$11 | 0x142375ce0 |
| toolbar_alignment::closure_env$12 | 0x142376050 |
| toolbar_alignment::closure_env$13 | 0x142376300 |
| undo_redo_alignment::closure_env$0 | 0x142376230 |
| undo_redo_alignment::closure_env$1 | 0x142375f80 |
| button_size_and_padding_update::closure_env$1 | 0x142375db0 |
| perform_layout::closure_env$1 | 0x142376120 |

`toolbar_alignment::closure_env_1/3/4/5/6/7/8/9` 均无独立符号（未实例化/已内联），
无法取得 taffy Style 写面 → **不伪造 setter**，登记未实现。

### 5.2 case 4/6 二跳：**未实现（索引寄存器/基址歧义）**

- 主跳表 0x14304e6d8（20 项）条目 4/6 均 → `0x14234485d`。
- `0x14234485d` 是二级派发：
  `lea rcx,[0x14304e6e8]; movsxd rax,[rcx+rax*4]; add rax,rcx; jmp rax`。
- **物理重叠**：`0x14304e6e8 = 主表 + 4 项`（LLVM 跳表后缀共享）。
  - 按主表偏移（base 0x6e8 + 同值）解释：4→`0x1423448d6`（三级表
    0x14304e6f8 派发）、6→`0x142344915`（`add rsp,0x20` 的中段字节，非合法
    指令起点）。
  - 按后缀表（index = alignment-4）解释：4/6→`0x14234486d`
    （= `taffy_mut_style<closure_env_13>` 尾段）。
  - 两种解释互斥：`jmp rax` 后 RAX 为目标地址而非索引，与
    `[rcx+rax*4]` 用法冲突；且 5/7/9/10/0xb 的部分条目只在某一解释下
    才是合法指令起点。
- 因此 **case 4/6 未实现**；不选任一解释落 setter 序列。
- 附带已核对：case 0/1/2/3 的四条 setter 序列、case 5/7 的
  `closure_env_13` 尾段、case 9/10/0xb 的空 setter 路径保持原判等；
  per-item 循环 0x1423447af 起（stride 0x20、`+0x10 != 0`）已解出
  alignment∈{1,2}→`closure_env_2,closure_env_13`、alignment∈{0,3}→
  `closure_env_10,closure_env_13`（0x1423447c0/0x1423447d0 段逐指令），
  其余 alignment 走二级/三级派发，语义未闭合。

## 6. 代码落点

| 文件 | 变更 |
|---|---|
| `mei/core.rs` | `MeiInputEvent::{scroll,tag,payload_vec2}`；`NodeContent::{process_input_action,on_input(ActionSetAction),pre_update(actions,cursor,state_bits),is_empty}`；`Node.{z,z_flags}`；`MeiContext.{actions,cursor_units,elapsed_wrapped}` + `frame_begin/snapshot_child_ids`；`process_input_action_handlers` 改为按 12B 记录并返回 bool；4 个新测试 |
| `mei/systems.rs` | `mei_pre_update` / `mei_post_update` 主体（§2/§3） |
| `ui_interaction.rs` | `render_taffy_ui_to_widgets` 主体（§4） |
| `toolbar_systems.rs` | closure_env 1/3..9 与 case 4/6 的 fresh 证据/分歧登记（逻辑未改） |
| `lib.rs` | 模块状态说明更新 |

## 7. 未实现总账（7 条）

1. `mei_pre_update`：`closure_env$2` 动作 retain（UiNavState +0x30）— core-type-gap。
2. `mei_pre_update`：hover 的焦点保持项（UiNavState +0x10/+0x18/+0x30）— core-type-gap。
3. `mei_pre_update`：drag 状态位 — slider3d DragState 未落库。
4. `mei_post_update`：`update_nav` 两遍 iter_subtree 闭包体 + 4 方向输入源 — 未取证/core-type-gap。
5. `mei_post_update`：z 比较器 `closure_env$0` + taffy `perform_layout` /
   `compute_min_size_with_children` 映射闭包 — 未取证。
6. `render_taffy`：`Ui2dExtent` 字段面 / `UiNavState` 焦点写面 / Signifier
   inner-marker / `(str,SettingsMenuState)` 事件面 — core-gap + 载荷 U。
7. `closure_env 1/3..9`（无单态）与 `toolbar_alignment` case 4/6（二跳索引
   语义）— 未实现。

## 8. 编译 / 测试

```
cargo check -p toolbar        # 0 error（toolbar 侧 0 warning）
cargo test  -p toolbar --lib  # 10 passed; 0 failed
```

说明：本批进行中曾遇到 country-core 的并行在途编辑
（`resources/window.rs` `MonitorMatch: Default` 缺失）导致依赖侧瞬时失败；
该编辑由并行 agent 修复后复跑通过，与本批 toolbar 改动无关。
