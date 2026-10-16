# 第四轮 ③ REVIEW（2026-10-16）

审查者：③ review agent。基准：第三轮 DELIVERY（A/B/C/D 组）"仍缺项" + 当前源码逐项核对
（`D:/game/tiny-glade/crates/country-core/`）。工具：rne-read 直读原字节 + decomp 行号对照。
T1（garden_audio）② 已交付，本文档包含其复审结论。

## 1. 缺口核对表

| 缺口 | 源码位置 | 现状(缺/水分/已实现) | 证据(原地址) | 深挖方向(给①) | 实现建议(给②) |
|---|---|---|---|---|---|
| T1 garden_audio 130.0 门+音量 | `systems/audio/garden_audio.rs` | 初始=缺（简化折叠）；**② 已重写并复审通过**（见 §4） | decomp-0x140a89480 L56-58/L164-241；decomp-0x14166d9f0；常量全部 rne 核实 | param_5 完整资源身份（见 F1a） | 已按 6 参契约落地；GardenAudioVolume 默认 1.0→0.0167 音量风险，宿主接线需覆盖 |
| T1 音量资源身份（`*param_5+0x1c`） | `systems/audio/garden_audio.rs`（GardenAudioVolume 适配） | **缺**（适配资源承�载，+0x1c 未定身份） | decomp-0x140a89480 L58 `*(float*)(*param_5+0x1c)/60.0`；同源 wall_audio decomp-0x140950920 `*param_8+0x1c`、terrain_stairs | 定位 param_5 资源（含光标面 +0/+4/+0xc 与 +0x1c 音量同体）；xref 0x140a89480 调用点与资源注册 | ① 定身份后把 GardenAudioVolume 换成真资源，删适配层 |
| T1 光栅资源身份（param_6） | `systems/audio/garden_audio.rs`（Res\<GardenRaster\>） | **已实现**（selector@+0x298→current_buffer 与 WorldRaster 布局完全吻合） | decomp L60-66（`+0x298` 字节选 +8/+0x10 vs +0x20/+0x28）；world_raster.rs L204-232 布局 | 复核 param_6 确为 `WorldRaster<512>` 资源（非别的栅格） | 已对齐；保留 |
| T2 run_game_thread 28-系统全局顺序 | `startup/run_game.rs` + `startup/mod.rs` | **缺**（`register_core_startup` 只挂 core 子集；`add_game_startup_systems` 0x140941110 只是转发） | 0x1405a3440（39KB）；0x140941110（28-tuple） | 出 28-系统顺序契约 + 各域注册入口（ivy/garden/sheep/input/audio 均为孤儿系统） | 按①契约把 A/B 组 add_*_systems 并入生产 Schedule |
| T2 load_game_session 完整 78KB 系统 | `systems/save/load_game.rs` | **缺**（纯函数 `load_game_session` 近似；`advance_load_game` 状态机已有） | 0x14166e7d0 | 出完整 SystemParam 面与状态机闭包 | 保持纯核心；① 契约到位后转 ECS 系统 |
| T2 会话切换系统 + NewSessionStartedCmd/SessionLoadedCmd 生产者 | `resources/session.rs`（事件注册）+ `meta_commander.rs`（commander case 0 写 NextSession） | **缺**（两事件 `init_resource` 后无任何 producer；`commander_update_system` 不发射） | session.rs L531-532；commander case 0 | 定位会话切换系统（public-session-names 第 9/11 条 SystemParam 面） | ① 契约到位后补生产者 + 注册 |
| T2 Schedule 库内执行桥 | `startup/*` | **缺**（crate 无 App/AppBuilder，`add_game_startup_systems` 只是函数转发） | 0x140941110 | ① 定 GameStartup 插件 build 全注册（28 系统 + 事件 + 资源 init 序） | 按契约重写 `add_game_startup_systems` |
| T2 init_game_state_from_snapshot 应用 | `systems/save/snapshot.rs` `apply_loaded_snapshot` | **缺**（空函数，诚实标注"不做虚假应用"） | 0x1417a9c50 | 域应用闭包（颜色/时间/羊/地形） | ① 契约到位后逐域接 |
| T2 PostGameLoadCommand::Apply 应用 | `systems/save/load_game.rs`（enum 带 `#[allow(dead_code)]`） | **缺**（只回放为 `ReplayedOnLoadCommand::Apply`，不应用；load_game_system 只发 LoadingGameComplete/StartupRevealStarted） | FUN_1412e0b90 | ① 定 Apply 的域载荷应用入口 | ① 契约到位后实现 Apply 应用 |
| T2 StartupRevealFinished 发射方 | `systems/save/load_game.rs` | **缺**（只注册+发射 Started；Finished 由宿主发，登记） | — | ① 定 Finished 发射时机（应用完加载后） | ① 契约到位后发射 |
| T3 ivy count_range.end | `systems/ivy/ivy_spawner.rs` `SPAWN_COUNT_RANGE=20..40` + `ivy_storage.rs` `spawn_ivy` count | **缺（机制定案，RNG 界值待证）**：native `count = 0x14 + usize_less_than(RNG_bound)`（capstone 0x1408696f0：`mov rdx,[rbp+0x190]`→call 0x141625a50→`add r14,r13`(r13=0x14)）；start=0x14 定案、`[rsp+0x30]` 确为记录数组基指针非计数（0x14094d010 r10=[rcx]）。crate 公式 `start + usize_less_than(end-start)` 与 native 形状一致，但 `end=40` 隐含 bound=20 未证实（真 bound=RNG_bound 待证） | ① t3-ivy-constants §4（capstone 直读 0x1408696f0） | RNG_bound 的 ivy_spawner 栈参映射（[rbp+0x190]↔caller [rsp+0x70+ 区]）低优先待证 | ② 保持 20..40 注释锁定；RNG_bound 定后改 `end = start + RNG_bound` |
| T3 ivy flag_8c leaf 清理谓词 | `systems/ivy/ivy_grower.rs` L406 `ivy.flag_8c=0` 直接清零 | **缺（已定案）**：native = grower 局部 `Vec<u64>` 受影响叶/根 id 列表（ptr@rbp+0x50/len@0x58，元素 stride 8，退出 drop）；谓词 = `flag_8c 置位 && 记录+0x60 字段 ∈ 列表 → 清零` | 0x14094ae63..96（grower.asm L305-315）；① t3-ivy-constants §3 | +0x60 字段语义（叶组 id/墙段 id）待取证 | ② 补 affected_ids 列表 + 查表谓词（先按洞命中 ivy id 建模，标注） |
| T3 ivy send_to_gpu seed/wall_normal | `systems/ivy/ivy_send_to_gpu_{segments,leaf}.rs` | **缺**（`seed=frame` 候选、`wall_normal`=切向垂线候选；arch_height 已接真墙 arch） | shader 布局对齐 | ① 定点 seed/wall_normal 真值 | ① 定值后替换；GPU 后置 |
| **T3 VERTICAL_MIN 常量** | `systems/ivy/ivy_spawner.rs` `VERTICAL_MIN=0.000928482` 注释 @0x142960690 | **水分（已定案=编造值）**：0.000928482 的 f32 字节 0x3a73655f **整 EXE 0 次出现**（③ 复核全文件检索）；0x142960690 实为 f32 **0.0**（前 4 字节 00000000，+4 起才是 "Resource does not…" 串）。正确下限 = **0.0** | rne-read 0x142960690=0.0；0x3a73655f 全 EXE 0 次；decomp-0x14094cf90 夹取门 `0.0<=f<=1.0` | ① 已定案（VERTICAL_MIN=0.0@0x142960690、1.5@0x1429258d0、-0.75@0x142ab3220、1.0@0x142925904、0.001@0x1429cd060、1e-20@0x142a7baa0） | ② 改 `VERTICAL_MIN=0.0`；更新 `vertical_u_clamps_to_the_machine_constant` 测试断言 [0.0,1.0] |
| T3 0.68 vs 0.63 | `systems/ivy/ivy_spawner.rs` `RANDOM_ROOT_OFFSET=0.68` | **已定案=0.68 正确**：0x142ab31f0=`7b142e3f`=0.68、0x142ab3210=`7b142ebf`=-0.68（capstone 引用）；0.1429258d4=0.63 是**另一常量**（非 ivy_spawner 引用集，全 EXE 11 次出现） | 0x142ab31f0/3210；① t3-ivy-constants §2 | SPEC 0.63 是取错地址，无需深挖 | 无需改（现 0.68 正确）；注释标注 SPEC 0.63 作废 |
| T3 ui_draw_terrain jumptable 分支 0/1/3 | `systems/terrain/ui_draw_terrain.rs` | **缺（已定案=4 分支全解）**：state0=无动作/1=点按/2=续笔（0x140924900 get_last_mut_stroke+0x140a97880 insert_or_morph_last_point）/3=释放读笔 | &UNK_142adc498；① t3-ui_draw_terrain 契约 | 已定案；新_stroke 生产面宿主 UI 待接 | ② 按契约补续笔/释放分支 |
| T3 EditType::Terrain tag | `resources/history/mod.rs` `native_tag` | **缺（已定案=现实现双错）**：原生 visit_str 0x140b3b630 全解——case7 "Terrain"→**0**、case4 "Tree"→**0xc(12)**（③ 独立复核字节）；现源码 Tree=0、Terrain=15 **均错**；其余 13 变体（Path/Cobblestone1-3/Water/Garden/Wall/Roof/Color/DecoratorV5/Decorator/AutoClutter=13/Stairs=14）与原生一致。deferred_terrain_history 0x140ac9770 **不经 EditType**（0x38B 记录 tag=1 直入 History，证实"可能不经 EditType"假设） | visit_str decomp（case4/case7 逐字节）；串表 0x142aee127/12b/14b/14f/15a；0x140ac9770 | 已定案，无深挖 | **需中央仲裁**后改共享 history：Tree 12 / Terrain 0 + 变体顺序对齐（影响存档/回放序列化，先与原侧 save 比对） |
| T3 ui_clutter parent-map 0x60B 槽位 | `systems/ui_clutter/ui_autoclutter.rs` | **缺（已定案=槽位 tag@0/+0x10/+0x30/0x38）**：0x60B parent 条目 tag@+0、+0x10、+0x30/+0x38（ClutterMeshTy::from 参） | decomp-0x14099ca60 L444-520；① t3-ui_clutter 取证 | 已定案；字段完整语义仍 U | ② 按契约对齐 AutoClutterParent 布局 |
| T3 capture/load 域采集器（wall/roof/decorator） | `systems/save/snapshot.rs` `capture_game_snapshot_save_data` 收调用方供 section_values | **缺**（三采集器未接） | FUN_141305170/141308b40/141309840 | ① 定三采集器 Query 面与序列化载荷 | ① 契约到位后接 |
| T3 sheep register_required_components ×5 | `resources/critters/sheep.rs` + `systems/debug/sheep_controls.rs` | **缺（已定案=5 宿主组件）**：SheepAudio/Sheep/UmbrellaHolder/SheepAccessory/FloatieHolder（procs.tsv 0x14089ec50/340/fa30/0x8a0120/0x8a0810）；每宿主精确 required 清单仍待取证 | ① t3-sheep 契约 §1 | 每宿主 required 清单（sheep_unstuck Query=&Transform+&mut CritterLocomotion+&mut Sheep → Sheep 至少 require Transform+CritterLocomotion） | ② 补 Sheep 的 #[require]（Transform+CritterLocomotion，标注 I）；其余 4 宿主组件待恢复 |
| T3 sheep picking/UI 分支 | `systems/debug/sheep_controls.rs` | **缺（待取证）**：25KB 反编译未定位到 move_state=0x101 写入（0x101={Walking,goal_change_timeout=1,…} 为判别候选，非立即数写面） | ① t3-sheep 契约 §3 | 保持待取证，不造 | ② 保持现实现，picking 分支登记 |
| T3 input update_mod_key_state 7-handler 跳表 | `systems/backend/input/mod_key.rs` + `mod.rs` | **缺**（只落两谓词；producer 未落） | 0x140af9c00 | ① 解码 7 组 ctrl/shift handler 跳表 + u32@+0 语义 | ① 契约到位后实现 producer |
| T3 GameProfile 直读 | `src/game_profile.rs` `GameKeyBindings.bindings: Vec<(u32,u32)>` | **缺**（占位 vs native 40×12B 数组；`BindingsTable::default` 40 槽逐字节已核） | 0x14089b110 | ① 定 GameKeyBindings 40×12B 布局 | ① 定布局后改 `GameKeyBindings` |

