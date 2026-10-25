# startup/startup_session.rs

> 文件作用：**游戏会话启动：起始 build 选择（full/demo/intro）、会话目录创建、meta.json 读写、历史/快照载入。**

函数总计 8：业务 2 / 内联无独立符号 3 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_session::load_game_session` | 0x14166e7d0 | 0 | **核心** `load_game_session`：起始 build（`assets/starting-builds/{full,demo}`+intro）、创建会话文件夹（`Creating folder for game session`）、读/写 `meta.json`（theme: Summer/Olden/Flowery/Autumn/Winter）、载入 `history.json`/`snapshots`/`screenshot.jpg`；`Continuing game session` 日志。78KB。 | **O** | decomp-0x14166e7d0.txt |
| `country_core::startup::startup_session::copy_dir_all<std::path::PathBuf,std::path::PathBuf>` | 0x141740c00 | 444 | `copy_dir_all<PathBuf>`：递归复制目录（std fs walk）。 | **O** | decomp-0x141740c00.txt |

## 内联/无独立符号函数

- `country_core::startup::startup_session::MaybeExists::get`（行 39）— **I**：`MaybeExists::get` — 路径存在性查询（内联）。
- `country_core::startup::startup_session::SessionLoadAttempt::from_session_id`（行 79）— **I**：`SessionLoadAttempt::from_session_id` — 由 session id 构造载入尝试（内联）。
- `country_core::startup::startup_session::load_session_meta`（行 438）— **I**：`load_session_meta` — 载入 meta.json（内联）。

## 派生胶水

- 3 项：closure$0, closure$1, closure$2 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **load_game_session 0x14166e7d0（78KB，O）** 完整流程：
  1. 读 SessionLoadAttempt（*param_1+0x10 空 → theme 默认 3；否则 attempt → theme byte @+6）
  2. 主题 jump-table（0x142c84d24：uVar20 = niche ^ 0x8000…；case0 → `internal error: entered
     unreachable code` panic 行 40；case 1/2 → 继续/新会话路径）
  3. 继续会话：FUN_140a9fa80 判有效 → 日志 'Latest game session ' / 否则 'Continuing game session '
     → 会话 Vec push（cap-first grow FUN_140562870，8B 元素，计数 @+0x40）→ 拼 `<会话>/history.json`
     （'history.json' 0x142c845e7, 12）
  4. 新会话：'Starting a new game session' 日志 → FUN_140c102e0 生成会话 id → create_dir
     （panic 'Creating folder for game session' 行 32）→ 写 meta.json（panic 'Writing new session
     meta.json' 行 29；theme 小写 summer/olden/flowery/autumn/winter，串 0x142c84cc0 区）
     → 起始 build：demo flag 0x143634d90 ? 'assets/starting-builds/demo'(0x142c849cb)
     : 'assets/starting-builds/full'(0x142c849b0, 27) → 内联 copy_dir_all
  5. 内联 copy_dir_all：递归 walk（FUN_140d59e80/59f10/62e10/61e00）→ 条目 Vec（4xqword，grow
     FUN_140562c30）→ **"intro" 子目录跳过**（名 5 字符 == "intro" 0x72746e69+o）→ 排序
     （FUN_1413320d0 / FUN_140b4c390）→ 逐项复制
  6. 日志/串：'Loading a pre-parsed game session'（0x142c84798）、'should not have been consumed
     earlier'（0x142c84830）、'Failed to load session meta. Using default. Error: '（0x142c84c40）、
     'Error deleting glade: '（0x142c84cc0 区）、session 文件族 meta.json/history.json/
     snapshots(0x142c848b8)/screenshot.jpg
- **copy_dir_all 0x141740c00（O）**：递归目录复制（std fs walk；&str 变体内联于 load_game_session）。
