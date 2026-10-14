# systems/render_loop/compute_passes/shader_refactoring.rs

> 文件作用：着色器重构对比：`render_shader_refactoring`（W54，抓取/对比新旧 shader 输出的调试 pass）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring` | 0x14098f9f0 | 15 | W54 着色器重构调试状态机：第一跳转表 0x142ab9ff0（disc 0→返回；1→kind2；2→`payload&3==3?0:2`；3/4→`payload&3==3?1:2`；5→Result::unwrap panic；6..9→退化合并分支，不可达）；第二跳转表 0x142aba004（更新规则 1→{2,0}；2→{2,payload+1}\|{3,0}；3/4→{3\|4,payload+1}+`[frame+0x20]=feedback desc 尾字`） | O（2026-10-14 跳转表机器码） | render-loop\decomp\decomp-0x14098f9f0.txt；wp5 compute-passes §22；render-gap\DIGEST.md §3.1 |

## 关键事实

- 用途：对比重构前后 shader 输出（`_capture`/`_compare` 两变体）。
- 与 `resources/shader_refactoring_state`（audio-misc 域）联动（状态禁用它则跳过）。
- **纠错（render-gap §3.1）**：disc 5 是 `Result::unwrap()` panic（跳进 ref-temporal unwrap thunk 中部），
  **不是** `unreachable!()`；disc 6..9 跳进共享收尾代码中部（帧槽未初始化 → 退化），且第二表更新规则只产出
  disc ∈ {0,2,3,4}，故 disc 5..9 **实际不可达**。reconstruction 的 `_ => None // 6..=9：U` 按 no-op 安全且正确。
