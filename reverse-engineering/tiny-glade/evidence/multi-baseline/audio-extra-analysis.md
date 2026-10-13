# country-core systems/audio 第二批补全：wall kind 逐条表 / building kind 配对 / 剩余事件系统 / music 参数（B 主基线，反汇编级证据）

2026-10-13。本文是 `audio-events-analysis.md`（第一批）的补全。目标函数均为 B patched exe
（VA 0x140001000 基址）；VA/大小取自 `evidence/multi-baseline/audio-inventory.json`
（llvm-pdbutil 修正）。

证据来源（本批）：
- **reva**（B 持久库，worker `audio-extra-owner`，run-script 只读线性反汇编+调用目标+mem_refs+
  定点反编译；B sha256 `cf048e27…7036323`）：`terrain_stairs_audio_system` 0x141462dc0、
  `water_ambience_audio_system` 0x1409ecea0、`water_ambience_audio_init_system` 0x1409ebf20、
  `menu_audio` 0x140a0b4c0、`menu_audio_init_system` 0x140a0b340、`sheep_audio_system`
  0x1409d5ea0、`clutter_audio_system` 0x1408c02f0、`music_audio_system` 0x140a72810；
  结果 JSON 落盘 `ghidra_scripts/audio-extra-owner/*.json`。
- **radare2**（RNE 发行原件 oracle，worker `audio-extra-owner` 独立进程会话）：跳表
  0x142ab4998（wall）、0x142ce4c88/0x142ce4ca0/0x142ce4d30（building）、0x142bad9f0（stairs）、
  0x142ac706c（menu）、0x142ac2cc8（sheep）、0x142aa1688（clutter）与字符串/浮点常量逐字节读；
  原始字节与反汇编见 `tool-usage/jobs/audio-extra/{radare-oracle,radare-oracle2,radare-oracle3,radare4}.json`
  及其 `result-*.json`、`parsed-hexdumps.json`。
- 复用第一批 `/tmp/audio-re/` 落盘：`fn-140950920.txt`（wall_audio_fmod RNE 全线性反汇编）、
  `fn-140a72810.txt`（music_audio_system）、`fn-1417d2ad0.txt`（building_audio）、
  `fn-140f880a0.txt`（stairs_audio_system）、`disasm_linear.json`、`names.json`、`jt.json`、
  `tables.json`。本次重新用 radare2 重算了所有跳表目标（旧 `read_jt.py` 以 `base+i*4` 计算
  相对偏移是错的，正确为 `base + sign_ext(dword)`）。

身份核对：reva 返回 `sha256=cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`
（B patched）；radare2 打开 `tiny-glade.rne`（发行 RNE，PE64）。

## 1. wall_audio_fmod：PlayWallAudio kind 0–23 逐条表（补全 §3b 的 unknown）

分派（0x140950dc0-0x140950dcc）：`movzx ecx, byte [rax+8]`（kind）→
`movsxd rcx, dword [r13+rcx*4]`（r13=0x142ab4998）→ `add rcx, r13; jmp rcx`。
事件槽 stride 0x28，payload 从 @+0x8 起（@+0..+8 bevy 书签）。跳表 24 项（kind 0–23），
RNE .data 逐字节确认（`base + sign_ext` 重算）。

