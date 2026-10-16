# restorer-audio — 卡F 音频域第三批 RESTORE-BATCH3 报告

日期：2026-10-16
状态：in_review（第三批 32 个 audio 系统文件 + AudioCore 其余方法落地）
涉及文件：`systems/audio/*.rs`（31 个新建/重写）+ `resources/audio_resources.rs`（扩展）+ `resources/audio_instance.rs`（扩展）

## 一、原函数 → Rust 位置 → 证据

### 波 1（高消费优先）

| 原文件/原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| terrain_audio_init_system / terrain_audio_system | 0x1414613e0 / 0x1414618c0 | terrain_audio.rs（4 循环 TerrainEditShape/TerrainEditHeight/Terrain Draw/TerrainMove + EditStateAudio 折叠 + 帧尾 start/stop） | fresh decomp + rne-read（0x142c62c1a/c88/cc0/cf0） |
| clutter_audio_system | 0x1408c02f0 | clutter_audio.rs（39-kind 表全解，kind 0x12 跳过；instantiate+set_position+set_parameter+start_if_not_playing） | fresh decomp 逐 case + rne-read 串表 0x142aa0e4c |
| garden_audio_init/system | 0x140a88fa0 / 0x140a89480 | garden_audio.rs（4 循环 GardenDraw/GardenDrawWater/GardenErase/GardenEraseWater；update_draw_audio 模板驱动） | fresh decomp + rne-read（0x142ad8f50/f90/fc8/d74f4） |
| terrain_stairs_audio_system | 0x141462dc0 | terrain_stairs_audio.rs（AppMode==5 早退；retain 清理 + StairDraw_OneShot 瞬发 + SuppressPathSound） | fresh decomp + rne-read（0x142c62e70） |
| sheep_audio_system | 0x1409d5ea0 | sheep_audio.rs（kind 0-7 表；kind2 名纠错 So→Sob；kind4 帧尾 loop；FirstPersonOff 参数） | fresh decomp + rne-read（0x142ac23b0） |
| decorator_audio_init/system | 0x14145c6b0 / 0x14145dc20 | decorator_audio.rs（16 循环 WindowDrag/DecoratorMoveWoodenFloorTEMP/Lantern*/Chimney*/Flags*/Cosmetics*；事件→loop 驱动） | fresh decomp（字符串常量逐条） |
| path_audio_init/system + update_draw_audio×2 | 0x1409d08d0 / 0x1409d0f80 / 0x14166d890 / 0x14166d9f0 | path_audio.rs（5 循环 CobblestoneDraw/Path Draw/Erase Draw/PathWaterDraw/PathWaterErase；update_draw_audio 共享模板） | fresh decomp + rne-read（0x142ac1660/1690/16c0/16f8） |
| roof_audio_init/system + instantiate_audio_w_position[_and_volume] | 0x140aa2fd0 / 0x140aa3860 / 0x140aa3360 / 0x140aa3590 | roof_audio.rs（3 循环 RoofShape/RoofTipMove/InterRoofMerlonsRearrange + 两个共享 one-shot 助手） | fresh decomp + rne-read（0x142adcb38/b68） |
| tree_audio_init/system | 0x1414633e0 / 0x1414638e0 | tree_audio.rs（4 循环 TreeMove+3×Amb；Query 最近树距离排序驱动前 3 循环；TreePlace/TreeDelete 瞬发 + tree_size/is_naked 参数） | fresh decomp + rne-read |
| water_audio_init/system | 0x140873140 / 0x140873630 | water_audio.rs（4 循环 WaterDraw/WaterErase/IceErase/IceDraw；事件驱动 draw/erase 对） | fresh decomp + rne-read |
| weather_audio_system | 行 5（内联） | weather_audio.rs（Weather 状态→全局参数；参数名 U） | 章节（待取证） |

### 波 2

