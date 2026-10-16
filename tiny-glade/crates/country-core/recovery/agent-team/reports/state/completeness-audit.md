# 库内完整性审计 — 组 state（reviewer-state，2026-10-16）

> 突破"限定"口径的逐函数功能完整性核验。方法：inventory 枚举全部原函数（区分业务/胶水/内联）
> → 与当前 src 一一对照 → 逐函数裁决（implemented_verified / implemented_pending_evidence /
> glue_legitimate / missing_in_library）→ 假实现扫描 → 残留分类（库内可闭合 vs 外部依赖）。
> 编译基线：全 lib 1709/1（唯一败 geometry::sphere，components-geometry 组，非 state）。
> 覆盖：save / session / music / history(res+sys) / startup / roofs / stairs / meta_commander / 域类型。
> 审计依据 = 16 轮逐函数核对（fresh 反编译 + RNE 机器码/串/跳表直读 + capstone）的累积结果。

---

## 0. 假实现扫描结论（先给红线）

- **未发现 stub/TODO/FIXME/unimplemented!/todo! 冒充实现**（grep 全模块零命中；唯一 "stub" 命中为
  `StairGraphEdge::is_stub`/`contains_edge_or_stub` 真实业务语义）。
- 全部"占位/延后"均为**文档化诚实路径**：`CommanderOutcome::Handled`（case 2/3/7/8/0xc，注释标明 sink
  载荷/资源面由宿主）、`is_known_raster_key`=false、`apply_loaded_snapshot` 显式 no-op、
  `panic_deprecated_operation`（原版 panic 行为）、`EditType::Terrain` tag=15 标注"不伪造"、
  `WallHistoryEdit::StateChange.observed_operation=[u8;12]`（"由真实 replay 消费者解释，不从枚举名猜测"）。
- **结论：库内无偷懒/假数据；所有未实现均为明确披露的边界（外部依赖）或证据不足的待取证。**

---

## 1. systems/save（145 原函数 / ~100 业务）

| 函数族 | 裁决 | 说明 |
|---|---|---|
| write_json_sections / save_game_to_memory / save_game_to_writer | **implemented_verified** | 字节级核对（fmt 串/分隔符/padding 位置）。 |
| change_section / replace_edits / add_history_edit_to_save_file / flush_deferred_and_sync | **implemented_verified**（replace_edits pending） | change_section 字节级核对；replace_edits 尾部假设 data.len()-4 已文档化，原版 case-6 逐 edit 日志待补。 |
| ShadowFile create/flush/sync_all/mark_dirty/write_at/truncate/ShadowFileWriter | **implemented_verified** | 与 0x1409193d0/0x14091a230/0x14091a450 一致；脏块合并为等价优化（注明）。 |
| save_thread 主循环 / execute_save_cmd / execute_open_file_cmd / UnsavedFileList / AutosaveErrorState | **implemented_verified** | No-active 门控/panic 串/Shutdown ack/1s tick/diff 全核；每迭代 drop 语义待取证（低）。 |
| backup_save / perform_manual_backup / capture_snapshot / start_snapshot_gc / consume_snapshot_gc_release / nuke_unreferenced_shapshots | **implemented_verified** | 串族与 GC 协议核对。 |
| SaveBackupMetronome add/consider/new_from_backup_list | **implemented_verified** | 阈值门（+0x148<60）补入；clamp 8.0/步进 -1.0/除数桶/remove 回退全核。 |
| SaveBackupList iter/new_from_session/kind 分类 | **implemented_verified**（计数器口径 pending） | 分类 0x14166df80 机器级闭合；计数器 max(parsed+1) 范围待取证。 |
| save_game_system（节拍器推进/备份触发/事件） | **implemented_verified** | log + CaptureSnapshotRequested 事件。 |
| parse_save_file_snapshots / snapshot_gc_thread | **implemented_verified** | 首键命中/扩展名过滤/stem 解析/取消语义。 |
| load_game advance/load_game_session/两遍回放/FenceRelease/version_gates/extract_max_wall_id | **implemented_pending_evidence** | 自洽且测试过；extend_on_load batch tag=3 与机器码对齐待 load_game 回放字节级复核（库内可闭合）。 |
| save_file_identity try_approx/derive | **implemented_pending_evidence** | 0x800B+"Identity" 搜索核；derive 字段序 U。 |
| load_completion_checker 纯函数 | **implemented_pending_evidence** | loading.tmp 探测语义 I；GameProfile 澄清不含。 |
| snapshot 节序/flate2/collect_trees/Arc 骨架 | **implemented_pending_evidence** | 基础设施核；域序列化器外部。 |
| to_json_string×7 / GameSaveData 组装 / save_game 20+SystemParam / init_game_state 写回 / load_completion ECS 接线 | **missing_in_library（外部依赖）** | 域资源/宿主；无伪造节载荷。 |
| AutoSaveState close_file/create_backup_of/last_error_state | **implemented_verified** | 0x140a43840/0x141e59860/0x140a439a0。 |
| **库内完整性 verdict** | **高** | 纯库逻辑（编解码/线程/GC/节拍器/读档核心）绝大多数 implemented_verified；剩余主要为外部依赖（域组装/宿主接线）+ 2 项库内可闭合（extend_on_load tag、SaveBackupList 计数器）。 |

