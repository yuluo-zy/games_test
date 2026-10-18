# LOGIC — systems/save + meta_commander + startup 逐函数逻辑判等（2026-10-17）

> verifier logic-save。方法：协议 planning/2026-10-16-logic-verify-protocol.md，每函数 Ghidra
> fresh 反编译（shared 只读）→ 重建控制流 → 对照 Rust body。反编译源存
> `recovery/agent-team/planning/logic-save/decomp/*.txt`（本批 fresh），分歧点仅以这些为准。
> verdict：判等 / 未实现-需重做 / 胶水-非业务。

## 总览

- **判等 31**、**未实现-需重做 19**（含 3 个"无/缺 body"：capture_game_snapshot、load_glade_audio、
  init_game_state_from_snapshot 部分）、**未判等 11**（未还原到可判等，标需重做口径）、**胶水-非业务 3**。
- 已实现业务函数审计面合计 **64**（另登记未实现业务函数 3）。
- 用户硬标准命中：**commander_update case 0 两路径状态机的 AppMode 门控极性与原版相反**；
  **save_thread Shutdown 无 flush 分支、flush_deferred_and_sync 落盘语义、CloseFile 分支**与
  原版不符；**save_game_system 仅实现了节拍器，原版内联的同步保存/节变更/快照捕获全缺**。
- 多处 RESTORE/文档声称的 VA 或语义经 fresh 反编译被纠正（见各条）。

---

## 1 systems/save

### 1.1 save_game.rs

| VA | 原函数 | Rust | verdict | 分歧点 |
|---|---|---|---|---|
| 0x140a3f470 | save_game（ECS 系统） | `save_game_system` | **未实现-需重做** | 节拍器段判等（阈值门 `+0x148 <60.0`、`assert delta>=0`、累计、clamp[0,8]、while time>=1 逐秒、7200/3600/600/300 粗粒度档、remove_counter 逆推、log "Capturing a game snapshot"、BackupSave(7) 发送均在场）。**缺**：① 各节 cooldown 变更→ChangeSection(5) 发送（原生 `if (section值!=缓存) { alloc→send }` 多段）；② 内联同步保存（History edits→`Sender::send` 后 unwrap）；③ 触发后内联快照采集（sheep/roof query + `History::current_snapshot()` 为空→`capture_game_snapshot_save_data`→Arc→`send` tag=0x9→`History::set_current_snapshot`）；④ `SessionMeta::write_to_disk`+Flush(0xa)+recv。Rust 把③重构为宿主事件、省略①②④，不判等。 |
| 0x140241ca0 / 0x141750c90 | save_game_to_file<File/Cursor> | `save_game_to_writer` | **未完全判等** | 字节包络判等（`{`+前 7 节 `write_json_sections`+`"History":{\n`+后 3 节+`"edit_chain":[\n`+编辑+`\n]}}`）。原生入参 `Iter<MultiEdit>` 并**函数内 serde 逐条序列化编辑**（`collect_seq`）；Rust 改为调用方预序列化 `&[SerializedJson]`，序列化移出本函数（数据流分歧）。 |
| 0x140242b40 / 0x141751c90 | write_json_sections | `write_json_sections` | **判等** | `"key":` 格式 + value + padding + newline + 区间记录，节 stride 0x28。 |
| 0x140a43810 | SaveFileSection::json_key | 同 | **判等** | disasm：`tag*8` 查 0x142ace328(键长)/0x142ace378(键指)。 |
| 0x140a43830 | SaveFileSection::padding_bytes | `padding()` | **判等** | disasm：查 0x142ace3c8。 |
| 0x140a43840 | AutoSaveState::close_file | `close_file` | **未实现-需重做** | 原生**先 `Sender::send<AsyncSaveCmd>`（载荷 tag=4 CloseFile，disasm CALL 0x1408da280）**，成功后清 +0x120、drop +0x28 `Option<HistoryChainSnapshots>`、+0x140=0、+0x144 写常量；Rust 仅 `sections=default; last_error=None`，不发送、不清原生字段。**AutoSaveState 结构布局整体不对**（原生无 `sections[10]`，节 cooldown 在系统侧）。 |
| 0x141e59860 | create_backup_of<PathBuf> | `create_backup_of` | **未实现-需重做** | 原生 = **同步请求-应答**：`sync_channel<Result<(),anyhow::Error>>`→发 AsyncSaveCmd→`recv` 结果→Err 构 anyhow::Error，**返回 `Result`**。Rust = 直接 `fs::create_dir_all` 返回 bool + log，无命令通道、返回类型不同。 |
| 0x140a439a0 | last_error_state | `last_error_state` | **未判等** | disasm：`LEA RAX,[RCX+0x10]; RET`（返回 +0x10 指针）；Rust 返回 `self.last_error.as_ref()`（布局偏移不符）。 |
| 0x140a439b0 | add_history_edit_to_save_file | `add_history_edit_to_save_file` | **判等** | disasm：push append_offset→edit_offsets；had_edits 才写 `",\n"`；写 value；mark_dirty。 |
| 0x1417522e0 | save_session_meta_to_file | `save_session_meta_to_file` | **判等** | `{"version":..,"state":..}` 包络（"vers"/"sion" LE）+ tmp + sync + rename。 |
| (域侧) | to_json_string<HistoryIdentity/HistoryChainIndex/...> | 域类型 `Serialize` impl / `identity_to_json_string` | **胶水-非业务** | 原生=serde 数值序列化；Rust 用域类型 derive/custom Serialize。 |

