# audio-misc 域 算法级深挖（2026-10-15）

> 覆盖 59 章：`systems/audio/**`（33）+ `music/*`（3）、`systems/steam/*`（2）、`systems/debug/*`（3）、
> `resources/{audio_*, color_*, grading_lut, gpu_lut, image_formation, window*, editing_highlight,
> feedback_report_resources, steam, shader_refactoring_state, texture_presets, render_ssbo,
> glade_color_customization, gothic_window_profile, placement_data}`（20）、顶层
> `{app_loading_state, frozen_map, meta_commander, name_hash}`（4）。
> 证据 = `audio-misc/decomp/*.txt` + round2 CLOSURE（MusicPlayer 双跳表 capstone 全解）+ rne-read 串。
> 标注：O=机器码证实 / I=推断 / U=未知。三 watch（asset_libraries 域）不重挖。

---

## 1. 处理链图

### 1.1 MusicPlayer::update 状态机（O，round2 CLOSURE 全解）

```
每帧：
  ① 读当前曲目 timeline position（get_timeline_position），clamp ≥ 0
  ② state(+0x118) == 0 ?
     ├─ 是 → switch_track()（FUN_1408b70d0 推进到下一个 [+0]==1 曲目）
     │       → 新曲目启动：主事件[+0x40]==1→start(main)；否则 track[+0]==1→start(track)；
     │         再 get_playback_state→Playing 且 set_paused(0) 恢复；state=1
     │      + verbosity>4 log "snapshot:/Track Switching"(0x142a9f5d0)
     └─ 否 → last_position@+0x128 = pos（曲目回卷 pos<last → 表 B + 清零节拍全局 0x143634c90；
           否则 表 A）；跳表 (state-1)*4 → 4 有效条目：
        A1(0x1408b56ef) acc>阈值1 且主使能→stop(立即)
        A2(0x1408b597d) Duration@+0x120(i64；负值 round-to-even (v>>1)|(v&1) 后 ×2)
                        超时 → cvttss2si 夹取 → 推进/切曲路径 0x1408b5b52
        A3(0x1408b59b0) 节拍同步：全局 0x143634c68==1 且 0x143634c90&3==0 → state=4 + start
        A4(0x1408b5826) acc>阈值2 → stop(ALLOWFADEOUT)；acc>阈值3 → state=0（下一帧 switch）
        B1/B2         = 回卷路径对应处理（并入 A4/A1 续块）
        B3(0x1408b5975) 无条件 → 0x1408b59be（0x143634c68==1 → state=4+start；随后 stop 事件@+0x48）
        B4=A4（同块）
  ③ 每 case 先 time_acc@+0x11c += dt
  ④ 每帧清零全局节拍计数 0x143634c90
```

### 1.2 AudioInstance 生命周期（audio_resources.rs）

```
判别@+0：0=Disabled / 1=Active / 2=Err(anyhow)
Active 布局：{disc@0, *mut FMOD EventInstance@+8, name String(cap-first)@+0x10, state u8@+0x28}
start(0x140a57240):  [+0]==1 → EventInstance::start；Err 判别 0x8000000000000004(Ok)
stop(0x140a57470):   [+0]==1 → stop(0)=STOP_IMMEDIATE
start_if_not_playing(0x140a57300): [+0]&1 → get_playback_state → (0xb>>state)&1==0
    （即 state∉{0,1,3}=∉{Playing,Sustaining,Starting}）→ start；两处 unwrap panic
set_position(0x140a57da0): set_3d_attributes 常量 forward(0,0,-1)/up(0,1,0)/velocity(0,0,0)
    （.data 0x142ad0460）；失败 anyhow "Failed to set 3d attributes for FMOD event "
set_parameter(0x140a57670, PDB 名 pause 错位): set_parameter_by_name(&str, f32)；失败→全局报告
update(0x140a53a00, PDB 名 DelayedBankLoader::load 错位): 听者三断言→set_listener_attributes(0)
    → Studio::update（失败 log "FMOD update error: "）
```

### 1.3 Steam 输入映射链（update_steam_inputs）

