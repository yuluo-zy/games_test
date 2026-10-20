# R10 G1（启动 → 装配 → 主循环）复核

复核时间：2026-10-19 13:18（代码快照：`main_camera.rs` 13:15:24，`startup/mod.rs` 12:57:56，
`startup/run_game.rs` 12:54:06；contract 落盘 13:05，delivery 落盘 13:17）
复核者：G1③（只读）
对象：`contracts/g1-startup-mainloop-CONTRACT.md`、`deliveries/g1-startup-mainloop-DELIVERY.md`、
`crates/country-core/src/startup/**`、`systems/camera_rig/**`、`systems/main_camera.rs`、
`crates/tiny-glade/src/run_game.rs`

## 结论：**不通过**

编译与全量单测已绿（见 §6），主相机三系统已真实进入宿主 Update 并写 `MainCamera` 四矩阵，
Walk/Butterfly rig 是真实 dolly 实现。但启动链的验收门未达到：

1. **装配入口仍是死代码**：`add_game_startup_systems` 全库无生产调用者，`register_core_startup`
   只被它调用（循环）；13 个模块的网格产者在生产 App 中一次也不会运行。
2. **多个“真实注册”实际指向不存在的资产文件**：`clearing/tree.json`、`clearing/bush.json`、
   `human_ref_cube.json`、`half_cube.json`、`snapping_line.json`、`snapping_circle.json` 均不存在；
   `register_mesh_file` 内部 `load_json_as_mesh(...).unwrap()`，一旦接线即 panic。
3. **`spawn_brush_previews` 的名称/文件配对与契约 O 证据不符**（整体错位一位、含重复项、
   计数 22 对 24、shader 也错），属伪造常量。
4. **契约 §1.5/§7 要求的 PreStartup + OnEnter(Game) 四批装配未实现**；仍沿用旧三表签名。
5. delivery 的 §3 对上述资产逐条宣称“真实注册”，与磁盘事实不符。

因此按契约 §6 验收门判定为不通过；编译/相机部分可作为有条件保留的增量。

## 水分计数：11 项（阻断 4、高 4、中 2、低 1）

