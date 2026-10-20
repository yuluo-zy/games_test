# 第 10 轮协调台账（2026-10-19）

用户指示：按五条链的调用顺序继续推进。第 9 轮已闭合读档主链与 world 构图主链，
本轮聚焦三条剩余链的**调用顺序**推进：启动→装配→主循环、读档残留+编辑历史回放、
帧循环 compile_main 尾部。

结构：3 组 × 3 agent（①逆向契约 / ②实现 / ③review）。

## G1 启动 → 装配 → 主循环

文件权（②写/新增）：`startup/**`、`systems/camera_rig/**`、`systems/main_camera.rs`、
`startup/{mod,run_game}.rs`；`crates/tiny-glade/src/run_game.rs` 仅 startup 注册与相机插入局部。

已知缺口：
1. startup 13 个骨架模块无源码：`clearing`、`load_audio`、`load_glade_grading`、
   `spawn_brush_previews`、`spawn_human_ref`、`spawn_preplaced_trees`、`startup_critters`、
   `startup_meadow`、`startup_roof`、`startup_sheep`、`startup_sky_and_sun`、
   `startup_time_of_day`、`startup_walls`。
2. 主相机链缺：`main_camera_update`、`paused_main_camera_update`、`reset_camera`、
   `camera_snapshot`、`build`；`camera_rig/walk.rs`、`butterfly.rs` 整文件缺；
   相机系统未进 Schedule。
3. 装配面未调用：`startup/mod.rs:54 add_game_startup_systems`、`configure_glade_audio`、
   `configure_music_consumers`、`add_ui_systems`、`add_backend_systems`、
   `add_collision_systems`、`add_prefab_systems`。

验收门：每个模块真实资源写入/实体 spawn + 消费者；相机进 update Schedule；
`cargo check -p country-core` 与 `-p tiny-glade` 通过；新增测试 0 失败。

产物：`contracts/g1-startup-mainloop-CONTRACT.md`、
`deliveries/g1-startup-mainloop-DELIVERY.md`、`reviews/g1-review.md`。

## G2 读档残留 + 编辑历史回放

文件权（②写/新增）：`systems/history/**`、`systems/history.rs`、`resources/history/**`、
`systems/save/snapshot.rs`（仅读档残留 apply 段）、`crates/systems/tree-tool/**`（仅适配）。

已知缺口：
1. 回放消费者 writer=0 reader=0：`ColorHistoryReplayApply`、`DecoratorHistoryReplayApply`、
   `WallHistoryReplayAudio`、`WallHistoryRestoreColor`；楼梯回放音频载荷全 0。
2. Pop 未注册：`systems/history.rs:532` 只有 `pop_empty_replay_bucket`；
   `replay_pop_bucket_system`（:402，含 aux/碰撞恢复）、`history_clear_previous_state`（:451）未注册。
3. 读档残留：`RoofAssociations` 未重建；空 prefab 树回退属 tree-tool `random_tree_prefab`；
   Clutter `to_info` 扩展簇；`OnTerrainChange` 有 writer 无 reader。

验收门：consumer 真实写回目标资源（undo/progress 参与计算）；Pop 顺序正确不重复；
读档残留真接或诚实登记；save 49 测试与 history 测试全绿。

产物：`contracts/g2-replay-consumers-CONTRACT.md`、
`deliveries/g2-replay-consumers-DELIVERY.md`、`reviews/g2-review.md`。

## G3 帧循环 compile_main 尾部

文件权（②写/新增）：`crates/tiny-glade/src/{world_postprocess,effects_host,production_renderer,render_bridge}.rs`、
`systems/render_loop/**` 尾部 pass 相关；必要时 `frame_setup/scene_raster` 局部。

已知缺口：warm-up builder 只覆盖 W1–W54 + TAA 导出，未含原生 compile_main 闭包尾部
grading / blur / final_post / UI；`render_bridge.rs:883-884` 的“complete compile_main”
声明暂不成立。

验收门：尾部 pass 真实声明并进入图；导出与 present 一致；等价性诚实判定；
`cargo check` 两包通过、`cargo test -p country-core --lib` 0 失败；GPU 真机仍后置。

