# 机器台账索引

本目录是 JSON 台账，供脚本与分析读取。Markdown 说明在 `../docs/`；两者冲突时以当前源码/最新绑定证据为准。

## 当前台账

| 文件 | 用途 |
|---|---|
| [`baselines.json`](baselines.json) | A/B/C 三版身份、路径与哈希 |
| [`version-map.json`](version-map.json) | 跨版本模块/文件映射 |
| [`module-progress.json`](module-progress.json) | 逐模块恢复状态（101 模块，生成物） |
| [`module-progress-overrides.json`](module-progress-overrides.json) | 模块进度的人工覆盖输入 |
| [`module-coverage.json`](module-coverage.json) | PDB 源路径覆盖库存；是 `scripts/update-module-progress.py` 的输入，不是完成率 |
| [`module-progress/`](module-progress/) | 逐模块明细 |
| `foundation-business-functions.json` | 基础库业务功能清单 |
| `shared-foundation-*.json` | 基础库接口、对照与未恢复项 |
| `dependency-policy.json`、`dependency-exclusions.json` | 依赖归属与筛除 |
| [`original-Cargo.lock`](original-Cargo.lock)、`original-packages.json` | 原作依赖身份 |
| [`package-graph.json`](package-graph.json) | 包依赖图与验收 |
| [`project.json`](project.json) | 项目元数据 |
| `func*`/`abc-*` | 函数索引与共享候选 |

## 历史快照

`three-version-plan.json`、`building-system-plan.json`、`reconstruction-strategy-plan.json`、`render-pass-pilot*.json`、`parallel-modules-stopped-snapshot.json` 等是历史计划快照。只作溯源，不作为当前状态。`latest-*-checkpoint.json` 已随 checkpoints 目录删除。

## 维护

- 模块进度：`python3 ../scripts/update-module-progress.py`
- 全项目进度：`python3 ../scripts/update-progress.py`（根路径由脚本按自身位置解析）
- 不要把 `module-coverage.json` 当作已恢复功能清单或完成率分母。
