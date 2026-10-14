# audio-misc 域 DIGEST（country-core 音频 / Steam / 调试 / 杂项资源）

- 版本：B patched（`workspace/input/tiny-glade.exe`，基址 0x140000000）；数据库 `tg_a8`。
- 证据：批量反编译 154 目标（136 ok / 18 NotFound，NotFound 均用 capstone 直读机器码补齐，
  见 `tools/disas.py`）+ rne-read 字符串 + 复验 `evidence/multi-baseline/audio-events-analysis.md`、
  `audio-extra-analysis.md`（radare2/reva 交叉基线，本批与其一致处不再重复原文）。
- 产物：`decomp/decomp-<va>.txt`（保留）、`summary.json`、`batch-job.json`、`tools/disas.py`。

## 1. 模块清单与过程数

| 文件域 | 过程数 | 状态 |
|---|---|---|
| resources/audio_resources.rs | 13 | 核心 9 个全部 O 精读；AudioCore::new/new_impl 未在本批（WP14 已录） |
| resources/audio_settings.rs | 0（无业务过程） | 布局 O（audio_update 证据） |
| resources/color_icon.rs | 17 | 2 个业务小函数 O 手解；15 个 register_required_components = bevy 注册胶水（TypeId 查表，同构样板） |
| resources/color_id.rs / color_id_roof.rs | 2 | color_id_roof::default_value O（**=1**，纠错见 §7） |
| resources/feedback_report_resources.rs | 6 | 5 个 O 精读；1 个（0x140909870）归属/语义纠错见 §7 |
| resources/gothic_window_profile.rs | 2 | 曲线函数入口 O（size 1/2/3 分派）；点列常量已在现有代码（WP21 逐 f32 转录，未重验） |
| resources/gpu_lut.rs | 5 | build/render_luts/compute_impl O 精读 |
| resources/grading_lut.rs | 5 | load_cube（.cube 解析器）O 全量精读；parse_rgb O；load（readdir+双 Vec）O |
| resources/image_formation.rs | 4 | 全部 O 手解（机器码与现有 WP21 代码一致） |
| resources/shutdown.rs | 1 | O 精读 |
| resources/window.rs | 2 | as_vec2 O 手解；0x140949c40 名/实错位见 §7 |
| resources/window_lights.rs | 1 | register_required_components = bevy 胶水 |
| resources/steam.rs | 4 | action_handle（动作名表全量 O）、try_load 族 |
| systems/audio/*（31 模块 + music/*） | ~80 | 已由 multi-baseline 两份分析覆盖的（wall/building/stairs/music/terrain_stairs/water_ambience/menu/sheep/clutter）本批抽验一致；其余（garden/path/water/roof/tree/forest/replay/game_loading/signifier/trigger/ivy/elevation/timberframe/firstperson）本批字符串级补全 |
| systems/steam/* | 13 | 6 个 O 精读（add_steam_integration/startup_steam/update_steam_inputs 结构级/action 表全量） |
| systems/debug/* | 3+2 | human_scale_ref O 结构；debug_viz_threshold_curve/spatial_hash_debug panic 串 O |
| 其余 debug（ui/debug_visualize_volumes、ui_clutter/debug_spawn_clutter、render_loop/primary_rt_debug） | 7 | 结构级（debug 工具面，未深挖） |
| music 域 | 9 | MusicPlayer::update 状态机 O 结构；stop_immediately 名/实纠错；get_timeline_beat_properties 行为纠错（见 §7） |

## 2. 布局表（O = 机器码直接证据）

### 2.1 AudioInstance（resources/audio_resources.rs）

判别 u64 @+0：`0=Disabled`、`1=Active`、`2=Err(anyhow)`。Active 布局（0x140a54440 写序 O）：

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x0 | 判别 u64 = 1 | `*param_1 = 1` |
| +0x8 | `*mut FMOD EventInstance`（8B 句柄） | `param_1[1] = create_instance 结果`；start/stop 用 `rcx+8` |
| +0x10 | name String cap（cap-first） | `param_1[2] = name_len` |
| +0x18 | name ptr | `param_1[3] = alloc` |
| +0x20 | name len | `param_1[4] = name_len` |
| +0x28 | state u8（初值 2） | `*(param_1+5) = 2` |

- `start`（0x140a57240）：`[+0]==1` → `EventInstance::start`（libfmod 0x1414372d0），错误为
  FMOD Result 判别 `0x8000000000000004`（= niche 变体 4 = Ok；PLAYBOOK 判别规则）。
- `stop`（0x140a57470）：`[+0]==1` → `stop(0)` = STOP_IMMEDIATE（0x141437440，r8d=0）。
- `start_if_not_playing`（0x140a57300）：`[+0]&1` → get_playback_state → **bitmask 0xb**
  （state∈{0,1,3} = Playing/Sustaining/Starting 时跳过）→ 否则 start。
- `set_position`（0x140a57da0）：`set_3d_attributes`，常量（.data 0x142ad0460 O）：
  forward=(0,0,-1)、up=(0,1,0)、velocity=(0,0,0)。失败 anyhow `"Failed to set 3d attributes
  for FMOD event "`（0x142ad2d40）。
- `0x140a57540`（5B）：`[+0x28]=0`（state 清零；I 语义，疑 stop 收尾/set_state）。
- `set_parameter`（0x140a57670，**PDB 名 pause 错位**）：签名 `(&self, name:&str, f32)`，
  `set_parameter_by_name`。失败走 §2.6 全局报告机制（槽 0x143634d60）。
- `update`（AudioCoreInner，0x140a53a00）：听者校验三断言 →
  `set_listener_attributes(listener=0)` → `Studio::update`（失败 log `"FMOD update error: "`）。
  断言原文：`(listener.forward.length()-1.0) < 0.001`、`(listener.up.length()-1.0) < 0.001`、
  `listener.forward.dot(listener.up).abs() < 0.001`（0x142ad2870/0x142ad28c8/0x142ad2918）。

### 2.2 Bank / DelayedBankLoader / AudioSamplesLoadedPromise

- `Bank::load`（0x140a53640）：`loaded[+0]&1` 早退；`Studio::load_bank_file(path@+0x10)`；
  失败 → **panic** `"Error loading FMOD bank file: {path} {err}"`（fmt 0x142ad25b0）。
  成功 → loaded=1 + 句柄@+8。
- `Bank::load_sample_data`（0x140a53870）：日志 `"Loading sample data of FMOD bank "`。
- `AudioSamplesLoadedPromise::recv`（0x140a57100）：crossbeam 三通道（array/list/zero）
  `recv_timeout(1_000_000_000 ns=1s)`；超时丢弃 payload 正常返回；通道断开 → panic
  `"called Result::unwrap() on an Err value"`（位置 audio_resources.rs）。
- 0x140a53a00 的 PDB 名 `DelayedBankLoader::load` **错位**（实为 AudioCoreInner::update）。

### 2.3 全局报告注册表（模块级业务机制，set_parameter/set_position 共用）

- 每函数一个独立槽：`0x143634d60/d58`（set_parameter）、`0x143634d80/d78`（set_position）。
  失败流程 O：`if 槽!=3` → 注册（0x1428d8820）；返回 1 → panic `"Mutex was poisonned"`
  （0x142ad2a48，19B，原文双 n）；否则拼字符串
  `"module_p_" + 错误fmt + "path!(…" + 参数名 + "$crate :: Level :: Warn" + err`
  （模板 0x142ad2a5b/0x142ad2a63、字面量 0x2928202168746170=`path!(!`、0x6e726157203a3a20=` :: Warn`）
  → 0x14143b8b0 入表；verbosity（全局 0x14362c3a8）>1 时 log；
  `0x14362c420` 全局（非 0 且 `FUN_1428d86b0()==0`）→ 写 name 缓冲第 2 字节=1；
  `LOCK; c=*p; *p=0; UNLOCK`（消费标记）；c==2 → 释放缓冲。

### 2.4 MusicPlayer（music/music_player.rs）

- 字段（O 偏移）：`[+0]` 当前曲目使能、`[+1]=+0x8` 曲目下标、`[+2]=+0x10` 主事件使能、
  `[+3]=+0x18` 主 fmod 事件、`[+8]=+0x40` 事件状态、`[+9]=+0x48` fmod 事件、
  `[+0x1e]=+0xf0` track Vec ptr、`[+0x1f]=+0xf8` track Vec len、`[+0x23]=+0x118` 状态机 state、
  `[+0x25]=+0x128` last_position。曲目 stride 0x30，`[+0]==1` 使能，`[+8]` 事件句柄。
- `update`（0x1408b5500）：读当前曲目 timeline position → 状态机（跳表 0x142a9fa34 与
  0x142a9fa24，按 `pos<last_position` 选表；Ghidra 未能展开全部 case，转移语义 U）；
  state==0 → `switch_track`；清零全局节拍计数 `0x143634c90`。
- `stop_immediately`（0x1408b72e0，**PDB 名 current_track 错位**）：遍历全部曲目
  `[+0]==1` → `stop(1)`（ALLOWFADEOUT）；主事件 `[+0x10]==1` → `stop(1)`；`[+0]=0`。
- `get_timeline_beat_properties`（0x1408b7620，NotFound 手解）：**不是 getter** —— 把输入
  结构 {i32,i32,f32,i32,i32} 写入 6 个进程全局 `0x143634c68/c70(f32)/c78/c80/c88/c90`
  （c88/c90 为带一次修正的 running max；c90 = MusicPlayer::update 每帧清零的节拍计数）。
  现有代码 `-> (i32,f32)` 签名错误（见 §5）。
- init：3×instantiate_audio（`event:/Music/mus_GladePreview`×2 + `event:/Amb/GladePreview`）
  + `start`（O 串 0x142ad6318/0x142ad6350）。曲库 `event:/Music/Halloween/*`（WP14 已录）。

### 2.5 AudioLoopPlayer（wall_audio_utils.rs，0x1409285a0 O）

`{instances Vec cap-first {cap@+0, ptr@+8, len@+0x10}, 参数 Vec @+0x18?, current usize@+0x28,
should_play u8@+0x38}`（stride 0x40）。`request_play`：`+0x38=1`；`+0x28 < +0x10` 否则
panic（index out of bounds，位置 wall_audio_utils.rs）；取 `instances[current]` →
`set_position` → `set_parameter("editing_speed")` → `set_volume`。

### 2.6 ColorMenuBrightnessValue（color_icon.rs，NotFound 手解 O）

- `to_f32`（0x140b06770）：**判别式索引 f32 表**（0x142ae785c O）：
  | 判别 | 值 |
  |---|---|
  | 0 | 0.25 |
  | 1 | 1.0 |
  | 2 | 4.0 |
  三变体无载荷枚举（Quarter/Normal/Quadruple 语义 I）。现有代码 `{Default, Custom(f32)}`
  **错误**（见 §5）。
- `default_f32`（0x140b06780）= 1.0（常量 0x142925904）→ 缺省变体 = 1（Normal）。

### 2.7 ImageFormationConfigs（image_formation.rs，NotFound 手解 O）

现有 WP21 代码与机器码**逐条一致**（无需改）：`current_mut`/`get` = `self + (i<<6)`；
`iter_non_default_mut` = {begin, begin+0xc0, 0}；`switch_kind` = `[+0xc0]=dl`。

### 2.8 FeedbackReportResources（feedback_report_resources.rs）

- `is_available`（0x140909860，NotFound 手解）：`[rcx+0x36] < 7` —— 状态字节 <7 即可用；
  现有代码把 `FeedbackReportBgTexture` 建模成独立 bool 资源**错误**（见 §5）。
- `reset_choices`（0x140909910）：`[+0x40..+0x46] = {0,1,1,0,0,0}`（现有代码 O 一致）。
- `clear_screenshot`（0x140909930）：读状态 `[+0x3e]`，`min(state-7,2)` 分派：
  ==0（state=7 空）跳过；==1 锁 `[+8]` Arc<Mutex>；==2 另一路径（func_0x00014003bb80 +
  FUN_140be24b0）；清空截图/纹理。
- `impl$3::build`（0x140909a70）：bevy 插件 build —— TypeId
  `{0x8e1bd251962ba784, 0x6a782374c0e6529b}`（FeedbackReportResources）查组件表 +
  注册默认资源；缺失 → panic（fmt 0x142af2cc0）。
- `feedback_report_texture_receive`（0x140909ab0）：bevy 系统
  `(ResMut<FeedbackReportResources>, EventReader<…>)`（ResMut 判别：`param[2] = param+0x1c`
  tick 回写 O）；有事件 → 按状态字节替换 `[+8]` 纹理 Arc（LOCK dec + drop）→ 把事件字段
  拷入 `[+0x18..+0x36]`（截图状态区）。
- 0x140909870（**PDB：FeedbackReportTags::to_list**）：机器码实为 **4 标志字节 → 羊叫
  字符串列表**装配：`"aww"(0x142aaae08,3)/"think"(0x142aaae0b,5)/"angry"(0x142aaae10,5)/
  "sob"(0x142aaae15,3)`，经 from_iter 闭包入 Vec<String>（见 §7 错位）。

### 2.9 GradingLut / GradingLuts（grading_lut.rs，O 全量）

- `GradingLut` 输出布局（load_cube 写序 O）：`{name String @+0（cap-first，来自调用者
  param_5），size u32 @+0x18（+0x1c/+0x20 重复×3），纹理句柄区 @+0x28..+0x58}`。
- `load_cube`（0x140b01050，**PDB 名 asset_name 错位**）完整 `.cube` 解析器 O：
  1. 读文件（Result 检查）。
  2. 逐行：空行/`#` 注释跳过；`TITLE` 跳过；`DOMAIN_MIN` → parse_rgb，全 0 才合法
     （否则 anyhow `"DOMAIN_MIN must be 0 0 0"` 0x142ae6d00，上下文 `"Parsing DOMAIN_MIN"`）；
     `DOMAIN_MAX` 必须 `(1,1,1)`（`"DOMAIN_MAX must be 1 1 1"` 0x142ae6d40）；
     `LUT_1D_SIZE` → **`"1D LUTs are not supported"`**（0x142ae6de8）；
     `LUT_3D_SIZE` → 手写十进制 u64 解析（± 号、≤17 位、溢出检查；错误 0x142ae6d50）→
     `size³` 数组扩容（×4B，0 填充）；数据行 → parse_rgb → **RGB9E5 打包**
     （func_0x0001417c66b0）入数组；超过 size³ → **`"Too many entries in the LUT.
     Bailing before an overrun"`**（0x142ae6db8）；行数不足 size³ → 越界 panic
     （0x1428d9518，位置 grading_lut.rs）。
  3. 建 3D 纹理：`{size,size,size}` → func_0x000140bd5310（尺寸准备）→
     FUN_1416a6700(device, 名 `"grading lut"`(0x142ae6c70, 11B), 数据, `size³×4` 字节)；
     失败 → panic `"called Result::unwrap() on an Err value"`。
- `parse_rgb`（0x140b01cc0）：`&str` 行 → 空白切分迭代器（0x140b00920）→ 3× f32 解析
  （0x140d8ba40）；失败返回 Err。
- `GradingLuts::load`（0x140b01df0）：目录 **`"assets/luts"`**（0x142ae6e20）read_dir，
  失败 panic `"read_dir"`；逐 DirEntry 经 per-file 闭包（0x140ae5bb0，未反编译，按扩展名
  分派 load_cube/load_unwrapped，I）→ 双 Vec：A stride 0x28（5 qword）、B stride 0x38
  （7 qword）；GradingLuts = {A {cap,ptr,len}@0, B {cap,ptr,len}@0x18}（cap-first O）。

### 2.10 GpuLut（gpu_lut.rs，O）

- `build`（0x14090e080）：`Vec::with_capacity(8)`（cap-first）；条目 stride 0x30 =
  `{+0x0 u64, +0x8 u64, +0x10 u64, +0x18 self ptr, +0x20 vtable ptr, +0x28 rendered u8}`；
  条目 1 vtable 0x142ad6290；**非 demo（进程全局 0x143634d90==0）再加条目 2**
  （vtable 0x142ad6260）→ `insert_resource<GpuLutTracker>`。
- `render_luts`（0x14090e280）：逐条目 `[+0x28]==0` → `[+0x10]=0` → `vtable[+0x28](...)`
  → `[+0x28]=1`。
- `compute_impl`（0x14090e310）：`vtable[+0x18]` 取 URI 入 Vec →
  `FUN_141600570(3, self, vtable, …)` 装 400B ComputePass → memcpy → `vtable[+0x18]` 计算。
- 0x143634d90 与 image_formation 档 2 `+0x3d` 共用同一进程级布尔（跨域线索，见 §6）。

### 2.11 Steam（systems/steam + resources/steam.rs，O）

- `SteamMainActionSet::action_handle`（0x1417c3720）：动作名表（0x142ce2049 起连续串 O）：
  动作集 `"GameControls"`；数字/模拟动作 28 个：`CameraPan、cursor_move、camera_height、
  left_click、right_click、undo、redo、tool_building、tool_path、tool_decorator、
  tool_customize、tool_eraser、tool_terrain、tool_garden、tool_tree、tool_water、
  time_of_day、photomode、toggle_snapping、zoom_in、zoom_out、main_menu、ui_cancel、
  ui_confirm、ui_up、ui_left、ui_right、ui_down`；菜单集 `"Menu"`：`ui_tab_prev、ui_tab_next`；
  另两组 `"CottageWindow"`/`"GothicWindow"`（窗口层，I）。
- `add_steam_integration`（0x140945320）：标准 bevy 插件 build —— 0x30B 系统配置
  `{1,1,0,0,8,0}`（8 个系统批量注册，Arc 克隆链 + add_systems 族 0x14152f4e0/0x140b67280）。
- `startup_steam`（0x140a9bbf0）：首帧（全局帧计数 0x140d4d740 的 Instant 计数）一次性
  `FUN_1412e0660(commands, args, 位置串)` 插入 Steam 启动资源束（6 qword 常量 0x142adafb0，
  默认句柄）；0x142923700/0x1429236e0 = steamworks 客户端 init 族。
- `update_steam_inputs`（0x140ae9740，437KB/7947 行）：帧计数推进 → 每 Steam 动作一个
  同构块（~14+ 块，每块 `FUN_140b5fff0(动作状态, &"crates/…/steam/mod.rs")` = 动作取值 →
  写入游戏输入资源 + PrevSteamInputState 帧差）→ 纯 steamworks 胶水，逐块内容未深挖（U）。
- `is_floating_gamepad_text_input_open`（0x140ae9470，NotFound 手解）：读进程全局字节
  **0x143634d94**（回调缓存，非 Steam API 调用）。
- `input_show_binding_panel`（0x140ae9480）：客户端非空 → `ShowBindingPanel(action_handle)`。
- `AppSteamIntegration::user_id`（0x140ae96f0）：**每次调用现算** —— `self.client.user()`
  （Arc clone + 临时 drop）→ `user.steam_id()` u64。现有代码存字段的模型**错误**（见 §5）。
- `AppSteamIntegration::client`（0x14067dd80）：`mov rax,rcx`（client 字段@+0）。
- `open_overlay_or_browser_to_webpage`（0x14026d3c0）：overlay 可用 → 开 overlay，否则
  浏览器（0x14026d080/0x14026d580 为 &str/String 两泛型实例）。
- `validate_param`（0x140aef1a0）/`get_param`（0x140af0460）：steamworks Input 句柄校验
  包装（小函数）。

### 2.12 杂项

- `WindowSize::as_vec2`（0x1400885d0，NotFound 手解）：`mov rax,[rcx]; ret` —— 头 8 字节
  即 Vec2（两个 f32）。现有代码 `{width:u32, height:u32}` **错误**（见 §5）。
- `ShaderRefactoringState::disabled`（0x140b1f210，NotFound 手解）：`[rcx]==0 → bool` ——
  **方法不是构造器**：`fn disabled(&self) -> bool`。现有代码构造器模型**错误**（见 §5）。
- `ShutdownGame`：`initiate_game_shutdown`（0x140b2ba40）= bevy 系统
  （EventReader 双缓冲读 + Commands::insert_resource 语义），现有代码的 Vec 参数模型错误。
- `ColorId/RoofColorId`：`impl$0::default_value`（0x14099ca50，**color_id_roof.rs 归属**）
  = **eax=1** —— 屋顶默认 ColorId 是 1 不是 0（见 §5/§7）。
- `gothic_window_profile`：两曲线函数入口 size∈{1,2,3} 分派（0x1409a4cc0/0x1409a4e70），
  WP21 点列转录未重验（常量表为独立 .rdata 区）。
- `NameHash::compute`（0x140c9cd80）= `xxh3_64_with_secret(secret, 0xc0, seed=0)` =
  xxhash-rust `xxh3_64` 默认路径（现有代码复用正确）。
- `MetaCommander`：peek（0x14091f0c0，VecDeque ring 头元素指针/None）、
  consume（0x14091f100）、is_profiling（0x14091f0f0 = `[+0x38]!=0`，该偏移在 hashbrown
  表中语义未钉死 U）——与 wp15 合同一致。
- `human_scale_ref`（0x1408b2b30）：5 参 bevy 系统，`*param_5+0xf8` 调试开关门禁，
  `[*param_2+0x110]` u32 模式跳表（0x142a9f3b0）→ 各模式画法（debug 工具面）。
- `debug_viz_threshold_curve`（0x140a8b0c0）：panic 断言
  `"assertion failed: w.simplified_curve_2d.is_valid()"`（0x142ada193）。
- `spatial_hash_debug`（0x1408ec0e0）：两 assert panic（位置 wall_spatial_hash.rs）。
- `music_debug_gui`（0x14083c4a0）：`(Res<EguiContextResource>, ResMut<MusicAudioState>,
  ResMut<DebugGui>)` egui 调试窗，读 `[+0x100]` 开关字节与 `MusicPlayer+0x1c0` richness。

### 2.13 音频事件系统（本批对 multi-baseline 的补全/确认）

- 已复验（与 audio-events/audio-extra 分析一致，不重复）：wall kind 0–23 全表、building
  kind 6/7/8 + 尾部 0/1/2/3/4/9/5、stairs kind 0–18、menu kind 0–12、sheep kind 0–7、
  clutter kind 0–0x26、music richness/NewGladeTime 计算。
- 本批新增/纠正的串（O rne-read）：
  - path_audio_init（0x1409d08d0）：`CobblestoneDraw` + **`Path Draw`(0x142ac1660)** +
    **`Erase Draw`(0x142ac1690)** + `PathWaterDraw/PathWaterErase`（WP14 缺前两者名）。
  - signifier init（0x1409d6320）：`UI Boop`、`Cursor/Grab`、`Cursor/Release`、
    `UI/HoldTimeSiginfier`（原版拼写）、`UI/UiRotateGizmoMovingBack`；
    system 参数 `event:/UI/camera/FirstPersonOn|Off` + `Tool`。
  - water init：`WaterDraw/WaterErase/IceErase`（+IceDraw/PathWater×2 邻接表）。
  - roof init（0x140aa2fd0）：`RoofShape/RoofTipMove/InterRoofMerlonsRearrange`。
  - elevation_supports（0x140ad51c0）：`WoodenBracketsAppear/BrickColumnAppear/
    WoodenColumnAppear`（WP14 只列了 1 个）。
  - terrain_stairs：`event:/SFX/StairDraw_OneShot`（0x142c62e70）。
  - replay/game_loading/ivy/forest/tree/music 串与合同一致。
- `add_audio_events`（0x140941450）= 16× register_event（**PDB 名 add_audio_startup_systems
  错位**）；`add_audio_systems`（0x1409411a0）= music_debug_gui + 两个 run_if 包。

## 3. panic / 日志串清单（原文 + 地址，O）

| 地址 | 原文 |
|---|---|
| 0x142ad2870 | `assertion failed: (listener.forward.length() - 1.0) < 0.001` |
| 0x142ad28c8 | `assertion failed: (listener.up.length() - 1.0) < 0.001` |
| 0x142ad2918 | `assertion failed: listener.forward.dot(listener.up).abs() < 0.001` |
| 0x142ad25b0(fat) | `Error loading FMOD bank file: ` |
| 0x142ad0600 | `called \`Result::unwrap()\` on an \`Err\` value`（recv/start/stop/set_vca/建纹理共用） |
| 0x142ad2a48 | `Mutex was poisonned`（报告注册表锁失败；原文双 n） |
| 0x142ad2d88(fat) | `FMOD error: ` |
| 0x142c79970(fat) | `get_event(` |
| 0x142ad2a10(fat) | `Couldn't set global FMOD parameter ` |
| 0x142ad2d40(fat) | `Failed to set 3d attributes for FMOD event ` |
| 0x142ad29c0(fat) | `FMOD update error: ` |
| 0x142ad2600 区 | `Loading sample data of FMOD bank ` |
| 0x142ae6e20 | `assets/luts`（目录名）；0x142ae6e2b `read_dir` |
| 0x142ae6d00 | `DOMAIN_MIN must be 0 0 0`（ctx `Parsing DOMAIN_MIN` 0x142ae6cd0） |
| 0x142ae6d40 | `DOMAIN_MAX must be 1 1 1`（ctx `Parsing DOMAIN_MAX` 0x142ae6d10） |
| 0x142ae6de8 | `1D LUTs are not supported` |
| 0x142ae6db8 | `Too many entries in the LUT. Bailing before an overrun` |
| 0x142c799a0(fat) | `Parsing line `（+行号） |
| 0x142ae6c70 | `grading lut`（纹理调试名） |
| 0x142aab718 | `LUT` + gpu_lut.rs（compute URI 前缀） |
| 0x142ada193 | `assertion failed: w.simplified_curve_2d.is_valid()`（debug_viz_threshold_curve） |
| 0x142ad2a5b/0x142ad2a63 | `$crate :: Level :: Warn` / `: Level :: Warn`（报告模板） |
| 0x142a9f5d0 | `snapshot:/Track Switching`（music_player 日志参数） |
| 0x142ab2a50 区 | steam 插件 build 日志 fmt（add_steam_integration） |
| 0x142af2cc0 | feedback plugin build panic fmt（0x27=39B，资源缺失时） |

日志门控：全局 verbosity `0x14362c3a8`（>4 trace / >3 debug / >1 info / !=0 基本）。

## 4. 与现有合同/代码的差异（“代码写 X，证据为 Y”）

1. **ColorMenuBrightnessValue**（src/resources/color_icon.rs）：代码 `{Default, Custom(f32)}`、
   to_f32 {1.0, v}；证据 = 三变体无载荷枚举，判别→表 {0.25, 1.0, 4.0}
   （0x140b06770 + 表 0x142ae785c）。default = 变体 1（1.0）。
2. **ColorId 默认值**（src/resources/color_id.rs + wp16 合同）：代码/合同写
   `default_value=ColorId(0)`（O 0x14099ca50）；证据 = 该地址返回 **1** 且 PDB 归属
   **color_id_roof.rs**（sysres-procedures）。默认色索引语义需以屋顶侧=1 重审。
3. **WindowSize**（src/resources/window.rs）：代码 `{width:u32,height:u32}`；证据 =
   `as_vec2` = 头 8 字节直接当 Vec2 返回（f32×2，0x1400885d0）。0x140949c40
   （PDB aspect_ratio）实为析构例程（cap-first Vec<stride 0x40> 逐元素双指针释放 +
   i64::MIN 哨兵）——aspect_ratio 真实函数体 U。
4. **ShaderRefactoringState**（src/resources/shader_refactoring_state.rs）：代码
   `disabled() -> Self` 构造器；证据 = `fn disabled(&self) -> bool`（`[+0]==0`，
   0x140b1f210）。语义 = “是否处于禁用态”。
5. **MusicPlayer::current_track/stop_immediately 对调**：代码把 0x1408b72e0 当作
   current_track（返回 first）；证据 = 0x1408b72e0 遍历全部曲目 stop(1)+主事件 stop(1) =
   stop_immediately。current_track 真实地址 U。
6. **MusicPlayer::get_timeline_beat_properties**：代码 `(&self) -> (i32, f32)`；证据 =
   输入结构→写 6 个进程全局的**发布器**（0x1408b7620，全局 0x143634c68..c90）。
7. **FeedbackReportBgTexture**：代码 = 独立 bool 资源 `is_available(&self)->bool`；证据 =
   `[+0x36] < 7` 方法（0x140909860，宿主类型含状态字节@+0x36）。
8. **FeedbackReportTags::to_list**（0x140909870）：代码/合同 = 标签表过滤；证据 = 4 标志
   字节 → {"aww","think","angry","sob"} 列表（羊叫域；PDB 归属 feedback_report_resources
   与字符串证据冲突，见 §7）。
9. **AppSteamIntegration**（systems/steam/mod.rs）：代码 `{client, user_id:u64}` 存字段；
   证据 = `user_id()` 每次现算 `self.client.user().steam_id()`（0x140ae96f0，Arc 临时
   clone+drop）；`client()` = 字段@+0（0x14067dd80）。`is_floating_gamepad_text_input_open`
   = 进程全局字节 0x143634d94（非 API 调用）。
10. **initiate_game_shutdown**（resources/shutdown.rs）：代码参数为 `&mut Vec`；证据 =
    bevy 系统（EventReader 双缓冲 + Commands，0x140b2ba40）。
11. **audio_resources.rs 小差**：Active 变体 event 为 8B 句柄@+8（非 Gap@+8..）；name
    String cap-first {cap@+0x10, ptr@+0x18, len@+0x20}（现有代码注释布局近似但未给全）。
12. **wp14 audio-state 合同**：`AudioInstance::pause` 地址（行 562）与 `set_parameter`
    （0x140a57670）在 sysres PDB 名中互换（见 §7）；`DelayedBankLoader::load` 名挂在
    AudioCoreInner::update（0x140a53a00）上。Bank::load 失败 = **panic**（合同未记）。
13. **wp14 audio-systems 合同**：path_audio_init 事件名补 `Path Draw`/`Erase Draw`；
    elevation_supports 补 2 事件名；signifier init 5 事件名全量给出。
14. **wp16 window-lights 合同**：`impl$3::default_value → 默认 ColorId(0)` 与证据（=1，
    color_id_roof）冲突，见 #2。
15. **placement_data.rs**（resources/placement_data.rs 现有代码）：`to_affine3a` 的
    `from_cols_array` 末元素（16 元素第 16 位 = m33）写成 `self.position.z`。函数体机器码
    （0x140ad5020 尾段 4×movaps 到 [rsi]，末条 [rsi+0x30]=xmm1，xmm1=movups [rdi]=position
    含 z）显示**原版确实把 position.z 写进 w 行第 4 元素**——即现有代码是忠实转录而非
    笔误；w 行未被着色器/矩阵消费（消费面未定位），m33≠1 属原版怪癖，暂记 I。

## 5. 跨领域线索

- `0x143634d90`：image_formation 档 2 `+0x3d` 与 gpu_lut build 的“非 demo 二条目”门禁
  **共用同一进程布尔**（渲染域与 LUT 域同源开关）。
- `0x143634c68..c90`：音乐节拍全局块 —— music_player::update 每帧清零 c90，而
  get_timeline_beat_properties 发布（疑 UI/音画同步消费者，消费者未定位）。
- `0x143634d60/d58、d80/d78`：FMOD 失败报告注册表（audio_resources 内两槽）；与
  全局 verbosity `0x14362c3a8`、进程状态字节 `0x14362c420` 联动。
- `0x143634d94`：Steam 浮动键盘打开状态缓存（steam 域）。
- 音乐参数：music_audio_system（0x140a72810）arg 资源含 AppMode `(mode|8)==0xa` 门禁与
  `MusicRichness/NewGladeTime` 平滑（multi-baseline §4 O，未复验全部常量）。
- trigger_audio_system（0x1409eafa0）读 `AudioTriggerLibrary`（asset_libraries 域）与
  `Aabb3::intersect_segment`（shared-foundation utils）——wp14 audio-trigger 合同保留。
- gothic_window_profile 曲线被 window/decorator 域消费（本域只确认函数存在与分派）。

## 6. 错位 / 纠错汇总（机器码为准）

| 地址 | PDB/sysres 名 | 实际行为（O 证据） |
|---|---|---|
| 0x140a53a00 | DelayedBankLoader::load | AudioCoreInner::update（听者校验+set_listener_attributes+Studio::update） |
| 0x140a54810 | impl$6::instantiate_audio::closure$0 | AudioCoreInner::set_vca_volume（get_vca→set_volume, unwrap panic） |
| 0x140a57670 | AudioInstance::pause | AudioInstance::set_parameter(&str, f32)（参数面 + 全局报告） |
| 0x1408b72e0 | MusicPlayer::current_track | MusicPlayer::stop_immediately（全曲目 stop(1)） |
| 0x140941450 | impl$1::add_audio_startup_systems | add_audio_events（16× register_event，与 multi-baseline §1a 一致） |
| 0x140b01050 | GradingLut::asset_name | GradingLut::load_cube（完整 .cube 解析器） |
| 0x140909870 | FeedbackReportTags::to_list（feedback_report_resources.rs） | 羊叫 4 标志→{aww,think,angry,sob} 列表装配（模块归属存疑） |
| 0x140949c40 | WindowSize::aspect_ratio | 析构例程（Vec<stride 0x40> 双指针释放） |
| 0x140b1f210 | ShaderRefactoringState::disabled | `fn disabled(&self)->bool`（行为对、签名错：非构造器） |
| 0x14099ca50 | color_id.rs impl$3::default_value（wp16 合同） | color_id_roof.rs impl$0::default_value = 1 |

## 7. 未解决（U，不臆造）

- MusicPlayer::update 两跳表（0x142a9fa24/0x142a9fa34）case 语义与状态枚举变体名。
- AudioLoopPlayer 参数 Vec 精确字段与 AudioLoopDesc 全量（utils/audio_utils 属
  shared-foundation 域）。
- update_steam_inputs 逐动作→游戏输入映射表（14+ 同构块，纯 steamworks 胶水）。
- 0x140949c40 析构例程的宿主类型（PDB 归属 window.rs；疑 AvailableMonitors 或邻接
  符号边界错位）。
- 全局 0x14362c420 的语义（“正在报告/抑制”标志，I）。
- FeedbackReportResources 完整字段语义（+0x18..+0x36 区）。
- placement_data::to_affine3a 末元素（疑点见 §4 #15）。
