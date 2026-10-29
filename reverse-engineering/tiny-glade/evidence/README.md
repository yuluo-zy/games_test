# 证据目录索引

本目录保存 113 个主题证据目录与原始运行日志。**证据不等于实现，也不等于验收**；每个目录的 `REPORT.md`、`README.md` 或 `findings.md` 说明其范围、命令、输入身份与验证边界。

## 主要入口

| 主题 | 入口 |
|---|---|
| toolbar / 主链轮次 | [`round11/REPORT.md`](round11/REPORT.md)、[`round12/REPORT.md`](round12/REPORT.md) |
| 建筑集成 | [`building-integrate/findings.md`](building-integrate/findings.md) |
| 渲染 / rhapsody | [`render-recovery/RHAPSODY_PROGRESS.md`](render-recovery/RHAPSODY_PROGRESS.md)、[`render-recovery/findings.md`](render-recovery/findings.md) |
| 基础库复核 | [`foundation-review-fixes-2026-10-14/REPORT.md`](foundation-review-fixes-2026-10-14/REPORT.md) |
| Shader 来源审计 | [`shader-origin-audit-2026-10-14/REPORT.md`](shader-origin-audit-2026-10-14/REPORT.md) |
| Neo4j 汇编图谱、真实链查询与性能验收 | [`code-map-2026-10-28/REPORT.md`](code-map-2026-10-28/REPORT.md) |

## 主题分组

| 前缀 | 内容 |
|---|---|
| `round9/`–`round13-frame-graph-gpu/` | 按轮次保存的构建、GPU 与链路验证 |
| `building-*` | 墙体、屋顶、门窗、楼梯、装饰物、生命周期等建筑子系统 |
| `render-*`、`shader-*` | 渲染架构、Pass、Shader 来源与构建 |
| `country-core-*` | country-core 业务、资源与深挖审计 |
| `foundation-*`、`shared-foundation*` | utils / asset / asset-pipe 基础库 |
| `multi-baseline/`、`multi-version/` | A/B/C 三版身份与差分 |
| `toolchain*`、`tool-usage` 相关 | MCP、Ghidra/ReVa、工具链复验 |
| `progress-*`（已迁至 `../rounds/`） | 2026-10-20/21 两轮推进记录 |

## 使用建议

- 先读目录内的 `REPORT.md` 或 `README.md`，再按其中链接定位 JSON/日志。
- 用 `rg` 限目录、限文件类型搜索；不要整篇加载反汇编、`*.jsonl.gz` 或数 MB 的运行日志。
- 根目录的 `round11-*.log`、`round12-*.log` 是旧 Windows 运行快照，含失败/截断记录；当前结论以各 `round*/REPORT.md` 为准。
- 部分历史报告中的绝对路径（`games_test/`、`D:/game/`、旧发行目录）是取证 provenance，不代表当前路径。

## 已清理项（2026-10-24）

- `shader-gap-investigation-2026-10-13/corpus-native-recipes/` 与 `ubo-prefix-recheck/`：最终结果在 `corpus-final-native/`。
- `native-pack-cache/`、`round11/rebuilt-index/`：零引用缓存/生成物。
- `building-integrate/.../stage4-frozen/` 的重复 B-*.json：保留 `summary.json`。
- `toolbar-origin-2026-10-16/procs.tsv` 改为指向 `country-core-audit/data/procs.tsv` 的相对符号链接。
