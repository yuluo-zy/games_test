# RESTORE — systems/roof 未判等函数体还原（B 版真实逻辑）

> restorer-state-roof，2026-10-17。fresh 反编译（Ghidra shared worker，get_functions fields=signature,decompiled_code）逐函数核对。
> 证据：`crates/country-core/recovery/agent-team/work/roof-restore/`（results/{batchA,batchB}.json + decomp/*.c）。
> 复核基准：`upper-modules-plan/systems/roof/ANALYSIS.md` §8（70 条符号清单）；本报告按该清单逐条标覆盖。

## 编译/测试

- `cargo check -p system-roof`（workspace 根）：**0 错误**，7 警告（4 属 country-core 他组在途；delete_roof/update_roof_spatial 2 条预存非本批文件）。
- `cargo check -p system-roof --tests`：通过。
- `cargo test -p system-roof`：2/2 通过（roof_replay_history）。
- 途经阻塞：terrain/decorator_storage 他组 WIP 错误已由该组自行闭合（复核时无残留）。

## 逐函数状态（§8 清单）

| 符号（ANALYSIS §8） | 状态 | fresh decomp | 说明 |
|---|---|---|---|
| `roof_trigger_volume::calculate_roof_trigger_volumes` | **未实现** | 0x1421a3220（1580 行/12048B） | 真逻辑=曲面三角化（细分 `(is_flat^1)*5+4`、矩形剖面采样+檐口 `−0.56/+0.4`、圆形 15 段 `Circle2d::new_rotated(0.05)`、UV 2.0/2.2、逐顶点 `apply_roof_noise`、`set_triangles/uvs/normals`）。本轮未逐语句重建；src 已移除伪造几何，仅登记查找骨架。分歧：细分计数/剖面采样/檐口/UV/噪声尺度。 |
| `simulate_roof_tile_for_audio::simulate_roof_tile_for_audio` | **重建** | 0x1421a7090 | 逐记录风驱动模拟：`angle'=dt·−45.126003+angle`、`dt==0` 早退、`is_flat` 跳过、`target=floor`、`−120`/`0.166`/`15.0` 混合钳制、`exp(dt·−13.300004)` 衰减、`0.1`/`0.01` 阈值发 RoofSound kind 5/4、累加器/标志位。**I 级差异**：core `RoofTileSimulation` 为单 24B 记录（原为 stride-0x18 记录数组）；RoofSound 扩展载荷属 core 重建边界。 |
| `wall_height_dt::wall_height_dt` | **判等** | 0x1421a2b60 | `roof[0x4c]−tile[4]→tile[0]`、`tile[4]=roof[0x4c]`，与 core `update_wall_height` 逐语句一致；`&mut` 查询自动标 Changed 等价原 tick 写。 |
| `visual::generate_roof_stone_floor_and_roof_bottom::…` | **重建**（渲染子项未实现） | 0x1421a6d40 | 控制流重建：事件→wall 哈希→roof→`(roof[8]&1)==0` 时 `log2f(radius·3)`→`Commands::entity(roof).insert(NaniInstanceDesc)` 入队（0x38B 命令项）。**未实现子项**：insert 命令执行体（NaniBundle/StoneFloorInstanceData 渲染边界缺）。 |
| `visual::switch_roof_type::switch_roof_type` | **部分重建**+未实现(mesh ops) | 0x1421ac4f0 | RoofSound kind 0/6、ridge==1→WallGableChanged、`source_flag==1` 墙高 `!=1.6`→`MAXSS(1.6)/MINSS(14)` 更新、RecalculateWallBricks 均按机器码。**未实现**：NaniBundle insert / DrawableMeshBundle remove / try_get_handle_by_name("roof")+deferred_delete_mesh / Handle<Vao> remove（渲染边界缺）；事件 target 类型字节未在 core SwitchRoofTypeCmd 承载（以 crossing==Up 表达）。 |
| `visual::roof_animation::roof_animation` | **判等** | 0x1421a2da0 | 删除分支 `−4.0/−10.0`、正常分支因子 `flat?2.0:1.0`、`2.857143`、`0.99→3.3333333`、clamp[0,1]，与 core `RoofAnimation::advance` 逐项一致。 |
| `ui_edit_roof::ui_edit_roof` | **未实现** | 0x141a18dd0 | 真逻辑 3254B：触发体三角相交拾取/slider3d/WidgetParams/HighlightRoofCmd/RecordRidgeSwitchHistoryCmd。依赖 UI 工具框架（批次 3）→ src 标未实现，不伪造交互。 |
| `ui_edit_roof::do_ui_edit_roof` | **未实现** | 0x141a19a90 | 真逻辑 4228B：`switch(clamp(payload−2,0,3))` 三 case（0x40 历史/拖拽量 SQRT>0.2/0x98 历史+ev_height_changed）。同上。 |
| `ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir` / `do_ui_edit_roof_ridge_dir` | **未实现** | 0x141a386b0 / 0x141a39490 | 真逻辑含 iter_removed_ids/RoofRidgeDims::to_world_space/box_trigger_volume/SignifierFactory；依赖缺失。 |
| `ui_edit_roof_ridge_dir::switch_roof_ridge_direction` | **重建** | 0x141a3a730 | 写 roof[0x3c]+Changed、RoofGeometryChanged kind1、WallGableChanged、RoofSound kind7（**pivot.y+height 修正**：原实现漏 +height）。 |
| `ui_edit_roof_ridge_dir::record_switch_roof_ridge_history` | **重建** | 0x141a3ac20 | 直写 History tag3：`payload[1]=event+0x10`、`[2]=event+0x11`、`[8..16]=wall`；`RoofHistoryEdit::direction` 映射确认。发射方 do_ui_edit 未重建（字段赋值待 UI 批次）。 |
| `ui_edit_roof_ridge_length::ui_edit_roof_ridge_length` / `do_ui_edit_roof_ridge_length` / `create_ui_widget` | **未实现** | 0x1421a8210 / 0x1421a8c50 / 0x1421aa170 | 真逻辑含 TweakerView/box_trigger_volume/project_cursor_onto_ray 反解/rotation_from_normal/0.75 偏移。依赖缺失。 |
| `ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused` | **未实现** | 0x1421ad440 | 真逻辑 5393B：CameraMatrices::eye_right/xz_rotation_bad/box_trigger_volume/signifier/反解链。依赖缺失。 |
| `deferred_record_roof_history::roof_record_history` | **判等** | 0x141a37fd0 | 逐事件构造 RoofEdit tag2 历史（Arc 头 + 字段 @+0x14..）→ History::add（失败不 panic）。`EditType::from(RoofHistoryEdit)` + `History::add` 与机器码写一致。 |
| `replay_roof::replay_roof` / `replay_roof::impl$0::process` | **判等（经 core 委托）** | 0x141a37ad0 / 0x1421b48e0 | 原桶扫描（stride 0x30，首字节 5=RoofEdit）→ 内联 process → 完成位收集 → 逆序 Vec::remove + tag drop。core `RoofReplayCommand::advance` + `advance_roof_bucket` 保留策略（bytes[21]!=1）与 process 状态机逐条一致。**I 级差异**：原始完成位 @0x1d 映射到 core bytes[21]。 |
| `highlight_roof::highlight_roof` | **未实现** | 0x141a284b0 / 0x141a31280 | 真逻辑：HighlightRoofCmd → HighlightColor::to_rgb → 写 RoofHighlight + changed tick + EditingHighlight 游标。`HighlightRoofCmd`/`HighlightColor` 在 core 未重建 → 无法按原签名接线；src 保留系统骨架。 |
| `lib.rs::gpu_flush_buffers` | **未实现** | 0x140de50a0 | 真逻辑：AssetSsboLibrary::get_mut<RoofTileSsbo> → GraphicsDeviceResource::deref → SparseInstanceBuffer::gpu_flush<RoofTileSsbo> + tick。`GraphicsDeviceResource` 在 core 缺（渲染边界）→ crate 未落位。 |
| `utils::components::impl$0/impl$1::register_required_components` | **glue-legitimate** | 0x14258d1f0 / 0x14258d8e0 | Bevy 0.16 derive(Component) 注册胶水（impl0 名长 0x29=RoofSpire、impl1 名长 0x30=RoofSupportBeams，decomp O）。src 增 RoofSupportBeams marker；derive 自动生成等价物。 |
| `ui_edit_roof_tip::…closure$0` / `visual::place_support_beams::…closure$1` / `visual::roof_snow::…closure$1/2/6` | **非本批** | — | B 未发射（ANALYSIS §8.1），骨架仅声明。 |

## 非本批（未改）

`create_roof`（M02）、`edit_roof`（M06 后段）、`delete_roof` 三函数、`update_roof_spatial`、`visual::assemble_roof_tiles`、`visual::place_spires`、`visual::assemble_{rectangular,circular}_roof`、`lib.rs::add_roof_systems`/`add_roof_events_and_resources`、`utils::assemble_*_roof` 已有实现 —— 不在本批未判等清单。

## 证据索引

- 批次结果：`work/roof-restore/results/batchA.json`（12 函数）、`batchB.json`（12 函数）；身份核对 `get_metadata`：tiny-glade.exe，patched B。
- 定点反编译（decomp/*.c，文件名=VA_符号）：0x1421a3220、0x1421a7090、0x1421a2b60、0x1421a6d40、0x1421ac4f0、0x1421a2da0、0x141a37fd0、0x141a37ad0、0x1421b48e0、0x140de50a0、0x141a284b0、0x141a31280、0x141a18dd0、0x141a19a90、0x141a386b0、0x141a39490、0x141a3a730、0x141a3ac20、0x1421a8210、0x1421a8c50、0x1421aa170、0x1421ad440、0x14258d1f0、0x14258d8e0。
- 复核基准：`upper-modules-plan/systems/roof/ANALYSIS.md`（§3/§4/§5/§7/§8）、`reconstruction/roof/`（surface/ridge/replay/edit_rules）、country-core `recovered_building/roof_*.rs`、`resources/roofs/geometry/`。

## 复核响应（reviewer-state-roof FIX×9，2026-10-17 已闭合）

| FIX | 处置 |
|---|---|
| 1 近似体·高 `do_ui_edit_roof_ridge_dir ^2` | **清退为纯签名**（不再写 SwitchRoofRidgeDirCmd/RecordRidgeSwitchHistoryCmd）；方向位翻转 `^1`（=`^0x100`）已登记真逻辑，待 UI 框架接线重建。 |
| 2 近似体·高 `do_ui_edit_roof_ridge_length` 发明投影 | **清退为纯签名**（不再写 EditRoofCmd）；`ridge_length_from_cursor` 删除。真投影链 `project_cursor_onto_ray→inverse_ridge_01_param` 登记。 |
| 3 近似体·高 `ui_focused` `/200.0` 拖拽差 | **清退为纯签名**（不再写 EditRoofCmd）；真逻辑 `project_cursor_onto_ray→inverse_lerp` 登记。 |
| 4 中 add_roof_systems 补注册 | **已补**：`roof_record_history`、`record_switch_roof_ridge_history`、`switch_roof_ridge_direction` 加入 `WallCpuStage::RoofDependencies`+`HistoryCpuStage::Apply` 链（原 0x14062fcb0 注册之）。 |
| 5 中 core 阻塞类型 | **登记**：core 需补 `HighlightRoofCmd`+`HighlightColor`（resources::roofs::commands / walls）与 `GraphicsDeviceResource`（render）；补后 `highlight_roof`/`gpu_flush_buffers` 照 decomp 落位（gpu_flush 逻辑全可观测，~5 行）。本批未跨库新增类型。 |
| 6 边界·低 GameClock.delta 0x24 vs +0x28 | **登记**：系统面经 `time.delta` 符号读取不受影响；core `GameClock` 字段序待原核对。 |
| 7 边界·低 RoofSound 载荷 | **登记**：kind4 载荷 `(accum−0.01)/0.19` 计算后暂弃（`_ = decay_01`），待 core RoofSound 扩展字段落位。 |
| 8 边界·低 RoofTileSimulation 单记录 vs 数组 | **登记**：现逐 roof 驱动单 24B 记录；core 重建 stride-0x18 数组后迭代需同步。 |
| 9 表述 RESTORE "不伪造" | **已收紧**：三处未实现 stub 已清退为纯签名（不再含发明公式/写真实事件），本表即为清退证据。 |

## 交付差异摘要

- 重建：simulate_roof_tile_for_audio、generate_roof_stone_floor_and_roof_bottom（控制流）、switch_roof_ridge_direction（pivot.y+height 修正）、record_switch_roof_ridge_history（载荷映射）。
- 判等（fresh decomp 复核）：wall_height_dt、roof_animation、roof_record_history、replay_roof/process。
- 未实现（列出分歧点，src 明确标注，不伪造）：calculate_roof_trigger_volumes、ui_edit_roof/do_ui_edit_roof、ui_edit_roof_ridge_dir/do_ui_edit_roof_ridge_dir、ui_edit_roof_ridge_length/do_ui_edit_roof_ridge_length/create_ui_widget、ui_edit_roof_ridge_and_height_focused、highlight_roof、gpu_flush_buffers、switch_roof_type（mesh ops 子项）。
- glue-legitimate：register_required_components（impl0/impl1）。
