# Tiny Glade 原作逆向任务的持久上下文

仅当任务涉及 `D:/game/ljxsj_92385/Tiny Glade` 的逆向与 Rust 还原时，先读 `D:/game/reverse-engineering/tiny-glade/PROJECT_MEMORY.md`、`tracking/project.json` 和 `TASK_BACKLOG.md`。

- 研究原作发行文件/PDB/游戏自有逻辑；不得转去分析用户 `tiny-garden` demo。
- 优先恢复可编译 Rust 源码与类型/控制流；场景回放和 GPU 暂时后置。
- 依赖版本/来源以原作 `build-info/Cargo.lock` 为准。直接复用原依赖；不得逆向或重新实现 std、Bevy、glam、fastrand、half、serde 等通用库。
- 用户说明引擎是 Bevy 分叉；按此作为项目约束记录。已证实 Bevy family 0.16.0，具体分叉仓库/commit 尚未核实，不编造。
- 私有游戏模块与自有工具函数仍需恢复；仅对必要的引擎分叉改动检查差异，不重新还原整个引擎。
- 优先使用 REA/ReVa/RE-MCP 的函数、引用、反编译、类型和持久数据库能力；脚本用于批量编排与差分验证，不能把写脚本本身当作逆向交付。

这些规则只适用于上述逆向任务，不约束本目录中的其他工程。
