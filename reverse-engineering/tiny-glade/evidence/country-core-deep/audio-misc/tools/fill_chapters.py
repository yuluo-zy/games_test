# -*- coding: utf-8 -*-
"""Round-3 chapter filler for audio-misc domain.
Reads each chapter skeleton, fills 文件作用 + per-function 作用/逆向结论.
Only rewrites chapters/ files (no reconstruction source)."""
import re, os, sys

ROOT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"

# file -> (purpose, {short_fn_name: (role, conclusion)})
DATA = {
"systems/audio/audio_update.rs.md": (
 "每帧/启动期音频更新：把 AudioSettings 音量推给 5 个 FMOD VCA，并把相机（或默认）位姿作为 3D 听者推给 Studio。",
 {"audio_update_system": ("5×set_vca_volume(Master@+0xf8/Music@+0xfc/Music-DawnJingle/SFX@+0x100/Amb@+0x104) + 相机资源(+0xc0 3×3+0xf0 平移)算听者 + AudioCore::update；puffin scope", "O"),
  "boot_audio_update_system": ("同 VCA 段；听者用固定默认位姿（不读相机）；启动期系统", "O")}),
"systems/audio/building_audio.rs.md": (
 "建筑编辑音效：PlayBuildingAudio 事件（瞬时 detach/attach/peek）与每帧 7 个循环参数的 FMOD 驱动。",
 {"building_audio_init_system": ("7×instantiate_loop：BuildingResize/RotateRectangle/RotateCircle/ChangeHeight/Move/PlasterChangeLoop/BuildingOverlapMove；参数 editing_speed", "O"),
  "building_audio": ("事件 kind6/7/8→DetachFromGround/AttachToGround/PeakReached 瞬发+set_parameter(editing_speed,BuildingSize=inverse_lerp(area))；帧尾 7 kind(0,1,2,3,4,9,5)循环参数 ifWindow/BuildingHeight/isHalfTimbered/IsOnGround/loop", "O"),
  "get_building_size_01": ("构建面积→[0,1]（BuildingSize 参数源）", "I（内联，名推断）")}),
"systems/audio/clutter_audio.rs.md": (
 "杂物摆放音效：PlayClutterSoundCmd kind(0..0x26=38)→event:/SFX/Cosmetics/* 瞬时播放。",
 {"clutter_audio_system": ("kind 表 0x142aa1688(39 项)→instantiate_audio+set_position+set_parameter+start_if_not_playing（串表 0x142aa0e4c：BirdsNest/Box/DoorBell/BoxFish/RackBrooms…）", "O")}),
"systems/audio/decorator_audio.rs.md": (
 "装饰物音效：16 个循环 loop（窗/灯/烟囱/旗帜）+ 按距离衰减的瞬时播放。",
 {"decorator_audio_init_system": ("16×instantiate_loop：WindowDrag/Place/PlacePreview/Remove/Combine/Hover、DecoratorMoveWoodenFloorTEMP、Lantern/MoveWall/MoveGround/Rotate/Amb、ChimneyChangeHeight/Drag、Flags/Rotate/Drag/DragGround", "O"),
  "decorator_audio_system": ("事件→(f32,Vec3) 稳定排序距离衰减→循环/瞬发分派", "O")}),
"systems/audio/elevation_supports_audio.rs.md": (
 "抬升支撑（木架/柱）出现音效。",
 {"elevation_supports_audio": ("kind→3 串 WoodenBracketsAppear/BrickColumnAppear/WoodenColumnAppear→instantiate_audio+start", "O")}),
"systems/audio/firstperson_steps.rs.md": (
 "第一人称脚步声：GroundType→FMOD 参数 + FirstPersonOff 全局参数。",
 {"firstperson_steps": ("GroundType::to_fmod_param(行128)+全局参数 FirstPersonOff；GroundType==3 且相机运动时双线性采样地形栅格定材质；GroundType==0 播放行走 loop 否则停", "O")}),
"systems/audio/forest_audio.rs.md": (
 "森林生长音效（1 个循环）。",
 {"forest_audio_init_system": ("1×instantiate_loop(event:/SFX/ForestGrow)", "O"),
  "forest_audio_system": ("森林生长事件驱动循环 start/stop/set_position", "O")}),
"systems/audio/game_loading_audio.rs.md": (
 "加载界面音频：snapshot:/Loading 快照 + event:/UI/Loading。",
 {"init_game_loading_audio": ("创建并立即 start snapshot:/Loading；状态装填", "O"),
  "loading_audio": ("加载进行时播 event:/UI/Loading；完成→停快照(状态→2、清标志)+1s 定时器二次保护", "O")}),
"systems/audio/garden_audio.rs.md": (
 "花园绘制/擦除音效（4 循环 + update_draw_audio 模板）。",
 {"garden_audio_init_system": ("4×instantiate_loop：GardenDraw/GardenDrawWater/GardenErase/GardenEraseWater", "O"),
  "garden_audio_system": ("事件栅格索引→update_draw_audio 模板×4（path_audio 共享辅助）", "O")}),
"systems/audio/ivy_audio.rs.md": (
 "常春藤生长音效（单瞬发）。",
 {"ivy_audio_init_system": ("instantiate_audio(event:/SFX/IvyGrow)", "O")}),
"systems/audio/menu_audio.rs.md": (
 "菜单 UI 音效：PauseMenu 快照 + 13 个 UI 事件。",
 {"menu_audio_init_system": ("instantiate_audio(snapshot:/PauseMenu)", "O"),
  "menu_audio": ("AppMode (mode|8)==0xa 门禁停/播快照；PlayMenuUiSound kind 表 0x142ac706c(0..12)→Menu/Hover、Ui2dHover、Menu/Select、Menu/SettingsTab、Menu/NewSession、Menu/StartSession、Menu/Back、Menu/Close、Ui2dLockedClickNew 等", "O")}),
"systems/audio/mod.rs.md": (
 "音频模块注册：16 个事件类型 + 运行期/启动期系统。",
 {"add_audio_systems": ("music_debug_gui + water_audio_system/water_ambience_audio_system 两个 run_if 包（0x26 SystemSet 归属）", "O"),
  "add_audio_events": ("16×register_event：PlayTerrainSound/SuppressPathSound/PlayReplayAudio/PlayDecoratorSoundCmd/PlayStairsAudioCmd/PlayWallUiSound/PlayWallAudio/RequestWallMaterialAudio/WallBendAudioParamReset/PlayTimberframeSound/PlaySheepReactSound/PlayMenuUiSound/PlayBuildingAudio/PlayElevationSupportSoundCmd/PlayClutterSoundCmd/PlayTreeSoundCmd（PDB 名 add_audio_startup_systems 错位）", "O")}),
"systems/audio/music/music_debug_gui.rs.md": (
 "音乐调试 egui 面板（开发用）。",
 {"music_debug_gui": ("(Res<EguiContextResource>,ResMut<MusicAudioState>,ResMut<DebugGui>)：读 [+0x100] 开关字节与 MusicPlayer+0x1c0 richness 渲染调试窗", "O")}),
"systems/audio/music/music_player.rs.md": (
 "音乐播放器：曲目池 + 4 状态机（含节拍同步）+ 快照触发。",
 {"update": ("状态机（state@+0x118∈0..4，双跳表 0x142a9fa24/A 0x142a9fa34/B 各 4 有效条目）：每 case time_acc@+0x11c+=dt；A1 超阈值停主事件；A2 Duration@+0x120(round-to-even)超时→推进；A3 节拍同步(0x143634c68==1 且 0x143634c90&3==0→state=4+start)；A4 超阈值 fadeout 停/超阈值3→state=0；B 表=回卷路径；state==0→switch_track；公共尾 get_paused→恢复/切；每帧清零 0x143634c90", "O"),
  "init": ("3×instantiate_audio(mus_GladePreview×2+Amb/GladePreview)→start", "O"),
  "load_tracks": ("曲库装载（Halloween/Tiny,Single,Forgotten,Enchanted 串 O）", "I"),
  "switch_track": ("切曲（track 下标推进+start/stop）", "I"),
  "current_track": ("当前曲目（PDB 名挂在 0x1408b72e0 上但该址实为 stop_immediately；真实地址 U）", "U"),
  "stop_immediately": ("0x1408b72e0（PDB 名 current_track 错位）：遍历全部曲目[+0]==1→stop(1=ALLOWFADEOUT)+主事件[+0x10]==1→stop(1)；[+0]=0", "O"),
  "pause": ("set_paused(true)（曲目+主事件）", "I"),
  "probabilistic_switch_track": ("fastrand 随机切曲（generate_track_playlist O 名）", "I"),
  "get_timeline_beat_properties": ("0x1408b7620：非 getter——把输入结构{i32,i32,f32,i32,i32}写入 6 个进程全局 0x143634c68/c70(f32)/c78/c80/c88/c90（c88/c90 running max；c90=update 每帧清零的节拍计数）", "O"),
  "stop_photomode_snapshots": ("停 PhotoMode 快照（snapshot:/PhotoMode O）", "I（内联，名推断）"),
  "trigger_fpv_snapshot": ("触发 snapshot:/FirstPerson（O 串）", "I（内联，名推断）"),
  "trigger_photomode_minus_fpv_snapshot": ("触发 PhotoMode 快照（非 FPV 态）", "I（内联，名推断）"),
  "generate_track_playlist": ("fastrand 生成随机曲目顺序", "I（内联，名推断）")}),
"systems/audio/music/music_system.rs.md": (
 "音乐系统：richness/NewGladeTime 全局参数 + 播放器驱动。",
 {"music_audio_init_system": ("GladeAudio::default→3×instantiate_audio(mus_GladePreview×2+Amb/GladePreview)→start→MusicPlayer::init→insert_resource<MusicAudioState>；参数串 MusicRichness/NewGladeTime/PreviewGladeType", "O"),
  "music_audio_system": ("richness=clamp(time/120)三档 snap-off(0.24/0.49/0.74)+time_continuous_lerp 平滑(@+0x1c0/0x1c4)；set_parameter(MusicRichness/NewGladeTime)；AppMode (mode|8)==0xa→MusicPlayer::update 否则 pause", "O"),
  "update_music_richness_based_on_time": ("accum=clamp(State+0x1c4+time/120.0,0,1)+三档 snap-off→平滑目标（内联进 music_audio_system）", "O（内联）")}),
"systems/audio/path_audio.rs.md": (
 "小路/卵石绘制音效（CobblestoneDraw 循环 + 4 个 snappy 循环）。",
 {"path_audio_init_system": ("instantiate_loop(CobblestoneDraw)+instantiate_loop_snappy×4：Path Draw(0x142ac1660)/Erase Draw(0x142ac1690)/PathWaterDraw/PathWaterErase", "O"),
  "path_audio_system": ("事件栅格→update_draw_audio 模板（行 69）", "O"),
  "brush_editing_speed_01": ("画笔编辑速度→[0,1]（water/garden/terrain/tree/wall 共享内联辅助）", "I（内联）"),
  "update_draw_audio": ("按编辑栅格索引驱动一个绘制 loop：应播标志→start_if_not_playing/stop", "I（内联模板）")}),
"systems/audio/playermade_stairs_audio.rs.md": (
 "玩家自制楼梯音效（6 循环 + kind≤18 分派）。",
 {"stairs_audio_init_system": ("6×instantiate_loop：StairMove/StairChangeHeight/StairChangeWidth/LadderMove/LadderChangeHeight/LadderChangeWidth", "O"),
  "stairs_audio_system": ("帧首 6 loop stop；kind 表 0x142bad9f0(≤18)：瞬发 StairPlace/StairDelete/StairsMerge/StairsMergeConfirm/LadderAppear/LadderPlace/StepsAppear/WalkwayAppear/StairSupportAppear/StairPillarAppear + 参数表 stair_width/size/editing_speed/extend_amount/h_offset/avg_arch_supports/height_above_ground/in_water/stair_edge_length/is_stub_preview/move_all", "O")}),
"systems/audio/replay_audio.rs.md": (
 "撤销/重做/拒绝输入音效。",
 {"replay_audio_init_system": ("2×instantiate_audio(event:/UI/Undo、event:/UI/Redo)", "O"),
  "replay_audio_system": ("kind 0/1→Undo/Redo、2→event:/UI/DenyInput；AppMode==0xd 设全局参数", "O")}),
"systems/audio/roof_audio.rs.md": (
 "屋顶编辑音效：3 循环 + 位置/音量即时实例辅助。",
 {"roof_audio_init_system": ("3×instantiate_loop：RoofShape/RoofTipMove(0x142adcb38)/InterRoofMerlonsRearrange(0x142adcb68)（重建代码 RoofAppear/RoofSwitchDirection 为错）", "O"),
  "roof_audio_system": ("按 RoofAudioState 驱动循环音量/位置", "O"),
  "instantiate_audio_w_position": ("真实签名 4 参(core,name&str,pos)：instantiate_audio+set_position（调用点 0x1408b7788 O）", "O"),
  "instantiate_audio_w_position_and_volume": ("instantiate_audio→set_position→start_if_not_playing→EventInstance::set_volume", "O")}),
"systems/audio/sheep_audio.rs.md": (
 "绵羊反应音效（kind 0-7 + 参数 FirstPersonOff）。",
 {"sheep_audio_init_system": ("SheepAudio::new（impl$0::new O 名；0x1409d5ea0 为 sheep_audio_system）", "I"),
  "sheep_audio_system": ("kind 表 0x142ac2cc8(0..7)：Aww/Think/So/Angry/Idea/Eyes/Confused→instantiate+set_parameter+start；kind4=帧尾 start/stop 分支；参数 FirstPersonOff", "O")}),
"systems/audio/signifier_audio.rs.md": (
 "UI 符号流音效（1 循环 + 4 瞬发 + 21 参数更新）。",
 {"signifier_audio_init_system": ("1×loop+4×instantiate：UI Boop(0x142ac2465)/Cursor/Grab/Cursor/Release/HoldTimeSiginfier(原版拼写)/UiRotateGizmoMovingBack", "O"),
  "signifier_audio_system": ("UiSignifierStream→set_parameter×21（含 FirstPersonOn/Off、Tool 参数）", "O")}),
"systems/audio/terrain_audio.rs.md": (
 "地形编辑音效（4 循环 + 编辑状态机）。",
 {"terrain_audio_init_system": ("4×instantiate_loop：TerrainEditShape/TerrainEditHeight/TerrainCreateChunk/TerrainMove（名 I 映射）", "O"),
  "terrain_audio_system": ("PlayTerrainSound 事件+地形编辑栅格→网格索引→kind 跳表→4 loop 应播标志 start_if_not_playing+set_parameter+set_position/stop", "O"),
  "EditStateAudio::new/update": ("编辑状态机 new/update（行 18/25 O 名）", "I")}),
"systems/audio/terrain_stairs_audio.rs.md": (
 "地形楼梯音效（SuppressPathSound + StairsCounter 状态 + StairDraw_OneShot 瞬发）。",
 {"terrain_stairs_audio_init_system": ("构造 TerrainStairsAudioState（ActiveAudioInstances retain(is_playing) 清理语义）", "I"),
  "terrain_stairs_audio_system": ("AppMode==5 早退；SSBO BufferState 名称断言(0x45B 源路径)；缓冲 id 同→stop 否则 instantiate_audio(StairDraw_OneShot)+set_parameter(inverse_lerp/60 音量)+start", "O")}),
"systems/audio/timberframe_audio.rs.md": (
 "木骨架音效（PlayTimberframeSound）。",
 {"timberframe_audio_init_system": ("构造 TimberframeAudioState（瞬发+循环混合）", "I"),
  "timberframe_audio": ("kind→TimberframeMove/Length/Width（+RectangleAppear 等）→instantiate+set_parameter(editing_speed)", "O")}),
"systems/audio/tree_audio.rs.md": (
 "树木音效（4 循环 + 最近树距离排序）。",
 {"tree_audio_init_system": ("4×instantiate_loop：Tree/Amb、TreeMove、TreePlace、TreeGizmoHover/Release", "O"),
  "tree_audio_system": ("PlayTreeSoundCmd（tree-tool 消费）+Query<&TreeAmbience> (dist,pos) 稳定排序→最近树 set_position/音量", "O")}),
"systems/audio/trigger_audio.rs.md": (
 "音频触发器：相机穿过树木触发盒的响度（4 实例 + audibility 平滑）。",
 {"trigger_audio_init_system": ("4×instantiate_audio：Camera Tree Rustle/Amb/High Wind/Camera Whoosh/snapshot:/InForest", "O"),
  "trigger_audio_system": ("相机段×AudioTriggerLibrary 全部盒 Aabb3::intersect_segment：命中 audibility+0.5 否则-0.1 clamp[0,1](@+0x34)；<=0 停 High Wind；音量=位移/阈值×0.0125×位置衰减 lerp 平滑(@+0x35)；相机 z>=11 或 dist<=70 停 InForest 快照否则 start+set_parameter；set_volume", "O")}),
"systems/audio/wall_audio_fmod.rs.md": (
 "墙体音频主系统：10 循环 + kind 0-23 分派 + 帧尾 start/stop。",
 {"wall_audio_init_system": ("10×instantiate_loop(desc{name,count=3,max=50})：Wall Draw/Wall Change Height/FenceExtend/LintelExtend/StoneArchExtend/WideningGate/PlatformStairsExtend/SewerGateWiden/FenceHeight/FenceMove；WallAudioState=固定 10×stride0x40 数组+参数 Vec@+0x48/0x88/0xc8/0x148+音量@+0x280", "O"),
  "wall_audio_fmod": ("arg1=AppMode(==13 音量 1.0) arg2=AudioCore arg3=EventReader<PlayWallAudio> arg4=ResMut<WallAudioState>+栈参音量(+0x1c/60.0)；kind 表 0x142ab4998(0..23)：循环类 request_play+set_parameter(WallHeight/FenceLength/HasBricks)，瞬发类 instantiate_audio_w_position_and_volume(FenceAppear/Merge/Rearrange/Lintel*/StoneArch*/Pillar*/Gate*/PlatformStairs*/WallSplit/SewerGate*)，帧尾 10 loop 应播 start/stop+[+0x280] 音量×powf(0.1,arg+0x58×5.0)", "O")}),
"systems/audio/wall_audio_orchestrator.rs.md": (
 "墙体材质音频协调器（RequestWallMaterialAudio）。",
 {"wall_audio_orchestrator": ("RequestWallMaterialAudio→hashbrown/atomic_float 协调参数更新（0x141234bf0）", "O")}),
"systems/audio/wall_audio_path.rs.md": (
 "墙体路径绘制音效。",
 {"wall_audio_path": ("路径绘制循环驱动（0x140a89bb0；细节 U）", "I")}),
"systems/audio/wall_audio_ui.rs.md": (
 "墙体 UI 音效（PlayWallUiSound kind 分派）。",
 {"wall_audio_ui": ("kind 分派（数据表 0x142a9f730 邻域）：ColorBricks/Gate/Halftimber/Window、ChangeWallStyle、WindowDrag、Lantern/TurnOn、Chimney/ToggleSmokeOn|Off（[+9] 字节二选一）、BuildingSupportsRemoveWooden、WallSplit 等→instantiate_audio_w_position", "O")}),
"systems/audio/wall_audio_utils.rs.md": (
 "墙体循环播放器（AudioLoopPlayer，10 循环的公共实现）。",
 {"AudioLoopPlayer::request_play": ("+0x38=1(应播)；+0x28<+0x10 否则 panic；instances[current]→set_position→set_parameter(editing_speed)→set_volume", "O"),
  "AudioLoopPlayer::new": ("由 instantiate_loop 结果装填（instances 池 cap-first Vec{cap@0,ptr@8,len@0x10}）", "I"),
  "AudioLoopPlayer::resolve": ("取当前实例 instances[current]（行 32 内联）", "I")}),
"systems/audio/water_ambience_audio.rs.md": (
 "水面环境音：按水网格逐格装配的 Amb 循环播放器。",
 {"water_ambience_audio_init_system": ("按水网格容量 from_iter 装配多个 AudioLoopPlayer（Map<Range,closure>）+get_parameter_by_name+insert_resource<WaterAmbienceAudio>+<WaterAudioGrid>；事件 event:/SFX/Water/Amb", "O"),
  "water_ambience_audio_system": ("演示模式禁用；逐 WaterAudioGrid 格：有水面→start_if_not_playing+set_position(eye_position 相对)+set_volume(0.015625/42.5 衰减)；无→stop", "O"),
  "load_index_unchecked": ("水网格索引越界直读（0x1409ebe80 O 名）", "I")}),
"systems/audio/water_audio.rs.md": (
 "水体绘制/擦除/结冰音效（4 循环 + mode）。",
 {"water_audio_init_system": ("4×instantiate_loop：WaterDraw/WaterErase/IceDraw/IceErase（+PathWaterDraw/PathWaterErase 邻接表）", "O"),
  "water_audio_system": ("WaterDrawEvent/WaterEraseEvent+WaterRaster 栅格索引→update_draw_audio(garden 模板)×2；mode@+0x58", "O")}),
"systems/audio/weather_audio.rs.md": (
 "天气/季节全局音频参数（Weather→Studio 全局参数）。",
 {"weather_audio_system": ("从 Weather 状态 AudioCore::set_parameter 驱动季节/天气参数（参数名 U）", "O")}),
"resources/audio_resources.rs.md": (
 "音频核心状态：FMOD Studio 内核、银行表、AudioInstance 实例模型、全局失败报告注册表。",
 {"AudioCore::visualize_sounds": ("调试可视化：遍历活动实例→位置→画 3D 标记（thread_local 访问）", "O"),
  "Bank::load": ("loaded&1 早退；Studio::load_bank_file(path@+0x10)；失败→panic \"Error loading FMOD bank file: {path} {err}\"(fmt 0x142ad25b0)；成功 loaded=1+句柄@+8", "O"),
  "Bank::load_sample_data": ("Bank::load_sample_data+日志 \"Loading sample data of FMOD bank \"", "O"),
  "impl$0::load::closure$0": ("Bank::load 失败路径 panic 格式化闭包（fmt+{path,err}）", "O"),
  "DelayedBankLoader::load": ("PDB 名挂在 0x140a53a00 上但该址实为 AudioCoreInner::update（错位）；真实 load U", "U"),
  "AudioCoreInner::set_parameter": ("Studio::set_parameter_by_name；失败→全局报告注册表(槽 0x143634d60/d58)：module_p_<名>path!(…)::Warn:<err>；锁失败 panic \"Mutex was poisonned\"；anyhow \"Couldn't set global FMOD parameter \"", "O"),
  "AudioCoreInner::instantiate_audio": ("Studio::get_event→create_instance；Active{event 8B@+8,name cap-first{cap@+0x10,ptr@+0x18,len@+0x20},state=2@+0x28}；失败 Err(anyhow \"FMOD error: \"/\"get_event(\")", "O"),
  "AudioCoreInner::update": ("0x140a53a00（PDB 名 DelayedBankLoader::load 错位）：听者三断言(|forward|-1|/|up|-1|/dot<0.001)→set_listener_attributes(listener=0)→Studio::update；失败 log \"FMOD update error: \"", "O"),
  "AudioCoreInner::set_vca_volume": ("0x140a54810（PDB 名 instantiate_audio::closure$0 错位）：Studio::get_vca→Vca::set_volume（unwrap panic）", "O"),
  "AudioSamplesLoadedPromise::recv": ("0x140a57100：crossbeam 三通道(array/list/zero) recv_timeout(1s)；超时丢 payload 正常返回；断开→unwrap panic", "O"),
  "AudioInstance::start": ("判别==1→EventInstance::start（0x1414372d0）", "O"),
  "AudioInstance::stop": ("判别==1→EventInstance::stop(0=STOP_IMMEDIATE)（0x140a57470）", "O"),
  "AudioInstance::start_if_not_playing": ("判别&1→get_playback_state→位掩码 0xb（{Playing,Sustaining,Starting} 跳过）→start", "O"),
  "AudioInstance::set_position": ("判别==1→set_3d_attributes：position=入参，forward=(0,0,-1)/up=(0,1,0)/velocity=0(常量 0x142ad0460)；失败 anyhow \"Failed to set 3d attributes for FMOD event \"+报告槽 0x143634d80/d78", "O"),
  "AudioInstance::set_parameter": ("0x140a57670（PDB 名 pause 错位）：签名(&self,name:&str,f32)→EventInstance::set_parameter_by_name；失败同全局报告机制", "O"),
  "AudioInstance::pause": ("set_paused(true)（行 562 O 名；PDB 名挂在 set_parameter 上）", "I"),
  "AudioInstance::resume": ("set_paused(false)（行 581 O 名）", "I"),
  "AudioInstance::is_paused": ("get_paused（行 572 O 名）", "I"),
  "AudioInstance::is_playing": ("get_playback_state==Playing（0x140a57550）", "O"),
  "AudioInstance::get_timeline_position": ("get_timeline_position（行 603 O 名）", "I"),
  "AudioInstance::stop_immediately": ("stop(0)（行 544 O 名）", "I"),
  "AudioInstance::stop_on_drop": ("Drop：判别==1→stop", "O"),
  "AudioInstance::set_volume": ("EventInstance::set_volume（0x140a575a0）", "O"),
  "AudioInstance::has_parameter": ("get_parameter_by_name 探测（行 621 O 名）", "I"),
  "AudioCore::instantiate_loop": ("from_iter(0..count,|_|instantiate_audio(desc.name)) 建实例池+首实例 get_parameter_by_name 得 has_parameter（0x140a59220）", "O"),
  "fmod_to_anyhow": ("libfmod 错误码→anyhow（行 680 O 名）", "I"),
  "AudioLoop::start_if_not_playing/stop/current": ("循环池方法：50ms 冷却 stop+current=(current+1)%count；start 置 last_start（utils::audio_utils 域，O）", "O")}),
"resources/audio_settings.rs.md": (
 "音频音量设置（GameConfig 内嵌，RON serde；4 个 f32 音量字段 O）。",
 {"AudioSettings::serialize/deserialize/clone/default": ("RON serde 派生（game_config 引用 O）", "O（派生）"),
  "AudioSettings::default": ("缺省（音量字段初值未逐字节证）", "I")}),
"resources/color_icon.rs.md": (
 "调色菜单图标组件族 + 亮度值枚举（三变体→{0.25,1.0,4.0} 表）。",
 {"ColorMenuBrightnessValue::to_f32": ("0x140b06770：判别式索引 f32 表 0x142ae785c={0.25,1.0,4.0}；三变体无载荷（重建代码 {Default,Custom(f32)} 错）", "O"),
  "ColorMenuBrightnessValue::default_f32": ("0x140b06780：常量 1.0（0x142925904）→缺省变体=1", "O"),
  "ColorIcon::register_required_components": ("bevy 组件注册胶水（TypeId 查表样板，×15 图标组件同构）", "O（胶水）")}),
"resources/color_id.rs.md": (
 "颜色索引资源族：ColorId(u32)+墙/门/半木三表+SSBO 载体。",
 {"WallColorIds::new": ("建空表（0x140b19890）", "O"),
  "GateColorIds::new": ("建空表（0x140b19900）", "O"),
  "HalftimberColorIds::new": ("建空表（0x140b19970）", "O"),
  "iter_nondefault": ("save/snapshot 内联：滤默认色（默认判定按屋顶侧=1 重审）", "O"),
  "default_value": ("注意：0x14099ca50 属 color_id_roof 且返回 1（非本文件 ColorId(0)）", "U（归属纠错）"),
  "register_required_components": ("bevy 组件注册（0x140b1cf80）", "O（胶水）")}),
"resources/color_id_roof.rs.md": (
 "屋顶颜色 ID SSBO。",
 {"RoofColorIdSsbo": ("read_storage_buffer<RoofColorIdSsbo> 载体（draw_bricks/instanced_roof O）", "O"),
  "default_value": ("0x14099ca50：返回 1（屋顶默认 ColorId=1；wp16 合同写 0 错）", "O")}),
"resources/editing_highlight.rs.md": (
 "编辑高亮资源（wall_highlight 写 rgba 的载体）。",
 {"EditingHighlight": ("wall_highlight 写 rgba（WP12 facade）", "I")}),
"resources/feedback_report_resources.rs.md": (
 "反馈报告资源：截图 Arc 双槽+元数据区+状态机+选择字节。",
 {"FeedbackReportBgTexture::is_available": ("0x140909860：[+0x36]<7（可用性枚举字节；非独立 bool 资源）", "O"),
  "FeedbackReportTags::to_list": ("0x140909870（PDB 名错位）：实为羊叫 4 标志→{aww,think,angry,sob} 列表装配（模块归属存疑）", "O（错位）"),
  "FeedbackReportResources::reset_choices": ("0x140909910：[+0x40..0x46]={0,1,1,0,0,0}", "O"),
  "FeedbackReportResources::clear_screenshot": ("0x140909930：状态[+0x3e] min(state-7,2) 分派：0(空)跳过/1 锁[+8]Mutex/2 换纹理 Arc；清空后 state=7", "O"),
  "impl$3::build": ("0x140909a70：bevy 插件 build——TypeId 查表注册 FeedbackReportResources 默认值；缺失 panic", "O"),
  "feedback_report_texture_receive": ("0x140909ab0：(ResMut<FeedbackReportResources>,EventReader<…>)：有事件→换[+8]纹理 Arc→拷事件元数据入[+0x10..0x38]→state=7/8 机转；第二分支(state==8)锁 Mutex 更新纹理内部结构", "O")}),
"resources/glade_color_customization.rs.md": (
 "谷地自定义配色（serde 存档；默认字节 O）。",
 {"GladeColorCustomization::default": ("0x8c5600(82B)：+0x8=0x186a3、+0x18=0x800000000、+0x42=1、+0x60=1.0 等（字段语义 U）", "O"),
  "StairCustomization": ("阶梯配色（O 名，U 布局）", "I")}),
"resources/gothic_window_profile.rs.md": (
 "Gothic 窗/门拱剖面曲线（size 1/2/3→Vec<Vec2> 点列）。",
 {"gothic_door_profile_curve": ("0x1409a4e70：size∈{1,2,3} 分派→常量点列；越界 panic", "O"),
  "gothic_half_dormer_profile_curve": ("0x1409a4cc0：同构（半老虎窗剖面）", "O")}),
"resources/gpu_lut.rs.md": (
 "GPU LUT 生成（brdf_fg/water_noise 两条目 tracker + 计算 pass）。",
 {"GpuLutPlugin::build": ("0x14090e080：Vec::with_capacity(8)（条目 stride0x30={u64,u64,u64,self,vtable,rendered}）；条目1 vtable 0x142ad6290；非 demo(全局 0x143634d90==0)加条目2(vtable 0x142ad6260)→insert_resource<GpuLutTracker>", "O"),
  "GpuLutTracker::render_luts": ("0x14090e280：逐条目[+0x28]==0→[+0x10]=0→vtable[+0x28]()→[+0x28]=1", "O"),
  "compute_impl": ("0x14090e310：vtable[+0x18] 取 URI 入 Vec→FUN_141600570 装 400B ComputePass→memcpy→vtable[+0x18] 计算", "O")}),
"resources/grading_lut.rs.md": (
 "颜色分级 LUT：完整 .cube 解析器 + assets/luts 目录装载。",
 {"GradingLut::load_cube": ("0x140b01050（PDB 名 asset_name 错位）：逐行解析 TITLE 跳过/#注释跳过/DOMAIN_MIN 必须(0,0,0)/DOMAIN_MAX 必须(1,1,1)/LUT_1D_SIZE→错 \"1D LUTs are not supported\"/LUT_3D_SIZE 手写 u64 解析(±号/≤17位/溢出检查)→size³ 数组(×4B)；数据行 parse_rgb→RGB9E5 打包→3D 纹理(名 \"grading lut\")；超 size³→错 \"Too many entries in the LUT…\"", "O"),
  "GradingLut::load_unwrapped": ("0x140b00bc0：展平 PNG→RGBA8→3D 纹理；路径前缀 \"assets/luts/\"", "O"),
  "parse_rgb": ("0x140b01cc0：空白切分→3×f32 解析；失败 Err", "O"),
  "GradingLuts::load": ("0x140b01df0：readdir \"assets/luts\"→逐 DirEntry 闭包(0x140ae5bb0：DirEntry 空 panic \"read_dir entry\"、按扩展名分派 cube/unwrapped)→双 Vec(stride 0x28/0x38 cap-first)", "O"),
  "ColorGradingState::flattened": ("0x140b00b20：{name,4×f32,byte} 扁平化→final_post/prepare_frame_constants 消费", "O"),
  "GradingLut::asset_name": ("PDB 名挂在 0x140b01050(实为 load_cube)上；真实地址 U", "U（错位）")}),
"resources/image_formation.rs.md": (
 "画面形成配置：三档 0x40B config+当前档 kind@+0xc0（缺省档=1）。",
 {"ImageFormationConfigs::current": ("configs[kind]（内联）", "O"),
  "ImageFormationConfigs::current_mut": ("0x1408aea50：self+(kind<<6)", "O"),
  "ImageFormationConfigs::get": ("0x1408aea80：self+(i<<6)（无边界检查）", "O"),
  "ImageFormationConfigs::iter_non_default_mut": ("0x1408aea60：{begin,begin+0xc0,idx=0} 三档遍历", "O"),
  "ImageFormationConfigs::switch_kind": ("0x1408aea90：[+0xc0]=kind", "O"),
  "ImageFormationConfigs::default": ("0x1408ae7f0：三档名 \"default\"、params(1,0,1,1)、档2 +0x30=(0,0.015)+0x3d=进程布尔(0x143634d90)；末尾 kind=1", "O"),
  "ArtisticConfig/CameraLensConfig": ("serde 子类型（O 名，U）", "I")}),
"resources/placement_data.rs.md": (
 "放置数据 SSBO 元素（position/scale/angle→Affine3A）。",
 {"PlacementDataSsbo::to_affine3a": ("0xad5020：绕 Y 旋转×scale+平移；4×movaps 写 4 行（末行 xmm1=position 含 z——w 行=pos.z 为原版怪癖，忠实转录）", "O"),
  "PlacementDataSsbo::position": ("0xad51a0", "O"),
  "PlacementDataSsbo::to_transform": ("to_affine3a 别名（6 内联）", "I"),
  "default_anim_t": ("放置动画参数（2 内联，语义 U）", "U")}),
"resources/shader_refactoring_state.rs.md": (
 "着色器重构开关状态。",
 {"ShaderRefactoringState::disabled": ("0xb1f210：fn disabled(&self)->bool（[+0]==0；重建代码构造器模型错）", "O")}),
"resources/shutdown.rs.md": (
 "游戏关闭命令（bevy 系统）。",
 {"initiate_game_shutdown": ("0xb2ba40：bevy 系统（EventReader<ShutdownGame> 双缓冲+Commands::insert_resource；重建代码 Vec 参数模型错）", "O")}),
"resources/steam.rs.md": (
 "Steam 动作集定义：GameControls 28 动作/Menu 2 动作/CottageWindow+GothicWindow 层。",
 {"SteamMainActionSet::action_handle": ("0x1417c3720：动作名表 0x142ce2049：动作集 \"GameControls\"+28 动作（CameraPan/cursor_move/camera_height/left_click/right_click/undo/redo/tool_*×10/time_of_day/photomode/toggle_snapping/zoom_in/out/main_menu/ui_*7）", "O"),
  "SteamMenuActionSet::try_load": ("0x1417c5770：Menu 集 ui_tab_prev/ui_tab_next 句柄加载（check_get_*_handle 内联）", "O"),
  "SteamMainActionSetAnalogAction::name_c_str": ("0x1409e8d80：动作名 C 串（NotFound 手解区）", "I"),
  "into_action_value": ("0x1409e6280：Steam 动作值→游戏输入值转换", "I")}),
"resources/texture_presets.rs.md": (
 "纹理预设绑定类型（TexturePreset 数值槽/TextureFromDisk 名字哈希）。",
 {"TexturePreset::from": ("0xa59890：From<u8> 判别映射→rhapsody TextureDesc（usage: 8=mip/10-12=depth/15-16=color）", "O"),
  "TextureFromDisk": ("64 位名字哈希（xxh3）取磁盘纹理（water_noise=0x579f39ab91a01865 O）", "O")}),
"resources/window.rs.md": (
 "窗口尺寸/光标资源（WindowSize 头 8B=Vec2 f32×2）。",
 {"WindowSize::as_vec2": ("0x1400885d0：mov rax,[rcx]——头 8 字节直接当 Vec2 返回（重建 u32 模型错）", "O"),
  "WindowSize::aspect_ratio": ("PDB 名挂在 0x140949c40 上但该址=静态析构注册（唯一引用 0x14368E2FC DATA 非 call）；真实体 U", "U（错位）"),
  "AvailableMonitors::clear": ("显示器列表清理（O 名；与 0x140949c40 析构邻接，宿主疑为显示器缓存静态）", "I")}),
"resources/window_lights.rs.md": (
 "窗光/灯资源族（WindowLight 组件/LightInfo/Global*Control）。",
 {"WindowLight::register_required_components": ("0x14086cac0：bevy 组件注册胶水（TypeId 查表）", "O（胶水）"),
  "WindowLight": ("decorator 系统挂窗实体；glass 绘制+bin_lights 光源消费（字段 U）", "O（消费链）")}),
"systems/steam/mod.rs.md": (
 "Steam 集成：客户端资源/覆盖层/截图/输入映射。",
 {"SteamClientResource::show_floating_gamepad_text_input": ("0x140ae9390：SteamAPI_ISteamUtils_ShowFloatingGamepadTextInput+Dismissed 回调", "O"),
  "SteamClientResource::is_floating_gamepad_text_input_open": ("0x140ae9470：进程全局字节 0x143634d94（回调缓存，非 API 调用；重建模型错）", "O"),
  "SteamClientResource::input_show_binding_panel": ("0x140ae9480：客户端非空→ShowBindingPanel(handle)", "O"),
  "AppSteamIntegration::client": ("0x14067dd80：mov rax,rcx（client 字段@+0）", "O"),
  "AppSteamIntegration::user_id": ("0x140ae96f0：每次现算 client.user().steam_id()（Arc 临时 clone+drop；重建存字段模型错）", "O"),
  "AppSteamIntegration::try_new": ("0x140ae94b0：创建 steamworks 客户端", "I"),
  "add_steam_integration": ("0x140945320：bevy 插件 build——0x30B 系统配置{1,1,0,0,8,0}（8 系统批量 add_systems 链）", "O"),
  "update_steam_inputs": ("0x140ae9740：14 个同构动作块：读动作状态→沿变化→游戏输入计数(+0x170 减/+0x178 增)→压 u16 输入码(0x200..0xf00)入环形队列+PrevSteamInputState 帧差；码→动作名配对需 ui-input 域枚举", "O（结构）"),
  "SteamOverlayOpenResources::can_use_overlay": ("0x140aef1a0（validate_param 系）：overlay 可用性校验", "O"),
  "SteamOverlayOpenResources::open_overlay_or_browser_to_webpage": ("0x14026d3c0：overlay 可用→开 overlay 否则浏览器（0x14026d080/0x14026d580 为 &str/String 两泛型实例）", "O")}),
"systems/steam/startup_steam.rs.md": (
 "Steam 启动系统（首帧一次性资源插入）。",
 {"startup_steam": ("0x140a9bbf0：首帧（全局帧计数）一次性 FUN_1412e0660 Commands 插入 Steam 启动资源束（6 qword 常量 0x142adafb0）；steamworks 客户端 init 族 0x142923700/0x1429236e0", "O"),
  "check_get_action_set_handle": ("动作集句柄检查（try_load 内联）", "I（内联）"),
  "check_get_digital_action_handle": ("数字动作句柄检查（内联）", "I（内联）"),
  "check_get_analog_action_handle": ("模拟动作句柄检查（内联）", "I（内联）")}),
"systems/debug/human_scale_ref.rs.md": (
 "人体比例参考调试可视化（多模式跳表）。",
 {"human_scale_ref": ("0x1408b2b30：5 参 bevy 系统；*param_5+0xf8 调试开关门禁；[*param_2+0x110] u32 模式→跳表 0x142a9f3b0；puffin scope", "O")}),
"systems/debug/lazy_grid_debug.rs.md": (
 "惰性网格调试开关（全内联）。",
 {"lazy_grid_debug": ("全内联（无独立过程）——bool 资源+开关", "I")}),
"systems/debug/sheep_controls.rs.md": (
 "绵羊调试控制（拾取/快照）。",
 {"sheep_controls": ("0x1416f11a0：调试绵羊控制（PickingMode 拾取）", "O（名）"),
  "apply_sheep_snapshot": ("0x1416f24f0：应用绵羊快照", "O（名）"),
  "SheepControl::new": ("0x1416f0ea0：调试控制组件构造", "O（名）")}),
}