```
帧计数推进 → 22 个同构块（每块 ~222 行）：
  [读 Steam 动作状态] 16B 槽 bitset SIMD lane 匹配找本帧变化动作 → 数字位/模拟值
  [沿变化簿记] 游戏输入资源计数 +0x170 减(释放)、+0x178 增(按住)
  [压码] 游戏输入环形队列 Vec<u16>（元素 2B，cap-first）：
          len==cap → FUN_140b5fff0 grow（Location 0x142ae39a0）
          *(u16*)(ptr + len*2) = 输入码；len++
  22 个输入码（代码序）：0x400,0x500,0x600,0x700,0x900,0x800,0xa00,0xb00,0xc00,0xe00,
  0xd00,0x200,0x300, 0x1300,0x1400,0x1600,0x1700,0x1800,0x1500,0x1200,0xf00, 0x101(尾块)
  → 码→动作名配对需 ui-input 域游戏 InputEvent 枚举（跨域）
```

### 1.4 GradingLut::load_cube 解析器

```
读文件(Result 检查) → 逐行：
  空行/# 注释/TITLE → 跳过
  DOMAIN_MIN → parse_rgb，全 0 才合法（否则 anyhow "DOMAIN_MIN must be 0 0 0"）
  DOMAIN_MAX → 必须 (1,1,1)（"DOMAIN_MAX must be 1 1 1"）
  LUT_1D_SIZE → panic/错误 "1D LUTs are not supported"
  LUT_3D_SIZE → 手写十进制 u64 解析（±、≤17 位、溢出检查）→ size³ 数组扩容（×4B，0 填充）
  数据行 → parse_rgb → RGB9E5 打包（func_0x0001417c66b0）
  → 超过 size³ → "Too many entries in the LUT. Bailing before an overrun"
  → 行数不足 size³ → 越界 panic（grading_lut.rs）
建 3D 纹理：{size,size,size} → 尺寸准备 → 上传 size³×4B（名 "grading lut"）→ unwrap panic
GradingLuts::load：read_dir "assets/luts"（失败 panic "read_dir"）→ per-file 闭包按扩展名
  分派 load_cube/load_unwrapped → 双 Vec {stride 0x28 A, stride 0x38 B}（cap-first）
```

---

## 2. 逐函数算法（O 全量优先）

### 2.1 音频核心（resources/audio_resources.rs，13 过程）

- **Bank::load @0x140a53640**（O）：`loaded[+0]&1 → 早退`；verbosity>4 log；
  PathBuf 转换异常 → panic_str("Error loading FMOD bank file: ", 0x142ad2578)；
  `Studio::load_bank_file(path, flags=0)` → **Ok(0x8000000000000004) → loaded=1 + 句柄@+8，返回**；
  Err → 格式化 panic（FUN_140a537e0 + breakpoint，fmt "Error loading FMOD bank file: {path} {err}"
  0x142ad25b0）。**失败 = panic，非日志**。
- **Bank::load_sample_data @0x140a53870**（O）：log "Loading sample data of FMOD bank " → libfmod。
- **AudioSamplesLoadedPromise::recv @0x140a57100**（O）：crossbeam 三通道（array/list/zero）
  `recv_timeout(1e9 ns=1s)`；超时丢弃 payload 正常返回；通道断开 → panic
  "called Result::unwrap() on an Err value"（audio_resources.rs）。
- **AudioCoreInner::update / set_vca_volume @0x140a54810**（O，PDB 错位）：get_vca→set_volume→unwrap。
- **全局报告注册表**（set_parameter/set_position 共用，O）：每函数独立槽 0x143634d60/d80；
  失败流程 `if 槽!=3 → 注册(0x1428d8820)；返回1 → panic "Mutex was poisonned"(0x142ad2a48, 双 n)`；
  否则拼 "module_p_"+错误fmt+"path!(…"+参数名+"$crate :: Level :: Warn"+err
  （模板 0x142ad2a5b/63、字面 0x2928202168746170="path!(!、"、0x6e726157203a3a20=" :: Warn"）
  → 入表 0x14143b8b0；verbosity 0x14362c3a8>1 → log；panic 中标志 0x14362c420 非 0 且
  FUN_1428d86b0()==0 → 写 name 缓冲字节2=1；LOCK; 消费标记; c==2 → 释放缓冲。

