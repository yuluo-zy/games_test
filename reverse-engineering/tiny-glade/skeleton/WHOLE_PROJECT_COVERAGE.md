# A/B/C 全项目结构覆盖核对

此清单按整个现有项目逐文件核对。`unknown` 为证据缺口；文件经过检查不代表其类型布局或游戏实现已经恢复。

| 版本 | 现有 .rs 文件 | 模块分组 | 新增局部布局 | 新增字段观察 | 资源/历史 schema | 已有证据关联 |
|---|---:|---:|---:|---:|---:|---:|
| A | 1070 | 38 | 1 | 1 | 0 | 75 |
| B | 1159 | 40 | 6 | 23 | 604 | 623 |
| C | 1438 | 61 | 3 | 11 | 0 | 85 |

## A

| 项目模块 | 文件数 | PDB 命名实体 | PDB 定义 | 新增局部布局 | Schema | 证据关联 |
|---|---:|---:|---:|---:|---:|---:|
| _references | 125 | 0 | 1155 | 0 | 0 | 0 |
| _unmapped | 3 | 1 | 12 | 0 | 0 | 0 |
| crates/asset | 9 | 22 | 0 | 0 | 0 | 0 |
| crates/asset-pipe | 2 | 3 | 0 | 0 | 0 | 0 |
| crates/conservative-line-raster | 1 | 0 | 0 | 0 | 0 | 0 |
| crates/country-core | 491 | 1058 | 0 | 1 | 0 | 42 |
| crates/crash-reporter-gui | 1 | 6 | 0 | 0 | 0 | 0 |
| crates/feedback-reporter | 1 | 4 | 0 | 0 | 0 | 0 |
| crates/frozen | 1 | 1 | 0 | 0 | 0 | 1 |
| crates/lang-lib | 2 | 1 | 0 | 0 | 0 | 0 |
| crates/motomoto | 1 | 4 | 0 | 0 | 0 | 0 |
| crates/paths | 1 | 0 | 0 | 0 | 0 | 0 |
| crates/rhapsody | 60 | 137 | 0 | 0 | 0 | 0 |
| crates/shader-variants | 1 | 0 | 0 | 0 | 0 | 0 |
| crates/systems/clutter | 20 | 34 | 0 | 0 | 0 | 0 |
| crates/systems/color | 17 | 6 | 0 | 0 | 0 | 0 |
| crates/systems/critters | 24 | 23 | 0 | 0 | 0 | 0 |
| crates/systems/decorator | 64 | 44 | 0 | 0 | 0 | 0 |
| crates/systems/forest | 3 | 2 | 0 | 0 | 0 | 0 |
| crates/systems/glade-border | 3 | 2 | 0 | 0 | 0 | 0 |
| crates/systems/onboarding | 17 | 27 | 0 | 0 | 0 | 0 |
| crates/systems/roof | 26 | 9 | 0 | 0 | 0 | 8 |
| crates/systems/timberframe | 4 | 2 | 0 | 0 | 0 | 0 |
| crates/systems/toolbar | 37 | 121 | 0 | 0 | 0 | 0 |
| crates/systems/tree-tool | 11 | 4 | 0 | 0 | 0 | 0 |
| crates/systems/wall-constructor | 46 | 34 | 0 | 0 | 0 | 18 |
| crates/systems/water | 4 | 3 | 0 | 0 | 0 | 0 |
| crates/tabloid | 1 | 5 | 0 | 0 | 0 | 0 |
| crates/tg-gpu-selector | 3 | 8 | 0 | 0 | 0 | 0 |
| crates/tg-iced | 3 | 4 | 0 | 0 | 0 | 0 |
| crates/tg-integrity-report | 2 | 3 | 0 | 0 | 0 | 0 |
| crates/tg-package-integrity | 1 | 3 | 0 | 0 | 0 | 0 |
| crates/tg-text | 3 | 15 | 0 | 0 | 0 | 0 |
| crates/theme-defs | 1 | 2 | 0 | 0 | 0 | 0 |
| crates/theme-fetcher | 2 | 3 | 0 | 0 | 0 | 0 |
| crates/theme-server-proto | 1 | 2 | 0 | 0 | 0 | 0 |
| crates/tiny-glade | 40 | 56 | 0 | 0 | 0 | 0 |
| crates/utils | 38 | 55 | 0 | 0 | 0 | 6 |

## B

