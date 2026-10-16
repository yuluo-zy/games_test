# REVIEW — render 消费桥(卡H 终验 → 闭合)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer RESTORE.md(reports/interaction/render_bridge/,本卡未改 src);
`cargo check --lib` exit 0(独立复验,3 警告)。

## 核对

### ① render 消费端接线 — PASS
- ivy GPU:`instanced_ivy.rs` `read_storage_buffer<IvySegmentSsbo>` + `<IvyFlowerSsbo|
  IvyLeafSsbo>`(绑定名 0x3dce10f97920242d)✓;提交侧 `ivy_send_to_gpu_leaf.rs`/
  `ivy_send_to_gpu_segments.rs` 既有,消费→提交闭环 ✓。
- garden pin:`draw_garden.rs`(draw_raster)既有 ✓。
- terrain 回读:`read_terrain_data.rs`(TerrainHeightSsbo/VisualTerrainHeightSsbo + 卡7
  mask 四类已建)✓。
- world raster:`compute_passes/upload_world_raster.rs` 既有 ✓。

### ② GPU 提交 — PASS
ivy send_to_gpu 经 `render/nani` 抽象提交,缓冲注册(ssbo_library add_new_with_data)既有 ✓。

### ③ 全局调度 — PASS(附措辞注意)
`startup/mod.rs::add_game_startup_systems`(0x140941110 合同)→ `run_game::
register_core_startup` 挂载核心子集(会话/存档/历史/音频/meta-commander/LoadGameManager)✓。
措辞注意:原版"28-系统 tuple"为**合同**,当前实际挂载的是核心子集;域系统(墙/屋顶/地形/
水…)由各域恢复后并入(文档明确"不在这里注册空系统")——诚实,非缺失冒充。

### ④ 登记待接 — PASS
宿主 `run_game` 顶层完整渲染循环(draw_shaders/compute_passes 执行序)由宿主层接线,
库内仅提供合同(登记);ivy/garden 实体→SSBO 数据同步(实例布局 48B)逐字段待取证。
无冒充。

## 结论与 DISPATCH 建议

- 卡H 复验通过:**render 消费桥 + GPU 提交 + startup 合同闭合**;宿主顶层渲染序与
  ivy/garden 48B 布局为登记待接。
- 建议:卡H → 闭合;第三批 A-H 全部闭合。
- 全 lib 编译 exit 0(3 警告;此前 audio_resources 瞬态已由 audio 组修复)。
