# WP7 契约 2/3 — 水 mask 资源与更新流

版本 **B**。数据源：skeleton `country-core/src/resources/{water_mask.rs,
water_exclude_clutter_mask.rs}`（均 metadata-only，5 记录，无 procedure）、
`systems/render_loop/compute_passes/{compute_water.rs, read_terrain_data.rs}`、
本包 Ghidra 反编译（`0x140a12680.c` / `0x14098c680.c`）、
`reconstruction/shader-source/shaders/water/*.hlsl`（SPIR-V 恢复，O）。
标注：**O**/I/U 同上。

## 1. GPU 水 mask 纹理（O，SPIR-V）

`Texture2D<float4> water_mask`（register t14）**512×512**：
- `.y` 通道 = 水覆盖（O：`read_water_mask.cs.hlsl` `water_mask[coord].y > 0.5 → water bit`、
  `> 0.2 → clutter bit`）。
- `draw_water_mask.cs.hlsl`（O，算法 + 成员名）：`RWTexture2D<float4> out_water_mask`（u14），
  `$Globals{out_water_mask_dims, is_add, is_subtract, cursor_position_ws}`；
  `world_xz = ((pixel+0.5)/dims*2-1)*65.0`；`coverage = ((2-clamp(distance,0,2))*0.5)>0?1:0`；
  add → `max(prev, float4(cov,cov,cov,1))`；subtract → `max(prev.x - cov, 0)` 三通道。
  即 **GPU 笔刷** 按光标 add/subtract 涂改 mask。`65.0 = 世界半宽`（I）、笔刷半径 2.0（I）。
- `water_mask_sampler.inc.hlsl`（O）：`uv = world_xz * 0.0076923077 + 0.5`
  （=1/130 世界映射）；`get_water_mask` 取 `.y`。消费方（O）：`_path_pebbles /
  _rocky_terrain_stairs_stairs / _water_spawn_reeds / _water_spawn_lilypads /
  _paths_path_mask_visual / _garden_lowpoly_flower` 内联。

## 2. `WaterMaskSsbo` / `WaterExcludeClutterMaskSsbo`（O 名，布局 U）

`country_core::resources::water_mask::WaterMaskSsbo` 与
`…::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo`：PDB `name_and_relations_only`，
**fields/size/alignment unknown**。生命周期（O，泛型胶水）：`AssetSsboLibrary::add_new/get_rg_buffer`、
`TypedStorageBuffer::new_storage_buffer`、`ComputePass::write_storage_buffer`、
`pass_resource_access::write_access_buffer`（0x140974a20 WaterMask / 0x140974e20 ExcludeClutter）。

语义（I）：`read_water_mask.cs.hlsl` 以 32 texel/thread 打包 512 行 → `__alias_mask_out`（水 bits）
与 `__alias_mask_out_clutter`（clutter bits）两 RWStructuredBuffer<uint>；`WaterMaskSsbo` =
水面位打包回读缓冲，`WaterExcludeClutterMaskSsbo` = 排除 clutter 的 shoreline 位回读缓冲
（水域内不摆 clutter，`systems/clutter/add_lantern_water_setdresssing.rs` 相关）。

> typed gap：两者元素类型/条目数（候选 `[u32; …]`，512×512/32=8192 u32）、打包顺序、阈值
> 0.5/0.2 的精确比较（`>`）均为 O 常数但缓冲布局 U。

## 3. 更新流（O，反编译）

### 3a. CPU → GPU：`water_rasterize_from_cpu`（W2，0x140a12680）
1. 按 type-id `0x1ad182936286a0ee`（= `WaterRasterStagingSsbo`，`world_raster_defs`）查
   staging SSBO；存在才继续（否则 `unwrap_failed`，O）。
2. 若 `WaterRaster+0x29b` 脏：写 `ssbo+0x5c = world+0x5b0`、清 0x29b、`memset(0x98, 0, 0x200)`。
3. `Vec` 容量 0x200×2 + 纹理注册，建纹理并跑 **4 个 compute pass**（O：
   `FUN_140905260`/`FUN_1408edd00`/`FUN_1408ee240` 成对、`FUN_1409dd4d0` dispatch；
   dispatch 常量 `{1,2,2}`-状），即 `_water_draw_water_mask` 族（I：wp5 命名）。
4. 每 pass 含 `write_uniform<f32>` + `write_uniform<i32>`（O：paint/brush 参数）+ 纹理绑定。

即：**把 CPU 水位栅格（WorldRaster<512> staging 512B）上传成 512×512 水 mask 纹理**。

### 3b. GPU → CPU：`read_terrain_data`（W5，0x14098c680，水部分）
- `if (_DEV_OVERRIDE_IS_DEMO_MODE == 0)` 分支：ComputePass 绑 **水 mask 纹理槽 0x13**
  （`local_258._0_1_=0x13`），`write_storage_buffer<WaterMaskSsbo>` +
  `write_storage_buffer<WaterExcludeClutterMaskSsbo>`，dispatch（常量 `0x200000000001 /
  0x100000001` → 1D 回读，O）。即 **`_water_read_water_mask.cs` 一次回读打包两缓冲**。
- 同函数其他回读：地形高度（槽 9/10 → `TerrainHeightSsbo`/`VisualTerrainHeightSsbo`）、
  rock mask（槽 0xe → `TerrainRockMaskSsbo`）、garden（槽 0x14 → `GardenMaskSsbo`）。
- 尾部 type-id `0xfc8691076ab4d826` 资源 `LOCK`/`UNLOCK` + 置发布字节 =
  `ScheduledTerrainReadback::on_gpu_copy_issued`（O）。

### 3c. 帧 pass 消费（O，见 water-systems.md）
`water_surface_pass`/`ice_surface_pass`（W45）经 `_water_deferred_rt.cs`/`_water_draw_water_mask.cs`
读 mask；`_water_spawn_{lilypads,reeds}`（W13 计算）按 mask 覆盖度生成实例；
clutter 摆放排除用 `WaterExcludeClutterMask`。

## 4. 集成裁决

- 两个 `*Ssbo` 均无 procedure/函数，仅名字 + SSBO 注册/访问胶水；Rust 侧 typed-gap 载体 +
  `SssoType::rg_buffer_hash`（原值 = `utils::name_hash(<type path>)`，I）。
- 阈值 0.5/0.2、世界映射 1/130、65.0 半宽、512×512 为 O 常数，作为 const 落盘。
- 不做 oracle；不回填 `Vec::is_empty` 之外的守卫语义。
