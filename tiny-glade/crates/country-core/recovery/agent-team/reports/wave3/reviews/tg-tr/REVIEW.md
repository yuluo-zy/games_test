# REVIEW — wave3 tree-tool / glade-border / timberframe / roof 落库核验（reviewer-wave3-tg-tr）

- reviewer：`reviewer-wave3-tg-tr`
- 日期：2026-10-17
- 依据：`reports/wave3/tree-glade/RESTORE.md` + `reports/wave3/timber-roof/RESTORE.md`
- 方法：逐函数 **fresh 反编译**（ghidra `--worker shared`，3 批 17 函数 + 1 个查询闭包定位，结果 `reports/wave3/reviews/tg-tr/evidence/{tg-batch1,batch2,batch3,find,query}.result.json` + `fresh-*.c`；get_metadata 身份 = tiny-glade.exe B patched cf048e27）→ 独立复述 → 与 Rust（`crates/systems/{tree-tool,glade-border,timberframe,roof}/src/*.rs` + country-core 类型面）逐语句对照 → verdict。
- 未改 src、未改库。分歧登记核验为本次重点。

---

## 〇、裁决总表

| 模块 | 函数（地址） | RESTORE verdict | 本次核验 | 说明 |
|---|---|---|---|---|
| tree-tool | process_tree_cmds（0x142267430） | 重建 | **重建**（Move/Delete 判等；Insert 未实现 T2 诚实） | §1 |
| tree-tool | replay_tree（0x141bc34f0） | 重建 | **重建**（时序/Move/Delete/Place/undo 判等；T2/T3 诚实） | §2 |
| tree-tool | ui_move_tree（0x14226b1a0） | 重建 | **重建**（载荷判等；分歧 15/16 诚实；**补登记 mode 0 门**） | §3 |
| tree-tool | convert_preplaced_trees（0x14226be70） | 重建 | **重建**（Wait flag + 全部门禁判等；分歧 15 诚实） | §4 |
| glade-border | display_glade_border（0x141be0ad0） | 未实现-需重做 | **未实现-需重做**（分歧①/② 均证实） | §5 |
| glade-border | closure$1（0x141be1080） | 重建 | **重建**（链+2×insert 判等；唤醒 gap 诚实） | §6 |
| glade-border | feedback_input_out_of_border（宿主 0x141be2280） | 判等 | **判等** | §7 |
| timberframe | construct_timberframe_rectangle（0x141a41ba0） | 重建+写路径 | **重建+写路径**（相位/计数判等；分歧 I 级诚实） | §8 |
| timberframe | construct_timberframe_circle（0x1421b67a0） | 重建+写路径 | **重建+写路径**（同上；主环 IDENTITY 占位 GAP 诚实） | §9 |
| roof | simulate_roof_tile_for_audio（0x1421a7090） | 判等（修正 angle' 三处） | **判等**（修正属实） | §10 |
| roof | highlight_roof（0x141a284b0/0x141a31280） | 重建 | **重建**（数据流判等；pad 8B 覆盖 I 级诚实） | §11 |

**判等 2 函数 / 重建 8 函数 / 未实现-需重做 1 函数。分歧登记全部核验通过（诚实准确）；补登记 2 项（见 §12）。**

---

## §1 process_tree_cmds（0x142267430）— 重建（Move/Delete 判等，Insert 未实现）

**fresh 反编译**（`fresh-process_tree_cmds.c`）独立复述：
- 事件读者依次为 Move（stride 0x20，行 161/173）、Insert（stride 0x48，行 203/209）、Delete（stride 0x10，行 803/816）。
- **Move 分支**（行 187-325）：`TreeStorage::get(index@+8)`（行 243，键 = 事件+8）；None → log「MoveTreeCmd references a non-existing tree」（行 259，串 @0x143035048，log level 门控）；Some → 查 PrefabInstanceData（行 272-281，`unwrap_failed` panic 路径行 277-280），写新位置，`bilinear_sample_at_world_xz`（行 301-302）→ `PlayTreeSoundCmd` 写面（行 314-323）：
  ```
  slot+0  = 事件计数（bevy 胶水）
  slot+8  = 1（kind）
  slot+9  = 0.0 < 采样（in_water）
  slot+0xc = inverse_lerp
  slot+0x10 = position.x / +0x18 = position.y（Vec3）
  ```