## 2. 功能缺失（消费者链断 / 无生产 / 无消费 / 身份未定）

- **F1a** garden_audio：音量 `+0x1c` 源资源身份未定（② 以适配资源承载）；光栅资源身份已对齐。功能面已完成（② 交付），资源身份阻塞 native 等价。
- **F1b** sheep save "Sheep" 节 producer 链**未闭环**：`sheep_section_serialized`（save_game.rs L396）与 `sheep_section_json`（control.rs L70）仅定义+测试调用，**写盘路径零调用**；`assemble_game_save_sections(…, sheep: SerializedJson, …)`（L408）也无 crate 内调用方（宿主接线）。`SheepControl` 资源（debug 域）未被 save 消费。→ 无真实 Sheep 节写盘。
- **F2** A/B 组系统**无生产注册**：`register_core_startup`（run_game.rs）只挂 session/save/history/commander/music/terrain/ui_clutter(history 链)；ivy（`add_ivy_systems` 7 系统）、garden、sheep、input 的 Schedule 均孤儿（仅测试跑）。原 22 ivy 系统 0x140944070 也未复现。→ 生产环境 ivy/garden/sheep/input 不运行。
- **F3** `garden_audio_system` 未进任何生产 Schedule：`add_audio_systems`（audio/mod.rs L210）只注册 music_debug_gui + water 两系统。
- **F4** `NewSessionStartedCmd`/`SessionLoadedCmd` 事件只注册（session.rs L531-532）无生产者；`commander_update_system` case 0 只写 NextSession。
- **F5** ui_clutter：`RmbAutoClutterHit` 生产者缺（输入域）；`SpawnAutoClutter` 消费方缺（宿主 deco-clutter 生成）；`RequestMoveClutter/RequestRotateDecorator/LastClonedClutter` 未接。
- **F6** Deletion→generation/mesh suppression 接口未建（无系统查询 `DeletedAutoClutter` 输出 suppression 集合）。
- **F7** `StartupRevealFinished` 无发射方；`StartupRevealStarted` 由 load_game_system 发射。
- **F8** 域采集器（wall/roof/decorator）+ X4 Query capture 未接 → 快照节载荷（除 Trees 收集器外）无真实生产者。
- **F9** `is_known_raster_key`（snapshot.rs L143）恒返回 false（占位，Raster 白名单待取证）；`SnapshotLoadResources` 只登记合同。

