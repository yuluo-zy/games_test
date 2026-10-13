# WP17 — 回放编排（replay orchestration）业务逻辑合同

B 版本（EXE sha256 `cf048e27…`，PDB sha256 `2d9686b8…`）。本文件记录
`crates/country-core/src/systems/history/{replay_mode, finish_replay,
signifier_replay_active, replay_pop_bucket}.rs` 与
`crates/country-core/src/resources/history/replay_cmd_buffer.rs` 的
**Observed / Inferred / Unknown** 证据与 typed gap。实现见 `src/`。
差分仅用于裁决关键数值/边界歧义，不作为实现手段。

## 一、ReplayCmd / ReplayBucket / ReplayBuffer（replay_cmd_buffer.rs）

### O — 直接观察（Ghidra 反编译 + PDB 骨架）

#### ReplayBuffer（O：`pop_front` 0x1409f18f0、`len` 0x140949170、各回放系统的前段）
环形缓冲：`{ head @+0x00, buf ptr @+0x08, cap @+0x10, len @+0x18,
modification_counter @+0x20 }`。`pop_front` 先 `counter+1`（`+0x20`），
len 非零时 `len-1`、按 `(cap - head)*0x40` 取前桶 8 个 u64 移到返回值
（0x40=ReplayBucket 元素）。前桶指针模式（`replay_wall`/`replay_terrain`/
`replay_auto_clutter`/`replay_stairs`/`replay_raster` 共用）。

- `new`（插入点 `insert_history_resources` 0x140943e10 内联）：空缓冲。
- `is_empty`（0x1408b36a0/0x140a3f470/0x140af5b30 内联）：`len==0`。
- `len`：`+0x18`。
- `pop_front`（O：0x1409f18f0）：弹前桶，写 modification_counter，推一个
  事件实例（payload 前 8 字节 = `i64::MIN` 哨兵；事件类型 Ghidra 误标
  `LastKnownGraphicsDevice`，U）。
- `front_mut_bucket`（O：0x1409f1a80；`+0x18` 非空检查后返回前桶可变引用；
  Ghidra 展开后为空 stub，语义 = `self.deque.front_mut()`）。
- `add`（内联进 `replay_edit` 0x140af6170）：把构造好的 ReplayBucket
  push 到环形尾部（`VecDeque::grow<ReplayBucket>`）。
- `make_animations_instant`（O：0x1409f1ab0）：遍历所有桶所有 ReplayCmd，
  对每个 `ReplayCmd::make_animation_instant()`。
- `current_replay_cmd`（内联进 `replay_audio_system` 0x140ad5800 /
  `signifier_replay_active` 0x140b1b860）：读前桶首条命令（payload 用）。
- `modification_counter`（内联进 `save_game` 0x140a3f470）。

#### ReplayBucket（O：0x40 元素；构造侧 replay_edit 写 8×u64）
字段语义（O：构造写入 + replay_pop_bucket 消费）：
`commands: Vec<ReplayCmd>`（元素 0x30）、`decorator_diff`（Arc 对）、
`decorator_cull_allow_list`（`Arc<[DecoratorId]>`，to_arc_slice 产生）、
`stairs_diff`（Arc 对）。精确字段偏移在构造侧与消费侧观察不一致
（构造：commands ptr@+0x10/len@+0x18；消费侧 replay_wall 读 ptr@+0x8/
len@+0x10），**布局 U**（typed gap）。

#### ReplayCmd（O：0x30 元素；drop glue 判别字节 + replay_terrain/wall/
auto_clutter/stairs 的字段访问）
判别字节 @+0x00（u8）；通用动画头：`instant` bit@+0x08、`duration` f32
@+0x0c、payload Arc/Vec @+0x10、`elapsed` f32 @+0x18、`undo` u8 @+0x1c。
drop glue（replay_wall/replay_terrain/replay_auto_clutter/replay_stairs
尾部共享 switch）判别映射：