## 2. resources/session（70 原函数 / ~64 业务，多为 serde）

| 函数族 | 裁决 | 说明 |
|---|---|---|
| SessionId new/save_folder/backup_folder/snapshots/session_file_list | **implemented_verified** | 路径协议/三件套核。 |
| GladeType ALL/assets_folder/is_*/audio_id/parse+serde | **implemented_verified** | 5 变体/首字母大小写不敏感/audio_id 表 [0,3,1,2,4] RNE 实测。theme_id（U 内联）→ missing（未消费，不构成缺口）。 |
| SessionMutator / SessionMeta（schema/write_to_path/try_load/is_halloween*/is_xmas） | **implemented_verified** | 字段名/谓词/元数据往返核；meta.json version 键待取证（低）。 |
| ActiveSession | **implemented_verified** | 原序列化器 2 键。 |
| NextSessionKind / OnLoadEntry / NextSession load/create/ignore_snapshot/append_fence/append_replay_terminator/extend_on_load | **implemented_verified**（extend tag pending） | 0x20B 条目/kind niche/fence 双 tag 核；extend_on_load 批量 tag 与 save 同项。 |
| SavedSession / SavedSessionList / load_saved_session_list / refresh_session_list / delete_session | **implemented_pending_evidence** | 字段集（screenshot_exists 等）与标记文件细节 I。 |
| SessionFlowPlugin::build | **implemented_pending_evidence** | ActiveSession 占位默认；bevy_app 由宿主装配。 |
| **库内完整性 verdict** | **高** | 会话切换链数据结构/schema 全核；完整 world 切换调度为外部（宿主）。 |

## 3. systems/audio/music（20 原函数 / ~18 业务）

| 函数族 | 裁决 | 说明 |
|---|---|---|
| MusicPlayer init/load_tracks/switch_track/update/stop_immediately/pause/stop_photomode_snapshots | **implemented_verified** | 4 快照名/洗牌弹出/4 态机 A/B 双跳表 capstone 逐 case 核。 |
| trigger_photomode_minus_fpv_snapshot / trigger_fpv_snapshot / current_track / probabilistic_switch_track / generate_track_playlist | **implemented_pending_evidence（折叠）** | 前二者并入 update 的 photomode/first_person 分支；后三者折叠进 switch_track/公开字段（RE-ANALYSIS 确认 PDB 名错位）。 |
| get_timeline_beat_properties / timeline_snapshot | **implemented_verified** | 存储顺序按机器码（bar 最后写）。 |
| music_audio_system / music_audio_init_system / update_music_audio / preview_glade_parameter | **implemented_verified** | richness snap-off/lerp/NewGladeTime/门 EscMenuState/三事件名全核。 |
| music_debug_gui | **implemented_verified**（按钮 I） | 窗/开关字节/richness 标签；按钮行为 egui 闭包内 I。 |
| native.rs（FMOD 适配） | **implemented_verified** | 真实 libfmod 2.222.6；Drop release/beat 回调。 |
| **库内完整性 verdict** | **高** | 播放器状态机/参数/适配全核；GladeAudio 曲库生产 + live FMOD = 外部（startup/host）。 |

## 4. resources/history + systems/history（406+92 原函数）