| kind | 目标 | 类别 | 事件名 / loop 索引 | 参数（FMOD set_parameter） |
|---|---|---|---|---|
| 0 | 0x140950dce | 循环 loop0 | event:/SFX/Wall Draw | —（request_play，editing_speed=音量源） |
| 1 | 0x1409510d8 | 循环 loop1 | event:/SFX/Wall Change Height | `WallHeight` = inverse_lerp(0.5, 14.0, 事件+0xc) |
| 2 | 0x140950f1f | 循环 loop2 | event:/SFX/FenceExtend | `FenceLength` = inverse_lerp(8.0, 50.0, 事件+0xc) |
| 3 | 0x140950ffc | 瞬发 | event:/SFX/FenceAppear (0x142ab41dd, 0x16) | 音量=事件+0x1c |
| 4 | 0x140950e77 | 瞬发 | event:/SFX/FenceMerge (0x142ab41f3, 0x15) | 同上 |
| 5 | 0x1409511d8 | 瞬发 | event:/SFX/FenceRearrange (0x142ab4208, 0x19) | 同上 |
| 6 | 0x140951282 | 循环 loop4 | event:/SFX/StoneArchExtend | — |
| 7 | 0x140951034 | 循环 loop3 | event:/SFX/LintelExtend | `FenceLength` = inverse_lerp(3.0, 15.0, 事件+0xc) |
| 8 | 0x14095137c | 瞬发 | event:/SFX/LintelAppear (0x142ab4221, 0x17) | 音量=事件+0x1c |
| 9 | 0x140950ee7 | 瞬发 | event:/SFX/LintelRearrange (0x142ab4238, 0x1a) | 同上 |
| 10 | 0x1409512f6 | 循环 loop5 | event:/SFX/WideningGate | `HasBricks` = 事件+0x9 非零 ? 1.0 : 0.0 |
| 11 | 0x140950e3f | 瞬发 | event:/SFX/LintelMerge (0x142ab425b, 0x16) | 音量=事件+0x1c |
| 12 | 0x140950eaf | 瞬发 | event:/SFX/StoneArchAppear (0x142ab4271, 0x1a) | 同上 |
| 13 | 0x140951248 | 循环 loop6 | event:/SFX/PlatformStairsExtend | — |
| 14 | 0x140950e07 | 瞬发 | event:/SFX/PillarStoneAppear (0x142ab428b, 0x1c) | 音量=事件+0x1c |
| 15 | 0x140950fc4 | 瞬发 | event:/SFX/PillarWoodAppear (0x142ab42a7, 0x1b) | 同上 |
| 16 | 0x140950d45 | 瞬发 | event:/SFX/LintelRearrange (0x142ab4238, 0x1a) | 同上（与 kind 9 同名同处理） |
| 17 | 0x140951166 | 瞬发 | event:/SFX/GateAppear (0x142ab42c2, 0x15) | 同上 |
| 18 | 0x1409512bc | 循环 loop7 | event:/SFX/SewerGateWiden | — |
| 19 | 0x1409513ec | 瞬发 | event:/SFX/PlatformStairsAppear (0x142ab42d7, 0x1f) | 音量=事件+0x1c |
| 20 | 0x14095119e | 循环 loop9 | event:/SFX/FenceMove | — |
| 21 | 0x140951210 | 瞬发 | event:/SFX/WallSplit (0x142ab42f6, 0x14) | 音量=事件+0x1c |
| 22 | 0x1409513b4 | 瞬发 | event:/SFX/SewerGateAppear (0x142ab430a, 0x1a) | 同上 |
| 23 | 0x140951424 | 循环 loop8 | event:/SFX/FenceHeight | — |

- loop 索引即 `AudioLoopPlayer ×10`（stride 0x40）偏移：loop0@+0x0、loop1@+0x40、loop2@+0x80、
  loop3@+0xc0、loop4@+0x100、loop5@+0x140、loop6@+0x180、loop7@+0x1c0、loop8@+0x200、
  loop9@+0x240（init 顺序 = 事件名顺序）。case 内经 `[rbp±]` 引用这些偏移（证据：radare2 全线性
  反汇编 prologue 的 10 个 `lea`）。
- 循环类 = `request_play(loop, pos, 事件+0x1c→set_volume, r8d=事件+0x20, [rsp+0x20]=editing_speed)`
  （0x1409285a0），其中 kind 1/2/7/10 额外 `set_parameter(elem, …)`，elem 来自各 loop 关联参数
  Vec（kind1→+0x48、kind2→+0x88、kind7→+0xc8、kind10→+0x148，ptr/cap/len，元素 0x30）。
- 瞬发类 = `instantiate_audio_w_position_and_volume(core, name, pos, 事件+0x1c)`（0x140aa3590），
  name=(ptr,len) 来自 .data 连续串表。
- 串长列：从 RNE .data 连续切片（无 NUL）按相邻起始地址差算出，与 case 内 `r8d` 立即数逐条一致
  （如 kind9/16 `r8d=0x1a` ↔ `event:/SFX/LintelRearrange` 26 字符）。
