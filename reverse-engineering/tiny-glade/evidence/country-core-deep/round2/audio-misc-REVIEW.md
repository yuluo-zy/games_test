# audio-misc 域 round-2 逐文件审查（reconstruction/country-core/src vs DIGEST O 证据）

审查基准：`evidence/country-core-deep/audio-misc/DIGEST.md`（O = 机器码直接证据）+ wp14/wp15/wp16 合同 +
multi-baseline 两份音频精读。状态：**OK**=实现与证据一致（含合法 typed gap）；**FIX**=具体错误（列错误内容）；
**GAP**=缺失（列缺失项）。不修改源码（本文档为审查产出）。

## systems/audio/**

| file | 状态 | 具体错误 | 缺失项 |
|---|---|---|---|
| audio_update.rs | OK | — | 相机 3×3+偏移 → 听者变换的具体行列对应（U，相机域）；puffin scope 未建模 |
| building_audio.rs | OK（7 事件名/参数 O 一致） | — | `building_audio` 体空（kind 6/7/8 瞬发 + 尾部 0/1/2/3/4/9/5 循环参数表，multi-baseline §2 全有）；`get_building_size_01`/`set_parameters` 空 |
| clutter_audio.rs | GAP | — | `clutter_audio_system` 体空（39 kind 表 0x142aa1688 + Cosmetics 串表 0x142aa0e4c 全部 O 可落码） |
| decorator_audio.rs | FIX | 16 名单与 O 串表映射 I 且缺 `Cosmetics/Clutter_Rotate`、`Cosmetics/MoveGround`（合同串名 O）；`AudioLoopDesc::new(name, Some("editing_speed"), 3, 0x32)` 参数假设未证 | `decorator_audio_system` 体空（稳定排序 `(f32,Vec3)` 距离衰减，行 811 O） |
| elevation_supports_audio.rs | FIX | 只记 `WoodenBracketsAppear`；O 串表 3 个（+`BrickColumnAppear`/`WoodenColumnAppear`，DIGEST §2.13） | system 体空（kind→3 串分派） |
| firstperson_steps.rs | GAP | — | `firstperson_steps` 体空（GroundType::to_fmod_param、FirstPersonOff、双线性地形采样） |
| forest_audio.rs | OK（1 loop O） | — | `forest_audio_system` 体空 |
| game_loading_audio.rs | OK（snapshot:/Loading + 状态布局 O） | — | `loading_audio` 体空（event:/UI/Loading + 状态→2 停 snapshot + 1s 定时器，O） |
| garden_audio.rs | OK（4 事件名 O） | — | `garden_audio_system` 体空（update_draw_audio 模板 ×4）；`GapEventPlaceholder` 假类型 |
| ivy_audio.rs | GAP | — | `ivy_audio_init_system` 体空（instantiate_audio("event:/SFX/IvyGrow") O） |
| menu_audio.rs | OK（PauseMenu 快照 O） | — | `menu_audio` 体空（13 kind 跳表 0x142ac706c + AppMode `(mode\|8)==0xa` 门禁，O） |
| mod.rs | OK（16 事件注册链顺序 O 一致） | — | `add_audio_systems` 空（music_debug_gui + 两个 run_if 包）；`add_audio_startup_systems` 空 |
| path_audio.rs | FIX | init 只有 CobblestoneDraw×2（loop+snappy）；O 证据 5 事件：`CobblestoneDraw` + **`Path Draw`(0x142ac1660)** + **`Erase Draw`(0x142ac1690)** + `PathWaterDraw/PathWaterErase`（DIGEST §2.13） | `path_audio_system`/`update_draw_audio`/`brush_editing_speed_01` 空 |
| playermade_stairs_audio.rs | OK（6 事件名 O） | — | `stairs_audio_system` 体空（kind≤18 跳表 0x142bad9f0 全量 O，multi-baseline §3a） |
| replay_audio.rs | OK（Undo/Redo O） | — | `replay_audio_system` 体空（Undo/Redo/DenyInput + AppMode==0xd 全局参数） |
| roof_audio.rs | FIX | init 事件名 `{RoofAppear, RoofSwitchDirection}` 与 O 串不符：应为 **`RoofTipMove`(0x142adcb38)、`InterRoofMerlonsRearrange`(0x142adcb68)**（+RoofShape 正确，DIGEST §2.13） | `roof_audio_system` 空；`instantiate_audio_w_position` 签名错（O 4 参 `(core, name&str, pos)`，0x140aa3360 调用点 0x1408b7788 证据） |
| sheep_audio.rs | GAP | — | `sheep_audio_init_system` 体空（impl$0::new O）；`sheep_audio_system` 体空（kind 0-7 表 0x142ac2cc8 + Aww/Think/So/Angry/Idea/Eyes/Confused + 参数 FirstPersonOff；0x140909870 羊叫列表装配在本域外） |
| signifier_audio.rs | FIX | init 实例名不全：O 证据 1 loop + 4 实例 = `UI Boop`(0x142ac2465)/`Cursor/Grab`/`Cursor/Release`/`HoldTimeSiginfier`(原版拼写)/`UiRotateGizmoMovingBack`（DIGEST §2.13）；现有代码用 Grab 当 loop 且缺 3 名 | `signifier_audio_system` 体空（set_parameter ×21 + FirstPersonOn/Off、Tool 参数） |
| terrain_audio.rs | OK（4 事件名 I 映射与合同一致） | — | `terrain_audio_system` 体空；`EditStateAudio` 空 |
| terrain_stairs_audio.rs | GAP | — | 全部体空（StairDraw_OneShot + SSBO BufferState 名称断言 + inverse_lerp/60 音量，multi-baseline §3b O） |
| timberframe_audio.rs | GAP | — | 体空（TimberframeMove/Length/Width + RectangleAppear 等 kind 表 O） |
| tree_audio.rs | GAP | — | `tree_audio_system` 体空（最近树排序 + set_position/音量 O）；init 名 TreeMove/Tree/Amb 部分 |
| trigger_audio.rs | OK（4 事件名 O + 状态偏移） | — | `trigger_audio_system` 体空（audibility ±0.5/−0.1 夹取、音量位移/阈值×0.0125、z>=11/dist<=70 停 InForest，wp14 合同全量） |
| wall_audio_fmod.rs | FIX | `WallAudioState.loops: Vec<AudioLoopPlayer>` 模型与 O 布局冲突：O = **固定 10×AudioLoopPlayer stride 0x40**（+0/+0x40/…/+0x240）+ 参数 Vec @+0x48/0x88/0xc8/0x148 + 应播标志 @+0x38+0x40k + 音量 @+0x280；init 里 `let _ = core.instantiate_loop(&desc)` 丢弃结果（应装入 state） | `wall_audio_fmod` 体空（kind 0-23 全表 O，multi-baseline §1 + audio-extra §1） |
| wall_audio_orchestrator.rs | GAP | — | 体空（RequestWallMaterialAudio 协调：hashbrown/atomic_float，0x141234bf0 O） |
| wall_audio_path.rs | GAP | — | 体空（0x140a89bb0 O） |
| wall_audio_ui.rs | GAP | — | 体空（PlayWallUiSound kind 分派；本 round 新证：事件串表含 `ChangeWallStyle`、`Lantern/TurnOn`、`Chimney/ToggleSmokeOn|Off` 对、`BuildingSupportsRemoveWooden`、`WallSplit` 等，数据指针表 0x142a9f730 区） |
| wall_audio_utils.rs | FIX | `AudioLoopPlayer` 字段序：O = `{instances Vec cap-first {cap@0,ptr@8,len@0x10}, …@+0x18(参数 Vec 疑), current@+0x28, should_play@+0x38}`；现有代码 `current`/`should_play` 注解偏移对但 `resolve` 语义未证（O request_play 直接 `instances[current]` + 越界 panic） | — |
| water_ambience_audio.rs | FIX | init 只建 1 loop；O（audio-extra §3c）= **按水网格装配多个 AudioLoopPlayer**（from_iter<Map<Range,…>> over grid）+ `get_parameter_by_name` + `insert_resource<WaterAudioGrid>` | `water_ambience_audio_system` 体空（逐格 start/stop/set_position/音量；常量 0.015625/42.5）；`load_index_unchecked` 空（0x1409ebe80 O 名） |
| water_audio.rs | OK（4 事件名 + mode@+0x58 O） | — | `water_audio_system` 体空 |
| weather_audio.rs | GAP | — | 体空（Weather→全局参数，参数名 U） |
| music/music_player.rs | FIX | ① `current_track` 与 `stop_immediately` 对调（0x1408b72e0 实为 stop_immediately，DIGEST §7）；② `get_timeline_beat_properties(&self)->(i32,f32)` 错：O = 输入结构→写 6 进程全局的发布器（0x1408b7620）；③ `update` 空——状态机 O 结构见 CLOSURE §1（4 状态 + 节拍同步 + switch_track） | `load_tracks/switch_track/pause/probabilistic_switch_track/trigger_*_snapshot` 空；MusicTrack 布局 I（stride 0x30, [+0]==1, [+8] 事件） |
| music/music_system.rs | OK（richness 结构） | — | `music_audio_system`/`update_music_richness_based_on_time` 空（richness 三档 snap-off 0.24/0.49/0.74 公式 multi-baseline §4 O） |
| music/music_debug_gui.rs | GAP | — | 体空（egui 调试窗，读 [+0x100] 开关与 richness@+0x1c0，O） |