| # | 级别 | 位置 | 水分事实 |
|---|---|---|---|
| W1 | 阻断 | `startup/spawn_brush_previews.rs:16-27,30-43,54-66` | 名称↔文件整体错位：`brush_arrow→brush_circle.json`、`brush_circle→brush_circle_cross.json`、`temp_sphere→hand.json`；`snapping_line/snapping_circle` 重复且无文件；表 10+12=22，自述 13 名式+11 文件=24；shader 用 `vertex_color` 而非契约 O 的 `/timed_terrain_gizmo.raster.hlsl`；17 条 spawn 记录一条未建 |
| W2 | 阻断 | `startup/clearing.rs:24-32`；`startup/spawn_preplaced_trees.rs:16-19` | 引用不存在的 `assets/meshes/clearing/tree.json`、`bush.json`（实际为 `assets/prefabs/clearing/tree/*.ron`，且契约 §2.1 的 `init_clearing` 产出只有 ice_cracks + distant_billboard）；接线即 `unwrap` panic |
| W3 | 阻断 | `startup/startup_walls.rs:25,29-31,43-49` | `half_cube.json` 不存在；`create_half_cube_mesh` 返回空属性/空索引 `Mesh::new`，却被 `startup_walls_meshes` 以真名写入生产 `AssetMeshLibrary`（假几何资源）；无 `TerrainHeightsData::subscribe`、无 7×0x88 spawn |
| W4 | 阻断 | `startup/spawn_human_ref.rs:12,21-23` | `human_ref_cube.json` 不存在，却由 `spawn_human_ref_meshes` 宣称真实注册；契约 O 的 Box 常量与 0x40 bundle 未实现 |
| W5 | 高 | `startup/startup_meadow.rs:15,32-51` | `let _ = &meshes;` 后只读文件并 `log::info!`，不产出任何资源/实体；路径 `assets/meshes/plants.json` 错误（实际 `assets/glade/<theme>/plants.json`）；契约要求 1×0x88 drawable |
| W6 | 高 | 9 个 host-trait 函数（clearing/load_audio/load_glade_grading/spawn_human_ref/startup_critters/startup_roof/startup_sheep/startup_sky_and_sun/startup_walls） | `impl StartupAssetHost/AudioStartupHost/GradingHost` 全库仅存在于各文件 `mod tests` 的 `Rec`；无生产实现 → `load_shader`/`spawn_drawable` 在生产永不发生（测试 double 冒充生产） |
| W7 | 高 | `startup/mod.rs:121`、`startup/run_game.rs:65` | `add_game_startup_systems` 全库零调用；delivery §6:127 称“真实调用入口为宿主 `register_core_startup`”，但后者只被 `add_game_startup_systems` 调用，构成自环；11 个 mesh producer 生产不可达 |
| W8 | 中 | `startup/mod.rs:144` + `startup/run_game.rs:151` | `add_startup_mesh_systems` 被注册两次（`add_game_startup_systems` 一次，`register_startup_systems` 一次）；每系统 `run_once` 各跑一次 → 网格重复写入、缺失文件会 panic 两次 |
| W9 | 中 | `systems/main_camera.rs:68-77`；`crates/tiny-glade/src/run_game.rs:367` | `MainCameraUpdateInput` 仅被 `insert_resource(default)`，全库无写入者；`main_camera_update` 实际永远读默认值（Normal/depth=0/非自由），delivery §4:79 称其承载真实输入字段 |
| W10 | 中 | `startup/load_audio.rs:13-26`；`startup/load_glade_grading.rs:10-38` | `load_audio` 只登记 3 个尺寸常量，未注册契约 §2.2 的 2 个 game-loading 系统，也无调用者；`load_glade_grading` 缺 `grading.json`、`RoastMeat_Cinema_709_1`、`unwrap` 语义，且无调用者 |
| W11 | 低 | `systems/main_camera.rs:218-241,245-262,268-301` | `camera_snapshot` 只写 `value/active` 无消费者；`reset_camera`、`init_walk_camera`/`teleport` 只在测试中调用，未进任何 Schedule |

未计水分的正向项：`startup_time_of_day` 真实读取 `assets/tod/default.json`（文件存在且结构匹配）；
sky/critters/sheep/roof 的网格文件均存在；Butterfly 的 1000.0 吸附与 Walk/Butterfly rig 均为真实实现。

## 逐项复核

### 1. 水分检查（逐条）

见上表 W1–W11。补充：空函数体/`todo!` 未发现；`U` 标注总体诚实，但 delivery 把 U 项在
§3 表格“真实产出”列写成了已完成（W1/W2/W3/W4/W5），属“注释标 U 却宣称完成”。

### 2. 启动顺序：实际系统表 vs 契约

代码实际顺序（`startup/run_game.rs`）：

```
register_core_startup                        (run_game.rs:65)
  1. initialize_core_resources(world)        (:95)
  2. configure_mode_manager(update)          (:76)
  3. update += synchronize_mode_manager_ui_context  (:81)
  4. register_startup_systems(startup)       (:139)
       - refresh_session_list_system [run_once]
       - music_audio_init_system::<B> [not exists MusicAudioState]
       - add_startup_mesh_systems            (:151)   ← 11 个 mesh producer, chain+run_once
  5. register_session_systems(session)       (:152)
       - (commander_update_system, load_game_session).chain()
  6. register_update_systems(update)         (:165)
       - delete_session / refresh_session_list (events)
       - add_audio_systems / history / save / terrain / terrain_tool / wall / wall_drawing
       - ivy / garden / input_cpu_consumers / sheep / time_of_day
       - main_camera_update, paused_main_camera_update, camera_snapshot
add_game_startup_systems                      (mod.rs:121)
  - register_core_startup
  - startup += (startup_garden_brush, startup_ivy) [run_once]
  - add_startup_mesh_systems(startup)         (:144)   ← 与 :151 重复
```

