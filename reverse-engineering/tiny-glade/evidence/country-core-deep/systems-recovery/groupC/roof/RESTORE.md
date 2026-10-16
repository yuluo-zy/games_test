# RESTORE — system-roof（组 C restorer，2026-10-16）

## 交付摘要

- **落点**：`tiny-glade/crates/systems/roof/`（crate `system-roof`），33 业务全对齐 + 注册 + 集成测试。
- **编译**：`cargo check --workspace` 零错误（country-core / system-roof / tg-building-runtime /
  system-wall-constructor 全部通过；system-color 为组 A 在途 crate，其剩余错误均为组 A 自己的
  缺模块/缺类型，与本次交付无关）。
- **架构（依赖方向还原）**：屋顶几何（surface/ridge/tiles/parameters/edit_rules/pipeline/replay/
  rectangular_*）按原 PDB 源路径归位 `country-core/src/resources/roofs/geometry/`；依赖方向恢复
  原 B 的 `system_roof → country_core`（消除 workspace 前序反向边，解除循环）。原 reconstruction/roof
  11 文件按 reviewer-C 组装为独立 `roof-geometry` crate（lib.rs/Cargo.toml + replay.rs 路径修正）。
- **事件映射**（重建命名 → 原 B 名）：RoofGeometryChanged ≡ RecalculateRoofCmd、
  WallGableChanged ≡ RecalculateGableRoofWallHeights、RoofSound ≡ PlayRoofSound、
  RecordRoofHistory ≡ RecordRoofHistoryCmd（0x98B 观测）、RoofTypeChanged → **SwitchRoofTypeCmd**
  （reviewer-C M1 / 原 edit_roof 面）。
- **reviewer-C FIX 落实**：1) 两 crate 落盘 ✓；2) roof-geometry 组装 + replay 路径 ✓；
  3) M1：resolved_update 不再恒 true（16B 通知 = gable 进出/变化或 profile/height/eave×gable-after；
  仅 ridge/tip 变化不推；Scalar 反馈含 |ridge| 与 16B 通知分开实现）✓；
  4) 6 个缺失 decomp：re-analyst-C 已闭环（4 fresh + 1 既有拷贝 + rhapsody 筛除）✓；
  5) roof_shape 14 业务补入 reconstruction/country-core roof_shape.rs（机器码契约）✓；
  6) SOURCE_MAP 对齐表见下 ✓。

## SOURCE_MAP 对齐表（33 业务 / 33 唯一 VA）

