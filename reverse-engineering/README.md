# Reverse Engineering 索引

本目录只保存 Tiny Glade 的逆向证据、PDB 骨架与验证工程；正式实现不在这里。

## 唯一入口

[`tiny-glade/README.md`](tiny-glade/README.md) — 按任务列出的阅读顺序、目录地图、token 约束。

## 按任务直达

| 任务 | 打开 |
|---|---|
| 当前状态 | [`../tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md`](../tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md) |
| 模块/文件映射 | [`tiny-glade/docs/reference/MODULE_PROGRESS.md`](tiny-glade/docs/reference/MODULE_PROGRESS.md) |
| 未恢复接口 | [`tiny-glade/docs/current/UNRECOVERED_INTERFACES.md`](tiny-glade/docs/current/UNRECOVERED_INTERFACES.md) |
| 原版证据 | [`tiny-glade/evidence/README.md`](tiny-glade/evidence/README.md) |
| PDB / 符号 / 结构 | [`tiny-glade/skeleton/README.md`](tiny-glade/skeleton/README.md) |
| 可编译验证实现 | [`tiny-glade/reconstruction/README.md`](tiny-glade/reconstruction/README.md) |
| 正式工程差异/移植 | [`tiny-glade/reconstruction/PORTING-CHECKLIST.md`](tiny-glade/reconstruction/PORTING-CHECKLIST.md) |
| MCP/Ghidra 工具 | [`tiny-glade/docs/current/AI_REVERSE_TOOLS.md`](tiny-glade/docs/current/AI_REVERSE_TOOLS.md) |
| 机器台账 | [`tiny-glade/tracking/README.md`](tiny-glade/tracking/README.md) |
| 脚本 | [`tiny-glade/scripts/README.md`](tiny-glade/scripts/README.md) |

## 渐进式读取

1. 先读 [`tiny-glade/README.md`](tiny-glade/README.md)。
2. 再按上表只打开一个子索引。
3. 最后用 `rg` 定位到具体 `REPORT.md` 或源码，按片段读取。

不要从本目录顶层递归加载 `_pdb`、`_assembly`、`task-index/work` 的原始 JSON/汇编或大日志。
