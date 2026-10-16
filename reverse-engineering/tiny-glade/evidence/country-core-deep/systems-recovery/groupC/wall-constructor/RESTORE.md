# RESTORE — system-wall-constructor（组 C restorer，2026-10-16）

## 交付摘要

- **落点**：`tiny-glade/crates/systems/wall-constructor/`（crate `system-wall-constructor`，
  已挂 workspace），13 子模块全建、97 业务对齐（93 唯一 VA；4 个 VA 为跨文件共享去重/泛型实例同址）。
- **编译**：`cargo check --workspace` 零错误（本 crate 零错误；6 warning）。
- **country-core 消费面补齐**（原 PDB 路径）：walls/arches、walls/plaster、walls/wooden_gates、
  walls/wall_stone_columns、walls/curved_halftimber、walls/elevation_support_stats、
  walls/material_stats、walls/wall_path_segment、stairs_walls::{StairsArchSupportRequests,
  StairsBracketsRequests}、systems/wall/wall_state/wall_roof_hashmap（原事件宿主路径再导出）、
  Roof::observed_mut/new、RoofAnimation/RoofTileSimulation::from_inner、observed_294/296 等。
- **复用链**（G04/G05/G06 冻结）：country-core recovered_building 的 wall_constructor（rows/
  holes/proposal/cut/emit）、wall_arches（construct_regular_wall_arches）、wall_segmentation、
  fences、gates 直接接线；本 crate 提供 ECS 系统 + 几何包装。
- **未冻结边界登记**（reviewer-C §4）：construct_arches 的 shape2 分支与 C 版 wooden_arch_frame、
  半木构逐块（26.7KB asm 手解，O 判定待 reviewer）、elevation supports 族（circle_categorize
  已三方核对 O，其余 fresh decomp 结构级）、row→proposal 中段（from_curve README 边界）。

## SOURCE_MAP 对齐表（97 业务 / 93 唯一 VA）

