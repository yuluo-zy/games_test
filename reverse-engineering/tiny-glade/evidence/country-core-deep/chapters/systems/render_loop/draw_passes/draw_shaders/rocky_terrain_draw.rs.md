# systems/render_loop/draw_passes/draw_shaders/rocky_terrain_draw.rs

> 文件作用：岩质地形绘制：`rocky_terrain_shader`（岩质地形网格绘制）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…draw_shaders::rocky_terrain_draw::rocky_terrain_shader` | 0x1409cb4e0 | 16 | 岩质地形绘制：DrawBuilder draw（岩质地形网格；读 rocky_terrain 位移数据） | I（绘制结构） | render-loop\decomp\decomp-0x1409cb4e0.txt；terrain-water\decomp\decomp-0x1409cb4e0.txt |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `rocky_terrain_shader`（0x1409cb4e0）回调（d_cb_b §2）：`rocky_terrain_shader(b, false, RockyTerrainMeshVertCount)`（env43）；顶点数读取时机在调用前（I：原版在 shader 内最后读，仅影响缺资源时 panic 先后）。
- 数据流：`displace_rocky_terrain`（W4，首 qword /3 截 u32 组数，5 纹理 9/0xa/0xe/5/0x13，dispatch_1d）→ `RockyTerrainVertexSSBO/TriangleSSBO`（`SparseInstanceBuffer` 或 TSB）→ `_rocky_terrain_indirect.raster` draw。
- shader body 不挖（GPU 后置）。

## 关键事实

- 消费 displace_rocky_terrain（W4）位移结果。
