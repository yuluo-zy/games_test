# country-core 优先模块推进台账（2026-10-16 第三轮）

协调简报：[COORDINATION.md](./COORDINATION.md)。四组并行：A 植被 / B 生物·音频·输入 / C 世界 / D rhapsody 接入。
中央 agent 只做台账维护、跨组文件仲裁、进度提醒；不改 country-core 源码、不跑 cargo。

## 第三轮实际状态（2026-10-16 用户批改后，以此为准）

- 用户已提交大批判改：tiny-glade commit `f510c7e7`「库内完整性核验收口与缺口闭合」+ merge `c164567a`，**9 个优先模块的大部分 SPEC 项已实现**（ivy grower 主生长闭包/send_to_gpu 48B shader 布局/pruner；input translate_input 动作↔槽映射/InputReplayInitialState；save_thread/load_game/save_game/snapshot/load_completion_checker；terrain ui_draw_terrain 等；ui_clutter rmb_autoclutter 等；session.rs；audio 参数/事件名定案）。
- `contracts.json` 的 `remaining` 是批改前占位，**已过期**；下表"合同剩余缺口"列保留仅为追溯，验收必须对当前源码实际 diff。
- 历史分析 agent 本轮职责为**核对**：对照 SPEC/原机器码逐项验证用户批改的真实实现，产出 `*-DELTA.md`（已真实落地 / 有水分 / 仍缺）。合成 agent 据此关闭真实剩余缺口。
- 工作树干净，`cargo check -p country-core` 通过（9 warning）。
- **合成实现阶段已派发（2026-10-16）**：A 组（aa5b60b8c8fc6e7a7，ivy+garden）、B 组（a907e474d80314271，sheep+music+input）、C 组（a9c6f52b7cc06d63f，save→session→terrain→ui_clutter）、D 组（a0266f71e6011aad8，rhapsody 接线）。各组按 DELTA 关闭真实剩余缺口，交付 `*-DELIVERY.md`。已预告仲裁请求方向：garden_audio_system 130.0 门、sheep save "Sheep" 节、sheep query owner、input UiHistoryButtonPressed 生产者、C 组 history/lib.rs/startup 注册、A 组 ivy send_to_gpu 消费者（D 组 render 域）。

## 第四轮（2026-10-16）逆向关闭剩余缺口——三人团队

用户指示身份转变：**① Rust 专职逆向（a5844e030a22117a1，只读） / ② 实现（ab30764d525b73aea，写源码） / ③ review（a92b9b4990d23a6b2，只写报告）**。目标：逆向关闭第三轮登记的全部剩余缺口。证据目录 `round4/{contracts,deliveries,REVIEW.md}`。

**文件权**：仅 ② 写 country-core 源码；**绝不可碰 `resources/roofs/geometry/*`、`systems/roof/*`（用户正在做 roof 重构，任何 agent 不可动）**；①③ 只写证据/报告。② 对组外共享文件（`resources/history/*` 等）的修改申请照旧仲裁。整 crate 编译门仍被用户 roof 重构阻塞；② 只保证自己文件无错，不跑依赖 roof 的整包 check。

| track | 契约 | 状态 | 证据 |
|---|---|---|---|
| T1 garden_audio 130.0 门 | 已解码契约（bilinear 光栅采样门 ≥0.01 分流、音量 [vol_res+0x1c]/60.0 当 mode!=0xd、130.0=0x43020000、native 6 参签名）→ ② 已实施，父复核通过：四常量全对原字节（0x3c23d70a=0.01/0x42700000=60.0/0x43020000=130.0）、门逻辑、音量、模板、bilinear_sample_at_world_xz=float_grid.rs:26 既有实现；③ 复审通过（1 风险）；**契约对齐复核通过（父 2026-10-16）**：6 参签名（Res<AppMode>/Res<CursorTerrainRaycast>/Res<WaterRaster>，纠偏 GardenRaster）、音量=cursor.speed/60.0（Replay→1.0）、模板补 PathSize=2*inverse_lerp(0.32@0x142ac56c0, 1.2@0x1429cd128, radius)、GardenAudioVolume 适配已删；cursor_terrain_raycast.rs 已扩 32B {tag:u32,pos,delta,speed} | **契约对齐复核通过（无水分）**；低优先仍缺：param_7+0x58 除数资源身份（写作端 mouse_terrain_raycast@0x1408ef320）、GardenAudioState @0x10/@0x1c 计数语义；风险：宿主未接线时 0.0167 近静音须覆盖 | round4/deliveries/garden_audio-gate-DELIVERY.md |
| T2 ECS 调度器 + startup 装配 | run_game_thread 0x1405a3440（39KB 装配+28 系统全局顺序）、load_game_session 0x14166e7d0（78KB 完整系统）、会话切换系统 + NewSessionStartedCmd/SessionLoadedCmd 生产者、Schedule 库内执行桥、init_game_state_from_snapshot 应用、PostGameLoadCommand::Apply（FUN_1412e0b90）、StartupRevealFinished 发射方。① 契约已落盘；② 已交付；**父复核通过（2026-10-16）**：load_game_session 13-param（三事件生产者+Local<bool> 启动门）行为串对应原机码、case1 就绪检查/case2 建目录+meta 写盘+copy_dir_all、register_core_startup 装配序+GameConfig init+register_input_events+load_game_session 注册、GameProfile Resource derive。仍缺（诚实登记待①取证/各域接入）：6 SystemParam 面内精确消费、SavedSessionList@+0x10 attempt 字段、28-系统逐名、0x14093fa60 default×60、两后台线程角色、会话切换消费者 6 系统、demo 旗标、Asset 库族/VideoSettings/OS 事件注册（宿主/D 组面） | **复核通过（无水分）** | round4/deliveries/t2-scheduler-startup-load_game_session-DELIVERY.md |
| T3 待取证小项定点 | **① 批次2 已定案（7 契约）**：VERTICAL_MIN=0.0 值确认编造（0.000928482 全文件 0 次出现），0.68 根偏移定案 0x142ab31f0、0.63@0x1429258d4 为另一常量；EditType 权威表（visit_str 0x140b3b630）Terrain=0/Tree=12（现源码 Tree=0/Terrain=15 双错需修，deferred_terrain_history 不经 EditType=0x38B tag1 直入 History）；ui_draw_terrain jumptable 4 分支（0 无动作/1 点按 new_stroke/2 续笔 get_last_mut_stroke+insert_or_morph_last_point/3 释放读笔）；域采集器 Tree=141305170 Query<(&PrefabInstanceData,&PrefabInstance,&TreeComponent)>/Wall=141308b40 inner_walls HashMap/u64-set=141309840，PostGameLoadCommand::Apply=Commands::insert_resource<NextSession>；sheep required 5 宿主组件（SheepAudio/Sheep/UmbrellaHolder/SheepAccessory/FloatieHolder）；update_mod_key_state 双跳表（表1 状态→7 handler、表2 动作码）；ui_clutter parent-map 0x60B 槽位 | **① 逆向收官；T3 复核通过（无水分，父 2026-10-16）**：② 已落地 VERTICAL_MIN=0.0/0.68 定案/flag_8c 谓词/EditType 双错修正（仲裁14）/ui_draw_terrain state2/update_mod_key_state producer/sheep 组件登记（required 集冲突按 registry=空集不伪造）/0x60B 槽位/save 域采集器+Apply 桥（仲裁15）。**里程碑：整 crate cargo check 零错误（roof 重构已收敛），父亲自验证**。待取证清单（契约内标注不编造）：28-系统逐名、count_range.end、send_to_gpu seed/wall_normal、sheep required+picking、update_mod_key_state 键码枚举、param_7+0x58 除数、GardenAudioState 计数写回、0x14093fa60 default×60、两后台线程角色、GameProfile 直读、ui_draw_terrain state3、0x60B 区间 U、save 节布局/Apply 载荷字段 | round4/contracts/t3-{ivy-constants,edittype-terrain-tag,ui_draw_terrain-jumptable,save-domain-collectors,sheep-register_required_components,input-update_mod_key_state,ui_clutter-parent-map-0x60B}-CONTRACT.md；round4/deliveries/t3-easy-evidence-DELIVERY.md |