| 原文件/原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| wall_audio_fmod + init | 0x140950920 / 0x14094fb80 | wall_audio_fmod.rs（10 循环 Wall Draw 等；kind 0-23 表复用 recovered_systems::audio WALL_KIND_TABLE；request_play+set_parameter 参数化；one-shot；帧尾 start/stop） | multi-baseline §1/§3 + recovered_systems::audio 已验证表 |
| wall_audio_utils::AudioLoopPlayer::request_play | 0x1409285a0 | resources/audio_instance.rs（AudioLoopPlayer::request_play 扩展 position 参数 + frame_end；wall_audio_utils.rs 合同模块） | DIGEST §2.5 + 现有代码扩展 |
| wall_audio_ui | 0x1408b7690 | wall_audio_ui.rs（kind 6/0xd/0xe/0x10 → ColorBricks/ColorLantern/Lantern TurnOn/ChangeWallStyle；其余待取证） | fresh decomp + rne-read |
| wall_audio_path | 0x140a89bb0 | wall_audio_path.rs（U 为主：Wall Draw loop 帧尾驱动；事件源待接线） | 章节 |
| wall_audio_orchestrator | 0x141234bf0 | wall_audio_orchestrator.rs（RequestWallMaterialAudio 消费面；参数映射 U） | 章节 |
| signifier_audio_init/system | 0x1409d6320 / 0x1409d6b90 | signifier_audio.rs（1 loop HoldTimeSiginfier + 4 one-shot UI Boop/Cursor Grab/Release/UiRotateGizmoMovingBack；UiSignifierStream→参数面） | fresh decomp + rne-read |
| timberframe_audio + init | 0x140a67b90 / 0x140a67810 | timberframe_audio.rs（3 循环 Move/Length/Width + 5 one-shot Rectangle/CircleAppear/Disappear/StartEdit/Appear；kind 分派） | fresh decomp + rne-read |
| firstperson_steps + init + GroundType::to_fmod_param | 0x140a889b0 | firstperson_steps.rs（GroundType 枚举 + FirstPersonOff 全局参数 + 行走 loop 开关；地形栅格双线性采样待接线） | fresh decomp + rne-read |
| forest_audio_init/system | 0x140aa18f0 / 0x140aa1f00 | forest_audio.rs（单 loop ForestGrow；Query<&ForestTree> 存在性 + EscMenuState 门禁驱动） | fresh decomp + rne-read + procs.tsv 签名 |
| game_loading_audio::init_game_loading_audio / loading_audio | 0x140b1ae70 / 0x140b1b1b0 | game_loading_audio.rs（snapshot:/Loading 启动即 start；event:/UI/Loading 进行时；完成→停快照 + 1s 二次保护） | fresh decomp + rne-read |
| menu_audio + init | 0x140a0b4c0 / 0x140a0b340 | menu_audio.rs（snapshot:/PauseMenu + AppMode (mode|8)==0xa 门禁；13 kind 线性表） | fresh decomp + rne-read（0x142ac6e30 串表） |
| trigger_audio_init/system | 0x1409ea8d0 / 0x1409eafa0 | trigger_audio.rs（4 one-shot Camera Tree Rustle/High Wind/Camera Whoosh/InForest；Aabb3::intersect_segment 命中→audibility±0.5/−0.1 clamp；音量 0.0125 lerp；InForest z≥11/dist≤70 门） | fresh decomp + rne-read |
| building_audio + init + get_building_size_01 | 0x1417d2ad0 / 0x1417d21a0 / 0x1417d3f10 | building_audio.rs（7 循环 BuildingResize 等；kind 6/7/8 one-shot DetachFromGround/AttachToGround/PeakReached；帧尾驱动） | multi-baseline §2/§4 + rne-read |
| elevation_supports_audio | 0x140ad51c0 | elevation_supports_audio.rs（kind→3 one-shot WoodenBracketsAppear/BrickColumnAppear/WoodenColumnAppear → instantiate_audio_w_position_and_volume） | 章节 + rne-read |
| playermade_stairs_audio::stairs_audio_system + init | 0x140f880a0 / 0x140f878d0 | playermade_stairs_audio.rs（6 循环 StairMove 等；kind≤18 跳表；循环/瞬发分派 + stair_width 参数） | multi-baseline §5 + rne-read |
| replay_audio_init/system | 0x140ad54d0 / 0x140ad5800 | replay_audio.rs（Undo/Redo 存储实例 + kind 2→DenyInput 瞬发；AppMode==0xd 全局参数 U） | fresh decomp + rne-read |
| water_ambience_audio::load_index_unchecked / init / system | 0x1409ebe80 / 0x1409ebf20 / 0x1409ecea0 | water_ambience_audio.rs（按水网格逐格 Amb loop；load_index_unchecked 网格装载；系统按水位阈值驱动 stop/start+position） | fresh decomp + rne-read |

