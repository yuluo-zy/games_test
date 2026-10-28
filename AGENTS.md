# Tiny Glade 逆向工具入口

本条仅适用于 Tiny Glade 原作逆向与 Rust 还原；其他项目沿用其目录规则。

## 目录入口（先选，不要递归遍历）

- 工作区总索引：[`README.md`](README.md)。
- 正式实现：`tiny-glade/`，入口 [`tiny-glade/README.md`](tiny-glade/README.md)。
- 逆向证据/骨架/验证工程：`reverse-engineering/tiny-glade/`，入口 [`reverse-engineering/tiny-glade/README.md`](reverse-engineering/tiny-glade/README.md)。
- 正式工程与 reconstruction 的差异：[`PORTING-CHECKLIST.md`](reverse-engineering/tiny-glade/reconstruction/PORTING-CHECKLIST.md)。

渐进式读取：先读 L0 索引，再按任务选一个 L1 入口；需要细节时读 L2 子索引（`evidence/README.md`、`skeleton/README.md`、`tracking/README.md`）；最后用 `rg` 定位到单个文件。不要从工作区顶层递归加载 `_pdb`、`_assembly`、`task-index/work`、`target/` 或大日志。

- 先读 [项目约束](reverse-engineering/tiny-glade/AGENTS.md) 与 [AI 工具使用说明](reverse-engineering/tiny-glade/docs/current/AI_REVERSE_TOOLS.md)。说明包含可直接运行的 MCP SDK 命令、JSON job、参数、worker 隔离和失败处理。
- 复用已分析数据库；默认 Ghidra/ReVa 只读。修改或 ReVa 脚本使用自己的唯一 worker。工具连接/测试不计作游戏业务恢复。
