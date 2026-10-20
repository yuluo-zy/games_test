# R10 G1 启动链与主相机交付（g1-startup-mainloop）

日期：2026-10-19  
工作目录：`D:/game/tiny-glade`  
契约：`evidence/round10/contracts/g1-startup-mainloop-CONTRACT.md`（13:05 出现，已按 §1.3/§1.5/§2/§3 对齐）

## 1. 最终验证

| 命令 | 结果 |
|---|---|
| `cargo check -p country-core` | **通过**（exit 0） |
| `cargo check -p tiny-glade` | **通过**（exit 0） |
| `cargo test -p country-core --lib` | **2510 passed / 0 failed** |

## 2. G1③ 复核 6 项阻断的修复

| # | 问题 | 修复位置 | 结果 |
|---|---|---|---|
| 1 | `add_startup_mesh_systems` 重复调用 | 删除 `startup/mod.rs` 内的旧调用与 `run_game.rs:151` 的旧入口；现在唯一调用点是 `startup/run_game.rs:157 register_session_systems` → `:173 super::add_startup_mesh_systems` | 单次装配 |
| 2 | brush_previews 名称错位/伪造表 | `startup/spawn_brush_previews.rs:16-40` 按契约 §2.4 重建 11 条 O 表（`brush_4_arrows`→`brush_terrain_edit`、`brush_2_arrows`→`brush_flat_roof_edit`、`brush_5_replay`→`brush_replay`、`hand`→`temp_sphere`、`dashed_lines`→`snapping_line`）；shader 改 `/timed_terrain_gizmo.raster.hlsl`→`timed_terrain_gizmo_shader`；删除 `snapping_circle` 与重复项；文件不存在时跳过 | 11 个文件全部存在于发行 `assets/meshes` |
| 3a | clearing 注册不存在文件 | `startup/clearing.rs:14-30`：只保留 `distant_billboard`（无条件）与 `ice_cracks`（theme 4/Winter）；cap=`min(x,0x44c)`=1100；uninit 27×0xc0、`+0x38=1`、`snapshot:/Loading`；移除 `clearing/{tree,bush}.json` | 不再引用不存在文件 |
| 3b | preplaced trees 路径/schema 错 | `startup/spawn_preplaced_trees.rs:43-128`：读 `<assets_root>/assets/glade/<theme>/trees.json`（发行资产实读 schema `{prefab,placements}`）；placements 文件 `{position,id,scale,rotation_angle,pad0,pad1}`；`TreeStorage::insert` + `Commands::spawn(Transform, InteractableForestTree)`；日志 `Converting pre-placed trees` / `deserialize trees` | 真实读盘 + 写 TreeStorage；宿主 `run_game.rs:293` 初始化 `TreeStorage`，`:529` OnEnter 注册 |
| 4 | walls 空几何注册 | `startup/startup_walls.rs:21-53`：网格改为契约 §2.13 的 `assets/meshes/brick.json`→`brick`；`create_half_cube_mesh` 只保留名字常量，**不再写入** `AssetMeshLibrary`；`TerrainHeightsData::subscribe` 以 U 跳过（订阅者所有者未恢复） | 生产库无空网格 |
| 5 | human_ref 假资产 | `startup/spawn_human_ref.rs:14-39`：删除 `human_ref_cube.json` 注册；改为 `StartupAssetHost::lookup_mesh("human_ref_cube")`（mesh atlas/prefab 通道）；查不到时只保留常量 `[-0.25,0.25,-0.9,0.9]`、`[0.5,1.8,0,0]`、`+0x30=0.5` 与 U 告警 | 不再注册伪造文件；系统从启动链移除 |
| 6 | add_game_startup_systems 自环/未接宿主 | 宿主 `crates/tiny-glade/src/run_game.rs:496-532` 按契约 §1.5 接 `OnEnter(Game)`：batch2 世界资产链（walls→sky→critters→roof→sheep→clearing→meadow）+ batch4（brush）+ save 批的 `spawn_preplaced_trees`；`startup_time_of_day` 在 `load_glade_tod_and_weather` 之前；PreStartup 批的 `load_audio` 由 `startup/run_game.rs:92` 在 `register_core_startup` 内追加 `game_loading_audio` 两系统（`:25`，带 `AudioCore<S>` 资源条件） | 真实宿主接线，无自环 |

## 3. 高优先项

- **startup_meadow 真实产出**：`startup/startup_meadow.rs:70` 读取 `<assets_root>/assets/glade/<theme>/plants.json`（发行 schema `{shader,mesh,placements}`），对每个条目注册真实存在的 `assets/meshes/<mesh>.json`；`init_meadow`（`:54`）校验 shader 名。1×0x88 spawn 仍 U（placements 实例 schema 未恢复）。
- **StartupAssetHost 生产实现**：`startup/mod.rs:119 WorldAssetHost`，直接使用宿主 `World` 的 `AssetMeshLibrary`/`AssetShaderLibrary`/`ShaderBackend` 并 `World::spawn(DrawableMeshBundle)`；名式加载通过 `lookup_mesh`/`lookup_shader`。
- **load_audio 调用者**：`startup/run_game.rs:92` 调用 `load_audio::<S>(None, startup_schedule)`；`load_audio` 追加 `init_game_loading_audio` + `loading_audio`（`:25`）。
- **load_glade_grading 调用者**：没有生产调用者。原因：契约要求的 `ResMut<GradingLut>` 在当前树中不是 `Resource`（`resources/grading_lut.rs` 无 Resource derive），且 LUT 解码需要 `TextureDevice`，本组文件权不含该文件。已提供 `GradingHost`（`:46`）供设备 owner 一行接入；`grading.json`/`RoastMeat_Cinema_709_1`/`default`/`assets/luts/<name>.png` 常量与路径已按契约落地。
- **MainCameraUpdateInput 真实生产者**：`systems/main_camera.rs:135 feed_main_camera_update_input` 从 `NewSessionStartedCmd` 置新会话门；已注册（core `startup/run_game.rs:219` 在相机更新前，宿主 `run_game.rs:374`）。鼠标深度字节与自由相机开关来源资源未恢复（U），字段保持默认。

