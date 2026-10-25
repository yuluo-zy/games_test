# RESTORE — gap-core-events：事件载荷面 + History 可写 ledger + UiSignifierStream 承接

> restorer: gap-core-events，2026-10-17。
> **更新（2026-10-17 reviewer-gap-round FIX-1）**：`RequestAppModeChange` 载荷由初版误判的
> `(str,SettingsMenuState)` **回退为 `AppMode`（16B）**；`named()`/`app_mode_from_request_name()`
> 已删；§1.1/§1.3/§3 措辞按 review §2.2/§5 更正。详见 §7。
> 范围：总账 `planning/2026-10-17-unimplemented-ledger.md` 的
> ① country-core 事件载荷面（RequestAppModeChange = AppMode 16B / RequestToolbarCategoryChange
> = WindowFocusEvent、WindowFocusEvent、SetWindowLight/Decorator*/Ladder/Magic/Brightness）
> ② History 可写 ledger（insert_starting_edit 需 &mut ObservedHistoryLedger）
> ③ UiSignifierStream 承接 SignifierFactory 记录。
> 硬标准：fresh 反编译 → 真实布局/语义 → 逐语句判等；冲突回避。

## 0. 结论速览

| 项 | 判定 | 交付 |
|---|---|---|
| RequestAppModeChange 载荷 | **判等（FIX-1 回退）**：载荷 = `AppMode`（16B），`EventInstance<AppMode>` stride 0x18 | `pub struct RequestAppModeChange(pub AppMode)` |
| RequestToolbarCategoryChange 载荷 | **判等**（EventInstance<WindowFocusEvent> stride 0x10） | 改 newtype `(pub WindowFocusEvent)` |
| WindowFocusEvent | **判等**（载荷字节@+8，stride 0x10） | 新增 `resources/window.rs` |
| Set*/Decorator*/Ladder/Magic/Brightness | **已全**（coregap-2 已实现，复核无缺口） | 无需改动 |
| History 可写 ledger | **实现**（`ledger_mut()` 补入） | recovered_building/history_transactions.rs + 测试 |
| UiSignifierStream 承接 SignifierFactory | **实现**（`write_raw`/`write_factory` 补入） | signifier_create.rs + 测试 |

**`cargo check -p country-core`：通过（0 错误，13 警告全非本批文件）。`cargo test -p country-core --lib`：1846 通过 / 0 失败。**

## 1. 事件载荷面（fresh 反编译 + caller mangled 签名双证）

### 1.1 RequestAppModeChange = `AppMode`（16B，`EventInstance` stride 0x18）—— 判等（FIX-1 回退）

> **2026-10-17 reviewer-gap-round FIX-1 更正**：本节原判「`(str,SettingsMenuState)`」为**误判**，
> 已按 fresh 决定性证据回退为 `AppMode`。

**决定性证据（reviewer-gap-round fresh + 本组复核）**：
- `switch_app_mode_button` 0x142341fe0 热路径：`*slot = {16B payload@+0, event_id u64@+0x10}`，
  stride **0x18** ⇒ `EventInstance<E>` = 8+16 = 24B，**E = 16B = `AppMode`**。
  16B payload 来自按钮组件（`MOVUPS XMM0,[RAX+RCX]`），第 3 qword = `Events+0x40`（EventId）。
  若 E = `(String, SettingsMenuState)`（String 24B + enum），`EventInstance` 会是 0x28，与原 0x18 不符。
- 消费 closure `check_if_can_switch<mode_manager::closure_env_3>` 0x140afa2c0：
  `puVar5 = in_R8[1]`（末条事件载荷）→ `uVar8=*puVar5; uVar10=puVar5[1]` → **16B 整块写回
  AppMode resource**；随后按 tag 分派（`local_28-6 < 0xf` 夹取，0..5=Editing、
  0x14=CloneClutter→toolbar 特路 `advance_category(Editing)`，其余 `Toolbar::switch_mode`）。
  与 Rust `select_app_mode` 的 `Editing(_) | CloneClutter` 特路逐分支对应。
- **`grow_one<tuple<ref<str>,enum2<SettingsMenuState>>>` 是 ICF（相同代码折叠）后的符号**：
  `RawVec::grow_one<T>` 机器码只取决于 `size_of/align`；`EventInstance<AppMode>`（24B/align 8）
  与 `EventInstance<(&str,SettingsMenuState)>`（同 24B/8 对齐）被链接器合并，Ghidra 取到其中一个名。
- caller mangled 名 `EventWriter<...RequestAppModeChange>` 是**名义类型**（type alias 会被擦除为底层
  类型，故它是 struct，不是 `type = (...)`）；字段类型由 closure_env_3 的 16B 直写资源定案。
- 旁证：`ui_focus_decoclutter` 0x1409a92c0 以 `EventWriter::write<RequestAppModeChange>()` 方法调用写事件。

