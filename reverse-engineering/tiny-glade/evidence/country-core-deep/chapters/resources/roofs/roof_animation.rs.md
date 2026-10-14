# resources/roofs/roof_animation.rs

> 文件作用：**RoofAnimation 组件**（屋顶瓦/结构动画状态，16 字节零化初值；`is_being_animated` 读首个 f32 字段）。
> `clone_behavior` 是多组件共享的空桩（0x140002370 = `xor eax,eax; ret`，返回 None，编译器跨模块去重体），
> `register_required_components` 为 Bevy 注册胶水。消费方：roof 动画系统、roof_tiles_gravity compute。

函数总计 5：业务 5 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$2::clone_behavior` | 0x140002370 | 5 | 共享空桩：`xor eax,eax; ret`（返回 None/空 behavior）。同一地址在 roof_tiles/roof_trigger_volume/大量模块复用 = 编译器去重 | O（胶水） | 手解（rne-read --hex 0x140002370；原 audio-misc 等 decomp 均为 NotFound 副本，弃用） |
| `impl$2::register_required_components` | 0x1408e06d0 | 5 | Bevy 注册胶水：FUN_140d40d70 注册 + FUN_1408db5b0 构造 required components 追加 | O（胶水） | roofs-rt/decomp/decomp-0x1408e06d0.txt |
| `RoofAnimation::is_being_animated` | 0x1408dbc20 | 13 | `cmpss [rcx], const, NEQ; movd eax; and eax,1` —— 首字段(动画计时) != 常量 → true | O | 手解（rne-read --hex 0x1408dbc20） |
| `RoofAnimation::new` | 0x1408dbc40 | 22 | `xorps xmm0/xmm1; ret` —— 返回 16 字节全零 RoofAnimation | O | 手解（rne-read --hex 0x1408dbc40） |
| `impl$4::register_required_components` | 0x1408e0740 | 24 | 同族 Bevy 注册胶水 | O（胶水） | roofs-rt/decomp/decomp-0x1408e0740.txt |

## 布局/结构要点

- RoofAnimation ≥ 16 字节（new 用两个 xmm 寄存器零化）；`is_being_animated` 读 +0 处 f32。
- 交叉域：与 render_loop/compute_passes/roof_tiles_gravity 及 roof_visual_shape 动画初态（Roof +0x50）关联。
