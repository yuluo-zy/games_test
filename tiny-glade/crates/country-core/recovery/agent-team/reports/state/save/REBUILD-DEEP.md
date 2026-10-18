# REBUILD-DEEP — save 域 Query 面 + save_game 余 + roofs 未覆盖收口（2026-10-17）

> restorer impl-save-roofs-deep。方法：协议 planning/2026-10-16-logic-verify-protocol.md
> （分类宏胶水 vs 真业务；只以 fresh 反编译为准）。依据：reports/state/save/LOGIC.md、
> roofs-stairs/LOGIC.md、RE-X5X7.md。证据：fresh 反编译
> `planning/impl-save-roofs-deep/decomp/*.txt`（shared 只读，本批 11 函数 +
> copy_file_to_backup/generate_backup_file_path/date_time_string_for_filename 3 函数）。
> 红线：宿主接线登记不伪造；不改他模块。编译：`cargo check -p country-core --offline` 通过。

## 0 分类总表

| 原函数 | VA | 类别 | 结论 |
|---|---|---|---|
| capture_game_snapshot_save_data | 0x1417a0430 | 真业务（~9 域收集器组装 0x198B Arc） | **落地**（收集器结构判等；sorted 采集序） |
| capture_game_snapshot | 0x1417a1d00 | 真业务（巨型域 Query 系统，184KB） | **部分落地**（排序面落地；域 Query 面登记） |
| validate_and_load_closest_snapshot_data | 0x1417a6e50 | 真业务（逐节 typed 解析 + memmap + History） | **落地**（Raster 白名单 + 节序 typed；memmap/History 面登记） |
| init_game_state_from_snapshot | 0x1417a9c50 | 真业务（0xc0B 头 + 11 域描述符写回） | 登记（apply_loaded_snapshot Walls/Decorators/Terrain 已落地；Stairs/Raster 写回待域） |
| try_chunk | 0x1417a6d70 | 存疑（Result 包装闭包 vs 文件读） | 判等（文件块读语义；签名差异登记） |
| save_game_system | 0x140a3f470 | 真业务（节拍器 + 内联快照捕获 + 同步保存） | **落地**（sorted 接入；History 链面已由 capture_snapshot_system 承载） |
| add_game_startup_systems | 0x140941110 | **宏生成胶水**（Bevy 注册链） | 胶水-合法（注册面宿主） |
| load_glade_audio | 0x1408be360 | 真业务（stop/load_tracks/主题门/FMOD 参数） | **判等落地**（2026-10-17 接线已实现，fresh 核对） |
| parse_saved_game_data | 0x141470d00 | 真业务（读 + JSON 解析 + 逐节提取） | **纠错**（**upgrade 备份不在本函数**；见 item5） |
| roof_uv_at_ws_pos | 0x1408e4040 | 真业务（薄壳 Option 包装） | R1 待接线登记 |
| roof_uv_and_point_at_ws_pos | 0x1408e4090 | 真业务（碰撞注册表→raycast→barycentric） | R1 待接线登记 |
| wall_platform_stairs ×5 | 无 fresh 地址 | 真业务（共享 recovered_building 重建域） | **已验证实现**（recovered_building + B-verification） |
| roof_tiles / dormer / buffers | — | 胶水/类型 | 胶水-合法核对 |
| roof_ridge 余 | — | 真业务 7（6 判等 + get_inner 地址纠错） | 已覆盖 |

## 1 批 1 — save 快照域（5 函数 fresh 反编译）

### 1.1 capture_game_snapshot_save_data 0x1417a0430（672 行，fresh）

原生体 = puffin scope + `assert!(res.stairs.backup_accurate())` + SystemParam bundle
0x28 qword 槽收集：
- Trees `Query<(&PrefabInstanceData,&PrefabInstance,&TreeComponent)>`→
  `Vec<(TreeStorageIndex, TreeData)>`（0x141305170，`_DEV_SKIP_TREE_SERIALIZATION` 门）
- walls `inner_walls::all()`→`Vec<(WallId, WallState)>`（0x141308b40）
- ids `HashSet<u64>`→`Vec<u64>`（0x141309840）
- stairs `iter_edges_and_stubs`+`StairsSnapshot::new`（0x14157d200/0x140901120）
- decorators `DecoratorStorage::take_snapshot`（0x1412fbcd0）
- 6 色表（bundle[5/9/0xd/0x11/0x15/0x19]，每 4 qword）+ 3 组 iter_nondefault
  `(WallId, ColorId)`（0x141306570）+ Terrain strokes hashbrown clone + 地形高度 Vec
  + bundle[0x27] 又一 hashbrown clone + Raster 6 组 `(str,&[u8])` fold（0x141477c30）