| 判别 | 负载 | 证据 |
|---|---|---|
| 0 / 1 / 2 | `Arc<path_edit::RasterEdit>`（Path/Cobblestone 系列） | drop_slow<RasterEdit> ×3 |
| 3 | `Arc<Option<terrain_edit::TerrainEdit>>` | drop_slow<Option<TerrainEdit>>；replay_terrain 判 `*==3` |
| 4 | `replay_wall::WallReplayCmd`（内联，无 Arc） | drop_in_place<WallReplayCmd>；replay_wall 判 `*==4` |
| 5 | `Arc<Option<roof_edit::RoofEdit>>` | drop_slow<Option<RoofEdit>> |
| 6 / 7 | `Arc<Option<decorator_edit::DecoratorHistoryEdit>>` | drop_slow ×2 |
| 8 | `Arc<Option<tree_edit::TreeEdit>>` | drop_slow<Option<TreeEdit>> |
| 9 | 自动杂物回放命令（replay_auto_clutter 判 `*=='\t'`=9） | drop glue case 9 |
| 10+ | 楼梯/快照等（replay_stairs 判 `*=='\n'`=10） | drop glue default |

`impl$0..impl$7::from`（20/4 宿主点）：各领域命令 → ReplayCmd 转换
（`generate_replay_cmds_from_edit` 内联）。`impl$15::eq`（replay_edit
内联）：ReplayCmd 相等（桶分组判定用）。`make_animation_instant`
（make_animations_instant 内联）：把动画命令标记为即时。

### U — 未知
- ReplayBucket 精确字段偏移/标签布局。
- ReplayCmd 判别 9/10+ 的精确负载类型（auto_clutter / stairs / 快照
  载荷；Ghidra 误标为 CrashMiscInfo / PhysicalSize<u32> 为 Arc 内层）。
- pop_front 所推事件的确切类型与用途。

## 二、replay_edit / generate_replay_cmds_from_edit（history_process 0x140af6170 / 0x140143fb50 等）

### O — 直接观察
- `replay_edit` 0x140af6170（PDB 名 `history_process::replay_edit`）：
  - 遍历目标 MultiEdit 的 `edits`（0x18 元素），逐个调
    `generate_replay_cmds_from_edit<closure_env$0>`（0x140143fb50，
    泛型体按 closure 实例化）展开为 `Vec<Option<ReplayCmd>>`。
  - `undo`（in_DL 非零）且命令数 >1 时**反转命令列表**（O：桶排序前的
    命令数组逆序，与原机器码逐字节一致；匹配
    reconstruction/history/transaction.rs 的 `ordered_edits` 反转观察）。
  - 按 ReplayCmd 判别把可合并命令**分组为 ReplayBucket**
    （switch 0..10 的跨命令比较 = “可与下一命令同桶”判定；`WallReplayCmd`
    单 AdjustHeight 特判 `is_single_adjust_height_cmd` + `eq`）。
  - 附加 MultiEdit 辅助容器的装饰 diff（`Arc<DecoratorHistoryDiff>`）、
    楼梯 diff（`Arc<StairsHistoryDiff>`）、装饰 cull allow-list
    （`to_arc_slice<DecoratorId, FlatMap<...>>`）到各桶。
  - 逐桶 push 到 ReplayBuffer（`ReplayBuffer::add` / VecDeque::grow）。
- `WallReplayCmd::is_single_adjust_height_cmd`（replay_wall 内联 0x140af6170）：
  单 AdjustHeight 命令判定（`Vec` 元素 0x18 计数==1 且 Arc 指针匹配
  `&DAT_8000000000000002` 常量）。
- `generate_replay_cmds_from_edit<C>`：泛型展开函数；闭包 C 把
  `&EditType` → `&mut Vec<ReplayCmd>`（4 个宿主实例化：garden
  0x140143d2c0 / replay_edit 0x140143fb50 / path 0x14014423e0 /
  water 0x1401444c70）。`ReplayCmd::from(...)`（impl$0..impl$7）在其中
  调用。
- `schedule_replay_commands`（+closure_env$2）：按 `Vec<usize>` 分组
  索引从命令数组构建 ReplayBucket（replay_edit 内联）。

