# systems/render_loop/compute_passes/stairs.rs

> 文件作用：楼梯（岩质台阶）生成：`stairs`（W20，按地形生成楼梯实例 + 间接绘制参数）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::stairs::stairs` | 0x140a68a50 | 23 | W20 楼梯：`/rocky_terrain/stairs/stairs.cs.hlsl`（wp5 O）；read + write_draw_indirect_cmd_buffer + read_draw_indirect_cmd_buffer + dispatch_2d + dispatch；`StairsTransformSsbo`；raster `_stairs_indirect.raster` | O | render-loop\decomp\decomp-0x140a68a50.txt；wp5 compute-passes §21 |

## 关键事实

- 结构体（U 布局）：`StairsTransformSsbo`；输出由 stairs_draw（draw_shaders）消费。
- 帧序 W20（frame-wiring）。
