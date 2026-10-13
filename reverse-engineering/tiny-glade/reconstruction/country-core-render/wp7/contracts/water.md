# WP7 契约 1/3 — `resources::water` 模块与水域 CPU 状态

版本 **B**（`tiny-glade.exe` sha256 cf048e27…，PDB `tiny_glade.pdb` 2d9686b8…）。
数据源：skeleton `country-core/src/resources/{mod.rs, water_mask.rs, water_exclude_clutter_mask.rs}`、
`skeleton/B/crates/systems/water/src/*`、本包 Ghidra 反编译（`evidence/*.c`）、
`evidence/multi-baseline/water-cpu-analysis.md`（上一轮水域 CPU 状态链，本包复核采纳）、
`reconstruction/shader-source/shaders/`（水 shader 消费面）。
标注：**O**=反编译/PDB/SPIR-V 直接支持；**I**=调用链/名称组合；**U**=未恢复。

## 1. 模块边界与命名实体（O，`resources/mod.rs.structures.json.gz`）

`country_core::resources::water` 模块的 PDB 命名实体（均 `name_and_relations_only`，
**fields/size/alignment unknown**）：

| 实体 | 生产者（O） | 消费者（O） |
|---|---|---|
| `RippleSourceSsbo` | `system_critters::duck::update_duck_ripples`（0x2314b70，鸭子入水波纹，跨 crate） | `water_planes_draw`（0x1409d9bc0，`read_storage_buffer<RippleSourceSsbo>`）；`_water_plane.raster.hlsl` / `_waterfall.raster.hlsl`（`RippleSourceSsbo` / `ripple_source_buffer` 绑定，O：shader bin names） |
| `WaterDrawEvent` | `draw_raster<Water…>` 与 `replay_raster<Water…>` 写入（O：`EventWriter<WaterDrawEvent>` 系统签名） | 历史系统（`record_water_history`、`replay_water` 触发链）；`water_audio`（I） |
| `WaterEraseEvent` | 同上（O：`EventWriter` 签名） | 同上 |

生命周期（O，泛型胶水）：`AssetSsboLibrary::add_new/get_rg_buffer<RippleSourceSsbo>`、
`TypedStorageBuffer::new_storage_buffer<RippleSourceSsbo>`、`pass_resource_access::read_access_buffer<RippleSourceSsbo>`
（0x140970320）、`draw_builder::read_storage_buffer<RippleSourceSsbo>`（0x162a190）。
`WaterDrawEvent`/`WaterEraseEvent` 由 `system_water::impl$0::build`（0x1406501d0）`register_event`。

> typed gap：`RippleSourceSsbo`/`WaterDrawEvent`/`WaterEraseEvent` 字段布局全部 U；
> `WaterDrawEvent`/`WaterEraseEvent` 载荷（点位、笔刷、事件序号）U。

## 2. CPU 水位状态 = `WaterRaster`（`world_raster_defs`，O）

水位 CPU 状态本体是 **`WorldRaster<512>` 光栅**，以 `WaterRaster` 资源注册
（O：`system_water::impl$0::build` 0x1406501d0 `App::insert_resource<WaterRaster>()`）。
布局（O，`water-cpu-analysis.md` 第 2 节复核；本包未重新提取偏移读写）：

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x08 | composite 数据指针 | `copy_*/commit` 的 memcpy 偏移 |
| +0x10 | composite len（字节） | len 比对 |
| +0x20 | scratch 数据指针 | 同上 |
| +0x28 | scratch len | 同上 |
| +0x30..+0x5f | DirtyTiles/retain 闭包状态 + 16B 脏块元数据 | `retain` 参数 RDX |
| +0x60/+0x68 | parking_lot Mutex | lock_slow |
| +0x98..+0x297 | 512B staging/脏区位图（64 u64） | memset 0x200 / memcpy / OR |
| +0x298 | 读缓冲选择字节（0→composite，1→scratch） | CMOVNZ |
| +0x299 | staging 清零脏字节 | copy_to_staging 开头 |
| +0x29a | published 字节 | copy_to_staging 尾置 1 |
| +0x29b | 再上传脏字节 | `water_rasterize_from_cpu` 判 0 |

栅格单元：**u8、512 列**（行数≥513，O：`bilinear_sample_at_world_xz<RasterBufferRef<512>>`
0x140a14df0 行距 `SHL …,0x9`）。世界→栅格：`grid = (world/130.0 + 0.5) * 512.0`
（O：130.0 @0x142a7b658、0.5、512.0；窗口约 130 单位 → 512 格，原点对应格 256）。
`WorldRaster::new<512>`（0x140a15530）分配 composite+scratch，初始填 0x4000B 重复 u64 对
`{0x07954dbb_07954dbb, _UNK_142ac7e58}`——**初始水位值/解码语义 unknown**（近空水）。

