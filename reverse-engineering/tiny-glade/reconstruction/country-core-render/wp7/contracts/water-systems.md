# WP7 契约 3/3 — 水内容系统（状态更新 / mask 栅格化 / spawn / 表面选择）

版本 **B**。数据源：本包 Ghidra 反编译（`evidence/0x*.c`）、
skeleton `systems/water/src/*` + `country-core/src/systems/render_loop/{compute_passes,
draw_passes}/*`、`evidence/render-passes/cpu-frame/sequence.md`、`frame-wiring.md`（wp5）。
标注：**O**/I/U 同上。

## 1. `system_water` crate（独立 crate，跨包消费者）

`system_water::impl$0::build`（0x1406501d0，O）：
- `add_systems<Update>`：`record_water_history`（参数
  `RasterHistoryRecordResources, EventReader<RecordWaterEditToHistoryCmd>`，`in_set<GameSystemSet>`）、
  `update_water_audio_grid`（参数 `Local<WaterAudioGridUpdateState>, Res<WaterRaster>,
  Raycasting, Res<MainCamera>, ResMut<WaterAudioGrid>`，`run_if(bool(*)(Option<Res<WaterAudioGrid>>))`）。
- `App::insert_resource<WaterRaster>()`（水位本体注册，O）。
- `register_event<RecordWaterEditToHistoryCmd / WaterDrawEvent / WaterEraseEvent>`（O）。

### 1a. `draw_water`（0x141ba9130，O）——实时水面编辑入口（system_water）
- demo 守卫；读工具 id（`*RCX` u32）：`id-5 ∈ {4,10}` 分支
  - `id==9`（eraser）：`inverse_lerp` 半径 → `fVar11*4+(1-fVar11)`、`UiState::clearing_canvas`、
    `CursorTerrainRaycast::position`、`CameraCursorRay::intersect_with_xz_plane`、
    `SignifierFactory::new` + `UiSignifierStream::write`（"eraser"）。
  - `id==15`（水）：`BrushSizes.water`（self+4，clamp[1,5]，默认 2.0）、`clearing_canvas`、
    `CursorTerrainRaycast::position`、`Circle2d::new`、`GladeBorder::is_shape_inside`（界内校验）；
    界内写 `WriteSettingsFileCmd` 事件 + `DragState::is_some`。
- 公共：`DrawState::update` + 状态机 switch（case 0/1/2/3，含 `ShowPinnedDecoClutter` 事件写入）、
  `WriteSettingsFileCmd` 事件、`ScheduledTerrainReadback::on_terrain_modified`、
  `TweakerView::as_inner_mut`（取笔刷大小）→ **`draw_raster<WaterRasterStagingSsbo,
  WorldRaster<512>, closure_env$1/2/3>`**（0x142260ae0，O）。

### 1b. `draw_raster<Water>`（0x142260ae0，O，size 2214）——实时链编排
`ActiveRasterStroke::index_mut` → 状态 0：`start_new`；状态 1：`InProgressRasterStroke::last`
距离 > ~0.0005 → `add_point` + `resample_and_smooth` + `DirtyTiles::clear` +
`iter_points_xz` + `rasterize_world_space_stroke` + `DirtyTiles::extend`×2；交换双份 512B 脏块位图；
`copy_composite_to_scratch<512>` → `iter_points_xz` → **`rasterize_stroke<512>`**
（0x142261d10，O：世界→栅格 `/130+0.5)*512`，生成 zeno Command 列表，
按 bit0/byte@0x30 选 blend 闭包 $0/1/2，调 `zeno::mask::Mask::blend_into`）→
SSBO lookup（WaterRasterStagingSsbo）→ map 写 → `copy_to_staging<512>` → 事件 + `WriteSettingsFileCmd`。

### 1c. `record_water_history`（内联进 Bevy FunctionSystem，U 独立地址）
由 build 注册参数 `(RasterHistoryRecordResources, EventReader<RecordWaterEditToHistoryCmd>)` 佐证；
把实时水编辑记录为存档历史（`WaterDrawEvent`/`WaterEraseEvent` → `RecordWaterEditToHistoryCmd`）。

### 1d. 历史回放（O）
- `replay_raster<WaterRasterStagingSsbo, WorldRaster<512>, closure_env$2>`（0x1409f4650）：
  系统参数 `RasterReplayResources, EventWriter<WaterDrawEvent>, ResMut<WaterRaster>,
  EventWriter<RecalculateHeightMapCmd>, Res<ScheduledTerrainReadback>`。迭代 `Vec<ReplayCmd>`
  （48B/条 = 点列+标志+权重 f32@+0x14+标记字节）；清除标记 → `clear_buffer<512>` 后重放嵌套命令；
  普通记录 → `Curve::try_new_from_points` + 重采样 + `copy_composite_to_scratch` +
  `rasterize_stroke<512>` + 脏块位图 OR（+0x98..0x290）+ SSBO lookup + `copy_to_staging<512>`；
  权重≥0.01 进 blend 分支（`new=brush*scale+old`，归一，`≥1.0` 时 `commit_scratch_to_composite`）；
  处理完 memmove 移除已处理条。**水位恢复 = 把存档 ReplayCmd 再跑一遍同一
  rasterize→staging→composite 链**，末由 `water_rasterize_from_cpu` 上 GPU。
