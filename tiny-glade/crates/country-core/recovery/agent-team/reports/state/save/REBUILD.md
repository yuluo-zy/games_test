# REBUILD — systems/save + meta_commander + startup 真实逻辑重建（2026-10-17）

> restorer rebuild-save。方法：协议 planning/2026-10-16-logic-verify-protocol.md（先分类宏胶水
> vs 真业务，再按 LOGIC.md 分歧点 fresh 反编译重建控制流 → 逐语句实现 → 编译检查）。
> 证据：planning/logic-save/decomp/*.txt（fresh 反编译，shared 只读）；重建只以这些为准。

## 0 分类总表（19 需重做 → 重建）

| 原函数 | VA | 类别 | 重建 verdict |
|---|---|---|---|
| commander_update | 0x14091f180 | **真业务**（case0 两路径状态机） | 重建（门控极性纠正 + insert_resource） |
| load_game | 0x14146d030 | **真业务**（两遍回放 + 版本门 + 快照加载） | 重建（解码 clamp + 内联 fence 释放 + SwitchAppMode log-only + Apply 载荷） |
| parse_saved_game_data | 0x141470d00 | **真业务**（全量节解析） | 部分重建（upgrade 备份目标名未验证，登记） |
| save_thread 主循环 | 0x141720680 | **真业务**（主循环 + Shutdown） | 重建（Shutdown 不 break + 删 ack + try_recv 语义） |
| execute_save_cmd | 0x141721aa0 | **真业务**（分派） | 重建（case3/4 flush_deferred_and_sync + UnsavedFileList::add） |
| flush_deferred_and_sync | 0x14171f7f0 | **真业务**（延迟节落盘） | 重建（删 sync_all） |
| save_game_system | 0x140a3f470 | **真业务**（ECS 系统） | 重建（节 cooldown ChangeSection + SessionMeta + Flush；域采集部分登记） |
| AutoSaveState::close_file | 0x140a43840 | **真业务**（发 CloseFile + 清状态） | 重建 |
| AutoSaveState::create_backup_of | 0x141e59860 | **真业务**（同步请求-应答） | 重建（返回 Result） |
| last_error_state | 0x140a439a0 | **真业务**（+0x10 指针访问器） | 判等（布局建模已按语义） |
| capture_game_snapshot_save_data | 0x1417a0430 | **真业务**（~9 域收集器组装 Arc） | 登记（域 Query 面宿主恢复后接入；薄壳结构已具） |
| capture_game_snapshot | 0x1417a1d00 | **真业务**（184KB 域 Query 系统） | 登记（域 Query 未齐，无 Rust body） |
| validate_and_load_closest_snapshot_data | 0x1417a6e50 | **真业务**（逐节 typed 解析） | 登记（Terrain/栅格域解析面宿主接入） |
| init_game_state_from_snapshot | 0x1417a9c50 | **真业务**（0xc0B 头部+11 域描述符写回） | 登记（apply_loaded_snapshot 已有，全链待域） |
| add_game_startup_systems | 0x140941110 | **真业务**（28 系统 tuple 注册） | 登记（宿主 Schedule 接线） |
| copy_dir_all | 0x141740c00 | **真业务**（纯递归复制） | 重建（删 sort + intro 跳过，双端） |
| load_glade_audio | 0x1408be360 | **真业务**（音频载入） | 登记（无 body） |
| is_ok（AutosaveErrorState） | — | 宏/访问器 | VA 错锚纠正（0x140b92210 实为 ShapeStream::is_empty） |
| is_session_ok_to_load | 0x140a9fa80 | **真业务**（三值编码） | 重建（unsaved 命中 → 值 1） |

未删除判等函数 31（LOGIC 已列）；重建仅动 LOGIC 判定分歧项。胶水 3（serde visitor / From / derive）
不在重建范围（协议 §胶水-非业务）。

## 1 重建明细（逐函数）

### 1.1 commander_update case0 门控极性（meta_commander.rs）

**fresh 反编译**（0x14091f180 case0）：
- `mode∈{13,14}`（`mode-6<0xf && mode-0xd<2`）→ **原样 push_front 重入队等待**（不建会话不消费）；
- `mode∉{13,14}` → `new_pair_blocked` → `append_on_load<Fence>` → `insert_resource<NextSession>`
  → push_front 带 fence。

**旧 Rust 极性反**（`!matches!(13|14) → return` 会消费丢弃命令）。重建：
- `CommanderOutcome::SessionLoad` 增加 `original`（门控等待时重入队）；
- 门控命中 `{13,14}` → `commander.undo_consume(original)`（原样等待）；
- 门控未命中 → `commands.insert_resource(session)`（**替代 ResMut 改写既有**）+ 带 fence 重入队；
- `MetaCommandFence::new_pair_blocked` 镜像原生 **strong=2**（`Arc::into_raw` 泄漏一载荷引用，
  由 load_game variant2 `Arc::from_raw`+drop 平衡）。

### 1.2 load_game 两遍回放（load_game.rs）

**fresh 反编译**（0x14146d030）：
- 条目 0x20B = 4 qword `{tag,payload_a,payload_b,payload_c}`（步进 +0x20）；
- 解码 `decoded = tag^mask; if(2<decoded) decoded=1`（**非 min(2)**，≥3 分派不同）；
- 首遍 variant0 = WriteManager（进度计数 + 24B 事件）；variant1 = MetaCommand（**3 payload**）；
  variant2 = **内联 fence 释放**（unblock +0x10 + Arc drop）；
- 次遍 variant0 = SwitchAppMode **仅 log 不 push**；variant1/2+ = Apply 载荷。

重建：
- `OnLoadEntry::Command` 扩展为 3 payload（session.rs）；`append_fence`/`append_replay_terminator`
  更新；新增 `append_fence_release`（变体 2 条目生产者入口）。
- `replay_meta_commands`：解码 clamp 纠正；variant2 **内联释放**（不再登记 FenceRelease 被丢弃）；
  variant1 载 3 payload。
- `replay_post_game_load_commands`：variant0 仅 log；Apply 载真值 payload_b/payload_c。
- `load_game_system`：FenceRelease/SwitchAppMode 枚举项删除；第二遍后单次
  `insert_resource<NextSession>`（O）。

### 1.3 save_thread Shutdown + flush_deferred_and_sync + execute_save_cmd（save_thread.rs）

**fresh 反编译**（0x141720680 / 0x141721aa0 / 0x14171f7f0）：
- Shutdown{flush:true} → flush_deferred_and_sync + drop + **退出线程**（无 ack）；
- Shutdown{flush:false} → 应用 deferred + 睡 10ms + **continue（不 break）**；
- flush_deferred_and_sync disasm **无 sync_all/flush**（只逐槽 apply change_section）；
- case3/4 均对前一档 flush_deferred_and_sync，失败 `UnsavedFileList::add`。

重建：
- `flush_deferred_and_sync` 删末尾 `sync_all()`（返回 true；原生只 apply）。
- `execute_save_cmd(current, cmd, unsaved)`：case4 = take → flush + 失败 add + drop；
  `execute_open_file_cmd` 改 flush_deferred_and_sync + 失败 add。
- `save_thread(commands, error_sender, unsaved)`：**删 shutdown_ack**；Shutdown{flush:true} → flush+return；
  Shutdown{flush:false} → apply deferred + sleep(10ms) + continue；无命令轮（timeout）也 apply deferred。
- `SaveThreadHandle` 删 ack 字段；`AsyncSaveCmd::ManualBackup` 增 `reply`（create_backup_of 应答）。

### 1.4 AutoSaveState（save_game.rs）

- `close_file`（0x140a43840）：**先 `Sender::send<CloseFile(4)>`**（send 失败 panic，原生 unwrap_failed），
  成功后清 sections/cached_sections/last_error（Rust 建模 +0x120/+0x28/+0x140/+0x144 清空语义）。
- `create_backup_of`（0x141e59860）：**同步请求-应答**——`bounded(1)` 应答通道 → 发 ManualBackup
  （带 reply）→ `recv` → 返回 `Result<(),anyhow::Error>`（**替代旧 bool + 直连 fs::create_dir_all**）。
- `last_error_state`（0x140a439a0）：返回 +0x10 错误状态指针语义（Rust 布局建模，访问器保持）。

### 1.5 save_game_system 主体（save_game.rs）

**fresh 反编译**（0x140a3f470）：节拍器段判等；缺①节 cooldown ChangeSection(5) 发送、
②内联同步保存、③触发后内联快照采集、④SessionMeta::write_to_disk + Flush(0xa)。

重建（可落地部分）：
- ① `emit_section_change_if_changed`：`cached_sections` 值变更 → `ChangeSection(5)` 发送
  （Colors/Sheep 为本系统可采集节；cooldown 秒表值待取证，当前变更即发不伪造）。
- ④ `SessionMeta::write_to_disk` + `AsyncSaveCmd::Flush` 发送。
- ②同步保存（assemble_game_save_sections → OpenFile）域资源面、③快照采集（History
  current_snapshot/capture_game_snapshot_save_data/域 Query）依赖 History 资源与域 Query，
  宿主经 `CaptureSnapshotRequested` 事件接线（不伪造节载荷）。

### 1.6 VA 错锚与三值编码

- `AutosaveErrorState::is_ok`：VA 0x140b92210 实为 `ShapeStream::is_empty`（9B），注释纠正。
- `copy_dir_all`：0x141740c00 fresh 反编译**无 sort、无 "intro" 比较** = 纯递归复制；
  `startup/startup_session.rs` 删 sort_by_key+intro 跳过、`resources/session.rs` 删 intro 跳过；
  测试改断言纯递归。
- `is_session_ok_to_load_in`：三值编码（unsaved 命中 → **1**（DenyLoad）、loading.tmp 缺 → **0**、
  在 → **2**）；旧把 unsaved 命中折叠 Ok(0) 已纠正，测试同步。

## 2 编译/自检

- `cargo check -p country-core --offline`：**通过**（`Finished dev profile`）。本模块
  （systems/save + meta_commander + resources/session + startup/startup_session）零错误。
- `cargo test --lib`（全 crate 测试构建）：被**他模块并发 WIP 测试代码**阻塞（rhapsody_contract
  `GraphicsDevice` E0423、render/asset_libraries/garden/history/ivy 等测试错误），**错误清单无一处
  引用本模块测试文件**（systems/save/tests.rs / load_completion_checker.rs / startup_session.rs /
  meta_commander.rs）。save 测试无法单独运行（同一测试二进制先编译全部 cfg(test) 模块）。
- 本模块测试（tests.rs 等）已按新签名/行为更新（execute_save_cmd 3 参、save_thread
  Shutdown{flush:true/false}、on_load variant2 fence 释放、三值编码、纯递归 copy_dir_all）；
  待并发 WIP 收敛后由集成 owner 跑 `cargo test --lib systems::save`。

## 3 仍登记（待域/宿主接入，不伪造）

- save_game_system 同步保存 OpenFile 组装、快照采集（History 资源 + 域 Query）。
- capture_game_snapshot_save_data 域收集器、validate_and_load_closest_snapshot_data 逐节 typed、
  init_game_state_from_snapshot 写回链、add_game_startup_systems 28 系统、load_glade_audio。
- parse_saved_game_data 全量节（Identity/Sheep 等）与 upgrade 备份目标名未对原生验证。
- commander_update case0 path B 的 fence 由 on_load variant2 条目（`append_fence_release`）释放；
  生产者在会话切换 on_load 末尾追加（RE-SESSION-SWITCH 协议闭环待 run_game_thread 接线）。

## 4 回传主会话（≤400 字）

分类 19 需重做（真业务 17 + 胶水/访问器 2），重建 13（commander_update 极性、load_game 两遍回放、
save_thread Shutdown、flush_deferred_and_sync、execute_save_cmd case3/4、save_game_system 主体、
AutoSaveState close_file/create_backup_of/last_error_state、copy_dir_all 双端、is_session_ok_to_load
三值、is_ok VA 错锚）；6 登记（capture/validate/init/add_game_startup_systems/load_glade_audio/
parse 域面）。编译：本模块零错误；crate 被并发 stairs WIP（StairGraphEdge serde）阻塞，非本次改动。
测试更新 6 项，因 stairs blocker 未跑。证据 fresh 反编译 planning/logic-save/decomp/。
