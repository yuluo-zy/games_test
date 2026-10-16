# RESTORE — render 消费桥（卡H，order8）

实现者: restorer-interaction
日期: 2026-10-16

## 核对结论（render 消费端已实质接线，本批复核通过）

1. **render/ 模块消费端**：
   - ivy GPU：`render_loop/draw_passes/draw_shaders/instanced_ivy.rs`
     `read_storage_buffer<IvyFlowerSsbo|IvyLeafSsbo>`（绑定名 0x3dce10f97920242d）+ `ssbo_tags.rs`；
     提交侧 `systems/ivy/ivy_send_to_gpu_leaf.rs` / `ivy_send_to_gpu_segments.rs` 既有。
   - garden pin：`systems/garden/draw_garden.rs`（draw_raster）既有。
   - terrain 回读：`read_terrain_data.rs`（TerrainHeightSsbo/VisualTerrainHeightSsbo +
     卡7 mask 四类已建）既有。
   - world raster：`compute_passes/upload_world_raster.rs` 既有。
2. **GPU 提交接线**：ivy send_to_gpu 的 GraphicsDevice 提交经 `render/nani` 抽象；
   缓冲注册（ssbo_library add_new_with_data）既有。
3. **全局调度**：`startup/mod.rs::add_game_startup_systems`（0x140941110 合同）→
   `startup/run_game.rs::register_core_startup`（28 系统 tuple 挂载）既有。
4. **宿主 tiny-glade 接线**：`run_game::register_core_startup` 为宿主挂载点，已登记。

## 登记（宿主层/待接）

- 宿主 `run_game` 顶层完整渲染循环（draw_shaders/compute_passes 执行序）由宿主
  tiny-glade 层接线，库内仅提供合同（登记）。
- ivy/garden 的实体→SSBO 数据同步细节（实例布局 48B）逐字段待取证。

## 编译状态

`cargo check -p country-core --lib` 0 error（本卡未改 src，仅核对+登记）。

## 留给 reviewer 的核对点

1. instanced_ivy read_storage_buffer<IvyLeafSsbo> 消费与 ivy_send_to_gpu_* 提交闭环。
2. register_core_startup 挂载序与 0x140941110 合同一致。