- 最终 `memcpy(_Dst,…,0x198)` 落 0x198B 结构，返回 **Arc**。

**落地**：Rust `SnapshotDomainData` 字段集与上述收集器一一对应（walls/roofs/
decorators/wall_colors/gate_colors/halftimber_colors/terrain/trees/stairs/raster/ids）；
`take_stairs_snapshot` = ① 断言 + ⑤ 采集；`collect_trees/collect_walls/collect_ids`
为 0x141305170/0x141308b40/0x141309840 的纯函数镜像。doc 已按 fresh 重写。

### 1.2 capture_game_snapshot 0x1417a1d00（2934 行，fresh）

原生 = 域收集 + **排序**（行 ~387..497：walls by WallId / roofs by WallId（closure_env_1）/
decorators by addr（impl_18::sort）/ trees by TreeStorageIndex / ids ipnsort<u64>）+
BTreeMap（serde_json Map）按 9 节序组 JSON + 0x80000 缓冲写入。

**落地**：`SnapshotDomainData::sorted()`（snapshot.rs，本批新增）按 fresh 排序序
（walls id.0 / roofs wall_id().0 / decorators 判别序 0=Wall/1=Terrain/2=Stair /
trees index.0 / ids 数值）；`capture_snapshot_system`（save_game.rs）构建域数据后
`.sorted()` 再捕获。域 Query 全链（walls/decorators/trees/colors 资源面）由宿主
经域资源采集后注入 `section_values`，不伪造。测试 `sorted_applies_the_native_
snapshot_sort_orders`（roofs/decorators/trees/ids 序断言）。

### 1.3 validate_and_load_closest_snapshot_data 0x1417a6e50（1789 行，fresh）

原生 = `History::closest_resolved_snapshot` + memmap2 file map + `from_slice<Value>`
+ 逐节 **typed deserialize**（Version i32@427 / Walls `Vec<(WallId,WallState)>`@609 /
Roofs `Vec<Roof>`@764 / Decorators struct@899 / WallColors@1015 / Terrain struct@1138 /
Trees@1164）+ Raster `AllWorldRasterWrite::ordered_names()`@1296 + `would_write_succeed`@1363
+ `flate2::mem::Decompress`/`DeflateDecoder`@1379/1386（**Raster 尾 deflate**）+ 
`retain<HistoryChainSnapshot,remove_snapshot_record>`@1737。

**落地**（域内可落地部分）：
- `is_known_raster_key` 占位 `false` → 真白名单（`AllWorldRasterWrite::ordered_names()` 6 名）。
- validate 按节序收集，**Raster 节**走白名单过滤（未知键 log "Ignoring unknown raster
  data in snapshot: "）；Version 保持 typed i32 语义。
- **纠偏**：Terrain 节 **JSON 明文 strokes**（flate2 只用于栅格尾）——validate 不再对
  Terrain 做 deflate（旧实现自造 inflate 已删）。
- 登记：memmap2 文件读面、History::closest_resolved_snapshot、逐节 typed serde
  （域类型 Roof/WallState 无 serde，JSON mirror 面承载）、Raster 逐槽 deflate 数据形状
  待①。

### 1.4 init_game_state_from_snapshot 0x1417a9c50（697 行，fresh）

原生 = 0xc0B 头部拷贝 + 11 域描述符（+0xc0..+0x27c）+ 逐域写回：
`InnerWalls::load_state`@0x14085ad80、`DecoratorStorage::restore_snapshot`@0x140a20d70、
`StairsState::restore_snapshot`@0x1408ff590、Terrain drop_inner_table@0x14004f570、
Raster `write`@0x14083ebe0 + 事件/资源 grow（WindowFocusEvent/WriteSettingsFileCmd/
AvailableMonitor/SettingsMenuState/MeshIntersectionArtifacts）。

