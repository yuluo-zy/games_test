# game 工作区索引

本目录有两套东西：**正式 Rust 实现**和**逆向证据/验证工程**。先按任务选一个入口，不要从顶层遍历。

## 按任务选入口

| 任务 | 首先打开 |
|---|---|
| 改正式实现、编译、测试、看当前状态 | [`tiny-glade/README.md`](tiny-glade/README.md) |
| 查原版行为、PDB、地址、差分证据 | [`reverse-engineering/tiny-glade/README.md`](reverse-engineering/tiny-glade/README.md) |
| 查 reconstruction 与正式工程的差异/移植清单 | [`PORTING-CHECKLIST.md`](reverse-engineering/tiny-glade/reconstruction/PORTING-CHECKLIST.md) |
| 查主题证据与报告索引 | [`evidence/README.md`](reverse-engineering/tiny-glade/evidence/README.md) |
| 查可编译的逆向验证工程 | [`reconstruction/README.md`](reverse-engineering/tiny-glade/reconstruction/README.md) |
| 查原作安装与原始数据 | `A版本内容/`、`B版本内容/`、`C 版本内容最新/` |
| 查 Blender 素材库 | [`美术资源 Blender 版本/README.md`](美术资源%20Blender%20版本/README.md) |
| 查本地逆向工具 | [`tools/README.md`](tools/README.md)、`.agents/skills/` |

## 渐进式披露

按层读取，不要一次加载整个工作区：

| 层 | 内容 | 何时读 |
|---|---|---|
| **L0** | 本文件 + [`AGENTS.md`](AGENTS.md) | 每次开始任务 |
| **L1** | 正式工程或逆向域的 `README.md` | 选定任务方向后只读一个 |
| **L2** | `docs/README.md`、`evidence/README.md`、`skeleton/README.md`、`tracking/README.md` 等子索引 | 需要该领域细节时 |
| **L3** | 具体 `REPORT.md`、证据目录、单个源码文件 | 用 `rg` 定位后再读 |

## 不要先读

- `_pdb/*.jsonl.gz`、`_assembly`、`skeleton/task-index/work/**` 的原始 JSON/汇编
- `target/`、二进制、`*.pdb`
- 大日志和整份历史报告

先用 `rg` 限定目录和关键词，再读命中片段。

## 目录速览

```text
game/
├── tiny-glade/                 正式 Rust workspace（生产实现）
├── reverse-engineering/
│   └── tiny-glade/             逆向证据域（读它的 README 进入）
├── A版本内容/ B版本内容/ C 版本内容最新/   原作发行文件
├── 美术资源 Blender 版本/       Blender 素材库
├── tools/                      逆向工具与 SDK
└── .agents/skills/             工作区技能
```

## Git 提交范围

2026-10-25 在当前工作区建立新的 Git 仓库，保存正式 Rust 实现、逆向验证源码、分析结论、相关文档和脚本；历史提交号仅作来源记录，不代表本仓库拥有旧历史。

A/B/C 原版发行文件不提交。生成的 A/B/C 符号骨架、PDB/汇编导出、SQLite 索引、编译产物、原始性能日志、本机工具运行时和数据库、编辑器配置及 Blender 素材库保留在本地，具体排除规则见 `.gitignore`。分析文档中的本地证据引用仍可能依赖这些文件。

本次整理执行了 `python3 tiny-glade/tools/verify_inventory.py`，检查未通过：现有源码包含外部证据引用、尚未恢复的占位实现，且当前依赖锁存在不在 B 版锁中的依赖身份。本次仅保存当前状态，未修改游戏业务代码，也不将此提交视为编译或业务验收通过。
