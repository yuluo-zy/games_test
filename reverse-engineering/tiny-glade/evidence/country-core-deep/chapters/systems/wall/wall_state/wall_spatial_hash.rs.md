# systems/wall/wall_state/wall_spatial_hash.rs

> 文件作用：墙空间哈希结构：固定 64×64 格（walls-audit）；add_segment 逐格填 0x38B 成员 {WallId, 段数据}（格条目 0x30B 链表）；remove_wall_id；get_cells_from_aabb/circle/rectangle2d/segment 栅格化查询（断言 RESAMPLING<cell_size 与 bbx 越界）。

函数总计 54：业务 23 / 内联无独立符号 10 / 派生胶水 21

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment::terrain_height_closest_to_ws_pos` | 0x140aa50e0 | 39 | WallSegment::terrain_height_closest_to_ws_pos（f64 距离加权地形高） | O | wall\decomp\decomp-0x140aa50e0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::new` | 0x140aa5150 | 109 | 构造/初始化 | I | wall\decomp\decomp-0x140aa5150.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::remove_wall_id` | 0x140aa5360 | 120 | WallSpatialHash::remove_wall_id（hashbrown erase） | O | wall\decomp\decomp-0x140aa5360.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query` | 0x140aa5730 | 130 | 查询/计算 | I | wall\decomp\decomp-0x140aa5730.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_segment` | 0x140aa5850 | 146 | 查询/计算 | I | wall\decomp\decomp-0x140aa5850.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle` | 0x140aa5940 | 162 | 查询/计算 | I | wall\decomp\decomp-0x140aa5940.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<forest::manage_tree_prefabs::manage_tree_prefabs::closure_env$0>` | 0x1422785d0 | 179 | WallSpatialHash::query_with_circle_callback 闭包实例（圆查询：f64 网格坐标换算 + 半径过滤 + 逐格成员距离校验 + bbx_min/RESAMPLING 断言） | O | wall\decomp\decomp-0x1422785d0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::convert_preplaced_trees::convert_preplaced_trees::closure_env$0>` | 0x1425defd0 | 179 | WallSpatialHash::query_with_circle_callback 闭包实例（同 0x1422785d0） | O | wall\decomp\decomp-0x1425defd0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0>` | 0x1425df320 | 179 | WallSpatialHash::query_with_circle_callback 闭包实例（同 0x1422785d0，含 bbx_max 断言） | O | wall\decomp\decomp-0x1425df320.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_aabb_clamped` | 0x140aa5a10 | 195 | 查询/计算 | I | wall\decomp\decomp-0x140aa5a10.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_rectangle2d` | 0x140aa5ba0 | 217 | 查询/计算 | I | wall\decomp\decomp-0x140aa5ba0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_rectangle2d<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_rectangle2d::closure_env$0>` | 0x1409ef210 | 232 | WallSpatialHash::new（bbx 越界断言族） | O | wall\decomp\decomp-0x1409ef210.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::add_segment::closure_env$0>` | 0x1409ef870 | 273 | 累加/追加 | I | wall\decomp\decomp-0x1409ef870.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_segment<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_segment::closure_env$0>` | 0x1409eff80 | 273 | 访问器/转换 | I | wall\decomp\decomp-0x1409eff80.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2>` | 0x1409f0e80 | 381 | 访问器/转换 | I | wall\decomp\decomp-0x1409f0e80.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_circle::closure_env$0>` | 0x1409f08f0 | 381 | 访问器/转换 | I | wall\decomp\decomp-0x1409f08f0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_center` | 0x140aa5c70 | 409 | WallSpatialHash 查询/换算方法（[+0x48] 分支，0x110 帧） | I | wall\decomp\decomp-0x140aa5c70.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_aabb<country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_aabb_clamped::closure_env$0>` | 0x1409f12e0 | 423 | 访问器/转换 | I | wall\decomp\decomp-0x1409f12e0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::add_segment` | 0x140aa5dd0 | 454 | WallSpatialHash::add_segment（逐格 0x38B 成员 push） | O | wall\decomp\decomp-0x140aa5dd0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord_unclamped` | 0x140aa6520 | 497 | world_space_to_grid_coord 浮点换算（[+0x50]/[+0x54] + 常量） | O | wall\decomp\decomp-0x140aa6520.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index` | 0x140aa65d0 | 521 | grid_coord_to_cell_index（gy×dims.x+gx：imul [rcx+0x48]） | O | wall\decomp\decomp-0x140aa65d0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size` | 0x140aa65e0 | 538 | 浮点网格坐标换算（cvtsi2ss [+0x48]/[+0x50]/[+0x54]） | O | wall\decomp\decomp-0x140aa65e0.txt |
| `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::bounds_ws` | 0x140aa6600 | 545 | cell 中心 f64 换算（movddup [+0x50] × 常量） | O | wall\decomp\decomp-0x140aa6600.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::new`（行 64）— 构造
- `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::add`（行 71）— 构造/注册/生成
- `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::remove_segments`（行 79）— 删除/清理
- `country_core::systems::wall::wall_state::wall_spatial_hash::GridCell::walls`（行 84）— 待推断
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle`（行 381）— 查询/计算
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_aabb`（行 423）— 查询/计算
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord`（行 505）— 待推断
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_index_to_grid_coord`（行 527）— 待推断
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cell_at_index`（行 534）— 查询/计算
- `country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::dims`（行 552）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure` 等 21 项
