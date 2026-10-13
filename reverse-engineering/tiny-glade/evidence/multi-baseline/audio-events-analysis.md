# country-core systems/audio 事件→FMOD 胶水链分析（B 主基线，反汇编级证据）

2026-10-13。目标函数均为 B patched exe（VA 0x140001000 基址）。VA/大小取自
`evidence/multi-baseline/audio-inventory.json`（llvm-pdbutil 修正），源文件行号取自
`skeleton/B/crates/country-core/src/systems/audio/<file>.rs.pdb.jsonl.gz`（offsets 相对函数入口）。

证据来源（本次精读）：
- **reva**（B 持久库，worker `audio-owner`，脚本 `dump_linear3.py` / `read_jt.py` /
  `read_tables.py` / `resolve_names.py`）：13 个目标函数全线性反汇编（getCodeUnitAt 逐字节，
  补 disassemble 填满 PDB size 内的未定义区）、调用目标 PDB 符号名、.data 跳表/字符串表/常量读取。
- **radare2**（RNE 发行原件 oracle，worker `audio-r2`）：同一批函数全线性反汇编与 `hexdump`
  逐条核对，确认 B patch 未改指令；.data 字符串注释直接标在反汇编行。
- **PDB 行号表**：mod.rs(95–196)、audio_update.rs(11–54)、wall_audio_fmod.rs(40–…)、
  building_audio.rs(34–…)、playermade_stairs_audio.rs(28–…)、music/music_system.rs、
  music/music_player.rs。
- **反编译核对**（reva get-decompilation）：`AudioLoopPlayer::request_play`、
  `AudioCore::instantiate_loop`、`AudioCoreInner::instantiate_audio` 等 FMOD 胶水 consumer 的 ABI。

## 0. 总体：事件→FMOD 调用模式

每个音频事件系统（wall/building/stairs/…）的模式一致（与 process_tree_cmds 的
PlayTreeSoundCmd 同一骨架）：
1. **事件读取**：bevy Events 双缓冲（+0x8/+0x10、+0x28/+0x30 两个 Vec，+0x40 计数），
   事件槽 stride = 各事件 EventInstance 大小；游戏 payload 从槽 @+0x8 起（@+0x0 为 bevy 书签）。
2. **按事件 kind 字节分派**（槽 @+0x8；`.data` 32 位相对跳表）。
3. **构造 FMOD 调用**：kind→(事件名串/循环 player) → `instantiate_audio` / `instantiate_loop`
   / `AudioLoopPlayer::request_play` / `instantiate_audio_w_position_and_volume`，随后
   `set_position` / `set_parameter(name,value)` / `start_if_not_playing` / `set_volume`。
4. **帧尾**：按"应播"标志对循环 player `start_if_not_playing`/`stop`。

FMOD 边界：libfmod 方法（`libfmod::EventInstance::start/stop/set_paused/get_timeline_position/
set_volume/get_parameter_by_name`，0x141436fe0/0x141437160/0x1414372d0/0x141437440/
0x1414375b0/0x141436b40 等）与 bevy 框架调用（add_systems/register_event/EventReader/
hashbrown）只标注不复原。游戏自有消费点是 `country_core::resources::audio_resources`
（AudioCore/AudioCoreInner/AudioInstance=impl$10）与 `wall_audio_utils::AudioLoopPlayer`。

## 1. mod.rs：add_audio_events / add_audio_systems（事件注册骨架）

### 1a. add_audio_events（0x140941450，size 1495，源码行 179–196）
- 入口 ABI：`fn add_audio_events(app: &mut App)`（arg1 RCX→RSI；`lea rdi,[rcx+0x38]` 取
  Components 注册表地址）。
- 主体 = 16 段相同块：`movabs rdx/r8`（TypeId 两半）→ `get_resource_id`（0x1400dd6f0，
  `call_once<std::hash::random::impl$0::new::KEYS…>`）→ 查 `[rsi+0x218]`（组件表长度）/
  `[rsi+0x210]`（组件数组）→ 未注册则 `register_event<Type>`（0x14150xxx）。
- **注册顺序与事件类型（硬证据，16 个 register_event 调用）**：

