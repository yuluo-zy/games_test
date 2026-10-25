# WP14 · country-core audio domain — contract 2: audio systems & FMOD glue

范围：`systems/audio/*`（31 模块）+ `systems/wall/snapping/snapping_audio` + `systems/audio/music/*`。
版本 B。核心方法同 `evidence/multi-baseline/audio-events-analysis.md`（本 WP 复用了其
wall/building/stairs/music 精读；其余由本 WP 反编译补充）。

## 0. 事件→FMOD 总模式（O，适用于每个事件系统）

1. bevy 事件双缓冲读取（槽 @+8 起 payload；`PlayBuildingAudio` 槽 @+0 即 payload）。
2. 按 `kind` 字节分派（.data 32 位相对跳表）。
3. FMOD 构造：kind→(事件名串 / 循环 player) → `instantiate_audio(name)` /
   `AudioCore::instantiate_loop(desc)` / `AudioLoopPlayer::request_play` /
   `instantiate_audio_w_position_and_volume` → `set_position` → `set_parameter(name,val)` /
   `start_if_not_playing` / `set_volume`。
4. 帧尾：按"应播"标志对循环 `start_if_not_playing` / `stop`，清标志。

## 1. mod.rs（O）

- `add_audio_events(app:&mut App)`（0x140941450，行 179–196）：16×`register_event`，顺序：
  `PlayTerrainSound`、`SuppressPathSound`、`PlayReplayAudio`、`PlayDecoratorSoundCmd`、
  `PlayStairsAudioCmd`、`PlayWallUiSound`、`PlayWallAudio`、`RequestWallMaterialAudio`、
  `WallBendAudioParamReset`、`PlayTimberframeSound`、`PlaySheepReactSound`、`PlayMenuUiSound`、
  `PlayBuildingAudio`、`PlayElevationSupportSoundCmd`、`PlayClutterSoundCmd`、`PlayTreeSoundCmd`。
- `add_audio_systems(app:&mut App)`（0x1409411a0，行 95–142）：`music_debug_gui` +
  `water_audio_system`（run_if：`Res<AppMode> + ResMut<WaterAudioState> +
  EventReader<WaterDrawEvent> + EventReader<WaterEraseEvent> + Res<CursorTerrainRaycast> +
  Res<ActiveSession>`）+ `water_ambience_audio_system`（run_if：`ResMut<WaterAmbienceAudio> +
  Res<WaterAudioGrid> + Res<MainCamera> + Res<UiSignifierStream>`）+ 0x26 SystemSet 归属。
- `add_audio_startup_systems`（内联入 `add_game_startup_systems` 行 144）：启动期系统
  （`load_audio`、`boot_audio_update_system` 等）。

## 2. audio_update.rs（O）

- `audio_update_system`（0x1409c8dd0，行 11–34）：`Res<AudioCore> + Res<相机变换资源> +
  Res<AudioSettings>`。每帧：(1) 5×`set_vca_volume`（AudioCoreInner+0x10）——
  `vca:/Master←settings@+0xf8`、`vca:/Music←+0xfc`、`vca:/Music-DawnJingle←+0xfc`、
  `vca:/SFX←+0x100`、`vca:/Amb←+0x104`；(2) 从相机资源 3×3+偏移变换算 3D 听者
  （position/forward/up）→ `AudioCore::update`。
- `boot_audio_update_system`（0x1409c9320，行 37–54）：同 VCA 段；听者用固定默认位姿
  （`[0]=0,[1]=-1,[2]=0, forward=-1`，I），不读相机。
- 帧内 `puffin` scope（串 `audio_update_system::f`）。

## 3. wall_audio（O，audio-events-analysis §3）

- `wall_audio_init_system`（0x14094fb80，行 40–94）：10×`AudioCore::instantiate_loop`（desc
  {name,count=3,max=50}）：`event:/SFX/Wall Draw /Wall Change Height /FenceExtend /
  LintelExtend /StoneArchExtend /WideningGate /PlatformStairsExtend /SewerGateWiden /
  FenceHeight /FenceMove`；`insert_resource<WallAudioState>`。
