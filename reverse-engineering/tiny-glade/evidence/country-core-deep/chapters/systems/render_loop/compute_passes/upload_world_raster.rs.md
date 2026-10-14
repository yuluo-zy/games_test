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