| 项目模块 | 文件数 | PDB 命名实体 | PDB 定义 | 新增局部布局 | Schema | 证据关联 |
|---|---:|---:|---:|---:|---:|---:|
| _references | 125 | 0 | 1155 | 0 | 0 | 0 |
| _unmapped | 2 | 0 | 12 | 0 | 0 | 0 |
| crates/asset | 9 | 22 | 0 | 0 | 0 | 0 |
| crates/asset-pipe | 2 | 4 | 0 | 0 | 0 | 0 |
| crates/conservative-line-raster | 1 | 0 | 0 | 0 | 0 | 0 |
| crates/country-core | 539 | 1328 | 0 | 6 | 604 | 459 |
| crates/crash-reporter-gui | 1 | 7 | 0 | 0 | 0 | 0 |
| crates/feedback-reporter | 1 | 6 | 0 | 0 | 0 | 0 |
| crates/frozen | 1 | 1 | 0 | 0 | 0 | 2 |
| crates/geforcenow | 1 | 2 | 0 | 0 | 0 | 0 |
| crates/lang-lib | 2 | 1 | 0 | 0 | 0 | 0 |
| crates/leekspin | 2 | 1 | 0 | 0 | 0 | 0 |
| crates/motomoto | 1 | 4 | 0 | 0 | 0 | 0 |
| crates/paths | 1 | 0 | 0 | 0 | 0 | 0 |
| crates/rhapsody | 69 | 154 | 0 | 0 | 0 | 6 |
| crates/shader-variants | 1 | 0 | 0 | 0 | 0 | 0 |
| crates/systems/clutter | 20 | 32 | 0 | 0 | 0 | 3 |
| crates/systems/color | 17 | 13 | 0 | 0 | 0 | 0 |
| crates/systems/critters | 24 | 24 | 0 | 0 | 0 | 29 |
| crates/systems/decorator | 74 | 62 | 0 | 0 | 0 | 11 |
| crates/systems/forest | 3 | 2 | 0 | 0 | 0 | 0 |
| crates/systems/glade-border | 3 | 2 | 0 | 0 | 0 | 0 |
| crates/systems/onboarding | 20 | 30 | 0 | 0 | 0 | 2 |
| crates/systems/roof | 26 | 13 | 0 | 0 | 0 | 10 |
| crates/systems/timberframe | 4 | 2 | 0 | 0 | 0 | 0 |
| crates/systems/toolbar | 38 | 140 | 0 | 0 | 0 | 8 |
| crates/systems/tree-tool | 11 | 4 | 0 | 0 | 0 | 0 |
| crates/systems/wall-constructor | 53 | 34 | 0 | 0 | 0 | 27 |
| crates/systems/water | 4 | 3 | 0 | 0 | 0 | 2 |
| crates/tabloid | 1 | 5 | 0 | 0 | 0 | 0 |
| crates/tg-gpu-selector | 3 | 9 | 0 | 0 | 0 | 0 |
| crates/tg-iced | 3 | 4 | 0 | 0 | 0 | 0 |
| crates/tg-integrity-report | 2 | 4 | 0 | 0 | 0 | 0 |
| crates/tg-package-integrity | 1 | 3 | 0 | 0 | 0 | 0 |
| crates/tg-text | 3 | 14 | 0 | 0 | 0 | 1 |
| crates/theme-defs | 1 | 2 | 0 | 0 | 0 | 0 |
| crates/theme-fetcher | 2 | 3 | 0 | 0 | 0 | 0 |
| crates/theme-server-proto | 1 | 2 | 0 | 0 | 0 | 0 |
| crates/tiny-glade | 45 | 70 | 0 | 0 | 0 | 4 |
| crates/utils | 42 | 64 | 0 | 0 | 0 | 59 |

## C