### 1.2 save_thread.rs

| VA | 原函数 | Rust | verdict | 分歧点 |
|---|---|---|---|---|
| 0x141720680 | save_thread 主循环 | `save_thread` | **未实现-需重做** | tick 段判等（disasm `CALL 0x14091a230` sync_all；错误 diff→"Auto-save I/O error resolved"/"Save thread I/O error; affected file count: "→mpsc send）；recv AsyncSaveCmd；OpenFile 前置 `save_folder_path`+join+`contains_file`→panic "Was asked to load an unsaved file" 判等；execute_save_cmd 分派判等。**分歧**：① **Shutdown 无 flush**：原生 `0x141721104` jumptable 应用 deferred `change_section`→`sleep(0x989680=10ms)`→**continue 循环**（不退出）；Rust 有活动档时 `break` 退出（Rust 自注"待取证"）。② **shutdown_ack 通道系自造**：原生无第 4 参/无 ack send，退出路径 drop error_sender+commands receiver 后 return；Rust 增 `shutdown_ack` 收发。③ Shutdown-flush：原生只调 `flush_deferred_and_sync`（其内部无 sync，见下）；Rust 的 flush_deferred_and_sync 内 sync_all。 |
| 0x141721aa0 | execute_save_cmd | `execute_save_cmd` | **未实现-需重做** | 分派判等：jumptable（`tag-3` clamp 0..8，越界→5=default，tag8=ManualBackup 落 default，0x142ccb400）；case3/4/5/6/7/9+default 语义在场（ChangeSection/ReplaceEdits 无活动档 panic、BackupSave/ManualBackup/CaptureSnapshot/Flush、公共 truncate 断言）。**分歧**：case3(OpenFile) 与 case4(CloseFile) 均对前一档 `flush_deferred_and_sync`（非 Rust 的 `sync_all`），失败时 **`UnsavedFileList::add()`**（Rust 全程不调用生产 `add`→unsaved 机制死链、OpenFile 前置 panic 恒不可达）；case4(CloseFile) 原生=flush+add+drop，Rust 仅 `*current=None`。 |
| 0x14171ece0 | change_section | `change_section` | **判等** | hash 查 section_ranges、growth+tail、padding 替换、write_at、mark_dirty。 |
| 0x14171f7f0 | flush_deferred_and_sync | `flush_deferred_and_sync` | **未实现-需重做** | disasm 全 CALL 清单**无 `0x14091a230`(sync_all)/`0x1409193d0`(flush)**——原生只逐槽 apply `change_section`（deferred @+0xd8 stride 0x28）。Rust 末尾 `self.sync_all()` 为**新增**。 |
| 0x14171ff80 | UnsavedFileList::add | `add` | **未判等** | 原生条目 **0x80B**（16 qword 拷贝）+ parking_lot Mutex；Rust `Vec<PathBuf>`(24B)。且 Rust 生产路径从不调用 `add`（见 execute_save_cmd）。 |
| 0x1417201a0 | UnsavedFileList::contains_file | `contains_file` | **判等** | 遍历比较（原生 reverse Components eq_by；Rust `PathBuf==`，对 use-case 行为等价）。 |
| 0x141720490 | impl$2::from（AutoSaveError From） | `AutoSaveError` From | **胶水-非业务** | From 转换。 |
| 0x140b92210 | AutosaveErrorState::is_ok | `is_ok` | **未判等** | **VA 错锚**：fresh 符号为 `tg_text::shape_stream::ShapeStream::is_empty`（9B 空函数），非 `AutosaveErrorState::is_ok`。 |
| 0x14172b280 | snapshot_gc_thread | `snapshot_gc_thread` | **判等** | Instant、BufReader(0x2000)、parse_save_file_snapshots→extend、send、extension=="snapshot"(0x746f687370616e73)、file_stem、referenced 过滤、remove、"Deleting unreferenced snapshot"/"Error deleting unreferenced snapshot"。 |