- **更正第一批**：`0x142ab4238` 实为 `event:/SFX/LintelRearrange`（0x1a=26），不是
  `LintelRearrangeHasBricks`；`HasBricks`（0x142ab4252, 0x9）只作 kind10 的参数名。
- 帧尾（0x14095145e 起）：按各 loop 的"应播"标志 `start_if_not_playing`/`stop` 并清标志；
  `[state+0x280]` 音量 *= `powf(0.1, [栈参资源]+0x58 * 5.0)`（xmm0=[0x142925984]=0.1、乘数
  [0x142925988]=5.0、[栈参资源]+0x58 未定）。
  音量源：`[entry+0x40 栈参]` deref 后 `+0x1c f32 / 60.0`（0x1429d18cc=60）；AppMode==0xd 用 1.0。

## 2. building_audio：PlayBuildingAudio kind 配对（补全 §4b 的 unknown）

分派（0x1417d2c69-0x1417d2ca7）：事件槽 stride 0x20、kind @+0x18、payload 从槽 @+0 起
（id u64@+0、position Vec3@+8、editing_speed f32@+0x14）。`movzx ecx, byte[rax+0x18]` →
`add cl, 0xfa`（即 kind-6）→ `cmp cl, 3; jae`（**只处理 kind 6/7/8**，其余 kind 事件被跳过）→
`shl eax, 3` → `lea rcx,[0x142ce4c88]` 取 `r15=[rcx+rax]`（name_len，8 字节项）、
`lea rcx,[0x142ce4ca0]` 取 `r12=[rcx+rax]`（name_ptr）。两表按 **(kind-6)*8** 索引，仅 3 项。

| kind | 表索引 (kind-6) | name_ptr (0x142ce4ca0) | name_len (0x142ce4c88) | 事件名（RNE 确认） |
|---|---|---|---|---|
| 6 | 0 | 0x142ce48f8 | 0x22 (34) | event:/SFX/Shapes/DetachFromGround |
| 7 | 1 | 0x142ce491a | 0x20 (32) | event:/SFX/Shapes/AttachToGround |
| 8 | 2 | 0x142ce493a | 0x1d (29) | event:/SFX/Shapes/PeakReached |

FMOD 序列（kind 6–8 路径）：`instantiate_audio(&out, core+0x10, name_ptr, name_len)`（0x140a54440，
r8=ptr、r9=len）→ `set_position(inst, 事件 position)`（0x140a57da0）→
`set_parameter("editing_speed"(0x142ce49a0, 0xd), 事件+0x14)`（0x140a57670）→
`set_parameter("BuildingSize"(0x142ce49ad, 0xc), inverse_lerp(area))`（area 由 hash 查到的 0x2a8
构建条目 `[entry-0x48]` 形态 0/1 选 `0x140c99520`（圆）/`0x140c97580`（矩形））→
`start_if_not_playing`（0x140a57300）→ drop。构建条目条件：`[entry-0x290]>=2` 且 `[entry-0x270]>0`。

**尾部默认路径**（事件循环结束后 0x1417d2f77 起）：对固定 7 个 kind `{0,1,2,3,4,9,5}` 每帧处理
（`[rbp-0x20]=0x3020100` 即 kind 0,1,2,3；`[rbp-0x1c]=0x0904` 即 kind 4,9；`[rbp-0x1a]=5`），
经跳表 **0x142ce4d30**（kind 0–9，10 项；kind 6/7/8 指向 0x1417d386e panic/不支持）分发到
BuildingAudioState 各 AudioLoopPlayer 并更新参数：

| kind | 跳表目标 | rbx = loop 偏移 | 事件名（init 顺序 loop） |
|---|---|---|---|
| 0 | 0x1417d3075 | state+0x0 | event:/SFX/Building/BuildingResize |
| 1 | 0x1417d30a0（默认 rbx=state+0x38） | state+0x38 | event:/SFX/Building/BuildingRotateRectangle |
| 2 | 0x1417d3087 | state+0x70 | event:/SFX/Building/BuildingRotateCircle |
| 3 | 0x1417d308d | state+0xa8 | event:/SFX/Building/BuildingChangeHeight |
| 4 | 0x1417d307b | state+0xe0 | event:/SFX/Building/BuildingMove |
| 5 | 0x1417d3081 | state+0x150 | event:/SFX/Building/BuildingOverlapMove |
| 6/7/8 | 0x1417d386e | —（尾部不支持） | （由事件路径处理） |
| 9 | 0x1417d3093 | state+0x118 | event:/SFX/PlasterChangeLoop |