### ③ REVIEW（round4/REVIEW.md，2026-10-16 已核）

- **水分确认 1 条**：VERTICAL_MIN=0.000928482 注释 @0x142960690 地址造作——rne-read 0x142960690=ASCII "Resource does not exist…"（52 65 73 6f…），非常量；值或真但地址错，已列 ① 重定位任务。
- **功能缺失 F1-F9**：F1a 音量资源身份（已由①定 param_5=CursorTerrainRaycast，②对齐中）；F1b **sheep "Sheep" 节写盘路径零调用**（sheep_section_serialized/assemble_game_save_sections 无 crate 内调用方，SheepControl 未被 save 消费）；F2 **ivy/garden/sheep/input 系统 Schedule 孤儿**（register_core_startup 未挂 add_ivy_systems 等，原 22 ivy 系统 0x140944070 未复现）；F3 garden_audio_system 未进生产 Schedule（add_audio_systems 只注册 music_debug_gui+water）；F4 会话事件无生产者（NewSessionStartedCmd/SessionLoadedCmd 只注册）；F5 ui_clutter RmbAutoClutterHit 生产者缺/SpawnAutoClutter 消费方缺；F6 suppression 接口未建；F7 StartupRevealFinished 无发射方；F8 域采集器+X4 Query 未接；F9 is_known_raster_key 恒 false。
- **T1 复审**：通过（1 风险：GardenAudioVolume 默认 1.0→0.0167 近静音，宿主须覆盖）+ 2 建议（恒采样注释、待取证三项登记）。
- **深挖方向已发 ①**（7 组）：音量资源身份、28-系统装配、VERTICAL_MIN 重定位、EditType tag12、ivy 定点、jumptable/0x60B、域采集器/Apply/sheep/update_mod_key_state/GameKeyBindings。

## 第五轮（2026-10-16）关闭三个集中缺口——续跑第四轮三人团队

续跑同一团队（① RE a5844e030a22117a1 / ② 实现 ab30764d525b73aea / ③ review a92b9b4990d23a6b2），保连续上下文。④轮里程碑：整 crate `cargo check` 零错误，T1/T2/T3 全部复核通过无水分。

| 块 | 范围 | 状态 | 证据 |
|---|---|---|---|
| A 块 ECS 调度器整合 | F2 + 库内执行桥 + 28 序：孤儿系统并入生产 Schedule；Schedule 库内执行桥；28-系统精确序。**父复核（修复后）：完全闭合**——F3 闭合（garden_audio_init/garden_audio_system 已入 add_audio_systems，systems/audio/mod.rs:241）；G6 对齐（add_audio_systems 现注册 timberframe/elevation_supports/clutter/forest/tree/firstperson_steps + 其余音频域 ~40 系统分批）；整 crate 编译零错误。wall/signifier 登记待接入。待取证：28-tuple 槽位映射、garden 系统名、Schedule.run 调用点、wall/signifier 接入 | **复核通过（完全闭合）** | round5/contracts/a-ecs-scheduler-integration-CONTRACT.md、audio-G6-alignment-CONTRACT.md；round5/deliveries/abc-blocks-DELIVERY.md |
| B 块 sheep save 写盘链 | F1b：sheep_section_serialized/assemble_game_save_sections 零调用方 → 接真实写盘。父复核通过：Sheep 节写盘链有真实注册生产调用点（run_game.rs:144 sheep_save_section_system + SheepSaveSectionValue 资源，宿主 assemble 取 SaveFileSection::Sheep=5，@0x140a51d40）。**父复核（修复后）：闭合**。待取证：X4 活羊 Query 面（不编造） | **复核通过（完全闭合）** | round5/contracts/b-sheep-write-path-CONTRACT.md；round5/deliveries/abc-blocks-DELIVERY.md |
| C 块 快照应用 + 会话消费端 | init_game_state_from_snapshot 应用（apply_loaded_snapshot 逐域应用，0x1417a9c50）；会话切换消费者 6 系统接线。① 已落盘逐域映射 + apply API + 节→记录解析 + 数据源定案（JSON serde，{count@0,ptr@8}=内存 Vec cap-first 头非节载荷，Terrain=deflate 经 JSON 承载）。**父复核（数据源纠偏落地）：apply 已升级为真实调用链（诚实渐进终点）**——Walls=parse_wall_section(JSON)→serde Vec<[u64;15]>→InnerWallStateRecord→InnerWalls::load_state（候选 schema 匹配即真实写回，不匹配 log-trace "JSON shape 待①"）；Decorators=parse_decorator_section 诚实返回 None（JSON↔0x28B serde 架构待①，不构造假快照）；Terrain=parse_terrain_section JSON→inflate（strokes 布局待证）。整 crate 零错误。**本轮可闭合项到此为止**：剩余 Walls 14 qword/Decorators serde/Terrain strokes 为①中优先字段级取证（非本轮可闭） | **数据源纠偏落地，apply 真实调用链（Walls 真路径、Decorators/Terrain 写回待① serde 架构）** | round5/contracts/c-snapshot-apply-session-consumers-CONTRACT.md、c-snapshot-apply-targets-CONTRACT.md、c-apply-api-formats-CONTRACT.md、c-apply-section-parsing-CONTRACT.md、c-apply-data-source-CONTRACT.md；round5/deliveries/abc-blocks-DELIVERY.md |

**验收门**：A 块后孤儿系统清零、Schedule 可执行；B 块后 Sheep 节真实写盘；C 块后 apply_loaded_snapshot 逐域应用。每块交付照旧 ③ 复审 + 父复核。**⑤轮里程碑（最终）**：A 块完全闭合（F3/G6 音频注册闭环，整 crate 编译零错误）、B 块闭合（Sheep 节真实写盘链）、C 块结构化+apply 真实调用链闭合（Walls 真路径写回；Decorators/Terrain 写回挂①字段级 serde 架构取证，非本轮可闭）。