| 源文件 | 函数 | VA | 落点 |
|---|---|---|---|
| wall-constructor/src/arch_walker.rs | system_wall_constructor::arch_walker::ArchWalker::new | 0x14074c2d0 | system_wall_constructor::arch_walker::ArchWalker::new |
| wall-constructor/src/arch_walker.rs | system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_arches::construct_arches::closure_env$3> | 0x141232bb0 | system_wall_constructor::arch_walker::ArchWalker::walk |
| wall-constructor/src/arch_walker.rs | system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> > | 0x141233380 | system_wall_constructor::arch_walker::ArchWalker::walk |
| wall-constructor/src/arch_walker.rs | system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets::closure_env$7> | 0x141231ea0 | system_wall_constructor::arch_walker::ArchWalker::walk |
| wall-constructor/src/arch_walker.rs | system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$6<false> > | 0x14048ab50 | system_wall_constructor::arch_walker::ArchWalker::walk |
| wall-constructor/src/construct_arches/mod.rs | system_wall_constructor::construct_arches::construct_arches | 0x14075f680 | system_wall_constructor::construct_arches::construct_arches |
| wall-constructor/src/construct_arches/profile.rs | system_wall_constructor::construct_arches::profile::ArchFunction::remap_t | 0x140765300 | system_wall_constructor::construct_arches::profile::ArchFunction::remap_t |
| wall-constructor/src/construct_arches/profile.rs | system_wall_constructor::construct_arches::profile::create_stone_arch_profile | 0x1407645f0 | system_wall_constructor::construct_arches::profile::create_stone_arch_profile |
| wall-constructor/src/construct_elevation_supports/circle_bottom_layer.rs | system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer | 0x14124b2c0 | system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer |
| wall-constructor/src/construct_elevation_supports/circle_bottom_layer.rs | system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick<ref_mut$<system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env$0> > | 0x141fe1de0 | system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick |
| wall-constructor/src/construct_elevation_supports/circle_bottom_layer.rs | system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick<ref_mut$<system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer::closure_env$1> > | 0x141fe1de0 | system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick |
| wall-constructor/src/construct_elevation_supports/circle_categorize.rs | system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize | 0x14124af60 | system_wall_constructor::construct_elevation_supports::circle_categorize::circle_categorize |
| wall-constructor/src/construct_elevation_supports/circle_pillars.rs | system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars | 0x14124ba30 | system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars |
| wall-constructor/src/construct_elevation_supports/circle_turret.rs | system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret | 0x14124cc30 | system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret |
| wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs | system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments | 0x141248620 | system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments |
| wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs | system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache::new | 0x141248300 | system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache::new |
| wall-constructor/src/construct_elevation_supports/maintain_stone_column_colliders.rs | system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders | 0x14078a6e0 | system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders |
| wall-constructor/src/construct_elevation_supports/mod.rs | system_wall_constructor::construct_elevation_supports::overhang_subscribe_to_tabloids | 0x140786440 | system_wall_constructor::construct_elevation_supports::overhang_subscribe_to_tabloids |
| wall-constructor/src/construct_elevation_supports/mod.rs | system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments | 0x1407867d0 | system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments |
| wall-constructor/src/construct_elevation_supports/mod.rs | system_wall_constructor::construct_elevation_supports::construct_circle_supports | 0x140788160 | system_wall_constructor::construct_elevation_supports::construct_circle_supports |
| wall-constructor/src/construct_elevation_supports/mod.rs | system_wall_constructor::construct_elevation_supports::construct_rectangle_columns | 0x140788a80 | system_wall_constructor::construct_elevation_supports::construct_rectangle_columns |
| wall-constructor/src/construct_elevation_supports/mod.rs | system_wall_constructor::construct_elevation_supports::construct_rectangle_brackets | 0x1407896b0 | system_wall_constructor::construct_elevation_supports::construct_rectangle_brackets |
| wall-constructor/src/construct_elevation_supports/rectangle_bottom_layer.rs | system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer | 0x14124eb80 | system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer |
| wall-constructor/src/construct_elevation_supports/rectangle_large_brackets.rs | system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets | 0x141253fc0 | system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets |
| wall-constructor/src/construct_elevation_supports/rectangle_pillars.rs | system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars | 0x14124fcb0 | system_wall_constructor::construct_elevation_supports::rectangle_pillars::construct_rectangle_pillars |
| wall-constructor/src/construct_elevation_supports/rectangle_pillars.rs | system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y | 0x141250720 | system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y |
| wall-constructor/src/construct_elevation_supports/rectangle_small_brackets.rs | system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets | 0x141250da0 | system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets |
| wall-constructor/src/construct_elevation_supports/util_cross_brick_pillar.rs | system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1> > | 0x142418f50 | system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar |
| wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs | system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base | 0x141fe2150 | system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base |
| wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs | system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns | 0x141fe3240 | system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns |
| wall-constructor/src/construct_fences/mod.rs | system_wall_constructor::construct_fences::construct_fences | 0x140765830 | system_wall_constructor::construct_fences::construct_fences |
| wall-constructor/src/construct_fences/process_segment.rs | system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0> | 0x1412388f0 | system_wall_constructor::construct_fences::process_segment::emit_fences |
| wall-constructor/src/construct_fences/process_segment.rs | system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0> | 0x141238d80 | system_wall_constructor::construct_fences::process_segment::process_segment |
| wall-constructor/src/construct_fences/process_segment.rs | system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle | 0x141fdf9e0 | system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle |
| wall-constructor/src/construct_floors.rs | system_wall_constructor::construct_floors::construct_floors | 0x1407661e0 | system_wall_constructor::construct_floors::construct_floors |
| wall-constructor/src/construct_gates/mod.rs | system_wall_constructor::construct_gates::construct_wooden_gate_doors | 0x141210110 | system_wall_constructor::construct_gates::construct_wooden_gate_doors |
| wall-constructor/src/construct_gates/mod.rs | system_wall_constructor::construct_gates::DoorMeshInProgress::add_quad | 0x141214ed0 | system_wall_constructor::construct_gates::DoorMeshInProgress::add_quad |
| wall-constructor/src/construct_gates/systems.rs | system_wall_constructor::construct_gates::systems::update_gates_hashmap | 0x140755690 | system_wall_constructor::construct_gates::systems::update_gates_hashmap |
| wall-constructor/src/construct_gates/systems.rs | system_wall_constructor::construct_gates::systems::construct_gates | 0x140756370 | system_wall_constructor::construct_gates::systems::construct_gates |
| wall-constructor/src/construct_lintels/mod.rs | system_wall_constructor::construct_lintels::construct_lintels | 0x14076d9f0 | system_wall_constructor::construct_lintels::construct_lintels |
| wall-constructor/src/construct_lintels/place_support_beams.rs | system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> | 0x141243740 | system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support |
| wall-constructor/src/construct_lintels/place_support_beams.rs | system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> | 0x141242530 | system_wall_constructor::construct_lintels::place_support_beams::place_support_beams |
| wall-constructor/src/construct_lintels/place_support_beams.rs | system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> | 0x141243c20 | system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams |
| wall-constructor/src/construct_lintels/place_support_beams.rs | system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::propose | 0x141fe09c0 | system_wall_constructor::construct_lintels::place_support_beams::SupportBeamGrower::propose |
| wall-constructor/src/construct_platform_stairs/mod.rs | system_wall_constructor::construct_platform_stairs::construct_platform_stairs | 0x1407744d0 | system_wall_constructor::construct_platform_stairs::construct_platform_stairs |
| wall-constructor/src/construct_playermade_stairs/brackets.rs | system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets | 0x1407834f0 | system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets |
| wall-constructor/src/construct_playermade_stairs/magic_support.rs | system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks | 0x14077a2f0 | system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks |
| wall-constructor/src/construct_playermade_stairs/mod.rs | system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls<true> | 0x140496800 | system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls |
| wall-constructor/src/construct_playermade_stairs/mod.rs | system_wall_constructor::construct_playermade_stairs::preprocess_curve | 0x140780e20 | system_wall_constructor::construct_playermade_stairs::preprocess_curve |
| wall-constructor/src/construct_playermade_stairs/pillars_visual.rs | system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals | 0x14077d520 | system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals |
| wall-constructor/src/construct_walls.rs | system_wall_constructor::construct_walls::construct_walls | 0x140757800 | system_wall_constructor::construct_walls::construct_walls |
| wall-constructor/src/curved_halftimber/mod.rs | system_wall_constructor::curved_halftimber::curved_halftimber_maintain_entities | 0x1407706d0 | system_wall_constructor::curved_halftimber::curved_halftimber_maintain_entities |
| wall-constructor/src/curved_halftimber/wooden_floor_borders.rs | system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders | 0x141244310 | system_wall_constructor::curved_halftimber::wooden_floor_borders::wooden_floor_borders |
| wall-constructor/src/init_wall_height_and_arch_segment_height.rs | system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height | 0x14078b550 | system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height |
| wall-constructor/src/lib.rs | rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > | 0x141fdca60 | rhapsody::raw::device::GraphicsDevice::create_buffer |
| wall-constructor/src/lib.rs | system_wall_constructor::construct_platform_stairs::construct_platform_stairs | 0x1407744d0 | system_wall_constructor::construct_platform_stairs::construct_platform_stairs |
| wall-constructor/src/lib.rs | system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals | 0x14077d520 | system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals |
| wall-constructor/src/lib.rs | system_wall_constructor::construct_playermade_stairs::preprocess_curve | 0x140780e20 | system_wall_constructor::construct_playermade_stairs::preprocess_curve |
| wall-constructor/src/lib.rs | system_wall_constructor::debug_force_update_arches | 0x14003bb80 | —（登记） |
| wall-constructor/src/lib.rs | system_wall_constructor::gpu_clear_wall_gate_pieces | 0x140773c00 | —（登记） |
| wall-constructor/src/lib.rs | system_wall_constructor::gpu_flush_plank_buffers | 0x140774010 | —（登记） |
| wall-constructor/src/place_pillars.rs | system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4> | 0x14120d670 | system_wall_constructor::place_pillars::assemble_stone_pillar |
| wall-constructor/src/place_pillars.rs | system_wall_constructor::place_pillars::PillarProposals::add_proposal | 0x1407534d0 | system_wall_constructor::place_pillars::PillarProposals::add_proposal |
| wall-constructor/src/place_pillars.rs | system_wall_constructor::place_pillars::place_pillars | 0x140753670 | system_wall_constructor::place_pillars::place_pillars |
| wall-constructor/src/plaster_systems.rs | system_wall_constructor::plaster_systems::maintain_plaster | 0x14075b810 | system_wall_constructor::plaster_systems::maintain_plaster |
| wall-constructor/src/plaster_systems.rs | system_wall_constructor::plaster_systems::impl$2::register_required_components | 0x14075e9e0 | system_wall_constructor::plaster_systems::impl$2::register_required_components |
| wall-constructor/src/plaster_systems.rs | system_wall_constructor::plaster_systems::create_plaster | 0x14075d460 | system_wall_constructor::plaster_systems::create_plaster |
| wall-constructor/src/plaster_systems.rs | system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > | 0x1412269b0 | system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities |
| wall-constructor/src/plaster_systems.rs | system_wall_constructor::plaster_systems::split_wall_into_sides | 0x14075dc60 | system_wall_constructor::plaster_systems::split_wall_into_sides |
| wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs | system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation | 0x14074f7a0 | system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation |
| wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs | system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation | 0x14074f310 | system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation |
| wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs | system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment | 0x140750420 | system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment |
| wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs | system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range | 0x140751240 | system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range |
| wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs | system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate | 0x140751610 | system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate |
| wall-constructor/src/segmentation/sample_wall_path_intersections.rs | system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections | 0x14074c720 | system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections |
| wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs | system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs | 0x140751960 | system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs |
| wall-constructor/src/segmentation/wall_path_segment.rs | system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl | 0x14122ff90 | system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl |
| wall-constructor/src/ui_arch_controls.rs | system_wall_constructor::ui_arch_controls::do_ui_arch_controls | 0x140767b70 | system_wall_constructor::ui_arch_controls::do_ui_arch_controls |
| wall-constructor/src/ui_arch_controls.rs | system_wall_constructor::ui_arch_controls::ui_arch_controls | 0x140766e70 | system_wall_constructor::ui_arch_controls::ui_arch_controls |
| wall-constructor/src/ui_lintel_controls.rs | system_wall_constructor::ui_lintel_controls::ui_lintel_controls | 0x140771a60 | system_wall_constructor::ui_lintel_controls::ui_lintel_controls |
| wall-constructor/src/utils/half_timber.rs | system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0> | 0x141fd1c30 | system_wall_constructor::utils::half_timber::add_underwindow_pattern |
| wall-constructor/src/utils/half_timber.rs | system_wall_constructor::utils::half_timber::half_timbering | 0x141218e30 | system_wall_constructor::utils::half_timber::half_timbering |
| wall-constructor/src/utils/half_timber.rs | system_wall_constructor::utils::half_timber::overlapping_windows | 0x1412218a0 | system_wall_constructor::utils::half_timber::overlapping_windows |
| wall-constructor/src/utils/half_timber.rs | system_wall_constructor::utils::half_timber::roof_outline | 0x141217be0 | system_wall_constructor::utils::half_timber::roof_outline |
| wall-constructor/src/utils/half_timber.rs | system_wall_constructor::utils::half_timber::get_window_aabb | 0x141221da0 | system_wall_constructor::utils::half_timber::get_window_aabb |
| wall-constructor/src/utils/half_timber.rs | system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows | 0x141221f70 | system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows |
| wall-constructor/src/utils/mod.rs | system_wall_constructor::utils::perturb_row_ends | 0x141240d30 | system_wall_constructor::utils::perturb_row_ends |
| wall-constructor/src/utils/mod.rs | system_wall_constructor::utils::split_horizontally | 0x141240ec0 | system_wall_constructor::utils::split_horizontally |
| wall-constructor/src/utils/resolve_hole_overlap.rs | system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap | 0x1412411c0 | system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap |
| wall-constructor/src/utils/split_into_random_rows.rs | system_wall_constructor::utils::split_into_random_rows::split_into_random_rows | 0x14123e110 | system_wall_constructor::utils::split_into_random_rows::split_into_random_rows |
| wall-constructor/src/utils/stairs_signifiers.rs | system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new | 0x141223d10 | system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new |
| wall-constructor/src/utils/trim_rows.rs | system_wall_constructor::utils::trim_rows::trim_rows | 0x14123eef0 | system_wall_constructor::utils::trim_rows::trim_rows |
| wall-constructor/src/utils/trim_rows.rs | system_wall_constructor::utils::trim_rows::impl$0::above_to_below | 0x14123fab0 | system_wall_constructor::utils::trim_rows::impl$0::above_to_below |
| wall-constructor/src/utils/trim_rows.rs | system_wall_constructor::utils::trim_rows::impl$0::below_to_above | 0x14123fb70 | system_wall_constructor::utils::trim_rows::impl$0::below_to_above |
| wall-constructor/src/utils/wall_corners.rs | system_wall_constructor::utils::wall_corners::add_wall_corners | 0x141215540 | system_wall_constructor::utils::wall_corners::add_wall_corners |
| wall-constructor/src/utils/wall_skirts.rs | system_wall_constructor::utils::wall_skirts::add_skirts | 0x1412221f0 | system_wall_constructor::utils::wall_skirts::add_skirts |
| wall-constructor/src/wall_constructor.rs | system_wall_constructor::wall_constructor::WallConstructor::from_curve | 0x140769760 | system_wall_constructor::wall_constructor::WallConstructor::from_curve |

