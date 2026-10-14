# systems/wall/shapes/calc_shape_height.rs

> 文件作用：形状预览高度计算：shape{tag=6,ty} ty1=circle/2=rectangle 否则高度 0；as_curve2 到 crashy_resample_remap_per_segment 地形采样；base rect 6.0/circle 8.0；height-ty 2 到 +地形否则 avg+base-0.46；final-(max-0.46) clamp[0.7,14.0] 写预览+0x40 + time_continuous_lerp。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::calc_shape_height::calc_shape_height` | 0x140ac80e0 | 0 | calc_shape_height（base 6.0/8.0、-0.46、clamp[0.7,14.0]、写预览+0x40） | O | wall\decomp\decomp-0x140ac80e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
