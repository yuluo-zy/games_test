# WP7 — 水域内容域（water state / water_mask / water systems）

恢复 Tiny Glade **country-core 水域内容域**：CPU 侧水状态与 water_mask 资源、以及帧 pass
消费的水内容系统，按 PDB 源码骨架实现 Rust 并装配进 `reconstruction/country-core`。
版本 **B**（EXE cf048e27…，PDB 2d9686b8…）。

## 交付

- `contracts/water.md` — `resources::water` 模块（`RippleSourceSsbo`/`WaterDrawEvent`/
  `WaterEraseEvent`）+ 水域 CPU 状态本体（`WaterRaster` = `WorldRaster<512>` u8 栅格，
  water-cpu-analysis 布局 O 复核采纳）+ 表面变体（`GladeType`@+0x58==4 冬季）+
  `BrushSizes::water` + `startup_water`。
- `contracts/water-mask.md` — GPU 水 mask 纹理（512×512，`.y` 覆盖，>0.5 水/>0.2 clutter 阈值）、
  `WaterMaskSsbo`/`WaterExcludeClutterMaskSsbo`、更新流（CPU staging→mask 上传 W2、
  mask→CPU 回读 W5、GPU 笔刷 draw_water_mask）。
- `contracts/water-systems.md` — 水内容系统：`system_water` 链（WaterPlugin build、draw_water、
  draw_raster<Water>、record_water_history、replay_raster<Water>、update_water_audio_grid）、
  country-core 计算 pass（water_rasterize_from_cpu W2、water_spawn_vegetation W13、
  read_terrain_data 水回读 W5）、draw pass（water_planes_draw/lilypad/lilypad_flower/reed、
  water_surface_pass/ice_surface_pass W45）、表面选择（is_winter→ice）。
- `src/` — 骨架对齐源码（country-core 装配同步）：
  - `src/resources/{water, water_mask, water_exclude_clutter_mask}.rs` + 水域切片
    `session.rs`（GladeType is_*）/`brush_sizes.rs`（set/reset_water）
  - `src/systems/render_loop/compute_passes/{compute_water, spawn_boats, read_terrain_data}.rs`
  - `src/systems/render_loop/draw_passes/transparency_pass.rs` +
    `draw_passes/draw_shaders/water_planes_draw.rs`（新模块）
- `evidence.json` — 反编译地址、skeleton bundle SHA、候选裁决、装配补丁。
- `evidence/*.c` + job/result JSON — 本轮 15 个反编译 + 3 批 Ghidra 会话结果。

## 集成状态（reconstruction/country-core）

- 已声明：`resources::{water, water_mask, water_exclude_clutter_mask, session, brush_sizes}`；
  `draw_passes::draw_shaders`（含 water_planes_draw）。
- `resources::Water(pub Gap)` 占位移除（无引用），由 `resources::water::Water` 承载
  （持 `WaterRaster` 水位本体，O）。
- `world_raster_defs::WaterRasterStagingSsbo`（type-id 0x1ad182936286a0ee O）已加入。
- 复验：`cargo check --offline` **0 错误**（134 警告，骨架 marker）；
  `cargo clippy --all-targets --offline` **0 错误**。
  （本轮与 WP8 地形域并发装配；期间曾出现 WP8 未完成文件的瞬时编译错误，WP8 修复后
  恢复 0 错误，不属于 WP7 模块。）

## 本轮关键裁决

- `resources::water` 无独立 `water.rs` bundle——命名实体经 `resources/mod.rs.structures.json.gz`
  暴露；`RippleSourceSsbo`/`WaterDrawEvent`/`WaterEraseEvent` 均为 name-only。
- 水位 CPU 状态 = `WaterRaster`（`WorldRaster<512>`），`WaterPlugin::build`（0x1406501d0）
  `insert_resource` 注册；`draw_water→draw_raster<Water>→rasterize_stroke<512>→
  copy_to_staging` 与历史回放 `replay_raster<Water>` 复用同一链，末由 W2 上 GPU。
- 表面选择 O：`render_world` W45 按 `GladeType::is_winter`（0x140ae8320，与
  `is_corrupted` 同址折叠）派发 `water_surface_pass` ↔ `ice_surface_pass`。
- `water_spawn_vegetation`（W13）O：两个 ComputePass（lilypad+lilypad_flower / reed），
  `write_storage_buffer` + `DrawIndexedIndirectBuffer` 并发 + `is_autumn` uniform + dispatch；
  boats/waterwheels 为动态 URI/CPU 侧（U，spawn_boats type-only + GPU stub 空体佐证）。
- `read_terrain_data`（W5）水回读 O：`_water_read_water_mask.cs` 一次打包
  `WaterMaskSsbo`/`WaterExcludeClutterMaskSsbo`（>0.5/>0.2 阈值）。

## 剩余缺口（typed gaps，不臆造）

- `RippleSourceSsbo`/`WaterDrawEvent`/`WaterEraseEvent`/`WaterMaskSsbo`/
  `WaterExcludeClutterMaskSsbo`/`LilypadSsbo`/`LilypadFlowerSsbo`/`ReedSsbo`/`BoatSsbo`
  字段布局全部 U（PDB name-only）。
- `WorldRaster<512>` 栅格行数/总尺寸、水位 0..255 码值语义、0x4000B 初始填充角色。
- `ActiveSession` 布局、type-id 0x74430dfcc6d8719b 归属（I：ActiveSession）、
  `Time` type-id 0x4bc362195b80f729（I）；`GladeType` 判别值域（仅 4=Winter O）。
- spawn_boats/waterwheels 确切 CPU 位置（I）；纹理 preset 槽号 9/10/0x13/5/0x14/0x17/0x16/0x18/8
  映射（O 常量，语义 U）。
- `system_water` crate 系统（draw_water 等）业务逻辑已整理为契约，但**未装配进 country-core**
  （跨 crate 边界）；其 Rust 归属系统 crate 恢复域。
- 全部 `implemented_unverified`：无 runtime/GPU 验收；合同是当前可审阅依据。

## 方法

1. 业务逻辑分析：Ghidra 共享只读反编译（15 函数）+ 上一轮 `water-cpu-analysis.md`
   （WorldRaster/回放链，本包复核采纳）+ shader-source SPIR-V（mask/brush 阈值）。
2. 整理：contracts/*（Observed/Inferred/Unknown + typed gap）。
3. 骨架实现：src/*，ECS 用真实 bevy_ecs 0.16 API；真实可实现的 CPU 逻辑（`is_winter`、
   `set_water` clamp、mask 打包阈值、世界→栅格映射、表面派发）落为真实代码，
   rhapsody PassBuilder 接入边界按既有装配模式 `unimplemented!()` + O 文档。
4. 差分仅作关键数值/边界裁决（如 0x140ae8320 符号同址、dispatch 常量）；未做 oracle。
