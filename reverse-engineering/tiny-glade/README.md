# Tiny Glade 逆向工作区

本目录是 Tiny Glade 的逆向取证、源码骨架与还原工程工作区。它是**研究资产库**，不是完成品；当前开发进度在 Rust 工程的 [`reports/business/2026-10-23-module-recovery/STATUS.md`](../../tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md)。

## 按任务直达

| 任务 | 打开 |
|---|---|
| 当前状态 / 下一步 | [`STATUS.md`](../../tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md) |
| 模块、文件、证据位置 | [`docs/reference/MODULE_PROGRESS.md`](docs/reference/MODULE_PROGRESS.md) |
| 未恢复接口 | [`docs/current/UNRECOVERED_INTERFACES.md`](docs/current/UNRECOVERED_INTERFACES.md) |
| 某主题的原版证据 | [`evidence/README.md`](evidence/README.md) → `evidence/<topic>/REPORT.md` |
| PDB / 符号 / 结构 / 调用点 | [`skeleton/README.md`](skeleton/README.md) |
| 可编译的验证实现 | [`reconstruction/README.md`](reconstruction/README.md) |
| 正式工程差异与移植清单 | [`reconstruction/PORTING-CHECKLIST.md`](reconstruction/PORTING-CHECKLIST.md) |
| MCP / Ghidra / ReVa | [`docs/current/AI_REVERSE_TOOLS.md`](docs/current/AI_REVERSE_TOOLS.md) → [`tool-usage/README.md`](tool-usage/README.md) |
| 机器台账 | [`tracking/README.md`](tracking/README.md) |
| 脚本与工具 | [`scripts/README.md`](scripts/README.md) → [`tools/`](tools/) |

## 渐进式披露（按层读，不要一次加载）

| 层 | 读什么 | 何时 |
|---|---|---|
| **L0** | 本文件 + [`AGENTS.md`](AGENTS.md) | 每次开始 |
| **L1** | 上表选一个入口 | 选定任务后 |
| **L2** | `evidence/README.md`、`skeleton/README.md`、`tracking/README.md`、`scripts/README.md` 等子索引 | 需要领域细节 |
| **L3** | 单个 `REPORT.md`、证据文件、源码文件 | 用 `rg` 定位后 |

**不要先读**：`_pdb/*.jsonl.gz`、`_assembly`、`skeleton/task-index/work/**` 的原始 JSON/汇编、`target/`、大日志、整份历史报告。先用 `rg` 限定目录和关键词，再读命中片段。

## AI 阅读顺序

1. [`AGENTS.md`](AGENTS.md) — 工作规则、证据门槛、token/磁盘约束。
2. 当前状态：[STATUS.md](../../tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md)（唯一权威）。
3. 模块/接口：[`docs/reference/MODULE_PROGRESS.md`](docs/reference/MODULE_PROGRESS.md)（映射，进度列已过期）→ [`docs/current/INTERFACE_PROGRESS.md`](docs/current/INTERFACE_PROGRESS.md) → [`docs/current/UNRECOVERED_INTERFACES.md`](docs/current/UNRECOVERED_INTERFACES.md)。
4. 代码与骨架：[`reconstruction/README.md`](reconstruction/README.md) → [`skeleton/README.md`](skeleton/README.md)。
5. 工具：[`docs/current/AI_REVERSE_TOOLS.md`](docs/current/AI_REVERSE_TOOLS.md) → [`tool-usage/README.md`](tool-usage/README.md)。

不要整篇加载历史文档、`_pdb/*.jsonl.gz`、`_assembly`、大 JSON 或证据目录中的原始反汇编。先用 `rg` 定位。

## 目录地图

| 目录 | 用途 | AI 使用建议 |
|---|---|---|
| `docs/current/` | 当前模块/接口台账、版本基线与工具说明 | 默认入口 |
| `docs/reference/` | 接口分类、机制差异、基础库对照 | 按主题查阅 |
| `evidence/` | 主题证据目录与原始日志 | 先读各目录 `REPORT.md`/`README.md`；见 [`evidence/README.md`](evidence/README.md) |
| `skeleton/` | A/B/C 三版 PDB 源码骨架、`_pdb`、`task-index` | 查符号/结构/调用；见 [`skeleton/README.md`](skeleton/README.md) |
| `reconstruction/` | 可编译的 Rust 还原工程（68 MB 源码） | 构建与差分入口；见 [`reconstruction/README.md`](reconstruction/README.md) |
| `tracking/` | 机器可读台账（baseline、模块覆盖、依赖策略等） | 见 [`tracking/README.md`](tracking/README.md) |
| `scripts/` | MCP/反编译/差分/进度脚本 | 见 [`scripts/README.md`](scripts/README.md) |
| `tools/` | country-core 审计、render-pass、spv-diff、package-graph | 独立工具集 |
| `tool-usage/` | MCP job、结果与 schema | 见 [`tool-usage/README.md`](tool-usage/README.md) |
| `workspace/` | 本地二进制探针与临时反编译产物 | 本地使用，gitignore |

## 关键事实

- 三个发行版本：A `2024-09-23`、B `2025-10-08`、C `2026-09-23`；B 是当前机器码与源码验证基线，A/C 为互补证据。详见 [`docs/current/MULTI_BASELINE.md`](docs/current/MULTI_BASELINE.md)。
- 当前版本原始文件位于工作区上级的 `A版本内容/`、`B版本内容/`、`C 版本内容最新/`。
- `skeleton/A|B|C` 是 PDB 注释骨架，不可直接编译；`reconstruction/` 是可编译实现。二者互补，不互相替代。
- 文件数、函数数、测试数或报告一致**不等于**整游戏验收。状态口径见 `AGENTS.md` 与 `PROJECT_PROGRESS.md`。

## 常用检索

```bash
rg -n "关键词" docs/current docs/reference
rg -n "0x14075dc60|函数名" skeleton/task-index/work
rg -l "未验收|partial|unknown" evidence/*/REPORT.md
```

## 维护记录

- 2026-10-24：删除构建产物、临时目录、历史轮次/计划/检查点/任务派工与交付快照（39 GB → 10 GB）；保留 `evidence/`、`skeleton/`（含 `task-index/work` 唯一分析产出）、`reconstruction/`、`docs/current`、`docs/reference`、脚本与工具。迁移与删除清单见 [`docs/MIGRATION-2026-10-24.json`](docs/MIGRATION-2026-10-24.json)。