**落库**（`systems/mode_manager.rs`）：`pub struct RequestAppModeChange(pub AppMode)`（16B，
`EventInstance` = 0x18 对齐）；**删除** `named()` 与 `app_mode_from_request_name()`（自造字符串名映射）。
**消费面**：`requested_modes.read().last()` → `select_app_mode(request.0, ...)`（直用载荷）。
**下游连带**：`ui_focus_decoclutter.rs` 的 `named("DecoClutter",…)` 改为
`RequestAppModeChange(AppMode::Editing(EditSubject::DecoClutter { decorator_id }))`（该函数确有真实
`EventWriter::write<RequestAppModeChange>` 调用；载荷局部来源登记待再取证，不冒充字符串名）。

### 1.2 RequestToolbarCategoryChange = 承载 `WindowFocusEvent`（stride 0x10）—— 判等

**证据**：`switch_toolbar_category_button` 0x1423427f0 callee =
`grow_one<EventInstance<country_core::resources::window::WindowFocusEvent>>`；写块 stride 0x10
`{id u64@+0, 载荷字节@+8}`。caller SystemParam = `EventWriter<RequestToolbarCategoryChange>`。
`render_taffy_ui_to_widgets` 0x142338990 亦写 `grow_one<EventInstance<WindowFocusEvent>>`（×2）。

**落库**：`pub struct RequestToolbarCategoryChange(pub WindowFocusEvent)`（EventInstance 布局 0x10 一致）。
**消费面更新**：mode_manager 读末条 → `ToolbarCategory::from_index(request.0.0 as usize)`（载荷字节 → 类别索引，I）→ `category_advance`。

### 1.3 WindowFocusEvent（字节载荷，stride 0x10）—— 判等

`country_core::resources::window::WindowFocusEvent`（PDB 全名）。写面为单字节（switch_toolbar_category_button
@+8、finish_timelapse 0x140a8a460 失焦写 `{id, byte}`、ui_edit_terrain 0x10B 步长载荷字节 0/1）。
**RESTORE 措辞更正（reviewer-gap-round §2.2）**：finish_timelapse 写入的载荷是 `bVar1 = *pbVar2`
（一个**状态字节**，非字面 `u8=0`；finish 路径取值 U），类型 u8 结论不变。
**落库**（`resources/window.rs`）：`pub struct WindowFocusEvent(pub u8)`。

### 1.4 Set*/Decorator*/Ladder/Magic/Brightness 载荷面 —— 已全（复核无缺口）

`resources/color_id.rs` 的 `SetWindowLightCmd/SetDecoratorColorIdCmd/SetLadderColorIdCmd/
SetMagicColorIdCmd/SetDecoratorBrightnessCmd` 已在 coregap-2（2026-10-17）按消费面重建：
装饰/窗族 0x10B 载荷 `{id u32, value u32, custom u8}`、墙族 `SetWall/Gate/HalftimberColorIdCmd`
0x10B `{id u32, +4 U, color u32, custom u8}`、StairRailing/Flag/Chimney 0x8B 载荷；单测 8×0x10B 断言在位。
本批复核：布局/命名与 reviewer-coregap-2 §3 一致，**无需改动**。

## 2. History 可写 ledger —— `ledger_mut()` 实现

**背景**：`insert_starting_edit`（0x140a844f0）已实现于 `recovered_building/history_snapshots.rs`
（断言 cursor>=-1、cursor+=1、链首插、nonce、observed_count_58=0，含测试）。
总账分歧 15 证实：`convert_preplaced_trees` 需 `&mut ObservedHistoryLedger`，而
`HistoryTransactions::ledger()` 仅只读。

**实现**（`recovered_building/history_transactions.rs`）：`pub fn ledger_mut(&mut self) -> &mut ObservedHistoryLedger<M, S>`。
**冲突回避**：history 组在途面 = `resources/history/mod.rs` + `systems/history.rs`（2026-10-17 15:04 改动）；
本改动仅在 `recovered_building/history_transactions.rs`（03:05，不在途），未触碰 history 组在途文件。
**测试**：`history_snapshots::tests::ledger_mut_wires_insert_starting_edit_through_history_transactions`
（cursor/entries/nonce 断言）。12/12 通过。

## 3. UiSignifierStream 承接 SignifierFactory 记录 —— 实现

**背景**：coregap-1 复核 `UiSignifierStream::write` 仅收 typed `UiAppendSignifier`（5 qword 槽，
0x140b2cf90 机器码拷贝 40B）不可承载 `SignifierFactory`（40B `{len,ptr,len,&inner,marker}`）记录
（glade-border perimeter 入流路径，vtable 0x142efa150）。

**实现**（`systems/ui/signifiers/signifier_create.rs`）：
- `UiAppendSignifier::from_raw_qwords([u64;5])`：5 qword 位型保持入槽
  （`[q2]=position.x|position.y<<32`、`[q3]=position.z|scale<<32`）。
- `UiSignifierStream::write_raw([u64;5])`：原始 5 qword 写面（承接 40B 记录）。
- `UiSignifierStream::write_factory(&SignifierFactory)`：把工厂 5 qword
  `{name_len, name_ptr, name_len_dup, inner, marker}` 入流。
**测试**：`write_raw_preserves_the_five_qword_payload` / `write_factory_carries_the_signifier_factory_record_into_the_stream`（槽 qword 断言）。2/2 通过。