- **Delete 分支**（行 786-896）：`TreeStorage::remove`（行 833）→ `do{…}while(remove 空 或 !Entities::contains)`（行 834-835：`Entities::contains()` 门控，实体仍在才继续）→ `Commands::insert(TreeMarkedForDeletion)`（行 846-853）→ 查实例位 → `PlayTreeSoundCmd kind=2`（行 890：`slot+8=2`、position@+0x10）。
- **Insert 分支**（行 326-785）：`ActiveWidget::is_dragging`（行 339）→ 树名 SIMD 匹配「clearing/naked_tree1」「clearing/naked_tree2」（行 344-440，len 0x1a=26）与 0x27 串 memcmp（行 346-352）→ `deny_load`/`is_corrupted` 门控（行 523-530，`bVar14`；scale clamp 0.7 行 527-533）→ `Commands::spawn_empty` + insert `TreeComponent`/`TreeAnimation`（alpha = `lerp(0.75,0.55)`）/`TreeMoveBend`/`TreeAmbience`（行 552-609）→ 变体计算（行 613-619）→ 「MISSING_TREE_PREFAB」匹配（行 623-672）→ `random_tree_prefab`/`PrefabName::new`（行 674-681）→ `SpawnPrefabCmd`（stride 0x50，行 697-718）→ `TreeStorage::insert`（行 720）→ `PlayTreeSoundCmd kind=0`（行 738-744）→ insert 冲突 → log + `DespawnPrefabCmd`（行 746-784）。

**对照 Rust**（`process_tree_cmds.rs`）：
- Move：`storage.get(TreeStorageIndex(cmd.index))` → None `log::warn!` / Some 写 position + 采样 + `PlayTreeSoundCmd{kind:1, position}`。**判等**。
- Delete：`storage.remove` → `entities.contains(entry.entity)` 门控 → `commands.insert(TreeMarkedForDeletion)` + `PlayTreeSoundCmd{kind:2, position}`。**判等**（分歧 18 已修属实）。
- Insert：Rust 为**空分支未实现**（行 124-130）。**未实现 T2 诚实**——decomp 证实该分支全链（4 组件插入 + 0x50 SpawnPrefabCmd + TreeStorage::insert + 冲突 Despawn）依赖 core 未提供类型。

**分歧登记核验**：
- **T1**（`PlayTreeSoundCmd` 共享型 `{kind,position}` 缺 `in_water@+9`/`inverse_lerp@+0xc`）：**证实**。decomp 明确写 +9（`0.0<采样`）、+0xc；Rust `_in_water` 计算但未写。audio 侧只按 kind 分派（kind 1 忽略）→ 无观察差异，登记准确。
- **T2**（Insert 分支未实现）：**证实**。decomp 全链复杂，依赖 InsertTreeCmd 0x48 / SpawnPrefabCmd 0x50 / TreeMoveBend / TreeAmbience 载荷，共享 `InsertTreeCmd`（`tree_edit.rs:100`，`{id,position,scale,prefab_name}` 4 字段）不承载。
- 补充：Move 分支查询失败 `unwrap_failed` panic（行 277-280）以 Rust `get_mut Ok` 兜底（graceful vs panic），源码注释已登记。

## §2 replay_tree（0x141bc34f0）— 重建（时序/Move/Delete/Place/undo 判等）

**fresh 反编译**（`fresh-replay_tree.c`）独立复述：
- `front_mut_bucket`（行 68）→ speed = `buffer+0x28`（行 66）；ReplayCommand stride 0x30（行 87），tag==8 = Tree（行 90），edit 指针 @+0x10；`elapsed @+0x18 == 0.0 且非 NaN`（行 93）才执行动画；undo 字节 @+0x1c（行 94）。
- **Move**（edit kind@+0x10==1，行 144-181）：非 undo → `MoveTreeCmd`（stride 0x20）index@+8 = edit+0x30（树 id）、position@+0x10 = edit+0x20/+0x28；undo → 读 **edit+0x14/+0x1c**（旧位置）。
- **Delete**（kind==2，行 92-143）：非 undo → `DeleteTreeCmd`（stride 0x10）id = edit+0x18；undo → 组 0x48 `InsertTreeCmd`（name clone + 多字段，行 106-141）。
- **Place**（kind 其他，行 183-219）：非 undo → 0x48 `InsertTreeCmd`；undo → `DeleteTreeCmd`。
- **时序记账**（行 220-242）：`flags@+8 bit0==0` → 记移除；否则 `delay=cmd@+0xc/speed` 下钳 0.05、`delay<elapsed` → 记移除、`elapsed += GameTime.delta(+0x28)`。
- **逆序移除**（行 245-410）：`Vec::remove` + tag switch Drop（case 8 = TreeEdit Arc）。