- `generate_replay_cmds_from_edit<replay_water>`（0x141444c70，O：60B 跳表 stub，主体 10371B 内联）
  按 `EditType` 跳表分发；调用方 `call_once<replay_water::closure_env$1,(EditType,&mut Vec<ReplayCmd>)>`
  （0x14083c110）。
- `replay_water::closure$2`（0x140a01560，O，191B）：push 两个 Vec + LOCK 原子计数自增
  （"回放完成"通知/计数闭包）。

### 1e. `update_water_audio_grid`（0x141baefe0，O）
`Local<WaterAudioGridUpdateState> + Res<WaterRaster> + Raycasting + Res<MainCamera> +
ResMut<WaterAudioGrid>`。按相机 `eye_position` 遍历 24B/格 的网格条目：
距离² < 6400.0 且 `bilinear_sample_at_world_xz<RasterBufferRef<512>>` > 0.5 且
`Raycasting::intersects_any` 为真 → 置位 water bit（+8，1<<(frame&0x3f)）并写 2×i16 网格坐标；
否则清位。纯 CPU 音频网格维护。

### 1f. `WaterAudioState` / `WaterAmbienceAudio` / `WaterAudioGrid`（country-core
`systems/audio/{water_audio, water_ambience_audio}.rs`，name-only，U）
`water_audio_system`（0x140873630）/`water_ambience_audio_system`（0x1409ecea0）消费 `WaterRaster`
水位做音频混合；字段 U。

## 2. country-core 帧计算 pass

### 2a. `water_rasterize_from_cpu`（W2，0x140a12680）——见 water-mask.md §3a（CPU staging→GPU mask）。

### 2b. `water_spawn_vegetation`（W13，0x140a13970，O，size 2269）
demo 守卫 → 两个 ComputePass：
- **Pass 1（lilypad + lilypad_flower）**：`ComputePass::new`（URI `/water/spawn_lilypads…`，
  I）+ `write_storage_buffer<LilypadSsbo>` + `write_storage_buffer<LilypadFlowerSsbo>`；
  资源 type-id `0x74430dfcc6d8719b`（= `ActiveSession`，I）`unwrap_failed` 断言存在；
  `write_uniform<bool>`（= `ActiveSession::is_autumn()`，O：wp5）+ 纹理绑定
  （preset 槽 9, 10, 0x13, 5, 0x14, 0x17——含水 mask/地形高度/autumn 纹理，I）+ 
  `write_storage_buffer_concurrent<DrawIndexedIndirectBuffer>` + `write_uniform<i32>` +
  `PassBuilder::render`（dispatch 常量 `0xe100000001/0x1000000e1`，O）。
- **Pass 2（reed）**：`write_storage_buffer<ReedSsbo>` + 纹理槽 9, 10, 0x13, 5, 0x16 +
  `write_storage_buffer_concurrent<DrawIndexedIndirectBuffer>` + `write_uniform<i32>` + render
  （dispatch `0x9600000001/0x100000096`，O）。

即：**按水 mask 覆盖度 + autumn 标志，GPU 生成 lilypad/lilypad_flower/reed 实例并写间接绘制参数**。
boats/waterwheels 不在本函数（U：见 2c）。

### 2c. `spawn_boats`（type-only，U）
`spawn_boats.rs` 仅 `BoatSsbo`（O 名，布局 U；0 个 procedure，5 条 metadata）。
`/water/spawn_boats.cs.hlsl` URI 在 255 静态 URI 中**缺失**（O）→ 运行时动态拼接或并入
`water_spawn_vegetation`（I，wp5 记 dispatch 并入）。BoatSsbo 生命周期：`read_access_buffer`
（0x14096fe20）/`add_new`（0x1412c23d0）/`get_rg_buffer`（0x1412d3af0）/`DrawBuilder::read_storage_buffer`
（0x141631690）。`spawn_waterwheels.cs.hlsl` GPU stub 为空体（O，SPIR-V）→ **waterwheel spawn 在 CPU 侧**（I）。

### 2d. `read_terrain_data`（W5）水回读——见 water-mask.md §3b。

## 3. country-core 帧 draw pass

### 3a. `water_planes_draw`（0x1409d9bc0，O，size 1522）
demo 守卫；资源 type-id `0x74430dfcc6d8719b`（`ActiveSession`，I）读 **字节@+0x58** 为
水面变体；`DrawBuilder::new` + `bind_common` + 纹理绑定（preset 槽 9, 10, 0x13, 0x18, 8；
`TextureFromDisk 0x579f39ab91a01865`=water_noise_texture，O）+ 
`read_storage_buffer<RippleSourceSsbo>` + `read_storage_buffer<DofAutofocusSsbo>` +
`write_uniform<Vec3>`（water 颜色/参数，O）+ `bind_ray_tracing`；`cVar1==4`（冬季）追加
`TextureFromDisk 0xc555853f6bd247ac`（雪/深度纹理）；`DrawBuilder::draw`。