与契约 §1.5 的偏差：
- 契约要求 **PreStartup batch1 / OnEnter(Game) batch2–4** 四批；当前没有 OnEnter、没有 batch 划分，
  也没有 `once_startup_walls.after(load_game_profile)`、`uninit_clearing`、`spawn_preplaced_trees`
  跟随 save 注册。
- 契约 §7.2 要求改写 `add_game_startup_systems` 为四批；未做。
- 重复注册（W8）与死入口（W7）使“顺序一致”无从成立。

### 3. 13 个 startup 模块

| 模块 | 状态 | 产出（实际） | 消费者 | 主要问题 |
|---|---|---|---|---|
| clearing | 部分 | `init_clearing_meshes` 写 4 网格（其中 tree/bush 文件缺失） | `AssetMeshLibrary` | W2；缺 2 drawable、cap=1100、uninit 27×0xc0 |
| load_audio | 仅骨架 | 3 个尺寸记录（0x1c/0xc/8） | 无 | W6/W10；契约 2 系统未入表 |
| load_glade_grading | 仅骨架 | 路径合成 + default 回退 | 无 | W6/W10；无 `grading.json`/LUT 名 |
| spawn_brush_previews | 仅骨架 | 22 条网格名（表伪造，2 条无文件） | `AssetMeshLibrary` | W1；0 spawn |
| spawn_human_ref | 仅骨架 | mesh 文件缺失 | 无 | W4/W6；0 spawn |
| spawn_preplaced_trees | 缺失 | 2 条错误网格注册 + JSON→Value | `AssetMeshLibrary` | W2；无 `TreeStorage`、无实体 |
| startup_critters | 部分 | 4 网格真实注册（文件存在） | `AssetMeshLibrary` | 缺 firefly 网格；4 spawn 仅 host 测试 |
| startup_meadow | 水分 | 只 log 条目数 | 无 | W5 |
| startup_roof | 部分 | 4 网格文本真实（文件存在） | `AssetMeshLibrary` | shader/spawn 仅 host 测试；契约 2×0x78 未建 |
| startup_sheep | 部分 | 5 网格文本真实（文件存在） | `AssetMeshLibrary` | 无 shader 编译、无实体/动画链 |
| startup_sky_and_sun | 部分 | skydome 网格真实 | `AssetMeshLibrary` | shader/spawn 仅 host 测试 |
| startup_time_of_day | 部分 | 读真实 JSON → `TodSequence` 资源 | `resources/time_of_day` | 契约 §2.12 写 `TimeOfDay`（I），目标资源不一致；只在死装配链中 |
| startup_walls | 仅骨架 | 2 shader 仅 host；空 `Mesh` 写入生产库 | `AssetMeshLibrary` | W3 |

### 4. 主相机链

| 项 | 结论 | 证据 |
|---|---|---|
| `main_camera_update` 真读输入并写 `MainCamera` | **部分** | `systems/main_camera.rs:133-191` 写 aspect、+0x344、mode、正文矩阵（`update_frame_matrices` 写 0x100 块）；但输入来自 W9 的默认资源；契约 9 参数 vs 实现 7 参数 |
| `paused_main_camera_update` | 真实 | `:194-214`，模式 body dt=0 更新 |
| `camera_snapshot` | 部分 | `:218-241` 只推进 value/active，27 模式体 U，且无消费者 |
| `reset_camera` | 实现但不可达 | `:245-262`；全库仅测试注册 |
| Walk | 真实（rig）但入口未接 | `camera_rig/walk.rs` 全部实现；`init_walk_camera` `:268` 无生产调用者，地形采样 U |
| Butterfly | 真实 | `camera_rig/butterfly.rs` 六 driver + 1000.0 吸附；`selected_index` 无写入者 |
| 进 Schedule | **是（宿主）** | `crates/tiny-glade/src/run_game.rs:370-392` 三条系统进 `Update`/`GameSystemSet(6)`，与 `GameClock.paused` 互斥；`prepare_frame_constants.rs:118` 每帧读 `MainCamera` 矩阵 |
| 双注册风险 | 存在 | 若 `add_game_startup_systems` 被接线，`startup/run_game.rs:196-227` 会再注册一次相机系统 |

### 5. 装配面

