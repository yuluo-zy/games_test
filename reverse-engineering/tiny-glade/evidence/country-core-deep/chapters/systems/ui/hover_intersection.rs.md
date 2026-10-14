# systems/ui/hover_intersection.rs

> 文件作用：悬停相交检测：par_chunk_map 射线×体积（(WidgetId,&WidgetParams)）→ Vec<WidgetIntersection> + 按 layer/distance 排序的 Vec<(&WorldCanvasLayer, WidgetIntersection)>，结果写入 UiState 上下文。

函数总计 14：业务 1 / 内联无独立符号 0 / 派生胶水 13

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::hover_intersection::hover_intersection` | 0x1408383a0 | 41 | 系统:输入处理 | O（wp15 b5 既有）：并行体积测试 + 排序（sort_by + stable merge） | wp15/analysis/b5_ui_state/hover_intersection_0x1408383a0.c |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 13 项
