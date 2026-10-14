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
