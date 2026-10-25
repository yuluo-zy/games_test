# reports 索引

本目录按**领域**归档 Tiny Glade 还原过程中的审查、重构、恢复与验收证据。每个子目录是一份独立报告，日期取该轮工作开始的日期。

## 结构

| 组 | 内容 |
|---|---|
| `foundation/` | 基础库（asset、asset-pipe、paths、lang-lib、motomoto、utils）、资产库、Vulkan 加载器 |
| `building/` | 建筑运行时、geometry 与墙砖生命周期 |
| `systems/` | 树/水/音频/clutter 等系统与资源恢复 |
| `runtime/` | 主链、主循环、渲染线程/性能、启动与 Logo |
| `business/` | business 模块恢复、toolbar 证据、模块进度审计 |
| `ui-input/` | input、GPU 文本、主题、tg-iced / tg-history |
| `_archive/` | 已退出主索引的早期迭代与原始转储，见 [\_archive/README.md](_archive/README.md) |

## foundation

| 报告 | 内容 | 入口 |
|---|---|---|
| `2026-10-17-foundation-refactor` | 五个基础包重构与验收（首轮） | [README](foundation/2026-10-17-foundation-refactor/README.md) |
| `2026-10-17-foundation-rereview` | 五包第二轮完整源码复审 | [README](foundation/2026-10-17-foundation-rereview/README.md) |
| `2026-10-17-source-review` | 五个基础包源码审查（修复前快照） | [README](foundation/2026-10-17-source-review/README.md) |
| `2026-10-17-utils-refactor` | utils 重构验证日志与性能探针 | [clippy.log](foundation/2026-10-17-utils-refactor/clippy.log) |
| `2026-10-18-asset-libraries-review` | 资产库首轮审查探针与单测记录 | [verification.json](foundation/2026-10-18-asset-libraries-review/verification.json) |
| `2026-10-18-asset-libraries-refactor` | 资产库工程化与修复 | [README](foundation/2026-10-18-asset-libraries-refactor/README.md) |
| `2026-10-18-asset-libraries-rereview` | 资产库第二轮审查与修复 | [README](foundation/2026-10-18-asset-libraries-rereview/README.md) |
| `2026-10-18-country-core-utils-refactor` | country-core utils 修复与工程化重构 | [README](foundation/2026-10-18-country-core-utils-refactor/README.md) |
| `2026-10-18-country-core-utils-review` | utils 分析与审查证据 | [README](foundation/2026-10-18-country-core-utils-review/README.md) |
| `2026-10-18-country-core-utils-rereview` | utils 重构后再次审查 | [README](foundation/2026-10-18-country-core-utils-rereview/README.md) |
| `2026-10-18-country-core-utils-review-fixes` | 再次审查问题修复 | [README](foundation/2026-10-18-country-core-utils-review-fixes/README.md) |
| `2026-10-18-utils-correctness` | utils 第二轮修复验证 | [README](foundation/2026-10-18-utils-correctness/README.md) |
| `2026-10-18-vulkan-loader-config` | 本机 Vulkan 加载配置修复与严格验证 | [README](foundation/2026-10-18-vulkan-loader-config/README.md) |

## building

| 报告 | 内容 | 入口 |
|---|---|---|
| `2026-10-17-building-foundation-review` | 建筑运行时与基础包源码审查 | [README](building/2026-10-17-building-foundation-review/README.md) |
| `2026-10-18-building-foundation-review` | 三包独立复审与工程化改造 | [README](building/2026-10-18-building-foundation-review/README.md) |
| `2026-10-18-geometry-refactor` | geometry 工程化重构 | [README](building/2026-10-18-geometry-refactor/README.md) |
| `2026-10-18-geometry-review` | geometry 源码审查（重构前快照） | [README](building/2026-10-18-geometry-review/README.md) |
| `2026-10-18-geometry-rereview` | geometry 重构后复审 | [README](building/2026-10-18-geometry-rereview/README.md) |
| `2026-10-18-geometry-lifecycle-fix` | 墙砖缓冲生命周期修复 | [README](building/2026-10-18-geometry-lifecycle-fix/README.md) |

## systems

| 报告 | 内容 | 入口 |
|---|---|---|
| `2026-10-18-core-systems-resources-recovery` | 树、水、音频、clutter 等 systems/resources 业务恢复与复核 | [README](systems/2026-10-18-core-systems-resources-recovery/README.md) |

## runtime

