# WP9 屋顶系统合同 — create/edit/delete/spatial/audio/渲染接线

Target: B。骨架：`skeleton/B/crates/systems/roof/src/*.rs`（system_roof crate）+
`country-core/src/systems/{render_loop/compute_passes/roof_tiles_gravity, draw_passes/
draw_shaders/instanced_roof, audio/roof_audio, startup/startup_roof}.rs`。标注 O/I/U。

## 1. create_roof（O，0x1421aaea0）

`system_roof::create_roof::create_roof`（line 17）。
- 输入：Commands + 事件读取（wall 关联变化）+ `PublicWalls` + wall state。
- 流程：遍历 2D 墙单元事件（stride 0x10，grid 0x40 槽）；`PublicWalls::roof_entity()` 解析
  墙→屋顶关联；每有效关联：
  1. `PublicWallState::max_y()` + `WallStyle::is_halftimbered()` 采集墙状态；
  2. `Roof::new(...)`（88B，8 参数）→ `Roof::ty()`/`Roof::is_gable()` 确定类型；
  3. `RoofTileSimulation::new()`、`RoofTriggerVolume::default()`、`RoofAnimation::new()`；
  4. `Commands::spawn((Roof, RoofTileSimulation, RoofTriggerVolume, RoofAnimation,
     RoofTiles, RoofSpire, RoofSupportBeams, NaniBundle<StoneFloorInstanceData>))`；
  5. 半木标记 → spawn 后按 `is_halftimbered` 分支补字段（uVar18=6/0）。
- 失败：`PublicWalls::get()` 空 → `unwrap_failed`；屋顶数>1 → panic。
- O。**深 ECS/world 遍历 = typed gap**（关联表结构 U）。

## 2. edit_roof（O，0x1421ab7c0，line 104 do_ui_edit_roof 之外的主系统）

- 输入：屋顶编辑命令流（**64B RoofEditEvent**，`reconstruction/roof/edit_rules.rs::RoofEditEvent`
  逐字段验证）+ 墙关联。
- 核心：对每屋顶应用 `apply_edit`（profile/height/ridge/eave/tip 的 Absolute=1/Delta=2 语义、
  clamp、`change_tick` 写、gable 进出/标量/尖移动反馈事件 tag 2/3/10/11）。O。
- 通知：`resolve` 请求 → 按 `Roof::is_gable`/`pivot_3d` 发历史/装饰事件。
- 深 ECS 事件写入 = typed gap；数值核心已机器码验证。

## 3. delete_roof 族（O）

- `process_invalid_roof_notifications`（0x14062eb80）：遍历墙关联，读 Roof（stride 0x58），
  无效屋顶 → 发 `SettingsMenuState` 事件。
- `mark_roof_for_deletion`（0x14062f3d0）：遍历墙单元（stride 0x18）收集墙 id → 对每屋顶
  标 `RoofMarkedForDeletion`。
- `delete_roofs`（0x14062f8c0）：遍历关联，检查关联有效性（两代指针），无效 → despawn。
- 全部 O；关联表内部结构 U。

## 4. update_roof_spatial（O，0x1421a7710）

- 每墙→屋顶关联：`Roof[0x4c]=PublicWallState::max_y()`；`Roof[0x54]` 与 `is_halftimbered()`
  不一致时写回并发事件（DecoratorId, flag=2）；墙 style<2 时把墙空间 bounds
  `(f×4@+0x25c, u64@+0x26c)` 复制到 `Roof[0x8..0x1c]`（含 center/dims，逐字节比较避免重复
  写）；`Roof::ty()` 圆 → `Roof::pivot_3d` 写 `[+0x10..0x18]` 并把高度 -0.69（底校正）；
  矩形 → 清零。
- 每写都置 change_tick（`lVar10+idx*4 = tick`）。O。

## 5. wall_height_dt（O，0x1421a2b60）

- 每屋顶 tile 索引：`delta = Roof[0x4c] - wall_base_y` 写 tile 记录 `[+0]=delta, [+4]=Roof[0x4c]`
  （stride 0x18），另写 u32 tick（stride 4）。→ gravity push `wall_height_delta` 的数据源。O。

## 6. simulate_roof_tile_for_audio（O，0x1421a7090）

- 时间资源（dt@+0x28）为 0 时早退；跳过 flat 屋顶（`Roof::is_flat` 循环）。
- 每 tile 记录（stride 0x18：`[+0]=height, [+2]=? , [+3]=velocity, [+4]=sound_timer,
  [+5]=active byte`）：
  - `v' = v + dt·(-45.126003)`；`h' = h + dt·v'`（含 clamp 逻辑）；
  - `h_eased = lerp(h', pivot_y, 1-exp(dt·-13.3))`；
  - 落地（`|h_eased-pivot|<0.1`）：timer 复位 0；active=0。
  - 从 0.1 开始下落：发音频/装饰事件（DecoratorId bucket，event 变体 4=音量 lerp、5=命中）；
  - 下落中 `v<0`：`timer += dt`，active=1。