| # | VA | register_event 类型 |
|---|---|---|
| 1 | 0x141514c40 | PlayTerrainSound |
| 2 | 0x141512d50 | SuppressPathSound（terrain_stairs_audio） |
| 3 | 0x141508c70 | PlayReplayAudio |
| 4 | 0x14151c800 | PlayDecoratorSoundCmd |
| 5 | 0x141521ea0 | PlayStairsAudioCmd（playermade_stairs_audio） |
| 6 | 0x141520c10 | PlayWallUiSound（wall_audio_ui） |
| 7 | 0x14150dce0 | PlayWallAudio（wall_audio_fmod） |
| 8 | 0x14151d460 | RequestWallMaterialAudio（wall_audio_orchestrator） |
| 9 | 0x14151b570 | WallBendAudioParamReset |
| 10 | 0x141509f00 | PlayTimberframeSound |
| 11 | 0x141515ed0 | PlaySheepReactSound |
| 12 | 0x14150d6b0 | PlayMenuUiSound |
| 13 | 0x141510830 | PlayBuildingAudio（building_audio） |
| 14 | 0x14150b7c0 | PlayElevationSupportSoundCmd |
| 15 | 0x141512720 | PlayClutterSoundCmd |
| 16 | 0x1415205e0 | PlayTreeSoundCmd（tree_audio，消费自 tree-tool） |

### 1b. add_audio_systems（0x1409411a0，size 680，源码行 95–142）
- 入口 ABI：`fn add_audio_systems(app: &mut App)`（arg1 RCX→RSI）。
- 两次 `add_systems`（bevy 依赖）+ 两个 `run_if` 条件构造：
  - 0x140b66c20 `add_systems<…, Update, void(*)(Option<Res<EguiContextResource>>, ResMut<MusicAudioState>, ResMut<DebugGui>)>` → **music_debug_gui**。
  - 0x140985d30 `run_if<…(Res<AppMode>, ResMut<WaterAudioState>, EventReader<WaterDrawEvent>, EventReader<WaterEraseEvent>, Res<CursorTerrainRaycast>, Res<ActiveSession>)>` → **water_audio_system** 的运行条件。
  - 0x1412ec640 `run_if<…(ResMut<WaterAmbienceAudio>, Res<WaterAudioGrid>, Res<MainCamera>, Res<UiSignifierStream>)>` → **water_ambience_audio_system** 的运行条件。
  - 0x140b67280 `add_systems<tuple$<>, Update, ScheduleConfigs<…>>` → 批量加入（含上面 run_if 包）。
  - 0x140941326：`mov byte [rbp+0x1d8], 0x26` + `intern<SystemSet>`（0x140b218a0）+
    `in_set_inner`（0x141453000）→ 某系统被放入 SystemSet（0x26）。
- 其余调用（chain_ignore_deferred 0x140b704a0/0x141451630、memcpy 0x142923c00、
  drop 0x140029880/0x1400d4d40、get_resource_id）均为 bevy 框架内部。
- 结论：mod.rs 只注册"系统与事件"，不直接构造 FMOD 调用；具体系统（wall_audio_fmod、
  building_audio、stairs 等）的注册点不在 add_audio_systems（见 add_audio_systems 的两次
  add_systems 只含 music_debug_gui / water 系列；其余音频系统由其他 mod 注册——unknown）。

## 2. audio_update.rs：audio_update_system / boot_audio_update_system（每帧音频更新）

### 2a. audio_update_system（0x1409c8dd0，size 1356，源码行 11–34）
- 入口 ABI（prologue 保存）：arg1 RCX→RBX=Res<AudioCore>，arg2 RDX→RSI=Res<相机变换资源>，
  arg3 R8→RDI=Res<AudioSettings>。
- 业务步骤：
  1. `are_scopes_on`（0x140c16a00）+ puffin scope（begin_scope 0x140c16d40 / end_scope
     0x140c170f0，scope 名串 0x142ac0857 = `audio_update_system::f` 源码位置）。
  2. **5 个 VCA 音量**：`set_vca_volume(AudioCore[+0x10], &"vca:/…", len, vol)`（0x140a54810）：
     | AudioSettings 字段 | VCA 名串 | r8d=串长 |
     |---|---|---|
     | [+0xf8] | vca:/Master（0x142ad2a97） | 0x0b |
     | [+0xfc] | vca:/Music（0x142ad2aa2） | 0x0a |
     | [+0xfc] | vca:/Music-DawnJingle（0x142ad2aac） | 0x15 |
     | [+0x100] | vca:/SFX（0x142ad2ac1） | 0x08 |
     | [+0x104] | vca:/Amb（0x142ad2ac9） | 0x08 |
     （每个 VCA 先判 `[rbx]` 非空；`movss xmm3,[r15+0xf8..104]` 取音量，r15=[rdi]=AudioSettings）
  3. **听者更新**：`[rsi]`（arg2 资源）+0xc0/+0xd0/+0xe0/+0xf0 的 3×3+偏移变换 →
     SIMD 计算 3D 位置/朝向到 `[rbp-0x48]` → `AudioCore::update`（0x140a53a00，rcx=[rbx]+0x10）。
  4. puffin scope 结束。