### 2.2 MusicPlayer（music/music_player.rs，9 过程）

- **update**：见 §1.1。字段：曲目 Vec stride 0x30（[+0]==1 使能、[+8] 事件句柄）、
  [+0x40] 主事件使能、[+0x48] 主 fmod 事件、[+0x118] state、[+0x11c] time_acc、
  [+0x120] Duration、[+0x128] last_position。
- **stop_immediately @0x1408b72e0**（O，PDB 名 current_track 错位）：遍历全部曲目
  `[+0]==1 → stop(1=ALLOWFADEOUT)`；主事件 [+0x10]==1 → stop(1)；`[+0]=0`。
- **get_timeline_beat_properties @0x1408b7620**（O 手解）：**发布器**——输入结构
  {i32,i32,f32,i32,i32} → 写 6 个进程全局 0x143634c68/c70(f32)/c78/c80/c88/c90
  （c88/c90 running max；c90=update 每帧清零的节拍计数）。现有代码 `->(i32,f32)` 签名错。
- **init @0x1408b63c0**（O）：3×instantiate_audio（`event:/Music/mus_GladePreview`×2 +
  `event:/Amb/GladePreview`）+ start（串 0x142ad6318/0x142ad6350）。
- **pause @0x1408b7490**：set_paused(true)（曲目+主事件）。

### 2.3 AudioLoopPlayer（wall_audio_utils.rs @0x1409285a0，O）

`{instances Vec cap-first, 参数 Vec @+0x18, current usize@+0x28, should_play u8@+0x38}`（stride 0x40）。
`request_play`：+0x38=1；`+0x28 < +0x10` 否则 panic（index out of bounds，wall_audio_utils.rs）；
取 instances[current] → set_position → set_parameter("editing_speed") → set_volume。

### 2.4 颜色资源（color_icon / color_id / color_id_roof）

- **ColorMenuBrightnessValue::to_f32 @0x140b06770**（O）：三变体无载荷枚举判别→f32 表
  0x142ae785c `{0:0.25, 1:1.0, 2:4.0}`；`default_f32 @0x140b06780` = 1.0（常量 0x142925904）
  → 缺省变体=1(Normal)。现有 `{Default,Custom(f32)}` 错误。
- **color_id_roof::impl$0::default_value @0x14099ca50**（O）：**eax=1** —— 屋顶默认 ColorId=1
  （wp16 合同 ColorId(0) 冲突）。

### 2.5 LUT（grading_lut / gpu_lut）

- **load_cube @0x140b01050**：见 §1.4（PDB 名 asset_name 错位）。输出 GradingLut
  `{name String@+0, size u32@+0x18, 纹理句柄区@+0x28..+0x58}`。
- **parse_rgb @0x140b01cc0**（O）：&str → 空白切分 → 3× f32 解析（0x140d8ba40）。
- **GpuLut::build @0x14090e080**（O）：Vec::with_capacity(8)；条目 stride 0x30
  `{+0 u64,+8 u64,+0x10 u64,+0x18 self,+0x20 vtable,+0x28 rendered u8}`；非 demo
  （全局 0x143634d90==0）加条目 2 → insert_resource<GpuLutTracker>。
- **render_luts @0x14090e280**（O）：逐条目 rendered==0 → [0x10]=0 → vtable[+0x28] → rendered=1。
- **compute_impl @0x14090e310**（O）：vtable[+0x18] 取 URI → FUN_141600570(3,self,vtable,…)
  装 400B ComputePass → memcpy → 计算。

### 2.6 截图/报告（feedback_report_resources.rs）

- **is_available @0x140909860**（O 手解）：`byte@+0x36 < 7`（背景纹理可用性枚举，7=不可用）。
- **clear_screenshot @0x140909930**（O）：状态@+0x3e `min(state-7,2)` 分派：
  state==7(空)→跳过；==8→锁 [+8] Arc<Mutex> 清纹理；清截图字节。