- O。tile 记录=RoofTileSimulation 字段候选（见 roof-tiles.md typed gap）。

## 7. ui_edit_roof 族（O，深 ECS U）

- `ui_edit_roof`（0x141a18dd0，line 49）：UI 编辑捕获 → `RoofParamsCacheResource`/
  `DoUiEditRoof`/`RoofShapeEditAmount`/`OriginalWallHeight`（命名实体）→ `Roof::ty()`。
- `do_ui_edit_roof`（0x141a19a90，line 104）：按 RoofType 分派，调 `update_wall_height_if_needed`。
- `ui_edit_roof_ridge_dir`（0x141a386b0）/`do_ui_edit_roof_ridge_dir`（0x141a39490，line 96）：
  gizmo 信息 → `switch_roof_ridge_direction`（0x141a3a730，line 261）+ `record_switch_roof_ridge_history`
  （0x141a3ac20，line 299）。
- `ui_edit_roof_ridge_length`（0x1421a8210）/`do_ui_edit_roof_ridge_length`（0x1421a8c50）/
  `create_ui_widget`（0x1421aa170，line 268）。
- 全部 ECS/UI 深层 U。

## 8. replay 与历史（O 命名，U 内部）

- `replay_roof`（0x141a37ad0，line 23）+ `impl$0::process`（0x1421b48e0）：回放 RoofReplayCmd。
- `roof_record_history`（0x141a37fd0，line 19，`RecordRoofHistoryCmd`）：记录历史。
- `deferred_record_roof_history.rs`：`RecordRoofHistoryCmd` 命名实体。
- `highlight_roof`（0x141a37xxx，inline）：高亮。

## 9. calculate_roof_trigger_volumes（O 命名，U 内部，0x1421a3220）

- 68KB 反编译；计算屋顶触发体（RoofTriggerVolume set_triangles/normals/uvs 的生产者），
  消费 `RoofTriggerVolume::get_position_from_mesh_uv`。O 结构/U 深。

## 10. visual 系统（O 命名，生产者链）

| 系统 | VA | 作用 |
|---|---|---|
| `visual::assemble_roof_tiles::assemble_roof_tiles` | 0x141a36700 | RoofTiles 记录生产（prior work） |
| `visual::generate_roof_stone_floor_and_roof_bottom` | 0x1421a6d40 | 石地板+屋顶底网格 |
| `visual::place_roof_spires` | 0x142196e70 | RoofSpire 实例（flag） |
| `visual::place_roof_support_beams` | 0x1421aXXXX | RoofSupportBeams 实例 |
| `visual::roof_animation` | 0x1421a2da0 | RoofAnimation 逐帧 |
| `visual::switch_roof_type` | 0x1421ac4f0 | Gable↔Flat 切换 + `stone_floor_bundle` |
| `visual::gen_roof_snow_mesh` | — | 雪网格（roof_snow） |

## 11. roof_audio（O，0x140aa2fd0/0x140aa3860）

- `roof_audio_init_system`（line 28）：3×`AudioCore::instantiate_loop` → `insert_resource<RoofAudioState>`。
- `instantiate_audio_w_position`（0x140aa3360）/`_and_volume`（0x140aa3590）：音频实例。
- `roof_audio_system`（0x140aa3860）：按 `RoofAudioState` 驱动循环音量/位置；消费
  `audio_utils`、历史事件（decorator_edit/path_edit/terrain_edit 引用）。O 结构/U 深。

## 12. startup_roof（O，0x1414547d0，line 14）

- 4×：`load_json_as_mesh` + `load_mesh_into_library` + `load_shader_into_library` →
  `spawn_empty` + 插入 `DrawableMeshBundle`（Transform {t=0, r=identity, s=1}）。
- → 注册屋顶 4 个网格资产（屋瓦/spire/支撑梁/旗布，U 名称）。O。

## 复用裁定（prior work）

- `reconstruction/roof/edit_rules.rs` → edit_roof 数值核心，**复用**。
- `reconstruction/roof/tiles.rs` → assemble_roof_tiles 数值核心，**复用**。
- wp5 W19 合同 → roof_tiles_gravity 结构，**复用**。
- wp6 `resources/roofs/{roof_animation,roof_shape}.rs` → 本包屋顶资源接续，**复用并扩展**。

## typed gaps（U）

全部 system 的 World/组件图深层遍历、关联表结构、RoofSpire/RoofSupportBeams 组件字段、
RoofParamsCacheResource/DoUiEditRoof 等 UI 类型布局、音频资源 API、RoofEdit 历史变体字段。