| 项目模块 | 文件数 | PDB 命名实体 | PDB 定义 | 新增局部布局 | Schema | 证据关联 |
|---|---:|---:|---:|---:|---:|---:|
| _generated | 2 | 0 | 0 | 0 | 0 | 0 |
| _references | 178 | 0 | 1155 | 0 | 0 | 0 |
| _unmapped | 67 | 0 | 12 | 0 | 0 | 0 |
| crates/asset | 13 | 36 | 0 | 0 | 0 | 0 |
| crates/asset-pipe | 5 | 13 | 0 | 0 | 0 | 0 |
| crates/clutter-icon-pipe | 1 | 10 | 0 | 0 | 0 | 0 |
| crates/clutter-pack-pipe | 2 | 24 | 0 | 0 | 0 | 0 |
| crates/conservative-line-raster | 1 | 0 | 0 | 0 | 0 | 0 |
| crates/country-core | 557 | 1628 | 0 | 2 | 0 | 45 |
| crates/crash-reporter-gui | 1 | 7 | 0 | 0 | 0 | 0 |
| crates/feedback-reporter | 1 | 6 | 0 | 0 | 0 | 0 |
| crates/frozen | 1 | 1 | 0 | 0 | 0 | 1 |
| crates/geforcenow | 1 | 2 | 0 | 0 | 0 | 0 |
| crates/lang-lib | 2 | 1 | 0 | 0 | 0 | 0 |
| crates/motomoto | 1 | 4 | 0 | 0 | 0 | 0 |
| crates/paths | 1 | 0 | 0 | 0 | 0 | 0 |
| crates/systems/clutter | 20 | 36 | 0 | 0 | 0 | 0 |
| crates/systems/color | 28 | 28 | 0 | 0 | 0 | 0 |
| crates/systems/critters | 24 | 24 | 0 | 1 | 0 | 0 |
| crates/systems/decorator | 86 | 64 | 0 | 0 | 0 | 1 |
| crates/systems/forest | 3 | 2 | 0 | 0 | 0 | 0 |
| crates/systems/glade-border | 3 | 2 | 0 | 0 | 0 | 0 |
| crates/systems/onboarding | 29 | 36 | 0 | 0 | 0 | 0 |
| crates/systems/roof | 24 | 13 | 0 | 0 | 0 | 8 |
| crates/systems/timberframe | 4 | 2 | 0 | 0 | 0 | 0 |
| crates/systems/toolbar | 49 | 151 | 0 | 0 | 0 | 0 |
| crates/systems/tree-tool | 11 | 4 | 0 | 0 | 0 | 0 |
| crates/systems/wall-constructor | 66 | 64 | 0 | 0 | 0 | 24 |
| crates/systems/water | 4 | 3 | 0 | 0 | 0 | 0 |
| crates/tabloid | 1 | 5 | 0 | 0 | 0 | 0 |
| crates/tg-gpu-selector | 3 | 8 | 0 | 0 | 0 | 0 |
| crates/tg-iced | 4 | 5 | 0 | 0 | 0 | 0 |
| crates/tg-integrity-report | 2 | 4 | 0 | 0 | 0 | 0 |
| crates/tg-mods | 5 | 10 | 0 | 0 | 0 | 0 |
| crates/tg-package-integrity | 1 | 2 | 0 | 0 | 0 | 0 |
| crates/tg-text | 3 | 15 | 0 | 0 | 0 | 0 |
| crates/theme-defs | 1 | 2 | 0 | 0 | 0 | 0 |
| crates/theme-fetcher | 2 | 3 | 0 | 0 | 0 | 0 |
| crates/theme-server-proto | 1 | 2 | 0 | 0 | 0 | 0 |
| crates/tiny-glade | 58 | 104 | 0 | 0 | 0 | 0 |
| crates/turbosloth | 7 | 42 | 0 | 0 | 0 | 0 |
| crates/usdl | 7 | 17 | 0 | 0 | 0 | 0 |
| crates/usdl-sys | 9 | 4 | 0 | 0 | 0 | 0 |
| crates/utils | 43 | 62 | 0 | 0 | 0 | 6 |
| generated | 2 | 0 | 0 | 0 | 0 | 0 |
| rhapsody2 | 67 | 277 | 0 | 0 | 0 | 0 |
| rhapsody2/_unlocated-packages/rhapsody-deferred | 1 | 0 | 0 | 0 | 0 | 0 |
| rhapsody2/crates/aftermath-rs | 2 | 8 | 0 | 0 | 0 | 0 |
| rhapsody2/crates/dxgi-enumerate-gpus | 1 | 1 | 0 | 0 | 0 | 0 |
| rhapsody2/crates/gpu-allocator | 6 | 13 | 0 | 0 | 0 | 0 |
| rhapsody2/crates/gpu-profiler | 3 | 10 | 0 | 0 | 0 | 0 |
| rhapsody2/crates/vk-sync-rs | 2 | 2 | 0 | 0 | 0 | 0 |
| rhapsody2/render-std/rhapsody-core | 5 | 12 | 0 | 0 | 0 | 0 |
| rhapsody2/render-std/rhapsody-dof | 1 | 1 | 0 | 0 | 0 | 0 |
| rhapsody2/render-std/rhapsody-gi | 1 | 1 | 0 | 0 | 0 | 0 |
| rhapsody2/render-std/rhapsody-post | 2 | 0 | 0 | 0 | 0 | 0 |
| rhapsody2/render-std/rhapsody-shadow | 1 | 2 | 0 | 0 | 0 | 0 |
| rhapsody2/render-std/rhapsody-taa | 1 | 0 | 0 | 0 | 0 | 0 |
| shabriri/_unlocated-packages/shabriri | 1 | 0 | 0 | 0 | 0 | 0 |
| shabriri/crates/shabriri-rt | 4 | 9 | 0 | 0 | 0 | 0 |
| shabriri/crates/shaderface | 6 | 37 | 0 | 0 | 0 | 0 |

逐文件完整状态在 `pdb-whole-project-coverage.json` 和各版本 `_pdb/<版本>/structures/project-coverage.json`。可确认的 PDB 结构字段/枚举在 `PDB STRUCTURES V1`，已有模块证据、资源 schema 和新增局部原指令字段在 `WHOLE PROJECT EVIDENCE V1`。

PDB C/C++ 定义记录含第三方依赖，不能当作已恢复的游戏结构。通用库实现、完整 Rust ABI、缺失字段/默认值和原作者源码仍没有补造。B 原指令与资源信息未套用到 A/C。
