# systems/render_loop/compute_passes/grass.rs

> 文件作用：草地生成：`grass_precompute`（瓦片布局预计算）、`grass_compute`（W29，草地实例生成 + DrawIndirect）、`register_required_components`（bevy 组件注册）、`clone_behavior`（克隆行为胶水）。`0x140002370` 两行是 bevy_ecs `is_exclusive`/派生 clone 共享地址（非业务）。

函数总计 5：业务 4 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::grass::grass_compute` | 0x140a60280 | 41 | W29 草地生成：串证据 `Grass/generate_grass.cs.hlsl` + grass_tile_buffer + `Grass bundle ranges/grass/calc_grasslet_bundle_ranges.cs.hlsl`；grass_precompute + ComputePass::write_draw_indirect_cmd_buffer + uniform + read_texture；dispatch_indirect | O | render-loop\decomp\decomp-0x140a60280.txt（串：Grass、/generate_grass.cs.hlsl、grass_tile_buffer、Grass bundle ranges）；wp5 compute-passes §5 |
| `bevy_ecs::system::schedule_system::impl$1::is_exclusive<…FunctionSystem…>` | 0x140002370 | 107 | bevy_ecs 泛型 `is_exclusive`（非业务；各域共享地址） | O（识别为依赖） | audio-misc\decomp\decomp-0x140002370.txt（多域同址） |
| `country_core::systems::render_loop::compute_passes::grass::impl$4::clone_behavior` | 0x140002370 | 107 | 派生 Clone 胶水（与 is_exclusive 同址，U 名错位；非业务） | I（胶水） | 同上；PLAYBOOK 事实 7 候选 |
| `country_core::systems::render_loop::compute_passes::grass::impl$4::register_required_components` | 0x140a66970 | 107 | bevy 组件注册：`Vec::push/pop`（行 107，wp5 O） | O | render-loop\decomp\decomp-0x140a66970.txt（已补；Vec::push 8B 元素注册） |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `grass_precompute`（行 132）— 预计算：bind_terrain_heightmap + read_texture + uniform + dispatch_1d/groups（wp5 O）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `grass_compute`（0x140a60280，W29）—— 11 pass + 9 缓冲
处理链（O，B_c2 §2 + B_i_infra §5）：
1. **8 个 pass + 9 个缓冲**（`/grass/{calc_tiles, tile_bundle_gen, tile_bundle_count, tiles_prefix, bundles_prefix, calc_dispatch_args, calc_grasslet_bundle_ranges}.cs.hlsl` + `_generate_grass.cs`）；
2. **brush_index 主题表 {1,1,2}**（AppMode tag 7..=9，round3 闭合）—— 主题选择 brush；
3. **两个 `dispatch_indirect_raw`**（偏移 0x10 / 0，缓冲取 `grass_dispatch_args_buffer` 句柄拷贝，I）；
4. **prefix scan 接缝**：`inclusive_prefix_scan_u32_1m(env, &SizedRgBuffer{bundle_index, 0x80000})` 三个 pass（i_infra §5）；子和缓冲 `subtotal_sum_buf` 0x1000B；
5. **`freeze_occlusion_culling` 直读**：`movzx r9d,[0x143634da0]` → uniform 值写 `fnv1a("freeze_occlusion_culling")=0x6d7707561ec0da2b`；`mode==8` 跳过；
6. 写 `DrawIndexedIndirectBuffer/DrawIndirectBuffer/AtomicCounterBuffer`（间接绘制参数）。
- 数据流：grass 实例 → `grass_shader`/`grass_lod1_shader`（draw_shaders 章）→ ObjectGrassPush（W9 推入）→ draw_instanced。
- 纠错：`grass_precompute` 无 PDB 对应过程（c2 已删，属臆造）；`register_required_components`/`clone_behavior` 是 bevy 派生胶水（0x140a66970 / 0x140002370，与 is_exclusive ICF）。

## 关键事实

- Shader URI（O）：`/grass/{calc_tiles, tile_bundle_gen, tile_bundle_count, tiles_prefix, bundles_prefix, calc_dispatch_args, calc_grasslet_bundle_ranges}.cs.hlsl` + `_generate_grass.cs`。
- 写 DrawIndexedIndirect/DrawIndirect/AtomicCounter（sequence.md）；`inclusive_prefix_scan_u32_1m` 复用（prefix_scan.rs）。
- 结构体：`GrassBladeDataSsbo`、`ObjectGrassPush`（draw_passes 共享）。
- **round3 闭合（2026-10-15，见 `round3/RENDER-U-CLOSURE.md` §8）**："主题资源"（TypeId `0xc2b8e941c91a6428/0x59f3c6d1d3da43a0`）
  = **`systems::mode_manager::AppMode`**（34 处命中含 `register_resource_with<enum2$<mode_manager…>>`）；首个 u32 = AppMode 判别，
  {1,1,2} 表按 tag 7..=9 取 brush index（tag 语义属 mode_manager 域）。`GladeThemeId` 占位名作废。
- 直读 `0x143634da0`（= `freeze_occlusion_culling` 调试开关字节，round3）作 uniform 值写
  `fnv1a("freeze_occlusion_culling")=0x6d7707561ec0da2b`（键值逐位吻合，O）；`mode==8` 时跳过（同 roof 的 delta_seconds 门控形制）。
