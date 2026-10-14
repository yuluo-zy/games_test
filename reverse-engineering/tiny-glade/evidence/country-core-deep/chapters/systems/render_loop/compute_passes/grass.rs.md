# systems/render_loop/compute_passes/grass.rs

> 文件作用：草地生成：`grass_precompute`（瓦片布局预计算）、`grass_compute`（W29，草地实例生成 + DrawIndirect）、`register_required_components`（bevy 组件注册）、`clone_behavior`（克隆行为胶水）。`0x140002370` 两行是 bevy_ecs `is_exclusive`/派生 clone 共享地址（非业务）。

函数总计 5：业务 4 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::grass::grass_compute` | 0x140a60280 | 41 | W29 草地生成：串证据 `Grass/generate_grass.cs.hlsl` + grass_tile_buffer + `Grass bundle ranges/grass/calc_grasslet_bundle_ranges.cs.hlsl`；grass_precompute + ComputePass::write_draw_indirect_cmd_buffer + uniform + read_texture；dispatch_indirect | O | render-loop\decomp\decomp-0x140a60280.txt（串：Grass、/generate_grass.cs.hlsl、grass_tile_buffer、Grass bundle ranges）；wp5 compute-passes §5 |
| `bevy_ecs::system::schedule_system::impl$1::is_exclusive<…FunctionSystem…>` | 0x140002370 | 107 | bevy_ecs 泛型 `is_exclusive`（非业务；各域共享地址） | O（识别为依赖） | audio-misc\decomp\decomp-0x140002370.txt（多域同址） |
| `country_core::systems::render_loop::compute_passes::grass::impl$4::clone_behavior` | 0x140002370 | 107 | 派生 Clone 胶水（与 is_exclusive 同址，U 名错位；非业务） | I（胶水） | 同上；PLAYBOOK 事实 7 候选 |
| `country_core::systems::render_loop::compute_passes::grass::impl$4::register_required_components` | 0x140a66970 | 107 | bevy 组件注册：`Vec::push/pop`（行 107，wp5 O） | O | render-loop\decomp\decomp-0x140a66970.txt；wp5 compute-passes §5 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `grass_precompute`（行 132）— 预计算：bind_terrain_heightmap + read_texture + uniform + dispatch_1d/groups（wp5 O）

## 关键事实

- Shader URI（O）：`/grass/{calc_tiles, tile_bundle_gen, tile_bundle_count, tiles_prefix, bundles_prefix, calc_dispatch_args, calc_grasslet_bundle_ranges}.cs.hlsl` + `_generate_grass.cs`。
- 写 DrawIndexedIndirect/DrawIndirect/AtomicCounter（sequence.md）；`inclusive_prefix_scan_u32_1m` 复用（prefix_scan.rs）。
- 结构体：`GrassBladeDataSsbo`、`ObjectGrassPush`（draw_passes 共享）。
