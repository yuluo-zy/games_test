# REVIEW — impl-host-wiring 复验（组 state：宿主接线库内部分闭合）

> 审查员：reviewer-state，2026-10-17。复核 impl-host-wiring（REPORTS-HOST-WIRING.md，库内 5 文件改动 + 1 登记）。
> 编译基线：`cargo check -p country-core --offline --lib` **通过**（13 警告，含他组在途）；
> **全 crate `--lib` 测试 1842/0 全部通过**（含 geometry::sphere 既知败已由 geometry 组修复；
> 并发组 in-flight 全部收口）——这是 country-core 库内测试首次全绿。

## 逐项复核

| 项 | verdict | 说明 |
|---|---|---|
| register_core_startup 补齐域系统 | **OK** | 会话流/命令器/GameConfig/事件注册/域系统（audio→history→会话切换→save→terrain→A 块 ivy/garden/input/sheep）→ **wall 域**（register_wall_cpu_resources + add_wall_cpu_systems：InnerWalls/PublicWalls/DecoratorStorage 权威存储 + 编辑/发布/空间哈希/洞/砖/删除链 + 墙高度/录制/屋顶事件）→ **History 资源 empty_glade**（存在则不重复插入）。与 0x140941110 调度合同/run_game_thread 359 调用序一致。 |
| ⚠️ add_game_startup_systems 纠偏 | **OK** | mod.rs 注释已纠偏：0x140941110 = **5 次 SubApp::add_systems**（PreStartup → OnEnter<AppLoadingState> → OnExit<AppLoadingState> → 再两次），非单 Startup；组构建器 G1-G6 签名登记（G1 10 系统 load_textures/load_shader 变体 / G2 8 系统 init_photomode/… / G3 PrefabLibrary / G4 / G5 11 系统 mesh+shader+AudioCore+ImageFormationConfigs / G6 7 系统）。现可挂载 G2 mesh 组。 |
| 域采集 + init 写回 | **OK** | capture_snapshot_system（InnerWalls/TerrainEditingData/StairsState 采集，缺失资源走 default 面）+ apply_stairs_writeback（0x1408ff590）/apply_raster_writeback（0x14083ebe0）/decode_raster_payload——与 restore_snapshot checksum 链 / AllWorldRasterWrite 逐槽语义对齐。 |
| 宿主登记点诚实 | **OK（登记）** | 全局调度序（PreStartup/OnEnter/OnExit 挂点由宿主）、资源供给（AssetMeshLibrary/Studio/AppMode/History/SSBO）、G1-G4 asset 载入、SparkleSsbo 写面粒子域、RoofTileSsbo 形态、init 全链。 |
| record_wall_history 接入 Record 槽 | **OK** | systems/history.rs HistoryCpuStage::Record 挂 `record_wall_history::<S>`（真实资源/事件，非空系统）。 |

## 结论与 DISPATCH 建议
- **宿主接线库内部分 → 闭合（实现层面）**。register_core_startup 全装配（含墙域/History 资源）与
  0x140941110 调度合同一致；add_game_startup_systems 5×SubApp 纠偏注释已落；域采集/写回与快照链对齐；
  record_wall_history 入 Record。**全 crate 1842/0 测试通过**。
- 残留登记（不伪造）：全局调度挂点、资源供给、G1-G4 asset 组、SparkleSsbo/RoofTileSsbo、init 全链
  （宿主/渲染/粒子域）。
- save/session/music/history/startup/roofs/stairs/meta_commander → **review_passed（限定已实现范围）**；
  库内完整性由 completeness-audit 判定（无假实现）+ 全绿测试基线。完整 core 验收仍 blocked（宿主调度挂点 +
  render/asset/粒子域接线 + 全局游戏线程装配）。
