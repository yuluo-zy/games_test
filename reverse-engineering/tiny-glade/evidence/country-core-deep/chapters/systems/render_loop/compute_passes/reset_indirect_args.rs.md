# systems/render_loop/compute_passes/reset_indirect_args.rs

> 文件作用：间接参数重置：`reset_indirect_args`（清空/重置间接绘制参数缓冲）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args` | 0x14090a870 | 24 | 重置间接参数：串证据 "Reset indirect args" + `/clearing/`（U 确切 shader）；ssbo_to_rg_handle + RenderGraphRecorder::add_pass + DynamicConstants::push_slice + impl$2::write（GpuOwned 追踪） | O（模式）；shader URI U | render-loop\decomp\decomp-0x14090a870.txt（串：Reset indirect args）；wp5 compute-passes §22 |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 关键事实

- 供 nani/植被/水的 DrawIndirect 缓冲清零复用（wp5 原语/调试族）。
