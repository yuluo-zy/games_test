# systems/render_loop/compute_passes/dof.rs

> 文件作用：景深：`dof_prepare`（W49，CoC/半分辨率预处理）、`calc_dof`（W51，多分辨率或全分辨率 DoF 模糊）、`calc_dof_multires`/`calc_dof_fullres`（内联实现）。

函数总计 6：业务 2 / 内联无独立符号 2 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::dof::dof_prepare` | 0x140a5a020 | 29 | W49 DoF 预处理：串证据 `DoF prepare/dof/prepare.cs.hlsl` + CoC / CoC tiles + dof output；`/dof/prepare.cs.hlsl`（+`/dof/init_buffers.cs.hlsl`、`/dof/clear_halfres.cs.hlsl`）；读 `DofAutofocusSsbo`；read_texture + write_texture + dispatch | O | render-loop\decomp\decomp-0x140a5a020.txt（串：DoF prepare、/dof/prepare.cs.hlsl、CoC、CoC tiles、dof output）；wp5 compute-passes §2 |
| `country_core::systems::render_loop::compute_passes::dof::calc_dof` | 0x140a5a570 | 62 | W51 DoF 模糊：串证据 `Depth of field/dof/fused.cs.hlsl` + CoC dilated/DoF tile offsets/DoF tile coords/DoF dispatch args；按 `calc_dof_multires`（162）或 `calc_dof_fullres`（79）分支；前者逐 mip 步进（Iterator::count + multiunzip）后者直接 dispatch | O | render-loop\decomp\decomp-0x140a5a570.txt（串：CoC dilated、DoF tile offsets、DoF tile coords、DoF dispatch args、Depth of field、/dof/fused.cs.hlsl）；wp5 compute-passes §2 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `calc_dof_fullres`（行 79）— `/dof/fullres.cs.hlsl`；dispatch
- `calc_dof_multires`（行 162）— `/dof/{halfres, fused, fullres_upsample, fullres_all_lod0, fullres_all_lod1}`；DynamicConstants::push_slice + push_constants + dispatch_1d/2d（wp5 O）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`next` 等 2 项（AsComputeShaderSource 实现：FullResDof/HalfResDof 等）

## 关键事实

- W50/W51 由 `param_6[0xa9+(res_type==0x13?1:0)] != 0` 门控（frame-wiring §3）。
- 结构体 `IndirectArgsType`（U 字段）。
- **补充（render-gap §2.1）**：5 个 `source_hash` 中 `impl$11`（FullResDof）@0x140a5de00、`impl$7`（FullResAllLod1Dof）@0x140a5d960
  已反编译确认 = `calculate_hash(&(uri:entry, &self))`（串 `/dof/fullres.cs.hlsl:cs_main` 0x1c / `/dof/fullres_all_lod1.cs.hlsl:cs_main` 0x25），
  与 `dof_source!` 宏实现逐位一致——**已实现，只需补地址标注**。证据：render-gap\decomp\decomp-0x140a5de00.txt / decomp-0x140a5d960.txt。
