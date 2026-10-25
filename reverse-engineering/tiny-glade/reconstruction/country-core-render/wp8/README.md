# WP8 — 地形内容域资源 + 地形系统（terrain_height / rocky_terrain / world_raster_defs / 地形系统）

恢复 `country_core` 的**地形内容域**（CPU 侧地形高度/岩质/世界光栅资源 + 帧 pass 消费的地形系统），
按 PDB 源码骨架实现 Rust，装配进 `reconstruction/country-core`。

## 交付

- `contracts/` — 业务逻辑契约（Observed/Inferred/Unknown + typed gap）：
  - `terrain-height.md` — `TerrainHeightsData<T>`（双实例化：`TerrainHeightSsbo`/`VisualTerrainHeightSsbo`）、
    方法清单（sample/intersect/subscribe/copy_from_gpu/as_raw/as_mut_raw/as_bytes/height_range）、消费者。
  - `rocky-terrain.md` — `RockyTerrainMesh::load`（文件→JSON→两 SSBO）+ `displace_rocky_terrain` 消费。
  - `world-raster-defs.md` — 命名世界光栅（Water/Path/Garden/Cobblestone1-3）+ staging SSBO 家族 +
    `AllWorldRasterRead`（原 `world_raster.rs` 内子模块；WP6 复验扩展）。
  - `terrain-systems.md` — 编辑输入（ui_edit/ui_draw/ui_focus/edit）、回读调度（draw_stroke/deferred/delayed）、
    鼠标射线（mouse/cursor/hover）+ compute 回读（read_terrain_data 等）、导数与光照
    （rasterize_terrain_stroke/displace/calc_terrain_lighting）——每系统含 O 级 ECS 签名。
- `src/` — 骨架对齐源码（与 country-core 装配同步）：
  - `src/resources/{terrain_height, rocky_terrain, world_raster_defs, terrain_stroke, terrain_editing,
    mouse_terrain_raycast, cursor_terrain_raycast, terrain_rock_mask, active_raster_stroke}.rs`
  - `src/systems/terrain/{mod, edit_terrain_stroke, draw_terrain_stroke, deferred_terrain_history,
    delayed_terrain_changed_msg, terrain_hover_idx, ui_draw_terrain, ui_edit_terrain,
    ui_terrain_focus_mode}.rs`
  - `src/systems/render_loop/compute_passes/{rasterize_terrain_stroke, displace_rocky_terrain,
    terrain_lighting_texture}.rs`（由 wp5 桩升级为带反编译结构的骨架）
  - `src/lib.rs` + `Cargo.toml` — 独立证据束 crate 根（镜像装配布局；`cargo check --offline` 0 错误）。
- `evidence.json` — 反编译地址、skeleton bundle SHA、候选复用裁决。

## 集成状态（reconstruction/country-core）

- `resources` 已注册：`terrain_height`/`rocky_terrain`/`terrain_stroke`/`terrain_editing`/
  `mouse_terrain_raycast`/`cursor_terrain_raycast`/`terrain_rock_mask`/`active_raster_stroke`/`world_raster_defs`。
- `systems::terrain` 已接线（`lib.rs` + `systems/mod.rs`）；三个 compute pass 文件已原位替换。
- `cargo check --offline`：**0 错误**（136 警告，均为骨架 marker/未使用）。

## 方法（按项目流程）

1. 业务逻辑分析：skeleton bundle（pdb.jsonl / structures.json.gz / enriched.json.gz 的 inlinee+行号、
   FunctionSystem 签名）+ 本轮 Ghidra 反编译（18 个资源/系统函数，worker shared，只读）。
2. 整理：contracts/*（O/I/U + typed gap）。
3. 骨架实现：src/*，未知字段 → typed gap；prior `reconstruction/terrain/crates/tg-terrain` 业务规则复用并复验。
4. 差分仅作关键数值/边界裁决（如 `TerrainStroke::insert_or_morph_last_point` 阈值 0.001/0x400 经
   0x140a97880 反编译与 prior 原机器码案例双证）。

## 本轮新发现

- **`TerrainHeightsData<T>` 双实例化**（O）：`insert_terrain_resources` 插入权威+视觉两个高度资源。
- **`CameraCursorRay` 布局 O**：`{tag, pos, dir, length}` 32B；`intersect_with_xz_plane` 浮点求交。
- **`ScheduledTerrainReadback` 布局部分 O**：counter@+0x00（atomic，on_terrain_modified）、flag@+0x08、
  delays Vec<u32>（延迟 2 帧回读）。
- **`read_terrain_data` 完整回拷链 O**：RasterPass 绘制 terrain/rocky → 高度/岩石/水/花园 SSBO 回拷 → flag 置位。
- **岩体掩码 512×512 位栅格 O**：`x/130+0.5` 世界→像素映射。
- **`rasterize_terrain_stroke` 结构 O**：active 笔画 16 位掩码迭代 + 16B 动态常量 + dispatch_with + 两遍 GaussBlur。

## 剩余缺口（typed gaps，不臆造）

- `TerrainHeightsData<T>`/`TerrainHeightFloatGrid`/`RockyTerrainMesh`/`WorldRaster<N>` 字段布局。
- `ui_edit_terrain`/`ui_draw_terrain`/`ui_terrain_focus_mode` 内部 UI 状态机。
- `intersect_ray` 求交数学、`resample_and_smooth`/`solidify_fluid_into` 算法。
- 各 SSBO 记录格式（`MouseTerrainRaycastSsbo`/`MouseDepthValueSsbo`/staging SSBO）。
- `TerrainStrokeParamsCache`/`MouseActivityTimer` 字段。

全部标记 `implemented_unverified`：无 runtime/GPU 验收；合同是当前可审阅依据。