- `wall_audio_fmod`（0x140950920，行 134–…，主事件系统）：arg1=AppMode（==13 时音量 1.0），
  arg2=Res<AudioCore>，arg3=EventReader<PlayWallAudio>，arg4=ResMut<WallAudioState>，音量源栈参
  （+0x1c f32 / const，divss）。kind 跳表 0-23：
  - A 循环类（draw/move/height/width/extend/fence）：`AudioLoopPlayer::request_play(state+off,
    pos, editing_speed param, vol)`（0x1409285a0：`+0x38=1`、`set_position`、
    `set_parameter("editing_speed",0xd)`、`set_volume`）。
  - B 瞬发类（FenceAppear/Merge/Rearrange、LintelAppear/HasBricks/Merge、StoneArchAppear、
    Pillar{Stone,Wood}Appear、GateAppear、PlatformStairsAppear、WallSplit、SewerGateAppear）：
    `instantiate_audio_w_position_and_volume`（0x140aa3590：instantiate → set_position →
    start_if_not_playing → set_volume）。
  - C 参数类：`set_parameter(elem, "FenceLength", inverse_lerp(...))`。
  帧尾：10 loop 依应播标志 start/stop；`[state+0x280]` 音量 × powf 时间调制。
- `wall_audio_utils::AudioLoopPlayer`：`new`（行 46）、`resolve`（行 32）、`request_play`（行 95）。
- `wall_audio_orchestrator`（0x141234bf0）：`RequestWallMaterialAudio` 协调（hashbrown/atomic_float）。
- `wall_audio_path`（0x140a89bb0）、`wall_audio_ui`（0x1408b7690，`PlayWallUiSound`：
  `event:/UI/…` + `event:/SFX/Color{Bricks,Gate,Halftimber,Window}`、`ChangeWallStyle`、`WindowDrag`）。
- `snapping_audio`（0x140a6a480，`WallSnapToSegment` 瞬发）。

## 4. building / stairs（O，audio-events-analysis §4–5）

- `building_audio_init_system`（0x1417d21a0）：7×instantiate_loop：`event:/SFX/Building/
  BuildingResize /BuildingRotateRectangle /BuildingRotateCircle /BuildingChangeHeight /
  BuildingMove` + `event:/SFX/PlasterChangeLoop` + `BuildingOverlapMove`；参数 `editing_speed`、
  `BuildingSize`。`insert_resource<BuildingAudioState>`。
- `building_audio`（0x1417d2ad0，行 101–…）：`ResMut<BuildingAudioState> +
  EventReader<PlayBuildingAudio> + Res<AudioCore> + Res<Building 数据表>`。`PlayBuildingAudio`
  槽 stride 0x20：`id u64@0（hash→0x2a8 条目）、position Vec3@8、editing_speed f32@+0x14、
  kind u8@+0x18`。kind 0–7：瞬发串 `event:/SFX/Shapes/DetachFromGround /AttachToGround /
  PeakReached`（kind 6–7 常量 0x6/0xf 与 0xf/0xc，U 语义）；序列：instantiate_audio →
  set_position → set_parameter("editing_speed")+set_parameter("BuildingSize",
  inverse_lerp(area)) → start_if_not_playing。
- `stairs_audio_init_system`（0x140f878d0）：6×instantiate_loop：`event:/SFX/Stairs/StairMove /
  StairChangeHeight /StairChangeWidth /LadderMove /LadderChangeHeight /LadderChangeWidth`。
- `stairs_audio_system`（0x140f880a0，行 140–318）：帧首 6 loop stop；`PlayStairsAudioCmd`
  槽 stride 0x38（kind@+8、f32@+c/+0x10/+0x14/+0x18、pos u64@+0x1c、z u32@+0x24、f32@+0x28/+0x2c）；
  kind≤18 跳表。瞬发串：`StairPlace /StairDelete /StairsMerge /StairsMergeConfirm /
  LadderAppear /LadderPlace /StepsAppear /WalkwayAppear /StairSupportAppear /StairPillarAppear`
  （`event:/SFX/Stairs/` 前缀，O）；参数串表：`stair_width size editing_speed extend_amount
  h_offset avg_arch_supports height_above_ground in_water stair_edge_length is_stub_preview
  move_all`（O）。

## 5. 其余事件系统（本 WP 反编译补充；串名 RNE .data O）