- **feedback_report_texture_receive @0x140909ab0**（O）：ResMut + EventReader（tick 回写）；
  有事件 → 状态==8 分支：锁 Mutex → 更新纹理内部（FUN_140c81e10）→ 拷 6 qword 元数据
  到 +0x10..+0x38 → 纹理内部 +0x4e=7、资源 +0x3e=7 → 换 Arc。
- **0x140909870（PDB FeedbackReportTags::to_list 错位）**（O）：4 标志字节 →
  羊叫串列表 {"aww"(0x142aaae08,3),"think"(0x142aaae0b,5),"angry"(0x142aaae10,5),
  "sob"(0x142aaae15,3)} 经 from_iter 闭包入 Vec<String>。

### 2.7 Steam（systems/steam + resources/steam.rs）

- **update_steam_inputs**：见 §1.3。22 块；帧计数（FUN_140d4d740/0x140d64e00）+ PrevSteamInputState
  帧差同步 + 31 处 FUN_140b5fff0（部分 prev 状态同步路径）。
- **action_handle @0x1417c3720**（O）：动作名表（0x142ce2049 连续串）：动作集 "GameControls" +
  28 数字/模拟动作（CameraPan、cursor_move、camera_height、left_click、right_click、undo、redo、
  tool_building、tool_path、tool_decorator、tool_customize、tool_eraser、tool_terrain、tool_garden、
  tool_tree、tool_water、time_of_day、photomode、toggle_snapping、zoom_in、zoom_out、main_menu、
  ui_cancel、ui_confirm、ui_up、ui_left、ui_right、ui_down）；菜单集 "Menu"：ui_tab_prev/ui_tab_next；
  另两组 "CottageWindow"/"GothicWindow"（窗口层，I）。
- **add_steam_integration @0x140945320**（O）：插件 build —— 0x30B 系统配置 {1,1,0,0,8,0}
  （8 系统批量注册）。
- **startup_steam @0x140a9bbf0**（O）：首帧（全局帧计数 0x140d4d740）一次性
  FUN_1412e0660 插入 Steam 启动资源束（6 qword 常量 0x142adafb0）；0x142923700/0x1429236e0
  = steamworks 客户端 init 族。
- **AppSteamIntegration::user_id @0x140ae96f0**（O）：每次现算 `self.client.user().steam_id()`
  （Arc 临时 clone+drop）u64。`client() @0x14067dd80` = `mov rax,rcx`（字段@+0）。
- **is_floating_gamepad_text_input_open @0x140ae9470**（O）：读进程全局字节 **0x143634d94**
  （回调缓存，非 API）。
- **input_show_binding_panel @0x140ae9480**（O）：客户端非空 → ShowBindingPanel(handle)。
- **open_overlay_or_browser_to_webpage @0x14026d3c0 / <String> @0x14026d580**（O）：overlay 可用
  → 开 overlay；否则浏览器（@0x14026d080）。can_use_overlay/validate_param @0x140aef1a0。

### 2.8 杂项资源

- **WindowSize::as_vec2 @0x1400885d0**（O 手解）：`mov rax,[rcx]` = 头 8B 直接当 Vec2（f32×2）。
  0x140949c40（PDB aspect_ratio 错位）= 析构例程（cap-first Vec<stride 0x40> 双指针释放 +
  i64::MIN 哨兵；xrefs 唯一 = 0x14368e2fc DATA = 静态析构注册表）。
- **ShaderRefactoringState::disabled @0x140b1f210**（O）：`[+0]==0 → bool` —— `fn disabled(&self)->bool`
  （非构造器）。
- **ShutdownGame::initiate_game_shutdown @0x140b2ba40**（O）：bevy 系统（EventReader 双缓冲 +
  Commands::insert_resource）。现有代码 Vec 参数模型错。
- **NameHash::compute @0x140c9cd80**（O）：xxh3_64_with_secret(secret, 0xc0, seed=0)。
- **MetaCommander**：peek @0x14091f0c0（VecDeque ring 头元素 ptr/None）、consume @0x14091f100、
  is_profiling @0x14091f0f0 = `[+0x38]!=0`（hashbrown 表语义 U）——wp15 合同一致。
