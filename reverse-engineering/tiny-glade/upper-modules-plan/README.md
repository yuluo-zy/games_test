# 上层模块（country-core 之上）恢复任务清单

> **2026-10-19 更新**：任务状态列（未开始/草稿/部分/候选）写于 2026-10-15，已与代码不符。当前的未实现、`todo!` 空壳、占位函数和缺失函数清单见 [STATUS-2026-10-19.md](STATUS-2026-10-19.md)；本文件的规模表、批次与卡点仍可参考，其中"落点待定"一项已有结论：系统专属逻辑已迁入 `tiny-glade/crates/systems/*`，`recovered_building`/`recovered_systems` 已删除。


本目录把 **B 原作 `country-core` 之上**的模块，按 B 源码目录层级拆成可查看、可勾选的细化任务。

## 1. 口径

- `core` = B 原作 `crates/country-core`。
- **上层模块** = 依赖 `country-core` 的 crate，共 14 个：
  - `crates/systems/*` 13 个：water、tree-tool、forest、timberframe、glade-border、roof、critters、clutter、color、onboarding、wall-constructor、decorator、toolbar
  - `crates/tiny-glade`（应用/集成层，依赖上面全部 + country-core + rhapsody + utils + theme/tg-*）
- 依赖方向：`tiny-glade` / `systems/*` → `country-core` → {asset, asset-pipe, rhapsody, utils, tabloid, …}。所以 `systems/*` 与 `tiny-glade` 都是 core 的上层。
- 数据源：`skeleton/B/crates` 的 PDB 骨架（`compiled_procedure` 符号）。规模只统计**本 crate 命名空间**的发射函数，已剔除依赖单态化（ron/png/image/serde/bevy/alloc）与派生胶水。
- 骨架只含**有独立符号**的函数；按 country-core 的 SOURCE_MAP 经验，另有约 1/6 为纯内联无符号，实际业务量约再高 1.3–1.5 倍。
- 目录层级严格照 B 的 `crates/.../src/...` 组织，便于对照骨架文件（每个 `*.rs` 骨架文件含该文件全部函数符号与依赖注解）。

## 2. 规模总表

| 模块（B 路径） | 文件 | 本 ns 函数 | 闭包 | 业务函数 | 难度 | 批次 |
|---|---:|---:|---:|---:|:--:|:--:|
| `systems/water` | 4 | 4 | 1 | 2 | 低 | 1 |
| `systems/tree-tool` | 11 | 15 | 2 | 12 | 低 | 1 |
| `systems/forest` | 3 | 4 | 0 | 3 | 低 | 1 |
| `systems/timberframe` | 4 | 4 | 0 | 3 | 低 | 1 |
| `systems/glade-border` | 3 | 3 | 1 | 1 | 低 | 1 |
| `systems/roof` | 26 | 37 | 6 | 27 | 中 | 1 |
| `systems/critters` | 24 | 56 | 9 | 42 | 中 | 2 |
| `systems/clutter` | 20 | 38 | 6 | 21 | 中 | 2 |
| `systems/color` | 17 | 31 | 6 | 21 | 中 | 2 |
| `systems/onboarding` | 20 | 66 | 4 | 48 | 中 | 2 |
| `systems/wall-constructor` | 53 | 105 | 23 | 73 | 高 | 3 |
| `systems/decorator` | 74 | 236 | 58 | 164 | 高 | 3 |
| `systems/toolbar` | 38 | 509 | 129 | 331 | 高 | 3 |
| `crates/tiny-glade` | 45 | 170 | 31 | 112 | 高 | 4 |
| **合计** | **342** | **1278** | **276** | **≈860** | | |

## 3. 批次与顺序

| 批次 | 内容 | 任务数 | 文件 |
|---|---|---:|---|
| 批次 1 | 低难度 + 已部分恢复的先收口 | 43 | [batches/1-低难度收口.md](batches/1-低难度收口.md) |
| 批次 2 | 并行（互不依赖，证据各补各的） | 80 | [batches/2-并行.md](batches/2-并行.md) |
| 批次 3 | 主线串行 | 141 | [batches/3-主线串行.md](batches/3-主线串行.md) |
| 批次 4 | tiny-glade 集成（依赖全部 + 平台后端） | 27 | [batches/4-集成.md](batches/4-集成.md) |
| **合计** | | **291** | |

顺序规则：
1. 批次 1 先收口，为后面提供已闭合的子系统与证据模板。
2. 批次 2 五域互不依赖，可并行分域 agent，每域一个唯一 worker。
3. 批次 3 串行：wall-constructor 剩余族 → decorator（最大，依赖前者）→ toolbar。
4. 批次 4 最后；tiny-glade 依赖全部上层 + 平台后端。

## 4. 任务文件（按 B 目录层级）

```
systems/
  water/TASKS.md              tree-tool/TASKS.md
  forest/TASKS.md             timberframe/TASKS.md
  glade-border/TASKS.md       roof/TASKS.md
  critters/TASKS.md           clutter/TASKS.md
  color/TASKS.md              onboarding/TASKS.md
  wall-constructor/TASKS.md   decorator/TASKS.md
  toolbar/TASKS.md
tiny-glade/TASKS.md
```

每个 `TASKS.md` 含：模块元信息 → 按 B 源码子模块分组的任务表 → 卡点/依赖 → 验收标准。

## 5. 任务编号与状态图例

编号：`UM-<模块码>-<两位序号>`（例：`UM-WATER-01`、`UM-DECOR-07`）。

| 状态 | 含义 |
|---|---|
| 未开始 | 尚无本机源码/分析 |
| 草稿 | 有反汇编草稿或骨架，未编译/未验证 |
| 部分 | 有可编译实现，覆盖不完整或未接权威状态 |
| 候选 | 实现+局部原码校验通过，未接真实消费者 |
| 待验收 | 实现完整，缺原侧机器码/系统级验收 |

每行任务的「验收」列写明该任务完成的最低证据要求（原指令样例 / 原入口对照 / 消费者接线 / 系统级行为）。

## 6. 全局卡点（影响多个模块）

1. **依赖 core 权威状态未闭合**：decorator / roof / wall-constructor 消费 `PublicWallState`、墙/屋顶 authority、历史事务；这些在 core 侧（TASK_BACKLOG 的 NR1/NR2）仍未闭合。上层可对着冻结契约先恢复，但端到端验收要等 core。
2. **音频跨切面**：FMOD 胶水散落在 6 个 crate 的 8 个文件（decorator/audio/*、clutter inter_roof_merlons、roof simulate_roof_tile_for_audio、timberframe、water_audio_grid、toolbar/mei/audio），不是单模块任务。
3. **profiling 跨切面**：`puffin` 出现在全部 systems crate，属统一接线。
4. **落点待定**：复现工程当前把 systems/* 折叠进 country-core 的 `recovered_building` / `recovered_systems`，未按 B 重建独立 crate。**继续折叠**还是**按 B 拆分**需先定口径，再决定每个任务的源码落点。本清单只按 B 目录组织，不预设落点。
5. **tiny-glade 定位**：大量为平台/后端胶水（winit/bevy glue、sentry、steamworks、DLSS/FSR）。若目标是「可运行游戏」则必需，若只做游戏业务逻辑可大幅降权。
6. **A/C 版本**：本清单只针对 B。A/C 不在此范围。