| 源文件 | 函数 | VA | 落点 |
|---|---|---|---|
| roof/src/create_roof.rs | system_roof::create_roof::create_roof | 0x1421aaea0 | system_roof::create_roof::create_roof |
| roof/src/deferred_record_roof_history.rs | system_roof::deferred_record_roof_history::roof_record_history | 0x141a37fd0 | system_roof::deferred_record_roof_history::roof_record_history |
| roof/src/delete_roof.rs | system_roof::delete_roof::process_invalid_roof_notifications | 0x14062eb80 | system_roof::delete_roof::process_invalid_roof_notifications |
| roof/src/delete_roof.rs | system_roof::delete_roof::mark_roof_for_deletion | 0x14062f3d0 | system_roof::delete_roof::mark_roof_for_deletion |
| roof/src/delete_roof.rs | system_roof::delete_roof::delete_roofs | 0x14062f8c0 | system_roof::delete_roof::delete_roofs |
| roof/src/edit_roof.rs | system_roof::edit_roof::edit_roof | 0x1421ab7c0 | system_roof::edit_roof::edit_roof |
| roof/src/lib.rs | system_roof::impl$0::add_roof_systems | 0x14062fcb0 | system_roof::impl$0::add_roof_systems |
| roof/src/lib.rs | system_roof::impl$0::add_roof_events_and_resources | 0x1406305f0 | system_roof::impl$0::add_roof_events_and_resources |
| roof/src/replay_roof.rs | system_roof::replay_roof::impl$0::process | 0x1421b48e0 | system_roof::replay_roof::impl$0::process |
| roof/src/replay_roof.rs | system_roof::replay_roof::replay_roof | 0x141a37ad0 | system_roof::replay_roof::replay_roof |
| roof/src/roof_trigger_volume.rs | system_roof::roof_trigger_volume::calculate_roof_trigger_volumes | 0x1421a3220 | system_roof::roof_trigger_volume::calculate_roof_trigger_volumes |
| roof/src/simulate_roof_tile_for_audio.rs | system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio | 0x1421a7090 | system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio |
| roof/src/ui_edit_roof.rs | system_roof::ui_edit_roof::ui_edit_roof | 0x141a18dd0 | system_roof::ui_edit_roof::ui_edit_roof |
| roof/src/ui_edit_roof.rs | system_roof::ui_edit_roof::do_ui_edit_roof | 0x141a19a90 | system_roof::ui_edit_roof::do_ui_edit_roof |
| roof/src/ui_edit_roof_ridge_dir.rs | system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir | 0x141a386b0 | system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir |
| roof/src/ui_edit_roof_ridge_dir.rs | system_roof::ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir | 0x141a39490 | system_roof::ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir |
| roof/src/ui_edit_roof_ridge_dir.rs | system_roof::ui_edit_roof_ridge_dir::switch_roof_ridge_direction | 0x141a3a730 | system_roof::ui_edit_roof_ridge_dir::switch_roof_ridge_direction |
| roof/src/ui_edit_roof_ridge_dir.rs | system_roof::ui_edit_roof_ridge_dir::record_switch_roof_ridge_history | 0x141a3ac20 | system_roof::ui_edit_roof_ridge_dir::record_switch_roof_ridge_history |
| roof/src/ui_edit_roof_ridge_length.rs | system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length | 0x1421a8210 | system_roof::ui_edit_roof_ridge_length::ui_edit_roof_ridge_length |
| roof/src/ui_edit_roof_ridge_length.rs | system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length | 0x1421a8c50 | system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length |
| roof/src/ui_edit_roof_ridge_length.rs | system_roof::ui_edit_roof_ridge_length::create_ui_widget | 0x1421aa170 | system_roof::ui_edit_roof_ridge_length::create_ui_widget |
| roof/src/ui_focused/ui_edit_roof_height_focused.rs | system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused | 0x1421ad440 | system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused |
| roof/src/update_roof_spatial.rs | system_roof::update_roof_spatial::update_roof_spatial | 0x1421a7710 | system_roof::update_roof_spatial::update_roof_spatial |
| roof/src/utils/assemble_circular_roof.rs | system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> | 0x1421b3120 | system_roof::utils::assemble_circular_roof::assemble_circular_roof |
| roof/src/utils/assemble_rectangular_roof.rs | system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> | 0x1421aeef0 | system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof |
| roof/src/utils/components.rs | system_roof::utils::components::impl$0::register_required_components | 0x14258d1f0 | system_roof::utils::components::impl$0::register_required_components |
| roof/src/utils/components.rs | system_roof::utils::components::impl$1::register_required_components | 0x14258d8e0 | system_roof::utils::components::impl$1::register_required_components |
| roof/src/visual/assemble_roof_tiles.rs | system_roof::visual::assemble_roof_tiles::assemble_roof_tiles | 0x141a36700 | system_roof::visual::assemble_roof_tiles::assemble_roof_tiles |
| roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs | system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom | 0x1421a6d40 | system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom |
| roof/src/visual/place_spires.rs | system_roof::visual::place_spires::place_roof_spires | 0x142196e70 | system_roof::visual::place_spires::place_roof_spires |
| roof/src/visual/roof_animation.rs | system_roof::visual::roof_animation::roof_animation | 0x1421a2da0 | system_roof::visual::roof_animation::roof_animation |
| roof/src/visual/switch_roof_type.rs | system_roof::visual::switch_roof_type::switch_roof_type | 0x1421ac4f0 | system_roof::visual::switch_roof_type::switch_roof_type |
| roof/src/wall_height_dt.rs | system_roof::wall_height_dt::wall_height_dt | 0x1421a2b60 | system_roof::wall_height_dt::wall_height_dt |