| 函数族 | 裁决 | 说明 |
|---|---|---|
| EditType 15 变体 + native_tag（E1 权威表） | **implemented_verified** | visit_str 0x140b3b630；tag 12 未解登记；Terrain 保留 15 不伪造。 |
| HistoryIdentity generate/mutate + serde | **implemented_verified** | 透明 u64/fastrand。 |
| ReplayBuffer / ReplayBucket / ReplayCommand（含 Stairs 变体）/ pop_empty_replay_bucket | **implemented_verified** | R6R7 后 14 测试过。 |
| 5 edit 文件（color/path/stairs/tree/decorator）+ CompoundEditBuilder + panic_deprecated_operation + WallHistoryOperationTag | **implemented_verified**（结构）/ **implemented_pending_evidence**（字段序 U） | 结构/地址核；~40 操作变体逐名/载荷待 E 并入（missing_in_library 登记，不伪造）。 |
| WallHistoryEdit 4 结构变体 + StateChange.observed_operation | **implemented_verified** | 结构核；op 载荷由真实 replay 消费。 |
| roof_edit（半实现，serde 对齐待 E） | **implemented_pending_evidence** | 操作枚举在 commands.rs + building_history_transactions。 |
| replay_raster/replay_roof/replay_wall/replay_auto_clutter/replay_stairs/finish_replay/replay_mode | **implemented_verified**（stairs 经 R6R7 修复） | 回放链/双空/进出检测。 |
| history_recording（roof 记录） | **implemented_pending_evidence** | recording.rs 覆盖 roof 记录；history_start/finish_recording（全局开关）→ missing_in_library（登记）。 |
| history_process / replay_edit / schedule_replay_commands / raster_edit_to_replay_cmd | **implemented_pending_evidence**（部分折叠） | process_history_cursor_commands 覆盖普通 Undo/Redo 分支；**第二事件流（byte[+9] 跳表 0x142ae5420）U**，replay 生成在 recovered_building/history_transactions（共享）。 |
| replay_garden/path/water/terrain/pop_bucket(完整楼梯 diff)/signifier_replay_active | **missing_in_library（登记）** | 域在途（terrain/garden/共享 history）。 |
| **库内完整性 verdict** | **中-高** | EditType 表/编辑结构/回放链核心核；40-op 变体 + serde/tag 对齐归 E（库内可闭合待挖），replay_garden/path/water/terrain + 第二事件流为外部/域依赖。 |

## 5. startup（36 文件 / 4 正式 inventory 函数）

| 项 | 裁决 | 说明 |
|---|---|---|
| register_core_startup / add_game_startup_systems / load_shader_into_library / run_game X7 | **implemented_verified** | 卡G/X7 核。 |
| 9 已实现文件（init_photomode_state/load_glade_audio/load_textures/startup_{garden_brush,ivy,session,terrain}+mod.rs） | **implemented_pending_evidence** | 存在性核；部分内部细节待核。 |
| 27 缺文件（域注册胶水 7 / 资产库绑定 17 / panic_reporter / init_color_library） | **missing_in_library（外部依赖分类，诚实）** | 依赖域类型/Asset 库/render 管线；R8 分类核。 |
| load_mesh_into_library | **missing_in_library（登记）** | 依赖资产库/render。 |
| **库内完整性 verdict** | **入口完整** | 库内注册入口与 core 链接线全；缺项全为外部依赖（render/asset/域）。 |

## 6. roofs（resources 107 + systems/roof）

| 函数族 | 裁决 | 说明 |
|---|---|---|
| 创建/删除/类型切换/回放链（editing/lifecycle/registration/replay + register_roof_events） | **implemented_verified** | 16 系统 chain + 事件；事件数 17 vs 声称 18（计数差）。 |
| Roof 0x58 投影 + 谓词（is_pitched/is_flat/is_flat_w_no_rim/is_gable/gable_sides） | **implemented_verified** | 偏移表核（HIGHLIGHTS #2）。 |
| roof_visual_shape 6 / roof_ridge 7 / roof_noise 8 | **implemented_verified** | R6R7 修复后 14 测试过；常量/哈希/角种子 RNE 核。 |
| roof_shape 剩余几何谓词（is_stepped/is_overhang/pattern_id/ridge_length_01/… ~13） | **missing_in_library（库内可闭合）** | 几何委托共享 system_roof（不重复实现依赖数学）；逐函数核对待续——**库内可闭合**（借共享 crate，无需宿主）。 |
| RoofTriggerVolume / RoofAnimation / RoofTileSimulation / commands / tiles/dormer/buffers（胶水） | **implemented_verified / glue_legitimate** | 三 Vec/动画细节待核（低）。 |
| **库内完整性 verdict** | **高** | 主链+视觉+噪声核；roof_shape 剩余谓词借 system_roof 库内可闭合；视觉 bundle/碰撞 = 外部。 |

## 7. stairs（resources 111 + systems/history/replay_stairs）

| 函数族 | 裁决 | 说明 |
|---|---|---|
| stairs_state 权威图/快照/差分/备份/TransientStairEdgeId | **implemented_verified** | 图操作/endpoint 度数守卫/checksum/diff 全核。 |
| structural_graph 7/7（R7 修复后） | **implemented_verified** | 表分布/句柄排序/索引 panic 忠实（@0x142a96bd0）。 |
| visual 4 核心 / stairs_collider（胶水）/ stairs_removed_supports / stairs_walls::add | **implemented_verified / glue_legitimate** | remove_edge/reset/record/erase/add 核；RtWorld/mesh 接线 = 外部（render/rt）。 |
| replay_stairs（R6R7 修复后） | **implemented_verified** | 门控 <= 先判后加/undo 分支/装饰过滤/音频事件/逆序移除。 |
| stairs_assembly_params（StairWallAnchor/Observed*/length_ws） | **implemented_pending_evidence** | 组装参数语义待取证。 |
| invalidate（invalidate_dirty_stair_artifacts） | **missing_in_library（库内可闭合，小）** | 失效标记；无 src 文件，登记。 |
| **库内完整性 verdict** | **高** | 权威图/结构/视觉存储/回放核；invalidate 库内可闭合（小）；RtWorld/mesh/完整 pop_bucket diff = 外部。 |