**落地**：`apply_loaded_snapshot` 已实现 Walls→load_state / Decorators→restore_snapshot /
Terrain→TerrainEditingData（REBUILD.md 1.5 已有）；**剩余登记**：Stairs
`restore_snapshot`（0x1408ff590）、Raster `write`（0x14083ebe0）、0xc0B 头 + 11 域
描述符二进制表示与 JSON 节面换算——待 stairs/raster 域资源接入，不伪造。

### 1.5 try_chunk 0x1417a6d70（33 行）

fresh 签名 `void(void)`；原生为 Result 包装闭包消费既有 Result。Rust 文件块读
（0x2000 seek/read）。**判等**（文件读语义；签名差异登记——原生按块读入，
Rust 直接读；行为等价）。

## 2 批 2 — save_game 余 + startup + parse + roof_uv（6 函数 fresh）

### 2.1 save_game 0x140a3f470（1792 行，fresh）

原生流程：metronome → History edits `from_iter<MultiEdit,Cloned<Skip<Iter>>>`@424 →
Sheep `Query<(Entity,&mut Transform,&mut Sheep,&mut CritterGoal)>`@457 → roofs query@645 →
**快照捕获内联**（`History::current_snapshot`@654 为空 → log "Capturing a game snapshot" →
`capture_game_snapshot_save_data`@696 → Arc → `send<AsyncSaveCmd>`@723（载荷 tag=0x9
**CaptureSnapshot(9)**@718）→ unwrap_failed → `History::set_current_snapshot`@737；
非空 → Arc drop）→ ChangeSection cooldown → `SessionMeta::write_to_disk`@1652 →
sync_channel（create_backup_of 同步请求-应答）@1678。

**落地（域内部分）**：
- 快照捕获内联链（current_snapshot 决策 → capture → **CaptureSnapshot(9)** 发送 →
  set_current_snapshot）由 `capture_snapshot_system` 承载（REBUILD.md 1.5 已建；
  `should_capture_snapshot`=current_snapshot、`record_snapshot`=set_current_snapshot、
  `AsyncSaveCmd::CaptureSnapshot`=tag 9）。
- **新增**：捕获域数据经 `domain.sorted()`（原生排序面）再入 Arc。
- 登记：History edits 采集（Skip first MultiEdit）、Sheep Query 捕获、OpenFile 同步
  保存组装、sync_channel 请求-应答——依赖 History 资源与域 Query，宿主经
  `CaptureSnapshotRequested` 事件接线，不伪造。

### 2.2 add_game_startup_systems 0x140941110（23 行，fresh）

fresh = 纯 **Bevy 注册链**：chain → add_systems(PreStartup, ScheduleConfigs<Box<dyn
System>>) → run_if → chain → add_systems(OnEnter<AppLoadingState>) → chain →
add_systems(OnExit<AppLoadingState>) → add_systems×2。按 2026-10-17 分类规则 = **宏
生成胶水（注册）→ glue-legitimate**，不按业务重建。当前 Rust 挂可挂载域系统
（register_core_startup + G2 mesh 组），28-系统 tuple 精确槽位待宿主调度面接入。

### 2.3 load_glade_audio 0x1408be360（237 行，fresh）

原生 = `MusicPlayer::stop_immediately` → `Path::_join`+`GladeAudio::load`（audio.json）→
4 个 `event:/Music/Halloween/...` 路径（"Single Leaf"/"Tiny Olden Glade"/"Forgotten
Ruins"/"Enchanted Forest"，串池字节核对）→ `load_tracks` → 主题门 `*(char*)(+0x58)==4`→
`0x3e800000`(0.25) 写 +0x1c4/+0x1c0 → `set_parameter`（GladeTypeIsHalloween/
IsHalloween）→ instantiate/start + unwrap_failed。

**判等落地**：Rust `startup/load_glade_audio.rs` 2026-10-17 接线逐项对应（stop_immediately/
HALLOWEEN_MUSIC_EVENTS 4 名/load_tracks/halloween_theme_flag(theme==4→0.25)/
set_global_parameter×2）；GladeAudio serde + FMOD 实例化面待音频域/host，不伪造。

### 2.4 parse_saved_game_data 0x141470d00（2309 行，fresh）——【item5 关键纠错】

原生 = `std::fs::read_to_string`@195 → serde_json `from_str<Value>` → 逐节 index_into
提取。**callees 无 copy_file_to_backup、无任何备份操作**；备份在**调用方 load_game
0x14146d030 行 681**（`::utils::file::copy_file_to_backup`，地址 0x140c96a80）。