### AudioCore 其余（resources/audio_resources.rs）

| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| AudioCore::instantiate_audio | 0x140a55f40 | audio_resources.rs（非空 core→Inner producer→Active；空→Disabled） | 章节 |
| AudioCoreInner::instantiate_audio producer | 0x140a54440 | Studio::create_event_instance（注入面） | 章节 + DIGEST §2.1 |
| AudioCore::instantiate_loop | 0x140a59220 | AudioCore::instantiate_loop / instantiate_loop_player（AudioLoopDesc{name,count=3,max=50}） | audio-events §3a |
| AudioCoreInner::set_parameter | 0x140a53db0 | AudioCore::set_parameter（全局参数） | 章节 |
| AudioSamplesLoadedPromise::recv | 0x140a57100 | AudioSamplesLoadedPromise::recv（crossbeam recv_timeout 1s；超时丢弃；断开 panic） | DIGEST §2.2 |
| AudioCore::visualize_sounds | 0x140a55ff0 | AudioCore::visualize_sounds（实例位置输出面；渲染待接线） | 章节 |
| AudioCore::set_master/music/dawn_jingle/sfx/ambience_volume | 行 233-249 | set_*_volume（VCA 包装） | 章节内联 |
| DelayedBankLoader | 行 64（内联） | DelayedBankLoader（延迟加载包装；PDB 名错位证实） | DIGEST §2.2 |

## 二、关键决策与依赖边界

1. **FMOD 注入面扩展**：`Studio` trait 新增关联 `Event` + `create_event_instance`（0x140a54440 生产者）与 `set_parameter_by_name`（0x140a53db0），均为 `&self`（匹配 native Res<AudioCore> 不可变签名，procs.tsv 证实）。所有系统泛型于 `S: Studio + Send+Sync+'static`（同 audio_update 既成模式）；真实 libfmod 绑定=库外（native.rs 同构）。
2. **循环双模型**：`AudioLoop`（utils，游标环 start_if_not_playing/stop）用于 tree/water/roof/forest/decorator/garden/path/water_ambience/terrain/terrain_stairs/firstperson/signifier；`AudioLoopPlayer`（resources/audio_instance，request_play+帧尾 should-play）用于 wall/building/stairs。`AudioCore::instantiate_loop` 与 `instantiate_loop_player` 分别产出。
3. **AudioLoopPlayer 扩展**（改既有 review-passed 文件，登记）：`LoopInstance` 增 `start`/`stop` 与 `set_position(position:[f32;3])`；`request_play` 增 position 参数；新增 `frame_end()`（应播标志→start/stop）。测试同步更新（10 测试全过）。
4. **事件类型**（mod.rs）：16 个注册事件按 native slot stride/字段偏移建模（PlayWallAudio kind@+8/value@+c/pos@+0x10/vol@+0x1c/flag@+0x20 等，multi-baseline §8）。
5. **kind 表**：clutter 39 项全解（fresh decomp 逐 case + rne-read 串表）；sheep kind2 纠错（"So"→"Sob"）；menu 13 项线性表；wall kind 0-23 复用 recovered_systems::audio 已验证 WALL_KIND_TABLE。
6. **跨域依赖**：缺失类型（TreeAmbience 组件、WaterDrawEvent/WaterEraseEvent、PublicWalls 消费、ActionSetStates、MainCamera、InteractableForestTree、Weather 状态等）在音频域以最小本地类型建模并在文档中登记"待接线"；FMOD/loop 业务逻辑库内完整。

