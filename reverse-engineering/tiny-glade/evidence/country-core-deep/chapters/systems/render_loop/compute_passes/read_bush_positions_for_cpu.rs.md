# systems/render_loop/compute_passes/read_bush_positions_for_cpu.rs

> 文件作用：灌木位置 CPU 回读：`read_bush_positions_for_cpu`（W15，把 GPU 灌木 DrawIndirect/位置回读到 CPU SSBO）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu` | 0x140866540 | 15 | W15 灌木位置回读：`/garden/read_bush_positions_for_cpu.cs.hlsl`（wp5 O）；ComputePass::read_draw_indirect_cmd_buffer + dispatch_1d；写 `BushCpuSsbo`（供 CPU 布娃娃/编辑查询） | O | render-loop\decomp\decomp-0x140866540.txt；wp5 compute-passes §20 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `read_bush_positions_for_cpu`（0x140866540，W15）
处理链（O，B_c3 §2）：
1. cpu-mapped 写 + 两个读 + `draw_indirect_buffer_offset=10`；
2. `dispatch_1d(0x2581)`（9601）；
3. 写 `BushCpuSsbo`（MappedStorageBuffer，memory 4 回读）。
- 数据流：灌木 GPU 位置 → CPU 副本 → 布娃娃/编辑查询（`AssetSsboLibrary` 侧 `get_cpu_shadow_words`，B_x_out §2）。
- 帧序 W15（W14 spawn_vegetation_around_buildings 后、W16 garden_spawn_vegetation 前）。

## 关键事实

- 结构体（U 布局）：`BushCpuSsbo`（灌木 CPU 副本）。
- 帧序 W15（frame-wiring，W14 spawn_vegetation_around_buildings 后）。
