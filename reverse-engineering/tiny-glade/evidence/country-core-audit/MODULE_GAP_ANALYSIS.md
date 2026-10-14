# country-core 模块差距分析（哪些模块还需要分析）

日期：2026-10-14。基线：提交 `ec054d20`（章节化源码地图 + 三轮回溯证据）之后的工作区。
本文只做分析，没有改任何源码。标注：O 实测 / I 推断 / U 未测量。

## 0. 结论

`ec054d20` 的深度证据只覆盖 `systems/**` 与 `resources/**`。**差距最大、还没有分析的是下面第一节的模块，它们在深度证据里的覆盖是 0%。**

## 统计口径

- 数据：`data/procs.tsv`（PDB 全部过程，85,312 行），按 `country_core::<模块>::<子模块>` 汇总。
- 已剔除名字含 `drop_in_place`、`fmt`、`serde`、`visit_`、`clone`、`Debug`、`schedule_system`、`FunctionSystem`、`SystemParam`、`type_id`、`as_any` 的胶水。
- “深度证据覆盖”= 该函数地址是否出现在 `evidence/country-core-deep/**/*.md` 或 `SOURCE_MAP.json`。
- “源码引用”= 该函数地址是否出现在 `reconstruction/country-core/src/**/*.rs`。
- 局限：只匹配地址字符串，泛型实例没有归并（所以渲染相关模块的源码引用偏低，见第二节）；只列 ≥6 KiB 的模块；只算 `country_core` 命名空间。
- 一次性统计脚本没有保存到仓库；复现需自行按上述口径重写。

## 1. 完全没分析的模块（深度证据 0%）

| 模块 | 机器码 | 说明 |
|---|---|---|
| `components/impl` | 76 KiB | 组件实现；证据 0%，源码引用 0% |
| `components/drawable` | 25 KiB | 同上，0% / 0% |
| `utils/load_json` | 31 KiB | 68 个调用方依赖 `load_json_as_mesh`，共享 `utils` 里也没有 |
| `utils/dirty_tiles` | 23 KiB | 地形脏区，渲染 C 层若干 pass 依赖 |
| `utils/debug_viz` | 7 KiB | 0% / 0% |
| `startup/load_prefabs` | 34 KiB | 预制体入库，是网格资产的生产者 |
| `startup/load_mesh_atlases` | 21 KiB | 网格图集入库 |
| `startup/startup_session` | 20 KiB | 会话启动 |
| `startup/*` 其余 | 约 40 KiB | `startup_nani`、`startup_terrain`、`startup_sheep`、`clearing`、`spawn_*` 等 |
| `input/action_set`、`input/input_stream` | 21 KiB | `input_stream` 仅 28% 过程被覆盖 |
| `game_key_bindings`、`game_profile` | 19 KiB | 按键绑定、游戏配置 |

合计约 300 KiB，约占 country-core 机器码的 8%。`startup` 源码里虽多处引用了地址（67%–100%），但没有逐函数分析。
“渲染需要、但在渲染目录之外”的内容基本在这里：没有它们，资产库是空的，pass 没有输入。

## 2. 证据口径不同、需要核对（渲染相关）

| 模块 | 机器码 | 深度证据覆盖 | 源码引用 |
|---|---|---|---|
| `render/pass_builder` | 212 KiB | 2% | 40% |
| `render/nani` | 157 KiB | 1% | 58% |
| `asset_libraries/ssbo_library` | 118 KiB | 1% | 16% |
| `render/ssbo` | 75 KiB | 0% | 22% |
| `render/shadowed_buffer` | 36 KiB | 0% | 13% |
| `render/sparse_instance_buffer` | 38 KiB | 0% | 39% |

这些由之前的渲染 B 阶段（`B_*.md`）覆盖，不是空白。但这里的源码引用与 `RENDER_GAP_ANALYSIS.md` 的“96% 单元已引用”对不上，
原因是本表按过程统计、泛型实例未归并。覆盖情况应以 `tools/country-core-audit/coverage.py` 的口径为准再确认。

## 3. 已分析，但源码里几乎没有对应实现

| 模块 | 证据覆盖 | 源码引用 |
|---|---|---|
| `resources/stairs` | 92% | 0% |
| `resources/steam` | 100% | 0% |
| `resources/terrain_height` | 100% | 0% |
| `resources/color_icon` | 100% | 0% |
| `resources/critters` | 86% | 2% |
| `resources/roofs` | 86% | 3% |
| `resources/toolbar` | 82% | 0% |
| `resources/world_raster` | 84% | 5% |
| `systems/mode_manager` | 92% | 8% |
| `resources/walls` | 88% | 15% |

## 4. 证据覆盖高，但内容可疑，需要二次分析

第二轮审查（`country-core-deep/round2/FIX-LIST.md`）中错误率高的域：

| 域 | 审查结果 | 有问题占比 |
|---|---|---|
| collision-eco | 5 WRONG、15 PARTIAL（共 25 条） | 80% |
| ui-input | 10 WRONG、16 PARTIAL（共 33 条） | 79% |
| audio-misc | 19 GAP、18 FIX（共 63 条） | 59% |

旧审计（`AUDIT_REPORT.md`）抽查 62 条合同 O 结论，仅约 56% 完全正确。新章节的 1413 条 O 没有做过同类抽查（U）。

## 5. 排序建议

1. `components/impl` + `components/drawable`（101 KiB）：没有任何证据，所有系统都依赖组件定义。
2. `startup/*`（约 115 KiB）：决定资产库有没有内容。
3. `utils/load_json` + `utils/dirty_tiles`（54 KiB）：有明确下游依赖方。
4. `input/*`、`game_key_bindings`、`game_profile`（40 KiB）。
5. 对 collision-eco、ui-input 的 O 结论做抽查。

## 6. 没覆盖到的范围

- 小于 6 KiB 的模块未列出。
- 原作里 country-core 之外的本地 crate（`frozen`、`toolbar`、`shader-pipeline`、`shader-variants`、`shader-compiler` 等）是另一类缺口，未计入。
- 本文没有改任何源码，也没有提交。

## 7. 相关文档

- `RENDER_GAP_ANALYSIS.md`：渲染模块未实现项。
- `../country-core-deep/INDEX.md`、`OVERVIEW-2026-10-14.md`、`round2/FIX-LIST.md`：深度证据入口与审查清单。
- `AUDIT_REPORT.md`：提交 `664f6a65` 的审计报告。