## 三、待取证 / 待接线（诚实登记，不编造）

1. **clutter/set_parameter 参数名**（common block 0x1408c0990 的 set_parameter 名未解析）——以 "editing_speed" 为文档化占位。
2. **sheep kind4 loop 事件名**（0x1409d61b0 帧尾分支的 loop event）——"event:/SFX/Sheep/Baah" 为最接近串表候选，未对跳表证实。
3. **firstperson WALK_LOOP_EVENT** —— 未在 RNE 串表定位，文档化占位待证实。
4. **weather 全局参数名 / Weather 状态类型** —— U。
5. **wall_audio_ui 其余 kind→事件映射**、**wall_audio_path 事件源**、**wall_audio_orchestrator 参数映射** —— U。
6. **decorator kind→loop 完整映射**、**roof_audio_system 事件类型** —— U。
7. **水/地形栅格采样消费**（water Ice loops、firstperson 双线性、terrain_stairs SSBO 比对）——跨域待接线。
8. **replay AppMode==0xd 全局参数名** —— U。
9. **systems/wall/snapping/snapping_audio.rs** —— 属 wall 域（非 systems/audio），登记待 wall 组接线。
10. **resources/audio_settings.rs** —— 0 业务过程（serde 胶水）；AudioSettings 结构已在 audio_update.rs（首批）覆盖。

## 四、编译与测试

- `cargo check -p country-core`：通过（仅 3 条他组在途警告：ON_LOAD_FENCE/LoadUpgradeFlags/HOUR_TO_MS）。
- `cargo test -p country-core --lib`：1665 过 / 1 败（`geometry::sphere`，他组在途，与 DISPATCH 基线一致）。
- 音频域测试：46 过（含新 33 + 既有 13）。
- **不宣布整模块完成**：待取证项未闭合前为 in_review。

---

## 五、中央转发 E3/E5 取证应用（2026-10-16 追加）

### E3（sheep kind4=Baah 循环，CONFIRMED）— 已应用
- 事件名表 @0x142ac23b0：kind 0–7 = Aww/Think/Sob/Angry/Idea/Eyes/Confused（7 一次性，**kind2 为 "Sob" 非 "So"**，rne-read 复核）；kind4 → 打包循环句柄 → 帧尾 start_if_not_playing/stop（0x1409d61b0）。
- 修正：`sheep_audio.rs` 的 `KIND4_LOOP_EVENT` 由"待取证候选"改为 **CONFIRMED**（`event:/SFX/Sheep/Baah`，rne-read + startup_sheep 载入 Baah+Pitch@0x142ac2460）；kind4 现置 should_play4 并在循环当前条目上设 `Pitch` 参数（`SHEEP_PITCH_PARAM`），帧尾按标志启动/停止连续循环。一次性 kind 保留 FirstPersonOff 参数（章节证据）。

### E5（ivy SsboId NAME 定案）— 复核**未证实键等价**
- 全名串已确认在位：`IvyLeafSsbo`@0x142a9eb87 len0x3d、`IvySegmentSsbo`@0x142a9e6a4 len0x44；`IvyFlowerSsbo` 相邻串表（rne-read 全路径串）。
- **复算（默认 xxh3_64 seed 0）**：
  - Leaf 全名 → 0xa077c75cc59ea5e5（≠ 0x9e3cfa7f2993f406）
  - Flower 全名 → 0xee534dffdb141399（≠ 0x1d6de5e69ac50d2a）
  - Segment 全名 → 0x5c83edb0d7b9a73d（≠ 0x2002d2fbdff3d80f）
  - 短名 IvyLeafSsbo/IvyFlowerSsbo/IvySegmentSsbo 及 Leaf/Flower/Segment 亦不匹配。
- **结论**：全名串为真实 NAME 常量，但 ivy 三键**不是**这些名字经默认 xxh3_64 产生；`ssbo_library.rs` 的"NAME 串未定位→待取证"结论保持成立。`ivy_send_to_gpu_leaf/segments.rs` 现有处理（NAME 全名常量 + 独立 SSBO_ID 键 + SsboId::of 建模缺口注释）**无需修改**。产生 ivy 键的机制/字符串仍待取证（可能非 NameHash 来源）。

