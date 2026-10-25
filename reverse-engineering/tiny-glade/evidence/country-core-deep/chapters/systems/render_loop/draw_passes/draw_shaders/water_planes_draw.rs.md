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

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `water_planes_draw` 回调（d_cb_b §2）：进程布尔 `WATER_PLANES_DISABLED`（.data 0x143634d90 区，运行期来源 U，默认 false）、`ActiveSession`（变体字节、字节序列）；**`water_planes_draw_rt(b, inputs)` = 真实 `DrawBuilder::bind_ray_tracing::<RtTriangles,RtTriangleMeta,RtBvh,WallColorId>()`**（B_i_infra §5：RT 四槽排在雪纹理之前 + 两个 `write_uniform<u32>` 写入缓冲字；闭包版 `water_planes_draw` 保留作兼容入口）。
- 三个水生植物 draw：`lilypad_draw`（LilypadSsbo）/`lilypad_flower_draw`（LilypadFlowerSsbo）/`reed_draw`（ReedSsbo），回调本体已恢复（d_gap §1.3）。
- 数据流：`water_spawn_vegetation`（W13，demo 守卫；Spawn lilypads grid 225² offset 8 / Spawn reeds grid 150² offset 9；`is_autumn = session[+0x58]==3`）→ 实例缓冲 → draw_instanced。
- shader body 不挖（GPU 后置）。

## 关键事实

- 三个水生植物 draw 消费 compute_water.rs（W13）生成实例。
