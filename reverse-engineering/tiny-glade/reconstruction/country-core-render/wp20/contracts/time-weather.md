# WP20 Contract — `time_of_day` / `time_update` / `weather_system` / `world_raster` / `timelapse_replay` / `mode_manager` / `prefab`

Source: `skeleton/B/crates/country-core/src/systems/{time_of_day,time_update,weather_system,
world_raster,timelapse_replay,mode_manager,prefab}.rs`
Evidence: Ghidra decompiles in `analysis/b4_time/`,`b4b_timelapse/`,`b8_prefab/`; PDB bundles 同名。

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## time_of_day.rs（O）

- `time_of_day` @0x140a73460（4912B）：
  - puffin scope；向 `AudioCoreInner::set_parameter` 上报。
  - 状态资源（O）：`pfVar17[0]`=当前小时 float、`pfVar17[1]`=当前太阳角弧度、`@+0xa`=小时
    过渡标志、`@+0xc`=角度过渡标志；目标资源 `piVar23[0]`=播放状态(0/1/2)、`[1]`=进度 f32、
    `[2]`=目标小时 int；`@+0x118`=小时倍率、`@+0xe3`=开发者热键开关。
  - 角度插值：`angle = fmodf(angle, 2π)`，向 `(2π - target)` 用 `exp(-3*dt)` 平滑，收敛
    <0.01 置过渡完成；小时插值向目标小时 wrap ±0.5。
  - 播放状态机（I）：`dev-toggle==0`：state0→0.0、state2→`progress+dt/-5` 下限 0；toggle 开：
    state1→`progress+dt/3` 上限 1。
  - 小时推进：`hour = fmodf(progress*24 + 8, 24)`（一天从 8:00 起）；`TIME_OF_DAY_SPEED`
    （O 常量名 `resources::time_of_day::TIME_OF_DAY_SPEED`）。
  - 日期：`year@[10]`（`piVar23[10]`），`OffsetDateTime::unix_timestamp()` +
    `suncalc::get_position()`（复用 suncalc）→ 方位/高度，`colatitude = π/2 - (azimuth+π)`。
  - `remap_hour_to_fmod_hour` @0x140a74790：小时→fmod 小时 keyframe 曲线（基
    `{0.0,3.5,9.0,17.0,20.0,24.0}`→`{48,17,11,24,22,...}`；冬季 bool 插入 4.52/6.2/5.0/9.0），
    线性插值查找。

## time_update.rs（O）

- `time_update` @0x140922c30：游戏时钟推进。参数：`GameTime`（`*in_RCX+0x10`=总秒、
  `+0x18`=纳秒）、重放 `InputRecordReplayState`（mode==Replaying 时用帧 @+0xc8 的录制 delta）、
  输出时钟 56B `{Duration secs+nanos, total_f32@+0x10, total_60ths_u64@+0x18,
  frac_seconds@+0x20, flag@+0x30, frame_count@+0x2c}`。首两帧 clamp dt ≤ 1/60（`0.016666668`）。

## weather_system.rs（O）

- `weather_system` @0x1408e5b00：`glade_type@+0x58 == 4`（冬）时：
  `intensity@+0xc += (sin(game_time/300*π) > 0 ? +0.1*frac : -0.1*frac)`，clamp [0,1]
  （雪强度 10 分钟正弦）。

## world_raster.rs（O）

- `record_raster_history` @0x140957990：按 variant idx（`idx*0x470`）取 `RasterStrokeCmd`
  的 Vec（elem 0xc B）→ Arc 克隆 → 记录到 `resources::history::History::add`，然后清零 Vec
  （cap 保留 4）。
- `upload_world_raster_buffers_to_staging` @0x140957bf0：puffin scope；输入脏流非空 →
  `resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging()`。
- `draw_raster<Ssbo,WorldRaster<512>,C0,C1,C2>`（O 模板，14 个 monomorph 实例，变体
  Path/Cobblestone1..3/Garden/Water 等）：输入笔划 `Vec<Vec3>`（`iter_points_xz`）→
  `rasterize_stroke<512>` / `rasterize_world_space_stroke`（走查 `resources::raster_stroke`），
  `AssetSsboLibrary` 查 `SsboId`（校验名字 `...RasterStagingSsbo`）→ `copy_to_staging<512>`。

## timelapse_replay.rs（O）

- `TimelapseState`（O 名；字节状态 0=idle/1=录制中/2=完成）；`is_active` @0x14092e6b0。
- `start_timelapse` @0x140a8a310：修剪流 → 有未消费帧 → `mode_manager::check_if_can_switch<
  closure>`（请求切到 Timelapse 模式）。
- `finish_timelapse` @0x140a8a460：修剪流 → 未完成时：推 `MetaCommand`（`VecDeque<MetaCommand>`
  extend）、state=1；无命令待处理时推事件 `{tag, byte}`（TimelapseReplayFinished/Interrupt
  族，O）。
- `cleanup_timelapse` @0x140a8a5d0：有未消费帧 → 日志 + state=2。
- 事件/命令：`StartTimelapseReplayCmd`、`InterruptTimelapse`、`TimelapseReplayFinished`、
  `ReplaySpeedMultiplierOverride`（O 名）。

## mode_manager.rs（O）

- `mode_manager` @0x140a86b60：读 `PrevAppMode`/`RequestAppModeChange` 等；早期返回条件
  （`byte@+0x18`+hash@+0x38+`@+0x28==2`）；修剪流；`check_if_can_switch<closure_env_0>`；按
  `mode@+0x80` jumptable 分发（15 个 `check_if_can_switch` monomorph，每个 ~280-373B）。
- `send_mode_just_changed_event` @0x140a875e0：`AppMode`（2 qword 枚举）前后不等 → 推事件
  `{prev,current}`（stride 0x28）并更新当前。
- 类型（O 名）：`AppMode`、`PrevAppMode`、`RequestAppModeChange`、`RequestToolbarCategoryChange`、
  `RequestHistoryCommand`、`EditSubject`、`WallShape`、`CobblestoneVariant`、`EraserLayer`、
  `UiHistoryPlaySelectAnimation`。`AppMode::is_tool_selected` @0x140a8a5d0、`Default` 折叠。

## prefab.rs（O）

- `spawn_prefabs` @0x140840610：消费 `SpawnPrefabCmd` → `PrefabLibrary::get<PrefabName>` →
  spawn 实体 + `(PrefabInstance, PrefabInstanceData)` 组件（insert tuple）。
- `update_prefabs` @0x140841d60：`update_instance_data<PlacementDataSsbo>` 更新 GPU 实例数据。
- `despawn_prefabs` @0x140842d60：消费 `DespawnPrefabCmd` → `despawn_prefab_contents()` +
  `EntityCommands::despawn`。
- `despawn_prefab_contents` @0x140843390、`create_prefab_rt` @0x140841550、
  `create_prefab_gpu_instances` @0x140841b00、`hot_reload_prefabs` @0x140843740（watch 重载）、
  `PrefabPlugin::build` @0x140840430（注册系统 + `register_required_components` @0x1408446b0）。

## Unknown (U / typed gaps)
- `GameTime` 内部字段精确布局（本包仅 56B 时钟 O 段）；`suncalc` 输入参数（lat/long/date
  编码，O 调用点）；`TimelapseState` 布局；`AppMode` 枚举变体集合与 15 个 `check_if_can_switch`
  逐变体条件；`PrefabInstanceData` / `PrefabInstance` 布局；`RasterStrokeCmd` 字段。

Reused deps：bevy_ecs、bevy_time（Time，离线门面）、puffin、suncalc、hexasphere、enum-iterator。