## 4. 启动模块逐项（最终）

| 模块 | 真实接线 |
|---|---|
| `startup_sky_and_sun.rs` | `:20` mesh→shader→spawn；`:28` skydome 真实注册。U：bundle 其余字段、太阳角度 |
| `startup_time_of_day.rs` | `:31` 写 `TimeOfDay`（`assets/tod/default.json`）；缺失时记录错误不覆盖。契约字段名仍 U |
| `startup_critters.rs` | `:56` 4 文件网格 + 5 shader + 4 spawn；`:81` 4 网格注册。`firefly` 只登记名（无文件，U） |
| `startup_sheep.rs` | `:45` 5 网格 + 2 shader；`:55` 5 网格注册；Baah/Pitch 常量 |
| `startup_roof.rs` | `:42` 4 网格 + 2 shader + 2 spawn；`:58` 4 网格注册 |
| `startup_walls.rs` | `:36` 2 LOD shader + brick；`:49` brick 注册；half_cube 仅名字 U |
| `startup_meadow.rs` | `:54/:70` 见 §3 |
| `spawn_preplaced_trees.rs` | `:70` 见 §2-3b |
| `spawn_brush_previews.rs` | `:40` 11 条真实网格注册；17 条 spawn 映射 U |
| `spawn_human_ref.rs` | `:24` 名式查找 + vertex_color shader；缺失只告警 |
| `clearing.rs` | `:38/:51` 见 §2-3a |
| `load_audio.rs` | `:25/:41` 见 §3 |
| `load_glade_grading.rs` | `:46/:58` 见 §3 |

装配：`startup/mod.rs:100 register_mesh_file`（缺文件跳过）、`:119 WorldAssetHost`；
`startup/run_game.rs:157` OnEnter 批资产链（受 `AssetMeshLibrary` 资源条件保护）、`:92` PreStartup `load_audio`。

## 5. 主相机链

`crates/country-core/src/systems/main_camera.rs`：

- `MainCameraUpdateInput` `:68`、`CameraSnapshot` `:81`；
- `feed_main_camera_update_input` `:135`（新会话事件 → 门）；
- `main_camera_update` `:146`：aspect → 新会话光标门（w>2 且 cursor==(0,0) return）→ 窗口<3 推 `WriteSettingsFileCmd` → 写 `+0x344` → DebugGui 模式 6→0 → 自由相机/动作模式分发；
- `paused_main_camera_update` `:207`、`camera_snapshot` `:231`（dt=`GameClock.delta × camera_snapshot_interpolation_speed`，mode≥27 拒绝）、`reset_camera` `:258`、`init_walk_camera` `:281`、`teleport` `:317`、`update_camera_for_mode` `:124`。

`camera_rig/walk.rs`：`WalkCamera::new/teleport/teleport_rig/rotation/update`；`camera_rig/butterfly.rs`：六 driver、索引变化 dt=1000 吸附；`camera_rig/main_camera.rs` 新增鼠标深度、rig 变换（rotation@+0x18/16B、position@+0x28/12B）与四矩阵写回。

U：逐模式 body 更新（0x140949fe0 等）、27 个快照应用体、`init_walk_camera` 的地形采样/球投、参数缓存清理资源类型。

## 6. 仍缺（显式）

1. `startup_meadow` 1×0x88 spawn 的 placements 实例 bundle。
2. `load_glade_grading` 的生产调用者（`GradingLut` 非 Resource + 无 TextureDevice owner）。
3. 鼠标深度字节/自由相机开关的真实输入来源资源。
4. `startup_critters` 的 `firefly` 名式网格通道、`spawn_brush_previews` 17 条 bundle 映射。
5. `create_half_cube_mesh` 几何、`TerrainHeightsData::subscribe` 订阅者所有者。
6. 主机 `run_game_thread` 未持有 FMOD Studio，因此 `load_audio` 的实际音频 owner 仍由宿主音频域提供。

## 7. 仅本组文件权内的修改

新增 13 个 `startup/*.rs` 与 `camera_rig/{walk,butterfly}.rs`；修改 `startup/{mod,run_game}.rs`、
`systems/main_camera.rs`、`camera_rig/{main_camera,mod}.rs`；`tiny-glade/src/run_game.rs` 仅相机资源/系统与
OnEnter 资产批、`TreeStorage` 初始化等 startup/相机局部。未改 `rhapsody`、roof、wall、save、render_loop、history。