**对照 Rust**（`replay_tree.rs`）：逐语句一致——`timing.elapsed == 0.0`（NaN 语义等价）、Move 非 undo `MoveTreeCmd{id, index: id, position: edit_position()}`、Delete 非 undo/Place undo → `DeleteTreeCmd{id: edit.id()}`、Delete undo/Place 非 undo → `InsertTreeCmd{id, position, scale, prefab_name}`、bit0 记账、`delay<elapsed` 移除、`elapsed += time.delta`、`removal.into_iter().rev()` 逆序移除。**判等**。

**分歧登记核验**：
- **T2**（Delete undo / Place non-undo 的 0x48 InsertTreeCmd 重建缺 height/variant/rotation/flags）：**证实**。decomp 明确组 0x48（含 name String clone、position、edit+0x40..0x50 多字段）；共享 `InsertTreeCmd` 只承载 name/position/scale。
- **T3**（Move undo 需 edit+0x14/+0x1c 旧位置，共享 `TreeEditData` 无旧位置）：**证实**。decomp 行 161-162 读 `edit+0x14`/`edit+0x1c`（Vec3 旧位），Rust Move undo 分支为空。

## §3 ui_move_tree（0x14226b1a0）— 重建（载荷判等；分歧 15/16 诚实）

**fresh 反编译**（`fresh-ui_move_tree.c`）独立复述：
- **门禁**（行 101-110）：`uVar8 = mode`；`uVar8-6<0xf` 时 `lVar14=uVar8-5`；通过条件 = `lVar14==0xb`（→mode 0x10=Tree）**或 `lVar14==0 && uVar8==0`（mode 0=None）**，且 `DEV_OVERRIDE==0`。
- `Decorators::iter_removed_ids()`（行 111）位图迭代候选树。
- 每树：PrefabInstanceData 定位（行 170-211）→ `PrefabName::new` + memcmp 特殊 prefab（行 214-232）→ `uVar22 = (匹配 ? 0x40200000=2.5 : 0x3f800000=1.0)`（行 236-239）→ 树位 `+0.5` 抬升（行 242）→ WidgetParams/UiSounds（hover/drag start/drag release/click，行 258-275）→ signifier + UiState::add（行 282-285）→ `CursorTerrainRaycast::position`（行 287）→ 光标 y<0 的混合（行 288-291）→ **`slider3d::into_drag_state`**（行 292）→ switch（行 299）：
  - **case1**（行 300-310）：记起点（`*cache=树位`、`cache+0xc=拖拽偏移`）。
  - **case2**（行 311-365）：`target=光标+偏移` → `sample_at_world_xz` + `GladeBorder::is_pos_inside`；**界外 → 写 DisplayOutOfBorder（行 319-329）+ FeedbackInputOutOfBorder（行 330-340）双事件**；界内 → `MoveTreeCmd`（stride 0x20，行 357-362：`+8=树 id`、`+0x10=x`、`+0x14=y=max(h,0)`、`+0x18=z`）。
  - **case3**（行 366-416）：同采样/门禁 → `MoveTreeCmd`（行 389-394）+ `History::add(TreeEdit Move disc=1)`（行 396-415）。

**对照 Rust**（`ui_move_tree.rs`）：
- MoveTreeCmd 载荷（case2/3 写 `{id, index: id, position=(x, max(h,0), z)}`）：**判等**（RESTORE §3 声明属实）。
- 分歧 15（`slider3d::into_drag_state` 以 `Local<TreeMoveCache>` 距离探测近似）：**诚实**。decomp 状态机（case1 起点/case2 拖动/case3 释放，含树位+偏移量）与 Rust 简化模型不同，已登记。
- 分歧 16（case2 界外双事件在本近似下不可达）：**诚实**。decomp 行 318-341 明确写双事件；Rust 顶部 `is_pos_inside` 界外早退使该分支不可达。
- 分歧 9（候选源 = `Decorators::iter_removed_ids` 位图 vs `storage.iter()` 代理）：**诚实**。

**补登记（新发现）**：Rust 门禁仅 `AppMode::Tree.native_tag()==0x10`；二进制门禁为 `mode ∈ {0x10, 0}`（`AppMode::None`/`Editing(None)` 也放行）。RESTORE 记「AppMode-5∈{11,0}…同一结论」——只取了 11→Tree，漏 0→None 放行面。mode 0 下二进制可处理树拖拽、Rust 早退。**minor 门条件缺失，补登记**（不影响主 verdict）。