| 入口 | 定义 | 生产调用者 | 结论 |
|---|---|---|---|
| `add_game_startup_systems` | `startup/mod.rs:121` | **0** | 死代码（W7） |
| `configure_glade_audio` | `systems/audio/music/load_glade_audio.rs:22` | **0** | 未接（契约 §5.1 要求 ActiveSession 之后） |
| `configure_music_consumers` | `systems/audio/music/music_system.rs:54` | **0** | 未接（契约要求 Update） |
| `add_ui_systems` / `add_collision_systems` / `add_prefab_systems` | **函数不存在** | — | 契约 §5.1 已点明 `add_ui` 缺失；collision/prefab 为插件入口（U） |
| `add_backend_systems` | **函数不存在**（仅 `tiny-glade/src/run_game.rs:457` 注释） | — | 契约 §5 要求补等价入口 |
| `add_startup_mesh_systems` | `startup/mod.rs:99` | 2 处（重复，W8） | 需去重 |

### 6. 编译与测试（本机实测）

| 命令 | 实测 | 说明 |
|---|---|---|
| `CARGO_TARGET_DIR=/d/game/tiny-glade/target cargo check -p country-core` | **exit 0**，165 warnings | 13:17 快照，`Finished dev ... 6.01s` |
| `cargo check -p tiny-glade` | **exit 0**，7 warnings | `Finished dev ... 0.94s` |
| `cargo test -p country-core --lib` | **2509 passed / 0 failed / 0 ignored**（14.12s） | 基线 2467 → 新增 42 条；13:18 快照 |

过程记录：13:04 快照曾出现 3 条 G1 相机测试失败（free 模式未写 mode byte、walk/reset 断言错），
G1② 在 13:15 修复；13:04–13:07 的 `tiny-glade`/test 失败由 G2 并发编辑（`decorator_edit`、
`replay_wall`）造成，非 G1 文件。最终快照全绿。

### 7. 缺口继承诚实性

- delivery §3 对 13 模块逐项列出“仍缺（U）”，方向诚实；但对 `clearing/tree/bush`、
  `half_cube`、`human_ref_cube`、`snapping_*`、`plants.json` 的路径存在性未核对，把“文件缺失”
  写成“真实注册”（W1–W5）。
- delivery §6:127 对 `add_game_startup_systems` 的生产可达性判断错误（W7）。
- delivery §7 承认 contract 未出现、装配顺序待对齐；contract 已落盘，相关待办应转为缺陷，
  不能继续作为“待确认”豁免。

## 必须修复清单

**阻断（接线前必须）**

1. `startup/mod.rs:144`（或 `startup/run_game.rs:151`）：删除其中一处 `add_startup_mesh_systems`
   调用，保证唯一注册（W8）。
2. `startup/spawn_brush_previews.rs:16-27,30-43,54-66`：按契约 §2.4 O 表重建——
   `brush_arrow/brush_arrow.json`、`brush_circle`、`brush_circle_cross`、`brush_circle_scribble`、
   `brush_arrow_flat`、`brush_4_arrows→brush_terrain_edit`、`brush_2_arrows→brush_flat_roof_edit`、
   `brush_5_replay→brush_replay`、`hand→temp_sphere`、`dashed_circle`、`dashed_lines→snapping_line`；
   shader 改 `/timed_terrain_gizmo.raster.hlsl`；删除 `snapping_circle` 与重复项；不存在的文件
   不得传给 `register_mesh_file`（W1）。
3. `startup/clearing.rs:24-32`、`startup/spawn_preplaced_trees.rs:16-19`：删除
   `assets/meshes/clearing/{tree,bush}.json`；clearing 按契约只建 ice_cracks（theme 4）+
   distant_billboard drawable、cap=`min(x,0x44c)`；preplaced trees 从
   `<assets_root>/assets/glade/<theme>/trees.json` 读并写 `TreeStorage`（W2）。
4. `startup/startup_walls.rs:25,29-31,43-49`：`half_cube.json` 不存在，不得注册；`Mesh::new` 空网格
   不得进入生产 `AssetMeshLibrary`；补 `TerrainHeightsData::subscribe` 或显式 U 并跳过（W3）。
