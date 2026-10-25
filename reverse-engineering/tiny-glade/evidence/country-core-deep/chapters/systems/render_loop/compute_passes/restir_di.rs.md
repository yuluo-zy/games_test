# systems/render_loop/compute_passes/restir_di.rs

> 文件作用：ReSTIR DI（直接光时空复用）：`restir_sample_direct_lights`（W41，ReSTIR reservoir 采样直接光 + 时域复用）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights` | 0x140a6fec0 | 24 | W41 ReSTIR 直接光：串证据 `RDI temporal/rdi/temporal.cs.hlsl` + `/rdi/temporal.cs.hlsl`（时域复用）；AssetSsboLibrary::get + InstanceBuffer::instance_count + read_texture + write_texture + dispatch_2d；reservoir（`rdi_reservoir_tex`） | O | render-loop\decomp\decomp-0x140a6fec0.txt（串：RDI temporal、/rdi/temporal.cs.hlsl）；wp5 compute-passes §14 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `restir_sample_direct_lights`（0x140a6fec0，W41）
处理链（O，B_c3 §2）：
1. **灯笼数 0 → 仅 1×1 占位纹理 "RDI reservoirs placeholder"，不声明 pass**；
2. 否则 temporal "RDI reservoirs"（`get_or_create_temporal_pair`）+ "RDI temporal" pass（`/rdi/temporal.cs.hlsl`）；
3. `AssetSsboLibrary::get + InstanceBuffer::instance_count` + read_texture（光照/深度）+ write_texture + dispatch_2d。
- 数据流：`bin_lights`（W40）灯光网格 → ReSTIR reservoir（`rdi_reservoir_tex`）→ `deferred_raster`（W43）读。
- 帧序 W41（W40 bin_lights 后、W42 apply_screen_shadows 前）。

## 关键事实

- 输出 ReSTIR reservoir（deferred_raster W43 读 `rdi_reservoir_tex`，wp5 draw-passes §5）。
- 候选 shader（U 确切）：`_rdi_temporal`（wp5 注）。
