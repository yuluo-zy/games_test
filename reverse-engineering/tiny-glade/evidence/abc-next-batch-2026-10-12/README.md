# 下一批选择分析

本轮仅分析，不领取、不实施、不启动agent、不构建或执行原码。

目前三版同码共211组，上批保存52组工具候选；剩余159组（121常规、38核心）分布79原文件。
这159是索引候选余额，不是159个全部尚无源码的任务，须先与现有reconstruction逐个核对。
其中19组C观察体只有ret。核心标签表示调用影响，不表示难度；例如xvy为27B、lerp为13B，
C去重索引调用者分别139与27。数量/字节均来自原索引，而非本轮原码执行。

推荐先选35组，按14几何基础、17建筑数据接口、4历史接口组织。详细范围proposal.json。
目的为减少上批投影类型缺口、补建筑消费者需要的接口。不得直接宣布35组均需新增实现；
已有wall_holes、wall_spatial_index、wall_constructor等名候选须检查ABI/原证据/实际调用，
名字泛化匹配（例如len）可能指向无关函数。前52组本身仍未验证或整合。

物理同址/同字节别名只支持受限复用；短体相同仍需核对返回类型、tag、指针和调用上下文。
同源文件但不同机器码的178组另列，不能当作下一批同码组。

## 推荐顺序

1. 从35组中先梳理几何与建筑接口，核对现有函数、共享类型和真实C调用参数，消除重复与未证投影。
2. 以装饰物附着和碰撞为第一个功能闭环：实际Source/asset/subtype输入→定位与collider/interactions→
   openings/wall geometry消费者→移动更新→移除清理；复用保存的DecoratorInfo72、Source88候选，
   优先关闭NativeID映射、anchor follow/cache transform和iter_modified，避免新建平行状态。
3. 并入MaterialStats14组和历史4接口：按实际重建/删除/撤销消费维护统计和事务，
   不把setter/getter当作完整材料系统或历史系统；统计字段/消费者需原码确认。
4. 屋顶ridge/trigger/UV定位作为后续功能包，需要时转入同源不同码范围，C-first另取证。

现有保存断点含Gate实体CPU、平台碰撞/DirtyTiles、矩形finish/cancel、DecoratorSource88、栅栏等
未验证候选，启动下一条功能链前应先读保存的冲突记录与源码，不重复恢复。

## 延后范围

UI/toolbar/tg-text、session/critters/audio/photo/Steam/crash等独立模块按消费者需要取用。
调试显示、生成空方法和纯容器胶水收益较低。公开库/std/Serde/Bevy的内部继续排除。
GPU/renderer仍后置；实际碰撞、TransientMesh所有权可按CPU消费者需要研究，不等于开启GPU恢复。

## 35组提议清单

