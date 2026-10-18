# LOGIC.md — systems/audio 逐函数逻辑判等审计（logic-verifier-audio，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`。只以 fresh 反编译为准
> （ghidra `--worker shared`，fields=signature,decompiled_code,callees；radare2
> 于 RNE 发行件作机器码 oracle，B patched 地址与 RNE 一致，已核对 update/wall_audio_ui/
> switch_track 三条 disasm 均与 B 反编译对齐）。
> 判定：`判等`=正常路径每分支/循环/调用/数据流逐条对应；`未实现-需重做`=核心逻辑
> 分歧（缺分支/错事件/自造近似）。错误路径分歧（native unwrap/报告 vs Rust `?` 传播）
> 统一登记为「错误路径注记」，不因此把已判等的正常路径降级。
> 未修改 src。FMOD 绑定（库外 libfmod）与注入 trait 语义按红线审；下面逐条列分歧。

## 一、判等（fresh 反编译逐条对应）

| VA | 函数 | 判等要点 | 注记 |
|---|---|---|---|
| 0x1409c8dd0 | `audio_update_system` | 5× set_vca_volume（Master/Music/Music-DawnJingle/SFX/Amb）+ 听者 + AudioCoreInner::update；decomp 明确 5 次调用链 | 错误路径：native 某 set_vca_volume 失败则跳过 update（`*plVar8==0` 短路），Rust `let _=` 丢弃后无条件 update——正常路径一致，登记 |
| 0x1409c9320 | `boot_audio_update_system` | 同构 + 固定默认听者 | 同上 |
| 0x1408b5500 | `MusicPlayer::update` | **disasm 全解**：position=max(0, tl)；looped=pos<last(+0x128)；BAR_COUNT 复位；SwitchTrack 臂（last=0/BAR_COUNT=0/switch_track/fade_in.start/track.start/state=1）；Playing 1.5（0x1429258d0=1.5）→fade_in.stop(0)；looped→state=2+ticks=0（不清 +0x11c）；PlayingLoopedOnce `elapsed>ticks`→ceil+`/120.0`（0x1429cd0f8）+`p²>rand`→state=3；SwitchAtNextOpportunity `(looped‖BAR_COUNT&3==0)&&CURRENT_MUSIC_BEAT==1`→state=4+track_switching.start；Switching 18.5/19.0/18.0（0x142a9f400/404/0x142a7ba80）逐序；末尾 unpause 路径（track Active 且 paused→双 set_paused(false)，Disabled 同样进入） | ① log 文案：Rust `"No music track available"` ≠ native `"Failed to switch music track; will retry!"`（0x142a9f568，disasm 0x1408b57c8）——低。② 随机源：native 在越过整 tick 后才采 fastrand（0x140caf3c0），Rust `update_with_random` 注入采样时机一致 ✓ |
| 0x1408b70d0 | `MusicPlayer::switch_track` | disasm：playlist len(+0x110)==0→identity 填充(0..tracks.len)→`fastrand::shuffle`(0x14132b1a0)→防首（`playlist[0]==previous` 时 swap）→`playlist.pop()`（+0x110=len-1） | **swap 下标分歧**：native `mov rax,len; and rax,~1`=swap(0, len&~1)，Rust `len/2`。奇数长（3→2 vs 1）不同；偶数长 native 越界（原版潜在 bug，Rust 未复现）。中低 |
| 0x1408b63c0 | `MusicPlayer::init` | 4 snapshot（Track Switching/PhotoMode/FirstPerson/MusicFadeIn），Disabled-studio 分支仅名、Active 分支 unwrap(disc==2)；current=None、state=0、last=0 | — |
| 0x1408b6b10 | `MusicPlayer::load_tracks` | 清 tracks(+0xf8)/names(+0xe0)；逐 path：instantiate（disc==2 unwrap）→Active 时 install beat callback→成功才 push instance+clone name，失败 drop（Disabled/Err 均不 push）→state=0 | — |
| 0x1408b6250 | `MusicPlayer::stop_photomode_snapshots` | photomode(+0x70)+first_person(+0xa0) disc==1→stop(0=ALLOWFADEOUT)，unwrap | — |
| 0x1408b72e0 | `MusicPlayer::stop_immediately` | 全 tracks stop + `*self=0`(None) + track_switching stop(1=IMMEDIATE) | — |
| 0x1408b7490 | `MusicPlayer::pause` | tracks[current] Active→set_paused + track_switching set_paused | — |
| 0x1408b7620 | `get_timeline_beat_properties` | null 忽略；beat/bar 读、tempo bits、count+=(bar 变)+max(count,bar)、upper/lower 符号扩展、**bar 最后写**——与 Rust 逐句一致 | — |
| 0x1408c02f0 | `clutter_audio_system` | 双缓冲事件迭代（stride 0x18，kind@+0x14）；跳表 0..0x26，0x12 跳过；串表 rne-read 0..0x12 逐条命中（BirdsNest/Box/…/WaterPlant），0x13 Bench 起始；公共块 instantiate→set_position→set_parameter→start_if_not_playing+drop | ① context 值：native `[0x142aa1590+ctx*4]` 无 clamp（越界读），Rust `.min(2)` 钳制——低。② 双事件列表=b e v y 双缓冲，语义单流 ✓ |
| 0x140a57240/57300/57470/57550/575a0 | AudioInstance start / start_if_not_playing / stop / is_playing / set_volume | disc 判别转发逐条对上；start_if_not_playing 位掩码 `(0xb>>(state&31))&1==0` 与 `bt 0xb` 一致 | **错误路径注记**：native 全部 `event.start/stop/set_volume().unwrap()`（失败 panic），Rust `?` 返回 Result 由调用方丢弃——正常路径一致，错误语义分歧（多次出现，统一登记） |
| 0x140a57670 | AudioInstance::set_parameter | 判别转发 ✓ | 缺 native 错误路径：格式化 `"Failed to set parameter named … on …"`（PTR 0x142ad2cc8）+panic/报告注册；Rust 返回 Err |
| 0x140a57da0 | AudioInstance::set_position | set_3d_attributes，forward(0,0,-1)/up(0,1,0)/vel 0 ✓ | — |
| 0x140a53640 | Bank::load | loaded&1 早退；load_bank_file；失败 panic `"Error loading FMOD bank file: {path} {err}"`；成功 loaded=1+handle@+8 | 缺加载前 log（`Loading FMOD bank file: {path}`，0x142ad24f0）——低 |
| 0x140a54440 | AudioCoreInner::instantiate_audio | get_event→create_instance→Active{event@+8,name@+0x10,state@+0x28=2}；两失败路径→disc=2(Err)+anyhow | `Ok(None)→Disabled` 为 Rust 自建分支（native 无 None 语义）——边界建模，登记 |
| 0x140a55f20/55f40 | AudioCore::set_parameter / instantiate_audio | 非空 core→委托 Inner；空→Disabled(名+state=2) ✓ | — |
| 0x140a57100 | AudioSamplesLoadedPromise::recv | crossbeam array/list/zero 三通道 recv；超时→drop+返回；断连→unwrap_failed | 超时=1s 无法从本 decomp 直接证实（crossbeam 内部），结构一致 |
| 0x1409285a0 | AudioLoopPlayer::request_play | should_play=1→bounds panic→set_position→set_parameter(editing_speed)→Active 才 set_volume ✓ | 错误路径：set_volume unwrap vs `?` |
| 0x140a596b0/59430/593f0 | AudioLoop start_if_not_playing / stop / current | 布局（+0x20 UNSET/+0x34/+0x35/+0x36/+0x28 cursor）与阈值/cursor 回绕 u32 逐条对上 | 错误路径：entry.stop 失败 panic vs `?` |
| 0x140a89480 | garden_audio_system | 4 loop 门（水栅格 0.01 gate+count）与音量模板；native 每 loop 一次采样+update_draw_audio×4，Rust 单采样+4 循环驱动——输出等价（采样输入帧内不变），登记 | — |
| 0x140a88fa0 | garden_audio_init_system | 4 loop ✓ | — |
| 0x140a72810/71f20 | music_audio_system / init | 富集度链（target+=game_delta/120→clamp→snap_off_boundaries 0.24/0.25/0.49/0.5/0.74/0.75→time_continuous_lerp→richness snap）+ menu 门（+0x100 tag）+ player.update/photo/preview 处理；init 建 Ambience/mus_GladePreview/GladePreview/player ✓ | prior reviewer 已复核门与 photo 细节；egui 闭包逐条受 decomp 局限 |
| 0x14083c4a0 | music_debug_gui | egui Window + force@+0x100 + richness@+0x1c0 读取 ✓（闭包内容受 decomp 局限） | — |
| 0x140a0b4c0/0b340 | menu_audio / init | `(mode|8)==0xa` 门禁 → snapshot stop/start；13 kind 线性表 instantiate+start_if_not_playing | 13 串逐条 rne-read prior 已验；本审计未逐条重读，登记 |
| 0x140a59220 | AudioCore::instantiate_loop | desc{count=3,max=50} 结构（init 各系统 decomp 均见 `local_c8=3; uStack_c0._0_4_=0x32`） | — |
| 0x140a7abb0 | AsyncLoadedAudioResources::new | 24B mpmc channel 对 | — |