5. `startup/spawn_human_ref.rs:12,21-23`：`human_ref_cube.json` 不存在，先查真实命名网格来源；
   查不到则只保留常量与 U，不调用 `register_mesh_file`（W4）。
6. `startup/mod.rs:121`：把 `add_game_startup_systems` 接入真实宿主（`tiny-glade` 的
   `run_game_thread`/Loading 或 game_loop），并按契约 §1.5 改为 PreStartup + OnEnter(Game)
   四批；接线前先完成 2–5，否则启动 panic（W7）。

**高**

7. `startup/startup_meadow.rs:15,32-51`：改真实 per-glade plants 路径；去掉 `let _ = &meshes`；
   产出契约要求的 1×0x88 drawable，否则不要注册该 system（W5）。
8. 为 `StartupAssetHost` 增加生产实现（持有 `AssetShaderLibrary`/`AssetMeshLibrary`/`Commands`/设备），
   在宿主装配中驱动 `init_clearing/startup_critters/startup_roof/startup_sheep/sky/human_ref/
   glade_startup_walls`；否则 shader 与实体仍为 0（W6）。
9. `startup/load_audio.rs:13-26`、`startup/load_glade_grading.rs:10-38`：按契约补
   game-loading 2 系统 / `grading.json` + 两 LUT；并接调用者（W10）。
10. `systems/main_camera.rs:68-77` + `crates/tiny-glade/src/run_game.rs:367`：为
    `MainCameraUpdateInput` 增加真实输入/设置生产者（ActionSetValues、CursorPositionSS、
    free-camera 开关、动作模式），或删除适配资源并直接读原资源（W9）。

**中/低**

11. `systems/main_camera.rs:245,268`：将 `reset_camera`、`init_walk_camera` 接入对应入口/系统；
    无法接线时在 delivery 明确标“不可达 U”。
12. `startup/startup_time_of_day.rs:30-38`：按契约 §2.12 核对目标资源应为 `TimeOfDay`（I）；
    当前写 `TodSequence`，确认后统一。
13. `startup/startup_critters.rs:22-46`：补契约 O 的 `firefly` 网格条；`CRITTER_DRAWABLES`
    保留 I 标识。
14. `systems/main_camera.rs:133-191,218-241`：契约 O 为 9 参数/5 参数；在 delivery 或代码注释中
    明确缺失参数与替代来源，不得以 7/3 参数静默等价。
15. delivery §3 表格：把“真实注册”改为“文件存在且可注册”/“文件不存在（U）”，并订正 §6:127
    的生产可达性描述。

## 给①的下一步取证方向（≤5）

1. **clearing/tree·bush 的真实资产形态**：`init_clearing`（0x140a78aa0）是否只建
   ice_cracks+distant_billboard；`clearing/tree`、`clearing/bush` 是名式网格加载、prefab RON
   （`assets/prefabs/clearing/tree/`），还是 atlas/程序化名？给出调用点与容器。
2. **spawn_brush_previews（0x140924b40）逐项表**：11 组文件↔注册名、shader
   `timed_terrain_gizmo_shader` 的使用点、17 条 spawn 的 bundle 尺寸→mesh 映射。
3. **load_audio（0x140b0e0f0）与 load_glade_grading（0x1408bf9c0）**：前者两个被追加系统的
   符号身份与目标 Schedule；后者 `grading.json` 读取、`RoastMeat_Cinema_709_1`/`default`
   选择条件与 halloween 分支。
4. **main_camera_update 9 参数与 camera_snapshot 5 参数**：各 param 对应的真实资源/字段，
   尤其 `param_1+0x18/+0x10/+0x168` 与 snapshot 的 mode 源；以及 27 模式下发跳表的目标体。
5. **宿主装配调用点**：`add_game_startup_systems`(0x140941110) 的实际调用者与注册阶段
   （PreStartup 根？）、`add_backend_systems`/`add_ui`/collision/prefab 的直调点与 set 号，
   以确定四批顺序与相机 set 的正确插入位置。
