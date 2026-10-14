# systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs

> 文件作用：水面平面/水生植物绘制：`water_planes_draw`、`lilypad_draw`（睡莲叶）、`lilypad_flower_draw`（睡莲花）、`reed_draw`（芦苇）。

函数总计 4：业务 4 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…water_planes_draw::water_planes_draw` | 0x1409d9bc0 | 0 | 水面平面绘制：DrawBuilder draw（水面；消费 water_surface/ice_surface 数据） | I（绘制结构） | render-loop\decomp\decomp-0x1409d9bc0.txt；terrain-water\decomp\decomp-0x1409d9bc0.txt |
| `…water_planes_draw::lilypad_draw` | 0x1409da580 | 74 | 睡莲叶绘制：draw_instanced（读 LilypadSsbo） | I | render-loop\decomp\decomp-0x1409da580.txt；terrain-water\decomp\decomp-0x1409da580.txt |
| `…water_planes_draw::lilypad_flower_draw` | 0x1409da760 | 91 | 睡莲花绘制：draw_instanced（读 LilypadFlowerSsbo） | I | render-loop\decomp\decomp-0x1409da760.txt；terrain-water\decomp\decomp-0x1409da760.txt |
| `…water_planes_draw::reed_draw` | 0x1409da940 | 108 | 芦苇绘制：draw_instanced（读 ReedSsbo） | I | render-loop\decomp\decomp-0x1409da940.txt；terrain-water\decomp\decomp-0x1409da940.txt |

## 关键事实

- 三个水生植物 draw 消费 compute_water.rs（W13）生成实例。
