# Round5 A/B/C 集中块 DELIVERY（2026-10-16 第五轮 ②）

> 实现对象：① round5 三契约（a-ecs-scheduler-integration / b-sheep-write-path /
> c-snapshot-apply-session-consumers）。文件权内（startup/run_game.rs、startup/mod.rs、
> systems/save/{save_game,snapshot}.rs + ⑤轮授予的 save/time_of_day 面）。**未碰 roof**。
> 整 crate `cargo check` 剩余 4 错误全在用户 wall/snapping 重构域（非本组）。

## A 块：孤儿系统并入生产 Schedule（F2 闭合）

**真实接线**（`startup/run_game.rs` `register_core_startup<B,S>` + `startup/mod.rs`
`add_game_startup_systems<B,S>`）：
- **ivy**：`register_ivy_storage` + `add_ivy_systems`（7 系统链 grower/spawner/pruner/
  leaf_spawner/animation；原 22 系统 0x140944070 序待①——① a 契约 §2 已解 4 系统面与现
  crate 对齐）。send_to_gpu×2 / ivy_audio 为 GPU/audio 宿主接线（① a 契约 §2 登记）。
- **garden**：`register_garden_events` + `add_garden_cpu_systems::<(),()>`（build
  0x140b6d290 CPU 生产面 Draw/Record 链）。
- **input**：`add_input_cpu_consumers`（translate_input→cursor_moved→delta→reset +
  mod_key/update_cursor_position + input_record_replay 回放族）。
- **sheep**：`SheepControl` 资源 + `sheep_controls`（debug 域，仲裁批准落地）。
- **audio**：`add_audio_events`（16 事件）+ `add_audio_systems::<S,B>`。**③ 复审修复
  （round5/REVIEW.md §3）**：**F3 闭合**——`garden_audio_init_system` + `garden_audio_system`
  （T1）补入；**G6 对齐**——① a 契约 §1 G6（0x140b6bd50）6 域（timberframe /
  elevation_supports / clutter / forest / tree / firstperson_steps）+ crate 其余音频域系统
  （audio_update/building/decorator/game_loading/menu/path/playermade_stairs/replay/sheep/
  signifier/terrain/terrain_stairs/trigger/wall_fmod/weather，init+main）按 native 音频组
  补入（bevy tuple ≤20，分 3 批 + weather 单条）。water×2 属 water 子组保留。
- **G2 mesh 载入族**（`startup/mod.rs`，① a 契约 §1）：`startup_garden_brush` +
  `startup_ivy`（Res<AssetMeshLibrary>）。
- **register_core_startup / add_game_startup_systems 改双泛型 `<B,S>`**（B=MusicBackend、
  S=Studio）：audio 系统族需 S。宿主调用面更新。
- **Schedule 库内执行桥**（① a 契约 §6）：run_game_thread 主循环 `Schedule::run` 调用点
  **待①**（capstone 定位 0x1404d8250 线程 run 即执行桥）；crate 已有 gui.rs/photomode.rs
  `schedule.run(&mut world)` 调用点，登记待①定原版驱动模型。

**仍缺**：G1/G3/G4 asset 载入组（load_textures/load_shader/SSBO-cursor/load_prefabs/
config-watch/terrain-height/audio-samples）为渲染域系统（GraphicsDevice 依赖，D组/host
装配面）；28-tuple 槽位映射待①；wall/signifier 孤儿登记待接入（wall 注册经
`register_roof_events` 触 roof 域，signifier_audio 有用户重构错误——不越权）。

## B 块：Sheep 节真实写盘链（F1b 闭合）

**真实接线**（`systems/save/save_game.rs`）：
- `sheep_section_serialized`（O 0x140a51d40，现源已对齐）+ `SheepSaveSectionValue` 资源
  （默认 None→null）+ `sheep_save_section_system`（读 `SheepControl` → 序列化 → 资源）。
- 生产链（真实调用点）：`sheep_controls`（A 块，捕获活羊）→ `SheepControl` →
  `sheep_save_section_system`（生产）→ `SheepSaveSectionValue` → 宿主
  `assemble_game_save_sections` 取 `SaveFileSection::Sheep`=5 真实值 → `write_json_sections`
  → `AsyncSaveCmd::OpenFile`。
- 10 节序（① b 契约 §2）现源 `assemble_game_save_sections` 已对齐（Version/Identity/
  BackwardCompatibility/Camera/TimeOfDay/Sheep/Colors/ChainIndex/HistoryStart/Snapshots）。