### 1.3 shadow_file.rs

| VA | 原函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x1409184d0 | create | **判等** | tempfile NamedTempFile create_helper + write_all + sync_all + persist。 |
| 0x1409193d0 | flush | **判等** | 无后端→tempfile rewrite；脏块合并写 + set_len。 |
| 0x14091a230 | sync_all | **判等** | flush→backend Some panic→`File::sync_all().context("Asking the OS to sync writes to disk")`→last_error@+0x78。原生首部多一条 trace 级日志（Rust 无，次要）。 |
| 0x14091a450 | mark_dirty | **判等** | 4096B 块 bitvec 区间置位。 |

### 1.4 snapshot.rs

| VA | 原函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x1417a0430 | capture_game_snapshot_save_data | **未实现-需重做** | 原生从 ~9 域收集器（walls/roofs/decorators/colors/terrain/trees/raster…）**函数内构建** SnapshotSaveData；Rust 仅 `Arc::new(SnapshotSaveData{..传入 Vec<(&str,Value)>})` 薄壳，数据采集缺。 |
| 0x1417a1d00 | capture_game_snapshot | **未实现** | 无 Rust body；fresh decomp 空（184KB 系统）。 |
| 0x1417a6d70 | try_chunk | **未判等** | 原生= `Result` 包装闭包（消费既有 Result，Ok(Some)/Ok(None)/Err）；Rust=直接 open/seek/read 文件函数，签名/数据流不符。 |
| 0x1417a6e50 | validate_and_load_closest_snapshot_data | **未实现-需重做** | 节序判等（"parsing Version/Walls/Roofs/Decorators/WallColors/GateColors/Terrain/Trees/Raster" 在场）。原生做**逐节 typed 解析**（TerrainEditingData serde、`AllWorldRasterWrite::ordered_names()/would_write_succeed()` 白名单、flate2 `DeflateDecoder` 栅格尾）；Rust 泛型收 JSON Value + 简化 inflate + `is_known_raster_key()==false` 占位。 |
| 0x1417a9c50 | init_game_state_from_snapshot | `apply_loaded_snapshot` | **未实现-需重做** | 原生=0xc0B 头部+11 域描述符写回链；Rust 为域 serde 部分接线（Walls record[0]+height 已证、其余槽位 0 待①），应用链不判等。 |
| 0x1417ab290 | init_state | — | **胶水-非业务** | 20 SystemParam `init_state`（capture 系统未实现）。 |

