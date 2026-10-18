# restorer-REBUILD.md — audio 域 21 项重建回执（restorer，2026-10-17）

> 依据：judge protocol `planning/2026-10-16-logic-verify-protocol.md` + SOP
> `2026-10-17-recovery-sop.md`。只以 fresh 反编译为准：
> `restorer-audio-rebuild/decomp/`（ghidra `--worker shared` 两批 +
> radare2 RNE 定点 disasm：`wallui-tail/trigger-*/tree-*/sheep-dis*/terrain-*/
> fp-*/stairs-*/roof-*/tf-*/sig-*`），原始 JSON 在 `restorer-audio-rebuild/results/`。
> 只改 audio 域文件（`src/systems/audio/*`、`src/resources/audio_resources.rs`）。
> 编译：`cargo check -p country-core --lib` **通过（0 error）**；audio 单测 74 个
> （含本轮新增断言）。`cargo test --lib` 当前被**他组在途** `systems/save/snapshot.rs`
> （未提交改动，`DecoratorStorage` 缺 import，2×E0433，非 audio 域）阻断，
> 未代改；解锁条件：该文恢复编译后跑 `cargo test -p country-core --lib audio`。

## 一、逐函数 verdict

| VA | 函数 | verdict | 关键点 |
|---|---|---|---|
| 0x1408b7690 | wall_audio_ui | **判等**（修 1 处） | 跳表 0x142a9fa44 全 0x17 项实测；kind0→全局 `StructureSize`=f32@+0xc + `event:/SFX/Bulldoze`；kind1 公共瞬发默认 `WallSnapToEnd`；2..8/0xa..0xc/0x13/0x14 串名逐条；kind9 Flags ToggleDisconnect\|ToggleConnect；0xf Chimney ToggleSmokeOff\|On；6 `isHalfTimbered`、0x10 `wall_style`={0,1,2}[sel]；0x11/0x12 与未列 kind→默认 `WallSplit`；0x15/0x16 表项指向自身数据（原生坏项，跳过）。**修复**：kind0xd `LanternColor` 原生 `mov eax,esi; cvtsi2ss` = 将 f32 位型当 u32 值转 float，Rust 原直接传 float → 改为 `value.to_bits() as f32` |
| 0x14089a280 / 0x140a57540 | AudioInstance::drop + stop_on_drop | 判等 | disc 先清 0；仅 state@+0x28≠2 才 stop（错误丢弃）；`release().unwrap()`；stop_on_drop 置 0 |
| 0x140a53a00 | AudioCoreInner::update | **判等**（修 NaN 语义） | 三断言 `0.001 <= |v|-1` / `0.001 <= |dot|`；**修复**：NaN 时原生不 panic，Rust `x<0.001` 会 panic → 改为 `assert!(!(0.001 <= …))`；set_listener 失败 panic；update 失败 log 后继续 |
| 0x140a53db0 | AudioCoreInner::set_parameter | **判等**（补错误报告） | 失败格式化 "Couldn't set global FMOD parameter: …" → 写入全局 BTreeMap（槽 0x143634d60）+ log，然后继续；已补 `GLOBAL_AUDIO_PARAM_ERRORS` 与只读快照 |
| 0x140a54810 | AudioCoreInner::set_vca_volume | 判等 | get_vca 失败 log+继续；Vca::set_volume 失败 unwrap(panic) |
| 0x140a53870 | Bank::load_sample_data | **判等**（修日志级别） | `*loaded&1==0 → unwrap_failed`；日志仅在 `MAX_LOG_LEVEL_FILTER>4`（Trace）→ info 改 trace；load_sample_data 失败 unwrap |
| 0x140873630 / 0x140873140 | water_audio_system + init | 判等 | init demo==0 才建 4 loop；两分支（state@+0x58==4 / else）代码等价：draw/erase 各一次 update_draw_audio 后无条件 stop 两个 ice loop |
| 0x1414638e0 | tree_audio_system | **重建** | `next` closure 0x1412f0680：距离=树位置到 **camera eye**；稳定排序；仅 slots 1..3（+0x38/+0x70/+0xa8）跟随最近三树；kind1 驱动 slot0(TreeMove)：selector==2→stop，否则 start+pos+`editing_speed`(mode 0xd?1.0:raycast.speed)+`in_water`=sel&1+`tree_size`；kind0 TreePlace：`tree_size`→pos→start→`in_water`(sel!=0)→`is_naked`(extra!=0)→drop；kind2 TreeDelete：`tree_size`→pos→start |
| 0x1409eafa0 | trigger_audio_system | **重建** | speed==0(非 NaN) 早退；eye + 移动段；`movement/speed>1` 才对 **每侧 -0.2 收缩**的 AABB 做 intersect_segment；命中 +0.5 否则 -0.1，clamp[0,1]；audibility>0 start rustle else stop；`camera_height`=\|eye\|×(ground flag?0.1:1)；wind target=factor×(movement/speed)×0.0125（in_forest ×10），lerp 0.1/0.2，≤2；`camera_speed`=wind；whoosh+high_wind 每帧 start；wind 实例音量 1.0 或 1+wind×0.5；`eye.y>=11 \|\| \|eye\|<=70`→stop InForest+InForest=0，否则 start+1 |
| 0x1409d5ea0 | sheep_audio_system | **重建** | 函数边界 0x1409d5ea0..0x1409d6201 `ret`；表 0x142ac2cc8 仅 0..7 指向本函数，8..0x51 指 `signifier_audio_system`（0x1409d6fb8..，Ghidra 合并伪影；事件 kind 域 0..7）。kind0..3/5..7 瞬发：instantiate→set_position(pos@+8)→`Pitch`@+0x14→start→drop；kind4 仅置帧尾 loop 标志（不设参数）；帧尾 start/stop |
| 0x1414618c0 | terrain_audio_system | **重建** | 跳表 0x142c63274 仅 0..5；kind0→slot2 仅 flag；kind1→slot0：`terrain_smoothness`=v18、speed=clamp01(lerp(0,5,v1c/speed))；kind2→slot1：smoothness=v18、`terrain_height`=lerp(0,20,v1c)、speed=clamp01(lerp(0,5,v20/speed))；kind3→slot3 同 kind2；kind4→LowPoint 瞬发（height=lerp(0,20,v18)）；帧尾顺序 **[2,1,0,3]**：flag→start+`editing_speed`+set_position(共享最后事件位置)，else stop，清 flag |
| 0x1409d0f80 | path_audio_system | **结构重建 + 登记** | 5 loop（0x00/0x38/0x70/0xa8/0xe0）；第一流有事件才进驱动块，否则 stop [2,1,0,4]；驱动 [2,1,0,4] 后**无条件**驱动 +0xa8；+0x70 谓词=`event && byte@+0x14==0 && raster<thr`，+0x38=`event && (byte@+0x14&1)!=0 && raster<thr`。登记：其余 closure 谓词/位置半径、raster gate 阈值在宿主 closure |
| 0x140a889b0 | firstperson_steps | **重建 + 登记** | `FirstPerson`=(camera@+0x4d0==3)?1:0；ground==3 且移动→采样链；`GroundType` 缺省 2.0，采样 >0.1→4.0 否则 0.0；loop gate=camera@+0x344==0。登记：SsboId 11 `TerrainRockMask`（名断言 len 0x3f）/bit 栅格、ActionSetStates+0x150 细节 |
| 0x141462dc0 | terrain_stairs_audio_system | **重建 + 登记** | AppMode==5 早退；无事件→直接 return（不停 ring）；editing_speed=clamp01(lerp(0,60, mode 0xd?1:raycast.speed))；retain 保留 **Active&&Playing**；Vec 空且 ring 当前 Playing → 计数/事件；SSBO 值相同→stop ring+return，不同→存值+StairDraw_OneShot(start+`editing_speed`)+push+ring start+`editing_speed`。登记：事件流类型、SsboId/栅格语义 |
| 0x1417d2ad0 | building_audio | **判等（结构）+ 登记** | filter_map 收集；loop kinds {0,1,2,3,4,9,5} **不被** `kind>=6&&<9` 过滤（旧 LOGIC 结论已被 fresh decomp 推翻；现码已按此）；kind 6/7/8 瞬发。登记：7 loop 的 BuildingSize/ifWindow/BuildingHeight/isHalfTimbered/IsOnGround 来自 0x2a8 墙表（他域） |
| 0x140aa3860 | roof_audio_system | **重建** | acc0/acc1+=speed；kind0/1 累积 >0.2 瞬发+清零；kind2 全局 `RoofHeight`=v0、`RoofProfile`=v1，loop0 源=v2；kind3 全局 `RoofTipMoveSpeed`=v1、`RoofTipOffset`=v0；kind4 `ImpactStrength`=v0；kind5/7/0xa/0xb 瞬发；kind6 `isHalfTimbered`=sel!=0；帧尾 loop0：selector==2 stop，否则 start+pos+`editing_speed`=clamp01(lerp(0,5,v2/speed))+`is_gable`=sel&1；loop1/loop2 start+pos |
| 0x1409ecea0 | water_ambience_audio_system | 判等 | demo 门；cell bits!=0 → start+set_position(x0y)+set_volume(sqrt(pop/(1+dist²/(pop·0.015625·42.5))))（state==1 守卫），否则 stop |
| 0x1409d6b90 | signifier_audio_system | **部分重建 + 登记** | 已判等：stride 0x10 流 tag0→`event:/UI/camera/FirstPersonOff`（len 0x1f），否则 On（0x1e）（字符串小写 camera，已修）；帧尾 hold-time loop stop。登记：另一 stride 0x20 流（跳表 0x142ac2db8，kind 0..7）的 21 个全局 `Tool`/参数值、`event:/UI/UI Tool Change`（len 0x18）触发条件、6 个事件流类型 |
| 0x140a89bb0 | wall_audio_path | **结构重建 + 登记** | 非 loop 驱动：3 个流（0x20/0x20/0x10）→ push 0x20B 元素（tag=2、bool=align.byte@+0x18==1）。登记：`Vec<PreferredNodeAlignment>` 属外部 `system_decorator`，已建字节兼容 host 面 `WallPathAudioOut` |
| 0x141234bf0 | wall_audio_orchestrator | **结构重建 + 登记** | 非 set_parameter：`hash_one<GlobalId>` 墙表→`MaterialStats::which_material_changed_the_most/max_pillar_movement`→`EventWriter::write<PlayWallAudio>` + indexmap `(DecoratorId,DecoratorId)→AlignKind`；`assert!(simplified_curve_2d.is_valid())`。已建 host 面 `WallMaterialAudioHost` 并发 PlayWallAudio；登记：MaterialStats 方法/AlignKind/墙表 |
| 0x140a67b90 | timberframe_audio | **重建** | 事件布局 kind@0,flag@+1,src@+4,pos@+8,val@+0x14；kind1→slot1(Length)、kind0→slot2(Width)：`editing_speed`=clamp01(lerp(0,30,src/speed))；kind3→slot0(Move)：clamp01(lerp(0,250,src))；每 loop `BuildingSize`=val，Move 另有 `is_on_ground`=flag&1；2/4/5/6/7 瞬发 |
| 0x14145dc20 | decorator_audio_system | **判等（结构）+ 登记** | 16 loop 槽（+0x10..+0x358）、分布/稳定排序、14 个 one-shot 名与长度、帧尾逐槽 stop/start。登记：slot↔kind、事件 stride 0x28 的 selector@+9/value@+0xc/pos@+0x18 载荷与 one-shot 参数 |
| 0x140950920 | wall_audio_fmod frame-end | 判等 | `state.volume *= powf(0.1, speed(+0x58) × 5.0)` 已实现（decomp line 564） |