## resources/

| file | 状态 | 具体错误 | 缺失项 |
|---|---|---|---|
| audio_resources.rs | FIX | ① Active 变体：event = 8B 句柄 @+8（现有 `_event: Gap` 位置含糊）；name String 为 **cap-first** {cap@+0x10, ptr@+0x18, len@+0x20}（现有注释只说 `state @+0x28`）；② `Bank::load` 失败 = **panic**（"Error loading FMOD bank file: …"，现有代码只 set loaded 无 panic 路径）；③ `AudioSamplesLoadedPromise::recv` = 三通道 recv_timeout(1s)+unwrap（现有 recv 空体）；④ `set_parameter`/`set_position` 失败后的**全局报告注册表**（0x143634d60/d80 族 + "Mutex was poisonned"）完全未建模；⑤ `start_if_not_playing` 位掩码 0xb 未记 | `AudioCoreInner::update` 体（听者三断言 + set_listener_attributes + Studio::update + "FMOD update error: " 日志）未落码 |
| audio_settings.rs | OK | — | @0..0xf8 字段 U（音量 4 字段 O） |
| color_icon.rs | FIX | `ColorMenuBrightnessValue {Default, Custom(f32)}` 错：O = 三变体无载荷，判别→{0.25, 1.0, 4.0}（表 0x142ae785c），default=变体1；`to_f32`/`default_f32` 实现随之错 | 15 个图标组件为 bevy 注册胶水（TypeId 查表样板），不需逐个体 |
| color_id.rs | FIX | `default_value=ColorId(0)` 引 0x14099ca50 错（=1，归属 color_id_roof）；`is_default()==0` 的过滤语义需以屋顶侧重审 | — |
| color_id_roof.rs | FIX | `RoofColorIdSsbo(Gap)` 未承载 `default_value()=1`（O 0x14099ca50） | — |
| gpu_lut.rs | GAP | — | `build`（2 条目/demo 门禁 0x143634d90 + vtable 0x142ad6260/0x142ad6290）、`render_luts`（+0x28 标志循环）、`compute_impl` 全部 unimplemented!（CLOSURE/DIGEST §2.10 全量算法可落码） |
| grading_lut.rs | GAP | — | `load_cube`（完整 .cube 解析器）、`parse_rgb`、`GradingLuts::load`（"assets/luts" 目录 + 双 Vec）unimplemented!（DIGEST §2.9 全量）；`load_unwrapped`（0x140b00bc0，路径前缀 "assets/luts/"，O 名）缺 |
| image_formation.rs | OK（机器码逐条一致，含缺省档=1） | — | — |
| placement_data.rs | OK（to_affine3a 忠实转录，含 w 行=pos.z 原版怪癖） | 建议加注释说明 w 行原版怪癖（现无） | — |
| texture_presets.rs | OK | — | — |
| render_ssbo.rs | OK（库侧 LanternSsbo 等 O） | — | — |
| shader_refactoring_state.rs | FIX | `disabled()` 建模为构造器；O = `fn disabled(&self)->bool`（`[+0]==0`，0x140b1f210） | — |
| window.rs | FIX | `WindowSize{width,height: u32}` 错：O `as_vec2`（0x1400885d0）= 头 8 字节直接当 Vec2（f32×2）；`aspect_ratio` 缺失；0x140949c40（PDB aspect_ratio）= 静态变量析构例程（见 CLOSURE §5）未记 | — |
| window_lights.rs | OK（合同级 typed gap） | — | — |
| feedback_report_resources.rs | FIX | ① `FeedbackReportBgTexture(pub bool)` 独立资源错：O `is_available` = `[+0x36] < 7`（状态字节，宿主=资源本体）；② `FeedbackReportTags {tags:&'static [&str]}` 错：0x140909870 实为羊叫 4 标志装配（DIGEST §7）；③ 布局缺 0x10..0x38 截图元数据区（见 CLOSURE §4） | `feedback_report_texture_receive` 空（事件→换纹理 Arc→拷元数据→状态 7/8 机转，O） |
| gothic_window_profile.rs | OK（WP21 点列转录） | — | panic 串（"gothic half dormer profile size"）未记 |
| glade_color_customization.rs | OK（默认字节 O） | — | — |
| shutdown.rs | FIX | `initiate_game_shutdown(&mut Vec, &mut Vec)` 签名错：O = bevy 系统（EventReader 双缓冲 + Commands，0x140b2ba40） | — |
| steam.rs（resources） | OK（analog/digital 状态结构，I 布局） | — | `is_any_active` 未证 |