### 1.5 load_game.rs

| VA | 原函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x14146d030 | load_game | **未实现-需重做** | 结构判等：kind 解码 `min(decoded,2)`；两遍回放 **fence 终止（tag==0x8000000000000003 break）**；parse_saved_game_data；`USE_FLOAT_GRID_SAMPLING_FIX=version>0x19`、`VERIFY_STAIR_CHECKSUMS=version>0x1a`；camera load_parameters；`init_game_state_from_snapshot`。**分歧**：① 条目解码 clamp：原生 `if(decoded>2) decoded=1`，Rust `.min(2)`（≥3 分派不同）；② 第一遍 variant0 WriteManager：原生推 0x18B `{payload_a,payload_b,counter}`，Rust 仅 1 字段；③ variant1 MetaCommand：原生载 3 payload，Rust 仅 payload[0]；④ variant2 fence 释放：原生**内联 unblock +0x10=0 + Arc drop**，Rust 仅登记 `FenceRelease` 且 `load_game_system` 的 `_ => {}` **丢弃→fence 永不放行→会话切换链断裂**；⑤ 第二遍 variant0 SwitchAppMode：原生仅 log **不 push**，Rust push `SwitchAppMode`；⑥ Apply payload_b：原生载真值，Rust 恒 0。 |
| 0x141470d00 | parse_saved_game_data | `parse_saved_game_data` | **未实现-需重做** | 原生解析**全量节**（含 Identity→HistoryIdentity、Sheep→SheepControlSnapshot 等）；Rust `ParsedGameData` 仅 Version/BackwardCompatibility/Camera/Time of day 4 字段。upgrade 备份：原生走 `utils::file::copy_file_to_backup`，Rust `history-recovered-{ts}` 目标名未对原生验证。 |

### 1.6 save_backup_list.rs / save_backup_metronome.rs / save_file_identity.rs / load_completion_checker.rs

| VA | 原函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x140ae8320 | is_corrupted | **判等** | disasm `CMP byte[RCX],0x4; SETZ`。 |
| 0x140ae8330 | iter_backup_files | **判等** | readdir→Err(anyhow ContextError)/Ok(filter_map 迭代器)；filter_map closure（0x14166df80）细节未单验。 |
| 0x140ae8470 | new_from_session | **未判等** | 重内联；counter max(n+1)/trailing_counter 口径（reviewer FIX-2 遗留）未由 fresh 定案。 |
| (save_game 内联) | add_active_game_time + consider_backing_up | **判等** | 见 1.1 save_game：阈值门、assert、clamp[0,8]、逐秒递减、粗档优先、remove_counter 逆推全在场。 |
| 0x141e58f20 | try_approx_parse_save_file_identity | **判等** | 0x800 alloc + `read_to_end<Take>` + from_utf8 + StrSearcher(搜索) + 解析 u64。Rust 用单次 `read`（原生用 Take::read_to_end），取 0x800 上限等价，次要。 |
| 0x141673140 | derive_fallback_save_file_identity | **未判等** | 原生：`utils::calculate_hash<Option<OsStr>>`（**文件名**哈希）+ stat + `calculate_hash<tuple<u64,SystemTime>>`（元数据哈希）；Rust：seconds^(nanos<<32) 位折叠，推导算法不同。 |
| 0x140a9f810 | check_load_completion | **未判等** | 原生为**完整系统**：定时计算 + AppMode 门（`mode>0xe` 时 `unlink loading.tmp` 清理）+ 标志位 `*in_R8=1`；Rust 仅 `loading.tmp` 存在性判 DenyLoad，缺 AppMode 门/删除/timer。 |
| 0x140833230 | is_ok | **判等** | disasm `CMP byte[RCX],0x0; SETZ`（读 tag 字节；RESTORE 称 +0x10 有误）。 |
| 0x140a9fa70 | deny_load | **判等** | disasm `CMP byte[RCX],0x1; SETZ`。 |
| 0x140a9fa80 | is_session_ok_to_load | **未判等** | 原生返回**三值**：unsaved contains→1、loading.tmp 缺→0、loading.tmp 在→2；Rust 把两种 ok 折叠为 `Ok(0)`，编码与原生不符。 |

