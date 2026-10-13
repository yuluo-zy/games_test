# B_x_out：渲染依赖但在渲染目录之外的恢复

## 1. `utils::float_grid::bilinear_sample_at_world_xz`（utils/float_grid.rs）
- O：RasterBufferRef<512> 实例 0x140a14df0（835 B）；TerrainHeightFloatGrid 实例 0x140257ba0（798 B，同算法，格点经 `terrain_height::impl$7::index`，130.0 内联）。
- O 调用约定：`rcx=&grid, xmm1=x, xmm2=z, xmm3=extent_x, [rsp+0x20]=extent_z`。flower 的 5 个调用点都是 `0x43020000` 同时写 xmm3 和栈，
  即“多出的第 4 个参数”实为成对世界范围 (130,130)，不是半宽；`p=(z/ez, x/ex)+0.5`（divps，lane0=z/行，lane1=x/列）。
- O 算法：格点 `u8/255`，`idx=(row<<9)|col`，`lerp(lerp(A,C,fz), lerp(B,D,fz), fx)`，A=(r0,c0) B=(r0,c1) C=(r1,c0) D=(r1,c1)，`lerp=a+(b-a)*t`（0x140c904d0）。
  入口读全局字节开关 `*[0x14362c308]`（指向 .bss 0x143638fac）选路径：
  - !=0：`t=min(p,1)*512-0.5`，`max(t,0)`，`i=sat_u32`，`i0=min(i,511)`，`i1=min(i+1,511)`，`frac=t-i`；
  - ==0：`t=clamp01(p)*512`（NaN→0），`i=min(sat_u32(t),510)`，`i1=i+1`，`frac=t-i`（t=512 时 frac=2，原样）。
- U：开关的写入者未找到（全文反汇编只有经指针读取），.bss 初值 0；用 `float_grid::CENTERED_PATH: AtomicBool`（默认 false=向量路径）镜像，需后续核实。
- 新 API：`FloatGridCells` trait（`cell(idx)->f32`）、`bilinear_sample_at_world_xz(grid,x,z,extent_x,extent_z)`（旧签名 `(grid,x,z)` 的泛型 `T` 改为 `G: FloatGridCells`）、`bilinear_sample_at_world_xz_130`。
- `RasterBufferRef<N>`（resources/world_raster.rs）：旧 `{_gap}` → `{data: &'static [u8]}`（O：采样函数 `[rcx]` 取首 qword，flower 里由 `WorldRaster` 双缓冲按 `+0x298` 字节选 (ptr,len) 构造）；`impl FloatGridCells for RasterBufferRef<512>`（`u8/255`）及 `.bilinear_sample_at_world_xz()` 方法。
- 调用点改动（render/nani/systems.rs，仅 nani_update_preplaced_flower 的 5 个调用）：补第 4/5 参数 `FLOWER_WORLD_SIZE, FLOWER_WORLD_SIZE`；顺带更新其文档注释；对该文件跑了 rustfmt。
- 未做：`TerrainHeightFloatGrid` 的 `FloatGridCells` 实现（其布局为 U，0x1408b54c0 的 index 未恢复）；`TerrainHeightsData::sample_at_world_xz` 仍是 `unimplemented!`（terrain_height.rs 非我拥有）。

## 2. `terrain_rock_mask_words`
- O：flower 里 `FrozenMap::get(SsboId)` → `assert_eq!(name, "country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo")`（memcmp 0x3f）→ `BufferState.data` vtable `+0x18` → 返回 `Option<&(ptr,len)>` 的 u32 影子数组（NULL → unwrap panic）→ `(v<<9|u)>>5 < len` 边界检查 + `bt`。
- 归属：访问器是 `AssetSsboLibrary::get_cpu_shadow_words::<T>()`（asset_libraries/ssbo_library.rs，追加）；`Ssbo` trait 追加带默认 `None` 的方法 `cpu_shadow_words()`（槽 +0x18，序号 I）；`impl Ssbo for ShadowedBuffer<T>`（4 字节元素返回 `Some`，I：存储类型为 ShadowedBuffer）。
- `TerrainRockMaskSsbo`（resources/terrain_rock_mask.rs）：unit struct → `#[repr(transparent)] struct(u32)`，实现 `StorageElement`（NAME，BUFFER_LEN=0x2000）和 `SsboType`（Storage=ShadowedBuffer）。
- `systems.rs::terrain_rock_mask_words` 现为一行转发；SSBO 已注册时 flower 不再 unwrap panic，未注册仍 panic（与原版同）。
- 注意：A3 的 `AssetSsboLibrary` 不是本任务指定文件，我只做了追加（trait 默认方法、`get_cpu_shadow_words`、一个 impl），请集成者核对。`TerrainRockMask::sample_at_world_xz`（terrain_rock_mask.rs，仍 `Gap`+返回 false）未动，其 0x1409ea800 反编译已在 doc 里，是遗留。

## 3. 重复占位清理
- `LutGalleryState`：resources/grading_lut.rs 的旧 0x40 B 全零占位删除，改 `pub use systems::render_loop::lut_gallery::LutGalleryState`；lib.rs:556 的 `consume::core` 重导出路径不变，无需改调用方。
- `PrefabName/PrefabNameRef`：resources/prefab/mod.rs 的两份借用视图占位与 `OwnedPrefabName` 删除，改 re-export `resources::render::mesh_atlas_library::{PrefabName(Arc<str>), PrefabNameRef(str)}`（O：旧占位的 “16B header + as_name_ref=+16” 即 Arc<str> 的 strong/weak 头）。`PrefabLibrary` 键改 `PrefabName`，`add/get/has` 签名去掉生命周期；新增 `hashbrown::Equivalent<PrefabName> for PrefabNameRef` 使 `has(&PrefabNameRef)` 为 hash 查找。crate 内无其他使用者。

## 4. utils/ 其余桩扫描（未做，列出）
todo!/Gap：`load_json.rs`（read_attribute_from_json 0x14083e210、load_json_as_mesh 0x140b0eeb0，68 调用方，网格启动加载，渲染相关优先级高）、
`dirty_tiles.rs`（cell_from_world_space/rasterize_*/retain/dilate，栅格上传相关）、`debug_viz.rs`、`geo.rs`（mesh_from_pts/update_curve_mesh/rewire_enclosed_curve）、
`project_cursor_onto_ray.rs`、`child_wall.rs`、`components/transform.rs`（look_at/abs_diff_eq）、`utils/mod.rs`（get_res/drain_events 等，O 内联）、`curve.rs`/`comparative_space.rs`（Gap）。
另 resources/grading_lut.rs 里 GradingLut/GradingLuts::load/load_cube 等仍 `unimplemented!`（未在本任务范围）。

## 测试
隔离台 /tmp/h_x_out（harness 副本：补 typed_gap、Gap 派生、fnv、hashbrown、GpuWorldSubsetKind::Canopy，挂 float_grid / world_raster / prefab / terrain_rock_mask）：277 passed, 0 failed。
float_grid 8 项（两条路径、行列轴、510 夹紧 frac=2、NaN、130 范围参与）；terrain_rock_mask 3 项（影子读取、未注册 None、类型名）；prefab 1 项；world_raster 1 项。
整个 crate 因离线缺 `dolly` 无法 `cargo check`，lut_gallery re-export 与 systems.rs 调用点未经编译验证（仅 rustfmt 通过）。