- **gothic_window_profile**：曲线函数入口 size∈{1,2,3} 分派（0x1409a4cc0/0x1409a4e70），
  WP21 点列转录未重验。
- **human_scale_ref @0x1408b2b30**（O）：5 参系统；`*param_5+0xf8` 调试开关门禁；
  `[*param_2+0x110]` u32 模式跳表（0x142a9f3b0）→ 各模式画法。
- **debug_viz_threshold_curve @0x140a8b0c0**：panic "assertion failed: w.simplified_curve_2d.is_valid()"
  （0x142ada193）；spatial_hash_debug @0x1408ec0e0：两 assert（wall_spatial_hash.rs）。
- **music_debug_gui @0x14083c4a0**（O）：egui 窗（Res<EguiContextResource>, ResMut<MusicAudioState>,
  ResMut<DebugGui>），读 [+0x100] 开关 + MusicPlayer+0x1c0 richness。

### 2.9 音频事件系统（补全，multi-baseline 已证部分不重复）

- path_audio_init @0x1409d08d0：`CobblestoneDraw` + **`Path Draw`(0x142ac1660)** +
  **`Erase Draw`(0x142ac1690)** + `PathWaterDraw/PathWaterErase`。
- signifier init @0x1409d6320：`UI Boop`、`Cursor/Grab`、`Cursor/Release`、
  `UI/HoldTimeSiginfier`（原版拼写）、`UI/UiRotateGizmoMovingBack`；event:/UI/camera/FirstPersonOn|Off+Tool。
- water init：`WaterDraw/WaterErase/IceErase`；roof init @0x140aa2fd0：
  `RoofShape/RoofTipMove/InterRoofMerlonsRearrange`；elevation_supports @0x140ad51c0：
  `WoodenBracketsAppear/BrickColumnAppear/WoodenColumnAppear`；terrain_stairs：
  `event:/SFX/StairDraw_OneShot`(0x142c62e70)。
- add_audio_events @0x140941450（PDB 名 add_audio_startup_systems 错位）= 16× register_event；
  add_audio_systems @0x1409411a0 = music_debug_gui + 两个 run_if 包。
- wall_audio_ui 事件（0x142a9f730 区）：`event:/SFX/ChangeWallStyle`、`Lantern/TurnOn`、
  `Chimney/ToggleSmokeOn|Off`（按 [+9] 字节二选一）、`BuildingSupportsRemoveWooden`、`WallSplit` 等。

### 2.10 音频事件系统增量（round2 REVIEW，O/I 混合）

- **WallAudioState.loops = 固定 10×AudioLoopPlayer**（stride 0x40，+0/+0x40/…/+0x240）+ 参数
  Vec @+0x48/0x88/0xc8/0x148 + 应播标志 @+0x38+0x40k + 音量 @+0x280（O；非 `Vec<AudioLoopPlayer>`）。
- **water_ambience**：按水网格装配多个 AudioLoopPlayer（from_iter<Map<Range,…>> over grid）+
  get_parameter_by_name + `insert_resource<WaterAudioGrid>`；常量 0.015625/42.5；
  load_index_unchecked @0x1409ebe80（O 名）。
- **music_system**：richness 三档 snap-off 0.24/0.49/0.74（multi-baseline §4 O）。
- **wall_audio_orchestrator @0x141234bf0**：RequestWallMaterialAudio 协调（hashbrown
  hash_one<GlobalId> 查表 + MaterialStats::max_pillar_movement / which_material_changed_the_most +
  EventWriter<PlayWallAudio> + ArrayVec<WallSound,4>::push）。
- **trigger_audio @0x1409eafa0**：audibility ±0.5/−0.1 夹取、音量位移/阈值×0.0125、
  z>=11 或 dist<=70 停 InForest（wp14 合同全量）。
- **menu_audio**：13 kind 跳表 0x142ac706c + AppMode `(mode|8)==0xa` 门禁（O）。
- **playermade_stairs**：kind≤18 跳表 0x142bad9f0（O 全量）；**sheep**：kind 0-7 表 0x142ac2cc8 +
  Aww/Think/So/Angry/Idea/Eyes/Confused 参数（O）；**clutter**：39 kind 表 0x142aa1688 +
  Cosmetics 串表 0x142aa0e4c（O）。