### 3b. 水植被 draw（O，各 ~250-330B）
- `lilypad_draw`（0x1409da580）：`read_storage_buffer<LilypadSsbo>` + preset 9, 10 +
  `bind_water_object_push`（`common_includes::impl$2`，水植被 push 常量）+ `TextureFromDisk
  0x4d64746dbda554d2` + `multi_draw_indexed_indirect`。
- `lilypad_flower_draw`（0x1409da760）：`read_storage_buffer<LilypadFlowerSsbo>` +
  preset 9, 10 + `bind_water_object_push` + `TextureFromDisk 0x9bbd5c16bda1314c` +
  `multi_draw_indexed_indirect`。
- `reed_draw`（0x1409da940）：`read_storage_buffer<ReedSsbo>` + preset 9, 10 +
  `bind_water_object_push` + `multi_draw_indexed_indirect`。

### 3c. `water_surface_pass`（W45 非冬，0x1409bf800，O，size 3099）
1. `create_raw_resource` ×2：buffer（`new_gpu_only`，名 `DAT_142abfbf1`@0x15）+ texture
   （`extent_2d`，名 `DAT_142abfc06`@0x13）→ `clear_rg_buffer`。
2. `RasterPass::new_impl` + 资源 type-id `0x4bc362195b80f729`（= `Time`，I）读
   `elapsed = (int)secs + nanos/1e9`；遍历 draw stream（0x80B/条）经 0x70 跳表闭包逐条
   `DrawBuilder::new/bind_common/bind_texture×N/write_rg_storage_buffer×2/write_uniform<i32>`；
   `RasterPass::render`。
3. ComputePass：`write_rg_storage_buffer` + dispatch_1d（常量 `{1,1,1}`）= `_water_calc_deferred_rt_args.cs`。
4. ComputePass：`read_rg_storage_buffer`×2 + 纹理绑定（preset 9/10/8 + `TextureFromDisk`
   0x579f39ab91a01865 + `TextureFromDisk` 0x579f39ab91a01865 槽）+ `write_uniform<i32>` +
   `bind_ray_tracing` + `dispatch_indirect_raw` = `_water_deferred_rt.cs`（软件 BVH）。

### 3d. `ice_surface_pass`（W45 冬季，0x1409c08d0，O，size 8874）
同水 + 更多 compute：`read_rg_storage_buffer`×2 + `bind_ray_tracing` + `dispatch_indirect_raw`
= `_water_ice_{refl_march,refl_resolve,refr_resolve,deferred_rt}.cs`；`write_uniform<bool>`（雪/冰标志，
O：0x1409c08d0.c:429）+ `write_uniform<i32>`；`RasterPass` 跑 `_ice_plane` + `_water_ice_cracks.raster`
（ice cracks draw）。纹理绑定含 shadowmap/Temporal 读回（I）。

### 3e. W44/W46/W53 相关（water-plane 邻接）
`refraction_copy_pass`（0x1409bf320，门 `\*0x14362c098 == 0`）：`_water_plane_refraction_copy.raster`
+ `_waterfall_refraction_copy.raster`；`pre_taa_transparency_pass`（0x1409c3a80）/
`post_taa_transparency_pass`（0x1409c40b0）：透明/玻璃。

## 4. 表面选择（O）

`render_world`（0x1404ec110）W45：`\*0x14362c098 == 0` 门 折射+水面；
`GladeType::is_winter`（0x140ae8320）派发 **water_surface_pass ↔ ice_surface_pass**。
`water_planes_draw` 以会话字节@+0x58==4 选雪面变体。`_DEV_OVERRIDE_IS_DEMO_MODE` 全局门
作用于 startup/water_rasterize/water_spawn/water_planes（O）。

## 5. typed gap 汇总

- `LilypadSsbo / LilypadFlowerSsbo / ReedSsbo / BoatSsbo / RippleSourceSsbo /
  WaterMaskSsbo / WaterExcludeClutterMaskSsbo` 字段布局 U（name-only）。
- `ActiveSession` 布局、type-id `0x74430dfcc6d8719b` 归属（I：ActiveSession）、
  `Time` type-id `0x4bc362195b80f729`（I）。
- spawn_boats/waterwheels 的确切 CPU 位置（I：并入 water_spawn_vegetation 或动态 URI）。
- 纹理 preset 槽号 9/10/0x13/5/0x14/0x17/0x16/0x18/8 的具体语义（O 常量，映射 U）。
- `dispatch_indirect_raw` / `multi_draw_indexed_indirect` 的间接参数来源（DrawIndexedIndirectBuffer，
  O 写入方 = spawn passes）。
- 全局 `\*0x14362c098` / `\*0x14362c0d8` 语义 U（frame-wiring 已有记录）。
