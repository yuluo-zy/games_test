# systems/terrain/ui_draw_terrain.rs

> 文件作用：**UI 绘制地形：高亮/编辑模式点按 → new_stroke + EvTerrainStrokeChanged；界外反馈（DisplayOutOfBorder/FeedbackInputOutOfBorder）。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain` | 0x140a9fdc0 | 40 | Res<AppMode, CursorTerrainRaycast, AssetSsboLibrary, WaterRaster, UiState, GladeBorder> + ResMut<TerrainEditingData, PendingTerrainHistory, DecoratorArchivist> + 事件组；点按新建笔画 new_stroke + EvTerrainStrokeChanged（ui-input 域细查状态机） | O | terrain-water\decomp\decomp-0x140a9fdc0.txt

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
