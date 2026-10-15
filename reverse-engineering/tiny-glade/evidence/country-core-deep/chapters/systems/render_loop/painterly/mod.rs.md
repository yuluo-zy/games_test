# systems/render_loop/painterly/mod.rs

> 文件作用：painterly 风格化笔触揭示链（渐进笔触会话动画）：`PainterlyState`（enum2$，判别 +8：Default=2 未启用；变体 0/1 载荷 `{progress f64@0, extent [u32;2]@0xc, timer f32@0x14}`）、`PainterlyPlugin::build`、`painterly_pass`（compute+raster 链）、`startup_painterly`。reconstruction `reconstruction/country-core/src/systems/render_loop/painterly.rs` 已按机器码全量恢复（O）。

函数总计 6：业务 4 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::painterly::PainterlyState::has_target` | 0x1408a63d0 | 49 | has_target = `byte[+8] & 1`（即阶段==Ready；手解机器码 `movzx eax,[rcx+8]; and al,1; ret`） | O | 手解：`rne-read.py --hex 0x1408a63d0 32`：`0f b6 41 08 24 01 c3 …`；reconstruction painterly.rs |
| `country_core::systems::render_loop::painterly::impl$1::build` | 0x1408a63e0 | 61 | PainterlyPlugin::build：`insert_resource(state=2)` + PreStartup 系统（reconstruction O） | O | render-loop\decomp\decomp-0x1408a63e0.txt；reconstruction painterly.rs |
| `country_core::systems::render_loop::painterly::painterly_pass` | 0x1408a6410 | 76 | painterly_pass（7118 B）：compute+raster 链；串证据：Painterly acquire target/convolve target/generate strokes/raster strokes pass + `/painterly/{acquire_target,convolve_target,generate_strokes}.cs.hlsl` + `/painterly/raster_strokes.raster.hlsl` + painterly_strokes SSBO + painterly_depth_target；STROKE_COUNT 1024、STROKES_PER_SECOND 1638.4、INSTANT_TIMER 100000（reconstruction 常量 O） | O | render-loop\decomp\decomp-0x1408a6410.txt；reconstruction painterly.rs |
| `country_core::systems::render_loop::painterly::startup_painterly` | 0x1408a7fe0 | 353 | startup_painterly：`load_shader_into_library` ×2（/painterly/raster_strokes + debug_target）；串证据：painterly_debug_target、save thumbnail、Failed creating session thumbnail texture | O | render-loop\decomp\decomp-0x1408a7fe0.txt；reconstruction painterly.rs |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `raster_strokes`（行 270）— `/painterly/raster_strokes.raster.hlsl` 光栅笔触

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. 状态机
`PainterlyState` enum2$ 判别 **+8**（Default=2 未启用；变体 0/1 载荷 `{progress f64@0, extent [u32;2]@0xc, timer f32@0x14}`）；`has_target = byte[+8] & 1`（阶段==Ready，手解 O）。`PainterlyPlugin::build`：insert state(=2) + PreStartup。

### 2. `painterly_pass`（0x1408a6410，7118B）—— 渐进笔触算法
- **instant 判定**：资源(0xff)字节 != 0 ? true : 资源(0x80).u32==0；instant 时 progress 步长=1024、timer uniform=100000.0（INSTANT_TIMER）；
- **目标 extent = `[(w<<8)/h, 256]`**（**h==0 → div by zero panic**）；
- 采集条件 = 阶段==Pending || !(target & depth 均复用)；
- generate 的 push constants = `[start u32, count u32, timer f32]`，线程数 = count（STROKE_COUNT 1024 分批，STROKES_PER_SECOND 1638.4/s）；
- raster 顶点 = `6 × floor(progress')`（progress 封顶）；timer += delta_f32。
- Pass 链：acquire target → convolve target → generate strokes → raster strokes；常量 FORMAT_COLOR_TARGET 0x25、FORMAT_DEPTH_TARGET 0x4c、STROKES_BUFFER_BYTES 0x4000、BUFFER_ALIGN 0x20。
- 与 vermeer 共用 PassHost 缝（app_mode / time / raster_shader / blur_pyramid 等）。

## 关键事实

- Pass 名（O）："Painterly acquire target" / "Painterly convolve target" / "Painterly generate strokes" / "Painterly raster strokes"；URI：`/painterly/{acquire_target.cs.hlsl, convolve_target.cs.hlsl, generate_strokes.cs.hlsl, raster_strokes.raster.hlsl, debug_target.raster.hlsl}`。
- 常量（O）：STROKE_COUNT 1024.0、STROKES_PER_SECOND 1638.4、INSTANT_TIMER 100000.0、FORMAT_COLOR_TARGET 0x25、FORMAT_DEPTH_TARGET 0x4c、STROKES_BUFFER_BYTES 0x4000、BUFFER_ALIGN 0x20。
- 资源 TypeId（O）：AppMode、PainterlyState、AssetShaderLibrary、time（与 vermeer 共用）+2 个未知资源。
- 与 vermeer 共用 PassHost 缝与描述符（reconstruction 架构 O）。
