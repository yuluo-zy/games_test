# systems/render_loop/compute_passes/shader_refactoring.rs

> 文件作用：着色器重构对比：`render_shader_refactoring`（W54，抓取/对比新旧 shader 输出的调试 pass）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring` | 0x14098f9f0 | 15 | W54 着色器重构调试：`_shader_refactoring.cs.hlsl_{capture,compare}`（wp5 O）；read_texture + write_texture + dispatch_2d；callees 仅 RwLock（作用 U） | I（wp5 记名；串仅 unwrap 错误） | render-loop\decomp\decomp-0x14098f9f0.txt；wp5 compute-passes §22 |

## 关键事实

- 用途：对比重构前后 shader 输出（`_capture`/`_compare` 两变体）。
- 与 `resources/shader_refactoring_state`（audio-misc 域）联动（状态禁用它则跳过）。