## 3. 深挖方向清单（给 ①，按优先级）

1. **T1 音量资源身份**：定位 garden_audio `param_5` 资源——同时携带 {tag@+0, x@+4, z@+0xc, +0x1c 音量}；xref 0x140a89480 调用点 + 同源 wall_audio `*param_8+0x1c`/terrain_stairs +0x1c 一并定案。
2. **T2 28-系统装配**：run_game_thread 0x1405a3440 + add_game_startup_systems 0x140941110 的 28-tuple 精确顺序 + 各域 add_*_systems 注册点；load_game_session 0x14166e7d0 完整 SystemParam 面；会话切换系统 + NewSessionStartedCmd/SessionLoadedCmd 生产者（public-session-names 第 9/11 条）。
3. ~~**T3 VERTICAL_MIN 重定位**~~ **已定案**：0x142960690=0.0（非 0.000928482，编造值 0x3a73655f 全 EXE 0 次）→ ② 改 VERTICAL_MIN=0.0。0.68/0.63 定论：0.68 正确（0x142ab31f0），0.63 为另一常量。
4. **T3 EditType tag12 4 字符名**：解 0x140b3b630 visit_str 表 tag12；确证 Terrain 是否原生 EditType（deferred_terrain_history 0x140ac9770 机制）。
5. **T3 ivy 定点**：count_range.end、flag_8c 清理 list 身份、send_to_gpu seed/wall_normal。
6. **T3 ui_draw_terrain jumptable 0/1/3** + ui_clutter 0x60B parent 槽位。
7. **T3 域采集器**：FUN_141305170/141308b40/141309840 Query 面；PostGameLoadCommand::Apply 应用；sheep register_required_components ×5 + picking 分支；update_mod_key_state 7-handler 跳表；GameKeyBindings 40×12B。