**仍缺**：X4 活羊 Query 精确面待①（b 契约 §3，中优先；现以 debug 域 sheep_controls 捕获
为 crate 内生产调用点）；同步保存 OpenFile 触发点由宿主事件响应（现源 L206 登记）。

## C 块：快照应用 + 会话消费端

**真实接线**：
- `systems/save/snapshot.rs`：`SnapshotDomainDescriptor {ptr,len,count}`（① c 契约 §1，
  O）+ `SNAPSHOT_HEADER_SIZE=0xc0` + `SNAPSHOT_DOMAIN_COUNT=11`；`apply_loaded_snapshot`
  按节序迭代 + **三目标 API + 解析层 + 数据源接线**（① c-snapshot-apply-targets +
  c-apply-api-formats + c-apply-section-parsing + **c-apply-data-source 纠偏**）：
  - **数据源定案**：快照文档 = serde_json JSON；节载荷 = JSON 值；0x78B/0x28B 记录 =
    反序列化后内存形式（`{count@0,ptr@8}` = 内存 Vec cap-first 头，非节载荷布局）。
  - Walls：`parse_wall_section(JSON 值)` → serde 反序列化（Vec<[u64;15]> O 形状候选）→
    `Vec<InnerWallStateRecord>`（record[0]=WallId）→ `InnerWalls::load_state`（0x14085ad80）。
  - Decorators：`parse_decorator_section(JSON 值)` → 快照（0x28B 形状；crate 16B≈0x28B
    标注，serde 架构待①）→ `DecoratorStorage::restore_snapshot`（0x140a20d70）。
  - Terrain：`parse_terrain_section(JSON 值)` → deflate 字节（JSON 字符串/数组承载）→
    inflate（flate2 @0x142cdc800）。
  - **待①（中优先）**：Walls/Decorators JSON↔记录 serde 架构、Walls 0x78B 其余 qword、
    Decorators 0x28B/0x16B 逐字段、Terrain strokes 布局。
- 会话消费端（① c 契约 §2）：`time_of_day`（crate 3 参子集系统）注册 + 资源
  （TimeOfDayState/TimeOfDayLightingSink）；`input_record_replay` 经 A 块 add_input_cpu_consumers
  注册；`reset_input_state` 同批。库外 3 消费系统（add_preplaced_autoclutter /
  detect_overhang_support_segments / sheep_unstuck）属 system_clutter/system_wall_constructor/
  system_critters——按契约只保证事件注册 + 资源在场，不实现库外系统。

**仍缺**：apply 三目标已定案分派（c 契约 §2），**数据格式/crate API 缺口待①**（InnerWalls
load_state / DecoratorStorageSnapshot serde / Terrain 字节格式）；Roofs/WallColors/GateColors/
Trees/Raster 域目标与写入序待①；`on_event<NewSessionStartedCmd>` run 条件（
public-session-names entry6，应用系统待①/宿主侧）。

## candidate 链测试

- 本组文件（run_game/startup mod/save_game/snapshot/time_of_day）在 lib 类型检查零错误。
- 整 crate `cargo check` 剩 4 错误全在 `wall/snapping/*`（用户并发重构，非本组）；此前
  signifier_audio 错误已由用户修复。

## 原侧验证

- ① a 契约：procs.tsv into_configs（G1-G6 系统面权威）+ capstone 0x140941110/0x140944070/
  0x140b6d290；ivy 4 系统面（0x1416643e0/5b0/930/d80）。
- ① b 契约：0x140a51d40（to_json_string<Option<SheepControlSnapshot>>）、0x140242b40/
  0x141751c90（write_json_sections）、10 节序对齐。
- ① c 契约：decomp-r5-0x1417a9c50（0xc0B 头 + 11 域描述符）；public-session-names 6 消费
  系统面；T2 契约事件链闭环。

## 组外申请记录

- 无本轮组外修改（save_game.rs/snapshot.rs 为⑤轮文件权授予；startup/* 为②文件权）。
  未触碰 roof（用户重构区）；wall/signifier 孤儿登记待接入未越权。

## 待取证汇总（登记不造）

A：G1/G3/G4 asset 组、28-tuple 槽位映射、Schedule::run 驱动模型、wall/signifier 接入。
B：X4 活羊 Query 精确面、OpenFile 触发点。
C：逐域应用闭包目标映射、on_event run 条件。