尾部参数（每 loop 的 param 元素 Vec，stride 0x30）：
`"editing_speed"(0x142ce49a0, 0xd)`、`"BuildingSize"(0x142ce49ad, 0xc)`、`"ifWindow"(0x142ce49f0, 0x8)`、
`"BuildingHeight"(0x142ce49f8, 0xe)`（`[entry-0x1c]` 比较）、`"isHalfTimbered"(0x142ce4a06, 0xe)`
（`[entry-0x20]` 与 1.0 相等判定）、`"IsOnGround"(0x142ce4a14, 0xa)`、`"loop"(0x142ad2f20, 0x4)`
（r14b 标志）。FMOD `start`/`stop`（0x1414372d0/0x141437440）按元素判别。

## 3. 剩余事件系统（各确认"事件→FMOD 参数映射"模式）

### 3a. playermade_stairs_audio（stairs_audio_system，0x140f880a0，RNE 全线性反汇编）
- 帧首停 6 个 loop（state+0x0/+0x38/+0x70/+0xa8/+0xe0/+0x118，stride 0x38）。
- 事件槽 stride 0x38；kind @+0x8；`cmp r12, 0x12; ja → 跳过`，跳表 **0x142bad9f0**（kind 0–18，重算）：
  | kind | 目标 | 类别 | 事件 |
  |---|---|---|---|
  | 0 | 0x140f88580 | 循环 loop0 | event:/SFX/Stairs/StairMove |
  | 1 | 0x140f88395 | 循环 loop1 (state+0x38) | StairChangeHeight |
  | 2 | 0x140f8837e | 循环 loop2 (state+0x70) | StairChangeWidth |
  | 3 | 0x140f88354 | 瞬发 (0x142bad6c8, 0x1c) | StairPlace |
  | 4 | 0x140f8839e | 瞬发 (0x142bad6e4, 0x1d) | StairDelete |
  | 5 | 0x140f883ba | 瞬发 (0x142bad701, 0x1d) | StairsMerge |
  | 6 | 0x140f88387 | 瞬发 (0x142bad71e, 0x24) | StairsMergeConfirm |
  | 7 | 0x140f88579 | 循环 loop3 (state+0xa8) | LadderMove |
  | 8 | 0x140f88370 | 瞬发 (0x142bad742, 0x1e) | LadderAppear |
  | 9 | 0x140f88573 | 循环 loop4 (state+0xe0) | LadderChangeHeight |
  | 10 | 0x140f8856d | 循环 loop5 (state+0x118) | LadderChangeWidth |
  | 11 | 0x140f88346 | 瞬发 (0x142bad760, 0x1d) | LadderPlace |
  | 12 | 0x140f88362 | 瞬发 (0x142bad77d, 0x1d) | StepsAppear |
  | 13 | 0x140f883ac | 瞬发 (0x142bad79a, 0x1f) | WalkwayAppear |
  | 14 | 0x140f88324 | 瞬发 (0x142bad7b9, 0x24) | StairSupportAppear |
  | 15 | 0x140f88335 | 瞬发 (0x142bad7dd, 0x23) | StairPillarAppear |
  | 16 | 0x140f8825e | 忽略（回循环顶） | — |
  | 17/18 | 0x140f88324 | 瞬发（同 14） | StairSupportAppear |
- 瞬发公共块（0x140f883d0）：`instantiate_audio(&out, core+0x10, name, len)` →
  `set_position(事件+0x1c/+0x24)`（0x140a57da0）→ `set_parameter("stair_width"(0x142bad818, 0xb),
  xmm11)` → `start_if_not_playing` → 二次参数分发（跳表 0x142bada3c，kind≤0xa）与
  0x142bada68（kind≤9）：`size`(0x142bad823, 4)、`extend_amount`(0x142bad827)、`h_offset`
  (0x142bad834)、`avg_arch_supports`(0x142bad841)、`height_above_ground`(0x142bad849)、
  `in_water`(0x142bad86e)、`stair_edge_length`(0x142bad876)、`is_stub_preview`(0x142bad887)、
  `move_all`(0x142bad896, 8)。