## 二、关键分歧点（本轮以 fresh 证据推翻/修正的旧结论）

1. **sheep “0x38+ 分派”**：跳表 0x142ac2cc8 的 8..0x51 目标位于 `signifier_audio_system`（0x1409d6fb8+）；sheep 函数体止于 0x1409d6201 `ret`。kind 域仅 0..7。
2. **building loop kinds**：fresh decomp 中 loop kinds 被 filter_map 收集并逐 loop 匹配；`kind>=6&&<9` 只决定 one-shot 分支，不是丢弃 loop 的过滤（旧 LOGIC 结论错误；现码已按新鲜证据）。
3. **wall_audio_ui kind0xd**：`LanternColor` 是“位型→u32→float”，不是原 f32 直传。
4. **firstperson**：`FirstPerson` 参数来自 camera ground==3，不是“相机存在”；`GroundType` 原生值为 {0,2,4}（缺省 2），不是 0..3 枚举；loop gate 是 camera+0x344==0。
5. **terrain_stairs**：驱动源是 SSBO 值比较 + 事件流，不是 SuppressPathSound；retain 保留 **Active&&Playing**。
6. **tree**：距离基准是 camera eye；kind1 确实驱动 TreeMove loop（旧文档写“忽略”错误）。
7. **timberframe**：kind1→Length、kind0→Width（旧文档反）；事件载荷含 flag/src/value。
8. **roof**：loop0 的 editing_speed/is_gable 值源；kind2/3/4 参数分别来自 value0/1/2，非 position.length。