def fill(path):
    full = os.path.join(ROOT, path)
    if not os.path.exists(full):
        print("MISSING", path)
        return
    txt = open(full, encoding="utf-8").read()
    purpose, fns = DATA[path]
    txt = re.sub(r"> 文件作用：\*\*[^*]*\*\*.*", f"> 文件作用：**{purpose}**", txt)
    lines = txt.split("\n")
    out = []
    for ln in lines:
        m = re.match(r"^\| `([^`]+)` \| (0x[0-9a-f]+) \| (\d+) \| ([^|]*) \| ([^|]*) \| ([^|]*)\|$", ln)
        if m:
            name, addr, srcline, role, concl, ev = m.groups()
            short = name.split("::")[-1].strip()
            key = None
            for k in fns:
                if short == k or short.startswith(k) or k.startswith(short) or short in k:
                    key = k
                    break
            if key:
                nrole, nconcl = fns[key]
                # keep evidence column, append extra if we have decomp ref
                ev2 = ev.strip()
                if not ev2 and addr and os.path.exists(os.path.join(ROOT, "..", "audio-misc", "decomp", f"decomp-{addr}.txt")):
                    ev2 = f"audio-misc\\decomp\\decomp-{addr}.txt"
                out.append(f"| {name} | {addr} | {srcline} | {nrole} | {nconcl} | {ev2} |")
                del fns[key]
            else:
                out.append(ln)
        else:
            out.append(ln)
    txt = "\n".join(out)
    open(full, "w", encoding="utf-8").write(txt)
    leftover = [k for k in fns if fns[k]]
    print(f"filled {path}" + (f"  [unmatched rows remain: {list(fns.keys())}]" if fns else ""))

for p in DATA:
    fill(p)