> typed gap：栅格值 0..255 的水位语义（≤0.01 判定来自 preplaced_flower，见下）、
> 0x4000B 填充块的角色、栅格行数。

## 3. 水表面变体选择（O）

`GladeType`（`resources::session`，方法 0x140ae8320 is_winter / 0x14092e6c0 is_autumn /
0x140a9fa70 is_olden / 0x14092e6a0 is_flowery）：
- 会话资源 **字节 @+0x58** 承载当前湖面/草地类型；`== 4` 选冬季（O：`startup_water`
  0x1408f1a20 `if (*(char*)(lVar2+0x58) == 4)` 条件装载 `water_grass_winter`/冰面网格；
  `water_planes_draw` 0x1409d9bc0 `cVar1 == 4` 绑定雪面纹理）。
- `GladeType::is_winter`（0x140ae8320）为 `render_world` W45 的 **water_surface_pass ↔
  ice_surface_pass** 派发标志（O，frame-wiring §3）。该地址与 `BackupSaveFileKind::is_corrupted`
  **符号同址**（public-symbols.json：两 S_PUB32 同 VA 0x140ae8320，identical code folding），
  Ghidra 共享库默认取后名；`is_winter` 语义按 PDB 名保留。

> typed gap：`GladeType` 判别值域（仅 4=Winter O）；`ActiveSession` 布局。

## 4. 笔刷尺寸（O，BrushSizes::water）

`BrushSizes::set_water`（0x1409f18a0）：`self+4 = clamp(value, 1.0, 5.0)`（O：0x3f800000/0x40a00000）。
`BrushSizes::reset_water`（0x1409f1890）：`self+4 = 2.0`（0x40000000）。
水笔刷默认 2.0、上限 5.0。水位采样阈值常量（O，radare2 .rdata）：0.5 @0x142925870、
0.9 @0x142a7c88c、0.0005 @0x3a83126f。

## 5. startup_water（0x1408f1a20，O）——水面资源装载与实体 spawn

- `_DEV_OVERRIDE_IS_DEMO_MODE` 非 0 → 整函数 no-op（O）。
- 逐资源 `load_json_as_mesh<&str>` + `load_mesh_into_library` + `load_shader_into_library`；
  .rdata 字符串（O）：meshes `water_plane.json / lilypad.json / fallen_leaves.json /
  lilypad_flower.json / water_grass_winter.json / reed.json / reed_autumn.json /
  water_stones.json`；shader `water_plane.raster.hlsl / ice_plane.raster.hlsl /
  water_plane_refraction_copy.raster.hlsl / water_snow_plane.raster.hlsl / lilypad.raster.hlsl /
  reed.raster.hlsl / lilypad_flower.raster.hlsl / water_x_paths.raster.hlsl`（O，本包复核）。
- 每次装载后 `Commands::spawn_empty` + `insert<DrawableMeshBundle(+DontCastShadow,
  +DesignatedRenderPass)>`；命令槽字段（O）：transform scale=1.0、translation=0、
  mesh handle @+0x38/+0x40、render-pass 标签 u32 @+0x48（0xf=水/冰面、0xe=…）、
  byte@+0x58=0。`lVar2+0x58 == 4`（冬季）时额外 spawn 少组件变体（水面雪/冰）。
- **角色修正**：startup_water 是水网格/着色器装载 + 水面实体 spawn，**不是**水位 mask 的
  CPU 初始化（水位在 `WaterRaster`）。

## 6. 波纹消费面（O，shader bin）

`_water_plane.raster.hlsl` 绑定 `ViewConstants / frame_constants / $Globals / RippleSourceSsbo /
water_noise_texture / half_depth_tex / __alias_water_rt_count_buffer`（O：bin names）。
水 shader 家族（sequence.md，O）：`_water_deferred_rt.cs / _water_calc_deferred_rt_args.cs /
_water_draw_water_mask.cs`；冰：`_water_ice_{deferred_rt,refl_march,refl_resolve,refr_resolve}.cs`
+ `_ice_plane` + `_water_ice_cracks.raster`。`water_object_push.inc.hlsl`（O：嵌入 lilypad/
lilypad_flower/reed raster + leaf_particles）承载水植被实例 push 数据（布局 U）。

## 7. 集成裁决

- `resources::water` 命名实体为 name-only；Rust 侧以 typed-gap 载体 + 真实 ECS
  `Event`/`Resource` 呈现，`Water` 资源外观持有 `WaterRaster`（O 水位本体）。
- 上一轮 `water-cpu-analysis.md` 的 WorldRaster<512> 布局与 0x140a12680/0x141ba9130 反编译
  本包复核一致，直接采纳；不重做其已提取的偏移读写。
- 未取回作者原 Rust 文本；全部行为恢复以原机器码/PDB 为准，不臆造字段。