## 二、未实现-需重做（核心逻辑分歧，附分歧点）

| VA | 函数 | 分歧点（以 fresh decomp/disasm 为准） |
|---|---|---|
| 0x1408b7690 | **wall_audio_ui** | **跳表 0x142a9fa44 全表实测**：kind0 设**全局 "StructureSize"**（0x142a9f710,len0xd）值=f32@+0xc——Rust 用 `"wall_style"+flag`（参数名与值源双错）；kind1 走公共瞬发**默认事件**（0x142a9f730）——Rust 跳过；kind9 选择器为 **`event:/SFX/Flags/ToggleDisconnect|ToggleConnect`**（byte@+9）——Rust 误用 Chimney smoke；kind0x11/0x12 → **`event:/SFX/WallSplit`**——Rust 跳过；kind0x15/0x16 表项指向数据地址（原版自身坏项）；事件布局 {kind@+8,byte@+9,value f32@+0xc,pos@+0x10,stride0x20}——Rust PlayWallUiSound 缺 value@+0xc 且 flag 语义错位。仅 2..8/0xa..0xe/0x10/0x13/0x14 逐条对 ✓ |
| 0x14089a280 | **AudioInstance::drop (impl$13) + stop_on_drop(0x140a57540)** | native Drop：disc→0；**仅当 state@+0x28≠2**（stop_on_drop 置 0）才 `stop()`（失败 drop 不 panic）；再 `release().unwrap()`。Rust Drop：Active 即 `event.stop(0)`，**无视 state 标志、无 release**；`stop_on_drop` 方法（置 state=0）缺失。后果：一次性音（instantiate→start→drop，state=2）在 native 不停止，Rust 会立即停——一次性声音被截断 |
| 0x140a53a00 | **AudioCoreInner::update** | native 先断言 `|forward|≈1 ∧ |up|≈1 ∧ forward⊥up`（0.001 容差，违反→panic），Rust 无断言；Studio::update 失败 native 打 `"FMOD update error: …"`（0x142ad29c0）后继续，Rust 传播 Err |
| 0x140a53db0 | **AudioCoreInner::set_parameter** | 失败路径 native 格式化 `"Couldn't set global FMOD parameter: …"`（0x142ad2a10）+ BTreeMap 全局报告注册（槽 0x143634d60）+ 日志；Rust 仅返回 Err |
| 0x140a54810 | **AudioCoreInner::set_vca_volume** | get_vca 失败→log+继续；Vca::set_volume 失败→unwrap(panic)；Rust 两路都传播 Err |
| 0x140a53870 | **Bank::load_sample_data** | native 首断言 loaded（未 loaded → unwrap_failed），并 unwrap load_sample_data 结果；Rust 无 loaded 断言 |
| 0x140873630 | **water_audio_system** | native 驱动 draw/erase 两 loop 后**无条件 stop 两个 ice loop**，且有字节@+0x58 双分支；Rust 只驱动 loops[0..2]，ice loops 永不停止 |
| 0x140873140 | **water_audio_init_system** | native `if (_DEV_OVERRIDE_IS_DEMO_MODE==0)` 才建 4 loop；Rust 无条件建 |
| 0x1414638e0 | **tree_audio_system** | native 按最近树仅驱动 **3 个 loop**（state+0x38/+0x70/+0xa8），第 4 loop 不驱动；Rust 驱动全部 4 个 |
| 0x1409eafa0 | **trigger_audio_system** | native：`volume@+0x58==0`（非 NaN）→早退；相机眼位读写（piVar10+0x30 标志/0x31/0x33）；命中触发 **Camera Tree Rustle/Camera Whoosh 瞬发**；wind 音量=segment/阈值×0.0125×距离衰减+lerp；InForest z≥11/dist≤70 门。Rust：无 gate、tree_rustle/camera_whoosh **从不播放**、音量/lerp 常量自造、CameraSegment 未接线 |
| 0x1409d5ea0 | **sheep_audio_system** | fresh decomp 为 ~75KB 大函数：**跳表 0x142ac2cc8 分派 case 0..0x37+，另嵌套表 0x142ac2ce8**，全部引用 sheep 7 事件串；frame-end loop 驱动（0x1409d61d6/61ed）匹配。Rust 仅实现 kind 0-7+kind4 loop——**8+ 分支（含 Pitch/参数差异）未实现**。需定向补挖确认 8+ 是否另一事件流 |
| 0x1414618c0 | **terrain_audio_system** | native 每个 start 的 loop 走 `start_if_not_playing→set_parameter→set_position`（set_parameter 值=SystemParam@+0x1c）；Rust 仅 start_if_not_playing→set_position，**缺 set_parameter**；variant→loop 跳表 0x142c63274 未逐条对账 |
| 0x1409d0f80 | **path_audio_system** | native：栅格采样+gate 后 **4 次 update_draw_audio（garden closure）+1 次（path closure）+无事件时 4×AudioLoop::stop**；Rust 3 次调用驱动 [0..2]/[2..3]/[3..5]——模板调用数与 loop 范围映射未对上，需实参级对账 |
| 0x140a889b0 | **firstperson_steps** | native：FirstPerson 参数读相机视图；ground@+0x4d0==3(Terrain)且移动→SSBO BufferState(0x3f 名断言)+CameraMatrices::eye_position+`bilinear_sample_at_world_xz` 多次采样定材质→GroundType 参数→ground==0 才走 loop。Rust：FirstPerson **硬编码 1.0**、state.ground **永为 Grass**（无采样更新）、无相机/移动门 |
| 0x141462dc0 | **terrain_stairs_audio_system** | native：AppMode==5 早退后**读 SSBO 栅格值并与状态比较**（相同→stop ring+return；不同→写状态+建 `event:/SFX/StairDraw_OneShot`+push+ring start）+ inverse_lerp；SuppressPathSound 事件非驱动源。Rust 用 SuppressPathSound 事件驱动 ring/one-shot——**自造耦合** |
| 0x140a89bb0 | **wall_audio_path** | 空壳：仅 `loops[0].frame_end()`，无事件源/无参数（self 注"待取证"） |
| 0x141234bf0 | **wall_audio_orchestrator** | 空循环：读 RequestWallMaterialAudio 事件后**无任何动作**（self 注"待取证"）；native 另有 `simplified_curve_2d.is_valid()` 断言 |
| 0x140a67b90 | timberframe_audio | kind→loop/one-shot 表 self 注"待取证"（Rust 用 kind-3 猜测）；需与 0x140a67b90 跳表逐条对账 |
| 0x14145dc20 | decorator_audio_system | kind_to_loop 为近似（self 注 partial）；native 收集+距离稳定排序+逐 loop stop/start 对驱动未全验 |
| 0x1417d2ad0 | **building_audio** | 事件 kind 6/7/8 才走 one-shot；**loop kinds{0,1,2,3,4,9,5} 事件被 `kind>=6&&<9` 过滤丢弃 → 7 个 loop 的 should_play 永不置位 → frame_end 恒 stop**；BuildingSize=editing_speed 为占位（native=inverse_lerp(area)） |
| 0x140aa3860 | roof_audio_system | 只驱动 3 loop 的 start/stop；**roof 编辑事件→8 one-shot 分派与 loop 参数（RoofHeight/RoofProfile/RoofTipMoveSpeed/RoofTipOffset/ImpactStrength/isHalfTimbered/is_gable）全缺**（self 注"待接线"） |
| 0x1409ecea0 | **water_ambience_audio_system** | native 有 `_DEV_OVERRIDE_IS_DEMO_MODE` 门；cell 激活按 cell "exists" 标志；逐 active cell `start→set_position(eye_position+x0y)→set_volume`；Rust 用 `water_level>=0.5` 自造阈值、无 set_volume、位置来自永不填充的默认 cell |
| 0x1409d6b90 | signifier_audio_system | native 驱动 21 参数+具体瞬发；Rust 仅对 4 实例设 `"Tool"` 参数+hold_time stop（近似） |