## 4. ② T1 garden_audio 复审结论（2026-10-16 初版 → 对齐后复审）

### 4a. 初版（2026-10-16，GardenAudioVolume 适配版）

**通过（需修正 1 项风险 + 2 项建议）**。逐项核对 decomp-0x140a89480 / decomp-0x14166d9f0 / rne-read：
- **常量**全部 rne 核实：GARDEN_WATER_GATE=0x3c23d70a（0x142960674 ✓）、VOLUME_NORMALIZER=0x42700000（0x1429d18cc ✓）、REPLAY_VOLUME=1.0（0x142925904 ✓）、GARDEN_RASTER_EXTENT=0x43020000（130.0 ✓，另 `_UNK_142a7b658` 也=130.0，extent 两参对称 ✓）。
- **loop 映射**（decomp L182/L202/L222/L241）：loop@+0=Draw(dry)、+0x38=DrawWater、+0x70=Erase(dry)、+0xa8=EraseWater ✓。
- **门逻辑**（L164-241）：`count!=0 && sample>=0.01 → water pair`、`sample<0.01 → dry pair`、count==0→0；含等号边界正确 ✓。
- **音量**（L56-58）：`mode!=0xd → [+0x1c]/60.0`、`==0xd → 1.0` ✓。
- **模板** FUN_14166d9f0：set_position + "editing_speed"(0xd=13 字符) + start_if_not_playing；stop 路径；双 start 注释诚实 ✓。
- 风险：GardenAudioVolume 默认 1.0→0.0167 近静音；采样无条件计算建议门控。