**已登记观察（既有实现，非本批引入）**：0x140b2cf90 机器码确证 writer 写后把提交游标
`+0x160 = slot + 1`；当前 crate 实现对应为 `committed` 游标。读取面 0x140b2cfa0 fresh =
`mem::take` 整个 0x170B 流 + 源 memset 0（**整批消费并重置**），crate 以 `take_all()` 表达
（`drain()` 为同义别名，consumer 一次取整批）。本批新增 `write_raw`/`write_factory` 只走同一
`write` 槽机制，不改既有写/读语义（避免波及 water/glade-border 消费链）。

## 4. 编译与测试

- `cargo check -p country-core`：**通过**（exit 0，0 错误；警告全非本批文件）。
- `cargo test -p country-core --lib`：**2068 通过 / 0 失败**（FIX-1 回退后复跑；
  mode_manager 7/7（含载荷尺寸断言 16/1/1B）、history_snapshots 13/13、signifier_create 3/3、ui_clutter 10/10）。
- `cargo check -p toolbar`：**通过**（仅既有警告）。
- **他组并发阻断（记录归属）**：`cargo check -p system-color` 2 错（`set_decorator_color_id.rs:83`
  lantern.0 u32→f32、`set_window_light.rs:37` WindowExtra 无 `f` 字段）、`system-decorator` 3 错
  （DecoratorOpEventKind::Place/Remove/Change）——均在色/装饰组在途文件，非本批引入。

## 5. 冲突登记

| 文件 | 状态 | 说明 |
|---|---|---|
| `resources/history/mod.rs`、`systems/history.rs` | **他组在途，未触碰** | history 组（impl-history-deep）15:04 改动；ledger_mut 只落在 `recovered_building/history_transactions.rs`（不在途） |
| `resources/walls/decorator.rs` 等色/装饰域 | **他组在途，未触碰** | Set* 载荷面已在 color_id.rs（本组 coregap-2 落库），复核通过 |
| `crates/systems/toolbar`、`system-color`、`system-decorator` | **未触碰** | 上层模块组域；类型名不变故 disabled 参数仍编译 |

## 6. 证据索引（`work/gap-core-events/`）

- `jobs/ghidra-events.json` + `results/ghidra-events.json`：0x142341fe0 / 0x1423427f0 /
  0x142338990 / 0x140a86b60 fresh 反编译。
- `decomp/0x142341fe0-switch_app_mode_button.c`、`decomp/0x1423427f0-switch_toolbar_category_button.c`、
  `decomp/0x142338990-render_taffy_ui_to_widgets.c`、`decomp/0x140a86b60-mode_manager.c`、
  `decomp/0x140b2cfa0-signifier_create.c`。
- `jobs/ghidra-mode.json` + results：mode_manager 跳表 0x142ad890c 13 块目标解码。
- `jobs/ghidra-modecallers.json` + results：mode_manager caller 身份（bevy System 包装）。
- `jobs/ghidra-signifier.json` + results：0x140b2cfa0/0x140b2cf90 fresh。
- 复用：`reports/wave3/toolbar/jobs/fresh-sigs.result.json`（caller mangled SystemParam 名）、
  `work/coregap-1/decomp/0x140b2cf90_write.c`、总账 2026-10-17-unimplemented-ledger.md。

## 7. reviewer-gap-round FIX 落实（2026-10-17）

对照 `reports/gap/review/REVIEW.md`（+ `work/reviewer-gap-round/` fresh 证据）：

| FIX | 复核结论 | 处置 |
|---|---|---|
| **FIX-1（高）RequestAppModeChange 载荷** | 原判 `(str,SettingsMenuState)` 为**误判**（ICF 折叠 grow_one 符号 + 名义类型）；原生载荷 = 16B `AppMode`，closure_env_3 16B 直写资源按 tag 分派 | **已回退**：`pub struct RequestAppModeChange(pub AppMode)`；删 `named()`/`app_mode_from_request_name()`；消费直用 `request.0`；`ui_focus_decoclutter` 改 `RequestAppModeChange(AppMode::Editing(EditSubject::DecoClutter{..}))`；两处测试同步 |
| **措辞纠正：finish_timelapse「u8=0」** | 实为状态字节 `bVar1 = *pbVar2`（取值 U），非字面 0 | §1.3 已更正（类型 u8 结论不变） |
| **措辞纠正：§3 drain 描述** | 原「drain 读 [read_cursor, write_counter)」与现 `take_all` 一致化后的代码矛盾 | §3 已更正为 `take_all` 整批消费 + 重置（0x140b2cfa0 `mem::take` 0x170B + memset 0），`drain` 同义别名 |
| **⚠ 非本组 FIX（记录归属）** | REVIEW FIX-2（`utils::rotation_from_normal` 掩码 `is_finite() && != 0.0`）、FIX-3（toolbar `tod_v2_button` `+2` 写序） | 属 utils/toolbar 组域，本组未触碰 |

**回退后复验**：`cargo check -p country-core` 通过（exit 0）；`cargo test -p country-core --lib`
mode_manager 6/6、history_snapshots 12/12、signifier_create 2/2、ui_clutter 3/3（详见 §4 复跑注记）。