## src/ 顶层与 systems/steam、systems/debug

| file | 状态 | 具体错误 | 缺失项 |
|---|---|---|---|
| app_loading_state.rs | OK（typed gap，变体 U 已注明） | — | — |
| frozen_map.rs | OK（契约复刻：insert 返回已有值引用/借用字节/Box 稳定） | — | 库本体不重写（正确策略） |
| meta_commander.rs | OK（80B 布局 + 标签分派 O） | `is_profiling` 的 `[+0x38]` 字段语义未注（U） | — |
| name_hash.rs | OK（xxh3_64 默认路径 = O 0x140c9cd80） | — | — |
| audio_events.rs | GAP | PlayStairsAudioCmd/PlayWallAudio 用 Gap 凑布局：O 偏移全有（@+0xc..0x2c f32 族/@+0x1c u64+u32 位置/@+0x20 byte）但字段名缺失 | RequestWallMaterialAudio `raw: Gap` |
| systems/steam/mod.rs | FIX | ① `AppSteamIntegration{client, user_id:u64}` 存字段模型错：O `user_id()` 每次现算 `client.user().steam_id()`（0x140ae96f0）；② `is_floating_gamepad_text_input_open` 模型错：O = 进程全局字节 0x143634d94；③ `add_steam_integration` 空（8 系统批量注册 O） | `update_steam_inputs` 空（14 动作块归纳见 CLOSURE §2）；`SteamOverlayOpenResources` 方法体空（open_overlay_or_browser O 0x14026d3c0） |
| systems/steam/startup_steam.rs | GAP | — | 体空（首帧一次性 Commands 插入资源束，O 0x140a9bbf0） |
| systems/debug/human_scale_ref.rs | GAP | — | `human_scale_ref` 空（5 参系统 + [+0xf8] 开关 + [+0x110] 模式跳表 0x142a9f3b0，O） |
| systems/debug/sheep_controls.rs | GAP | — | `sheep_controls`/`apply_sheep_snapshot` 空（0x1416f11a0/0x1416f24f0 O 名）；PickingMode 变体 U |
| systems/debug/lazy_grid_debug.rs | GAP | — | 全内联（无独立过程）——占位合理 |

## 汇总

- OK：18 文件；FIX：17 文件；GAP（仅缺体）：~18 文件。
- 最高优先 FIX：roof/path/signifier 事件名、WallAudioState 固定数组布局、color_icon 亮度枚举、
  color_id 默认值、shader_refactoring_state::disabled、window WindowSize、shutdown 签名、
  steam user_id/is_floating、feedback 三处。
- 最高优先 GAP：grading_lut::load_cube 全量、gpu_lut 三函数、music_player 状态机、wall_audio_fmod
  kind 表、stairs kind 表、menu kind 表、clutter kind 表（证据全部现成，纯落码工作）。