## 三、需对账/未 fresh 复核（登记）

- `recovered_systems::audio::WALL_KIND_TABLE` 支撑 wall_audio_fmod kind 0..23（0x140950920）——本审计确认结构（loop kinds→request_play、one-shot kinds→instantiate_audio_w_position_and_volume、跳表 0x142ab4998）匹配，但 **frame-end 音量调制 `powf(0.1, speed×5.0)`（decomp 行 564）在 Rust 未复现**——中分歧。
- `add_audio_systems`（0x1409411a0）/`add_audio_events`（0x140941450）：注册清单 prior 已验 16 事件；系统注册批与 0x140b6bd50 契约一致性未在本审计逐条重验。
- `glade_audio.rs`/`load_glade_audio.rs`/`audio_core_backend.rs`（music/ 下）：inventory 0 原函数（非本审计业务范围）；`load_glade_audio` 的主题旗标写入已在 music_player 注释登记。
- `audio_settings.rs`：serde 胶水，0 业务（协议排除）。
- `weather_audio_system`：inline（行 5，无独立符号）——仅按 prior rne-read `"RainStrength"` 确认参数名；系统单 set_parameter 结构一致，登记限定。
- `ivy_audio.rs`：init（0x140ad5350）为 log 占位（prior PARTIAL 登记维持）；本审计未 fresh 重验。