| 分组 | 函数 | C观察体字节 | 原文件 |
|---|---|---:|---|
| 几何基础 | `utils::lerp` | 13 | crates/utils/src/lib.rs |
| 几何基础 | `utils::swizzlers::impl$1::xvy` | 27 | crates/utils/src/swizzlers.rs |
| 几何基础 | `utils::rng::init_rng` | 43 | crates/utils/src/rng.rs |
| 几何基础 | `utils::loop_range::LoopRange::len` | 8 | crates/utils/src/loop_range.rs |
| 几何基础 | `utils::geometry::aabb::Aabb2::width` | 10 | crates/utils/src/geometry/aabb.rs |
| 几何基础 | `utils::geometry::aabb::Aabb2::height` | 11 | crates/utils/src/geometry/aabb.rs |
| 几何基础 | `utils::geometry::aabb::Aabb2::contains` | 35 | crates/utils/src/geometry/aabb.rs |
| 几何基础 | `utils::geometry::rectangle::Rectangle2d::area` | 11 | crates/utils/src/geometry/rectangle.rs |
| 几何基础 | `utils::geometry::rectangle::Rectangle2d::perimeter` | 15 | crates/utils/src/geometry/rectangle.rs |
| 几何基础 | `utils::geometry::rectangle::Rectangle3d::extent` | 38 | crates/utils/src/geometry/rectangle.rs |
| 几何基础 | `utils::geometry::rectangle::Rectangle3d::translate` | 46 | crates/utils/src/geometry/rectangle.rs |
| 几何基础 | `utils::geometry::rectangle::Rectangle3d::set_orientation_xz` | 10 | crates/utils/src/geometry/rectangle.rs |
| 几何基础 | `utils::geometry::intersect::Triangle::new` | 44 | crates/utils/src/geometry/intersect.rs |
| 几何基础 | `utils::geometry::circle::Circle2d::contains_point` | 37 | crates/utils/src/geometry/circle.rs |
| 建筑数据接口 | `country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id` | 14 | crates/country-core/src/resources/walls/wall_attachment.rs |
| 建筑数据接口 | `country_core::resources::walls::wall_attachment::impl$1::default` | 3 | crates/country-core/src/resources/walls/wall_attachment.rs |
| 建筑数据接口 | `country_core::resources::walls::WallId::get_inner` | 4 | crates/country-core/src/resources/walls/mod.rs |
| 建筑数据接口 | `country_core::systems::wall::valid_enclosure::ShapeParameters::center` | 15 | crates/country-core/src/systems/wall/valid_enclosure/mod.rs |
| 建筑数据接口 | `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size` | 29 | crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs |
| 建筑数据接口 | `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index` | 12 | crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs |
| 建筑数据接口 | `country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length` | 6 | crates/country-core/src/resources/walls/wall_holes.rs |
| 建筑数据接口 | `country_core::resources::walls::wall_holes::FinalizedHoleStorage::inner` | 4 | crates/country-core/src/resources/walls/wall_holes.rs |
| 建筑数据接口 | `country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates` | 17 | crates/country-core/src/resources/walls/wall_holes.rs |
| 建筑数据接口 | `country_core::resources::walls::wall_holes::InProgressHoleStorage::holes` | 9 | crates/country-core/src/resources/walls/wall_holes.rs |
| 建筑数据接口 | `country_core::resources::walls::decorator_subtype::DecoratorLibKey::new` | 5 | crates/country-core/src/resources/walls/decorator_subtype.rs |
| 建筑数据接口 | `country_core::resources::walls::decorator::TransientDecoratorIdGen::new` | 19 | crates/country-core/src/resources/walls/decorator.rs |
| 建筑数据接口 | `country_core::resources::walls::decorator::DecoratorId::new` | 4 | crates/country-core/src/resources/walls/decorator.rs |
| 建筑数据接口 | `country_core::resources::walls::decorator::DecoratorRank::new` | 3 | crates/country-core/src/resources/walls/decorator.rs |
| 建筑数据接口 | `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_colliders` | 5 | crates/country-core/src/resources/walls/decorator_archivist.rs |
| 建筑数据接口 | `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_interactions` | 8 | crates/country-core/src/resources/walls/decorator_archivist.rs |
| 建筑数据接口 | `country_core::resources::roofs::roof_animation::RoofAnimation::new` | 7 | crates/country-core/src/resources/roofs/roof_animation.rs |
| 历史接口 | `country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence` | 27 | crates/country-core/src/resources/history/wall_compound_edit.rs |
| 历史接口 | `country_core::resources::history::history_chain::HistoryChainIndex::new` | 3 | crates/country-core/src/resources/history/mod.rs |
| 历史接口 | `country_core::resources::history::replay_cmd_buffer::ReplayBuffer::len` | 5 | crates/country-core/src/resources/history/replay_cmd_buffer.rs |
| 历史接口 | `country_core::systems::history::history_process::HistoryCmd::interrupt` | 6 | crates/country-core/src/systems/history/history_process.rs |

出处：skeleton/task-index/ABC-三版共同项/functions.jsonl.gz，以及checkpoints/building-parallel-2026-10-11T100110Z/CURRENT_IMPLEMENTATION.md。
