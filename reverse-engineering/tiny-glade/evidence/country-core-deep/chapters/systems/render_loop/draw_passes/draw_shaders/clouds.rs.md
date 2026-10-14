# systems/render_loop/draw_passes/draw_shaders/clouds.rs

> 文件作用：云绘制 shader：`CloudsPlugin::build`、`startup_clouds`（启动加载 `/cloud.raster.hlsl`）。

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…clouds::impl$0::build` | 0x14085b8a0 | 19 | 插件 build（NotFound 手解）：`mov rcx,rdx; jmp <shared>` 尾调用 thunk（与 billboard_forest/stars 同型共享 builder） | O（thunk 结构）；共享目标 U | 手解：`rne-read.py --hex 0x14085b8a0 40`：`48 89 d1 e9 68 c6 30 00 …` |
| `…clouds::startup_clouds` | 0x14085b8b0 | 24 | 启动加载：串证据 `cloud_shader` + `/cloud.raster.hlsl` + `cloud/cloud.raster.hlsl` | O | render-loop\decomp\decomp-0x14085b8b0.txt（串：cloud_shader、/cloud.raster.hlsl） |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 关键事实

- shader：`/cloud.raster.hlsl`（云光栅，GPU 后置不挖内部算法）。