- 循环公共块（0x140f88580）：`start_if_not_playing(loop)` + `set_parameter(elem,"stair_width",xmm11)`。

### 3b. terrain_stairs_audio（0x141462dc0，reva/radare2）
- `[AppMode]==5` 时直接返回；事件读取为 `SuppressPathSound`（bevy Events）+ 内部 StairsCounter
  状态（`retain<AudioInstance>` 清理 + `get_playback_state` 0x141437730 libfmod 检查）。
- FrozenMap::get<SsboId, BufferState>（0x140969720）取地形楼梯 SSBO 缓冲，`memcmp` 0x45 字节
  名称断言（源路径 `country_core::systems/audio/terrain_stairs_audio…`）。
- 若缓冲 id 与当前相同 → `AudioLoop::stop`（0x140a59430）；否则
  `instantiate_audio("event:/SFX/StairDraw_OneShot"(0x142c62e70), …)`（0x140a54440）→
  `start_if_not_playing`（0x140a57300）→ `set_parameter`（0x140a57670）并压入实例 Vec。
- 音量/速度：`inverse_lerp`（0x140c904a0）+ `[0x1429d18cc]=60` 归一化（同 wall 音量源模式）。
- **模式确认**：事件→(SSBO 状态比对)→instantiate_audio→set_parameter→start_if_not_playing。

### 3c. water_ambience_audio（0x1409ecea0 + init 0x1409ebf20）
- init：`instantiate_loop` 闭包（from_iter<AudioInstance, Map<Range, instantiate_loop::closure>>）
  按水网格装配多个 AudioLoopPlayer，`get_parameter_by_name`（0x141437a60）、FMOD
  `stop`/`release`（0x141437440/0x1414378f0）；`insert_resource<WaterAmbienceAudio>` +
  `insert_resource<WaterAudioGrid>`。事件名 `event:/SFX/Water/Amb`（0x142ac5298）。
- system：puffin scope；遍历 WaterAudioGrid，未激活网格先 `AudioLoop::stop`（0x140a59430），
  激活者 `AudioLoop::start_if_not_playing`（0x140a596b0）+ `set_position`（0x140a57da0，
  位置由 `eye_position` 0x140c96d40 / `x0y` 0x140c9cdc0 计算）+ libfmod `set_volume`
  （0x141436b40）。常量：`[0x142ac4d00]=0.015625`、`[0x142ac4d20]=42.5`（距离/衰减范围，语义未定）。
- **模式确认**：循环 player 网格 + 位置/音量更新（无逐 kind 事件分发；触发经
  `run_if(WaterAudioGrid…)` 门禁，见第一批 §1b）。

### 3d. menu_audio（0x140a0b4c0 + init 0x140a0b340）
- init：`instantiate_audio`（0x140a54440）`"snapshot:/PauseMenu"`（0x142ac6db8）等 →
  `insert_resource<MenuAudioState>`（0x1412df050）。
- system：AppMode 门禁（`[arg2]+0 | 8 == 0xa` → libfmod `stop` 0x141437440 快照；否则
  `start_if_not_playing` 0x140a57300）。事件槽 stride 0x10、kind @+0x8；跳表 **0x142ac706c**
  （kind 0–12，13 项；13+ 越界）。每 case：`instantiate_audio(&out, core+0x10, name_ptr, len)`
  （内联，无独立调用）→ `start_if_not_playing`。事件名（RNE 串表 0x142ac6e30 起）：
  kind0 `event:/UI/Menu/Hover`(0x142ac6e30)、1 `Ui2dHover`(0x142ac6e44)、2 `Menu/Select`
  (0x142ac6e57)、4 `Menu/SettingsTab`(0x142ac6e83)、7 `Menu/NewSession`(0x142ac6ed1)、
  8 `Menu/StartSession`(0x142ac6eea)、9 `Menu/Back`(0x142ac6f05)、11 `Menu/Close`(0x142ac6f2b)、
  12 `Ui2dLockedClickNew`(0x142ac6f3f)；其余 kind 3/5/6/10 为表内相邻名（Settings/SliderTick/
  ShowSessions/Open，逐条未钉死）。
