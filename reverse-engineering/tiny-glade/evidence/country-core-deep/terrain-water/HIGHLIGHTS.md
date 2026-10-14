# terrain-water 域 HIGHLIGHTS（tg_a4 · 2026-10-14，≤10 条）

1. **旧 sysres 索引在本域有 11 处名/址错位，全部已用 PDB 骨架 RVA + v2 S_PUB32 双重纠正**。最危险的三处：0x140a15530 是 `WorldRaster::new<512>` 不是 DirtyTilePublisher::unique（真 unique=0x1409393b0）；0x1409393e0 是 `build_tile_hasher` 不是 copy_composite_to_scratch（真模板实例=0x140a161a0）；0x140868fb0 是 `InProgressRasterStroke::add_point` 不是 resample_and_smooth（真=0x140868e40）。完整表见 DIGEST §7（含 0x140a15da0=clear_all、0x140a197c0=deserialize\<Value>、0x14083eb10=ordered_names、0x140ad3f50=try_add_raw_input_points、0x14145c350=DrawingInputSmoother::process、0x140ad2970=is_shape_inside、0x14003bb80=ICF 裸 ret 等）。

2. **ActiveRasterStroke 记录 = 0x470B**（index_mut = base+idx·0x470），布局已解：三个 12B-Vec3 列（+0x10 固化曲线 / +0x28 进行中原始点 / +0x48 重采样平滑后曲线）、累计弧长 f32@+0x40（>1.0 触发 solidify）、笔刷 f32@+0x60、模式字节@+0x64、blend 标志 u32@+0x68、计数@+0x6c、双 512B 脏瓦片位图@+0x70。`resample_and_smooth` = **10 遍 0.25/0.5/0.25 三点平滑**（O 常量+外层循环计数）。

3. **AllWorldRasterWrite 全链路 O 闭合**：6 槽名序 = PathRaster, Cobblestone1/2/3Raster, WaterRaster, GardenRaster（0x142a93ca8 区胖指针表）；`write` 把系统写入拷回 composite 并 **memset(staging@+0x98, 0xff, 0x200) + `+0x299=1`（全脏+staging 脏）**——prior 悬案“+0x299 置位点”已定位；`upload_to_staging` 按 type-id 查 6 个 StagingSsbo（memcmp 全名如 "…world_raster_defs::PathRasterStagingSsbo"）后 `copy_to_staging(selector=0)`。

4. **WorldRaster::new 精确初始化 O**：composite/scratch 各 0x40000B（=512×512 u8 ✓）+ DirtyTiles 位图 0x4000B（=4096 u32 = 64×64 瓦片，每瓦片 8×8 px，初始状态字 **0x07954dbb**，语义 U）+ 从原型拷贝 56B DirtyTiles 头（含 parking_lot Mutex@+0x60/68，锁语义由 subscribe 的 lock_slow 佐证）+ 清零 0x204B staging/flag 区。

5. **subscribe 的 panic 串解锁 DirtyTilePublisher 语义**："Can't subscribe to a secondary dirty tile publisher. Go for the primary one instead."（0x142ac85f8）——unique/primary 才可订阅；订阅记录 = 0x28B Arc（压入 +0x78 订阅者 Vec，8B 元素，grow=FUN_140562870）。PDB 行号 unique/shared/secondary = 64/68/72（合同“23/29/47”作废）。

6. **BrushSizes 布局闭合**：path@+0（默认 0.8，clamp[0.32,1.2]）/ water@+4（2.0，[1,5]）/ garden@+8（3.0，[2,4]）——现有 reconstruction 把 water 放 +0，需重排并补 path/garden 常量。

7. **TerrainHeightsData 数据面 O**：+0x08 = **u16** 512×512 网格（权威实例 sample 返回 u16；视觉实例 0x141246fb0 为双线性变体，4×load_index_unchecked(0x1408b54c0)+3 lerp，u16→f32 解码用 +0x50/54）；min/max f32@+0x50/54 默认 [-2.5, 24.0]，copy_from_gpu 发现非默认即复位 + 8×8 瓦片 16B 差分拷贝置 64 位位图；intersect_ray = AABB 进出 t + 32 步行进 + 每步 3 次二分。

8. **water_rasterize_from_cpu 的脏协议精化**：读 WaterRasterStagingSsbo（type-id 0x1ad182936286a0ee）记录的 +0x10 即 WaterRaster 本体；`+0x29b` 脏时同步 u32 版本（SSBO+0x5c ← 库+0x5b0）、清 0x29b、memset(+0x98,0,0x200) 后跑 "water blur temp" 4 compute pass（含 /gauss_blur_mask.cs.hlsl；"Spawn lilypads/reeds" 两 pass 属 water_spawn_vegetation）。**+0x29b 的另一置位点仍 U**（write 只置 +0x299）。

9. **mouse_terrain_raycast 完整链 O**：CameraCursorRay 有效 → xz 平面解；否则 TerrainHeightsData::intersect_ray → **与 y=0 平面交点比距取近** → closure$0 实例网格精化。terrain_hover_idx：AppMode 掩码 0x421 门、SSBO（名 0x6c 字符）读 id@+0x10（-1 哨兵）、水位/光标/距离 0.5 门控。MouseDepthValue = {w@0, xyz@+8}，来自 2×float4 的 NaN 过滤回读（+0x40 帧哨兵）。

10. **补批新增 30 份反编译**（v2 增量：record_raster_history 0x140957990、delayed_terrain_changed_msg 0x140aabdd0、rasterize_terrain_stroke 0x1408c7120、read_terrain_data 0x14098c680、视觉实例等）+ 手解 12 个小函数（含 on_terrain_modified=`LOCK inc; ret`、DirtyTilePublisher::unique 38B、brush 6 函数）。字符串证据批量提取 152 行存 strings-extracted.txt（panic 串 10 条原文+地址，见 DIGEST §4）。
