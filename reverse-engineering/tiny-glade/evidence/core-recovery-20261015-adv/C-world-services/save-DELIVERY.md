# save-DELIVERY — 实现交付（2026-10-16，C 组实时复合 agent）

> 对照 save-DELTA 的合成 agent 应关闭缺口逐项落地。文件：`crates/country-core/src/systems/save/*`
> （组内）。共享文件 `startup/run_game.rs` 生产接线获中央仲裁批准。
> 逐项分离：**真实代码接线 / candidate 链测试 / 原侧验证 / 仍缺项**。

## 1. 生产接线（DELTA 合成 #1：save_thread/save_game/snapshot GC 无生产调用方）

- **真实代码接线**：
  - `systems/save/save_thread.rs` 新增 **`SaveThreadHandle`** 资源 + **`spawn()`** +
    **`init_save_thread`** 系统（幂等：已绑定命令端则跳过）：生产 `save_thread`
    独立线程（commands/error/shutdown_ack 三通道 + `UnsavedFileList`），命令端绑定到
    `AutoSaveState`；
  - `systems/save/mod.rs::add_save_load_systems_impl` 注册 `save_game_system::<()>` +
    `load_game_system` + `init_save_thread`；
  - `startup/run_game.rs::register_core_startup` 调用 `add_save_load_systems_impl(schedule)`
    （**中央仲裁批准**）——save 系统进入生产 Schedule；
  - `save_game_system` 节拍器命中 → **真实备份生产**：`BackupDecision.counter →
    history-auto-{counter}.json`（`AsyncSaveCmd::BackupSave`，`remove_counter` 一并携带），
    log "Capturing a game snapshot" + `CaptureSnapshotRequested` 事件（宿主构建快照 Arc）。
- **candidate 链测试**：`init_save_thread`/`spawn` 为通道装配纯逻辑（save_thread 循环已
  有 `save_thread_receives_shutdown_with_flush_and_acks` 单测覆盖 Shutdown 语义）。
- **原侧验证**：save_thread 0x141720680 主循环已机器码证实（DELTA §1 全部"真实实现"；
  常量 CAP/THRESH/STEP/gate 已 RNE 核实）；`init_save_thread` 对应 run_game_thread 装配点。
- **仍缺项**：`capture_game_snapshot_save_data` 真实域节采集器（wall/roof/decorator 三
  采集器 FUN_141305170/141308b40/141309840）未接（分属墙/屋顶/装饰域，宿主/对应域恢复后
  接入）；`History::set_current_snapshot` → `CaptureSnapshot(9)` 完整发送链未接（宿主
  响应 `CaptureSnapshotRequested` 事件）。

## 2. AsyncSaveCmd / GameSaveData 字段名纠错（DELTA #2）

- **真实代码接线**：`systems/save/save_game.rs`：
  - 每个 `AsyncSaveCmd` 变体标注**语义字段**（由分发行为/存档 codec 合同推导，非原字节
    布局字段名）；已观测载荷分段（tag@+0、OpenFile path 段、ChangeSection +8/+0x10/
    +0x18/+0x20、ReplaceEdits {byte 标志@+3, edit 列表 ptr/len}）登记，字段名 U/待取证；
  - `GameSaveData` 显式标注 `sections`/`edits` 为**自定语义名**，240B 完整字段名 U/待取证
    （修正"未显式标注自定名"水分）。
- **仍缺项**：各变体载荷的**原字节字段名**仍 U（SPEC §1.4 待取证）；不编造。

## 3. ManualBackup 身份比对 + retention 删除（DELTA #3）

- **真实代码接线**：`systems/save/save_thread.rs`：
  - `MANUAL_BACKUP_IDENTITY_OFFSET = 0x53` / `MANUAL_BACKUP_SNAPSHOT_TAG = 0x746f687370616e73`
    （"snapshot" u64 小端，O）；`perform_manual_backup` 读影子文件 +0x53 身份字节，
    命中 snapshot 标记 → 拒绝备份；
  - `remove_stale_backup(folder, counter)`：删除 `history-auto-{counter}.json`，日志
    "Backup limit reached; removing " / "Error deleting stale backup "；
  - `execute_save_cmd::BackupSave` 携带 `remove_counter` → 备份后执行 retention 删除。
- **原侧验证**：0x141721aa0-normalized.txt default 区行 2666-2718（`param_2[0x53]`/
  `+0x299..0x2a7` 身份比对 + "snapshot" 常量）；行 2698-2718 复制路径。
- **仍缺项**：`+0x299..0x2a7` 16 字节旧态身份比对载荷结构 U（待取证，不编造）；与
  `locked_backups` 的删除竞态策略 U（SPEC §3.4）。

## 4. sheep 节 producer 接线（中央仲裁：B 组供接口）

- **真实代码接线**：`save_game.rs` 新增 `SerializedJson::from_raw_string` +
  `sheep_section_serialized(Option<&SheepControlSnapshot>)`（调 B 组
  `sheep::control::sheep_section_json`，None → null）。真实 producer 路径：
  save_game 同步保存的域采集器（X4 Query → `capture_sheep_snapshot`）→ 本入口 → Sheep 节
  （SaveFileSection::Sheep=5 @0x140a51d40）。
- **原侧验证**：B 组接口字段名 rne-read 核实（0x142c91fa9 等）；`to_json_string<Option<_>>`
  @0x140a51d40。
- **仍缺项**：X4 Query 采集器本身（sheep 域 Query → capture）未接（宿主/域恢复后接入）。

## 5. save_thread Shutdown 无 flush 分支（DELTA #5）

- **真实代码接线**：`save_thread` 无 flush 且有活动档分支标注**待取证**（原版走 0x141721104
  未恢复 jumptable）；当前直接 break 退出（语义标 O 实为待取证，诚实登记）。

## 6. load_game 系统本体（DELTA #6 + session DELTA #1）

- **真实代码接线**：`systems/save/load_game.rs`：
  - `load_game_system` ECS 系统：门 `AppMode::LoadGame`（0xe）→ `advance_load_game`
    状态推进 + kind 解码 + on_load 两遍回放（fence 终止）→ 写 `loading.tmp`（
    `write_loading_tmp`）→ 回放 MetaCommand 入 `MetaCommander::extend_script`（commander
    update case 0 会话加载链消费）→ 事件 `StartupRevealStarted`/`LoadingGameComplete`；
  - 经 `add_save_load_systems_impl` + run_game.rs 接入生产（仲裁批准）。
- **原侧验证**：0x14146d030 门控/状态字节 +9/kind 解码/两遍回放/fence 常量已 DELTA §5
  核为"真实实现"。
- **仍缺项**：`init_game_state_from_snapshot` 应用（0x1417a9c50 域应用）、
  `PostGameLoadCommand::Apply` 应用（FUN_1412e0b90）、`StartupRevealFinished` 发射方
  （宿主应用完加载后发）；`app_mode==0xe` 门由宿主设置 AppMode。

## 汇总

- **真实接线**：init_save_thread/spawn + save_game 节拍器备份生产 + load_game_system +
  ManualBackup 身份比对 + retention 删除 + sheep 节 producer 接线 + run_game.rs 生产接入。
- **仍缺项**：域采集器（capture/load 各域）、PostGameLoadCommand 应用、0x141721104
  jumptable、身份比对载荷结构 U。
- **原侧验证**：save_thread/save_game/load_game 主循环已 DELTA 核为"真实实现"（机器码 +
  RNE 常量全核实）；新增项均对 decomp 行号。
- **编译**：组内文件无编译错误；整 crate 被其它组在制改动阻塞（中央已协调，roof 域在制）。