## 判定汇总（按 reviewer-C 矩阵分组）

- **O**：arch_walker（4 实例同构数值契约）、circle_categorize（asm↔decomp↔.rdata 三方核对）、
  place_pillars/assemble_stone_pillar 常量族、split_into_random_rows/trim_rows/resolve_hole_overlap
  （G04 冻结）、wall_path_segmentation（B-process）、fences/gates 冻结链、construct_walls 调用铁律
  （不 CALL arches/lintels）。
- **I（结构级）**：elevation supports 其余函数（circle/rectangle pillars/turret/bottom_layer/
  crossbrick）、lintels（materializer→SupportBeamBuilder 链）、plaster 族（generate_plaster_mesh
  71.6KB 泛型）、half_timber 族（26.7KB asm 逐块待差分）、playermade stairs 族、platform stairs
  （B-platform-* 冻结链接线）、gpu_* 三函数（SSBO 直写，渲染边界后置登记）。
- **依赖筛除**：0x141fdca60 rhapsody create_buffer 按 DEPENDENCY_FILTERING 不计业务（与 reviewer 一致）。


---

## 轮 2（reviewer-C 复验）修复记录

> **状态：不得标 review_passed**——以下按 reviewer-C 优先级逐项落实，未恢复项
> 全部显式登记（代码内 `未恢复（壳登记）` 标记 + 本表），不称已恢复。