## §4 convert_preplaced_trees（0x14226be70）— 重建（Wait flag + 全部门禁判等；分歧 15 诚实）

**fresh 反编译**（`fresh-convert_preplaced_trees.c` + `fresh-query_with_circle_callback.c`）独立复述：
- **Wait 门**（行 122-128）：有未读事件 → `flag=1, count=1`（行 124-125）；否则 flag 置位 → `count+1 > 2`（旧 count>=2）→ `flag=0` 执行转换块（行 127-128）。
- 转换块：`FrozenMap::get<TerrainRockMaskSsbo>` + 名断言（len 0x3f，行 155-167）→ `Decorators::iter_removed_ids`（行 172）→ 每候选树查询（行 333-363）→ 6× `bilinear_sample_at_world_xz`（行 369-380）→ **冲突判定**（行 381-383：`water>0 || garden>0 || rock`）→ 密度 `clamp01(path_sum)`（行 387-397）→ `density<=0` 时 `Circle2d::new` + `query_with_circle_callback`（行 399-402）。
- **墙查询闭包位语义（本次补证）**：`fresh-query_with_circle_callback.c` 行 142-145——圈内命中墙单元 → `*in_R8 = 0`（**清 blocked 位**）并 return；未命中 → 位保持（初始 `density<=0 → 1`）。故：**墙命中 → 位清零 → 走 DeleteTreeCmd；无墙 → 位保持 → TreeEdit 构建 + `insert_starting_edit`**。与 Rust `blocked=true on 墙命中 → delete` **一致**（方向判等）。
- 未冲突（位保持）：组 stride-0x48 TreeEdit 记录（行 440-449）→ 收集 Vec → `History::insert_starting_edit`（行 279）；冲突 → `DeleteTreeCmd` 事件（stride 0x10，payload=树 id，行 455-467）。

**对照 Rust**（`convert_preplaced_trees.rs`）：
- Wait flag 一次性门控（分歧 12 已修）：**判等**（Rust `count+1>2` 与二进制 `2<old_count+1` 等价，触发阈值一致）。
- 6 路采样 + 水/岩/花园/路径密度冲突 + 墙 `query_with_circle`（`blocked=true`）：**判等**（墙方向经闭包 disasm 补证）。
- 分歧 15（`History::insert_starting_edit` 未实现）：**证实**。`history_snapshots.rs:124 insert_starting_edit(&mut ObservedHistoryLedger,…)` 需 `&mut`；`recovered_building/history_transactions.rs:102` 仅 `ledger(&self)` 只读面，`History<S>`=HistoryTransactions 无 `ledger_mut`。Rust 空分支诚实。
- 分歧 13（岩石掩码 buffer 接线）、分歧 9/14（候选源代理）：**诚实**。

## §5 display_glade_border（0x141be0ad0）— 未实现-需重做（分歧①/② 均证实）

**fresh 反编译**（`fresh-display_glade_border.c` + `fresh-DebugLine_show.c` + `fresh-SignifierFactory_new.c` + `fresh-UiSignifierStream_write.c`）独立复述：
- 模式门 `mode != 0x13`（行 50）→ `as_curve2` + Map(closure$0=地形抬升)（行 51-57）→ DisplayOutOfBorder 事件门（行 90）→ **`DebugLine::new/color/line_width/set_ui_pass/show`**（行 116-124，show 在行 124）→ `DEV_OVERRIDE_IS_DEMO_MODE==1` → ShowDemoBorderOverlay（行 125-139）→ `GameConfig+0x121==0` → `perimeter` + `format!("glade_border" + frame_count)`（行 142-153）→ 组 48B 记录 `{tag=1,kind=1,折线 Vec,perimeter f32@+0x28}`（行 161-166）→ `SignifierFactory::new`（行 167）+ `UiSignifierStream::write`（行 173）。
- `DebugLine::show`（0x140b04f70）fresh：算折线总长（行 53-98）+ 组 0x50 记录 → `AppendOnlyVec::push<UiAppendSignifier>`（行 126-127）。
- `SignifierFactory::new`（0x14089d8b0）fresh：`{len, ptr, len, &inner, marker}` 5 qword = **40B**。
- `UiSignifierStream::write`（0x140b2cf90）fresh：拷贝 0x28B 槽记录（stride 0x28）。

