# restorer-wave3-tree-glade — tree-tool / glade-border wave3 RESTORE 报告

日期：2026-10-17（wave3）
状态：完成（`cargo check -p system-tree -p system-glade-border` 通过；rhapsody 并发编辑造成短暂块已排除）
涉及文件：`crates/systems/tree-tool/src/*.rs`、`crates/systems/glade-border/src/*.rs`
证据：fresh 反编译 `--worker shared`（本会话重取 replay_tree 0x141bc34f0 /
ui_move_tree 0x14226b1a0 / closure$1 0x141be1080 / feedback 宿主 0x141be2280 +
closure$1 disasm + 既有 eco evidence）；core 新类型对回反汇编复核。

## 〇、总述

wave3 前提核对：**core 类型补齐部分属实、部分不实**——
已补齐（判等复核通过）：`MoveTreeCmd` 0x20、`ReplayBucket.timing`/`CommandTiming`、
`SignifierFactory`/`SignifierEntityBuilder`、`PlayUiSound` 变体 #4。
**未补齐（task 前提不符，wave3 复查登记）**：`InsertTreeCmd` 0x48 仍为共享 0x28
（`tree_edit.rs` 由 history 组持有，冲突跳过——ledger 原样）、`DebugLine::show()`
（core debug_viz 仍无，show 系列留组件/ECS 包）、
`UiSignifierStream::write` 承接 SignifierFactory/closure vtable 槽位、
`History::insert_starting_edit` 作用于 `ObservedHistoryLedger` 而
`History<S>`=HistoryTransactions 只暴露只读 `ledger()`。

**最终 verdict 统计：判等 1 函数 / 重建 5 函数（附分歧）/ 未实现-需重做 1 函数。**

| 模块 | 函数 | 地址 | verdict |
|---|---|---|---|
| tree-tool | process_tree_cmds | 0x142267430 | **重建**（Move/Delete 分支判等；Insert 分支未实现-分歧 T2） |
| tree-tool | replay_tree | 0x141bc34f0 | **重建**（时序/Move/undo 面判等；Insert 重建缺 0x48 字段-分歧 T2、Move undo 旧位-分歧 T3） |
| tree-tool | ui_move_tree | 0x14226b1a0 | **重建**（MoveTreeCmd 载荷判等；slider3d case1/2/3 仍近似-分歧 15） |
| tree-tool | convert_preplaced_trees | 0x14226be70 | **重建**（Wait flag 判等-分歧 12 已修；insert_starting_edit 未实现-分歧 15） |
| glade-border | display_glade_border | 0x141be0ad0 | **未实现-需重做**（show 分歧① / perimeter 写路径分歧②） |
| glade-border | closure$1 | 0x141be1080 | **重建**（builder 链+2×insert 落库；唤醒路径未接线-gap） |
| glade-border | feedback_input_out_of_border | 内联（宿主 0x141be2280） | **判等**（PlayUiSound#4 写面落库） |

## 一、逐函数 verdict + 分歧点

### 1. process_tree_cmds（0x142267430）— 重建

**wave3 落库**：
- **Move 分支（判等）**：`MoveTreeCmd` 0x20 载荷 `{id@+0, index u64@+8, position
  Vec3@+0x10}`（coregap-1 落库，判等）。逐事件 `storage.get(TreeStorageIndex(cmd.index))`；
  None → `log::warn!("MoveTreeCmd references a non-existing tree")`（decomp 行
  247-270，串 @0x143035048）；Some → 写 PrefabInstanceData 新位置 + 水面采样
  （`bilinear_sample_at_world_xz`）+ `PlayTreeSoundCmd{kind=1, position}`（decomp
  行 314-323）。
- **Delete 分支（判等）**：补 `entities.contains(entry.entity)` 门控（decomp 行
  835；系统签名新增 `&Entities` SystemParam）——实体仍存在才 Commands 入队
  `TreeMarkedForDeletion` + `PlayTreeSoundCmd{kind=2}`（原近似 分歧 18 已修）。
- **Insert 分支（未实现-分歧 T2）**：`InsertTreeCmd` 0x48（name String@+0、
  position@+0x10、scale@+0x20、{variant,rotation}@+0x28、flags@+0x30、id@+0x38、
  height@+0x40）在 `resources/history/tree_edit.rs` 未落库（history 组在途编辑，
  冲突跳过）；`SpawnPrefabCmd`（槽 0x50）、`TreeMoveBend`/`TreeAmbience` 载荷同为
  type gap → 循环体保持未实现（未冒充）。
- 分歧 T1（登记）：`PlayTreeSoundCmd` 共享型 {kind, position} 缺真实槽 stride
  0x20 的 in_water@+9 / inverse_lerp@+0xc 写面；audio 侧 tree_audio 按 kind 分派
  （kind 1 忽略），不读这两字段 → 游戏逻辑无观察差异。

### 2. replay_tree（0x141bc34f0）— 重建