- **模式确认**：PlayMenuUiSound kind → instantiate_audio(event:/UI/…) → start_if_not_playing，
  AppMode 门禁控制快照 stop/start。

### 3e. sheep_audio（0x1409d5ea0，reva/radare2）
- 事件槽 stride 0x20、kind @+0x18；跳表 **0x142ac2cc8**（kind 0–7；8+ 越界）。每 case 设
  (name_ptr, len) 并跳公共块 0x1409d6060：`instantiate_audio(&out, core+0x10, name, len)` →
  set_parameter → start_if_not_playing；帧尾按标志 `start_if_not_playing`/`stop`
  （0x140a596b0/0x140a59430）。
- 事件名（0x142ac23b0 串表）：kind0 `event:/SFX/Sheep/Aww`(0x142ac23b0, 0x14)、1 `Think`
  (0x142ac23c4, 0x16)、2 `So`(0x142ac23da)、3 `Angry`(0x142ac23ee, 0x16)、5 `Idea`(0x142ac2404,
  0x15)、6 `Eyes`(0x142ac2419, 0x15)、7 `Confused`(0x142ac242e, 0x19)；kind4 → 0x1409d61b0
  （帧尾 start/stop 分支，无瞬发）。
- **模式确认**：PlaySheepReactSound kind → instantiate_audio(event:/SFX/Sheep/…) +
  set_parameter + start_if_not_playing，帧尾 loop start/stop。

### 3f. clutter_audio（0x1408c02f0，reva/radare2）
- 事件槽 stride 0x18、kind @+0x14；跳表 **0x142aa1688**（kind 0–0x26=38，39 项；0x27+ 越界）。
  每 case：`mov r9d, <len>; mov [rbp+0x78], rcx; lea r8, <name_ptr>; jmp 0x1408c0990`。
- 公共块 0x1408c0990：`instantiate_audio(&out, core+0x10, r8=name, r9=len)` →
  set_position(事件位置 @+0/+8) → set_parameter → start_if_not_playing（全部内联，无外部 FMOD
  调用）。
- 事件名全部 `event:/SFX/Cosmetics/…`（RNE 串表 0x142aa0e4c 起）。抽查：kind 0x1d→DoorBell
  (0x142aa1299)、0x1c→BoxFish(0x142aa125e)、0x1f→RackBrooms(0x142aa127a)、0x1d→OldenLog
  (0x142aa1167)、0x20→Veggiestall(0x142aa123e)、0x2e→ClothlineConnectionAppear(0x142aa1013)、
  0x1d→Shutters(0x142aa0fd2)、0x1b→Bucket(0x142aa0ebe)、0x20→BoxWaterCan(0x142aa0e9e)、
  0x1c→ClayPot(0x142aa0f0d)、0x1a→Anvil(0x142aa11e5)、0x1d→Firewood(0x142aa0f7c)、
  0x1b→Ladder(0x142aa0f47)、0x1b→Basket(0x142aa110c)、0x1c→Pumpkin(0x142aa1127)、
  0x1c→BoxSack(0x142aa0e82)、0x24→ClothlineAppear(0x142aa0fef)、0x26→Box(0x142aa0e6a)；
  其余逐 kind 未钉死（模式一致）。
- **模式确认**：PlayClutterSoundCmd kind → (name_ptr,len) 表 → instantiate_audio +
  set_position + set_parameter + start_if_not_playing。

## 4. music 细节：MusicRichness / NewGladeTime 参数计算（music_audio_system 0x140a72810）

反汇编 + reva 反编译（High P-code）交叉核对。参数资源（arg 角色，identity 部分未定）：