**对照 Rust**（`display_glade_border.rs`）：
- 控制流 1-3、closure$0 抬升、demo 覆盖、格式串（无空格）：**判等**（既有声明维持）。
- **分歧①（DebugLine::show 未落库）**：**证实**——core `utils/debug_viz.rs` `impl DebugLine` 仅有 new/color/set_ui_pass/dashed/line_width，**无 show**（模块 doc 注明 show 系列留组件/ECS 包）；`_line` 构建后 drop。二进制确实调用 0x140b04f70。
- **分歧②（perimeter SignifierFactory 写路径未落库）**：**证实**——core `signifier_create.rs:81 UiSignifierStream::write(&mut self, record: UiAppendSignifier)` 仅接受 `UiAppendSignifier`（typed `slots: [UiAppendSignifier; N]`），无法承接 SignifierFactory 记录/closure vtable；`let _ = &mut *signifiers;` 空分支。不以 UiAppendSignifier 投影冒充——诚实。
- verdict：**未实现-需重做**，登记 country-core gap（DebugLine show 系列 + UiSignifierStream 承接 SignifierFactory/closure 唤醒）。

## §6 closure$1（0x141be1080）— 重建（链+2×insert 判等；唤醒 gap 诚实）

**fresh 反编译**（`fresh-closure1.c`）独立复述：
- 复制折线（`alloc 12*len + memcpy`，行 20-38，env+0x10=len、env+8=ptr）→ `SignifierEntityBuilder::new` → `set_ui_pass` → `animation_duration` → `custom_shader` → `build`（行 39-43）→ 2× Commands insert：
  - `DrawTopology`：值 `0x4000000000000001`（行 57，tag 1 + f32 2.0，mangled 名 `insert<DrawTopology>`）。
  - `DashedLineLength`：`env+0x18 f32 × 0.7`（行 61-72，mangled 名 `insert<DashedLineLength>`）。

**对照 Rust**（`display_glade_border.rs::closure_1`）：
- 折线复制（from_raw_parts to_vec）：**判等**。
- payload 6 qword（首 = mesh_kind `0x8000000000000002`，余 `{len,ptr,len,0,0}`）：与 disasm（RESTORE §6 引用）一致；core `SignifierEntityBuilder::new` 现正确拆分 mesh_kind=payload[0]（signifier_factory.rs:257-269，coregap-1 需修项已修）。
- `set_ui_pass(0)/animation_duration(0.0)/custom_shader("dashed_line")/build`：链**判等**（build 内组件 GAP 已在 core 登记）。
- 2× insert（`DrawTopology::Alt(2.0)` + `DashedLineLengthView(env.length*0.7)`）：**判等**（二进制组件名 `DashedLineLength` vs core `DashedLineLengthView` 为布局等价重命名，RESTORE 已注明）。
- **唤醒未接线**：vtable 0x142efa150 → call_once 0x141be0a30 由 signifier 消费端触发；core `UiSignifierStream` 无 closure 槽位 → `#[allow(dead_code)]` 未达。**诚实 gap**（RESTORE §6 声明属实）。
- verdict：**重建**（可调用但观察可达性依赖 country-core 唤醒接线）。

## §7 feedback_input_out_of_border（宿主 0x141be2280）— 判等

**fresh 反编译**（`fresh-feedback_host.c`）独立复述：
- 事件读者消费（行 44-101，含游标推进到末尾）；`has_feedback`（行 79-81）；**cache@+0x158==0（上一帧无反馈，上升沿）**才写 `PlayUiSound`（行 82-96）：`slot+0 = 4`（kind）、`slot+0x18 = 事件计数`（id）、stride 0x20；`cache@+0x158 = has_feedback`（行 98）；last_run tick @+0x130（行 99）。

**对照 Rust**（`feedback_input_out_of_border.rs`）：`feedback.read().last().is_some()` + `!cache.had_feedback_last_run` 上升沿去抖 + `PlayUiSound::variant4(0)`（`{kind=4@+0, 载荷@+8..0x17, id@+0x18}`，stride 0x20）+ cache 置位/复位。**判等**。RESTORE 声明（kind=4@+0、id@+0x18、stride 0x20、EventWriter 自动补 id）全部证实。

## §8 construct_timberframe_rectangle（0x141a41ba0）— 重建 + 梁写路径

