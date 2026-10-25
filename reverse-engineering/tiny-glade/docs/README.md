# 文档索引

> 状态分层：`current/` 是当前口径；`reference/` 是按主题查阅的参考材料。历史轮次、计划与任务记录已于 2026-10-24 删除。

## 新鲜度与权威顺序

**当前状态永远以 Rust 工程的 [`reports/business/2026-10-23-module-recovery/STATUS.md`](../../../tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md) 为准。**

| 文档 | 实质内容截止 | 定位 |
|---|---|---|
| `reference/MODULE_PROGRESS.md` | 2026-10-13 | 模块/文件映射与证据定位（进度列已过期） |
| `current/INTERFACE_PROGRESS.md` | 2026-10-14 | 共享基础接口进度 |
| `current/UNRECOVERED_INTERFACES.md` | 2026-10-13 台账 | 未恢复接口清单 |
| `current/MULTI_BASELINE.md` | 长期有效 | A/B/C 身份与哈希 |
| `current/THREE_VERSION_RUST_PLAN.md` | 长期有效 | 三版本还原计划 |
| `reference/FOUNDATION_BUSINESS_COMPARISON.md` | 2026-10-14 | 补全前差异快照，仅作计数参考 |

## 归并建议

- **不建议合并**：`MODULE_PROGRESS`（模块映射）、`INTERFACE_PROGRESS`（接口进度）、`UNRECOVERED_INTERFACES`（未恢复清单）是不同检索维度，合并会形成超大文档并降低定位效率。
- **已移入 reference（2026-10-24）**：`MODULE_PROGRESS` 仅保留模块/文件映射与证据定位，进度列不再作为当前状态。
- **已删除（2026-10-24）**：历史里程碑 `PROJECT_PROGRESS`、历史任务 `TASK_BACKLOG`、旧计划/判断与长期记忆档案已删除；当前状态只以 STATUS 为准。
- **已合并（2026-10-24）**：`SHARED_FOUNDATION_README.md` 的构建/测试内容已并入 `FOUNDATION_BUSINESS_FUNCTIONS.md` 的“构建与测试”段。
- **保留双视图**：`INTERFACE_PROGRESS` 与 `UNRECOVERED_INTERFACES` 有重叠但列不同，已互相链接，不重复维护完整表。

## current/ — 合同、清单与工具

| 文件 | 内容 |
|---|---|
| `INTERFACE_PROGRESS.md` | 共享基础库接口进度与证据链接 |
| `UNRECOVERED_INTERFACES.md` | 未恢复接口的源码/原因/审计缺口 |
| `FOUNDATION_BUSINESS_FUNCTIONS.md` | 基础库业务功能清单 |
| `MULTI_BASELINE.md` | A/B/C 三版身份、哈希与差异 |
| `THREE_VERSION_RUST_PLAN.md` | 三版本还原计划与验收边界 |
| `AI_REVERSE_TOOLS.md` | MCP/Ghidra/ReVa 工具入口与 job 协议 |
| `DEPENDENCY_FILTERING.md` | 依赖归属与筛除规则 |

## reference/ — 参考与对照

| 文件 | 内容 |
|---|---|
| `MODULE_PROGRESS.md` | 逐模块/文件映射与证据定位（进度列止于 2026-10-13，不作当前状态） |
| `INTERFACE_CLASSIFICATION.md` | 接口分类与归属判断 |
| `MECHANISM_DIFF.md` | 原作机制与实现差异 |
| `FOUNDATION_BUSINESS_COMPARISON.md` | 基础库业务对照（2026-10-14 快照） |

## 迁移

2026-10-24 的目录迁移与删除清单见 [`MIGRATION-2026-10-24.json`](MIGRATION-2026-10-24.json)。
