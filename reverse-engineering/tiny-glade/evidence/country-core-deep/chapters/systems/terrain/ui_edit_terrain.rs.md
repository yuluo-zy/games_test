# systems/terrain/ui_edit_terrain.rs

> 文件作用：**UI 编辑地形：按 hover 笔画 id 生成 EditTerrainStrokeCmd/SetTerrainStrokePtsCmd；维护 MouseActivityTimer/TerrainStrokeParamsCache。ui-input 域共享。**

函数总计 3：业务 1 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::terrain::ui_edit_terrain::ui_edit_terrain` | 0x140aef1f0 | 34 | 按鼠标落点+hover 笔画 id 生成 EditTerrainStrokeCmd/SetTerrainStrokePtsCmd；事件组含 CursorCaptureCmd/EnableArrowGizmoStretch/AddTerrainHighlightCmd 等（ui-input 域主责） | O | terrain-water\decomp\decomp-0x140aef1f0.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::terrain::ui_edit_terrain::MouseActivityTimer::new`（行 211）— 鼠标活动计时器构造（ui_edit_terrain 自有资源）

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