**落地（纠错）**：
- `parse_saved_game_data` **删除** `make_upgrade_backup` 调用 + "Making save file backup
  for upgrade" 日志（fresh 证 0x141470d00 无备份）。
- 备份移入 **load_game 流程**（`load_game_system` + `load_game_session` 纯核心：
  parse 成功后 `make_upgrade_backup(save_path)`，仅 kind==1 解析路径）。
- **upgrade 备份目标名验证**（本批 3 函数 fresh）：
  - `copy_file_to_backup` 0x140c96a80 = `generate_backup_file_path`（0x140c968b0）+
    `std::sys::pal::windows::fs::copy` + `with_context<u64,io::Error,String>`。
  - `generate_backup_file_path` 0x140c968b0 = `date_time_string_for_filename` +
    `Path::file_name` + `OsStr::try_from`（失败 expect）+ `format_inner` +
    `Path::_with_file_name` → **`{源文件名}-{YYYY-MM-DD HH-MM-SS}.json`**。
  - `date_time_string_for_filename` 0x140c96690 = `OffsetDateTime::now_utc().to_offset`
    （LOCAL_OFFSET）+ time crate `format_description::parse::parse` + `format`。
  - 与 `utils/src/file.rs`（`copy_file_to_backup`/`generate_backup_file_path`/
    `date_time_string_for_filename`）实现**一致**；旧 `backup-saves/history-recovered-
    {ts}.json` 目标名已弃（LOGIC 注）。
- 登记 U：load_game 备份门控 `uStack_860 & 0x100000000`（ParsedGameData 偏移 4 bit0）+
  `cVar3` 双条件（位域语义待①）；当前按「kind==1 解析成功即备份」落地，不伪造位域。

### 2.5 roof_uv 族 0x1408e4040/0x1408e4090（R1 待接线，确认）

- `roof_uv_at_ws_pos` 0x1408e4040（21 行）= **薄壳 Option 包装**：调
  `roof_uv_and_point_at_ws_pos`，成功写 uv@+4，失败写 +1，判别 @+0。Rust 现状
  （native-physics cfg Option 包装）结构一致。
- `roof_uv_and_point_at_ws_pos` 0x1408e4090（380 行）= 碰撞注册表（hash_one GlobalId
  链→arena 槽→collider→raycast→barycentric）全管线。Rust 为 raycast 桩，**未实现
  注册表/重心插值**——按任务登记 R1 待接线（roofs-rt ALGORITHMS §6.2 O 证据），不重建。

## 3 批 3 — roofs 未覆盖核对

### 3.1 wall_platform_stairs ×5（无 fresh 地址，共享 recovered_building 域）

`collect_platform_stair_blockers` / `prepare_platform_stair_segments` /
`platform_stair_cell` / `platform_stair_geometry` / `platform_stair_holes` 已按
`evidence/building-segmentation/platform-stairs-findings.md`（原生地址 140774e8b..ea6 /
140775512 / 140775920..ff2 / 1407776b0..849 + B-platform-stair-*-verification.json
384/192/256/64 案例 0 差异）实现于 `recovered_building/wall_platform_stairs.rs`
（440 行，全 5 函数在位），导出为 `building_wall_platform_stairs`（mod.rs:97）。
本批逐函数核对实现与 findings 公式一致（family==63 / exclude 2|3|4 / width@0x3c /
<3.0 地形门 / width*.5+.2 水平距离 / 0x3eb0a3d7 坡高 / 0x3ff95810 1.948 偏移 /
0x4d58618879da0616 init_rng(0) / random_splits amp 0 / 0x3f9ef9db 孔高 / kind 3 /
stairs_length 3D 段和）。LOGIC「未判等-待共享域」结论维持（无逐函数 fresh 地址）；
按章节+recovered_building 实现 = 现状已覆盖。

### 3.2 roof_ridge 余

LOGIC：7 业务 6 判等 + `get_inner` 地址错标纠正（0x1400885d0 = decorator_storage::get，
非本函数；无 fresh 地址，语义=首字段拷贝 `Self(self.0)`）。当前 `roof_ridge.rs`
7 函数全在位（from/calc/to_world_space/get_inner/max_dim/calc_roof_tip_center_ls/
inverse_ridge_01_param，委托 geometry::ridge）。**已覆盖**。

