# systems/render_loop/compute_passes/roof_tiles_gravity.rs

> 文件作用：屋顶瓦片重力模拟：`roof_tiles_gravity_compute`（W19，屋顶瓦片受重力位移/掉落模拟）。

函数总计 5：业务 1 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute` | 0x140a29b10 | 13 | W19 屋顶瓦片重力：串证据 `Roof tiles/roof_tiles_gravity.cs.hlsl` + `/roof_tiles_gravity.cs.hlsl`；AssetSsboLibrary::get（RoofTileSsbo）+ World::query（RoofTiles 组件）+ flat_map + uniform；PushConstants（闭包 closure$0/closure$1，dispatch_1d）；`+frame_constants_storage_buffer@1.1`（O） | O | render-loop\decomp\decomp-0x140a29b10.txt（串：Roof tiles、/roof_tiles_gravity.cs.hlsl）；wp5 compute-passes §21；render-gap\decomp\decomp-0x140a29b10.txt |

> 补充（render-gap §3.5，2026-10-14；**round3 2026-10-15 名字还原**）：`ComputePass::new` 名长 0xa、URI 长 0x1b；`bind_texture(0xc3483cd70105de2f)`；
> 资源 `TypeId 0x4bc362195b80f729/0x982798ce80cbcefb`（= **`resources::time::GameTime`**，round3 全 EXE 53 处证实）在场且门控字节 `cStack_282 != 8` 时
> `bind_uniform(0x601a45ce43af912f, *(f32*)(buf+0x28))`——**uniform 名 = `delta_seconds`**（round3 O：`fnv1a("delta_seconds") = 0x601a45ce43af912f` 逐位吻合；
> 值 = `GameTime.delta_f32 @+0x28`，time.rs 章布局 O；名字不在重建 HLSL——CPU 侧 uniform 名常量折叠）。
> 闭包 0x1415f9210：20 B PushConstants 数组逐项 `push_constants`+`dispatch_3d(count,1,1)`，与 `dispatch_with` 一致。

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项（PushConstants 构造闭包 ×2 + 迭代器）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `roof_tiles_gravity_compute`（0x140a29b10，W19）
处理链（O，B_c2 §2 + round3 §6）：
1. **先收集 push constants**，再建 pass；`collect_push_constants`（20B `{first_instance, instance_count, wall_height_delta, roof_pivot_y, animation_t}`）；
2. `AssetSsboLibrary::get（RoofTileSsbo）` + `QueryState<(RoofTiles, RoofTileSimulation, Roof, RoofAnimation)>` + flat_map；
3. **`delta_seconds` uniform**：`GameTime` 在场且 `mode != 8` → `bind_uniform(0x601a45ce43af912f, *(f32*)(buf+0x28))`（round3 O：fnv1a("delta_seconds") 吻合；值 = GameTime.delta_f32@+0x28）；
4. `dispatch_with` 闭包（0x1415f9210）：逐项 `push_constants` + `dispatch_3d(count,1,1)`；**空表也登记 pass**。
- 数据流：RoofTileSsbo（SparseInstanceBuffer 分块）→ 重力位移 → `instanced_roof`（draw_shaders 章）draw_instanced。
- 帧序 W19。

## 关键事实

- 读 `RoofTileSsbo`（roof_tile_ssbo 域资源，nani 剔除同用）；查询 `RoofTiles` 组件。
- 帧序 W19（frame-wiring）。
