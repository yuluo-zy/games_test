# systems/garden/draw_garden.rs

> 文件作用：**花园画笔系统：工具 id 分发（9=橡皮/17=花丛），半径=笔刷+1.5，界内校验后走 draw_raster<GardenRasterStagingSsbo> 绘制花园掩码。**

函数总计 4：业务 1 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::garden::draw_garden::draw_garden` | 0x140ae4620 | 25 | 工具 id-6<0xf 且 id-5∈{4,0xc} 分支；Circle2d 半径=笔刷+1.5（常量 1.5@0x1429258d0）；GladeBorder::is_shape_inside 界内校验 → draw_raster<GardenRasterStagingSsbo> | O | terrain-water\decomp\decomp-0x140ae4620.txt

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项

## 实现方式（算法级）

`draw_garden`（0xae4620）：工具 id 分发（`id-6<0xf` 且 `id-5∈{4,0xc}`；9=橡皮/17=花丛）；Circle2d 半径 = 笔刷 + 1.5（0x1429258d0）；GladeBorder::is_shape_inside 界内校验 → draw_raster<GardenRasterStagingSsbo>。