- **W-B1 冻结链接线（已完成）**：lintels → `materialize_wall_segments` +
  `construct_lintel_supports`（64B 木板落盘）；fences → `construct_fences_cpu`
  （64B 木板 + 统计落盘）；gates → ConstructGateCmd（arches 产出）→ Gates 表 +
  门实体；arches → `construct_regular_wall_arches` 真实调用。
- **W-B2 construct_arches 输出落盘（已完成）**：96B WallBrickInstance 记录 +
  ConstructGateCmd120（payload 120B + 事件）→ WallConstructorOutput 资源 +
  gate 事件流；闭合 first→last 复制由 recovered 链承担。
- **W-B3 half_timber（结构级，逐块未恢复）**：26.7KB asm 前导结构落码
  （try_get_wall_shape → gable_sides/is_flat → SSBO 名查找 61 字符 →
  尾部握手）；**中段 ~0x6860B 的 plank 坐标登记为未恢复细节，O 判定待
  reviewer asm 复核**。
- **W-B4 plaster 5 函数（结构级）**：maintain_plaster（Create/Delete/Recalc
  命令消费 + 实体生命周期）；create_plaster；split_wall_into_sides（两面
  面片）；generate_plaster_mesh_and_entities（洞 AABB 过滤 → 面片顶点）；
  register_required_components。**逐面网格坐标登记为未恢复细节**。