### 4b. 对齐后复审（2026-10-16，按①契约 + 仲裁批准）

**通过（无水分；1 项功能依赖登记）**。逐项复核：
- **签名**：procs.tsv 49396 权威 FunctionSystem = `void(*)(Res<AppMode>, ResMut<GardenAudioState>, EventReader<GardenDrawEvent>, EventReader<GardenEraseEvent>, Res<CursorTerrainRaycast>, Res<world_raster_defs::WaterRaster>)`——本实现 6 参逐一吻合，**无 AudioCore 参** ✓。
- **param_6=WaterRaster 纠偏**（非 GardenRaster）：PDB 签名证实；语义自洽（光标处水栅格值≥0.01 → water 变体）；`+0x298` selector/缓冲对 与 WorldRaster 布局吻合 ✓。
- **param_5=CursorTerrainRaycast 32B**：`{tag:u32@0, pos:Vec3@4, delta:Vec3@0x10, speed:f32@0x1c}` 由写入者 decomp-0x1408ef320 逐槽核实（+0x10/+0x14/+0x18 delta 写、+0x1c speed=`SQRT(Δ·Δ)/[*param_7+0x58]`）✓；tag u8→u32 宽化兼容 `tag&1`/`tag==0`；8 处 render 测试构造补 `..Default::default()` ✓。
- **音量**：`cursor.speed(+0x1c)/60.0`（Replay→1.0）——适配资源已删，+0x1c 一等字段 ✓。
- **PathSize 二次参数**：常量 rne 核实（0x142ac56c0=`0bd7a33e`=0.32、0x1429cd128=`9a99993f`=1.2、串 0x142c84553="PathSize" 8 字符、0x142c84546="editing_speed" 13 字符）；`garden_path_size=2*inverse_lerp(0.32,1.2,radius)` 与 decomp L117-128（inverse_lerp 0x140c904a0 无 clamp + 2× `extraout_XMM0_Da+extraout_XMM0_Da`）一致 ✓。
- **采样门控**：双通道无事件不采样（采纳初版建议），与 native per-loop count==0 门控输出等价 ✓。
- **事件布局**：GardenDraw/EraseEvent `{position@0, radius@0xc, observed_flag@0x10}` stride 0x20 与契约 §6 一致 ✓。