- `terrain_audio_init_system`（0x1414613e0）：4×instantiate_loop（`event:/SFX/TerrainEditShape
  /TerrainEditHeight /TerrainCreateChunk /TerrainMove`，串名与顺序 I）；`TerrainAudioState`。
  `terrain_audio_system`（0x1414618c0）：`PlayTerrainSound` 事件 + 地形编辑栅格 → 计算网格索引
  → kind 跳表 → 对 4 loop 依应播标志 `start_if_not_playing + set_parameter + set_position`
  或 `stop`。
- `tree_audio_init_system`（0x1414633e0）：4×instantiate_loop（含 `event:/SFX/Tree/Amb` 循环 +
  `TreeMove/TreePlace/TreeGizmo{Hover,Release}` 瞬发）。`tree_audio_system`（0x1414638e0）：
  `EventReader<PlayTreeSoundCmd>`（wp10 tree-tool 消费）+ `Query<&TreeAmbience>` 最近距离排序
  （`(dist, pos)` 稳定排序）→ 最近树音频 `set_position`/音量。
- `forest_audio_init_system`（0x140aa18f0）：1×instantiate_loop（`event:/SFX/ForestGrow`）。
  `forest_audio_system`（0x140aa1f00）：森林生长循环按事件驱动。
- `garden_audio_init_system`（0x140a88fa0）：4×instantiate_loop（`event:/SFX/Garden/GardenDraw
  /GardenDrawWater /GardenErase /GardenEraseWater`）。`garden_audio_system`（0x140a89480）：
  `EventReader<PlayGarden…>` → `path_audio::update_draw_audio` 模板。
- `path_audio_init_system`（0x1409d08d0）：`instantiate_loop`（`event:/SFX/CobblestoneDraw`）+
  `instantiate_loop_snappy`（行 65）。`path_audio_system`（0x1409d0f80）：`update_draw_audio`
  （0x14166d890/0x14166d9f0 两模板实例，行 69）+ `brush_editing_speed_01`（行 234，O，共享辅助：
  water/garden/terrain/tree/wall 均内联）。
- `decorator_audio_init_system`（0x14145c6b0）：16×instantiate_loop（`event:/SFX/WindowDrag /
  WindowPlace /WindowPlacePreview /WindowRemove /WindowCombine /WindowHover /
  DecoratorMoveWoodenFloorTEMP /Lantern/MoveWall /Lantern/MoveGround /Lantern/Rotate /
  Lantern/Amb /ChimneyChangeHeight /ChimneyDrag /Flags/Rotate /Flags/Drag /Flags/DragGround
  /Cosmetics/Clutter_Rotate /Cosmetics/MoveGround`，串名 O，映射 I）。`decorator_audio_system`
  （0x14145dc20，行 811 稳定排序 `(f32,Vec3)`）：`PlayDecoratorSoundCmd` 按距离/音量衰减选择。
- `clutter_audio_system`（0x1408c02f0）：`PlayClutterSoundCmd` → `instantiate_audio` +
  set_parameter + set_position + start_if_not_playing（`event:/SFX/Cosmetics/Clutter_Detach /
  Clutter_Pickup /Clutter_Rotate /MoveBuilding /MoveGround`，O）。
- `signifier_audio_init_system`（0x1409d6320）：1×instantiate_loop + 4×instantiate_audio
  （`event:/Cursor/Grab /Cursor/Release` 等）。`signifier_audio_system`（0x1409d6b90，行 82/268）：
  `UiSignifierStream` → 20+ 参数更新（`set_parameter` ×21）。
- `menu_audio_init_system`（0x140a0b340）：1×instantiate_audio（`snapshot:/PauseMenu`）。
  `menu_audio`（0x140a0b4c0，行 47 分派）：`PlayMenuUiSound` → 13 个 UI 串：
  `event:/UI/Menu/{Hover,Select,Settings,SettingsTab,SliderTick,ShowSessions,NewSession,
  StartSession,Back,Open,Close}` + `event:/UI/Ui2dHover /Ui2dLockedClickNew`（O）。
- `replay_audio_init_system`（0x140ad54d0）：2×instantiate_audio（`event:/UI/Undo /UI/Redo`）。
  `replay_audio_system`（0x140ad5800）：`PlayReplayAudio` 栅格 → kind 0/1 播放 Undo/Redo、
  2 播放 `event:/UI/DenyInput`（瞬发）；AppMode==0xd 时设全局参数。
