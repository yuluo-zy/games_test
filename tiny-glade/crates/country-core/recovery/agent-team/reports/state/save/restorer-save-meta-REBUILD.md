# REBUILD — save+meta 19 函数重建（restorer-save-meta，2026-10-17）

> 方法：判等协议 planning/2026-10-16-logic-verify-protocol.md（8 条分歧检查表）+ 复原 SOP。
> 只以 fresh 反编译/机器码为准。本批新增 fresh 证据（shared 只读）：
> `work/restorer-save-meta/result-fresh1..6.json`（capture_game_snapshot 机器码+callees、
> try_chunk 逐指令、UnsavedFileList::add/contains_file、save_game_to_file 双实例 callees、
> save_game callees、init_game_state_from_snapshot 头/描述符、字符串表 read_memory）。
> 编译：`cargo check -p country-core --lib` **通过**（Finished dev profile；仅剩他模块警告）。
> 测试：`cargo test -p country-core --lib systems::save` **29 passed / 0 failed**；
> `meta_commander` 4 passed；`startup::` 18 passed。本批新增 9 项全绿。

## 总表

| # | 函数 | VA | verdict | 关键改动/分歧点 |
|---|---|---|---|---|
| 1 | commander_update case0 | 0x14091f180 | **重建（极性已对，补 fence 分配位置）** | 门控 `mode∈{13,14}` 等待、否则建会话（既有实现已正确）。fresh：wait 分支**不分配 fence**；旧 Rust 在 `update()` 内先分配（路径 A 等待时泄漏 raw Arc），现把 `new_pair_blocked` 移入门控未命中分支。其余 case 判等维持。 |
| 2 | load_game | 0x14146d030 | **重建** | ① kind clamp：`.min(2)` → `clamp_next_session_kind`（`decoded>2 → 1`，两处）；② 其余（0x18 三字段、variant1 三 payload、variant2 内联 unblock+Arc drop、第二遍 SwitchAppMode 不 push、payload_b 真值）复核已在位。 |
| 3 | save_thread | 0x141720680 | **判等（前批已修，本批复核）** | Shutdown flush=true → flush+return；flush=false → apply deferred + 10ms + continue；无 shutdown_ack。 |
| 4 | execute_save_cmd | 0x141721aa0 | **判等（前批已修，复核）** | case3/4 `flush_deferred_and_sync` + 失败 `UnsavedFileList::add`；CloseFile drop 前一档。 |
| 5 | flush_deferred_and_sync | 0x14171f7f0 | **判等** | 仅逐槽 `change_section`，无 sync_all/flush。 |
| 6 | save_game_system | 0x140a3f470 | **重建** | fresh callees 确认：`capture_game_snapshot_save_data`/`current_snapshot`/`set_current_snapshot`/`send<AsyncSaveCmd>`/`save_folder_path`/`write_to_disk`/`sync_channel+recv`/`try_recv<AutosaveErrorState>`。现实现：触发时**内联**采集（walls/roofs/terrain/stairs）+ `send_command_expect(CaptureSnapshot)` + `record_snapshot`；ChangeSection 变化才发（Colors/Sheep）；`SaveGameSyncData` 注入时**内联同步保存** OpenFile（send panic 语义）；SessionMeta 写；Flush；错误状态 drain。域 Query/历史 edits 由宿主经 `SaveGameSyncData` 注入。 |
| 7a | AutoSaveState::close_file | 0x140a43840 | **重建** | send CloseFile tag4 + unwrap；新增 metronome 重置（`+0x140=0`、`+0x144=0.0`、`+0x148=f32 0x501502f9=1e10`）。 |
| 7b | create_backup_of | 0x141e59860 | **判等（前批已修）** | sync_channel 请求-应答 + recv + Result。 |
| 7c | last_error_state | 0x140a439a0 | **重建** | 4B accessor（`LEA RAX,[RCX+0x10]`）：返回错误状态字段引用（`&Option<anyhow::Error>`），不再返回 `Option<&Error>`。 |
| 8 | capture_game_snapshot_save_data | 0x1417a0430 | **重建（收集器入口）** | 新增 `SnapshotDomainData::collect`（9 收集器：trees/walls/ids/stairs/decorators/三色/terrain/raster/roofs，一次组装）；save_game 与 capture 系统共用 `collect_snapshot_domain`。 |
| 9 | capture_game_snapshot | 0x1417a1d00 | **重建（可证面）** | fresh 证明**非空**（6947B，含重叠跳转）：`__rust_alloc(0x80000)`、5 组排序、`collect_seq<serde_json::value::ser::Serializer>`、`Map::insert`、`CompactFormatter`。新增 `capture_game_snapshot(domain)->Vec<u8>`：`sorted()` + 逐节插入 + 0x80000 缓冲序列化。域 Query 全链登记宿主。 |
| 10 | try_chunk | 0x1417a6d70 | **重建** | 逐指令：入参 `Option<io::Result<Vec<u8>>>`；Some(Ok)→Vec；Some(Err(io))→`anyhow::Error::from<io::Error>`；None→`anyhow!("No chunks left in iterator")`（串 0x142cdc635 len 0x1a）。**删**旧 open/seek/read 自造。 |
| 11 | validate_and_load_closest_snapshot_data | 0x1417a6e50 | **重建** | Version typed i32（非 as_i64）；Walls→`Vec<(u64,WallStateJson)>`、Decorators→`DecoratorsSectionJson`、Terrain→`TerrainSectionJson`、Trees→`Vec<SnapshotTreesEntry>`、三色→`Vec<(u64,u32)>` 逐节 typed；Raster 白名单 + 逐键 flate2 解压（新增 `rasters` 字段），未知键忽略并 log。 |
| 12 | init_game_state_from_snapshot | 0x1417a9c50 | **重建（前缀+可用域）** | fresh：`memcpy 0xc0` + **14** 个 0x20B 描述符（纠正旧 11），字段 `+0x00 数据字 / +0x10 目标指针 / +0x1c 计数`；新增 `SnapshotPrefix::from_bytes` + `init_game_state_from_snapshot`（头/描述符解析 → Walls/Decorators/Terrain/Raster/Stairs 写回；WindowFocusEvent/AvailableMonitor/SettingsMenuState grow 面登记宿主）。 |
| 13 | parse_saved_game_data | 0x141470d00 | **重建** | 全量节：Version(默认 0x19，门 3..=0x1b)、Camera(typed, "parsing camera" fatal)、Time of day(3×f32→legacy f32→默认+告警)、Sheep(Option + "Error parrsing sheep snapshot: ")、Colors(default 回退 + 两条日志)、History(4 字段, "parsing history")、Identity(fallback derive + 日志)、BackwardCompatibility。 |
| 14 | add_game_startup_systems | 0x140941110 | **胶水-合法（注册面）** | fresh 仍为 5 次 `SubApp::add_systems` + chain/run_if 的注册链；28-tuple 名称映射已记录，可挂系统经 `register_core_startup` 聚合；缺宿主 SubApp/域系统的槽位登记，不注册空系统。 |
| 15 | copy_dir_all | 0x141740c00 | **判等（前批已修，纠文档）** | 纯递归复制；删除误导性 `SKIPPED_ENTRY="intro"` 常量、修正 resources/session.rs 陈旧注释（无 sort/intro）。 |
| 16 | load_glade_audio | 0x1408be360 | **判等（前批复核）** | stop_immediately/4 事件/theme==4→0.25/两参数。 |
| 17 | save_game_to_file | 0x140241ca0 / 0x141750c90 | **重建** | fresh callees 证入参 `Iter<MultiEdit>` + 函数内 `serialize<serde_json::value::ser::Serializer>`（×多）+ CompactFormatter。改为 `save_game_to_memory<E: Serialize>(sections, impl IntoIterator<Item=E>)`，函数内 `to_value`+`to_string`；`SerializedJson` 补 Serialize impl 保持现有调用。 |
| 18 | UnsavedFileList::add | 0x14171ff80 | **重建** | fresh：`len<<7`、grow_one、**16 qword=0x80B 整条目拷贝**、parking_lot RawMutex。`UnsavedFileEntry`（path@0 + 0x68 不透明，size 断言 0x80）+ `parking_lot::Mutex<Vec<..>>`；生产路径（execute_save_cmd/OpenFile/load_completion_checker）改 `add_path`。 |