### I — 推断
- 每个领域（wall/terrain/raster/pop_bucket）回放系统对应一类判别；
  一次 undo/redo 展开的桶按顺序 push，领域系统逐帧从队首取桶。

## 三、replay_mode（0x140a7c550）

### O — 直接观察
- 系统签名（enriched 文件级接口 7e240e5e…）：
  `(Res<AppMode>, Res<PendingHistoryCommands>, ResMut<ReplayBuffer>,
  EventReader<ModeJustChanged>, ResMut<HoldToFastReplay>,
  ResMut<RestoreAppTo>, ResMut<ReplayModeElapsedTime>,
  EventWriter<HistoryCmd>, Res<ActionSetStates>, Res<GameTime>,
  Res<UiState>, Res<ReplaySpeedMultiplierOverride>)`。
- `RestoreAppTo`（enum2$；`default` @源码行19、`set` @行24、`get` @行50）：
  `Option<AppMode>` 语义；`set` 在 `mode_manager` 0x140a86b60 内联、
  `get` 在 `finish_replay` 内联。
- `HoldToFastReplay`：enum2$ 资源（长按快进标记）。
- 逻辑（反编译 0x140a7c550）：
  - 读 `PendingHistoryCommands`（Vec<HistoryCmd>，4 字节元素）；空时
    按 `HoldToFastReplay` / 输入动作决定是否发 `HistoryCmd`。
  - AppMode==0xd（Replay）时：
    - 若 `HoldToFastReplay` 按住且未到阈值：发 `HistoryCmd(0x03000100)`
      （redo，byte[1]==1 的 `0x3000100`/`0x3000000` 命令字变体）。
    - `UiState::clearing_canvas_w_layer_and_sound()`；`time_continuous_lerp`
      驱动 `ReplayModeElapsedTime` 计时；`GameTime` 累加。
  - AppMode!=Replay：日志（log max_level gate，DAT_143634d88 惰性标记）。
- `ReplayModeElapsedTime`（定义在 signifier_replay_active 模块）：计时资源。

### U
- HistoryCmd 命令字 0x03000000/0x03000100/0x03000200 的精确语义
  （`0x03000000` 为某“帧重放”字，与 interrupt 0x02000200 并列）。
- HoldToFastReplay 的精确状态位。

## 四、finish_replay（0x1408b36a0）

### O — 直接观察
- 系统签名（enriched 8ba632a7…）：
  `(ResMut<AppMode>, EventReader<HistoryCmd>, ResMut<Toolbar>,
  ResMut<MetaCommander>, Res<PendingHistoryCommands>, Res<ReplayBuffer>,
  Res<ActionSetStates>, Res<RestoreAppTo>,
  EventWriter<CmdUnfreezeSheep>)`。
- 逻辑（反编译 0x1408b36a0）：
  - AppMode==0xd（Replay）：若 `PendingHistoryCommands` 恰 1 条且
    byte[1]==1（redo）→ 写 `CmdUnfreezeSheep` 事件（EventWriter
    `grow_one<EventInstance<...>>`；Ghidra 误标 WriteSettingsFileCmd）。
    若 pending 空 && ReplayBuffer 空（`+0x18==0`）→ 回放完成：
    - `RestoreAppTo::get()` → 写回 AppMode（`*AppMode = restore`）。
    - 恢复模式为“游玩模式”范围（uVar8-5 ∈ [0,0xf] → 模式 6..20）
      时 `Toolbar::switch_mode` / `switch_category`；
    - `MetaCommander::peek` 顶部为 `i64::MIN` 哨兵时 `log`
      （“Replay 结束”日志）+ `pop` 命令。
  - AppMode!=Replay：MetaCommander 顶部哨兵检查 → 日志。
- 内联依赖：`ReplayBuffer::is_empty`、`PendingHistoryCommands::is_empty`、
  `RestoreAppTo::get`、`switch_app_mode`、`Toolbar::switch_mode/switch_category/
  last_selected_mode/next_mode`、`MetaCommander::peek`、`VecDeque::back/
  front/pop_front`（MetaCommander 命令队列）。

