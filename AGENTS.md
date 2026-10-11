# Tiny Glade 原作逆向任务的持久上下文

仅当任务涉及 `D:/game/ljxsj_92385/Tiny Glade` 的逆向与 Rust 还原时，先读 `D:/game/reverse-engineering/tiny-glade/PROJECT_MEMORY.md`、`tracking/project.json` 和 `TASK_BACKLOG.md`。

- 研究原作发行文件/PDB/游戏自有逻辑；不得转去分析用户 `tiny-garden` demo。
- 优先恢复可编译 Rust 源码与类型/控制流；场景回放和 GPU 暂时后置。
- 依赖版本/来源以原作 `build-info/Cargo.lock` 为准。直接复用原依赖；不得逆向或重新实现 std、Bevy、glam、fastrand、half、serde 等通用库。
- 用户说明引擎是 Bevy 分叉；按此作为项目约束记录。已证实 Bevy family 0.16.0，具体分叉仓库/commit 尚未核实，不编造。
- 私有游戏模块与自有工具函数仍需恢复；仅对必要的引擎分叉改动检查差异，不重新还原整个引擎。
- 优先使用 REA/ReVa/RE-MCP 的函数、引用、反编译、类型和持久数据库能力；脚本用于批量编排与差分验证，不能把写脚本本身当作逆向交付。

这些规则只适用于上述逆向任务，不约束本目录中的其他工程。

三版 Tiny Glade 的逆向统一遵守 `reverse-engineering/tiny-glade/DEPENDENCY_FILTERING.md`：先查 Cargo/PDB 来源与定义模块，排除已确认库实现，恢复游戏调用策略。不要把含 Wall/Roof 的通用库泛型实例当作游戏业务，也不要把全部 Cargo/local 包一律当作游戏私有。优先完成建筑系统的权威状态、生命周期、几何、附着及历史联接。

## Token 效率约定

本仓库的逆向任务单次请求固定开销很大，遵守以下约定：

- `rea` MCP 经 `~/.codex/mcp-tools/toolgate` 代理，只暴露 29 个精选工具；需要更多工具时编辑 `~/.codex/mcp-tools/toolgate/rea.json`，不要把配置改回直连原 server。
- 反编译按需取用：先用 `procedure_info`/`search_procedures` 定位，再对目标函数 `batch_decompile`；不要整段读取大型函数的完整汇编。
- 大文件与二进制不走普通文件读取；用 REA 的 `open_binary`/`read_bytes` 带范围查询。
- 需要批量命令行处理时用 `rea` CLI 的 `--token-limit`/`--token-count`/`--filter-output`。