### 2b. boot_audio_update_system（0x1409c9320，size 1292，源码行 37–54）
- 入口 ABI：arg1 RCX→RDI=Res<AudioCore>，arg2 RDX→RSI=Res<AudioSettings>（**无 arg3**）。
- 与 2a 的 VCA 段逐条一致（同样的 5 个 set_vca_volume，同样的音量字段）。
- 听者更新：**不读相机**，构造固定默认位姿（`[rbp-0x40]` 0 化 + 0xbf800000(-1.0) +
  0x3f800000…），`AudioCore::update`。
- 结论：`audio_update_system` 运行期每帧（相机听者 + VCA 音量）；`boot_audio_update_system`
  启动期（默认听者 + VCA 音量）。两函数 VCA/听者逻辑几乎同构，仅听者来源不同。

## 3. wall_audio_fmod.rs：wall_audio_init_system / wall_audio_fmod（代表事件系统①）

### 3a. wall_audio_init_system（0x14094fb80，size 3480，源码行 40–94）
- 入口 ABI：arg1 RCX→RDI=Res<AudioCore>（`mov rdi,[rcx]`），arg2 RDX→RSI=App。
- 业务：**10 次 `AudioCore::instantiate_loop`**（0x140a59220），每次 desc 在栈 `[rbp+0x470]`：
  `{name_ptr, len, u64=3, u32=0x32}`（count=3、max=0x32(50)，经 instantiate_loop 反编译确认：
  desc+0x18→AudioLoopPlayer+0x30）。10 个事件名（RNE .data）与串长：

| # | 事件名（.data） | 串长 |
|---|---|---|
| 1 | event:/SFX/Wall Draw（0x142ab3f70） | 0x14 |
| 2 | event:/SFX/Wall Change Height（0x142ab3fd8） | 0x1d |
| 3 | event:/SFX/FenceExtend（0x142ab4010） | 0x16 |
| 4 | event:/SFX/LintelExtend（0x142ab4040） | 0x17 |
| 5 | event:/SFX/StoneArchExtend（0x142ab4070） | 0x1a |
| 6 | event:/SFX/WideningGate（0x142ab40a8） | 0x17 |
| 7 | event:/SFX/PlatformStairsExtend（0x142ab40d8） | 0x1f |
| 8 | event:/SFX/SewerGateWiden（0x142ab4110） | 0x19 |
| 9 | event:/SFX/FenceHeight（0x142ab4148） | 0x16 |
| 10 | event:/SFX/FenceMove（0x142ab4178） | 0x14 |

- 每个 loop 结果 0x38 字节（3 个 xmmword + 尾部字节）装入 WallAudioState 数组，最后
  `insert_resource<WallAudioState>`（0x1412df4a0，arg2=App，state 指针 = 栈上装配的 0x30×10）。
- WallAudioState 布局证据（system 侧读取，见 3b）：AudioLoopPlayer ×10 于 +0x0/+0x40/+0x80/
  +0xc0/+0x100/+0x140/+0x180/+0x1c0/+0x200/+0x240（stride 0x40）；元素参数 Vec 在 +0x88/+0xc8/
  +0x48/+0x148（ptr/cap/len，元素 0x30 字节）；每 loop 的"应播"标志 +0x38/+0x78/+0xb8/+0xf8/
  +0x138/+0x178/+0x1b8/+0x1f8/+0x238/+0x278。