**fresh 反编译**（`fresh-construct_timberframe_rectangle.c`）独立复述（相位）：
1. 槽空 → `FrozenMap::get<WoodenPlankSsbo>` + 名断言（len 0x3d）+ vtable → `new_allocation` 存槽（行 137-201）。
2. 二次查询 + 互斥（allocation+0x18）+ `SparseInstanceBufferWriter::clear`（行 202-241）。
3. 事件迭代（stride 0x28），`while((*event & 1)==0)` 处理 bit0=1 条目（行 274-321）。
4. **垂直梁**：`as_curve3` 每顶点 → `orientation_mat3`（0x140c99570）→ 四元数 → `impl$1::from` → push（截面 0.2），count++（行 326-421）。
5. **等距梁**：相邻点 `n=floor(SQRT(d²)/1.51)`，`1<n` 时 `count+=n-1`，`i in 1..n` `u=i/n` lerp → push（截面 0.17999999=0x3e3851eb）（行 428-560）。
6. **裙层**：`split_height_into_skirts` + 栈 `{0.0, cache+0xc}` 两固定环；每环每边 `mid + xvy + rotation_from_normal` → push（截面 0.2），count++（行 567-727）。
7. **屋脊**：`if(event[0x1c]==0)` → `roof_height_ws_from_wall_shape` + `ridge_length_01_from_rectangle_ratio` + `RoofRidgeDims::calc` → `as_curve2`×2 → `count+=min(lenA,lenB)`（每对中点 + rotation_from_normal + push 截面 0.2）；`perimeter()>0.5` → `get_point`×2 中点 +1 根（行 736-896）。
8. `BeamCount+8 = lVar19`（每事件末尾，行 899）；推入失败 `uVar24 bit0` → log「Timberframe constructor ran out of memory」（行 278-299）。

**对照 Rust**（`timberframe_rectangle.rs`）：相位/计数/门控逐语句**判等**（含 `n=⌊d/1.51⌋`、截面 0.2/0.18、裙层两固定环环序、屋脊 `min(lenA,lenB)` + `perimeter>0.5→+1`、`BeamCount.count(i64@+8)` 写回、`event[0x1c]` 门控 = `cmd.shape[24]`）。写路径（`writer.clear` + 逐梁 `push(WoodenPlankSsbo{position,size,rotation})` + ran-out-of-memory 标志）**已落库**。

**分歧登记核验**：
- 库面 `Res<AssetSsboLibrary>`（frozen 内部可变）→ core 占位 `RwLock<HashMap>` 需 `&mut` → `ResMut`：**诚实**（同 world_raster/rasterize_buildings 写库面登记）。
- `orientation_mat3`（0x140c99570）本地镜像：**判等证实**——fresh decomp 写 `Mat3::from_cols((axis.x,0,axis.y), Y, (-axis.y,0,axis.x))`，与 Rust `orientation_mat3` 完全一致（含 `in_RCX[6]=uVar2^0x80000000` = -axis.y）。
- `rotation_from_normal`（0x140c90a40）→ `beam_rotation(edge)=Quat::from_mat3(orientation_mat3(edge))` 替代：**诚实 I 级**。fresh decomp 证实 0x140c90a40 为法向→四元数算法（先叉乘+归一化+半 sqrt），与 beam_rotation 的矩阵→四元数非逐语句等价；RESTORE 已标 I 并给 utils 补 `rotation_from_normal` 后换回的明确路径。另注：beam_rotation 输入 edge 未归一化，glam `Quat::from_mat3` 对非正交列的行为为潜在次差（I 级内，不推翻登记）。
- `RenderInstance` 中间记录（`impl$1::from` 输入面）字段序 I：**诚实**——decomp 每 push 前经 `impl$1::from()` 组记录，Rust 按 core `WoodenPlankSsbo` 直写。
- 事件 bit0 门控（rect bit0=1）以 `get_shape()` 代理：**诚实 I 级**（core 契约未暴露事件首字节）。
- `as_curve3` 高度实参（0.0 复用调用点）：**诚实 I 级**（只影响梁 y 位、计数面等价）。

## §9 construct_timberframe_circle（0x1421b67a0）— 重建 + 梁写路径

