# systems/terrain/draw_terrain_stroke.rs

> 文件作用：**笔画变化转发：EvTerrainStrokeChanged 事件 → RecalculateHeightMapCmd + PlayTerrainSound。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::terrain::draw_terrain_stroke::terrain_stroke_changed` | 0x140a03ba0 | 17 | EvTerrainStrokeChanged 非空 → 发 RecalculateHeightMapCmd + PlayTerrainSound | O | terrain-water\decomp\decomp-0x140a03ba0.txt

## 实现方式（算法级）

`terrain_stroke_changed`（0xa03ba0）：EvTerrainStrokeChanged 非空 → 发 RecalculateHeightMapCmd + PlayTerrainSound（转发生效链，非本域实现）。
