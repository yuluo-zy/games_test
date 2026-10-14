# resources/mod.rs

> 文件作用：**country-core 顶层资源注册入口**——游戏启动/新会话时把渲染资源、SSBO 库与业务资源插入 Bevy App。
> 三个入口全部经 `bevy_app::App` 的 `init_resource/insert_resource` 单态化族（FUN_14153*/FUN_14154* 区域）注册资源，
> 并调用各资源的真实构造函数。`init_populate_ssbo_library` 负责渲染 SSBO 库的静态填充（FUN_1412c* 族 = SSBO 注册 helper）。

函数总计 4：业务 3 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::impl$0::insert_rendering_resources` | 0x140940190 | 208 | 注册渲染类资源（rt 0x1408ba490/0x1408bab30、SSBO 0x140867700、mesh atlas 等；12× func_0x000142923c20 / 8× func_0x000142923c00 = bevy App 注册调度辅助） | O | roofs-rt/decomp/decomp-0x140940190.txt |
| `country_core::resources::init_populate_ssbo_library` | 0x140909e80 | 243 | 填充渲染 SSBO 库：逐槽注册（FUN_1412c4*..1412c6f* 连续 30+ 个注册函数，每个对应一类 SSBO；配 func_0x000140613c10 分配 / 0x613c20 释放） | O | roofs-rt/decomp/decomp-0x140909e80.txt |
| `country_core::resources::impl$1::insert_resources` | 0x140940d80 | 386 | 注册游戏业务资源：逐资源调真实构造函数（0x140b18550/0x140c97500/0x140cff670/0x140cf8250/0x140d3fa40/0x140d4d740/0x140d64e00 等 = world_raster/roof tiles/terrain…），再经 bevy_app insert_resource 入 App | O | roofs-rt/decomp/decomp-0x140940d80.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::init_populate_minimal_startup_ssbo_library`（行 369）— 最小启动集 SSBO 库填充（加载屏所需）；与 init_populate_ssbo_library 同模式，只注册少量必需 SSBO

## 布局/结构要点

- 三入口均为 `fn(app: &mut App)`：唯一参数是 `*mut App`；
  `func_0x000142923c00`/`func_0x000142923c20` 是编译器为 `App::init_resource<T>` / `insert_resource<T>` 生成的调度辅助
  （同地址多模块复用），内部调 `FUN_14153*…14154*`（`bevy_app` 单态化）。
- 交叉域：rt 资源构造经 0x1408ba490/0x1408bab30（见 rt/mod.rs 章）；渲染 SSBO 库插槽与 audio-misc 域的 render_ssbo 相关。