**fresh 反编译**（`fresh-construct_timberframe_circle.c`）独立复述：
- 空槽 → **`unwrap_failed` panic**（行 126-129，非 return；无 new_allocation——**与 rect 不同，不 clear**）→ 互斥 + 事件迭代（stride 0x28），`while((*event & 1)!=0)` 处理 **bit0 清零**条目（行 182-235，与 rect 相反）。
- **主环**：`count = trunc(perimeter/1.5)` 下钳 6（行 239-251）→ `as_points2_ptcount` → `u=i/(count-1)`（越界/NaN panic，行 260-278）→ `get_pos_at_u` + xvy → 高度 cache+0xc、截面 0.2 → push（count 根，行 279-317）。主环旋转输入 = `impl$1::from` 面（初始 `{0, cache+0xc, 0, 0}`）。
- **裙层**：`split_height_into_skirts` + `{0.0, cache+0xc}` 两固定环；每环 `u=i/(count-1)` 相邻点法向 → `xvy + rotation_from_normal` → mid，高度 `h+cache+8` → push（每环 count 根）（行 319-466）。
- **屋顶环**：`if(event[0x1c]==0)` → `roof_height_ws_from_wall_shape` → 高度 `roof_height + cache+8 + cache+0xc` → 同采样 + rotation_from_normal → push（count 根）（行 475-581）。
- `BeamCount+8 = lVar15`（行 584）；ran-out-of-memory log（行 185-209）。

**对照 Rust**（`timberframe_circle.rs`）：`count=max(6,⌊perimeter/1.5⌋)`、`u=i/(count-1)` 采样、裙层 `{0.0, main_height}` + `mid.y+ring+skirt_offset`、屋顶环 `roof_height+skirt_offset+main_height`、空槽 panic、BeamCount 写回——**判等**。**无 writer.clear（与 rect 不同）——Rust 同样不 clear，忠实**。

**分歧登记核验**：
- 主环朝向 `Quat::IDENTITY` 常值占位（RenderInstance 字段序 U）：**诚实 GAP**——decomp 主环 push 前 `impl$1::from()` 输入为初始态（`{0, cache+0xc, 0, 0}`），RESTORE 声明与实测一致，明确待 RenderInstance 合同。
- 库面/rotation_from_normal/bit0 代理：同 rect，诚实。
- 屋顶环高度公式 `roof_height + cache+8 + cache+0xc`：**判等证实**（行 482-483）。

## §10 simulate_roof_tile_for_audio（0x1421a7090）— 判等（wave3 修正属实）

**fresh 反编译**（`fresh-simulate_roof_tile_for_audio.c`）独立复述：
- `dt==0 且非 NaN → return`（行 49-50）。
- 逐 roof（is_flat 跳过，行 123-124）；RoofTileSimulation 记录 stride 0x18：`[0]=cur, [2]=prev_h, [3]=angle, [4]=accum, [5]=flag`（行 120）。
- `over = cur/dt <= -120`（行 129）；`smaller = target-cur < prev_h`（行 130）；`angle' = dt*-45.126003 + angle`（行 131）；`mixed = dt*angle' + mix`（mix 掩码：smaller||cur>=0||over → prev_h，否则 target，行 132-136）。
- `mixed<=target`：`15.0 < angle'` → clamp panic（行 137-140，**用新角**）；`mixed = NaN?target:max(target,mixed)`（行 142-147）；`angle_final = min(max(angle', (cur/dt)*0.166), 15.0)`（行 148-155，**用新角**）。
- `decay = expf(dt*-13.300004)`（行 157）；`new_h = (1-decay)*target + decay*mixed`（行 158-159）；写 `[2]=new_h`、`[3]=angle_final`。
- `diff=|new_h-target|`：`flag==0` → `diff<0.1` 写 `[4]=0.0`（行 164-166）否则 RoofSound kind 5（行 168-188）；`flag!=0 && diff<=0.1` → `accum>0.01` 时 `(accum-0.01)/0.19` clamp[0,1] + RoofSound kind 4（行 190-222）；`else` → `flag=1` 且 **`angle_final<0`** → `[4]+=dt`（行 226-233）。
- 写 `[5]=flag`（行 235）。

**对照 Rust**（`simulate_roof_tile_for_audio.rs`）：逐语句**判等**。wave3 修正三处全部**属实**：panic 判据（`15.0<angle'`）、角度钳制（`angle'` 非旧角）、`angle<0` 累加器门控（钳制后**最终角** `final_angle` 非旧角）。RESTORE「判等（修正 angle' 三处）」成立。

## §11 highlight_roof（0x141a284b0/0x141a31280）— 重建（数据流判等；pad 覆盖 I 级）

**fresh 反编译**（`fresh-highlight_roof_store.c` 权威 + `fresh-highlight_roof_iter.c` 对照）独立复述：
- `RoofHighlight.tag@+1 = 0`（写前清 tag，行 85/96）。
- 逐事件（stride 0x18：`wall u64@+8`、`color 判别 u8@+0x10`）：`value ← wall 低 u32`（8 字节写覆盖 `{value,pad}`）、`tag ← color 字节`、`HighlightColor::to_rgb()` → `EditingHighlight.color`（Vec3），tick 逐事件写（行 125-133）。
- iter 变体（0x141a284b0）经 `EventIteratorWithId::next`，读出偏移与 store 变体不同（store 读 +8/+0x10，iter 读返回指针 +0/+1）。