---

## 2 meta_commander.rs

| VA | 原函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x14091f180 | commander_update | **未实现-需重做** | consume+sentinel、`switch(tag^0x8000000000000000)`、case1-0x13 分派表在场。**关键分歧 case0 路径 A**：原生 `if (mode∈{13,14}) { 原样重入队(等待) } else { new_pair_blocked→NextSession(kind=2)+append_on_load<Fence>→Commands::insert_resource<NextSession>→重入队带 fence }`；Rust `if !matches!(mode,13|14) { return }`（**极性相反**，正常游戏态会直接消费丢弃会话加载命令），且用 ResMut 改写既有 NextSession 而非 insert_resource。路径 B 判等（fence +0x10 阻塞→重入队；释放→Arc drop）。 |
| 0x14091f0c0 | peek | **判等** | `len@+0x18!=0`→front。 |
| 0x14091f100 | consume | **判等** | pop_front（head 环绕+len--）、空→`0x8000000000000014`；Rust 用 Option 表示（表示差异，行为等价）。 |
| 0x14091f0f0 | is_profiling | **判等** | disasm `CMP qword[RCX+0x38],0; SETNZ`；Rust `!profiler_sinks.is_empty()`（偏移 0x38 语义待钉，登记）。 |
| 0x14091ef90 | new_pair_blocked | **判等（带登记偏差）** | alloc 0x18B Arc（strong/weak），+0x10 显式写 1；Rust `Mutex::new((true,true))` 的第二标志 +0x11 为原生 alloc 残留（RESTORE 已注）。 |
| 0x14091eff0 | new | **判等** | VecDeque 空 + RandomState + 空 HashMap。 |
| 0x141328ac0 | append_on_load<Fence> | **判等** | `{tag=0x8000000000000000,payload=fence_ptr}` + 0xa0B 拷贝。 |
| 0x1403b5150 | append_on_load<[MetaCommand;3]> | **判等** | tag=3 + Box<0xa8B>（reviewer 二轮修正确认）。 |
| 0x140b5a180 | extend_script | **判等** | `VecDeque::spec_extend<IntoIter>` 标准库 extend，Rust `commands.extend` 同操作。 |

---

## 3 startup