---

## 六、中央派"库内可闭合缺口"闭合（2026-10-16 追加；逐项 rne-read/disasm 定案）

| 待取证项 | 定案结果（证据） | 落地 |
|---|---|---|
| clutter set_parameter 参数名 | **"clutter-context"**（0x142aa12d0 len 0xf）；值查表 {0.0,1.0,2.0}@0x142aa1590，按事件字节@+0x15 索引（disasm 0x1408c0aa7/aab） | clutter_audio.rs：`PlayClutterSoundCmd` 增 `context:u8`；set_parameter("clutter-context", 表值) |
| sheep Pitch 值（原硬编码 0.0） | set_parameter 名 **"Pitch"**（0x142ac2460 len 5，章节"FirstPersonOff"系误读）；值=事件 f32@+0x14（0x1409d6158 `movss xmm3,[rbx+0x14]`） | sheep_audio.rs：`PlaySheepReactSound` 增 `pitch:f32`；一次性+kind4 loop 均 set_parameter("Pitch",event.pitch) |
| firstperson WALK_LOOP_EVENT | **"event:/SFX/Player/Walking"**（0x142ad8e28 len 0x19）；全局参数 **"FirstPerson"**（0x142ad8eb0 len 0xb，非 FirstPersonOff）+ **"GroundType"**（0x142ad8ed8） | firstperson_steps.rs 常量改实 + 参数面 |
| weather 参数名 | **"RainStrength"**（0x142c63000 len 0xb；内联点 0x141556935 set_parameter） | weather_audio.rs：`Weather.rain_strength` → set_parameter("RainStrength") |
| replay AppMode==0xd 全局参数名 | **"undo"**（0x142ae1778 len 4）+ **"replay"**（0x142ae177c len 6），值 1.0（0x142925904；disasm 0x140ad5d8e/dac） | replay_audio.rs：mode==0xd 时 set_parameter("undo",1.0)+("replay",1.0) |
| wall_audio_ui 其余 kind→事件 | 跳表 0x142a9fa44 全映射（kind 2 WallSnapToSegment/3 ShapeSnap/4 WallsMerged/5 WallsMergedEnclosed/6 ColorBricks/7 ColorGate/8 ColorFlag/9+0xf ToggleSmoke 选择器@+9/0xa ColorWindow/0xb ColorRoof/0xc ColorHalftimber/0xd ColorLantern/0xe Lantern/TurnOn/0x10 ChangeWallStyle/0x13·0x14 BuildingSupportsRemoveWooden；kind 0 设全局 wall_style；0x11·0x12·0x15·0x16 跳过） | wall_audio_ui.rs 全表 + ToggleSmoke 选择器 |
| roof 事件类型 / ridge 常量 | roof_audio_system 分派 roof 编辑事件（RoofAppear/RoofRemove/RoofTilesLand/RoofTilesDetach/MerlonsAppear/GableAppear/GableDisappear/RoofSwitchDirection）；循环参数 RoofHeight/RoofProfile/RoofTipMoveSpeed/RoofTipOffset/ImpactStrength/isHalfTimbered/editing_speed/is_gable；bound 常量 0.2/0.05/5.0（0x1429cd068/0x142930d70/0x142925988）；ridge 几何属 resources/roofs/roof_ridge.rs（跨域） | roof_audio.rs 增事件/参数常量登记 |
| decorator/roof 事件类型 | decorator=PlayDecoratorSoundCmd（16 事件已注册）；roof 事件流非 16 注册事件，属 roof 域（待接线）；decorator kind→loop 精确映射仍近似（循环名已验证） | 登记待接线 |

**编译/测试**：`cargo check -p country-core` 通过；全 lib 1724 过/1 败（geometry/sphere 他组）；音频域 32 测试全过。各定案项均以 fresh disasm（capstone 于 RNE 发行件）+ rne-read 串表为证。