**对照 Rust**（`highlight_roof.rs`）：`tag=0` 清位、`value=wall 低 u32`、`tag=color.0`、`color=cmd.color.to_rgb().into()`——**数据流判等**。分歧（`*puVar6 = uVar7` 8 字节写覆盖 pad 不可表示；两单态读出不同以 store 为准）**诚实 I 级**。RESTORE verdict 重建成立（严格说数据流判等、仅 pad 位面不可表示）。

---

## §12 分歧登记核验汇总 + 补登记

**核验结论：tree-glade 与 timber-roof 两份 RESTORE 的分歧登记全部诚实准确**——`T1`（PlayTreeSoundCmd in_water/inverse_lerp 缺位）、`T2`（InsertTreeCmd 0x48 未落库，Insert/replay 重建分支未实现）、`T3`（Move undo 旧位置 edit+0x14/+0x1c 不可表达）、`分歧①`（DebugLine::show 未落库）、`分歧②`（UiSignifierStream 仅收 UiAppendSignifier，SignifierFactory/closure 路径不可承载）、`分歧 12`（Wait flag 已修判等）、`分歧 15`（insert_starting_edit 需 `&mut ObservedHistoryLedger`，HistoryTransactions 只读 `ledger()`）、`分歧 16`（case2 界外双事件不可达）、`ResMut` 库面、`orientation_mat3` 镜像（判等）、主环 `Quat::IDENTITY` 占位、事件 bit0 门控 `get_shape()` 代理、`rotation_from_normal`→`beam_rotation` I 级替代——全部经 fresh 反编译证实，无高估、无冒充。

**补登记（2 项）**：
1. **ui_move_tree 门禁 mode 0（`AppMode::None`）**：二进制 `mode ∈ {0x10, 0}`，Rust 仅 `AppMode::Tree(0x10)`。RESTORE「AppMode-5∈{11,0}…同一结论」漏掉 0→None 放行面（minor，mode 0 下二进制可处理树拖拽）。
2. **convert_preplaced_trees 墙查询闭包位向（本次补证）**：`query_with_circle_callback<closure_env$0>`（0x1425defd0）fresh 显示圈内命中墙 → `*in_R8 = 0`（清位）；无墙 → 位保持 → 转换。Rust `blocked=true on 墙命中 → delete` 方向**判等**。原 RESTORE 方向（冲突→DeleteTreeCmd）经本补证成立（此前依赖 reviewer-eco-treetool 复核 + HISTORY 推断，现为闭包 disasm 直接证据）。

**其他观察（非分歧）**：
- construct_timberframe_circle **不调用 writer.clear**（与 rect 不同）——Rust 同样不 clear，忠实二进制。
- closure$1 二进制组件名 `DashedLineLength` vs core `DashedLineLengthView`：布局等价重命名，RESTORE 已注明。
- `highlight_roof`/`gpu_flush_buffers` 未入 `add_roof_systems` 的注册面说明：与本批 verdict 无关，维持既有登记。

---

## 证据索引

- fresh 反编译（`reports/wave3/reviews/tg-tr/evidence/`）：`tg-batch1/batch2/batch3/find/query.result.json` + `fresh-{process_tree_cmds,replay_tree,ui_move_tree,convert_preplaced_trees,display_glade_border,closure1,feedback_host,construct_timberframe_rectangle,construct_timberframe_circle,simulate_roof_tile_for_audio,highlight_roof_store,highlight_roof_iter,DebugLine_show,SignifierFactory_new,UiSignifierStream_write,orientation_mat3,rotation_from_normal,query_with_circle_callback}.c`。
- 身份：get_metadata 每批首步（tiny-glade.exe，B patched，base 140000000）。
- 源码对照：`crates/systems/{tree-tool,glade-border,timberframe,roof}/src/*.rs`、`crates/country-core/src/{utils/debug_viz.rs, systems/ui/signifiers/{signifier_create,signifier_factory}.rs, resources/{history/{tree_edit,replay_cmd_buffer}.rs, tree_storage.rs}, recovered_building/{history_transactions,history_snapshots}.rs, systems/mode_manager/types.rs}`。