产物：`contracts/g3-compile-tail-CONTRACT.md`、
`deliveries/g3-compile-tail-DELIVERY.md`、`reviews/g3-review.md`。

## 中央裁决规则

- 三组文件权互不重叠；跨界需求先报中央。
- 当前基线（第 9 轮末）：`cargo check -p country-core` ✅、`-p tiny-glade` ✅、
  `cargo test -p country-core --lib` 2467 passed / 0 failed。
- 任何组不得破坏该基线；若必须临时调整，验后还原并在交付注明。
- 禁止静默 no-op、测试 double 冒充生产、log 冒充写入、伪造字段/常量/API。
- 用户重构区（rhapsody 本体、roof、wall 含 snapping/signifier）不可改。

## 并发限制（用户指示 2026-10-19）

**最多 5 个 agent 同时运行。** 超出时按“契约优先、实现次之、复核等待”排队：
当前在跑 5 个 = G1① 契约、G1② 实现、G2① 契约、G2② 实现、G3① 契约。
排队：G3② 实现（等 G3① 契约落盘或槽位释放）、G1③/G2③/G3③ 三个复核
（等对应 DELIVERY 落盘且槽位释放）。每有一个 agent 结束，中央只补 1 个新 agent，
始终保持 ≤5。


## 最终结果（2026-10-19）

### 编译 / 测试（真树，中央复跑）
- `cargo check -p country-core` → Finished
- `cargo check -p tiny-glade` → Finished
- `cargo test -p country-core --lib` → 2510 passed / 0 failed
- `cargo test -p tiny-glade --lib` → 37 passed / 0 failed

### G1 启动 → 装配 → 主循环：修复后有条件通过（未二次送审）
- 13 个 startup 模块已创建；主相机三系统进 host Update set6，真实写 MainCamera 四矩阵；Walk/Butterfly 为真实 dolly。
- G1③ 首轮不通过（水 11，阻断 6）；G1② 已修：重复注册、brush 表错位、不存在 mesh 引用、空 half_cube 注册、human_ref 假资产、host 接线。中央抽查确认 host run_game.rs:507-513 直接注册 batch2，无重复；不存在的 mesh 引用已删。
- 仍显式 U：meadow 1×0x88 spawn、grading 生产调用者、鼠标深度/自由相机输入源、firefly 通道、half_cube 几何、FMOD owner。
- 未做二次 ③ 复核，只能记“有条件通过”。

### G2 读档残留 + 编辑历史回放：实现完成，复核未闭合
- 四类 consumer 已按契约重写；Pop 顺序修正；RoofAssociations 改 Pending 重建、空 prefab 走 InsertTreeCmd sentinel、Clutter 保留原始槽、FlatRoof 发 SetFlatRoofStyleCmd。
- G2③ 水 2；G2② 已修 6 项：Pop 先校验后弹桶、颜色无 panic、树 sentinel 走 tree-tool、RoofAssociations 后置重建、FlatRoof、撤回楼梯伪字段。
- 残留跨 crate 接缝：FlatRoof consumer 在 system_roof、树 sentinel 解析在 tree-tool、stairs 音频系统只读 kind/position。

### G3 帧循环 compile_main 尾部：通过（范围限定）
- 尾部按原生序接：flattened → blur → final_post(msaa) → menu/painterly/vermeer/logo → wireframe/ui/mei/cursor/loading；导出改为 "msaa color out"（0x2b、4×MSAA），present 走 MSAA resolve。
- 明确判定：W1–W54 + 尾部子集，不是完整 compile_main；T11/T12/T13/T16/T21 未恢复，U 控制默认 off；load_glade_grading 未恢复时 tail 返回命名硬错误。
- 中央复核：水 0。

### 本轮结束后状态
- 五条链：启动(部分) / 读档(部分) / 编辑历史(部分) / 构图(部分) / 存档(基本通)。
- 未做：G1 二次复核、GPU 真机、FlatRoof/tree-tool/stairs 音频跨 crate 收口。