| 报告 | 内容 | 入口 |
|---|---|---|
| `2026-10-18-core-rhapsody-bridge` | country-core ↔ rhapsody 调用调查与接线 | [README](runtime/2026-10-18-core-rhapsody-bridge/README.md) |
| `2026-10-18-main-chain-recovery` | 主链恢复分析、实现与渲染边界 | [ANALYSIS](runtime/2026-10-18-main-chain-recovery/ANALYSIS.md) |
| `2026-10-18-startup-refactor` | startup 工程修复与重构 | [README](runtime/2026-10-18-startup-refactor/README.md) |
| `2026-10-18-startup-review` | startup 分析与审查证据 | [README](runtime/2026-10-18-startup-review/README.md) |
| `2026-10-19-main-loop-render-recovery` | 主循环与渲染恢复（多批次、含当前审计） | [CORE-CHAIN-CONTINUATION](runtime/2026-10-19-main-loop-render-recovery/CORE-CHAIN-CONTINUATION.md) |
| `2026-10-21-logo-startup-repair` | Logo 与启动链修复、验证 | [VALIDATION](runtime/2026-10-21-logo-startup-repair/VALIDATION.md) |
| `2026-10-21-render-perf-architecture` | 渲染性能重构文档集 | [README](runtime/2026-10-21-render-perf-architecture/README.md) |
| `2026-10-22-render-thread` | 渲染线程架构与并行改造 | [ARCHITECTURE](runtime/2026-10-22-render-thread/ARCHITECTURE.md) |

## business

| 报告 | 内容 | 入口 |
|---|---|---|
| `2026-10-21-business-module-recovery` | 业务模块恢复（toolbar / taffy / UI 交互等） | [WORKING-STATUS](business/2026-10-21-business-module-recovery/WORKING-STATUS.md) |
| `2026-10-21-module-progress-audit` | 各模块恢复进度审计（按模块分 `DETAIL-*.md`） | [DETAIL-building](business/2026-10-21-module-progress-audit/DETAIL-building.md) |
| `2026-10-23-business-module-recovery-toolbar` | toolbar 原生导航独立证据 | [README](business/2026-10-23-business-module-recovery-toolbar/README.md) |
| `2026-10-23-module-recovery` | signifier / audio 等模块恢复 | [STATUS](business/2026-10-23-module-recovery/STATUS.md) |

## ui-input

| 报告 | 内容 | 入口 |
|---|---|---|
| `2026-10-17-theme-ui-refactor` | 主题与辅助 GUI 重构 | [README](ui-input/2026-10-17-theme-ui-refactor/README.md) |
| `2026-10-17-theme-ui-source-review` | 主题三包与 tg-iced 源码审查 | [README](ui-input/2026-10-17-theme-ui-source-review/README.md) |
| `2026-10-17-theme-ui-rereview` | 主题与 GUI 重构后二次复审 | [README](ui-input/2026-10-17-theme-ui-rereview/README.md) |
| `2026-10-18-input-diagnosis` | input 再次诊断 | [README](ui-input/2026-10-18-input-diagnosis/README.md) |
| `2026-10-18-input-fixes` | input 五项复审问题修复 | [README](ui-input/2026-10-18-input-fixes/README.md) |
| `2026-10-18-input-edge-fixes` | 输入边沿、同步点与捕获链修复 | [README](ui-input/2026-10-18-input-edge-fixes/README.md) |
| `2026-10-18-input-refactor` | input 功能修复与工程化 | [README](ui-input/2026-10-18-input-refactor/README.md) |
| `2026-10-18-input-review` | input 源码审查证据 | [README](ui-input/2026-10-18-input-review/README.md) |
| `2026-10-18-input-rereview` | input 重构后重新审查 | [README](ui-input/2026-10-18-input-rereview/README.md) |
| `2026-10-18-gpu-text-refactor` | GPU selection 与 text 模块源码审查 | [README](ui-input/2026-10-18-gpu-text-refactor/README.md) |
| `2026-10-18-gpu-text-style` | 写法与注释标准复核 | [README](ui-input/2026-10-18-gpu-text-style/README.md) |
| `2026-10-18-iced-history-refactor` | tg-iced 与 tg-history 源码审查及重构 | [README](ui-input/2026-10-18-iced-history-refactor/README.md) |
| `2026-10-18-iced-history-b-comparison` | 与 B 版目录及功能覆盖对照 | [README](ui-input/2026-10-18-iced-history-b-comparison/README.md) |

## 新增报告约定

1. 放到对应领域的 `reports/<group>/`，目录名用 `YYYY-MM-DD-<slug>`（日期在前，slug 用小写连字符）。
2. 报告目录内应有 `README.md` 作为入口；没有时用主文档（`STATUS.md`、`VALIDATION.md`、`ANALYSIS.md`）。
3. 原始日志/转储优先放子目录（如 `logs/`、`evidence/`），不要把几十 MB 的生成物散在根层。
4. 被后续报告取代的旧迭代移入 `reports/_archive/<date>-<topic>/`，并在 [\_archive/README.md](_archive/README.md) 登记；归档只移动、不删除。
5. 目录移动后更新本索引与 `crates/*/README.md` 中的链接。历史移动映射见 [\_archive/move-manifest-2026-10-24.tsv](_archive/move-manifest-2026-10-24.tsv)。

## 已知失效链接（移动前即存在）

以下类型的链接在本次重构前就无法解析，未随本次移动修改：

- 指向旧工作区绝对路径 `.../games_test/...` 或 Windows 路径 `D:/game/...` 的历史记录。
- `before/` 快照目录内部保留的是搬迁前的仓库布局，其相对链接按快照自身结构解释。
- `crates/country-core/recovery/agent-team/*` 引用的 `reports/eco/`、`reports/wave3/`、`reports/reviews/` 等目录在更早已经不存在。