## 8. meta_commander（inventory no_formal_mapping）

| 项 | 裁决 | 说明 |
|---|---|---|
| MetaCommand/MetaCommander/MetaCommandFence/peek/consume/undo_consume/extend_script/update 全 case | **implemented_verified**（数据结构）/ **implemented_pending_evidence**（case 2/3/7/8/0xc sink 载荷） | 0x38B/空哨兵 0x8000000000000014/fence 双 tag 核；sink 载荷/资源面 = 外部（宿主/域）。 |
| commander_update_system / configure_session_switch_systems | **implemented_verified** | AppMode {13,14} 门控核。 |
| **库内完整性 verdict** | **高** | 数据结构+分派主体全核；sink 应用外部。 |

## 9. 域类型（game_profile / glade_color / camera_rig）

| 项 | 裁决 | 说明 |
|---|---|---|
| GladeColorCustomization 15 字段 + ColorId | **implemented_verified** | 字段序/index 表核；3 字节枚举 u8 登记（变体名待取证）。 |
| GameProfile ron 5 字段 + player_id skip | **implemented_pending_evidence（FIX 低）** | 字段序与原文 serialize 序不同（ron 按名兼容，字节序差）；建议重排。 |
| CameraRigParametersCache 6 字段 | **implemented_verified** | position (0,5,0)/6 字段核。 |
| camera_rig MainCamera/free/normal/walk 体（~70 函数） | **missing_in_library（外部/相机域）** | 相机模式控制器未恢复，登记；不在本组验收面。 |
| **库内完整性 verdict** | **中** | 3 存档类型核；GameProfile 字段序 FIX（低）；相机体外部。 |

---

## 残留总分类

### 库内可闭合（无需宿主/render/他 crate，待证据或挖取）
1. save：extend_on_load 批量条目 tag=3 与机器码对齐（待 load_game 回放字节级复核）；SaveBackupList 计数器口径（0x14166df80 前缀偏移源）。
2. history：~40 操作变体逐名/载荷（E 挖）+ serde/MultiEdit deserialize 跳表 0x142aee31c 对齐；terrain 历史机制（E）；history_process 第二事件流（byte[+9] 0x142ae5420）。
3. roofs：roof_shape 剩余 ~13 几何谓词（借 system_roof 逐函数核对）。
4. stairs：invalidate（小）。
5. game_profile：ron 字段序重排（FIX 低）。

### 外部依赖（宿主/render/资产库/他域，登记不阻塞）
- GameSaveData 域资源组装（20+ SystemParam）+ snapshot 域序列化器 + init_game_state 写回 + load_completion ECS 接线。
- commander_update case 1/2/3/7/8/0xc sink 载荷/资源面应用。
- GladeAudio 曲库生产 + live FMOD + 全局注册。
- 27 startup 缺文件（资产库绑定/域注册胶水/panic_reporter/init_color_library）+ load_mesh_into_library。
- replay_garden/path/water/terrain/pop_bucket（完整楼梯 diff）/signifier_replay_active + 视觉 RtWorld/mesh。
- camera_rig MainCamera/free/normal/walk 控制器（相机域）。

---

## 结论（每模块库内完整性 verdict）

| 模块 | 库内完整性 | 一句话 |
|---|---|---|
| save | **高** | 编解码/线程/GC/节拍器/读档核心 implemented_verified；域组装/宿主接线外部；2 项库内待闭合。 |
| session | **高** | schema/serde/会话切换链数据结构全核；world 切换调度外部。 |
| music | **高** | 播放器状态机/参数/适配全核；曲库生产/live FMOD 外部。 |
| history | **中-高** | EditType 表/编辑结构/回放链核心核；40-op 变体/terrain 机制库内待挖；replay 缺项外部。 |
| startup | **入口完整** | 注册入口/core 链接线全；27 缺全外部。 |
| roofs | **高** | 主链+视觉+噪声核；roof_shape 剩余谓词借 system_roof 库内可闭合。 |
| stairs | **高** | 权威图/结构/视觉/回放核；invalidate 库内可闭合（小）。 |
| meta_commander | **高** | 数据结构+分派主体核；sink 应用外部。 |
| 域类型 | **中** | 3 存档类型核；GameProfile 字段序 FIX（低）；相机体外部。 |

**库内无假实现**；所有未闭合项均为明确的"待取证（库内可闭合）"或"外部依赖（宿主/render/他域）"。
完整 core 验收仍被外部依赖（域资源收集/组装 + 全局调度 + render/资产接线）阻塞，非库内缺失。