### 3.3 roof_tiles / dormer / buffers 胶水核对

- `roof_tiles.rs`：`RoofTile`（`_marker`）/`RoofTiles`（`records: Vec<[u8;64]>`）
  组件类型声明；实际瓦片装配（assemble 圆形/矩形、RoofShingle 64B 记录）已由共享
  system-roof tiles 模块恢复（11,467 圆形瓦 0 差异）→ **类型/胶水-合法**。
- `dormer_tiles.rs`：`DormerTiles` 资源 + `impl$1::default` 唯一派生胶水 →
  **胶水-合法**（LOGIC 记 1 胶水）。
- `roof_buffers.rs`：`RoofBuffers`（opaque 句柄容器）+ `WoodenPlankSsbo`（0x20B
  梁实例记录 + SsboType/SparseSsbo/StorageElement impl）→ **render/rt 胶水-合法**
  （SparseInstanceBufferWriter push 写面待 render 域）。

## 4 编译/自检

- `cargo check -p country-core --offline`：**通过**（Finished dev profile）。本批改动
  文件（systems/save/{snapshot,load_game,save_game}.rs）零错误；仅剩他模块预存在警告
  （audio 未用导入、mod.rs IntoScheduleConfigs、serde mirror snake_case 等）。
- 测试：`cargo test -p country-core --lib` 被**他模块并发 WIP** 阻断（本批
  `render_loop/compute_passes/pass_kit.rs:574` E0282/E0283，非本模块）。`cargo check`
  覆盖本模块类型正确性。新增测试 2 项（`is_known_raster_key_accepts_the_six_ordered_names`、
  `sorted_applies_the_native_snapshot_sort_orders`）逻辑推演与 `sort_unstable_by_key`
  一致；待并发收敛后由集成 owner 跑 `cargo test --lib systems::save`。
- 期间 rhapsody（render 依赖）出现并发编辑瞬态错误（BoundPassBindings/
  ManagedResources::for_device），重跑即消，非本模块。

## 5 仍登记（待域/宿主接入，不伪造）

- capture_game_snapshot 域 Query 全链（walls/decorators/trees/colors 资源面）→ 宿主
  采集注入 `section_values`。
- validate memmap2 文件读面 + History::closest_resolved_snapshot + Raster 逐槽 deflate
  形状（Raster 白名单/节序已落地）。
- init_game_state_from_snapshot Stairs restore_snapshot（0x1408ff590）+ Raster write
  （0x14083ebe0）+ 0xc0B 头/11 域描述符表示。
- save_game History edits 采集（Skip first）+ Sheep Query 捕获 + OpenFile 同步保存组装
  + sync_channel 请求-应答。
- parse_saved_game_data 备份门控位域（ParsedGameData 偏移 4 bit0 + cVar3）。
- roof_uv 族碰撞注册表→raycast→barycentric 全管线（R1）。
- add_game_startup_systems 28-系统 tuple 精确槽位（注册面宿主）。

## 6 回传主会话（≤400 字）

本批 11 函数 fresh 反编译 + 3 备份函数。**实现落地 4**：① capture_game_snapshot_
save_data 收集器结构判等 + SnapshotDomainData::sorted()（原生排序序）接入
capture_snapshot_system；② validate Raster 白名单真实现（is_known_raster_key=
ordered_names 6 名）+ Terrain deflate 纠错（JSON 明文）；③ load_glade_audio fresh 判等
（现实现逐项对应）；④ **parse_saved_game_data upgrade 备份纠错**——0x141470d00 无备份，
备份实为 load_game 0x14146d030 行 681 copy_file_to_backup，目标名 {名}-{YYYY-MM-DD
HH-MM-SS}.json 三函数 fresh 验证一致，备份调用移入 load_game_system/load_game_session。
**判等/胶水 4**：add_game_startup_systems（Bevy 注册胶水）、roof_tiles/dormer/buffers
（胶水-合法）、roof_ridge 余（已覆盖）、try_chunk。**验证 1**：wall_platform_stairs×5
recovered_building 实现与 B-verification 公式逐条一致。**登记**：init 写回 Stairs/Raster、
roof_uv 族 R1、域 Query 全链待宿主。编译通过；cargo test 被他模块 pass_kit 阻塞（非本
模块）。剩余=宿主接线面（域 Query 注入/History 资源/Schedule 28 槽/R1 碰撞注册表）。