### 3b. wall_audio_fmod（0x140950920，size 4698，源码行 134–…，主事件系统）
- 入口 ABI（prologue 保存 4 寄存器 + 栈参）：arg1 RCX→R14=Res<…AppMode>（`[r14]` u32
  `cmp dword [rax],0xd`＝13），arg2 RDX→R13=Res<AudioCore>（`[r13]`=AudioCoreInner），
  arg3 R8=EventReader<PlayWallAudio>，arg4 R9→RBX=ResMut<WallAudioState>（`[rbx]`=State）；
  **音量源为栈参**（entry_rsp+0x40 → `[rbp+0x188]` deref 后 +0x1c f32，divss /[0x1429d18cc]）；
  AppMode==13 时音量用常量 1.0（[0x142925904]）。（全量参数清单待钉死——unknown，疑 8 参 bevy 系统。）
- 事件迭代：bevy Events 双缓冲（`[rdi]`/`[rdi+8]`、`[rdx+0x18/0x38]` 计数）；事件槽 stride 0x28。
- **kind 分派**：`movzx ecx, byte [rax+8]` → 32 位相对跳表 0x142ab4998（kind 0–23 有效），
  分派目标覆盖 0x140950dce–0x140951480。
- 每种 kind 的行为（case 反汇编）分三类：

**A. 循环类（wall draw/move/height/width/extend/fence 等，`AudioLoopPlayer::request_play`）**
- request_play 反编译确认（0x1409285a0，源码 audio_utils.rs）：`self+0x38=1`；
  索引 `self+0x28` < `self+0x10`（cap）；取 `instances[idx]`（0x30 字节 AudioInstance）：
  `set_position` → `set_parameter("editing_speed",0xd,vol)` → 若实例存在
  `libfmod::EventInstance::set_volume`。
- 调用方传：rcx=loop player（state 各 offset），rdx=&position，xmm3=[事件+0x1c]（set_volume），
  [rsp+0x20]=xmm14（**计算音量**，来自音量源栈参 +0x1c 的 `/const` 归一化，见下），
  r8d=[事件+0x20]（request_play 未读）。

**B. 瞬发类（appear/merge/rearrange，`instantiate_audio_w_position_and_volume`）**
- 0x140aa3590 反编译：`core[+0]` 非空 → `AudioCoreInner::instantiate_audio(&out, core+0x10,
  name_ptr, name_len)`（0x140a54440）→ `set_position(&out,pos)` → `start_if_not_playing`
  → 若存在 `libfmod::EventInstance::set_volume(out,[rsp+0xb0])`。
- 事件名串表（.data 0x142ab41c8 起连续 blob，`(ptr,len)` 切片）：
  FenceAppear(0x142ab41dd)、FenceMerge(0x142ab41f3)、FenceRearrange(0x142ab4208)、
  LintelAppear(0x142ab4221)、LintelRearrangeHasBricks(0x142ab4238)、HasBricks(0x142ab4252)、
  LintelMerge(0x142ab425b)、StoneArchAppear(0x142ab4271)、PillarStoneAppear(0x142ab428b)、
  PillarWoodAppear(0x142ab42a7)、GateAppear(0x142ab42c2)、PlatformStairsAppear(0x142ab42d7)、
  WallSplit(0x142ab42f6)、SewerGateAppear(0x142ab430a)。

**C. 参数类（fence 元素参数，`set_parameter`）**
- 如 kind 2（0x140950f27）：先 request_play(state+0x80 loop) → 读 WallAudioState 元素 Vec
  `[state+0x88]`(ptr)/`[+0x90]`(cap)/`[+0xa8]`(len)，`inverse_lerp`（0x140c904a0）+ clamp →
  `set_parameter(elem, "FenceLength"(0x142ab41d2,len 0x0b), val)`（0x140a57670）；stride 0x30。
- 帧尾（0x14095145e 起）：10 个 loop 依"应播"标志 `start_if_not_playing`/`stop`（0x140a596b0/
  0x140a59430），随后清标志；另有 `[state+0x280]` 音量乘 `powf` 时间调制。
- 音量计算：`[r14](arg1)+0x1c f32 / [0x1429d18cc]`（divss）→ xmm14；`cmp [rax],0xd` 判定
  AppMode 走默认/自定义。

## 4. building_audio.rs：building_audio_init_system / building_audio（代表事件系统②）