**文件权（② 写）**：startup/run_game.rs、startup/mod.rs、src/lib.rs（执行桥/注册）、systems/{ivy,garden,backend/input,debug/sheep_controls,audio/mod.rs} 注册面、systems/save/{save_game,snapshot}.rs、resources/critters/sheep/*、systems/{terrain,time_of_day,input_record_replay} 资源面、meta_commander.rs。**绝不可碰 `resources/roofs/geometry/*`、`systems/roof/*`（roof 重构区仍禁区）**。共享文件（startup/lib.rs/save 域等）改动照旧中央仲裁。

## 第六轮（2026-10-16）按当前架构推进到完成——字段级取证→实现闭合

续跑三人团队（① RE a5844e030a22117a1 / ② 实现 ab30764d525b73aea / ③ review a92b9b4990d23a6b2）+ 中央台账。基线：整 crate 编译零错误。wall/signifier 属用户重构域（收敛后接，第六轮先登记契约不越权）。② 组外申请照旧仲裁。

| 组 | 项 | 内容 | 状态 | 证据 |
|---|---|---|---|---|
| F1 字段级取证→实现闭合 | Walls 0x78B 其余 14 qword | 0x78B 记录={WallId u64@0 + WallState 0x70B@8}，WallState +0x54 height f32/+0x60 freehand u8/+0x64 arch u8/+0x67 removed_supports u8。**父复核通过（2026-10-16）**：WallState 解码真（height@+0x54→0x14085ad60、freehand@+0x60→0x140aa99e0、arch@+0x64→0x140aa9eb0、removed_supports@+0x67）；曲线区 +0x00..0x53 待①诚实标注 | **复核通过（无水分）** | round6/contracts/r6-walls-0x78b-CONTRACT.md |
| F1 | Decorators serde 架构 | 0x28B 记录 {u32 id, 8B×4, u32×2}；DecoratorId 旋转 XOR 16B 掩码 0x1429cd080 编解码；子侧状态公式 (b-2)<=2?b-1:0；take 空桶前置（restore_snapshot 可直接落地）。**父复核通过（2026-10-16）**：snapshot.rs DecoratorSectionRecord([u8;0x28]) + encoded_id/encoded_u64（rotate_right(32)^0，16B XOR 掩码 0x1429cd080 待①=0 诚实标注）+ side_from_state(b) 公式。仍待：JSON 逐字段名 | **复核通过（无水分）** | round6/contracts/r6-decorators-serde-CONTRACT.md |
| F1 | Terrain strokes 序列化格式 | strokes 布局 | 待①契约 | round6/contracts/*（待） |
| F1 | GameSaveData 240B / AsyncSaveCmd 各变体 / session create 96B+32B+24B | AsyncSaveCmd 判别已解。**父复核通过（2026-10-16）**：save_game.rs:139 枚举 #[repr(u64)] 判别 3 OpenFile/4 CloseFile/5 ChangeSection/6 ReplaceEdits/7 BackupSave/8 ManualBackup/9 CaptureSnapshot/10 Flush/11 Shutdown 全对齐①契约，各变体载荷标"语义字段+字节布局待取证"不编造。仍待：GameSaveData/session create 字段名 | **复核通过（无水分）** | round6/contracts/r6-async-savecmd-gamesavedata-CONTRACT.md |
| F1 | sheep 布局 + spawn/movement + X4 羊 Query | stride 0x68 已证偏移（+0x30/+0x31/+0x38/+0x58）；being_petted 阈值**纠偏：0.01@0x142960674（0x3c23d70a），非 0x292f**（round-3 复核的 0x292f@0x142930f74 为地址算术错，③ 独立解码 ucomiss rip 目标 + 父 rne 复核）。**父复核通过（2026-10-16）**：布局确认；**② 修 sheep.rs + GameKeyBindings #[repr(packed)] 中**。spawn/movement 状态机、X4 Query 仍待① | **复核通过（无水分）；sheep 常量纠偏 + KeyBinding repr 修正中** | round6/contracts/r6-sheep-layout-CONTRACT.md |
| F1 | ui_clutter AutoClutterRecord 0x58B / parent 0x60B | 完整布局 | 待①契约 | round6/contracts/*（待） |
| F1 | ivy count_range RNG 界 / send_to_gpu seed·wall_normal / mod_key 键码枚举 / GameKeyBindings 40×12B | GameKeyBindings 40 槽×12B {key@0,value@4} default 全解（0x14089b110，0xc3 未绑定哨兵），GameProfile 固定数组直读。**父复核通过（2026-10-16）**：40 槽固定数组直读真（槽 36 [0x1b0]=0xc3 哨兵、serde 经 Vec 中转 N>32 限制诚实标注、ron 5 字段序列化/6 名反序列化）。count_range/send_to_gpu/mod_key 键码仍待① | **复核通过（无水分）** | round6/contracts/r6-gamekeybindings-modkey-CONTRACT.md |
| F1 | Roofs/WallColors/GateColors/Trees/Raster 5 域写入序 + on_event 应用对象 | 快照节写回序 | 待①契约 | round6/contracts/*（待） |
| F2 调度/装配（可登记部分） | 28-系统逐名、G1/G3/G4 asset 组、Schedule.run 宿主驱动模型 | 产出契约供宿主/D 组；wall/signifier 接入待用户重构收敛 | 待①契约 | round6/contracts/*（待） |
| F3 跨域接线登记 | RmbAutoClutterHit/UiHistoryButtonPressed 生产者、SpawnAutoClutter 消费方、宿主资源注入 | 登记契约，宿主面 | 待①契约 | round6/contracts/*（待） |

**验收门**：每个字段级项 ①契约→②实现→③复审→父复核；整 crate 保持零错误；C 块 apply 达"逐字段真应用"；save/session 字段名定案；sheep 布局/spawn·movement 闭合。wall/signifier 收敛后接入。

**第六轮批次1（2026-10-16）**：4 契约落盘（r6-walls-0x78b / r6-gamekeybindings-modkey / r6-sheep-layout / r6-field-status），② 实现中。r6-field-status 汇总仍待证：Decorators serde、Terrain strokes、save/create 字段名、count_range/send_to_gpu、0x58B、5 域序、28-系统逐名。

**第六轮批次 2 重定向（2026-10-16，避开用户三重构区）**：rhapsody / roof / wall（含 wall/snapping/signifier）为**用户重构区**，任何 agent 不可改（Walls 曲线区跳过属 wall 区由用户收）。批次 2 重定向到完全独立 country-core 项：Decorators serde（0x28B/0x16B JSON↔记录架构，快照 apply 关键）、save/session 字段名（GameSaveData 240B / AsyncSaveCmd 变体 / create 96B+32B+24B）、sheep spawn·movement 状态机 + X4 羊 Query（0x140a3f470）、Terrain strokes 格式 / ui_clutter 0x58B / ivy count_range / mod_key 表 2 键码、camera_rig / weather / time_of_day 深层补证、28-系统逐名（契约供宿主，不碰 rhapsody 依赖的 G1/G3/G4 asset 组实体）。流程：① 取证独立项 → ② 实现闭合 → ③ 复审。注：rhapsody vulkan.rs:470 编译门挂 rhapsody 重构区，country-core 自身零错误。② 组外申请照旧仲裁。

**批次 2 契约落盘（累计 7 份）**：r6-decorators-serde（0x28B 记录 {u32 id,8B×4,u32×2}+DecoratorId 旋转 XOR 掩码 0x1429cd080 编解码+子侧状态公式 (b-2)<=2?b-1:0+take 空桶前置，restore_snapshot 可落地）、r6-async-savecmd-gamesavedata（AsyncSaveCmd 判别 3 OpenFile/4 CloseFile/9 CaptureSnapshot/10 Flush/0xb Shutdown）、r6-batch2-status（汇总待证：Decorators JSON 逐字段名、GameSaveData/create 字段名、sheep movement+X4、Terrain strokes、0x58B、count_range、表2、camera_rig/weather/time_of_day、28-系统槽位+Schedule::run 调用点）。**② 实现中，三区已避开**。

**批次 3（收官，2026-10-16）**：① 取证剩余全部独立项，② 实现闭合，**父直接复核（③ 已停）**。避开三区。**父复核：批次 3 通过（无水分），第六轮三批全部闭合**——run 条件真接（reset_input_state.run_if(on_event<NewSessionStartedCmd>)、input_record_replay.run_if(resource_exists<NextSession>) 于 input/mod.rs add_input_cpu_consumers、time_of_day.run_if(resource_exists<NextSession>) 于 run_game.rs:142）、28-系统逐名表落 startup/mod.rs 宿主面。country-core 零错误、三区零改动。

**第六轮全部闭合**：Walls load_state / GameKeyBindings / sheep 纠偏 / audio G6 / Decorators 0x28B / AsyncSaveCmd 判别 / 28-系统+run 条件。剩余 8 项待取证（Decorators JSON 字段名 / GameSaveData·create 载荷 / sheep movement+X4 / Terrain strokes / 0x58B / count_range / 表2 / camera_rig·weather·time_of_day / 5 域写入序）诚实登记挂后续。

**① 六轮收官完成（round6/contracts 累计 9 份）**：r6-28-system-names——GameStartup 28-系统逐名定案（G1=10 资产载入/G2=8/G3·G4 各1/G5·G6 各6）+ on_event<NewSessionStartedCmd> 应用对象（4 消费系统 + resource_exists 2 系统）；r6-closing-status——可直接落地 5 组 + 待取证 8 项全标地址。**六轮全部契约闭环（round4 10 / round5 8 / round6 9）**。

## 第七轮（2026-10-16）聚焦主流程三链推到完成

用户指定最高影响优先：**save/load 主链、ECS 调度器、session 切换链** 三链推到完成。避开三区（rhapsody/roof/wall）。流程 ① 取证→② 实现→父复核（③ 已停）。② 组外申请照旧仲裁。

| 链 | 内容 | 状态 | 证据 |
|---|---|---|---|
| 链1 save/load 主链 | GameSaveData 240B / AsyncSaveCmd 载荷字段名 + Decorators serde + Terrain strokes + 5 域序。① 字段级突破 + 写回尾项定案（35+ 真实样本）。**父复核（2026-10-16）：真实 serde 全落地、写回尾项样本受限（诚实）**——DecoratorsSectionJson{states:[[addr,{state:[{id,DecoratorInfo 8 字段}]}]]} 与 DecoratorInfoJson 已落地（snapshot.rs:447-509），apply Decorators 分支用真实 serde 并 log 状态数，但 0x28B/0x16B 打包→restore_snapshot 真写回仍待①（可变曲线装不进固定 0x70B，样本受限）；Terrain 真 serde→TerrainEditingData 但未写活资源（宿主接线）；Walls 真写回。country-core 零错误——**主流程写回在真实数据证据下的完整收敛位置** | **真实 serde 全落地 + Walls 真写回；Decorators 0x28B 打包/Terrain 活资源接线待①或宿主（样本受限诚实）** | round7/contracts/r7-chain1-save-load-CONTRACT.md、r7-chain1-field-CONTRACT.md、r7-chain1-writeback-CONTRACT.md |
| 链2 ECS 调度器 | 28-tuple 槽位映射 + Schedule.run 调用点。**父复核通过（2026-10-16）**：28-系统逐名表 + G2 mesh + G6 音频 + register_core_startup<B,S> 装配序对齐（真实）。低优先待证：28-tuple 槽位 fn_ptr/Schedule.run 调用点/纯 Commands 身份 | **复核通过（无水分）** | round7/contracts/r7-chain2-ecs-scheduler-CONTRACT.md |
| 链3 session 切换链 | 消费端接线。**父复核通过（2026-10-16）**：库内 3 消费端 run 条件已接（on_event/resource_exists），库外 3 宿主注册点契约登记。camera_rig/weather/time_of_day 资源面 | **复核通过（无水分）** | round7/contracts/r7-chain3-session-switch-CONTRACT.md |

其余待取证 8 项（sheep movement+X4 / 0x58B / count_range / 表2）随链推进顺带闭合。整 crate 零错误基线。

**第七轮收尾完成（2026-10-16）**：整合报告已落盘交付——`FINAL-INTEGRATION-REPORT.md`（107 行：推进历程/模块与目录进度/抓出真水分清单/剩余待取证/仲裁汇总/证据索引/结论≈83%）。三链状态：链1 真实 serde 全落地+Walls 真写回（Decorators 0x28B 打包/Terrain 活资源接线待①或宿主，样本受限诚实）、链2/链3 复核通过。后续轮按整合报告剩余项推进。

## 第八轮（2026-10-16）剩余模块全深推——4 组 × 3 agent

**每组 3 agent（①逆向/②实现/③review），不削减**；父作为中央 agent 动态分发（契约/DELIVERY 落盘后分派下一任务、定序、裁决组外申请、逐项复核）。避开三区（rhapsody/roof/wall）。② 组外申请照旧仲裁。

| 组 | 范围 | 状态 | 证据 |
|---|---|---|---|
| G1 环境 | camera_rig / weather·time_of_day / 水 / terrain 剩余（state3/is_shape_inside/光标变换）。**③ review 已交付（基于当前树独立审查）抓到真实整链缺口**：①camera_rig 主相机链全缺（MainCamera::new/main_camera_update/paused/immediately_finish_interpolating/NormalCamera::update）；②time_of_day 太阳→光照→音频链断（suncalc 未接、TimeOfDayLightingSink 空适配、TIME_OF_DAY_SPEED 未读）；③water 事件生产者缺（draw_water 0x141ba9130 未实现 + 3 mask SSBO hash=0 占位）；④两硬伤——terrain_rock_mask 位索引错误（原 (z*512\|x)>>5 位字段，现每格 u32 恒 0）、replay_terrain::terrain_replay_from_edit 恒 None 桩；⑤time_of_day 偏移纠偏 +3→+0xc。深挖方向已发 G1①、修复项已发 G1② | **review 已抓整链缺口，G1② 修复推进中** | round8/REVIEW-G1.md |
| G2 建筑 | stairs / collision / mode_manager·toolbar / photo_mode / stairs_walls | 待①契约 | round8/contracts/*（待） |
| G3 GUI+输入 | GUI 全链 / input 剩余 / paths·prefab·brush_preview / onboarding | 待①契约 | round8/contracts/*（待） |
| G4 数据+工具 | save 内存二进制 / sheep movement·X4 / ui_clutter 0x58B / ivy count_range / components·geometry / asset CPU 面 / history 深层 | 待①契约 | round8/contracts/*（待） |

**4 组文件权（互不重叠，详见 COORDINATION 第八轮段）**：G1 `systems/camera_rig/*`、weather_system.rs、time_of_day.rs、water*.rs、terrain_rock_mask.rs、rocky_terrain.rs、`systems/terrain/*`；G2 `resources/stairs/*`、collision.rs、mode_manager.rs、toolbar.rs、photo_mode.rs、stairs_walls.rs；G3 gui.rs、ui_settings.rs、onboarding.rs、`systems/backend/input/*`、paths.rs、prefab.rs、brush_preview.rs；G4 `systems/save/*`、sheep/*、ui_clutter/*、ivy/*、components/*、geometry/*、asset_libraries/*(CPU)、systems/history.rs 深层。

**流程**：每组 ① 逆向/取证（契约）→ ② 实现（DELIVERY）→ ③ review（REVIEW+深挖方向→①，修复→②）。整 crate 零错误基线；各组按整合报告剩余项闭合，父逐组复核。


## 模块台账

状态：批改已实现，待核对（历史分析 agent 核对后推进为"已核对/仍有缺口"，合成 agent 实现后推进为"复核"，父复核通过为"完成"）。"编译通过≠完成"。

| 模块 | 组 | 状态 | 证据 | 合同剩余缺口（已过期，以 *-DELTA 核对为准） |
|---|---|---|---|---|
| ivy | A 植被 | 复核通过（无水分） | SPEC: A-vegetation/ivy-SPEC.md；DELTA: A-vegetation/ivy-DELTA.md；DELIVERY: A-vegetation/ivy-DELIVERY.md；用户提交 f510c7e7/c164567a；父复核通过 | 完整 grower/root spawner、wall-change point-validity pruner、root/branch 删除策略、UI/ActionSet 门、wall jump/hole 约束、GPU 上传/渲染（后置）；`ivy_leaf_spawner` 需接 GladeSettings +1c 真实旗标 |
| garden | A 植被 | 复核通过（无水分） | SPEC: A-vegetation/garden-SPEC.md；DELTA: A-vegetation/garden-DELTA.md；DELIVERY: A-vegetation/garden-DELIVERY.md；用户提交 f510c7e7/c164567a；父复核通过 | 完整 UI 仲裁/其他 WidgetParams 变体、signifier 所有者、原全局调度、56B ShowPinnedDecoClutter union 与 renderer、下游 garden scatter/audio 消费者 |
| sheep | B 生物 | 复核通过（无水分） | SPEC: B-creatures-audio/sheep-SPEC.md；DELTA: B-creatures-audio/sheep-DELTA.md；DELIVERY: B-creatures-audio/sheep-DELIVERY.md；用户提交 f510c7e7/c164567a；父复核通过 | 完整 Sheep owner 与生命周期、真实 Sheep query 所有者、save/load 接线、spawn/movement |
| music | B 音频 | 复核通过（无水分） | SPEC: B-creatures-audio/music-SPEC.md；DELTA: B-creatures-audio/music-DELTA.md；DELIVERY: B-creatures-audio/music-DELIVERY.md；用户提交 f510c7e7/c164567a；父复核通过 | 完整 GladeAudio track producer、AudioCore/device/bank 启动、live FMOD 集成（libfmod 2.222.6，锁 checksum 75defbe5…）；music_player::load_tracks 已实现，完整 GladeAudio 加载消费者未恢复 |
| input | B 输入 | 复核通过（无水分） | SPEC: B-creatures-audio/input-SPEC.md；DELTA: B-creatures-audio/input-DELTA.md；DELIVERY: B-creatures-audio/input-DELIVERY.md；用户提交 f510c7e7/c164567a；父复核通过 | 键盘/Steam 映射、reset/replay 集成、update_cursor_position、原全局调度、OS 事件生产者 |
| terrain | C 世界 | 复核通过（无水分） | SPEC: C-world-services/terrain-SPEC.md；DELTA: C-world-services/terrain-DELTA.md；DELIVERY: C-world-services/terrain-DELIVERY.md；用户提交 f510c7e7/c164567a；父复核通过 | terrain UI/new-stroke 输入生产者、原 plugin 资源初始化与全局调度、terrain history/replay 生命周期、audio 播放、GPU 执行（后置）、下游 wall/collision/scatter 订阅链 |
| save | C 世界 | 复核通过（无水分） | SPEC: C-world-services/save-SPEC.md；DELTA: C-world-services/save-DELTA.md；DELIVERY: C-world-services/save-DELIVERY.md；用户提交 f510c7e7/c164567a；父复核通过 | 完整 AsyncSaveCmd 变体、deferred 命令、主后台保存循环/unsaved-file 重试、每私有域的游戏线程捕获/序列化、快照二进制捕获/load、自动/手动备份生产、session 切换自动跑 GC、完整 save-session 生命周期 |
| session | C 世界 | 复核通过（无水分） | SPEC: C-world-services/session-SPEC.md；DELTA: C-world-services/session-DELTA.md；DELIVERY: C-world-services/session-DELIVERY.md；用户提交 f510c7e7/c164567a；父复核通过 | 完整 NextSession(160B) 所有者、GameSaveData(240B)/MetaCommand/SessionOnLoadCmd 派发、SessionLoadAttempt、完整 world load/commit 事务；`create` 0x140b46960、`append_on_load` 0x1403b5150 等入口 |
| ui_clutter | C 世界 | 复核通过（无水分） | SPEC: C-world-services/ui_clutter-SPEC.md；DELTA: C-world-services/ui_clutter-DELTA.md；DELIVERY: C-world-services/ui_clutter-DELIVERY.md；用户提交 f510c7e7/c164567a；父复核通过 | 原 RMB picking、parent/roof transform 解析、auto→decorator/stair 转换、UI 交互、消费删除 ID 的 generation/mesh owner（owner 后置可先留真接口） |

注：input 归属已由父确认为 B 组（COORDINATION 所有权表 B 输入行，文件 `systems/backend/input/*`）。涉及组外文件时一律先仲裁。

## DELTA 核对摘要（2026-10-16，A/B/C 组历史核对已完成）

真实剩余缺口清单以各 `*-DELTA.md` 末尾为准，此处登记关键项与跨组归属：

- **ivy**（水分较多）：G6 洞/跳墙约束整段缺（PrevWallHoles 盒包含 0x140f86d00+0x141240140、flag_8c、0.42 落点门）；G2.3 spawner 参数错（count_range 4..16 非 0x14、scalar_range 未按 path 区分、source_60 传 0 非墙 id、每帧 70 上限缺）；G1.5 get_direction 种子用 source_60 非 ivy.id(+0x20)；G1.1 守卫未接 IvyStorage.flag_48；G3.4 根部弹根缺；G2 预算用 next_id 非 total_point_count；G8 send_to_gpu 无消费者且 arch_height/seed 占位（→D 组挂系统链时登记）；G2.4/G5 UI/ActionSet 门待取证。已纠正旧稿：leaf spawner 门是 Res<GladeSettings> 读 +1c（非 In<bool>），该 G7 项真实实现。
- **garden**（基本真实无冒充）：P2 已定论（A 组原侧差分 garden-draw.asm 0x140ae4651..677：draw_garden 跳表只分派 tag17(Garden)/tag9(Eraser)，tag13(Replay) 落入默认 epilogue 0x140ae4b7a 直接返回——**draw_garden 无 Replay 分支**，现行 `_ => return` + route_garden_replay_feedback 旁路即等效实现，无需改动）。跨组仍缺：ShowPinnedDecoClutter 56B union/renderer（C 组 ui_clutter 域，已由 C 组 DELIVERY 落地 CPU 侧语义）；`garden_audio_system` 130.0 距离门+音量（B 组 audio 域，已路由 B 组实施）；SPEC 修正登记：Eraser 分支 name 为 "eraser"(0x142ae2e80)，现行代码正确。
- **sheep**：capture/apply 真实；仍缺 ①全 crate 无 `struct Sheep`（stride 0x68/move_state@+0x30/being_petted@+0x58）；②save "Sheep" 节载荷接线（save_game.rs 的 sheep: SerializedJson 无调用方，→C 组 save 域需仲裁）；③真实 query owner systems/debug/sheep_controls@0x1416f11a0（组外，需仲裁）。spawn/movement 未抄旧草稿，正确。**纠偏记录（2026-10-16）：being_petted 阈值 0.01@0x142960674（0x3c23d70a），非 0x292f@0x142930f74（后者为地址算术错，round-3 结论据此更新）**。
- **music**：五状态 player/load_tracks/native.rs(AudioCore/Bank/AudioInstance)真实；仍缺 ①load_glade_audio producer@0x1408be360（源码明写"未做"）；②GladeAudio 72B 结构待取证；③`impl MusicBackend for AudioCore` 未做（现泛型 FmodMusicBackend 注入）；T5 DelayedBankLoader::load 薄壳待定。
- **input**：translate_input/reset_input_state/input_record_replay 真实且常量 rne 复核通过；仍缺 ①update_cursor_position 与 SPEC 签名不符（无 CursorPositionSS/CursorMovedScreenSpace，现 I 级近似）；②UiHistoryButtonPressed 死参数（+0x98/+0x99 undo/redo 折叠未做，生产者组外）；③mod_key（update_mod_key_state@0x140af9c00）不存在；④GameProfile 用硬编码 default 表（0x14089b110 逐槽待核）；trailer Save 按钮只 log。
- **save**：主循环 save_thread 0x141720680/分发 0x141721aa0/节拍器真实（常量 CAP8.0/THRESH1.0/STEP-1.0/gate60/桶 300/600/3600/7200 全部核实）；仍缺 ①save_thread/save_game/snapshot/GC 无生产接线（仅测试调用）；②AsyncSaveCmd 全变体字段名编造（需纠错删编造名、GameSaveData 显式标 U）；③capture/load 域采集器（wall/roof/decorator 三采集器 0x141305170/0x141308b40/0x141309840）未接；④retention 删除队列/BackupSave history-auto 前缀生产未接；⑤ManualBackup 身份比对缺；⑥load_game 系统本体（app_mode==0xe 门、loading.tmp、init_game_state_from_snapshot）未接生产。
- **session**：NextSession owner/append_on_load（fence + 批量 0x1403b5150）/ignore_snapshot/delete_session 真实；仍缺 ①load_game 消费端未接（commander_update 已写 NextSession，无系统消费回放结果）；②create 缺 96B 模板+32B 选项+24B 串/路径字节旗标（现仅 id+kind）；③load_latest 随机 UUID 占位（应取最近会话）；④SessionLoadAttempt/SessionMetaSource/copy_dir_all 缺；⑤SessionLoadedCmd/NewSessionStartedCmd 事件只注册无生产者；⑥NextSessionKind 变体 1/3 未完整表达。注意：session.rs 改动牵动 B 组 music 读 ActiveSession，布局改动需仲裁。
- **terrain**：deferred_terrain_history 0x140ac9770 真实（哨兵/0x38B 记录/History::add/8B 事件）；仍缺 ①terrain_hover_idx 高度≤0 分支与机器码相反（≤0 且 id 有效要写）、hovered 位门缺、dist 门无条件应用（应仅高度>0）；②ui_draw_terrain 每帧 new_stroke 假行为（应状态机点按分支启动）+ 结尾 jumptable + tag 0xc 界外事件缺；③replay_terrain + TerrainReplayCmd::forward/backward 完全未实现（SPEC 标"可直接落地"）；④ui_edit_terrain 16 位脏掩码（仅 3 位自创）与台阶/decorator Bucket push 缺；⑤terrain 系统全未注册生产（register_terrain_events/add_terrain_tool_systems 仅 tests）；⑥EditType::Terrain 入共享 history 的 tag 定 15 未取证（SPEC 预期 9..12），共享文件改动待仲裁确认。
- **ui_clutter**：history 链已接生产（history.rs→register_ui_clutter_resources/add_ui_clutter_cpu_systems 6 系统）；仍缺 ①rmb_autoclutter 是 ID 适配器冒充原 picking（无 hashbrown probe/组件表/0x58B 记录）且**错误写 DeletedAutoClutter**（原版不写）；②EvRightClickedAutoClutter 事件无生产者（decomp 行 254-272 推 0x48B）；③History::add 经间接链 tag 13 而非原版直接 tag 0xd；④ui_convert rotation=0 桩（get_clutter_rot_in_roof_space 死代码未接）+ AutoClutterParentMap 计数器冒充 + InstaCreateDeco/StairPoint 缺；⑤decoclutter_show_pins 6 行桩 vs 1128 行 decomp + register_required_components 空桩；⑥事件链（RmbAutoClutterHit/EvRightClickedAutoClutter/SpawnAutoClutter）无生产/消费闭环；⑦Deletion→generation/mesh suppression 接口未建。

## D 组（rhapsody 接入，新增第四组）— 复核通过（无水分）

- 状态：**复核通过（无水分）**（父 2026-10-16 逐项核对 execution_host.rs/rhapsody_contract.rs/rhapsody_types.rs 与 crates/rhapsody 真实签名：execute_graph_with_context executor.rs:277、VulkanGraphBackend::new vulkan.rs:309、AccessTable::original vulkan.rs:74、TimelineSemaphore::completed semaphore.rs:21、ExecutionChain::new executor.rs:614、RenderGraph::new graph.rs:159、RenderPassApi::dispatch_groups_3d pass_api.rs:595、GraphExecution.stats.submit_ns/record_ns 全部真实匹配；转发类型均指真实类型；缺口 10 条登记不补造；rhapsody 本体零改动）。证据：`D-rhapsody/rhapsody-WIRING.md`、`D-rhapsody/rhapsody-DELIVERY.md`。
- DELIVERY 防水分核验：逐项分离真实接线（8 文件）/ candidate 链（D 组文件编译干净，整 crate 13 错误全在 A/B/C 在制文件，未跑 cargo test 已登记）/ 原侧验证（rne-read import_unowned prologue、device.rs O 证据）/ 仍缺项 10 条（登记不补造）。rhapsody crate 本体零改动，无虚构函数。
- **整 crate 验收门**：整 crate `cargo check` 仍被 A/B/C 组在制文件（roofs/geometry/*、systems/save/*、audio/music、sheep_controls、ivy_grower 等 13 错误）阻塞；D 组本身已复核通过，整 crate 门等 A/B/C 组收敛后做。GPU 真机后置。
- 映射表要点：GraphicsDevice(pub u64)→Arc<rhapsody::raw::device::GraphicsDevice>（raw/device_api.rs 适配层存在）；ManagedHandle→raw::managed；TextureDesc→raw::texture；CompiledShaderPipeline/RasterPipelineDesc→rhapsody::shader_pipeline；ExecutionChain 落 render_frame.rs RenderHost（execute_main_graph/begin_frame/end_frame），构图走 rg::graph::RenderGraph+PassBuilder::render_with_context，后端 rg::vulkan::VulkanGraphBackend。
- 5 处 panic 桩替换计划：create_persistent_descriptor_set（签名需 state+writes）；issue_full_pipeline_barrier（直改 vk::CommandBuffer）；dispatch（删桩→RenderPassApi::dispatch_groups_3d）；update_persistent_descriptor_set/full_barrier_required 登记为缺口不补造。Cargo 加 rhapsody.workspace=true 无 feature 冲突。
- 文件所有权：`crates/country-core/src/rhapsody_contract.rs`、`src/rhapsody_types.rs`、`src/render/*`（含 pass_builder/nani）、`src/systems/render_loop/*`、渲染侧 `src/asset_libraries/*`、`crates/country-core/Cargo.toml`（加 rhapsody 依赖）。
- 证据入口：evidence/render-recovery/、country-core-deep/rhapsody/、crates/rhapsody/README.md。
- 起点状态（WIRING 已覆盖）：`Cargo.toml` 无 rhapsody 依赖；`rhapsody_contract.rs` 5 处 `panic!("GPU backend is not connected")` 桩；`rhapsody_types.rs` 本地占位；全 crate 无 `ExecutionChain/ExecutionBackend/execute_graph` 引用。
- **`crates/rhapsody` 本体任何组都不可改**；只调用其已存在公开 API，缺接口登记缺口。
- 交付：`D-rhapsody/rhapsody-WIRING.md`（历史分析/核对，已完成）＋`rhapsody-DELIVERY.md`（实现，**已完成**）。GPU 真机执行不验收（后置）；本组验收=真实调用接线 + cargo check + 原侧入口核对。

## 跨组文件仲裁记录

共享但需仲裁的文件：`resources/history/*`、`systems/history.rs`（history 账本；ui_clutter 与 terrain/garden/save 都可能消费）、`resources/glade_settings.rs`（ivy 读 +1c 旗标）、`resources/session.rs`（music 读 ActiveSession）、`resources/audio*`/FMOD、`rhapsody` crate 本体（任何组都不可改，缺接口登记缺口）。

| 时间 | 请求方 | 目标文件 | 请求 | 裁定 |
|---|---|---|---|---|
| 2026-10-16 | B组 a907e474d80314271 | systems/save/save_game.rs | sheep "Sheep" 节载荷接线（SaveFileSection::Sheep=5，原生 @0x140a51d40）；现 `sheep: SerializedJson` 参数无调用方 | **批准接口路径**：B 组提供纯函数接口 `to_json_string<Option<SheepControlSnapshot>>`（None→null），不直接写 save_game.rs；接线由 C 组 save agent 在 save_game.rs 完成（避免与 C 组并行写同文件）。已通知父与 C 组。 |
| 2026-10-16 | B组 a907e474d80314271 | systems/debug/sheep_controls | 落地 `SheepControl` 调试资源 + `sheep_controls` 系统（query (Entity,&mut Transform,&mut Sheep,&mut CritterGoal)）+ `apply_sheep_snapshot` @0x1416f24f0 | **批准**。systems/debug/* 无归属组、无其它组写入；真实 query owner 消费者（0x1416f11a0），属"加已观测消费者"。 |
| 2026-10-16 | B组 a907e474d80314271 | resources/window.rs | 加 `CursorMovedScreenSpace` 事件定义（原类型 country_core::resources::window，PDB into_configs @0x1413ee760，WindowSize 同级） | **批准**。window.rs 无归属组、无冲突；按原类型定义，供 input update_cursor_position 重做签名。 |
| 2026-10-16 | B组 a907e474d80314271 | resources/audio_resources.rs | `EventInstance` trait 补 `install_beat_callback`（MusicEvent 必需，spec §2.4 方案 A 阻塞点） | **批准**。audio 共享文件，仅加观测到的 trait 方法（native.rs set_callback(TIMELINE_BEAT) 已存在）；impl 放 B 组 music 模块，不改 audio_resources 其它部分。当前无其它组写入该文件。 |
| 2026-10-16 | C组 a9c6f52b7cc06d63f | startup/run_game.rs | 生产接线：`register_core_startup` 追加 save（add_save_load_systems_impl：save_game_system 节拍器+BackupSave 生产、load_game_system 消费 NextSession 门 AppMode::LoadGame、init_save_thread 生产 save_thread）+ terrain（register_terrain_events/add_terrain_cpu_systems/add_terrain_tool_systems 含 replay_terrain）。系统依赖宿主注入资源，运行期缺资源由宿主提供、不伪造 | **批准**。startup/run_game.rs 为共享装配点（ui_clutter history 链已由 C 组注册）；save/terrain 生产注册是 DELTA 标"仍缺"的真实缺口闭合。注明：该文件后续 A/D 组挂系统链时需再经仲裁定序。 |
| 2026-10-16 | C组 a9c6f52b7cc06d63f | resources/history/replay_cmd_buffer.rs | 加 `ReplayCommand::Terrain(TerrainReplayCmd)` 变体 + `ReplayBucket::expand` 对 `EditType::Terrain` 展开 + `make_animations_instant` 分支；我方已实现 systems/terrain/replay_terrain.rs::TerrainReplayCmd（forward/backward 时长 clamp(Σ点距*0.01,0.4,3.0) 无点 0.3；instant/动画 eased=1-(1-t)^3） | **批准**。resources/history/* 为共享文件，但为"加已观测变体/消费者"（replay_terrain 缺口闭合），当前无其它组并行写该文件；C 组在批准下动该共享文件。 |
| 2026-10-16 | C组 a9c6f52b7cc06d63f | resources/history/mod.rs（EditType::Terrain tag） | tag=15 为自定、SPEC 预期 9..12 证据不足；请求台账登记"待取证"，不改共享文件 | **批准登记**（非文件修改）：EditType::Terrain 原 tag 记为待取证，暂用 tag=15 不动共享文件；deferred_terrain_history 侧已诚实标注。待 RNE 取证后修正。 |
| 2026-10-16 | B组 a907e474d80314271 | src/lib.rs | 为已批准的 systems/debug/sheep_controls 落地追加 `pub mod debug;` 声明 | **追溯批准登记**：lib.rs 为共享 crate 根装配点，该声明是已批准 systems/debug 工作的必要接线，无并行写报告；后续其它组在 lib.rs 加模块声明需协调避免冲突。 |
| 2026-10-16 | A组 aa5b60b8c8fc6e7a7 | systems/audio/garden_audio.rs | garden_audio_system 的 130.0 距离门（0x43020000）与音量调制（[vol_res+0x1c]/除数）位于 B 组 audio 域文件；A 组 producer 侧 draw/erase 事件已满足契约 | **路由 B 组后裁定为跨组缺口**：B 组不实施（systems/audio/garden_audio.rs 不在 B 组严格所有权；音量/光栅两个资源身份未取证不伪造）。完整已解码契约登记 music-DELIVERY 新增段（门=float_grid::bilinear_sample_at_world_xz<RasterBufferRef<512>>、采样≥0.01 分流 DrawWater/EraseWater、音量 [vol_res+0x1c]/60.0 当 mode!=0xd、130.0=0x43020000 rne 复核、native 6 参签名），待正确 owner 确认资源身份后关闭或 B 组补做。 |
| 2026-10-16 | A组 aa5b60b8c8fc6e7a7 | workspace 级（Cargo.toml 成员/依赖） | workspace 正被并行重构（新成员缺目录 water/forest、country-core↔system-roof 依赖循环、roof crate staged 删除）阻塞整 crate 编译验收，建议中央仲裁重构窗口 | **已升级父**：中央仲裁重构窗口/归属，避免各组 cargo check 互相踩；各组各自文件验证不受影响。已登记。 |
| 2026-10-16 | ② 实现 ab30764d525b73aea | resources/cursor_terrain_raycast.rs | 按 ① T1 契约（PDB procs.tsv 49396：param_5=CursorTerrainRaycast，原生 32B {tag:u32@0,pos:Vec3@0x04,delta:Vec3@0x10,speed:f32@0x1c}）由 16B 扩为 32B；tag u8→u32（`tag&1`/`tag==0` 兼容）、position()/distance()/update() 保持、8 处测试构造点补 `..Default::default()`；garden_audio 随契约对齐（param_6→WaterRaster、音量读 cursor.speed/60.0、删 GardenAudioVolume 适配、模板补 PathSize=2*inverse_lerp(0.32,1.2,radius)） | **批准**。共享只读资源，① 契约驱动、增量字段（delta/speed）追加、既有消费面（tag 位测试/position/distance）行为不变；无并行写报告（仅 garden_audio 读 speed）。仍待取证：param_7+0x58 除数身份、GardenAudioState @0x10/@0x1c 语义。 |
| 2026-10-16 | ② 实现 ab30764d525b73aea | resources/history/mod.rs（EditType 表） | 按 ① T3 契约（t3-edittype-terrain-tag §2，visit_str 0x140b3b630 权威全解）修正 EditType::native_tag：Tree tag 0→12（case4 "Tree"→0xc）、Terrain tag 15→0（case7 "Terrain"→0）且变体序按原生对齐（Terrain 首变体、Tree 移 tag12 槽）、其余 13 变体不动；deferred_terrain_history 机制定案：地形历史不经 EditType（0x140ac9770 0x38B 记录 {tag=1, 6 qword} 直入 History Vec），crate EditType::Terrain/ReplayCommand::Terrain 为本地模型保持+标注（CPU 侧等效）不改走 EditType | **批准**。共享 history 文件、① 权威契约驱动修正已确认双错（Tree=0/Terrain=15→Terrain=0/Tree=12）；② 须一并修 crate 内引用旧 tag 的消费点（deferred_terrain_history/replay_cmd_buffer）；deferred 机制保持本地模型+标注；无并行写报告。影响存档/回放序列化 tag 对齐属正确性修正。 |
| 2026-10-16 | ② 实现 ab30764d525b73aea | systems/save/snapshot.rs、systems/save/load_game.rs | 按 ① T3 契约（t3-save-domain-collectors）落地 save 域缺口：snapshot.rs 补 Wall 采集器（0x141308b40，inner_walls::all() HashMap<WallId,WallState>→Vec）与 u64 采集器（0x141309840，HashSet<u64>→Vec，来源集身份待取证），Tree 已有 collect_trees；load_game.rs 补 PostGameLoadCommand::Apply（FUN_1412e0b90）=Commands::insert_resource<NextSession> 桥（现只登记不应用）。序列化节布局/Apply 载荷→NextSession 重构字段待取证，不编造字段名 | **批准**。① 契约驱动、真实缺口闭合（F8 域采集器/F9 Apply 未应用）；inner_walls 容器访问面需确认；u64 来源集身份、节布局、Apply 载荷字段待取证登记不补造；无并行写报告。 |

规则：
- 只加已观测字段/消费者且不与其它组冲突 → 批准并登记。
- 两组要写同一文件 → 指定先后顺序，通知父裁决。
- 未经批准的组外修改一律拒绝并记入本台账。
- rhapsody crate 本体不可改；D 组缺接口时登记缺口而非补造函数。

## DELIVERY 摘要（2026-10-16，四组全部交付，均父复核通过无水分）

各 DELIVERY 防水分核验：真实代码接线 / candidate 链测试 / 原侧验证 / 仍缺项逐项分离。仍缺项均诚实登记不补造。

- **sheep**（B，父复核通过）：`Sheep` component（stride 0x68、move_state@+0x30、being_petted_timer@+0x58，未知区间不填）+ `being_petted` 谓词；`sheep_section_json` 纯函数接口已放 control.rs（C 组已接线 save_game.rs）；systems/debug/sheep_controls.rs 落地（SheepControl 资源 + sheep_controls + apply_sheep_snapshot，lib.rs 加 pub mod debug）。仍缺：字段布局未知区间、register_required_components×5、picking/UI 交互内联分支、spawn/movement（保持待取证）。**纠偏记录（2026-10-16）：being_petted 阈值常量原记 0x292f@0x142930f74 为地址算术错，正确为 0.01@0x142960674（0x3c23d70a，③ 独立解码 + 父 rne 复核）；② 修 sheep.rs 中**。
- **input**（B，父复核通过）：update_cursor_position 按 PDB 签名重做（读 ActionSetStates→写 CursorPositionSS→发 CursorMovedScreenSpace，window.rs 已加事件）；UiHistoryButtonPressed 补 u8 载荷消费（undo/redo→digital[16]/[17]）；mod_key.rs 新建（Modifiers 资源 + 两已证谓词）；BindingsTable::default 40 槽逐字节复核 0x14089b110 全对上。仍缺：update_mod_key_state（7-handler 跳表）待取证未造空壳、GameProfile 直读（组外占位）、UiHistoryButtonPressed 生产者（UI 域）、trailer Save/Replay 按钮仍只 log、Steam 全链库外。
- **music**（B，父复核通过）：load_glade_audio producer 按 0x1408be360（pause→GladeAudio::load→4 Halloween 事件→load_tracks→GladeTypeIsHalloween/IsHalloween 参数→theme 旗标 0.25）；GladeAudio 72B 字段名由发行 assets/glade/{5 主题}/audio.json 全量核实 + default 回退；impl MusicBackend for AudioCore（AudioCoreEvent 适配，EventInstance trait 已按批准补 install_beat_callback）。仍缺：theme 旗标字段语义 T3/T4 待取证、configure_glade_audio 进 GameStartup 调度（run_game.rs 库外需仲裁）、DelayedBankLoader::load 薄壳待取证、live FMOD 真机后置。
- **save**（C，父复核通过）：生产接线（SaveThreadHandle/spawn/init_save_thread + save_game_system 节拍器备份生产 + load_game_system + run_game.rs 接入，均按批准）；AsyncSaveCmd/GameSaveData 字段名纠错（语义字段+自定名显式标注 U/待取证）；ManualBackup 身份比对（0x53/"snapshot" 0x746f687370616e73）+ retention 删除；sheep 节 producer（sheep_section_serialized 接 B 组接口）。仍缺：域采集器（wall/roof/decorator 三采集器）、set_current_snapshot→CaptureSnapshot(9) 链、0x141721104 jumptable、身份比对载荷结构 U。
- **session**（C，父复核通过）：load_game_system 消费 NextSession 链（门 0xe/两遍回放/loading.tmp/MetaCommander/事件）；load_latest 真实语义（取最近会话）；SessionLoadAttempt（theme@+6 默认 3）/SessionMetaSource/copy_dir_all（intro 跳过）；create 96B/32B/24B 载荷面登记。仍缺：load_game_session 完整系统（78KB）、会话切换系统 + NewSessionStartedCmd/SessionLoadedCmd 生产者、create 载荷字段名 U。
- **terrain**（C，父复核通过）：terrain_hover_idx 两分支门控修正（≤0 且 id 有效要写）；ui_draw_terrain 状态机 + tag0xc 界外事件 + new_stroke 移到点按分支；replay_terrain + TerrainReplayCmd（duration clamp 常量 RNE 核实）接入共享 ReplayBuffer；生产注册（run_game.rs 批准）。仍缺：jumptable 分支 0/1/3、is_shape_inside/光标变换宿主面、EditType::Terrain tag 待取证、28-系统精确顺序待取证。
- **ui_clutter**（C，父复核通过）：rmb handler 还原原 picking（hashbrown probe/旋转数学/0x48B 事件/History tag 0xd/start_input_recording/**删除错误 DeletedAutoClutter 写入**）；ui_convert parent map + rotation 接入 + InstaCreateDeco/StairPoint 生产；decoclutter_show_pins 56B union 语义 + 真实组件注册；EvRightClickedAutoClutter 闭环。仍缺：AutoClutterRecord 0x58B 布局 U、实体/组件机制宿主面、RmbAutoClutterHit 生产者、SpawnAutoClutter 消费方、suppression 接口（GPU 后置）。
- **ivy**（A，父复核通过，A-vegetation/ivy-DELIVERY.md）：spawner 预算门 total_point_count+0x13<0x9c40（0x14094d09b..a7）+每帧 0x46 上限（0x14094db6e..b9e）+count_range 起点 0x14/scalar 表 0..3/0..1（0x142ab3218）+source_60 墙 id 已落；grower G6 洞约束（LANDING_MAX_DISTANCE=0.42@0x142ab31b0、hole_blocks_growth 盒包含、flag_8c 写/消费）、方向种子改 ivy.id（0x14094b4bd）、flag_48 守卫接入 IvyStorage；pruner 根点 pop_front 已实现；send_to_gpu arch_height 接 wall_arch_height 真实墙 arch。仍缺（诚实登记）：count_range.end、flag_8c leaf 清理谓词、send_to_gpu seed/wall_normal、0.68 vs 0.63、VERTICAL_MIN 疑点等待取证。
- **garden**（A，父复核通过，A-vegetation/garden-DELIVERY.md）：无源码改动；P2 Replay 有汇编差分定论（draw_garden 跳表 tag17/9/默认返回、无 cmp 0xd——"无 Replay 分支"证据成立）。跨组：garden_audio 130.0 门登记为跨组缺口（B 组裁定不实施，音量/光栅资源身份待取证；完整契约已解码登记 music-DELIVERY 新增段，待正确 owner 关闭）。
- **rhapsody 接入**（D，已父复核通过无水分，见 D 组段）。

## 防水分

- "编译通过=完成" 或 "新增测试=完成" 一律打回。
- 历史分析 `*-DELTA.md` 必须逐项标注：已真实落地 / 有水分 / 仍缺，不得用 `contracts.json` 旧 remaining 冒充核对。
- 合成 agent 的 DELIVERY 必须逐项分离：真实代码接线 / candidate 链测试 / 原侧验证 / 仍缺项。
- 发现水分向父报告；不打进度、不改台账状态直至 DELTA/DELIVERY 合规。
