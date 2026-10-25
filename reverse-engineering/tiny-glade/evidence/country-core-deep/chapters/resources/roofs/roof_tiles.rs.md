# resources/roofs/roof_tiles.rs

> 文件作用：**RoofTile 组件（屋顶瓦片实例）**——本文件只含组件声明/注册面；
> 瓦片实例的实际 CPU 生成（assemble 圆形/矩形、RoofShingle 64 字节记录）已由 reconstruction/roof/tiles.rs 恢复
> （11,467 片圆形瓦 0 差异，见 evidence/roof-tiles/）。`clone_behavior` 为共享空桩。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$2::clone_behavior` | 0x140002370 | 8 | 共享空桩：`xor eax,eax; ret`（返回 None；与 roof_animation/roof_trigger_volume 等多模块共用同一去重地址） | O（胶水） | 手解（rne-read --hex 0x140002370） |
| `impl$2::register_required_components` | 0x140903f40 | 8 | Bevy 注册胶水（FUN_140d40d70 + required components 追加） | O（胶水） | roofs-rt/decomp/decomp-0x140903f40.txt |

## 与 reconstruction/roof 的映射

瓦片业务不在本文件函数内，而在 `reconstruction/roof/tiles.rs`（`assemble_circular_from_observed`/`assemble_circular_tiles`/`TileRng`/`random_splits`）：
RoofShingle 64 字节 GPU 接口字段见 evidence/roof-tiles/shader-interfaces.json（base_position/current_height/clipping_plane/scale__t/rotation_quat/roof_id/velocity/animation_t/seed/is_circle/is_main_roof_body）。
