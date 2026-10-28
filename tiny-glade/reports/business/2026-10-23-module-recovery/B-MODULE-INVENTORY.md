# B 函数与当前源码：初筛清单

本表是可复跑的源码定位初筛，尚不是业务复原率。body_candidate 仅表示唯一同名候选有非空且无显式 TODO 的函数体；没有审计其算法、完整性、生产接线或原版等价。unmatched_review 包含改名、合并、宏和真正缺失，不能归为未实现。

分母：B 索引 category 非空条目；排除 closure/closure_env 及用户删除的三类页面，按原路径和去泛型后的完整符号名合并。保留 impl$ 等不确定符号。M01/M16 按工具栏及菜单主归属分开；跨模块 helper 列 SHARED，不重复分摊。M17 没有独立函数分母。

| 模块 | 初筛函数组 | 唯一非占位体候选 | 含 TODO | 纯占位 | 空体待审 | 多候选待审 | 未匹配待审 |
|---|---:|---:|---:|---:|---:|---:|---:|
| M01 公共交互 / toolbar | 451 | 191 (42.4%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 104 (23.1%) | 156 (34.6%) |
| M02 地形编辑 | 19 | 13 (68.4%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 6 (31.6%) |
| M03 路径 / 铺石 | 27 | 2 (7.4%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 8 (29.6%) | 17 (63.0%) |
| M04 花园 | 3 | 2 (66.7%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1 (33.3%) |
| M05 水与林地边界 | 7 | 5 (71.4%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 2 (28.6%) | 0 (0.0%) |
| M06 墙体 / 规则建筑 | 900 | 314 (34.9%) | 2 (0.2%) | 9 (1.0%) | 2 (0.2%) | 280 (31.1%) | 293 (32.6%) |
| M07 屋顶 | 139 | 64 (46.0%) | 0 (0.0%) | 0 (0.0%) | 11 (7.9%) | 16 (11.5%) | 48 (34.5%) |
| M08 楼梯 / 平台 / 扶手 | 185 | 78 (42.2%) | 3 (1.6%) | 4 (2.2%) | 12 (6.5%) | 52 (28.1%) | 36 (19.5%) |
| M09 装饰物 | 247 | 149 (60.3%) | 0 (0.0%) | 1 (0.4%) | 2 (0.8%) | 39 (15.8%) | 56 (22.7%) |
| M10 颜色 / 自定义 | 38 | 22 (57.9%) | 0 (0.0%) | 0 (0.0%) | 1 (2.6%) | 4 (10.5%) | 11 (28.9%) |
| M11 clutter | 128 | 47 (36.7%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 28 (21.9%) | 53 (41.4%) |
| M12 ivy | 59 | 50 (84.7%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 5 (8.5%) | 4 (6.8%) |
| M13 树工具 | 17 | 15 (88.2%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1 (5.9%) | 1 (5.9%) |
| M14 森林 | 7 | 5 (71.4%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1 (14.3%) | 1 (14.3%) |
| M15 小动物 | 103 | 45 (43.7%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 11 (10.7%) | 47 (45.6%) |
| M16 菜单 / 设置 / 主题 / 引导 | 201 | 47 (23.4%) | 0 (0.0%) | 4 (2.0%) | 0 (0.0%) | 76 (37.8%) | 74 (36.8%) |
| SHARED 共享及未分配 | 4185 | 1201 (28.7%) | 0 (0.0%) | 0 (0.0%) | 2 (0.0%) | 2179 (52.1%) | 803 (19.2%) |

## 明确占位位置（仍需判定是否原业务必要分支）

- M06 `country_core::resources::walls::decorator_storage::DecoratorStorage::verify_integrity` → `tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:1307`：empty_body_review
- M06 `country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl` → `tiny-glade/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_height_impl.rs:103`：partial_stub
- M06 `system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars` → `tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/circle_pillars.rs:30`：explicit_stub
- M06 `system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders` → `tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/maintain_stone_column_colliders.rs:18`：explicit_stub
- M06 `system_wall_constructor::construct_elevation_supports::construct_circle_supports` → `tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/mod.rs:152`：explicit_stub
- M06 `system_wall_constructor::construct_elevation_supports::construct_rectangle_brackets` → `tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/mod.rs:174`：explicit_stub
- M06 `system_wall_constructor::construct_elevation_supports::construct_rectangle_columns` → `tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/mod.rs:164`：explicit_stub
- M06 `system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments` → `tiny-glade/crates/systems/wall-constructor/src/construct_elevation_supports/mod.rs:118`：explicit_stub
- M06 `system_wall_constructor::construct_floors::construct_floors` → `tiny-glade/crates/systems/wall-constructor/src/construct_floors.rs:56`：explicit_stub
- M06 `system_wall_constructor::curved_halftimber::curved_halftimber_maintain_entities` → `tiny-glade/crates/systems/wall-constructor/src/curved_halftimber/mod.rs:23`：partial_stub
- M06 `system_wall_constructor::debug_force_update_arches` → `tiny-glade/crates/systems/wall-constructor/src/lib.rs:142`：empty_body_review
- M06 `system_wall_constructor::plaster_systems::create_plaster` → `tiny-glade/crates/systems/wall-constructor/src/plaster_systems.rs:84`：explicit_stub
- M06 `system_wall_constructor::plaster_systems::maintain_plaster` → `tiny-glade/crates/systems/wall-constructor/src/plaster_systems.rs:65`：explicit_stub
- M07 `country_core::resources::roofs::roof_animation::impl$2::register_required_components` → `tiny-glade/crates/systems/roof/src/utils/components.rs:30`：empty_body_review
- M07 `country_core::resources::roofs::roof_animation::impl$4::register_required_components` → `tiny-glade/crates/systems/roof/src/utils/components.rs:30`：empty_body_review
- M07 `country_core::resources::roofs::roof_shape::impl$37::register_required_components` → `tiny-glade/crates/systems/roof/src/utils/components.rs:30`：empty_body_review
- M07 `country_core::resources::roofs::roof_tile_sim::impl$2::register_required_components` → `tiny-glade/crates/systems/roof/src/utils/components.rs:30`：empty_body_review
- M07 `country_core::resources::roofs::roof_tiles::impl$2::register_required_components` → `tiny-glade/crates/systems/roof/src/utils/components.rs:30`：empty_body_review
- M07 `country_core::resources::roofs::roof_trigger_volume::impl$2::register_required_components` → `tiny-glade/crates/systems/roof/src/utils/components.rs:30`：empty_body_review
- M07 `system_roof::ui_edit_roof::do_ui_edit_roof` → `tiny-glade/crates/systems/roof/src/ui_edit_roof.rs:66`：empty_body_review
- M07 `system_roof::ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir` → `tiny-glade/crates/systems/roof/src/ui_edit_roof_ridge_dir.rs:83`：empty_body_review
- M07 `system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length` → `tiny-glade/crates/systems/roof/src/ui_edit_roof_ridge_length.rs:77`：empty_body_review
- M07 `system_roof::utils::components::impl$0::register_required_components` → `tiny-glade/crates/systems/roof/src/utils/components.rs:30`：empty_body_review
- M07 `system_roof::utils::components::impl$1::register_required_components` → `tiny-glade/crates/systems/roof/src/utils/components.rs:30`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs::generate_playermade_stair_decorators` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs.rs:338`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs::maintain_stair_graph` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs.rs:781`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_assemble::create_structural_colliders_and_initial_graph` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:284`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:322`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_assemble::playermade_stairs_add_wall_holes` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:276`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:167`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assembly_propagate_dirty_flags` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:269`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_assemble::stairs_subscribe_lazy` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:76`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_assemble::structural_graph_subscribe` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:50`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_railings_assemble::add_corner_stone_railing_piece` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs:62`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs:20`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_railings_assemble::construct_wooden_railing` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_railings_assemble.rs:48`：empty_body_review
- M08 `system_wall_constructor::construct_platform_stairs::construct_platform_stairs` → `tiny-glade/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs:34`：explicit_stub
- M08 `system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/brackets.rs:17`：partial_stub
- M08 `system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/magic_support.rs:16`：partial_stub
- M08 `system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs:100`：explicit_stub
- M08 `system_wall_constructor::construct_playermade_stairs::preprocess_curve` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs:90`：explicit_stub
- M08 `system_wall_constructor::construct_playermade_stairs::pillars_process_proposals::process_playermade_stair_pillars_proposals` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/pillars_process_proposals.rs:138`：explicit_stub
- M08 `system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/pillars_visual.rs:19`：partial_stub
- M09 `system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part` → `tiny-glade/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs:68`：empty_body_review
- M09 `system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part` → `tiny-glade/crates/systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs:63`：empty_body_review
- M09 `tiny_glade::decorator_plugin::decorators_on_glade_unload` → `tiny-glade/crates/tiny-glade/src/decorator_plugin.rs:49`：explicit_stub
- M10 `system_color::ui_color_wall_and_roof::color_cursor_icon` → `tiny-glade/crates/systems/color/src/ui_color_wall_and_roof.rs:61`：empty_body_review
- M16 `tiny_glade::gui::gui_debug::do_freecam_save` → `tiny-glade/crates/tiny-glade/src/gui/gui_debug.rs:59`：explicit_stub
- M16 `tiny_glade::gui::gui_debug::do_gui_debug` → `tiny-glade/crates/tiny-glade/src/gui/gui_debug.rs:51`：explicit_stub
- M16 `tiny_glade::gui::gui_error_log::do_gui_error_log` → `tiny-glade/crates/tiny-glade/src/gui/gui_error_log.rs:11`：explicit_stub
- M16 `tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game` → `tiny-glade/crates/tiny-glade/src/gui/gui_link_to_official_game.rs:12`：explicit_stub
- SHARED `country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi::track_gpu_lifetime` → `tiny-glade/crates/country-core/src/render/pass_builder/compute_pass_builder.rs:308`：empty_body_review
- SHARED `utils::name_hash::NameHash::debug_name` → `tiny-glade/crates/utils/src/name_hash.rs:21`：empty_body_review

完整逐项清单见 B-MODULE-INVENTORY.csv / .json；JSON 保存候选源码 SHA256。未运行游戏或测试。