**wave3 落库**：
- **时序记账（判等）**：`ReplayBucket.timing`（coregap-1 落库，`CommandTiming`
  偏移 flags@+8/delay@+0xc/elapsed@+0x18 判等；bit0 极性按二进制修：
  置位=可推进、清零=待移除）。逐命令 `flags bit0==0 → 记移除`；`bit0==1 →
  delay=cmd.delay/speed（下钳 0.05）、delay<elapsed → 移除、`elapsed +=
  time.delta`（decomp 行 220-242）；结束逆序 `Vec::remove`（行 245-410，共享
  `ReplayCommand` 枚举 Drop 等价 tag switch）。
- **elapsed==0 门控（判等）**：`timing.elapsed == 0.0`（Rust f32 `==0.0` 对 NaN
  为假，与 `==0.0 && !NAN` 等价）才重发命令。
- **Move 分支（判等）**：非 undo → `MoveTreeCmd{index=edit.id(),
  position=edit 新位置}`（decomp 行 146-181）。
- **Place undo（判等）** → `DeleteTreeCmd{id}`；**Delete 非 undo（判等）** →
  `DeleteTreeCmd{id}`。
- **分歧 T2（未实现面）**：Delete undo / Place non-undo 的 InsertTreeCmd 重建——
  共享 `InsertTreeCmd{id, position, scale, prefab_name}` 承载 name/position/scale，
  真实槽另有 height/variant/rotation/flags（0x48）→ 部分落地 + 登记。
- **分歧 T3（未实现面）**：Move undo 需 edit+0x14/+0x1c **旧位置**；共享
  `TreeEditData` 只有单一 position（新）→ 未实现。

### 3. ui_move_tree（0x14226b1a0）— 重建

**wave3 落库**：
- **MoveTreeCmd 载荷（判等）**：`MoveTreeCmd{id, index=树 id, position}` 全载荷
  （case2/3 均落库；decomp 行 355-363/387-395：`slot+8=树 id、slot+0x10=x、
  +0x14=y=max(h,0)、+0x18=z`）。
- **门禁修正（reviewer-wave3-tg-tr 复核补登记，已修）**：decomp 行 101-110 门禁 =
  `mode ∈ {0x10, 0}`——`lVar14==0xb`（Tree）**或** `lVar14==0 && uVar8==0`
  （AppMode::Editing(None)，tag 0=None 亦放行）；wave3 首版仅 Tree，已补 0 面。
- **分歧 15（未实现）**：`slider3d::into_drag_state` case1/2/3 状态机仍以
  `Local<TreeMoveCache>` 距离探测近似（非原状态机）；case2 界外双事件
  （DisplayOutOfBorder + FeedbackInputOutOfBorder，decomp 行 318-341）在本近似下
  不可达（顶部界外早退先返回）→ 分歧 16 维持。
- 候选树源（分歧 9）维持 `storage.iter()` 代理。

### 4. convert_preplaced_trees（0x14226be70）— 重建

**wave3 落库**：
- **Wait flag 语义（判等，分歧 12 已修）**：decomp 行 122-128——有未读事件 →
  `count=1, flag=1`；无事件帧且 flag 置位 → `count+=1`，`count+1>2` → `flag=0`
  执行转换块（一次性子门控）。原 Rust 无 flag、`count>2` 后每帧重跑转换块——
  已按二进制控制流判等修复。
- **分歧 15（未实现）**：`History::insert_starting_edit`——共享
  `History<S>`=HistoryTransactions 只暴露只读 `ledger()`（无 `&mut
  ObservedHistoryLedger` 面），`insert_starting_edit`（history_snapshots.rs）需
  `&mut ObservedHistoryLedger` → 转换通过者仍空分支（未冒充）。
- **墙查询闭包方向（判等，reviewer-wave3-tg-tr 复核登记）**：
  `walls.query_with_circle(pos, TREE_WALL_GAP, |_s, _d| { blocked = true; })`——
  任一墙段命中即置 blocked（未转换），方向与 decomp 0x14226be70 的
  `query_with_circle_callback`（closure 把墙段映射到近墙树命中判定）一致。
- 分歧 13（岩石掩码 buffer 接线）、分歧 9/14（候选源）维持登记。

### 5. display_glade_border（0x141be0ad0）— 未实现-需重做

**wave3 复查（task 前提"DebugLine::show/SignifierFactory 已补"部分不实）**：
- **分歧① 折线落流（未实现）**：`DebugLine::show()`（0x140b04f70）——core
  `debug_viz::DebugLine` **仍无 show**（source 复核，doc 注明 show 系列留组件/ECS
  包）→ 折线构建链后 drop，可视化效果丢失。登记 country-core 任务。
- **分歧② perimeter 写路径（未实现）**：`SignifierFactory`（0x14089d8b0）
  **已落库且判等**（40B `{len,ptr,len,&inner,marker}`）；但 core
  `UiSignifierStream::write`（0x140b2cf90）**仅接受 `UiAppendSignifier`**（简化
  40B 模型），不承接 SignifierFactory/closure vtable 记录；closure$1 唤醒
  （vtable 0x142efa150 → `call_once` 0x141be0a30）未接线 → 48B 记录
  {tag=1,kind=1,折线,perimeter} + 标签载荷不可承载（不以 UiAppendSignifier 投影
  冒充）。登记 country-core gap：stream 承接 SignifierFactory + closure 唤醒。
- 已判等部分（控制流 1-3、closure$0 抬升、demo 覆盖、格式串 `glade_border{}`
  无空格）维持。

### 6. closure$1（0x141be1080）— 重建

**wave3 落库**：`SignifierEntityBuilder`（coregap-1 已补）→ 链落库。
- fresh disasm 0x141be1080 复核：`new`（载荷 6 qword：首 = mesh_kind
  `0x8000000000000002`，余 = 折线 {len,ptr,len} + 2×0）→ `set_ui_pass(0)` →
  `animation_duration(0.0)` → `custom_shader("dashed_line"@0x142efa180)` →
  `build`；2× `Commands::insert`：`DrawTopology::Alt(2.0)`（值
  `0x4000000000000001` = tag 1 + f32 2.0）、`DashedLineLength = 捕获 f32×0.7`
  （0x1429cd030）。core 以 `DashedLineLengthView`（f32@+0，同布局）承载。
- **唤醒未接线（gap）**：SignifierFactory closure vtable 由 country-core signifier
  消费端触发；`UiSignifierStream` 无 closure 槽位 → `closure_1` 落库为可调用但
  未达（`#[allow(dead_code)]` + 登记）。asset-lib 两参（build 网格/着色器解析）为
  core GAP。