### 4a. building_audio_init_system（0x1417d21a0，size 2344，源码行 34–…）
- 入口 ABI：arg1=Res<AudioCore>（`mov rdi,[rcx]`），arg2=App。
- 与 wall/stairs init 同构：多次 `AudioCore::instantiate_loop`（0x140a59220，
  desc{name_ptr,len,count=3,max=0x32}）。循环事件名表（.data 0x142ce4632 起）：
  event:/SFX/Building/BuildingResize（0x142ce4632，len 0x22）、BuildingRotateRectangle
  （0x142ce46a8）、BuildingRotateCircle（0x142ce46f0）、BuildingChangeHeight（0x142ce4730）、
  BuildingMove（0x142ce4770）、event:/SFX/PlasterChangeLoop（0x142ce47a8）、
  BuildingOverlapMove（0x142ce47e0）；
  参数名：editing_speed、BuildingSize（0x142ce49a0/0x142ce49ad）。
- 结尾 `insert_resource<BuildingAudioState>`（0x1412e1c40）。

### 4b. building_audio（0x1417d2ad0，size 5183，源码行 101–…，主事件系统）
- 入口 ABI：arg1 RCX→RDI=ResMut<BuildingAudioState>，arg2=EventReader<PlayBuildingAudio>，
  arg3=Res<AudioCore>（`[rbp+0x58]`=AudioCoreInner），arg4（R9→`[rbp+0x50]`）=
  Res<Building 数据表>（内容经 0x1405b0d70 `hash_one<RandomState,GlobalId>` 查 hashbrown，值 0x2a8）。
- 事件槽 stride 0x20。**PlayBuildingAudio 字段（payload 从槽 @+0 起，无 bevy 书签）**：
  id u64 @+0（hash 键 → 0x2a8 构建条目）、position Vec3 @+8（x,y u64 / z u32 @+0x10）、
  editing_speed f32 @+0x14、kind u8 @+0x18。
  `filter_map`（0x1412fe860 `from_iter<PlayBuildingAudio, FilterMap<…closure_env$0>>`）收集。
- kind 分派：`byte[slot+0x18]`；两表 0x142ce4c88/0x142ce4ca0 存 `(name_ptr,name_len)`：
  | kind | 串/长度 |
  |---|---|
  | 0–2 | 0x142ce48f8/0x142ce491a/0x142ce493a，长 0x22/0x20/0x1d |
  | 3–5 | 同三串，长 0x6/0xf/0xc |
  | 6–7 | 常量 0x6/0xf、0xf/0xc（unknown） |
  串内容（RNE）：`event:/SFX/Shapes/DetachFromGround`、`AttachToGround`、`PeakReached`。
- FMOD 序列：`instantiate_audio(&out, core+0x10, name_ptr, name_len)`（0x140a54440）→
  `set_position(inst, pos)`（0x140a57da0）→ `set_parameter("editing_speed",0xd,xmm10)` +
  `set_parameter("BuildingSize",0xc, inverse_lerp(area))`（0x140a57670，area 0x140c99520/
  0x140c97580 + inverse_lerp 0x140c904a0）→ `start_if_not_playing`（0x140a57300）→ drop。
- 构建条目字段（0x2a8 值内，相对条目基址偏移证据）：`[-0x290]` u64（须 ≥2）、`[-0x270]`
  f32（须 >0）、`[-0x48]` u32（area 形态 0/1/2）、`[-0x118]` 等（unknown 语义，部分确认）。
- 尾部（0x1417d2f77 起）：无事件路径构造默认 BuildingAudioState 并处理（逻辑与有事件路径对称，
  参数串同 0x142ce49a0/0x142ce49ad）。

## 5. playermade_stairs_audio.rs：stairs_audio_init_system / stairs_audio_system（代表事件系统③）

### 5a. stairs_audio_init_system（0x140f878d0，size 1992，源码行 28–…）
- 入口 ABI：arg1=Res<AudioCore>（`mov rdi,[rcx]`），arg2=App。
- 6 次 `instantiate_loop`（0x140a59220，desc{name,len,count=3,max=0x32}，同 wall_init）：
  event:/SFX/Stairs/StairMove（0x142bad500）、StairChangeHeight（0x142bad578）、
  StairChangeWidth（0x142bad5b8）、LadderMove（0x142bad5f8）、LadderChangeHeight
  （0x142bad630）、LadderChangeWidth（0x142bad670）。
- 结尾 `insert_resource<StairsAudioState>`（0x1412e04f0）。

### 5b. stairs_audio_system（0x140f880a0，size 3031，源码行 140–318，主事件系统）
- 入口 ABI：arg1=ResMut<StairsAudioState>（6 loop 于 +0x0/+0x38/+0x70/+0xa8/+0xe0/+0x118，
  stride 0x38），arg2=EventReader<PlayStairsAudioCmd>，arg3=Res<…元素表>。