### I
- “回放完成” = ReplayBuffer 与 PendingHistoryCommands 同时空 + 恢复
  AppMode；`CmdUnfreezeSheep` 在 redo 命令被消费后发出（重新激活绵羊）。

## 五、replay_pop_bucket（0x140b38250）

### O — 直接观察
- 系统签名（enriched be1df256…）：
  `(ResMut<ReplayBuffer>, EventWriter<InputReplayCommand>,
  ResMut<DecoratorStorage>, ResMut<DecoratorArchivist>,
  ResMut<DecoratorIdGen>, ResMut<StairsState>, ResMut<RaycastWorld>,
  ResMut<DecoratorPhysicsColliders>, Res<DeferredDecoratorOpEvents>)`。
- 逻辑（反编译 0x140b38250）：
  - 前桶 `Vec<ReplayCmd> len(+0x10) == 0` 时 `ReplayBuffer::pop_front()`：
    - 若桶含装饰 diff（`Arc<DecoratorHistoryDiff>`）：
      `DecoratorStorage::restore_decorators<closure_env$0>`（closure 宿主
      0x14017bdae0 = `decorator_edit::send_state_diff_events<...closure_env$0>`）。
    - 若桶含楼梯 diff（`Arc<StairsHistoryDiff>`）：`StairsState::restore_backup`
      + `apply_diff` + `iter_edges_and_stubs`；IndexMap clear；+0x178=0。
    - `DecoratorArchivist::extract_diff()`；drop_inner_table<
      (DecoratorStorageAddr, DecoratorStorageHistoryDiff)>。
  - 空桶也调 `RaycastWorld::flush_colliders()`。
- closure$0 内联宿主 = `send_state_diff_events<closure_env$0>` @ 0x14017bdae0。

### I
- replay_pop_bucket = “队首空桶弹出一致”：领域回放系统把桶内命令清空后，
  本系统负责弹出桶并应用装饰/楼梯 diff（含碰撞/归档收尾）。

## 六、signifier_replay_active（0x140b1b860 + 0x140b1d680）

### O — 直接观察
- 系统签名（enriched a3c33749…）：
  `(Res<ReplayBuffer>, ResMut<ReplayModeElapsedTime>,
  Query<(&mut Transform, &SignifierReplayIndex)>,
  Res<MainCamera>, Res<GameTime>, Res<CursorTerrainRaycast>,
  Res<ReplayBuffer>)`。
- `ReplayModeElapsedTime` 资源（enum2$；判别==1 时活跃）；`SignifierReplayIndex`
  组件（挂到 signifier entity）。
- `ease_out_elastic_jiggle`（@源码行75）：`0.5*exp2(-10t)*sin(7.853982t)`。
- 逻辑（反编译 0x140b1b860）：
  - `ReplayModeElapsedTime` 判别!=1 → 返回；判别==1 时累加 GameTime delta。
  - 计算弹性抖动进度：`f = frac(elapsed / clamp(0.9/dt, 0.5, ..))`。
  - 遍历 Query：每个 `SignifierReplayIndex` entity 从
    `CursorTerrainRaycast`/`MainCamera` 射线表取目标位置（索引匹配），
    用 `CameraMatrices::eye_position` + `Transform::look_at` 计算朝向，
    把 Transform 位置向目标插值（jiggle 偏移），并写 ReplayBuffer
    `current_replay_cmd` 驱动的激活态。
  - `register_required_components` 0x140b1d680：注册所需组件。
- 消耗 `ReplayBuffer::current_replay_cmd`（0x140b1b860 内联）与
  `ReplayBuffer::is_empty`。

### I
- “回放激活”指示：游标处 signifier 弹性抖动，由 ReplayBuffer 状态驱动。

## U（编排侧汇总）
- HistoryCmd 未证命令字；HoldToFastReplay/RestoreAppTo/ReplayModeElapsedTime
  精确布局与 AppMode 判别数值完整表（0xd=Replay 已证）。
- 各领域命令 → ReplayCmd 的完整 `from` 映射与 EditType 判别对应。
- ReplayBucket 字段偏移。