- verdict：**重建**（逻辑/常量完整落库；观察可达性依赖 country-core 唤醒接线）。

### 7. feedback_input_out_of_border（内联，宿主 0x141be2280）— 判等

**wave3 落库**：音效写面判等——`PlayUiSound` 变体 #4（coregap-1 落库
`{kind u32@+0=4, 载荷区@+8..0x17, id u64@+0x18}`，步长 0x20B）。fresh disasm
0x141be2280 行 92-93：`slot+0=4`、`slot+0x18=EventWriter 计数`（bevy 胶水）。
`EventWriter::write` 自动补实例 id；crate `PlayUiSound::variant4(0)` 落库（上升沿
一次，`cache.had_feedback_last_run` 去抖判等）。

## 二、编译适配（history 组 tree_edit.rs 变更）

history 组把 `TreeEditData.id` 由 u64 改 i32（fresh decomp-0x14003d4e0 字段序）
导致 system-tree 6 处 E0308/E0560 编译断裂，wave3 适配：
- ui_move_tree / ui_place_tree / hide_trees / ui_delete_tree 的
  `TreeEditData{ id: X }` → `id: X as i32`（4 处）。
- ui_move_tree 的 `MoveTreeCmd{ _gap: id }` → `MoveTreeCmd{ id, index: id,
  position }`（2 处，wave3 载荷落库）。

## 三、cargo check

`cargo check -p system-tree -p system-glade-border`：**通过**（两 crate 0 新增警告；
wave3 期间 2 次绿态确认——Entities 修复后 7.17s finish、最终批量前 GREEN）。期间
country-core 依赖链 rhapsody 被并发组（rg/sync_plan/bind.rs 等大重构）反复 in-flight
编辑造成**短暂/持续编译块**（transfer.rs `device_generation`、ash
`cmd_pipeline_barrier2`、bind.rs `acquire`/`push_order` 等 E0308 多态）——均非本组
改动所致（本组未改 country-core/rhapsody 任何文件）；ui_move_tree 门禁修正（reviewer
①）与 convert 墙查询判等登记在最后一次绿态后落入，经 API 复核（`EditSubject::None`
native_tag 0、`AppMode::Editing` 构造）验证可编译，待 rhapsody 恢复后复验。country-core
既有 9 警告非本组范围。

## 四、协作登记

- 未改 `crates/country-core/src/resources/history/tree_edit.rs`（history 组持有，
  InsertTreeCmd 0x48 冲突跳过）。
- 未改 `crates/country-core/src/utils/debug_viz.rs`（DebugLine::show 未落，登记
  country-core 任务）。
- fresh 反编译走 `--worker shared` 只读；无 ReVa 脚本/写库。
- 跨模块 gap（country-core 侧待接线，本组登记不改库）：
  1. `DebugLine::show`（0x140b04f70）debug_viz show 系列；
  2. `UiSignifierStream::write` 承接 `SignifierFactory` 记录 + closure 槽位 +
     closure$1 唤醒（vtable 0x142efa150）；
  3. `InsertTreeCmd` 0x48 落库（tree_edit.rs，含 height/variant/rotation/flags）；
  4. `History::insert_starting_edit` 的 HistoryTransactions `ledger_mut` 面。