- 帧首：6 个 loop `stop`（0x140a59430）。
- **PlayStairsAudioCmd 字段（事件槽 stride 0x38）**：
  kind u8 @+0x8；字节 @+0x9/@+0xa/@+0xb；f32 @+0xc/@+0x10/@+0x14/@+0x18；u64 @+0x1c
  （position x,y）；u32 @+0x24（position z）；f32 @+0x28/@+0x2c。
- kind 分派：`cmp r12,0x12` 跳表 0x142bad9f0（kind ≤18）；kind 0=StairMove 走循环路径。
- 瞬发事件名表（.data 0x142bad6c8 起）：StairPlace、StairDelete、StairsMerge、
  StairsMergeConfirm、LadderAppear、LadderPlace、StepsAppear、WalkwayAppear、
  StairSupportAppear、StairPillarAppear（ref 0x142bad6c8/6e4/701/71e/742/760/77d/79a/7b9/7dd，
  串长 0x1c/0x1d/0x1d/0x24/0x1e/0x1f/0x1d/0x1f/0x24/0x23）。
- FMOD 序列：`instantiate_audio`（0x140a54440，name=表项，arg3 资源 core+0x10）→
  `set_position`（0x140a57da0，position=事件+0x1c/+0x24）→ `set_parameter("stair_width"(0x142bad818,
  0xb),xmm11)` + 条件 `set_parameter("size"(0x142bad823,4),inverse_lerp)` → `start_if_not_playing`
  → drop；循环 kind 直接对 state 内 loop `start_if_not_playing`。
- 参数串表（0x142bad818 起连续）：stair_width、size、editing_speed、extend_amount、h_offset、
  avg_arch_supports、height_above_ground、in_water、stair_edge_length、is_stub_preview、move_all。

## 6. music：music_audio_init_system / music_audio_system / MusicPlayer::update

### 6a. music_audio_init_system（0x140a71f20，size 2288）
- 入口 ABI：arg1=App。
- `GladeAudio::default`（0x1408bf2c0）→ 3 次 `instantiate_audio`（0x140a54440）：
  event:/Music/mus_GladePreview（0x142ad6318）、mus_GladePreview 短名（0x142ad6325）、
  event:/Amb/GladePreview（0x142ad6350）→ `start`（0x1414372d0，FMOD）→
  `MusicPlayer::init`（0x1408b63c0）→ `insert_resource<MusicAudioState>`（0x1412e00c0）。
- 参数串表（0x142ad6398 起）：MusicRichness、NewGladeTime、PreviewGladeType。

### 6b. music_audio_system（0x140a72810，size 3143）
- 入口 ABI：arg1=Res<…时间/模式>（`[rsi+0x28]` 时间），arg2=ResMut<MusicPlayer>
  （`[r14]`=MusicPlayer），arg3=ResMut<MusicAudioState>（`[rbx]`=State），arg4=Res<AppMode>。
- 业务：richness 计算（time/常量 + clamp，State+0x1c4）+ `time_continuous_lerp`（0x140c904e0）
  平滑；`set_parameter(MusicPlayer 事件, "MusicRichness"(0x142ad6398), …)`（0x140a53db0）、
  `set_parameter("NewGladeTime"(0x142ad63a5))`；AppMode `|8==0xa` 时
  `MusicPlayer::update(player, [args 资源+0x58])`（0x1408b5500）。

### 6c. MusicPlayer::update（0x1408b5500，size 3406）
- `fn update(&mut self /*rcx*/…)`：若 track Vec 非空且 `[self+8]<[self+0xf8]`，取 track 条目
  （stride 0x30，`[+0]`==1）→ `get_timeline_position`（0x1414375b0，libfmod）→ 状态机跳表
  0x142a9fa34（按 `[self+0x118]` 状态）→ FMOD `start/stop/set_paused/get_paused`
  （0x1414372d0/0x141437440/0x141437160/0x141436fe0）+ `switch_track`（0x1408b70d0）+ log
  （0x140d4a330）。全为 FMOD/libfmod 边界，仅标状态字段 `[+0x118]` 状态、`[+0x128]` 上次位置、
  `[+0x11c]` 时间累加、`[+0x40]` 事件状态、`[+0x48]` FMOD 事件指针、track Vec `[+0xf0]`/`[+0xf8]`。