## 判定汇总

- **O（既有实现复用/验证）**：create_roof、delete_roof 三函数、edit_roof、update_roof_spatial、
  wall_height_dt、roof_animation、replay_roof、process、roof_record_history、assemble_roof_tiles
  （CPU 链 4514+ 案例）、assemble_circular/rectangular（tiles/pipeline 原码验证）、
  switch_roof_type（editing 链）、register_required_components ×2（胶水）。
- **I（结构级，细节登记）**：calculate_roof_trigger_volumes（圆 15 段环 O；矩形盒体/平顶/gable
  檐口面数待 reviewer 抽查）、ui_edit_roof/do_ui_edit_roof、ui_edit_roof_ridge_dir 族
  （SwitchRoofRidgeDirCmd 布局 {wall, direction, previous} 已证；UI widget 交互为 toolbar 后置）、
  ui_edit_roof_ridge_length 族（ProportionalScreenSpace 投影）、ui_focused、simulate_roof_tile_for_audio
  （0.28B 载荷 tag 4/5）、highlight_roof（PDB inline）、generate_roof_stone_floor_and_roof_bottom
  （随机 1..5 瓦数 O；延迟命令执行体渲染后置）、place_spires（SSBO 后置）。
- **U**：无（全部至少 I）。

## 关键事实登记

- create_roof panic：`Was requested to create a roof on a wall that is not a shape: IsEnclosed::`
  （0x142ff74a8，re-analyst §1）。
- 共享双缓冲队列 = bevy Events 双缓冲（cap-first Vec）——create_roof/update_roof_spatial/
  assemble 族的队列 walk 即 EventReader。
- `~id*0x30` 反向索引（delete_roofs/update_roof_spatial 族）。
- roofs-rt ALGORITHMS.md §8 的 `_UNK_142934604 = 16.0` 纠错为 **8.0**（re-analyst §8.1；
  实现侧 surface.rs 圆基 8.0 与正文一致）。


---

## 轮 2（reviewer-C 复验 + 中央机器码裁决）修复记录

- **必修**：
  1. `ridge_length_01_from_rectangle_ratio` 除数 0.94736844 → **5.25**
     （0x142aa50dc 实测 0x40a80000；旧注释 rne-read 声称作废，已改）。
  2. `cap_height`/`get_height_01_unclamped_from_ws` 圆基 16.0 → **8.0**
     （0x142934604 = 0x41000000；roof_visual_shape.rs 已改）。
- **应修**：
  3. `create_ui_widget` 空桩 → country-core UI 面实现（UiSounds::empty +
    WidgetParams::new + 交互派发；toolbar TweakerView 滑块面批次 3 后置）。
  4. `place_spires` 壳 → RoofSpire 记录生产 + RoofSpireRecords 缓冲写回
    （GraphicsDeviceResource 参数面按原候选登记）。
  5. `add_roof_systems` 重复注册 ×2 已删（wall_roof_hashmap/update_roof_spatial
    各保留一次）。
  6. EditOutcome `resolved_update` 误导注释已改（M1 语义说明）。


---

## 轮 3（reviewer-C 终验）修复记录

- **roof_shape.rs `cap_height` 圆基 16.0 → 8.0**（roof_shape_at_intersection_y
  0x1408e3820 共用块用 _UNK_142934604 = 8.0；轮 2 只改了 roof_visual_shape，
  本轮补 roof_shape.rs——两处统一，可标 review_passed(限定)）。