## 四、系统性结论

1. **判等**：music_player 全部 8 个业务函数 + get_timeline_beat_properties、clutter（39 表 0..0x12 逐条）、AudioInstance 判别/位掩码 0xb、Bank::load、AudioLoop 族、garden_audio（4-loop 门）、audio_update（5 VCA）——以上正常路径与 fresh decomp/disasm 逐条对应，**真实实现**。
2. **未实现-需重做（核心分歧）**：wall_audio_ui（kind0/1/9/0x11/0x12 + 事件布局 5 处分歧）、AudioInstance Drop/stop_on_drop（state 门缺失→一次性音截断）、AudioCoreInner update/set_parameter/set_vca_volume 错误报告路径、Bank::load_sample_data loaded 断言、water（ice loops+demo 门）、tree（loop 数）、trigger（gate+双瞬发缺失）、sheep（0x38+ 分派未实现）、terrain_stairs（SSBO 驱动 vs 事件自造）、firstperson（stub）、water_ambience（自造阈值）、signifier/building/roof/wall_path/wall_orchestrator/timberframe/decorator（空壳或近似）。
3. **错误路径注记（不降级但须登记）**：AudioInstance 全部 FMOD 方法 native 均 `unwrap()`（失败 panic）或格式化报告+继续，Rust 统一 `?` 传播 Result——正常路径等价，错误语义分歧；若按协议"数据流错"从严可批量转未实现，本审计判定为注入 trait 边界语义，登记待定夺。
4. **实质发现**：wall_audio_ui 的 kind0 参数名（"StructureSize" 非 "wall_style"）、kind9 事件族（Flags 非 Chimney）与 sheep 0x38+ 分派、AudioInstance Drop state 门、building loop kinds 从不驱动——均为 prior RESTORE/REVIEW 声称"逐条对上"但**实际分歧**之处，与用户硬标准（此前"结构对上了但函数内部逻辑全是假的"）直接相关。

## 五、证据

- decomp/: batch1-priority（20）、batch2-audiocore（16+跳表）、batchA-systems（20）、batchB1（20）、batchB2（13）共 89 份 fresh 反编译。
- disasm-musicupdate.txt（0x1408b5500 状态机 400+ 指令）、disasm-wallui.txt（0x1408b7690 431 指令）、disasm-switchtrack.txt（0x1408b70d0 177 指令）——radare2 于 RNE。
- 常量 rne-read：1.5@0x1429258d0、120.0@0x1429cd0f8、18.5@0x142a9f400、19.0@0x142a9f404、18.0@0x142a7ba80；wall_audio_ui 跳表 0x142a9fa44 全 0x17 表项；clutter 串 0x142aa0e4c（0..0x13）。
- 结果原始 JSON：results/*.json。