| 参数 | 读取 | 语义 |
|---|---|---|
| arg1 (RCX) | 资源 inner `+0` u32 | `==0x13` 时 MusicRichness 平滑目标强制 1.0 |
| arg2 (RDX) | `*[rdx]` = MusicPlayer | set_parameter / update / pause 目标 |
| arg3 (R8) | `*[r8]` = MusicAudioState | 字段 +0x1c0 smoothed、+0x1c4 accum、+0x1c8 NewGladeTime accum |
| arg4 (R9) | 资源 inner `+0` u32 | AppMode；`(mode|8)==0xa` 才 MusicPlayer::update，否则 pause |
| arg5 栈 `[rbp+0xe0]` | 资源 inner `+0x28` f32 | 时间源 |
| arg6 栈 `[rbp+0xd8]` | 资源 inner `+0x100` byte | 置位时 MusicRichness 用 1.0 |
| arg7 栈 `[rbp+0xe8]` | 资源 inner `+0x58` f32 | MusicPlayer::update 的节奏/节拍参数 |

**MusicRichness 计算**（`update_music_richness_based_on_time` 内联，music_system.rs line 136）：
```
accum   = clamp(State+0x1c4 + time/120.0, 0, 1)            # 0x1429cd0f8 = 120.0
# "snap off" 边界（避免恰好 0.25/0.5/0.75）：
#   [0.24, 0.25)  → max(x, 0.251)      # 0x142a7fdf8=0.24, 0x142925950=0.25, 0x142ad58e4=0.251
#   [0.49, 0.5]   → max(x, 0.501)      # 0x142ad58d4=0.49, 0x142925870=0.5, 0x142ad58e0=0.501
#   [0.74, 0.75]  → max(x, 0.751)      # 0x142ad58d8=0.74, 0x1429edb90=0.75, 0x142ad58dc=0.751
target  = 1.0 if [arg1]+0 == 0x13 else accum
smoothed = time_continuous_lerp(State+0x1c0, target, ...)   # 0x140c904e0
smoothed = clamp(smoothed, 0, 1) 且 snap off（同上三档）→ State+0x1c0
set_parameter(MusicPlayer 事件, "MusicRichness"(0x142ad6398, 0xd),
              if [arg6]+0x100 != 0 then 1.0 else smoothed)  # 0x140a53db0
```
**NewGladeTime 计算**：
```
t = clamp(time + State+0x1c8, 0, 0.8)      # 0x142a16cb8 = 0.8
State+0x1c8 = t
set_parameter(MusicPlayer 事件, "NewGladeTime"(0x142ad63a5, 0xc), t)
```
**帧尾**：`(AppMode|8)==0xa` 时 `MusicPlayer::update(player, [arg7]+0x58)`（0x1408b5500），
否则 `MusicPlayer::pause()`（0x1408b7490）；随后按 MusicAudioState 内判别/标志做
`stop_photomode_snapshots`（0x1408b6250）、`start_if_not_playing`、libfmod
`stop/set_paused/get_paused`（0x141437440/0x141437160/0x141436fe0）。参数串表
0x142ad6398 起：`MusicRichness`、`NewGladeTime`、`PreviewGladeType`（0x142ad63b1, 0x10）。

## 5. 剩余 unknown（不伪造）

- wall kind 0/6/13/18/20/23 循环类的 `editing_speed` 与音量归一化的精确来源（栈参 +0x1c/60.0，
  语义待钉死）；wall_audio_fmod 全量参数清单（疑 8 参 bevy 系统）。
- building 事件路径只覆盖 kind 6/7/8；kind 0–5、9+ 的 PlayBuildingAudio 事件在事件循环被跳过
  （其语义经尾部默认 kind 每帧处理——是否由事件写状态驱动尾部行为未证实）。
- menu kind 3/5/6/10 与 clutter 39 个 kind 的逐条事件名未全部钉死（模式一致，抽查已列）。
- sheep kind4 语义（0x1409d61b0 = 帧尾 start/stop 分支）。
- music arg1/arg4/arg5/arg6/arg7 的具体 Rust 资源类型未定（PDB 无 Rust 类型信息；反编译只给出
  偏移访问）；`0.8`/`120`/`0.99` 常量角色按证据记录，未回放场景验证。
- terrain_stairs 的 SSBO BufferState 名称断言（0x45 字节源路径）为调试/归属检查，语义未定。
- water ambience 距离衰减常量 `0.015625`/`42.5` 的角色未定。
