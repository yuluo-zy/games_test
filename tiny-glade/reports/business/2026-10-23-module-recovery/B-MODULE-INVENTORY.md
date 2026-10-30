# B 函数与当前源码：初筛清单

本表是可复跑的源码定位初筛，尚不是业务复原率。body_candidate 仅表示唯一同名候选有非空且无显式 TODO 的函数体；没有审计其算法、完整性、生产接线或原版等价。unmatched_review 包含改名、合并、宏和真正缺失，不能归为未实现。

分母：B 索引 category 非空条目；排除 closure/closure_env、用户删除的三类页面、trait/serde/SystemParam 等机械名称及无法凭同名确定 owner 的 new/get/insert/remove 后，按原路径和去泛型后的完整符号名合并。后两类不生成候选关联，原 B ID 与排除理由保留在 JSON 的 excluded_associations；不删除原函数、汇编或调用证据，也不据此宣称这些方法没有业务。保留其他 impl$ 等不确定符号。M01/M16 按工具栏及菜单主归属分开；跨模块 helper 列 SHARED，不重复分摊。M17 没有独立函数分母。

| 模块 | 初筛函数组 | 唯一非占位体候选 | 含 TODO | 纯占位 | 空体待审 | 多候选待审 | 未匹配待审 |
|---|---:|---:|---:|---:|---:|---:|---:|
| M01 公共交互 / toolbar | 359 | 158 (44.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 67 (18.7%) | 134 (37.3%) |
| M02 地形编辑 | 13 | 8 (61.5%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 5 (38.5%) |
| M03 路径 / 铺石 | 20 | 2 (10.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 8 (40.0%) | 10 (50.0%) |
| M04 花园 | 3 | 2 (66.7%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1 (33.3%) |
| M05 水与林地边界 | 7 | 4 (57.1%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 2 (28.6%) | 1 (14.3%) |
| M06 墙体 / 规则建筑 | 576 | 315 (54.7%) | 1 (0.2%) | 0 (0.0%) | 2 (0.3%) | 69 (12.0%) | 189 (32.8%) |
| M07 屋顶 | 87 | 59 (67.8%) | 0 (0.0%) | 0 (0.0%) | 3 (3.4%) | 5 (5.7%) | 20 (23.0%) |
| M08 楼梯 / 平台 / 扶手 | 112 | 66 (58.9%) | 3 (2.7%) | 4 (3.6%) | 1 (0.9%) | 11 (9.8%) | 27 (24.1%) |
| M09 装饰物 | 192 | 126 (65.6%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 17 (8.9%) | 49 (25.5%) |
| M10 颜色 / 自定义 | 35 | 22 (62.9%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 2 (5.7%) | 11 (31.4%) |
| M11 clutter | 70 | 45 (64.3%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 4 (5.7%) | 21 (30.0%) |
| M12 ivy | 52 | 49 (94.2%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 3 (5.8%) |
| M13 树工具 | 16 | 15 (93.8%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1 (6.2%) | 0 (0.0%) |
| M14 森林 | 6 | 5 (83.3%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 1 (16.7%) | 0 (0.0%) |
| M15 小动物 | 59 | 45 (76.3%) | 0 (0.0%) | 0 (0.0%) | 0 (0.0%) | 2 (3.4%) | 12 (20.3%) |
| M16 菜单 / 设置 / 主题 / 引导 | 152 | 38 (25.0%) | 0 (0.0%) | 4 (2.6%) | 0 (0.0%) | 62 (40.8%) | 48 (31.6%) |
| SHARED 共享及未分配 | 2616 | 1098 (42.0%) | 0 (0.0%) | 0 (0.0%) | 2 (0.1%) | 813 (31.1%) | 703 (26.9%) |

## 明确占位位置（仍需判定是否原业务必要分支）

- M06 `country_core::resources::walls::decorator_storage::DecoratorStorage::verify_integrity` → `tiny-glade/crates/country-core/src/resources/walls/decorator_storage.rs:1312`：empty_body_review
- M06 `country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl` → `tiny-glade/crates/country-core/src/systems/wall/adjust_wall_height/adjust_wall_height_impl.rs:103`：partial_stub
- M06 `system_wall_constructor::debug_force_update_arches` → `tiny-glade/crates/systems/wall-constructor/src/lib.rs:142`：empty_body_review
- M07 `system_roof::ui_edit_roof::do_ui_edit_roof` → `tiny-glade/crates/systems/roof/src/ui_edit_roof.rs:66`：empty_body_review
- M07 `system_roof::ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir` → `tiny-glade/crates/systems/roof/src/ui_edit_roof_ridge_dir.rs:83`：empty_body_review
- M07 `system_roof::ui_edit_roof_ridge_length::do_ui_edit_roof_ridge_length` → `tiny-glade/crates/systems/roof/src/ui_edit_roof_ridge_length.rs:77`：empty_body_review
- M08 `system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble` → `tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:278`：empty_body_review
- M08 `system_wall_constructor::construct_platform_stairs::construct_platform_stairs` → `tiny-glade/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs:34`：explicit_stub
- M08 `system_wall_constructor::construct_playermade_stairs::brackets::construct_playermade_stair_brackets` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/brackets.rs:17`：partial_stub
- M08 `system_wall_constructor::construct_playermade_stairs::magic_support::playermade_stair_floating_bricks` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/magic_support.rs:16`：partial_stub
- M08 `system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs:100`：explicit_stub
- M08 `system_wall_constructor::construct_playermade_stairs::preprocess_curve` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs:90`：explicit_stub
- M08 `system_wall_constructor::construct_playermade_stairs::pillars_process_proposals::process_playermade_stair_pillars_proposals` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/pillars_process_proposals.rs:128`：explicit_stub
- M08 `system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals` → `tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/pillars_visual.rs:19`：partial_stub
- M16 `tiny_glade::gui::gui_debug::do_freecam_save` → `tiny-glade/crates/tiny-glade/src/gui/gui_debug.rs:59`：explicit_stub
- M16 `tiny_glade::gui::gui_debug::do_gui_debug` → `tiny-glade/crates/tiny-glade/src/gui/gui_debug.rs:51`：explicit_stub
- M16 `tiny_glade::gui::gui_error_log::do_gui_error_log` → `tiny-glade/crates/tiny-glade/src/gui/gui_error_log.rs:11`：explicit_stub
- M16 `tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game` → `tiny-glade/crates/tiny-glade/src/gui/gui_link_to_official_game.rs:12`：explicit_stub
- SHARED `country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi::track_gpu_lifetime` → `tiny-glade/crates/country-core/src/render/pass_builder/compute_pass_builder.rs:308`：empty_body_review
- SHARED `utils::name_hash::NameHash::debug_name` → `tiny-glade/crates/utils/src/name_hash.rs:21`：empty_body_review

完整逐项清单见 B-MODULE-INVENTORY.csv / .json；JSON 保存候选源码 SHA256。未运行游戏或测试。
