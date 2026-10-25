# crates/country-core/src/resources

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

地形、水、树存储、critters/window lights等跨系统资源有局部字段证据；不是整个resources目录恢复。

## 接线与验证

接线：not_established。

水模块候选9测试；其他资源主要分析或缺失交付记录。

## 已恢复的函数和类型范围

- TreeStorage get/insert/remove与next_index
- WorldRaster<Water> CPU状态与格点映射
- TerrainHeightsData双实例字段分析

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| WindowSize::aspect_ratio | [reconstruction/math/window_aspect.rs](../../reconstruction/math/window_aspect.rs)：`aspect_ratio_observed` | B：1280输入、原20字节完整局部f32结果；不证明完整WindowSize布局 | reported_verified_local：[evidence/native/window-aspect-verification.json](../../evidence/native/window-aspect-verification.json) |

## 明确缺口

- 每个资源精确字段/原类型和生产者仍待对齐
- WP7–10 country-core消费资源源码本机缺失

下一闭包：每个资源精确字段/原类型和生产者仍待对齐

## 证据

- [evidence/multi-baseline/README.md](../../evidence/multi-baseline/README.md)
- PROJECT_MEMORY.md

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/resources/active_raster_stroke.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/audio_resources.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/audio_settings.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/brush_sizes.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/clutter_diagnostics.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/clutter_meshes.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/clutter_packs.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/clutter_registry.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/color_icon.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/color_id.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/color_id_roof.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/compute_mouse_depth_value.rs` | A,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/critters.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/cursor_terrain_raycast.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/daily.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/drawing_input_smoother.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/editing_highlight.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/feedback_report_resources.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/garden_mask.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/glade_border.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/glade_color_customization.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/glade_settings.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/gothic_window_profile.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/gpu_lut.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/grading_lut.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/gui.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/image_formation.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/in_progress_curve.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/loading_screen.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/logo_animation.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/mod_status.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/mouse_capture.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/mouse_terrain_raycast.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/onboarding.rs` | A,B,C | [skeleton/task-index/work/TG-ab43622efad901bc85/abc-tools-52/crates/country-core/src/resources/onboarding.rs](../../skeleton/task-index/work/TG-ab43622efad901bc85/abc-tools-52/crates/country-core/src/resources/onboarding.rs)：explicit_candidate_delivery_mapping | country_core::resources::onboarding::WelcomeScreenState::is_closed（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/photo_mode.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/placement_data.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/plant_sample_raster.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/platform.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/prefab.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/preplaced_autoclutter.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/preplaced_clutter.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/raster_stroke.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/read_mouse_depth_value.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/rocky_terrain.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/session.rs` | A,B,C | [skeleton/task-index/work/TG-a4422b4c9680d489d4/abc-tools-52/crates/country-core/src/resources/session.rs](../../skeleton/task-index/work/TG-a4422b4c9680d489d4/abc-tools-52/crates/country-core/src/resources/session.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-a4422b4c9680d489d4/abc-tools-52/crates/country-core/src/resources/session.rs](../../skeleton/task-index/work/TG-a4422b4c9680d489d4/abc-tools-52/crates/country-core/src/resources/session.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-a4422b4c9680d489d4/abc-tools-52/crates/country-core/src/resources/session.rs](../../skeleton/task-index/work/TG-a4422b4c9680d489d4/abc-tools-52/crates/country-core/src/resources/session.rs)：explicit_candidate_delivery_mapping | country_core::resources::session::GladeType::is_flowery（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::session::GladeType::is_autumn（implemented_unverified；业务等价以限定证据为准）<br>country_core::resources::session::GladeType::is_olden（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/shader_refactoring_state.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/shape_intersections.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/shutdown.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/sparkle.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/stairs_walls.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/steam.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/stone_foundation_frames.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/terrain_editing.rs` | A,B,C | [reconstruction/terrain/crates/tg-terrain/src/stroke.rs](../../reconstruction/terrain/crates/tg-terrain/src/stroke.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/terrain_height.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/terrain_rock_mask.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/terrain_stroke.rs` | A,B,C | [reconstruction/terrain/crates/tg-terrain/src/schema.rs](../../reconstruction/terrain/crates/tg-terrain/src/schema.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/texture_presets.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/time.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/time_of_day.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/toolbar.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/tree_storage.rs` | A,B,C | [reconstruction/tree-tool/src/process_tree_cmds.rs](../../reconstruction/tree-tool/src/process_tree_cmds.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/country-core/src/resources/ui_settings.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/water_exclude_clutter_mask.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/water_mask.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/weather.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/window.rs` | A,B,C | [reconstruction/math/window_aspect.rs](../../reconstruction/math/window_aspect.rs)：explicit_scoped_function_binding | WindowSize::aspect_ratio（reported_verified_local；业务等价以限定证据为准） | candidate_present；WindowSize::aspect_ratio：1280输入、原20字节完整局部f32结果；不证明完整WindowSize布局：[evidence/native/window-aspect-verification.json](../../evidence/native/window-aspect-verification.json) |
| `crates/country-core/src/resources/window_lights.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/world_raster.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/resources/world_raster_defs.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/math/window_aspect.rs

[reconstruction/math/window_aspect.rs](../../reconstruction/math/window_aspect.rs)

SHA256：`2dea9bca049c0a6dfb8f6df8b786fe8de5306fd5ecd500d17e2ce6889a3c0b6f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `aspect_ratio_observed` | 9 | 未由扫描证明；以证据限定范围为准 |
| `main` | 14 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/terrain/crates/tg-terrain/src/schema.rs

[reconstruction/terrain/crates/tg-terrain/src/schema.rs](../../reconstruction/terrain/crates/tg-terrain/src/schema.rs)

SHA256：`dfc33b570017d6f6a65a093cd50746d80242f046e66c2219e10114e2c206ce27`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/terrain/crates/tg-terrain/src/stroke.rs

[reconstruction/terrain/crates/tg-terrain/src/stroke.rs](../../reconstruction/terrain/crates/tg-terrain/src/stroke.rs)

SHA256：`1f2fc447052acd866e4ed743fb72170a55987d383ca175786a764006c38e5994`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `distance` | 20 | 未由扫描证明；以证据限定范围为准 |
| `insert_or_morph_last_point` | 25 | 未由扫描证明；以证据限定范围为准 |
| `new_stroke` | 57 | 未由扫描证明；以证据限定范围为准 |
| `get_last_mut_stroke` | 65 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/terrain/crates/tg-terrain/src/water.rs

[reconstruction/terrain/crates/tg-terrain/src/water.rs](../../reconstruction/terrain/crates/tg-terrain/src/water.rs)

SHA256：`e4bcdf13f176d061d64a7644aa6f595f0d0e43f512fd8d4fa802a8ca39fc43d9`；缺口注释标记数：10。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `point_to_grid_xz` | 44 | 未由扫描证明；以证据限定范围为准 |
| `build_stroke_commands` | 63 | 未由扫描证明；以证据限定范围为准 |
| `new` | 104 | 未由扫描证明；以证据限定范围为准 |
| `copy_composite_to_scratch` | 119 | 未由扫描证明；以证据限定范围为准 |
| `commit_scratch_to_composite` | 126 | 未由扫描证明；以证据限定范围为准 |
| `copy_to_staging` | 133 | 未由扫描证明；以证据限定范围为准 |
| `bilinear_sample_at_world_xz` | 148 | 未由扫描证明；以证据限定范围为准 |
| `replay_restore_water` | 179 | 未由扫描证明；以证据限定范围为准 |
| `select_write_mode` | 222 | 未由扫描证明；以证据限定范围为准 |
| `coverage_to_alpha` | 232 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：//! 只恢复已证实的水域 CPU 状态链业务规则；所有未知项以 `// [unknown]` 标注，不伪造。
- L31：/// `WorldRaster::new` 填一块 0x4000B 初始块（初始水位），解码语义 [unknown]。
- L54：/// [unknown] 后 16B 字段语义（zeno 依赖，未还原）。
- L91：/// [unknown] 栅格行数/总尺寸；composite/scratch 以 Vec<Cell> 建模仅为近似（布局非 ABI 复刻）。
- L99：/// 512B staging（与脏块共用区，见证据）；[unknown] 精确语义。
- L132：/// [unknown] 原实现用 DirtyTiles::retain 增量复制 + parking_lot Mutex；这里只表达语义。
- L178：/// [unknown]：权重 fVar5 与栅格 u8 值的具体换算；Curve/zeno 依赖实现不在此恢复。
- L186：// [unknown] 实际以闭包经 zeno::mask::Mask::blend_into 写入栅格。
- L189：// [unknown] brush 来自 ActiveRasterStroke/BrushSizes。
- L210：///   `a = 覆盖度>>9` clamp 0xff（zeno 覆盖度定点刻度 [unknown]）。
### reconstruction/tree-tool/src/process_tree_cmds.rs

[reconstruction/tree-tool/src/process_tree_cmds.rs](../../reconstruction/tree-tool/src/process_tree_cmds.rs)

SHA256：`68b62d8df469902c146ca9fd016ce220d35dafadfbbc40c0c64f3dc4f0c60a85`；缺口注释标记数：16。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `next_index` | 46 | 未由扫描证明；以证据限定范围为准 |
| `insert` | 52 | 未由扫描证明；以证据限定范围为准 |
| `get` | 57 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 60 | 未由扫描证明；以证据限定范围为准 |
| `process_tree_cmds` | 191 | 未由扫描证明；以证据限定范围为准 |
| `water_sample` | 274 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：//!   `unknown`。未确认字段保留 `unknown`，不伪造。
- L92：/// f32 @+0x20（unknown 语义）。
- L96：/// byte @+0x28（spawn 门控输入；unknown 语义）。
- L100：/// @+0x38..0x40 未读字段（unknown）。
- L107：/// 证据：position Vec3 @+0（12B）；f32 @+0xc（inverse_lerp 读取，unknown 语义）。
- L111：/// f32 @+0xc（unknown，疑似水面/高度缓存）。
- L118：/// 其余 unknown。
- L122：/// u64 @+0x10（unknown）。
- L126：/// f32 @+0x24（unknown）。
- L128：/// f32 @+0x28（unknown）。
- L132：/// f32 @+0x30（=1.0，unknown）。
- L134：/// 其余 @+0x34..0x48（unknown）。
- L141：/// 值（unknown：疑似树索引/实体；Insert 分支由 TreeStorage::insert 返回值写入）。
- L167：/// is_corrupted 方法门控种树。对象实际 Rust 类型 unknown。
- L169：/// [evidence] +0x58 处会话状态对象（内部布局 unknown）。
- L173：/// ActiveSession[+0x58] 处对象（仅确认三个查询方法行为，内部 unknown）。
### skeleton/task-index/work/TG-a4422b4c9680d489d4/abc-tools-52/crates/country-core/src/resources/session.rs

[skeleton/task-index/work/TG-a4422b4c9680d489d4/abc-tools-52/crates/country-core/src/resources/session.rs](../../skeleton/task-index/work/TG-a4422b4c9680d489d4/abc-tools-52/crates/country-core/src/resources/session.rs)

SHA256：`90899f00767eb462a9d6e7e4b9b434b52b159eb07d01ebed1f4ef9952b81dbcd`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_flowery` | 15 | 未由扫描证明；以证据限定范围为准 |
| `is_autumn` | 22 | 未由扫描证明；以证据限定范围为准 |
| `is_olden` | 29 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-ab43622efad901bc85/abc-tools-52/crates/country-core/src/resources/onboarding.rs

[skeleton/task-index/work/TG-ab43622efad901bc85/abc-tools-52/crates/country-core/src/resources/onboarding.rs](../../skeleton/task-index/work/TG-ab43622efad901bc85/abc-tools-52/crates/country-core/src/resources/onboarding.rs)

SHA256：`60ddc40a5784fb9fbcafe7e7b2630dbce81a179282e5574468f892a1c6ee3bbd`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_closed` | 14 | 未由扫描证明；以证据限定范围为准 |
