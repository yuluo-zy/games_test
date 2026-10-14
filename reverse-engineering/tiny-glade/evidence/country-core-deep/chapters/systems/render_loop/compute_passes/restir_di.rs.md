# systems/render_loop/compute_passes/restir_di.rs

> 文件作用：ReSTIR DI（直接光时空复用）：`restir_sample_direct_lights`（W41，ReSTIR reservoir 采样直接光 + 时域复用）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights` | 0x140a6fec0 | 24 | W41 ReSTIR 直接光：串证据 `RDI temporal/rdi/temporal.cs.hlsl` + `/rdi/temporal.cs.hlsl`（时域复用）；AssetSsboLibrary::get + InstanceBuffer::instance_count + read_texture + write_texture + dispatch_2d；reservoir（`rdi_reservoir_tex`） | O | render-loop\decomp\decomp-0x140a6fec0.txt（串：RDI temporal、/rdi/temporal.cs.hlsl）；wp5 compute-passes §14 |

## 关键事实

- 输出 ReSTIR reservoir（deferred_raster W43 读 `rdi_reservoir_tex`，wp5 draw-passes §5）。
- 候选 shader（U 确切）：`_rdi_temporal`（wp5 注）。
