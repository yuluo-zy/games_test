# systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs

> 文件作用：楼梯绘制：`stairs_draw`（岩质台阶实例绘制，间接参数）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…draw_shaders::stairs_draw::stairs_draw` | 0x14086a780 | 15 | 楼梯绘制：DrawBuilder draw_indexed_indirect（`_stairs_indirect.raster`；消费 stairs W20 生成的间接参数 + StairsTransformSsbo） | I（间接绘制结构） | render-loop\decomp\decomp-0x14086a780.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 关键事实

- 与 stairs.rs（W20）生成配对。