## 新增测试（9）

- `snapshot::tests::try_chunk_wraps_an_existing_result_three_ways`
- `snapshot::tests::capture_game_snapshot_serializes_sections_in_native_order`
- `snapshot::tests::validate_parses_version_typed_and_ignores_unknown_raster_keys`
- `snapshot::tests::snapshot_prefix_reads_header_and_fourteen_descriptors`
- `save::tests::close_file_resets_native_metronome_fields`
- `save::tests::next_session_kind_clamps_decoded_values_above_two_to_one`
- `save::tests::parse_saved_game_data_parses_all_native_sections`
- `save::tests::unsaved_file_list_uses_the_native_0x80_byte_entry`
- `save::tests::save_game_to_memory_serializes_edit_items_inside_the_function`

## 仍登记（缺宿主/跨域资源，不伪造）

1. capture_game_snapshot 的域 Query 全链（walls/decorators/trees/三色/Raster 资源面）→ 宿主采集注入。
2. init_game_state_from_snapshot：原生 14 描述符中的裸内存指针解引用；WindowFocusEvent/
   AvailableMonitor/SettingsMenuState 写回；Stairs/Raster 之外的域目标映射。
3. save_game_system：Camera `store_parameters`（CameraRigParametersCache 非 Resource）、
   History `MultiEdit` 资源、`SaveGameSyncData.sections` 的域组装（宿主注入）。
4. add_game_startup_systems 28-系统 tuple 的精确 SubApp/调度槽位（宿主装配面）。
5. `save_game_to_file` 的 `MultiEdit` 域类型 serde（当前以泛型 `Serialize` 承接）。
6. `AutoSaveState` 原生 +0x28 `Option<HistoryChainSnapshots>` / +0x120 字段：Rust 无对应
   所有权字段（HistoryChainSnapshots 属 history 域），close_file 仅重置已建模字段
   （sections/cached/error/metronome）；布局级字段登记，不伪造。
