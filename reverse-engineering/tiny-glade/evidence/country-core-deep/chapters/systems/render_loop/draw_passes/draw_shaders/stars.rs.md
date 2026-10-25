# systems/render_loop/draw_passes/draw_shaders/stars.rs

> 文件作用：星星绘制 shader：`StarsPlugin::build`、`startup_stars`（启动加载 `/star.raster.hlsl`）。

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…stars::impl$0::build` | 0x140903c90 | 19 | 插件 build（NotFound 手解）：`mov rcx,rdx; jmp <shared>` 尾调用 thunk（与 clouds/billboard_forest 同型） | O（thunk 结构）；共享目标 U | 手解：`rne-read.py --hex 0x140903c90 40`：`48 89 d1 e9 58 2a 26 00 …` |
| `…stars::startup_stars` | 0x140903ca0 | 24 | 启动加载：串证据 `star_shader` + `/star.raster.hlsl` + `star/star.raster.hlsl` | O | render-loop\decomp\decomp-0x140903ca0.txt（串：star_shader、/star.raster.hlsl） |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `StarsPlugin::build`（0x140903c90）：`mov rcx,rdx; jmp <shared>` thunk（clouds/billboard_forest 同型共享 builder）。
- `startup_stars`（0x140903ca0）：串 `star_shader` + `/star.raster.hlsl` + `star/star.raster.hlsl`（三名称全中，O 抽查）；load_mesh/shader + spawn_empty。
- draw_lists 分派目标（star 材质族）；shader body 不挖（GPU 后置）。

## 关键事实

- shader：`/star.raster.hlsl`（夜空星星光栅）。
