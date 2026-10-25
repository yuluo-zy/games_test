# systems/terrain/edit_terrain_stroke.rs

> 文件作用：**编辑地形笔画：EditTerrainStrokeCmd/SetTerrainStrokePtsCmd 事件 → TerrainEditingData 查询（缺失 panic）→ falloff/height 应用（Absolute 直写/Delta 限幅）→ RecalculateHeightMapCmd + 声音。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::terrain::edit_terrain_stroke::edit_terrain_stroke` | 0x14094e000 | 0 | 按命令 id 查 TerrainEditingData（缺失 panic 'Was requested to edit a terrain stroke  that doesnt exist'）；falloff∈[0,1]/height Abs≥0/Delta≥0.1 应用；发 RecalculateHeightMapCmd + PlayTerrainSound | O | terrain-water\decomp\decomp-0x14094e000.txt

## 实现方式（算法级）

`edit_terrain_stroke`（0x94e000）：按命令 id 查 TerrainEditingData（缺失 panic "Was requested to edit a terrain stroke  that doesnt exist"）；falloff∈[0,1] / height（Abs≥0 / Delta≥0.1）应用（prior tg-terrain apply_resolved）→ RecalculateHeightMapCmd + PlayTerrainSound。