- **W-B5 elevation（circle_categorize 为示范）**：circle_pillars 按 1016 行
  decomp 结构实现（PublicWalls::get → get_wall_shape → contains_point →
  is_halftimbered 门控 → 环向支柱）；circle_turret/bottom_layer/
  rectangle_* 已按常量族实现；**环形步进精确数值登记为 decomp 复核项**。
- **其余壳逐项处置**：corners/skirts/stairs_signifiers 真实几何；brackets →
  ArchWalker walk + 64B 落盘；pillars_process/visuals → 44B 提议消费；
  平台/玩家楼梯系统接线；ui_arch/ui_lintel 分发逻辑；segmentation 助手
  （process_segment/add_splits/score）结构实现；**gpu_clear_wall_gate_pieces/
  gpu_flush_plank_buffers/debug_force_update_arches 三函数、from_curve 中段
  row→proposal 桥、half_timber 逐块坐标、plaster 逐面坐标 = 显式未恢复
  （壳登记），不称已恢复**。


---

## 轮 3（reviewer-C 终验）修复记录

- **circle_pillars（0x14124ba30）按机器码重做**（三方核对标准）：
  - 弃旧实现（无 0x3f99999a(1.2)/0x3e4ccccd(0.2) 常量；τ/count 均匀分割 +
    0.2 内缩不实）；
  - `count = round(lerp(4, 7, clamp01(ratio)))`（_UNK_142a7cd48 = **7.0f**、
    _UNK_142934694 = **4.0f**；ratio = func_0x000140c904a0 归一，钳上界）；
  - 角向均匀 `t = i/count`（断言 t∈[0,1] 非 NaN，同 walker 契约）；
  - 每点地形采样（FUN_141246fb0 = TerrainHeightsData::sample_at_world_xz）+
    `terrain + max(0,ratio) <= wall+0x280` 门控；
  - 支柱高度 = find_stone_column_start_y（0x141250720）+ smoothstep
    （(h−y+_UNK_142a7b650)/5.0 → (3−2x)x² → ·1.5）；
  - `wall+0x298` 字节门控半木构第二变体（0.5×3 常量族 + `wall+0x280·
    _UNK_14292c76c` 计数）登记。
- **gates 接入 construct_wooden_gate_doors 冻结链**：
  - 弃伪门宽 continue + spawn_empty；
  - payload 全解：wall@0/ordinal@8/段计数@16/段数据@20..120 →
    MaterializedWallSegment → `construct_gate_cpu`（内部按墙高比选
    construct_wooden_gate_doors 或 nani 分支）；
  - 输出落盘：96B 支撑砖 → WallConstructorOutput；门扇 136B spawn payload
    （TransientMesh 0..16、变换 16..56、mesh 64、shader 72、top-position
    80..104、WallId 104、端点方向 112..128、seed 128、+29a 132）→
    door_spawn_payloads + Gates 表；nanis 渲染边界登记；
  - seed = wrapping WallId·ordinal + leaf_ordinal cast u32；shader 键
    solid_vertex_color_wooden_gate。
- **roof_shape.rs cap_height 圆基 16.0 → 8.0**（轮 2 只改了 roof_visual_shape，
  本轮补 roof_shape.rs 的 roof_shape_at_intersection_y 共用块；
  0x142934604 = 8.0 两处统一）。
