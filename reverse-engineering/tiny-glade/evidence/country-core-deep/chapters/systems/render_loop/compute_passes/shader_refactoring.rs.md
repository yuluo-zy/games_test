# systems/render_loop/compute_passes/shader_refactoring.rs

> 文件作用：着色器重构对比：`render_shader_refactoring`（W54，抓取/对比新旧 shader 输出的调试 pass）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring` | 0x14098f9f0 | 15 | W54 着色器重构调试状态机：第一跳转表 0x142ab9ff0（disc 0→返回；1→kind2；2→`payload&3==3?0:2`；3/4→`payload&3==3?1:2`；5→Result::unwrap panic；6..9→退化合并分支，不可达）；第二跳转表 0x142aba004（更新规则 1→{2,0}；2→{2,payload+1}\|{3,0}；3/4→{3\|4,payload+1}+`[frame+0x20]=feedback desc 尾字`） | O（2026-10-14 跳转表机器码） | render-loop\decomp\decomp-0x14098f9f0.txt；wp5 compute-passes §22；render-gap\DIGEST.md §3.1 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `render_shader_refactoring`（0x14098f9f0，W54）—— 双跳转表状态机
状态（O，round3 §4 精确化）：
- **第一表 0x142ab9ff0**：disc 0 → 直接返回（不碰 tick）；1 → kind 2；2 → `payload&3==3 ? kind0 : 2`；3/4 → `payload&3==3 ? kind1 : 2`；5 → Result::unwrap（跳进 ref-temporal unwrap thunk 中部，崩溃）；**6..9 → 指令中部死条目（UB/崩溃，round3 逐条定位）**。
- **第二表 0x142aba004**（更新规则）：1 → {2,0}；2 → `payload<4 ? {2,payload+1} : {3,0}`；3/4 → `payload+1` + **把 feedback 纹理 desc 尾字拷进 frame 句柄 +0x20**（`desc_tail_copied`，36B 拷贝，O）。
- kind 2 无 pass；kind 0 = capture 一遍；kind 1 = capture 到临时纹理后 compare（`_capture`/`_compare` 两入口）。
- 写回 `ResMut<ShaderRefactoringState>`（布局 `{u32 disc, u32 payload}`，TypeId 0xc95ed87f413070ab）的 changed tick。
- **结论**：第二表更新规则只产出 disc ∈ {0,2,3,4} → 5..9 不可达；6/7 的第一表条目指向第二表共享块"带前缀跳过"形态（编译器对死变体冷块合并）。reconstruction 按 no-op 安全正确，注释应改"死变体"。
- 联动：`eval_gi_screen_radiance` 的 `allow_non_determinism = (state==0)`。

## 关键事实

- 用途：对比重构前后 shader 输出（`_capture`/`_compare` 两变体）。
- 与 `resources/shader_refactoring_state`（audio-misc 域）联动（状态禁用它则跳过）。
- **纠错（render-gap §3.1）**：disc 5 是 `Result::unwrap()` panic（跳进 ref-temporal unwrap thunk 中部），
  **不是** `unreachable!()`；disc 6..9 跳进共享收尾代码中部（帧槽未初始化 → 退化），且第二表更新规则只产出
  disc ∈ {0,2,3,4}，故 disc 5..9 **实际不可达**。reconstruction 的 `_ => None // 6..=9：U` 按 no-op 安全且正确。
- **round3 精确化（2026-10-15，`round3/RENDER-U-CLOSURE.md` §4）**：disc 6 → 0x14099061b（指令对齐起点 = 第二表 idx2 处理器 0x140990613
  跳过 8 字节前缀；`payload<4→{2,payload+1}，否则{3,0}` + tick + ret，但用遗留寄存器/未初始化帧槽 → UB）；
  disc 7 → 0x1409905ff（`movdqa xmm1` 指令中部）；disc 8/9 → 0x1409905d2（`mov edx,2` 指令中部，崩溃）、disc 5 → 0x140990721（`lea r9` 指令中部，崩溃）——**死条目**。
  结论不变：6..9 不可达；但机器码有确定转移块（= 第二表 idx2/idx3-4 同一代码），reconstruction 注释应从"no-op"改为"死变体"。`desc_tail_copied`（36 B 拷贝）机器码确认。
