# session-SPEC — 完整 NextSession / GameSaveData / MetaCommand / SessionLoadAttempt 世界载入事务（C 世界，2026-10-15 第二轮）

> 历史分析结果，供同组实时复合 agent 直接落地。已实现边界见 `evidence/core-recovery-20261015/session/REPORT.md`（GladeType/SessionMutator/SessionMeta/ActiveSession/ImageFormationConfig 均已就位并经中央 fixture 复核）。
> 依赖 save 域的 `GameSaveData`（见 save-SPEC §2）。本域改 `resources/session.rs` 属组内文件；若牵动 save/load_game 文件先经中央仲裁。
> 证据源：`evidence/core-recovery-20261015/session/*.c|*.asm|*.json`、`public-session-names.txt`、`session-types.txt`、`startup-switch.c`、`new-session-ui.asm`、`session-save/ALGORITHMS.md`、sysres v2。

## 状态判定

- **可直接落地**：NextSession 160B owner 与 kind 判别；`create`/`ignore_snapshot`/`append_on_load`（两单态）；MetaCommand/SessionOnLoadCmd 条目布局；会话切换系统的 SystemParam 面（public-session-names 给出完整签名）。
- **需先取证**：NextSessionKind 变体 1（load_latest）的**生产者**（UI/会话流 vtable 卡点，见 ALGORITHMS §6）；SessionOnLoadCmd 各变体的精确载荷字段名；`load_game_session`（0x14166e7d0，78KB）的完整内联策略。

---

## 1. NextSession（160B / 0xa0B）owner

### 1.1 布局（O）

```
NextSession @0x00 .. 0xa0（160 字节）
  +0x00 Vec<SessionOnLoadCmd> {cap@+0x00, ptr@+0x08, len@+0x10}（cap-first）
  +0x18 kind（u64 niche）
  +0x20 uuid（16B SessionId）
  …
  +0x98 ignore_snapshot（bool）
```

- `NextSession::load 0x140b46920`：{**空 Vec cap=8**, kind=0x8000000000000002, uuid@+0x20, ignore_snapshot=false@+0x98}。**空 Vec 是 cap 0 + dangling ptr 8**（旧章节"capacity 8"错误）。
- **kind 五条 selector 路径**（O）：
  - `0`：空/启动不可达；
  - `1`：load_latest；
  - `2`：显式 SessionId；
  - `3`：预解析 SessionId + boxed `systems::save::save_game::GameSaveData`（240B，**owner 在 save 域**）；
  - 其余：SessionMeta96 + 32B 创建选项（create 载荷）。
- 解码（load_game 侧）：`decoded = kind ^ 0x8000000000000000; sel = min(decoded, 2)`（见 save-SPEC §2.4）。**fence 常量 0x8000000000000003**（append_on_load<MetaCommandFence> 条目、load_game MetaCommand 回放终止条件）。

### 1.2 `create` 0x140b46960（O）

- 签名：`create(out NextSession, meta_template96B, option32B)`。从 UI 构造 0x141ebb5ad 调用点（`new-session-ui.asm` 0x141ebb5ad → call 0x140b46960，rcx=&输出、rdx=&[rbp+0x1f0]、r8=&[rbp+0x270]）可见：
  - 96B 模板（rdx）+ 32B 尾部（r8）构造；kind@+0x18 = 模板首 qword；Vec 清空。
  - 24B 可选串/路径 + 字节旗标随模板传递。
- `ignore_snapshot 0x140b46a20`：写 +0x98 bool + memcpy 0xa0（builder 风格）。
- `loading_background_path 0x140b46a50` = save_folder_path()+"loading_background.jpg"（v1 名 screenshot_path 错位）。

### 1.3 `append_on_load` 两单态（O）