**需登记的功能依赖（非水分）**：`CursorTerrainRaycast.speed/delta` 无 crate 内生产者（写入者 0x1408ef320 未实现为系统；`update()` 只写 tag/pos）→ 非 Replay 下 standalone 音量 = 0.0/60.0 = 0（静音）直至宿主挂 mouse_terrain_raycast。这是 T2 宿主接线的真实依赖，DELIVERY 已诚实登记写入者与 `[param_7+0x58]` 除数身份待取证。

**待取证登记合理**：param_7+0x58 除数资源身份、GardenAudioState @0x10/@0x1c 计数写回、非内联数组布局、28-系统注册、双 start 幂等单次。

## 5. ② T2 load_game_session + register_core_startup 复审结论（2026-10-16）

**通过（无水分；2 项偏差注释 + 仍缺登记诚实）**。对照 ① 两份契约（t2-load_game_session-session-switch / t2-run_game_thread-register_core_startup）+ PDB public-session-names + decomp-0x14166e7d0 + 源码：

- **13-param 面**：PDB `public-session-names.txt` L9/L11 权威 `FunctionSystemState` 元组与本实现签名**逐参逐序完全一致**：`(Res<SavedSessionList>, Option<ResMut<ActiveSession>>, Res<GameProfile>, Res<AutoSaveState>, ResMut<NextSession>, ResMut<LoadGameManager>, ResMut<AppMode>, ResMut<GladeSettings>, ResMut<MetaCommander>, EventWriter<NewSessionStartedCmd>, EventWriter<SessionLoadedCmd>, EventWriter<DisplayCorruptedSaveFileWarning>, Commands)`。decomp-0x14166e7d0 L296 `func_0x000142923c00(&buf, param_5, 0xa0)` = NextSession 160B 拷贝 → param_5=ResMut<NextSession> ✓。
- **行为**：continue_latest → 新建（'Starting a new game session'→建目录/meta/起始 build full）／继续（'Continuing game session {id}'→loading.tmp/history.json 就绪检查→corrupt_warn）→ 载入 meta→ActiveSession（失败 log+default 回退）＋settings→GladeSettings → NewSessionStartedCmd+SessionLoadedCmd 发射。契约 §2 行为串全部对应 ✓。
- **装配顺序**：run_game.rs `register_core_startup` = 会话流 → MetaCommander → GameConfig（新增 init_resource，native 0x140522ad0）→ 事件注册（add_save_load_events_and_resources + register_input_events 新增 + register_history_events）→ 域系统（music/history/load_game_session 新增/add_save_load_systems_impl/terrain）→ 后台线程（init_save_thread 在 add_save_load_systems_impl 内）。与契约 §1/§5"本地可落地部分"一致；Asset 库族/VideoSettings/OS 窗口事件留给宿主/D 组（仍缺 #4 诚实）✓。
- **GameProfile Resource derive**：game_profile.rs L37 已补 `#[derive(Resource,…)]`（native Res<GameProfile> 第 3 参 O）✓。
- **冗余清理**：重复 `init_resource::<LoadGameManager>` 已删（add_save_load_events_and_resources save/mod.rs:38 已 init）✓。
- **事件面**：`DisplayCorruptedSaveFileWarning` 为 Event（load_completion_checker.rs L82）；`GladeSettings::load`/`SessionMeta::write_to_disk`/`copy_dir_all`（sorted，native 0x141740c00 排序语义）存在 ✓。

