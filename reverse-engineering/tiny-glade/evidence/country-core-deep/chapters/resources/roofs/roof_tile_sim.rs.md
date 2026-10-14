# resources/roofs/roof_tile_sim.rs

> 文件作用：**RoofTileSimulation 组件**（屋顶瓦片物理/坠落模拟状态）。`new` 构造：+0 状态 u32、+4 计时 f32=入参、
> +0x8..+0x14 余量零化（编译器重叠零写）。`register_required_components` 为 Bevy 注册胶水。
> 消费方：render_loop/compute_passes/roof_tiles_gravity（瓦片重力模拟）。

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$2::register_required_components` | 0x140939470 | 3 | Bevy 注册胶水（FUN_140d40d70 注册 + FUN_140935f60 required components 追加到 Vec） | O（胶水） | roofs-rt/decomp/decomp-0x140939470.txt |
| `RoofTileSimulation::new` | 0x140939390 | 14 | 构造：`dword[+0]=0; movss[+4]=xmm1(timer 入参); qword[+8]=0; qword[+0xd]=0`——状态+计时+余量零化 | O | 手解（rne-read --hex 0x140939390） |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项（impl$3::default）

## 布局/结构要点

- +0x00 state u32、+0x04 timer f32（new 入参）、+0x08 起零化至 ≥0x14（重叠零写由编译器生成，字段边界需下游确认）。
- 交叉域：roof_tiles_gravity compute pass 消费。