- **`append_on_load<[MetaCommand;3]> 0x1403b5150`**：把三个 56B MetaCommand → 一个 VecDeque → push **一个 32B SessionOnLoadCmd**。
- **`append_on_load<MetaCommandFence> 0x141328ac0`**：push **0x20B 条目 {u32 tag=0, u32 0x80000000, u64 payload}** + memcpy 0xa0。
- **MetaCommand 头语义（I）**：{0, 0x80000000}（@0x141328ac0 载荷区；ALGORITHMS §6 卡点保留）。
- **SessionOnLoadCmd（32B）**：on_load 命令；load_game 回放时条目为 4B tag 判别，tag==0x8000000000000003 → fence 终止。

### 1.4 待取证（session）

- SessionOnLoadCmd 各变体的**载荷字段名**（32B 总布局已见，字段 U）。
- NextSessionKind 变体 1（load_latest）的**生产者地址**（UI/会话流 vtable 间接调用）。
- `create` 的 24B 可选串/路径精确策略（字段 U，不编造）。

---

## 2. GameSaveData（240B）/ MetaCommand / SessionOnLoadCmd 派发

### 2.1 GameSaveData owner（save 域）

240B 结构，由 save_game 组装（~9 资源 +0x28..+0x100 密集字段，字段名 U）。load_game 的 `parse_saved_game_data 0x141470d00` 消费。**落地分工**：owner 属 save 域（见 save-SPEC §2）；session 域只持 boxed 引用（kind==3 路径）。

### 2.2 MetaCommand 派发（load_game 内，O）

见 save-SPEC §2.4：MetaCommand 回放（tag^0x8000000000000000 解码，>2 钳 1；变体 0 写 manager 状态+push 24B 事件；变体 1 → FUN_140b5a180 push（meta_commander.rs 路径 @0x142c5ae38）；变体 2+ → Arc drop）。PostGameLoadCommand 第二轮回放（SwitchAppMode 跳过 @0x142c642e0）。

### 2.3 SessionLoadAttempt / SessionMetaSource（startup_session）

- **`SessionLoadAttempt::from_session_id`**（行 79，内联）：由 session id 构造载入尝试；布局见 `startup_session.rs.md`：`*param_1+0x10` 空 → theme 默认 3；否则 attempt → theme byte @+6。
- **`SessionMetaSource`**：枚举类型（sysres/public-session-names，字段 U）。
- **`copy_dir_all<PathBuf,PathBuf> 0x141740c00`**：递归目录复制（std fs walk；"intro" 子目录跳过——名 5 字符 == "intro" 0x72746e69+o）。

### 2.4 待取证（session）

- SessionLoadAttempt/SessionMetaSource 的完整字段布局（theme byte@+6 已见；其余 U）。
- 会话切换系统中 NextSession 消费后到 load_game_session 的精确时序（I：见 §3/§4）。

---

## 3. 会话切换系统（O：SystemParam 面完整）

`public-session-names.txt` 第 9/11 条给出会话选择系统的 **SystemParam 签名**（这就是 `refresh_session_list` 或 SavedSessionList 消费系统）：
`Res<SavedSessionList>` + `Option<ResMut<ActiveSession>>` + `Res<GameProfile>` + `Res<AutoSaveState>` + `ResMut<NextSession>` + `ResMut<LoadGameManager>` + `ResMut<AppMode>` + `ResMut<GladeSettings>` + `ResMut<MetaCommander>` + `EventWriter<NewSessionStartedCmd>` + `EventWriter<SessionLoadedCmd>` + `EventWriter<DisplayCorruptedSaveFileWarning>` + `Commands`，并调用 `startup_session::load_game_session`。

`startup-switch.c`（0x14166e9x 区，switch uVar20）：
- case 0 → panic `internal error: entered unreachable code`（0x142c84625, 0x28 字）；
- case 1/2 → 继续/新会话：`is_session_ok_to_load 0x140a9fa80`（save_folder_path×2 + join×2 + File::create×4）→ 有效 → log "Latest game session " / "Continuing game session " → 会话 Vec push（8B 元素 cap-first grow，计数 @+0x40）→ 拼 `<会话>/history.json`（'history.json' 0x142c845e7, 12）；无效 → 走新会话路径。
- 事件 `NewSessionStartedCmd` / `SessionLoadedCmd`（`resources/session.rs` 类型，尚未在源码）；`load_completion_checker::DisplayCorruptedSaveFileWarning`（sysres v2：`check_load_completion 0x140a9f810`、`SessionLoadReadiness::is_ok 0x140833230`、`deny_load 0x140a9fa70`、`is_session_ok_to_load 0x140a9fa80`）。

