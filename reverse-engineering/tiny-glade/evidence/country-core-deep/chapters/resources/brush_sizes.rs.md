# resources/brush_sizes.rs

> 文件作用：**笔刷尺寸资源：path@+0 / water@+4 / garden@+8 三个 f32 槽位，各自 set=clamp / reset=默认。draw_water/draw_path/draw_garden 读取对应槽位。**

函数总计 7：业务 6 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::brush_sizes::BrushSizes::reset_path` | 0x1409f1860 | 28 | path 笔刷重置为 0.8（mov [rcx], 0x3f4ccccd） | O | terrain-water/DIGEST.md
| `country_core::resources::brush_sizes::BrushSizes::set_path` | 0x1409f1870 | 31 | path 笔刷 = clamp(value, 0.32, 1.2)（maxss/minss 常量 0x142ac56c0/0x1429cd128） | O | terrain-water/DIGEST.md
| `country_core::resources::brush_sizes::BrushSizes::reset_water` | 0x1409f1890 | 38 | 水笔刷重置为 2.0（[rcx+4]=0x40000000） | O | terrain-water/DIGEST.md
| `country_core::resources::brush_sizes::BrushSizes::set_water` | 0x1409f18a0 | 41 | 水笔刷 = clamp(value, 1.0, 5.0)（0x142925904/0x142925988） | O | terrain-water/DIGEST.md
| `country_core::resources::brush_sizes::BrushSizes::reset_garden` | 0x1409f18c0 | 48 | 花园笔刷重置为 3.0（[rcx+8]=0x40400000） | O | terrain-water/DIGEST.md
| `country_core::resources::brush_sizes::BrushSizes::set_garden` | 0x1409f18d0 | 51 | 花园笔刷 = clamp(value, 2.0, 4.0)（0x14292e6f0/0x142934694） | O | terrain-water/DIGEST.md

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
