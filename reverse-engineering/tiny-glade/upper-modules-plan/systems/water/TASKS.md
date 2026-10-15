# systems/water 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/water` |
| package | `system-water` |
| 规模 | 4 文件 / 4 本 ns 函数 / 1 闭包 / **2 业务函数** |
| 难度 | 低 |
| 批次 | 1 |
| 上游依赖 | `country-core`（WaterRaster / world_raster）、`rhapsody`、`utils`、`perchance`、`glam` |
| 复现工程现状 | `country-core/src/recovered_systems/water.rs` 已有 WaterRaster CPU 状态、world_raster/replay/upload 调用；`evidence/country-core-deep/terrain-water/` 有 DIGEST |

## 任务表

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-WATER-01 | `src/record_water_history.rs` | 恢复水面编辑的历史记录：把水面变更打包成历史命令并提交。本文件无独立符号，需从消费者/被内联点定位。 | core 历史事务（NR2） | 无 | 原侧入口对照 + 与 core 历史命令 schema 一致 | 未开始 |
| UM-WATER-02 | `src/draw_water.rs` | 恢复 `draw_water`：水面的每帧绘制/上传调度（CPU 侧提交，GPU 后置）。 | WaterRaster、rhapsody | 仅索引 | 原入口对照；GPU 消费登记为后置 | 未开始 |
| UM-WATER-03 | `src/water_audio_grid.rs` | 恢复 `update_water_audio_grid`：水面音频网格更新（音频跨切面）。 | 音频胶水 | 无 | 原指令样例 + 网格参数与原一致 | 未开始 |
| UM-WATER-04 | `src/lib.rs` | 恢复插件注册 `impl Plugin::build`：系统/事件/资源注册顺序。 | 全部上面 | 仅索引 | 与原 `add_systems` 顺序对照 | 未开始 |

## 卡点

- `record_water_history` 无独立符号，需先定位内联宿主（可能内联进 draw_water 或 core 的水系统）。
- 音频（FMOD）为跨切面，UM-WATER-03 与 decorator/roof/timberframe 的音频任务共用同一胶水策略。

## 验收标准

- 每任务 ≥ 原入口对照或原指令样例；插件注册顺序与原一致。
- 水面 CPU 状态与 core 的 WaterRaster（WorldRaster<512> u8 网格）不重复实现。