### 3.1 待取证（session）

- case 1（继续会话）UI 选择的具体调用链（startup-switch.c 片段 + 0x1405a3316/0x1405a39c0 → NextSession::load 0x140b46920 的调用者身份）。

---

## 4. 完整 world load/commit 事务（load_game_session 0x14166e7d0）

`country_core::startup::startup_session::load_game_session`（78KB，O，`startup_session.rs.md`）：

1. 读 SessionLoadAttempt（§2.3）→ theme；
2. 主题 jump-table（0x142c84d24：uVar20 = niche ^ 0x8000…；case0 → `internal error: entered unreachable code` panic 行 40；case 1/2 → 继续/新会话）；
3. 继续会话：`FUN_140a9fa80` 判有效 → 日志 'Latest game session ' / 'Continuing game session ' → 会话 Vec push → 拼 history.json；
4. 新会话：'Starting a new game session' 日志 → `FUN_140c102e0` 生成 id → create_dir（panic 'Creating folder for game session' 行 32）→ 写 meta.json（panic 'Writing new session meta.json' 行 29；theme 小写 summer/olden/flowery/autumn/winter，串 @0x142c84cc0 区）→ 起始 build：demo flag 0x143634d90 ? 'assets/starting-builds/demo'(0x142c849cb) : 'assets/starting-builds/full'(0x142c849b0, 27) → 内联 copy_dir_all；
5. 内联 copy_dir_all：递归 walk（FUN_140d59e80/59f10/62e10/61e00）→ 条目 Vec（4xqword）→ "intro" 子目录跳过 → 排序（FUN_1413320d0 / FUN_140b4c390）→ 逐项复制；
6. 日志/串：'Loading a pre-parsed game session'（0x142c84798）、'should not have been consumed earlier'（0x142c84830）、'Failed to load session meta. Using default. Error: '（0x142c84c40）、session 文件族 meta.json/history.json/snapshots(0x142c848b8)/screenshot.jpg。

### 4.1 会话资源补全（可直接落地清单）

- `SavedSession` / `SavedSessionList` / `SessionFlowPlugin` / `NewSessionStartedCmd` / `SessionLoadedCmd` / `NextSessionKind` / `SessionOnLoadCmd` 类型（session-types.txt 已列权威名）。
- `refresh_session_list 0x140b46b00`：清空后 paths::saves() readdir 重建（可直接落地）。
- `delete_session 0x140b46d60`：deleted_saves()/uuid 标记文件 → rename 移动 → 两段删除；失败 log "Error deleting glade: "（@0x142aef5a0）。
- `SavedSession::loading_background_path`（§1.2）。

## 5. 真实消费者接线清单

| 生产者 | 消费者 | 状态 |
|---|---|---|
| UI 会话选择（0x141ebb5ad create / 0x1405a3316 load） | NextSession::create/load → kind 判别 | 未实现 NextSession |
| NextSession kind 解码 | load_game 0x14146d030（save 域） | 未接线 |
| 会话切换系统 | AutoSaveState + LoadGameManager + GC 构造（save 域） | 未接线 |
| load_game_session 0x14166e7d0 | SessionMeta::write_to_disk / try_load_from_path（已实现）| 未接线 |
| SessionLoadedCmd / NewSessionStartedCmd | 全域（music/sheep/wall 等读取 ActiveSession） | 事件未实现 |

## 6. 明确排除项

- 不重写 uuid/serde/bevy_ecs；直接复用。
- GameSaveData owner 在 save 域实现（本域只持引用）。
- `resources/session.rs` 为本组文件；改动 session.rs 若与 music（B 组读 ActiveSession）冲突需中央仲裁定序。
