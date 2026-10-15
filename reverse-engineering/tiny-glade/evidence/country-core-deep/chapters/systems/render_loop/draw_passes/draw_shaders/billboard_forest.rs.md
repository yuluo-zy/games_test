# systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs

> 文件作用：Billboard 森林绘制 shader：`BillboardForestPlugin::build`、`startup_billboard_forest`（启动加载 `/billboard_forest.raster.hlsl`）。

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…billboard_forest::impl$0::build` | 0x140ae8070 | 18 | 插件 build（NotFound 手解）：`mov rcx,rdx; jmp <shared>` 尾调用 thunk → 共享插件 build（Clouds/Stars 同型） | O（thunk 结构）；共享目标 U | 手解：`rne-read.py --hex 0x140ae8070 40`：`48 89 d1 e9 d8 ff 07 00 …` |
| `…billboard_forest::startup_billboard_forest` | 0x140ae8080 | 27 | 启动加载：串证据 `/billboard_forest.raster.hlsl` + `billboard_forest_shader` | O | render-loop\decomp\decomp-0x140ae8080.txt（串：billboard_forest、/billboard_forest.raster.hlsl、billboard_forest_shader） |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `BillboardForestPlugin::build`（0x140ae8070）：`mov rcx,rdx; jmp <shared>` 尾调用 thunk → 共享插件 build（Clouds/Stars 同型，d_cb/startup 族）。
- `startup_billboard_forest`（0x140ae8080）：`load_mesh_into_library + load_shader_into_library`（/billboard_forest.raster.hlsl + `billboard_forest_shader`）+ spawn_empty。
- draw_lists 分派目标之一（nani_billboard_forest 材质族）；shader body 不挖（GPU 后置）。

## 关键事实

- draw_lists 分派目标之一（nani_billboard_forest 材质族）。