## 7. FMOD 调用边界（只标注不复原）

- libfmod 内联进 country_core 的实例方法（0x141436fe0 get_paused、0x141437160 set_paused、
  0x1414372d0 start、0x141437440 stop、0x1414375b0 get_timeline_position、
  0x141436b40 set_volume、get_parameter_by_name 等）只作边界；B 用 libfmod-2.222.6
  （rcgu [1601,3100,3103,3104,3105]，内联进 country_core [485,548,1623]）。
- bevy 框架：add_systems/register_event/EventReader/EventWriter/Res(AsRef)/hashbrown/
  TypeId/get_resource_id 一律依赖复用。
- 游戏自有 FMOD 胶水（本模块还原对象）：audio_resources.rs 的 `AudioCore`（+0x10
  AudioCoreInner）、`AudioCoreInner::instantiate_audio`（0x140a54440）、`AudioCore::instantiate_loop`
  （0x140a59220）、`impl$10::set_position`（0x140a57da0）/`set_parameter`（0x140a57670、
  0x140a53db0）/`update`（0x140a53a00）、`set_vca_volume`（0x140a54810）、
  `start_if_not_playing`（0x140a57300/0x140a596b0）、`stop`（0x140a59430）；以及
  `wall_audio_utils::AudioLoopPlayer::request_play`（0x1409285a0）与
  `roof_audio::instantiate_audio_w_position_and_volume`（0x140aa3590）。

## 8. 事件字段布局总表（证据：事件槽 stride + 槽内偏移，radare2/reva 一致）

| 事件 | 槽 stride | 槽内字段 |
|---|---|---|
| PlayWallAudio | 0x28 | kind u8 @+8；f32 @+c；position Vec3 @+0x10（x,y u64 / z u32）；volume f32 @+0x1c；byte @+0x20（@+0..+8 为 bevy 书签） |
| PlayBuildingAudio | 0x20 | id u64 @+0（hash 键，**payload 从槽 @+0 起，无 bevy 书签**）；position Vec3 @+8（x,y u64 / z u32 @+0x10）；editing_speed f32 @+0x14；kind u8 @+0x18 |
| PlayStairsAudioCmd | 0x38 | kind u8 @+8；字节 @+9/@+a/@+b；f32 @+c/@+0x10/@+0x14/@+0x18；position u64/u32 @+0x1c/@+0x24；f32 @+0x28/@+0x2c（@+0..+8 为 bevy 书签） |

> bevy EventInstance 书签放置不统一（同 tree-process-cmds 结论）：PlayBuildingAudio 的 id 即
> 槽 @+0 首个 payload 字段（filter_map 收集后元素 +0 仍是 id）；wall/stairs 则 @+0..+8 为书签。

## 9. 剩余 unknown（不伪造）

- add_audio_systems 的第二次 add_systems 具体系统清单（ScheduleConfigs 内容，bevy 内部）与
  0x26 SystemSet 对应系统；其余音频系统（clutter/decorator/forest/…）的注册点。
- PlayWallAudio kind→case 的完整逐条对照（跳表 0-23 有效，其中 16/24+ 为未用/越界槽）；
  每 kind 对应哪个 loop player offset 的精确对应（10 loop 的 order 已按 init 顺序标，system
  侧按 `[rbp+xx]`=state+0x40… 引用的对应尚待逐一钉死）。
- PlayBuildingAudio kind 0–7 的 `(name_ptr,name_len)` 配对（0x142ce4c88/0x142ce4ca0 两表
  交错，串长与"DetachFromGround/AttachToGround/PeakReached"的精确对应）与 6–7 kind 常量语义。
- building 条目 0x2a8 值内 `[-0x290]/[-0x270]/[-0x48]/[-0x118]` 之外字段语义；
  `[-0x290]` 计数、`[-0x270]` 浮点的确切含义。
- stairs_audio_system 的 18 kind 到"瞬发事件名/循环 loop"的逐条映射（跳表 0x142bad9f0 已读，
  各 case 的 esi/r15 常量已录，完整逐 kind 表待补）。
- AudioLoopDesc 的 `count=3`、`max=0x32` 与 AudioLoopPlayer+0x20 的 1000000000 的语义。
- wall_audio_fmod 全量参数清单（疑 8 参 bevy 系统）与音量源栈参的实际类型；
  stairs_audio_system arg3=Res<…元素表> 的实际 Rust 类型。