**2 项偏差（注释级，非水分）**：
1. **native 建目录/meta 写失败是 panic**（'Creating folder for game session'/'Writing new session meta.json' panic 串），② 改为 log+corrupt_warn+return 软失败——库内合理适配，建议 doc 标注偏离。
2. **'Latest game session ' 日志**（契约 §2.3 双串）未在 load_game_session 显式记录（continue_latest 侧路径），行为等价，建议注释。

**仍缺登记诚实**：未消费 6 SystemParam（GameProfile/AutoSaveState/LoadGameManager/AppMode/MetaCommander/Commands）underscore 登记；会话切换 6 消费系统（reset_input_state/time_of_day/input_record_replay/add_preplaced_autoclutter/detect_overhang_support_segments/sheep_unstuck）事件接线待各域接入；28-系统逐名/0x14093fa60 default×60/两后台线程角色待取证——均不伪造 ✓。

**功能缺失更新**：**F4 已闭合**（NewSessionStartedCmd/SessionLoadedCmd 现由 load_game_session 生产，消费端仍待接）；F2 部分缓解（register_input_events 已入装配，input 系统调度仍待 T2 契约 §3 消费接线）。

## 6. ① T3 取证反馈 + ③ 独立复核（2026-10-16）

① 已全量取证（契约 10 份落盘 round4/contracts/）。③ 对关键项独立复核（rne 全文件检索 + visit_str decomp 逐字节）：

- **VERTICAL_MIN 编造确认**：0.000928482 的 f32 字节（0x3a73655f）**整 EXE 0 次出现**（③ rne 复核）→ 实为编造值；正确 = 0.0@0x142960690。② 待办：改 VERTICAL_MIN=0.0 + 更新测试。**水分定案**。
- **EditType tag 双错确认**：visit_str 0x140b3b630 case7 "Terrain"→0、case4 "Tree"→0xc(12)（③ 逐字节复核）；现源码 Tree=0/Terrain=15 均错。其余 13 变体与原生一致。deferred_terrain_history 不经 EditType（0x38B tag=1 直入 History）。**需仲裁**改共享 history（影响存档/回放序列化）。
- **0.68 定案**：0x142ab31f0=0.68（0x3f2e147b，EXE 3 次）、0x142ab3210=-0.68；0.1429258d4=0.63 为另一常量（EXE 11 次）。现代码 0.68 正确。
- **flag_8c 谓词定案**：grower 局部 Vec<u64> affected 列表（rbp+0x50/0x58），谓词=flag_8c&&记录+0x60∈列表→清。② 补列表+查表谓词。
- **ui_draw_terrain jumptable 4 分支定案**（state0 无动作/1 点按/2 续笔 0x140924900+0x140a97880/3 释放读笔）；ui_clutter 0x60B 槽位 tag@0/+0x10/+0x30/0x38 定案。
- **sheep 5 宿主组件定案**（SheepAudio/Sheep/UmbrellaHolder/SheepAccessory/FloatieHolder）；required 清单待取证；picking 分支待取证（25KB 反编译未现 0x101 写面）。
- **三采集器定案**（Tree=141305170/Wall=141308b40/u64-set=141309840）；PostGameLoadCommand::Apply=Commands::insert_resource<NextSession>（0xa8B）；update_mod_key_state 双跳表（表1@0x142ae5f50）。
- **count_range.end 机制定案**（① 补证，③ 复核 crate 公式）：native `count = 0x14 + usize_less_than(RNG_bound)`（capstone 0x1408696f0）；crate `spawn_ivy` 的 `start + usize_less_than(end-start)` 形状一致，start=0x14 ✓；`end=40` 隐含 bound=20 未证实（真 bound=RNG_bound 待证，[rbp+0x190]↔caller [rsp+0x70+] 低优先）。
- **待取证汇总**：count_range.end 的 RNG_bound、send_to_gpu seed/wall_normal（GPU 后置）、sheep required 清单与 picking 分支、28-系统逐名（解叶子 builder）、update_mod_key_state 键码枚举、flag_8c +0x60 字段语义、ui_clutter 0x60B 完整字段。