- `game_loading_audio`：`init_game_loading_audio`（0x140b1ae70）创建并立即 start
  `snapshot:/Loading`；`loading_audio`（0x140b1b1b0）：加载进行时播放 `event:/UI/Loading`，
  完成后停止 snapshot（状态→2、清标志），1s 定时器二次保护停止。
- `water_audio_init_system`（0x140873140）：4×instantiate_loop（`event:/SFX/Water/WaterDraw /
  WaterErase /IceDraw /IceErase /PathWaterDraw /PathWaterErase`，串名 O，映射 I）。
  `water_audio_system`（0x140873630）：wp7 `WaterRaster` 栅格 + `update_draw_audio`（garden 模板）
  处理绘制/擦除 loop；`mode u8@+0x58`。
- `water_ambience_audio_init_system`（0x1409ebf20，行 49）：`AudioLoopDesc::new` +
  `instantiate_loop`（`event:/SFX/Water/Amb`）。`water_ambience_audio_system`（0x1409ecea0）：
  演示模式禁用（`_DEV_OVERRIDE_IS_DEMO_MODE`）；逐 `WaterAudioGrid` 格：有水面→
  `start_if_not_playing + set_position（相机相对）+ set_volume`，无→`stop`。
- `firstperson_steps_init_system`（行 29，`(Res<AudioCore>, Commands)`）：instantiate_loop
  （`event:/SFX/Player/Walking`）。`firstperson_steps`（0x140a889b0，行 128 `GroundType::
  to_fmod_param`）：全局参数 `FirstPersonOff`（O 串）+ 地面类型参数；`GroundType==3` 且相机运动
  时双线性采样地形栅格定材质；`GroundType==0` 时播放行走 loop 否则停止。
- `sheep_audio_init_system`（行 26，`(Res<AudioCore>, Commands)`）：`impl$0::new`。
  `sheep_audio_system`（0x1409d5ea0）：`PlaySheepReactSound` → `event:/SFX/Sheep/{Idle,Eyes,
  Confused,Pet,Aw,Baah,So,UmbrellaClose,UmbrellaOpen}`（O）+ 参数 `FirstPersonOff`；4×
  instantiate_audio + 3 AudioLoop。
- `timberframe_audio`（0x140a67b90）：`PlayTimberframeSound` → `event:/SFX/Timberframe{Move,
  Length,Width,Appear,Disappear,RectangleAppear}`（O）+ editing_speed。
- `elevation_supports_audio`（0x140ad51c0）：`PlayElevationSupportSoundCmd` → instantiate_audio
  （`event:/SFX/Shapes/WoodenBracketsAppear`，I）。
- `ivy_audio_init_system`（0x140ad5350）：instantiate_audio（`event:/SFX/IvyGrow`）。
- `weather_audio_system`（行 5，`(Res<AudioCore>, Res<Weather>)`，全内联）：从 `Weather` 状态
  `AudioCore::set_parameter`（全局）驱动季节/天气参数；参数名 U（gap，见 evidence）。

## 6. music 系统（O，audio-events-analysis §6）

- `music_audio_init_system`（0x140a71f20）：`GladeAudio::default` → 3×instantiate_audio
  （`event:/Music/mus_GladePreview`×2 + `event:/Amb/GladePreview`）→ `start` →
  `MusicPlayer::init` → `insert_resource<MusicAudioState>`；参数串：`MusicRichness、
  NewGladeTime、PreviewGladeType`（O）。
- `music_audio_system`（0x140a72810）：richness = clamp(time/const)，`time_continuous_lerp`
  平滑；`set_parameter("MusicRichness")` + `set_parameter("NewGladeTime")`；AppMode `|8==0xa`
  时 `MusicPlayer::update`。

## 7. Typed gaps（U）

- 各 kind→串 的逐条精确映射（跳表 case 对应；部分已录）；
- `wall_audio_fmod` 全量 8 参签名与音量源类型；`stairs_audio_system` arg3 元素表类型；
- `weather_audio_system` 参数名；`firstperson_steps` 地面类型参数名与 GroundType 枚举变体；
- `PlayGarden…/PlayPath…/PlaySignifier…` 等未列事件类型名（U，事件槽证据缺）。
