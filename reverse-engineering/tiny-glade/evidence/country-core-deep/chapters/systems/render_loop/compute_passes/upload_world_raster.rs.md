# systems/render_loop/compute_passes/upload_world_raster.rs

> 文件作用：世界光栅上传（CPU 光栅 staging → GPU 纹理）：`upload_raster_staging_to_texture<PathRaster>` 等 6 单态（W1：PathRaster/Cobblestone1/2/3/WaterRaster/GardenRaster）。

函数总计 6：业务 6 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…upload_raster_staging_to_texture<Cobblestone1Raster>` | 0x1403da050 | 16 | W1 鹅卵石1光栅上传：`/upload_cpu_raster.cs.hlsl`（r16f/rgba16f 入口）；`Upload cpu raster`；utils::get_res_mut + on_uploaded_to_texture + write_texture + uniform + dispatch_2d | O | render-loop\decomp\decomp-0x1403da050.txt（串：Upload cpu raster、r16f、rgba16f、/upload_cpu_raster.cs.hlsl）；terrain-water\decomp\decomp-0x1403da050.txt；wp5 compute-passes §20 |
| `…upload_raster_staging_to_texture<Cobblestone2Raster>` | 0x1403da570 | 16 | W1 鹅卵石2光栅上传（同模式） | O | render-loop\decomp\decomp-0x1403da570.txt；terrain-water\decomp\decomp-0x1403da570.txt；wp5 §20 |
| `…upload_raster_staging_to_texture<Cobblestone3Raster>` | 0x1403d90f0 | 16 | W1 鹅卵石3光栅上传（同模式） | O | render-loop\decomp\decomp-0x1403d90f0.txt；terrain-water\decomp\decomp-0x1403d90f0.txt；wp5 §20 |
| `…upload_raster_staging_to_texture<GardenRaster>` | 0x1403d9b30 | 16 | W1 花园光栅上传（同模式） | O | render-loop\decomp\decomp-0x1403d9b30.txt；terrain-water\decomp\decomp-0x1403d9b30.txt；wp5 §20 |
| `…upload_raster_staging_to_texture<PathRaster>` | 0x1403d8bd0 | 16 | W1 路径光栅上传（同模式） | O | render-loop\decomp\decomp-0x1403d8bd0.txt；terrain-water\decomp\decomp-0x1403d8bd0.txt；wp5 §20 |
| `…upload_raster_staging_to_texture<WaterRaster>` | 0x1403d9610 | 16 | W1 水面光栅上传（同模式） | O | render-loop\decomp\decomp-0x1403d9610.txt；terrain-water\decomp\decomp-0x1403d9610.txt；wp5 §20 |

## 关键事实

- 统一模式（B1，O）：`utils::get_res_mut` + `on_uploaded_to_texture` + write_texture + uniform + dispatch_2d；6 单态按 `world_raster_defs` 类型。
- r16f（单通道）/rgba16f（双格式入口）由 shader 反射表选（串证据 output_tex_、r16f、rgba16f）。
- 帧序 W1（帧最前，把 CPU 编辑的光栅掩码上传 GPU）。

## 实现方式（算法级）

### upload_raster_staging_to_texture<T>（W1，6 单态）

```
ssbo = 库按 type-id hash（如 Path 0x85c987d23d58d2de）查 *RasterStagingSsbo
if ssbo && published(+0x29a) != 0:
  建纹理（格式 0x4c="r16f"→4 通道 / 0x61="rgba16fr16f"→7 通道；其它 → panic "not yet implemented"）
  ssbo.version(+0x5c) ← 库 version(+0x5b0)
  *(u16*)(+0x29a) = 0x100   → +0x29a=0（清已发布）、+0x29b=1（mask 重传置位）
  compute pass："Upload cpu raster" /upload_cpu_raster.cs.hlsl → write_texture + dispatch_2d
else: 不发布则跳过（无脏不上传）
```

**+0x29b 置位方**（闭合 round2 U）：本函数发布后的 word 写 0x100。消费方 = water/garden/path-masks 三处 rasterize_from_cpu（见 ALGORITHMS.md §1.5）。
