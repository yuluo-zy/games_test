# country-core 系统目录（save/history/audio/ivy/garden/terrain）逐文件还原度复核

- 审计日期：2026-10-19
- 工作目录：`D:/game/tiny-glade/crates/country-core`
- B 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src`
- 机械统计：`D:/game/reverse-engineering/tiny-glade/evidence/audit-2026-10-19/fn_audit.json`
- 基线：`D:/game/tiny-glade/crates/country-core/recovery/STUBS-2026-10-19.md`
- 范围：`systems/save/**`、`systems/history/**`、`systems/audio/**`、`systems/ivy/**`、`systems/garden/**`、`systems/terrain/**`，共 89 个 `.rs`（`systems/history.rs` 作为并列模块补列，不计入 89）。
- 只读审计；未修改源码、未运行测试、未重新反编译。

## 0. 口径与限制

1. `fn_audit.json` 的 `file_cov` 按**骨架文件路径**统计，已去除闭包、泛型实例和部分派生胶水。范围 89 文件对应审计行 181 条：`in_file` 135（74.6%）、`no_trace` 24、`name_only` 10、`moved` 7、`va_only` 5。
2. 因代码已发生**跨文件搬移/合并**，`name_only`/`no_trace` 不能直接当“未实现”。本报告逐条区分：闭包产物 / 泛型实例 / 派生胶水 / 改名合并 / 真缺失。
3. “性质”分四类：**声明**（模块/事件/资源类型）、**合同**（常量/布局/枚举映射）、**实现**（有真实算法/状态/IO 的系统）、**混合**（真实实现 + 明确未恢复分支或占位字段）。
4. “状态”指链闭合程度：**闭合**（生产者→消费者→副作用在本仓库可追通）、**未接线**（实现存在但无注册/无调用者）、**部分**（核心逻辑有，但关键字段/分支/下游缺失）、**空壳**（函数体只有日志/空体）、**测试**。
5. 未复现/不确定项均标注“需人工”，不把“有函数名/有测试”当作语义验收。`STUBS` 的“对得上”只证明名字存在，本报告另判行为。
6. 表中行数为文件总行数（含末行/空行），与 `wc -l` 可能相差 1。

## 1. 总览

- 89 文件中：测试 4（`tests.rs`）、纯声明/注册 5、合同型 3、真实实现约 57、混合约 19、空壳 1（`audio/ivy_audio.rs`）。
- 范围内无 `todo!`/`unimplemented!`；唯一显式空函数是 `save/mod.rs::add_save_load_systems`（L23），且已有 `add_save_load_systems_impl` 被启动装配调用。
- 明确日志/占位：`audio/ivy_audio.rs::ivy_audio_init_system` 只有 `log::trace!`（L21-23）。
- 6 个目录的主流 CPU 链大多闭合；最大的缺口集中在：**快照读回、历史回放的装饰/颜色/楼梯 aux、音频事件上游生产者、音乐主系统接线、Ivy GPU 上传接线**。

## 2. 文件级状态表

### 2.1 systems/save（12）

| 文件 | 行数 | 性质 | 骨架覆盖 | 状态与证据 |
|---|---:|---|---|---|
| `load_completion_checker.rs` | 135 | 实现 | 3/3 in_file | 闭合；加载完成/损坏判定、`loading.tmp` 读取。 |
| `load_game.rs` | 1102 | 混合 | 2/2 in_file | 部分；解析/版本/相机/时间/羊/颜色/History 应用真实；**有效快照一律拒绝**（L267），`ApplyCounter` 纯推进分支要求缺失 producer（L711），未知读档后域命令直接 bail（L1036）。 |
| `mod.rs` | 86 | 声明/注册 | 2/2 in_file | `add_save_load_systems` 为空壳（L23-30）；`add_save_load_systems_impl`（L75-85）被 `startup/run_game.rs:182` 调用，事件/资源注册闭合。 |
| `save_backup_list.rs` | 146 | 实现 | 2 in_file +1 name_only(`fmt`) | 闭合；备份目录扫描/过滤/计数。`fmt` 为派生胶水。 |
| `save_backup_metronome.rs` | 118 | 实现 | 未在审计（新文件） | 闭合；阈值/计时/retention 决策。NaN 比较语义待取证（L73）。 |
| `save_file_identity.rs` | 149 | 实现 | 2/2 in_file | 闭合；文件头身份解析、回退派生。 |
| `save_game.rs` | 1282 | 混合 | 11 in_file / 4 no_trace / 3 va_only / 1 moved | 核心闭合；JSON 节/编辑链序列化、节缓存、异步命令、同步 flush 真实。`padding_bytes` 以 `SaveFileSection::padding` 合并（L89）；节 cooldown 表值待取证（L552）。 |
| `save_thread.rs` | 898 | 混合 | 8 in_file +1 no_trace(closure) | 闭合；真实任务线程、ShadowFile、Close/Open/Flush/Backup、快照写盘。手动备份身份只校验 +0x53 字节（L330-343，待取证）；`start_snapshot_gc` 无生产调用者（L437/454）。 |
| `shadow_file.rs` | 166 | 实现 | 4/4 in_file | 闭合；同目录临时文件 + persist、脏块/flush。 |
| `snapshot.rs` | 2061 | 混合 | 4 in_file +1 name_only(`init_state`) | 部分；捕获/编码/解码/6 域 JSON 真实；应用侧 Roofs/WallColors/GateColors/Trees 仅 trace（L1332-1333），Walls 曲线/枚举打包待取证（L1392-1409），Raster 需宿主写目标（L1327）。`init_state` 为 Bevy 派生胶水。 |
| `snapshot_gc.rs` | 127 | 实现 | 1/1 in_file | 实现未接线；解析/删除真实，但仅测试调用 `snapshot_gc_thread`，打开存档流程未启动 GC。 |
| `tests.rs` | 484 | 测试 | — | 覆盖 save/load/snapshot/线程协议。 |

### 2.2 systems/history（12 + 并列 `systems/history.rs`）

| 文件 | 行数 | 性质 | 骨架覆盖 | 状态与证据 |
|---|---:|---|---|---|
| `finish_replay.rs` | 167 | 实现 | 1/1 in_file | 闭合；自然结束/显式停止/模式恢复/羊解冻事件。 |
| `recording.rs` | 979 | 实现 | 合并 `history_recording.rs` | 闭合；录制开始/结束生命周期、屋顶视觉/方向记录、与 UI/InputReplay/Decorator/Stairs 的真实联动。 |
| `replay_auto_clutter.rs` | 58 | 实现 | 1/1 in_file | 闭合；前桶过滤、undo 集增删、时长门控。 |
| `replay_color.rs` | 73 | 混合 | 未在审计（合并自 replay_color） | 部分；命令推进/缓动真实，但只发 `ColorHistoryReplayApply`，**本 crate 无消费者**（L10-14）。 |
| `replay_decorator.rs` | 72 | 混合 | 未在审计 | 部分；同上，`DecoratorHistoryReplayApply` 无消费者（L10-13）。 |
| `replay_mode.rs` | 416 | 实现 | 1/1 in_file | 闭合；进入/退出、清画布、长按快进、速度插值、HistoryCmd 写入。 |
| `replay_raster.rs` | 478 | 混合 | 2 in_file /1 name_only /5 no_trace | 大体闭合；Path/Cobble1-3/Water/Garden 六域统一推进、undo 重建 composite、scratch/staging 真实。`Dirt/CobblestoneXPathFilter` 未按类型建模（no_trace，使用 selector/flags；需人工确认语义等价）；`backward` 合并为 undo 分支（name_only）。 |
| `replay_stairs.rs` | 259 | 混合 | 1/1 in_file | 部分；边集 undo/redo、装饰过滤、音频事件真实；**decorator 载荷被丢弃，`PlayStairsAudioCmd` 以 zeros 发送**（L78-85）。 |
| `replay_wall.rs` | 361 | 混合 | 1 in_file /1 no_trace /1 va_only | 部分；墙权威快照、分组/并发边界、instant/timed 效果真实；`WallHistoryReplayAudio` / `WallHistoryRestoreColor` **无消费者**（L24-29）；变体 5..16 无 wall 字段（L49-53 哨兵）。 |
| `tests.rs` | 531 | 测试 | — | 覆盖 pop/undo/音频静默/加载回放。 |
| `wall_replay_interpreter.rs` | 546 | 实现 | 未在审计（新文件） | 闭合（已恢复操作）；StateChange op0-2、Enclosure、AdjustHeight、并发组边界；未知 op>2 返回 `UnrestoredOperation`（`wall_replay_transition.rs:61-63`）。 |
| `wall_replay_transition.rs` | 118 | 实现 | 未在审计 | 闭合；StateChange op1/2 与 NaN 感知算术。 |
| `systems/history.rs`（并列） | 554 | 实现/注册 | 对应 `history_process.rs`/`history/mod.rs` | 闭合；`history_process_system`、命令入队策略、Undo/Redo 展开、`replay_pop_bucket`、`history_clear_previous_state`、事件/系统注册。**但 `replay_pop_bucket_system` 与 `history_clear_previous_state` 未注册到 `add_history_cpu_systems`（L532-536 只注册空桶 pop）**。 |

### 2.3 systems/audio（41）

| 文件 | 行数 | 性质 | 骨架覆盖 | 状态与证据 |
|---|---:|---|---|---|
| `audio_update.rs` | 140 | 混合 | 2/2 in_file | 部分；VCA/监听器/更新真实；本文件另建 `AudioSettings`（L31），与 `resources/audio_settings.rs:18` 真实类型重复，**crate 内无生产者**；布局待取证（L28）。 |
| `building_audio.rs` | 138 | 混合 | 2 in_file +1 name_only(`fmt`) | 部分；7 loop + 3 one-shot 真实；`BuildingSize` 用 `editing_speed` 顶替，0x2a8 building 表待接线（L98-101、L118-120）。 |
| `clutter_audio.rs` | 103 | 实现 | 1/1 in_file | 闭合；39-kind 分发表。 |
| `decorator_audio.rs` | 183 | 实现 | 2/2 in_file | 闭合；loop/one-shot 分派。 |
| `elevation_supports_audio.rs` | 53 | 实现 | 1/1 in_file | 闭合；支撑事件名/分派。 |
| `firstperson_steps.rs` | 166 | 实现 | 1/1 in_file | 闭合；脚步 loop/参数。`to_fmod_param` 以局部映射合并。 |
| `forest_audio.rs` | 93 | 混合 | 2/2 in_file | 部分；FMOD loop 业务真实；`ForestTree` 是 tree_storage 组件的本地占位（L29-31，待接线）。 |
| `game_loading_audio.rs` | 88 | 实现 | 2/2 in_file | 闭合（音频侧）；loading 域 producer 待接线（L54）。 |
| `garden_audio.rs` | 331 | 实现 | 2/2 in_file | 闭合；4 loop、门控/参数/停止。 |
| `ivy_audio.rs` | 37 | 空壳 | 1/1 in_file | **空壳/未接线**；`ivy_audio_init_system` 只有日志（L21-23），没有 `instantiate_audio(event:/SFX/IvyGrow)`；`ivy_audio_system` 接收注入闭包而非 `AudioCore`，且未注册。 |
| `menu_audio.rs` | 94 | 实现 | 2/2 in_file | 闭合；snapshot + 13 UI one-shot。 |
| `mod.rs` | 414 | 声明/注册 | 3/3 in_file | 事件类型与 `add_audio_events`/`add_audio_systems` 真实；但注册了缺生产者的 `audio_update`、`weather_audio`、`terrain_audio`（见缺口清单）。仅注册 `music_debug_gui`（L254），未注册主音乐系统。 |
| `music/audio_core_backend.rs` | 101 | 实现 | 新文件 | 闭合；`AudioCore`→`MusicBackend/MusicEvent` 适配。 |
| `music/glade_audio.rs` | 129 | 实现 | 新文件 | 闭合；`GladeAudio` 72B 布局与 audio.json 读取/回退。 |
| `music/load_glade_audio.rs` | 141 | 实现 | 新文件 | 实现未接线；主题音频加载函数真实，但 `configure_glade_audio` 无调用者（L22-24）。 |
| `music/mod.rs` | 11 | 声明 | — | 模块声明。 |
| `music/music_debug_gui.rs` | 56 | 实现 | 1/1 in_file | 闭合；调试窗口与强制切歌。 |
| `music/music_player.rs` | 484 | 实现 | 9/9 in_file | 闭合；播放列表/切歌/概率切换/淡入/拍照快照状态机；缺名 `current_track`/`generate_track_playlist`/`probabilistic_switch_track` 已合并进字段/`switch_track`/`update_with_random`。 |
| `music/music_system.rs` | 180 | 实现 | 2/2 in_file | **实现未接线**；richness/NewGladeTime/预览/拍照快照逻辑真实，`configure_music_consumers` 无调用者（L54-63）；`music_audio_system` 不在启动调度中。 |
| `music/native.rs` | 103 | 实现 | 新文件 | 闭合；真实 libfmod 2.222.6 适配（feature `native-audio`）。 |
| `music/parameters.rs` | 40 | 实现 | 新文件 | 闭合；MusicRichness 边界与 NewGladeTime 累加。 |
| `path_audio.rs` | 180 | 混合 | 2/2 in_file +2 no_trace(闭包) | 部分；5 loop 与三流分派真实；路径/擦除事件 producer 在 paths 域待接线（L36-49），raster gate 由宿主写（L59-63）。 |
| `playermade_stairs_audio.rs` | 140 | 实现 | 2/2 in_file | 闭合；6 loop + 台阶 kind。 |
| `replay_audio.rs` | 86 | 实现 | 2/2 in_file | 闭合；Undo/Redo/Deny 音效。 |
| `roof_audio.rs` | 304 | 实现 | 4/4 in_file | 闭合；3 loop + one-shot；producer 在 roof 域（L75）。 |
| `sheep_audio.rs` | 146 | 实现 | 1/1 in_file | 闭合；kind 0..7。 |
| `signifier_audio.rs` | 133 | 混合 | 2/2 in_file | 部分；1 loop + 4 one-shot/工具参数；21 参数映射待取证（L9-10）。 |
| `terrain_audio.rs` | 657 | 实现 | 2/2 in_file | 闭合（消费侧）；6 类事件/loop/one-shot 真实；`Res<EditingSpeed>` 非 Option（L100），本 crate 无生产者。 |
| `terrain_stairs_audio.rs` | 293 | 混合 | 1/1 in_file | 部分；楼梯绘制 loop/SSBO counter 真实；SSBO 类型 id/域 producer 待取证（L88-92）。 |
| `timberframe_audio.rs` | 207 | 实现 | 2/2 in_file | 闭合；编辑 loop/参数。 |
| `tree_audio.rs` | 196 | 实现 | 2/2 in_file | 部分；4 loop + place 真实；tree-tool 域 producer 待接线（L51）。 |
| `trigger_audio.rs` | 226 | 实现 | 2/2 in_file | 闭合；相机分段/触发盒。 |
| `wall_audio_fmod.rs` | 137 | 实现 | 2/2 in_file | 闭合；10 loop + kind 0..23；EditingSpeed 可选（L70）。 |
| `wall_audio_orchestrator.rs` | 92 | 实现 | 1/1 in_file | 闭合；材质音频协调、写 `PlayWallAudio`。 |
| `wall_audio_path.rs` | 119 | 实现 | 1/1 in_file | 闭合；墙路径 producer。 |
| `wall_audio_ui.rs` | 209 | 实现 | 1/1 in_file | 闭合；UI kind 分派。 |
| `wall_audio_utils.rs` | 45 | 合同 | 1/1 in_file | 仅有 loop 常量与共享实现转发；业务在 `resources::audio_instance`。 |
| `wall_kinds.rs` | 123 | 合同 | 新文件 | kind 0..23 映射/证据笔记，无独立系统。 |
| `water_ambience_audio.rs` | 170 | 混合 | 3/3 in_file | 部分；逐格 loop、popcount 音量真实；网格尺寸/采样偏移待取证（L31-33），cell state producer 待接线（L52）。 |
| `water_audio.rs` | 117 | 实现 | 2/2 in_file | 闭合；4 loop。 |
| `weather_audio.rs` | 46 | 混合 | 新文件 | 部分；只写 `RainStrength`；本文件另建 `Weather`（L15），与 `resources/weather.rs:7` 真实类型重复，crate 内无生产者；参数名/字段映射待取证（L6、L19）。 |

### 2.4 systems/ivy（9）

| 文件 | 行数 | 性质 | 骨架覆盖 | 状态与证据 |
|---|---:|---|---|---|
| `ivy_branch_thickness_animation.rs` | 20 | 实现 | 1/1 in_file | 闭合；按点数推进粗细。 |
| `ivy_grower.rs` | 927 | 实现 | 3/3 in_file | 闭合（CPU）；10 阶段状态机、墙体相交/跳墙/孔洞门控真实；少量 native 字段语义待取证（L196-203、L323、L505）。 |
| `ivy_leaf_animation.rs` | 43 | 实现 | 1/1 in_file | 闭合；延迟生长/端点粗细传播。 |
| `ivy_leaf_spawner.rs` | 254 | 实现 | 1/1 in_file | 闭合；线段完成→叶/花。 |
| `ivy_pruner.rs` | 323 | 实现 | 4/4 in_file | 闭合；删除命令、墙体形状重验证、高度带。 |
| `ivy_send_to_gpu_leaf.rs` | 441 | 实现 | 1/1 in_file | 实现未接线；48B 行打包/上传/instance_count 真实；花通道概率语义待取证（L126-131），上传函数不在 Schedule。 |
| `ivy_send_to_gpu_segments.rs` | 150 | 实现 | 1/1 in_file | 实现未接线；48B 段打包/上传真实，未注册。 |
| `ivy_spawner.rs` | 492 | 实现 | 1/1 in_file | 闭合（CPU）；光标根/随机根、预算/距离门控真实；上界 40 待取证（L77）。 |
| `registration.rs` | 181 | 实现/注册 | 对应 ivy/mod.rs | CPU 链接线（`startup/run_game.rs:187`）；GPU 上传与音频待接线（L79-84）。 |

### 2.5 systems/garden（4）

| 文件 | 行数 | 性质 | 骨架覆盖 | 状态与证据 |
|---|---:|---|---|---|
| `draw_garden.rs` | 153 | 实现 | 1/1 in_file | 闭合；模式/笔刷/边界/widget 状态机→`draw_raster`→draw/erase/record 事件。 |
| `mod.rs` | 115 | 声明/注册 | 新文件 | 事件、CPU 顺序、`add_garden_cpu_systems` 真实；被 `startup/run_game.rs:188` 调用。 |
| `record_garden_history.rs` | 39 | 实现 | 新文件 | 闭合；消费通知游标，槽 5 记录一次。 |
| `tests.rs` | 164 | 测试 | — | 覆盖绘制/记录/回放反馈。 |

### 2.6 systems/terrain（11）

| 文件 | 行数 | 性质 | 骨架覆盖 | 状态与证据 |
|---|---:|---|---|---|
| `deferred_terrain_history.rs` | 70 | 实现 | 1/1 in_file | 闭合；待定地形编辑→History，断言单槽。 |
| `delayed_terrain_changed_msg.rs` | 45 | 实现 | 1/1 in_file | 闭合（CPU）；两帧延迟、authority/visual 两次 copy；**`OnTerrainChange` 本 crate 无消费者**（L32 只写）。 |
| `draw_terrain_stroke.rs` | 27 | 实现 | 1/1 in_file | 闭合；合并未读变更、请求重算。 |
| `edit_terrain_stroke.rs` | 110 | 实现 | 1/1 in_file | 闭合；点集/参数编辑→`apply_resolved`→音频/重算/readback。 |
| `mod.rs` | 182 | 声明/注册 | 1 in_file +2 no_trace | 事件/资源/系统集注册真实；`insert_terrain_resources`/`add_terrain_events` 合并进 `register_terrain_events`（L54+）。 |
| `replay_terrain.rs` | 204 | 实现 | 合并 `history/replay_terrain.rs` | 闭合；typed 历史→scratch 应用→edit/point/changed 事件；`forward/backward` 合并进 `TerrainReplayCmd::expanded/eased/advance`。 |
| `terrain_hover_idx.rs` | 187 | 实现 | 1/1 in_file | 闭合；悬停 id/水体采样/回读。 |
| `ui_draw_terrain.rs` | 973 | 实现 | 1/1 in_file | 闭合；模式门、画布、边界、widget 状态机、新笔画/续画/提交、碎石高度采样。 |
| `ui_edit_terrain.rs` | 728 | 实现 | 1/1 in_file | 闭合；逐笔画 widget、拖拽 case 分派、历史 pending、提示。 |
| `ui_terrain_focus_mode.rs` | 241 | 实现 | 1 in_file +4 no_trace(闭包) | 闭合；聚焦选择/射线/widget/绘制拓扑。 |
| `tests.rs` | 279 | 测试 | — | 覆盖编辑/回放/延迟回读。 |

## 3. 空壳、水分与占位

### 3.1 明确空壳/空体

| 位置 | 内容 |
|---|---|
| `systems/save/mod.rs:23-30` | `add_save_load_systems` 空体 + 注释“留待域恢复后接入”；但 `add_save_load_systems_impl` 已真实接线，属**遗留空壳**。 |
| `systems/audio/ivy_audio.rs:21-23` | `ivy_audio_init_system` 只有 `log::trace!`，未实例化 `event:/SFX/IvyGrow`；为**真空壳**。 |
| `systems/audio/ivy_audio.rs:29-36` | `ivy_audio_system` 用注入 `FnMut` 代替 AudioCore，且未注册；不是生产路径。 |

### 3.2 只日志/只默认值/只占位

- `systems/audio/ivy_audio.rs:21` 日志唯一副作用（同上）。
- `systems/save/snapshot.rs:1332-1333` Roofs/WallColors/GateColors/Trees 应用只有 `log::trace!`。
- `systems/save/snapshot.rs:1267` Walls 解析为空时只 trace；`L1392-1409` 曲线/枚举区明确“待①”，记录保持 0。
- `systems/audio/weather_audio.rs:12-23` 本文件 `Weather` 为待接线替身类型，不是真实 `resources::weather::Weather`。
- `systems/audio/audio_update.rs:26-37` 本文件 `AudioSettings` 为 5 音量占位；真实 serde 类型在 `resources/audio_settings.rs:18`，注释明确“用后删除占位”（`resources/audio_settings.rs:9-11`）。
- `systems/audio/forest_audio.rs:29-31` `ForestTree` 为 tree_storage 组件占位。
- `systems/audio/building_audio.rs:98-101、118-120` `BuildingSize` 用 `editing_speed` 顶替，building 表 0x2a8 未接线。
- `systems/audio/path_audio.rs:36-49` 三个路径事件 producer 属 paths 域，待接线。
- `systems/audio/water_ambience_audio.rs:31-33、52` 网格尺寸/位置偏移/state producer 待取证或待接线。
- `systems/audio/signifier_audio.rs:9-10` 21 参数映射待取证。
- `systems/audio/terrain_stairs_audio.rs:88-92` SSBO 类型 id/域 producer 待取证。
- `systems/history/replay_stairs.rs:85` decorator 载荷丢弃，音频命令以 zeros 发送。
- `systems/save/save_thread.rs:330-343` 手动备份只实现 +0x53 身份字节，+0x299 起结构待取证。
- `systems/save/save_game.rs:539、552` 存档载荷字段名/`save_cooldown_seconds` 表值待取证。
- `systems/ivy/ivy_send_to_gpu_leaf.rs:126-131` 花概率 split 待取证。
- `systems/ivy/ivy_grower.rs:196-203、323、505` 若干 native 字段/变换待取证。

> 说明：以上“待取证/待接线”不都是空壳；多数是核心算法已实现、字段/上游/下游未闭合。

## 4. 骨架 no_trace 对照（24 条，去闭包/泛型口径）

> `no_trace` 定义：全 crate 无同名函数且无同地址函数。下表逐条判定。结论：**24 条中 18 条是闭包/泛型/派生/搬移，2 条真缺失，4 条 path filter 需人工确认**。

| 骨架位置 | 函数/符号 | 判定 |
|---|---|---|
| `audio/path_audio.rs` | `closure_env$1>`、`closure_env$5>` | 闭包产物（路径位置/半径回调），非缺失。 |
| `history/history_process.rs` | `closure_env$0>` | 闭包产物（`generate_replay_cmds_from_edit` 实例）。 |
| `history/mod.rs` | `add_history_systems`、`insert_history_resources`、`add_history_events` | **改名合并**：`systems/history.rs::add_history_cpu_systems`/`register_history_events`。 |
| `history/replay_path.rs` | `generate_path_replay_cmds` | **改名合并**：`ReplayBucket::expand_with_aux` + `replay_raster` 统一域；路径专属过滤需人工。 |
| `history/replay_raster.rs` | `DirtPathFilter>`、`Cobblestone1/2/3PathFilter>` | **真缺失/未按类型建模**：当前用 `RasterDomain` + selector/flags；是否语义等价需人工。 |
| `history/replay_raster.rs` | `closure_env$2>` | 闭包产物。 |
| `history/replay_wall.rs` | `_deprecated_morph_curve` | 原版弃用符号，未恢复属预期。 |
| `save/save_game.rs` | `File>`、`HistoryChainSnapshots>`、`HistoryChainIndex>` | 泛型实例名/序列化胶水，由 `SerializedJson`/`assemble_game_save_sections` 覆盖。 |
| `save/save_game.rs` | `padding_bytes` | **改名合并**：`SaveFileSection::padding`（L89）。 |
| `save/save_thread.rs` | `closure$0` | 闭包产物。 |
| `terrain/mod.rs` | `insert_terrain_resources`、`add_terrain_events` | **改名合并**：`register_terrain_events`/`add_terrain_cpu_systems`。 |
| `terrain/ui_terrain_focus_mode.rs` | `closure$1..4` | 闭包产物。 |

同时：

- `name_only` 10 条中：`fmt`×2、`init_state` 为派生/框架胶水；`history_process`→`history_process_system`；`history_finish_recording`→`recording.rs`；`backward`→`advance_raster_replay` undo 分支；`forward`/`backward`（terrain）→`TerrainReplayCmd`；**`signifier_replay_active` 与 `register_required_components` 为真缺失**。
- `moved` 7 条均确认为搬移：`replay_edit`→`expand_with_aux`；`replay_pop_bucket`→`systems/history.rs`；`replay_terrain`→`terrain/replay_terrain.rs`；ivy `add/insert/events`→`registration.rs`；`add_history_edit_to_save_file`→`save_thread.rs`。
- `va_only` 5 条：`interrupt`→`InterruptTimelapse` 事件；`GladeColorCustomization>`/`TimeOfDay>`/`CameraRigParametersCache>` 为 `serialize<T>` 泛型实例；`closure_env$0>` 为闭包。
- **重点文件结论**：`save/snapshot`、`save/save_game`、`save/load_game`、`save/save_thread`、`history/replay_*`、`history/history_process`、`audio/music`、`ivy grower/spawner/send_to_gpu`、`terrain ui_*`、`garden` 的**核心具名业务函数均能找到对应实现或合并等价物**；真正的“全无痕迹”主要是 `signifier_replay_active`（2 函数 + `ease_out_elastic_jiggle`）和 4 个 path filter 类型。

## 5. 抽读最大文件（12+）结论

抽读：`snapshot.rs` 2061、`save_game.rs` 1282、`load_game.rs` 1102、`recording.rs` 979、`ui_draw_terrain.rs` 973、`ivy_grower.rs` 927、`save_thread.rs` 898、`ui_edit_terrain.rs` 728、`terrain_audio.rs` 657、`wall_replay_interpreter.rs` 546、`history/tests.rs` 531、`replay_raster.rs` 478；另读 `music_player.rs` 484、`ivy_spawner.rs` 492、`audio/mod.rs` 414、`history.rs` 554、`snapshot_gc.rs`/`shadow_file.rs` 等。

1. **save 写盘链是真实实现，不是签名壳**：`save_game_system`→`AsyncSaveCmd`→`save_thread`→`ShadowFile` 全量重写/增量节/编辑追加/flush，`auto_save` 节缓存、同步 flush、备份与 retention 均有真实代码；`snapshot.rs` 的编码/解码也使用 `motomoto` 真实分块与 flate2。
2. **save 读回链只闭合到“无快照的历史编辑回放”**：`load_game.rs:267` 一旦发现可用快照就 `bail!`，`init_game_state_from_snapshot` 在 crate 内无调用者；`snapshot.rs:1332-1333` 的 4 个域应用只有 trace，Walls 曲线/枚举仍待取证。因此 load→应用不是完整闭合。
3. **history undo/redo 主链真实**：`history_process_system` 解析 4 字节命令、录制门控、音频/中断效果、`cursor_step_expanding`→`ReplayBucket`→各 replay 系统→`finish_replay` 均实现并有测试。**但生产 Pop 阶段只注册 `pop_empty_replay_bucket`，未注册 `replay_pop_bucket_system`（history.rs:532-536）**，导致 decorator/stairs aux diff 与 collider flush 不在调度中；`history_clear_previous_state` 也未注册。
4. **history 的“应用端”分裂**：wall 直接写 `InnerWalls`；raster/terrain 写各自 owner；stairs 写 edge 集；而 color/decorator 只发事件且无消费者，stairs 音频载荷用 zeros。故 undo/redo 对建筑几何/raster/terrain 闭合，对颜色/装饰/楼梯音频不闭合。
5. **terrain CPU 链闭合**：`ui_draw/ui_edit`→`EditTerrainStrokeCmd`→`edit_terrain_stroke`→`RecalculateHeightMapCmd`/`ScheduledTerrainReadback`→`delayed_terrain_changed_msg` 均真实；`OnTerrainChange` 本 crate 无消费者，GPU 端后置。
6. **ivy CPU 链闭合，GPU 链未接线**：grower/spawner/pruner/leaf-spawner/animation 已注册并实现真实状态机；`upload_ivy_leaves/segments` 是真实打包+`gpu_write`，但不在 Schedule（`registration.rs:79-84`）。
7. **音频消费侧普遍真实、生产侧缺失**：`add_audio_systems` 注册 38+ 音频系统；各文件多为真实的 FMOD 参数/loop/one-shot 分派。但 `PlayBuildingAudio`/`PlayClutterSoundCmd`/`PlaySheepReactSound`/`PlayMenuUiSound`/`PlayTreeSoundCmd`/`PlayTimberframeSound`/`PlayElevationSupportSoundCmd` 在本 crate 内**0 个 writer**。
8. **音乐播放器状态机真实，但主系统未接线**：`music_player.rs` 的切歌/概率/淡入/拍照快照逻辑完整；`music_audio_system` 与 `load_glade_audio` 的配置函数均无调用者，`add_audio_systems` 只加 `music_debug_gui`。
9. **garden 真实且闭合**：`draw_garden` 的模式/笔刷/widget/边界→`draw_raster`→draw/erase/record 事件，`record_garden_history` 槽 5 记录，`route_garden_replay_feedback` 消费 raster 回放反馈；专属 `replay_garden` 文件已合并进统一 `replay_raster`。
10. **save 的 GC/备份有实现但未完全接入**：`snapshot_gc_thread` 解析/删除真实，仅测试启动；`start_snapshot_gc` 无生产调用；`perform_manual_backup` 身份校验仅 +0x53。
11. **wall replay 真实但边界收窄**：`advance_wall_groups_observed` + `advance_wall_transition` 实现 op0-2/Enclosure/AdjustHeight/并发组，未知 op 报错；`WallHistoryReplayAudio`/`WallHistoryRestoreColor` 事件无消费者。
12. **音频存在“注册了但资源不存在”的运行时风险**：`audio_update::AudioSettings`、`weather_audio::Weather`、`terrain_audio::EditingSpeed` 在本 crate 无插入点，而系统签名是 `Res<...>`（非 Option），一旦调度运行会缺资源 panic；需宿主注入（需人工确认宿主是否已做）。

## 6. 真实业务链闭合判定

| 业务链 | 判定 | 说明 |
|---|---|---|
| save→写盘（显式/自动保存） | **闭合** | `SaveGameCmd` producer（`meta_commander.rs:362`）→`save_game_system`→异步线程→ShadowFile/快照落盘；`startup/run_game.rs:182` 注册。缺口：cooldown 表值、手动备份身份、GC 启动。 |
| load→应用世界 | **部分** | 解析/版本/相机/时间/羊/颜色/History/编辑链应用真实；有效快照读回被拒绝；未知读档后域命令 bail；`ApplyCounter` 纯推进分支缺 producer。 |
| history undo/redo | **部分-闭合** | 命令/游标/桶/墙/raster/terrain/auto-clutter/stairs 闭合；decorator/颜色应用与 aux pop 未闭合；signifier replay 缺文件。 |
| replay（模式/速度/结束） | **闭合** | `replay_mode`/`finish_replay`/`ReplayBuffer` 真实并注册。 |
| ivy 生长 | **CPU 闭合 / GPU 未接线** | grower/spawner/pruner/leaf/animation 注册；上传函数未注册；ivy 音频空壳。 |
| terrain 编辑 | **CPU 闭合** | UI→编辑→重算→回读→历史真实；`OnTerrainChange` 无消费者，GPU 后置。 |
| garden 绘制/擦除/历史 | **闭合** | draw→raster→draw/erase/record；replay feedback 有消费者。 |
| audio 各域 | **消费闭合 / 生产部分缺失** | 音频系统与事件分派真实；多个上游事件在 crate 内无 producer；音乐主系统未接线。 |

## 7. 仍未复现/未闭合清单（≥12 条）

1. **快照读回未接线**：`systems/save/load_game.rs:267-269` 对可打开快照直接 `bail!`；`init_game_state_from_snapshot` 仅注释引用，无生产调用者。
2. **快照应用 4 域缺失**：`systems/save/snapshot.rs:1332-1333` Roofs/WallColors/GateColors/Trees 只 `log::trace!`；Walls 曲线/枚举打包待取证（L1392-1409）。
3. **快照 GC 未启动**：`systems/save/save_thread.rs:437`/`454` 的 GC 入口无生产调用者；打开存档流程未启动。
4. **历史 Pop 丢失 aux/碰撞恢复**：`systems/history.rs:532-536` 只注册 `pop_empty_replay_bucket`；`replay_pop_bucket_system`（L402）与 `history_clear_previous_state`（L451）未注册。
5. **颜色回放无消费者**：`systems/history/replay_color.rs:10-14` 只发 `ColorHistoryReplayApply`，crate 内无 `EventReader`。
6. **装饰回放无消费者**：`systems/history/replay_decorator.rs:10-13` 只发 `DecoratorHistoryReplayApply`，crate 内无 `EventReader`。
7. **墙回放音频/颜色无消费者**：`systems/history/replay_wall.rs:24-29` 定义 `WallHistoryReplayAudio`/`WallHistoryRestoreColor`，无 reader。
8. **楼梯回放音频载荷占位**：`systems/history/replay_stairs.rs:78-85` `PlayStairsAudioCmd` 全 zeros，decorator 被 `let _ =` 丢弃。
9. **signifier 回放缺失**：骨架 `signifier_replay_active.rs` 的 `signifier_replay_active`/`register_required_components`/`ease_out_elastic_jiggle` 在 crates 内无对应实现。
10. **路径 filter 未按类型恢复**：`DirtPathFilter`/`Cobblestone1/2/3PathFilter` 为 `no_trace`（`fn_audit`），当前统一用 `RasterDomain` + selector/flags；语义等价需人工。
11. **音乐主系统未接线**：`systems/audio/music/music_system.rs:54-63` `configure_music_consumers` 无调用者；`add_audio_systems` 只注册 `music_debug_gui`（`audio/mod.rs:254`）。`load_glade_audio.rs:22-24` 同样无调用者。
12. **ivy 音频空壳**：`systems/audio/ivy_audio.rs:21-23` 只日志，未实例化 `event:/SFX/IvyGrow`；`ivy_audio_system` 未注册。
13. **ivy GPU 上传未注册**：`systems/ivy/registration.rs:79-84` 明确 `待接线`；`upload_ivy_leaves`/`upload_ivy_segments` 无调度消费者。
14. **音频资源类型重复/缺生产者**：`audio_update.rs:31` 的 `AudioSettings` 与 `resources/audio_settings.rs:18` 重复；`weather_audio.rs:15` 的 `Weather` 与 `resources/weather.rs:7` 重复；两者 crate 内无插入点。`terrain_audio.rs:100` 的 `Res<EditingSpeed>` 无生产者。
15. **7 个音频事件无 producer**：`PlayBuildingAudio`、`PlayClutterSoundCmd`、`PlaySheepReactSound`、`PlayMenuUiSound`、`PlayTreeSoundCmd`、`PlayTimberframeSound`、`PlayElevationSupportSoundCmd`（crate 内 writer 数均为 0）。
16. **地面变化事件无消费者**：`systems/terrain/delayed_terrain_changed_msg.rs:32` 只写 `OnTerrainChange`，crate 内无 reader。
17. **建筑音频参数顶替**：`systems/audio/building_audio.rs:98-101、118-120` 用 `editing_speed` 代替 `BuildingSize=inverse_lerp(area)`，0x2a8 表未接线。
18. **手动备份身份不完整**：`systems/save/save_thread.rs:330-343` 只校验 +0x53 字节，`+0x299` 起结构待取证。
19. **墙回放未知变体**：`systems/history/replay_wall.rs:49-53` 变体 5..16 无 wall 字段；`wall_replay_transition.rs:61-63` 未知 op>2 返回错误（非恢复）。
20. **音乐花/叶概率语义待取证**：`systems/ivy/ivy_send_to_gpu_leaf.rs:126-131`。

## 8. 完成度估计与置信度

> 口径：按“可执行行为链闭合度”加权，不按函数名数量；不考虑 GPU/场景回放（项目已后置）。这是审计估计，不是语义验收。

| 目录 | 估计完成度 | 依据 | 置信度 |
|---|---:|---|---|
| systems/save | **76%** | codec/异步线程/ShadowFile/snapshot 捕获编码强；快照读回、4 域应用、GC 启动、备份身份缺。 | 中高 |
| systems/history | **68%** | undo/redo/墙/raster/terrain/stairs 主链真实；decorator/颜色消费者、aux pop、signifier、path filter、墙事件消费者缺。 | 中 |
| systems/audio | **72%** | 消费端与事件分派覆盖广；上游 producer、音乐主系统、重复资源类型、若干表/参数待接线。 | 中 |
| systems/ivy | **80%** | CPU 生长链完整；GPU 上传未注册、音频空壳、少数字段待取证。 | 中高（CPU） |
| systems/garden | **85%** | 绘制/记录/回放反馈均闭合；专属 replay 文件已合并，filter 精确性需人工。 | 中 |
| systems/terrain | **85%** | UI/编辑/历史/回放/回读 CPU 链闭合；`OnTerrainChange` 无消费者，GPU 后置。 | 高 |
| **范围内合计** | **约 75%**（区间 73-77%） | 按行数加权；主要扣分来自 save 快照读回、history 应用端、audio 生产/接线。 | 中 |

### 与 `STUBS-2026-10-19.md` 的差异

- `STUBS` 的“对得上”按名字/地址粗匹配，未区分**跨文件合并**；本报告确认 `history_process`、`history_finish_recording`、`replay_edit`、`replay_pop_bucket`、`replay_terrain`、ivy `add_*`、`add_history_edit_to_save_file` 均是搬移/改名，不是缺失。
- `STUBS` 称 `systems/save 58/67`、`history 25/41`、`audio 68/77`、`ivy 18/19`、`garden 2/2`、`terrain 11/11`；按本报告去重后，真缺失集中在 `signifier_replay_active` 与 path filter 类型，**名目缺口被高估，行为缺口被低估**（尤其快照读回、aux pop、音频 producer）。
- `STUBS` 的 “2452 tests passed” 未复跑；本次只审源码与接线，测试通过不等于语义验收。

## 9. 需人工确认

- 宿主是否在 country-core 之外插入 `audio_update::AudioSettings`、`weather_audio::Weather`、`EditingSpeed`；若没有，`add_audio_systems` 会在运行时因缺资源 panic。
- 宿主是否调用 `configure_music_consumers`/`configure_glade_audio`；本仓库未见调用者。
- `RasterDomain` + selector/flags 是否与 `Dirt/CobblestoneXPathFilter` 闭包逐像素等价。
- `replay_pop_bucket_system` 是否由上层 systems crate 注册；本 crate 的 `add_history_cpu_systems` 未注册。
- `snapshot` 4 域（Roofs/WallColors/GateColors/Trees）与 Walls 曲线区的原始打包布局。