## 三、登记（缺宿主/跨域，写明解锁条件）

1. **signifier 21 参数**：解锁=得到 stride 0x20 signifier 事件流类型（OnWallChanged/ModeJustChanged/LoadingGameComplete/InitAdjWallHeightMode/FinishAdjWallHeightMode/PlayUiSound/PlayPhotomodeSound）+ 记录字段语义；届时按 0x142ac2db8 逐 kind 落 `Tool` 值与 `UI Tool Change` 瞬发。
2. **wall_audio_path / orchestrator**：解锁=外部 `PreferredNodeAlignment`/`AlignKind` 与墙表 `MaterialStats::which_material_changed_the_most/max_pillar_movement`；当前 host 面已就位。
3. **building 7 loop 参数 / decorator slot↔kind + 载荷**：解锁=0x2a8 墙表 / DecoratorAudioState 完整布局与事件 producer。
4. **firstperson rock-mask**：解锁=SsboId 11 名断言与 bit 栅格 + ActionSetStates+0x150；当前消费 MainCamera raw 字节。
5. **terrain_stairs**：解锁=地形楼梯编辑事件流类型与 SSBO id/栅格；当前 host 面 `TerrainStairsAudioEvent`/`StairsRasterValue`。
6. **terrain 原生 kind5（TerrainCreateChunk）**：`PlayTerrainSound` 只有 5 个变体（tag 0..4），无 tag5 承载；解锁=terrain 域补第六变体。kind4→LowPoint 的字段映射（falloff→v18）按 tag 近似，待 terrain 域确认。
7. **path closures**：解锁=world_raster 阈值与各 closure 的位置/半径来源；当前只复现 +0x70/+0x38 两条可读谓词。
8. **EditingSpeed(+0x58)**：trigger/terrain/roof/timberframe 现以 `Res<EditingSpeed>` 作为必需系统参数（与原生一致），宿主需 insert；写入者仍为宿主面。

## 四、证据与代码

- 反编译/反汇编：`recovery/agent-team/reports/eco/audio/restorer-audio-rebuild/{decomp,jobs,results}/`。
- 代码改动：`src/resources/audio_resources.rs`、`src/systems/audio/{mod,wall_audio_ui,tree_audio,trigger_audio,sheep_audio,terrain_audio,path_audio,firstperson_steps,terrain_stairs_audio,roof_audio,timberframe_audio,signifier_audio,wall_audio_path,wall_audio_orchestrator}.rs`。
- 未改：water_audio / water_ambience_audio / building_audio / decorator_audio / wall_audio_fmod（fresh 判等后保持）。