- **game_loading_audio**：snapshot:/Loading + event:/UI/Loading + 状态→2 停 snapshot + 1s 定时器（O）。
- **firstperson_steps**：GroundType::to_fmod_param + FirstPersonOff + 双线性地形采样（O）。
- **decorator_audio**：稳定排序 `(f32,Vec3)` 距离衰减 + `Cosmetics/Clutter_Rotate`、
  `Cosmetics/MoveGround`（O）。
- **audio_events 事件载荷布局**：PlayStairsAudioCmd/PlayWallAudio @+0xc..0x2c f32 族 /
  @+0x1c u64+u32 位置 / @+0x20 byte（O）。

---

## 3. 状态机汇总

| 状态机 | 判别/字段 | 转移 |
|---|---|---|
| AudioInstance | u64@+0：0/1/2 | Disabled→Active(start/load)；Active→Disabled(stop 清零 state@+0x28)；Err 带 anyhow |
| MusicPlayer | u8 state@+0x118 ∈ 0..4 | 0=switch_track；A/B 表 4 case（超时/节拍/回卷）；A3 节拍同步→4 |
| 音乐节拍全局 | 0x143634c68（是否节拍）0x143634c90（计数&3） | A3/B3：==1 且 &3==0 → state=4+start |
| Steam 浮动键盘 | 进程字节 0x143634d94 | 回调缓存 0/1 |
| FeedbackReportResources | +0x3e 状态字节 7=空/8=收到 | clear_screenshot 按 min(state-7,2) 清理 |

---

## 4. 常量表

| 地址 | 值/内容 | 用途 |
|---|---|---|
| 0x142ae785c | f32 {0.25, 1.0, 4.0} | ColorMenuBrightnessValue 判别→值 |
| 0x142925904 | f32 1.0 | ColorMenuBrightnessValue::default_f32 |
| 0x142ad0460 | forward(0,0,-1) up(0,1,0) velocity(0,0,0) | set_3d_attributes |
| 0x142aa159c | 颜色 u32 {0xff0000ff, 0xffff00ff} | ScreenLog 环着色（backend 共用） |
| 0x143634c68..c90 | 6×i64/f32 音乐节拍发布全局 | get_timeline_beat_properties 输出 |
| 0x143634d90 | 进程布尔（非 demo） | image_formation 档2 + gpu_lut 双条目门禁 |
| 0x143634d94 | 进程字节 | Steam 浮动键盘状态 |
| 0x143634d60/d80 | 报告注册表槽 | set_parameter/set_position 失败 |
| 0x14362c3a8 | verbosity 全局 | >4 trace/>3 debug/>1 info |
| 0x14362c420 | panic 进行中标志 | 报告缓冲抑制（history 域同读） |
| 0x142ad2870/28c8/2918 | 听者三断言串 | AudioCoreInner::update 断言 |
| 0x142ad25b0 | "Error loading FMOD bank file: " | Bank::load 失败 fmt |
| 0x142ae6d00/6d40/6de8/6db8 | DOMAIN_MIN/MAX/LUT_1D/Too many 串 | load_cube 错误 |
| 0x142ae6e20 | "assets/luts" | GradingLuts::load 目录 |

---

## 5. 未决（U）

- MusicPlayer A1/A2/B1/B2 阈值常量语义（0x142ad01cc 区浮点）；0x1408b5b52 后精确动作。
- update_steam_inputs 码→动作名配对（需 ui-input 枚举）。
- FeedbackReportResources 元数据 6 qword 各字段（宽/高/格式/句柄）。
- 0x140949c40 析构例程宿主类型；FeedbackReportTags::to_list 模块归属。
- gothic_window_profile 点列常量未重验。
- **DebugSphere::radius**（country_core::utils::debug_viz，w9 分片）：builder 形态待核
  （debug_viz 阈值曲线章节按 builder 语义补证，地址 U）。