**汇总**：缺项 22（水分定案 1=VERTICAL_MIN 编造、EditType 双错待仲裁）、功能缺失 F1-F9（F4 已闭合）、给① 深挖方向 7 组（T3 大多已定案）、已复审 ② T1/T2 通过（均无水分）。

## 7. ② T3 批次（10 项）复审结论（2026-10-16）

**通过（无水分；1 项文档 stale 需修 + 1 项功能依赖登记）**。逐项对照 ① 契约 + 源码 + 机器码：

| # | 项 | 复核 |
|---|---|---|
| 1 | VERTICAL_MIN→0.0 | ✓ `ivy_spawner.rs` L97 `VERTICAL_MIN=0.0`，编造值文档化（0x3a73655f 全 EXE 0 次）；公式/常量注释对齐 |
| 2 | 0.68 定案注释 | ✓ 0x142ab31f0=0.68/-0.68@0x142ab3210 注释；0.63 标注另一常量 |
| 3 | flag_8c 清理谓词 | ✓ `GrowerState.affected_ids`（L391）+ `mem::take` 上轮列表（L409）+ 谓词 `flag_8c!=0 && prev.contains(id)`→清（L413-414）+ 洞命中 push（L532）；+0x60 以 ivy id 建模标注 |
| 4 | EditType 双错修正 | ✓ **中央批准**（PROGRESS.md 仲裁 L96）；Terrain 首变体 tag0 / Tree tag12 / 其余 13 不变，与 visit_str 0x140b3b630 逐字节复核一致；无测试断言旧 tag |
| 5 | ui_draw_terrain state 2 续笔 | ✓ `get_last_mut_stroke`+`insert_or_morph_last_point`（0x140924900/0x140a97880），MORPH_SPACING=1.0 标 I；state 3 无 release 事件登记待取证 |
| 6 | update_mod_key_state producer | ✓ 表1 分派（组0 ON/1,3,4 OFF/2 setae/5,6,7 待取证 OFF）+ setb 门（state<3）；ModKeyConfig 注册（input/mod.rs L49）+ 挂调度（L65）。**M1 stale**：input/mod.rs L11-12 模块 doc 仍写"producer 待取证未落"，与代码矛盾，需更新 |
| 7 | sheep required 集冲突 | ✓ **定案=空集**（③ 用 procs.tsv 函数尺寸独立证实）：5 个 sheep `register_required_components`（0x14089ec50/340/fa30/8a0120/8a0810）全为 **1776B=0x6f0**，与空模板 `FollowMouse`（0x140844f20=1776B）**同尺寸** → sheep required=空。② 不加 #[require] 正确；① 推断 Transform+CritterLocomotion 是 Query 需求≠#[require]，驳回。① 契约"18KB"指 decomp 文本非函数尺寸 |
| 8 | ui_clutter 0x60B 槽位 | ✓ `AutoClutterParent` tag@0/slot_count@0x10/mesh_ty_arg@0x30/mesh_ty_arg2@0x38；+0x04..0x0f 等区间标 U |
| 9 | save 域采集器 | ✓ `collect_walls`（0x141308b40 inner_walls→Vec）+ `collect_ids`（0x141309840 HashSet→Vec）；**中央批准**（PROGRESS.md L97） |
| 10 | Apply 桥 | ✓ `load_game_system` 加 Commands + `ReplayedOnLoadCommand::Apply`→`insert_resource<NextSession>`（FUN_1412e0b90）；**中央批准**（L97）；0xa0B 载荷重构待取证以 clone 桥 |

**M1 需修**：`input/mod.rs` L11-12 模块 doc 过时（update_mod_key_state 已落地但 doc 写"未落"）——注释同步。
**M2 功能依赖登记（非水分）**：ui_draw_terrain state 3（释放读笔）因 `TerrainHoverInput` 无 release 事件不可达——宿主 UI 输入面须提供 release（T2 契约 §3 接线），已登记。
**仍缺登记诚实**：send_to_gpu seed/wall_normal、count_range.end RNG_bound、mod_key 组 5/6/7 键码、sheep 其余 4 宿主组件本体、ui_clutter 0x60B 剩余区间、Apply 载荷字段——均不伪造。