| VA | 原函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x140b3fe40 / 0x140b40210 | load_shader_into_library | **未完全判等** | clone name→compile_shader_variants→add_raster 在场；原生 `unwrap_failed`（编译失败/库 miss **panic**），Rust 返回 `anyhow::Result`（错误处理分歧）。 |
| 0x140941110 | add_game_startup_systems | **未实现-需重做** | 原生注册 **28 系统 tuple**（PreStartup/OnEnter/OnExit<AppLoadingState>，chain/run_if）；Rust 只挂 2 个 mesh 系统 + 委托 register_core_startup，不判等。 |
| 0x14166e7d0 | load_game_session | **未判等** | 原生 78KB：attempt→new/existing→建目录/meta→demo 旗标（0x143634d90）选起始 build→`copy_dir_all(0x141740c00)`→SessionMeta try_load→settings→事件；Rust 实现其可落地子集，GameProfile/AutoSaveState/LoadGameManager/AppMode/MetaCommander 消费与主题跳表（0x15 表）未判等（RESTORE 自认）。 |
| 0x141740c00 | copy_dir_all | **未实现-需重做** | disasm 无 sort、无 "intro" 比较：原生=**纯递归复制**（create_dir_all→read_dir→file:fs::copy / dir:recurse）；Rust 增 `sort_by_key` + `intro` 跳过（双端）。**且 session.rs 的 copy_dir_all 错锚此 VA**。 |
| 0x1409e6950 | should_load | **判等** | disasm：8 前缀（0x142ac4780..）+ demo 门控背景（非 demo `default_loading_background` len0x1a / demo `default_loading_background_demo` len0x1f）。 |
| 0x140928100 / 0x140928350 | grid_gen_cw / grid_gen_ccw | **判等** | disasm：`(end+1-start)>5`→`head=min(start+4,end)`→递归 `[start,head)`→emit；CW `[p,q,p+1],[p+1,q,s]`、CCW `[p,p+1,q],[q,p+1,s]`；start>end→unwrap panic。 |
| 0x1415f87c0 | startup_garden_brush | **未判等** | 原生=load_json_as_mesh→load_mesh_into_library→load_shader_into_library→`spawn_empty`+insert `DrawableMeshBundle`(±DontCastShadow)（0x78B 命令入队）；Rust 仅 `AssetMeshLibrary::add`，缺 shader 载入与实体生成。 |
| 0x140a21a30 | startup_ivy | **未判等** | 同上（含 cube 几何 + 3 网格 + shader + spawn）。 |
| 0x1408705b0 | PhotomodeState::new/default | **判等** | scalar demo?1:3、+4..+0xb=0xffffffff、+0xc..+0xd=0x0100；Rust Default→new(false) 与原生读全局 demo 旗标略有差异（游戏内恒非 demo，等价）。 |
| 0x140870570 | on_enable_photomode | **判等** | scalar+index+flag_08(+8)+hide_ui(+0xa)+walk_camera_initialized(+0xc)。 |
| 0x1408be360 | load_glade_audio | **未实现** | Rust 仅常量（HALLOWEEN_MUSIC_EVENTS/参数名）；函数本体未还原。 |
| run_game_thread 0x1405a3440 | new_session_on_load_commands / build_new_session_on_load | **未判等** | cmd2=变体8（0x8000000000000008）形状在场；cmd0/cmd1 tag/载荷仍待定点（RESTORE X7 残留）。 |

---

## 4 关键结论（回传主会话用）

1. **判等/未判等数**：判等 31，未实现-需重做 19，未判等 11（按需重做口径），胶水 3。
2. **最严重分歧**：
   - **commander_update case0 路径 A AppMode 门控极性反**（原生 `mode∉{13,14}` 才建 NextSession+insert_resource；Rust 相反，正常态直接消费丢弃命令）。
   - **load_game 第一遍 variant2 fence 释放被 Rust 丢弃**→fence 永不释放→会话切换链断裂。
   - **save_thread Shutdown 无 flush 分支**（原生应用 deferred+睡 10ms+继续；Rust break 退出）+ **shutdown_ack 自造**。
   - **flush_deferred_and_sync 原生不 sync**（Rust 新增 sync_all）。
   - **execute_save_cmd OpenFile/CloseFile**：原生 flush_deferred_and_sync + 失败 `UnsavedFileList::add`；Rust 用 sync_all、生产从不 add（unsaved 死链）。
   - **save_game_system** 只实现了节拍器，缺内联同步保存/节变更/快照捕获/SessionMeta 写/Flush。
   - **AutoSaveState 结构布局与 close_file/create_backup_of/last_error_state** 与原生整体不符。
3. **VA 错锚**：`AutosaveErrorState::is_ok` 0x140b92210 实为 `ShapeStream::is_empty`；`copy_dir_all` 0x141740c00 无 sort/intro；`is_session_ok_to_load` 三值编码与 Rust 枚举不符。

证据（fresh 反编译/decomp 全文、disasm）：
`recovery/agent-team/planning/logic-save/decomp/`、`result-*.json`（本轮 shared worker）。
