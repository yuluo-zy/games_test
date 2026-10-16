# 分派台账 DISPATCH

> 中央智能体维护。模块 → 组/角色 → 状态 → 验收门槛。状态:queued / in_progress(restorer) /
> in_review(reviewer) / needs_reanalysis / fix_pending / review_passed / blocked。
> 更新时保留日期与更新者(agent 名)。

## 初始化分派(2026-10-15 主会话)

依据 `recovery/README.md` 2026-10-15 优先级表分三组。初始分派状态由主会话于 2026-10-15 写入
(10 个后台 agent 已并行启动;coordinator 异步细化验收门槛并流转状态)。

2026-10-15 coordinator 细化:验收门槛按原函数清单/真实消费者/原生证据分列;文件在位性 =
当前 `src` 与 inventory `expected_formal_path` 逐文件比对;"在位=N" 表示该原文件尚未创建,
restorer 需新建。`in_progress` 状态沿用主会话写入值(restorer 已开工,报告尚未落盘)。

## 本批收口确认(coordinator, 2026-10-15;9 模块 review_passed(限定))

| 模块 | 状态 | 限定边界(验收层面开放项) |
|---|---|---|
| `resources/ivy`+`systems/ivy` | review_passed(限定) | GPU 提交待 render 域、SsboId::of 对账、grower 主闭包逐相/48B↔104B 叶映射/噪声阈值待取证 |
| `systems/garden` | review_passed(限定) | pin 渲染与 scatter/audio 消费、全局注册(impl$0::build)待接线 |
| `resources/critters/sheep` | review_passed(限定) | 完整 Sheep owner/being_petted 边界外;存档 Sheep 节接入=save 消费端 |
| `systems/audio/music` | review_passed | native FMOD 现场集成、GladeAudio 曲目生产者待接线 |
| `systems/save` | review_passed(限定) | GameSaveData 域组装(域类型已派 re-analyst-state)、commander_update/on_load 回放、load_completion_checker |
| `resources/session` | review_passed(限定) | 完整切换调度/MetaCommand 应用/全局接线(会话切换链) |
| `systems/backend/input` | review_passed(限定) | 动作↔槽映射、Steam 全链、帧 serde/try_load/SyncPoint、录制往返 |
| `systems/terrain` | review_passed(限定) | EditType::Terrain 真 tag、focus_mode switch 表、ui_draw 消费面、脏掩码位序、AppMode==10 门裁决 |
| `systems/ui_clutter` | review_passed(限定) | DecoratorStorage 实体、ui_convert quat、decoclutter 注册、ui_focus 主体、debug_spawn 消费端 |

- **确认口径**:"限定复验"= 已实现范围的原函数与真实消费者/机器码逐条对上,无空壳/假数据/伪造
  加载成功;**不等于整模块完成**。开放项已按依赖编排为下一批任务卡,见
  `planning/2026-10-15-next-batch.md`(卡0 域类型证据已派 re-analyst-state)。
- **编译注**:本批收口时 `cargo check -p country-core` 通过(库);`--lib --tests` 曾因 restorer
  在途文件(~20 errors)失败,已在途收口;以各轮 RESTORE/REVIEW 落盘时的现场 check 为准。
- **证据链**:contracts.json 的 `evidence/core-recovery-20261015/*/REPORT.md` 缺口追踪见
  「证据链缺口追踪」节(OPEN→INVESTIGATED→REANCHOR_PENDING)。

### 组 eco(生态编辑)— restorer-eco / reviewer-eco / reanalyst-eco

| 模块 | 起点 | 本批目标 | 验收门槛(coordinator 细化 2026-10-15) | 状态 |
|---|---|---|---|---|
| `resources/ivy` + `systems/ivy` | 仅 closest_segment_index,无权威存储 | 实际株/叶存储、坐标与动画消费(ivy_storage/ivy_leaf_storage/ivy_leaf_animation/ivy_grower/ivy_spawner) | **原函数闭合**:13 原文件 46 业务函数。在位:ivy_storage(20)、ivy_leaf_storage(7)、ivy_direction_proposer(3)、wall_coord(3)、ivy_grower(11)、ivy_pruner(4)、ivy_leaf_spawner(1)、ivy_leaf_animation(1)、ivy_branch_thickness_animation(1)。**缺文件(需新建)**:ivy_spawner(5:ivy_spawner/try_random_ivy_root/try_ivy_root_next_to_cursor/is_point_inside_wall_segment)、ivy_send_to_gpu_leaf(4)、ivy_send_to_gpu_segments(4)、mod.rs(add_ivy_systems/insert_ivy_resources/add_ivy_events;registration.rs 仅部分覆盖)。派生胶水不计独立业务。**消费者**:spawn→grow(事件)→leaf_spawner→pruner→leaf/branch 动画→GPU 发送全链;render_loop draw_shaders/instanced_ivy::draw_instanced_ivy_branch_shader;decorator::remove_ivy_under_windows;tiny-glade run_game。**证据**:各文件章节 + fresh 反编译(存储 0x14090e510 区、grower、spawner VA)+ round3 ivy closure。区分实现层面(库内有株/叶存储可看台账)与验收层面(生产/消费链、GPU 消费闭合)。 | review_passed(限定;GPU 提交待 render 域、SsboId::of 待 ssbo_library 对账,grower 主闭包逐相语义等按 RESTORE §四待取证) |
| `systems/garden` | 原两文件均无系统实现 | 接真实模式/画笔/UI/边界,绘制与历史记录(draw_garden/record_garden_history) | **原函数闭合**:3 原文件。在位:draw_garden(4:draw_garden+3 closure)、record_garden_history(1)。**缺文件(需新建)**:systems/garden.rs(1:impl$0::build 插件注册/调度;当前 mod.rs 有事件与 replay 路由但不含原 build)。**消费者**:world_raster::draw_raster<GardenRasterStagingSsbo>、History::Garden 规范记录 + replay 反馈、GardenDraw/Erase/PinnedClutter 事件、pin 渲染与 scatter/audio 消费、全局注册。**证据**:章节 + fresh 反编译(draw_garden/record_garden_history VA)+ round3 garden closure。 | review_passed(限定) |
| `resources/critters/sheep` | 仅移动类型谓词,旧草稿有 Gap | 核实移动状态和快照字段,实现捕获/恢复;不复制空壳 Sheep | **原函数闭合**:2 原文件。sheep.rs(28)在位但仅 SheepMoveState 枚举 + control;业务谓词 = Sheep::being_petted,其余为 serde/register_required_components 胶水(按 AGENTS.md 计合同非独立业务)。control.rs(15)在位:capture_sheep_snapshot、apply_sheep_snapshot、SheepSnapshot/SheepControlSnapshot schema。**边界**:完整 Sheep owner 按 README 不复制空壳;验收聚焦捕获/恢复闭合 + 存档 Sheep 节接入。**消费者**:save_game::to_json_string<Option<SheepControlSnapshot>> 与 save_game(Sheep 节)、systems/debug/sheep_controls(handle_mouse_picking/sheep_controls)、system_critters::sheep_behavior::set_sheep_velocities_towards_goals 与 sheep_walk_animation。**证据**:章节 + fresh 反编译(capture 0x1412f03f0 / restore 0x1416f24f0 / serializer 0x1416f9eb0/0x1416f9fd0)。 | review_passed(限定) |

### 组 eco 评审结论(reviewer-eco, 2026-10-15 首轮;REVIEW.md 落盘 ivy/garden/sheep)

- **ivy = fix_pending**:storage(20+7)/direction_proposer/closest_segment_index/leaf_animation/branch_thickness/leaf_spawner/pruner_cmd 与机器码一致(OK;closest_segment_index 过滤 1 方向已按 RNE ucomiss 定案:两端点均 > max 才跳)。**FIX-1 确认**:WallCoord::into_world_space 原生 out.y = v±offset_z(RNE 0x140b19acd/b72..ba7),当前仅 v,函数无调用者可后置。**GAP**:grower 10 相状态机(0x142ab3be0/world+0x118)/空间哈希/地形格仍候选近似;ivy_spawner/pruner 近似且作者自注待取证。ivy.rs 未声明 pub mod ivy_grower → tests E0432×3。
- **garden = review_passed(限定)**:模式/笔刷/UI/边界/绘制/历史四链与 0x140ae4620/0x141695e10 对上;章节"半径=笔刷+1.5"实为引脚高度,工具 id 9/5 已证。登记映射即可。
- **sheep = fix_pending(边界由中央定)**:SheepMoveState serde 对上 0x1413ff520;capture/apply 语义合理;control.rs 快照 wire decomp(0x1416f9eb0/0x1416f24f0)本机缺失待取证;**完整 Sheep owner/being_petted 组件族按 README 降为本批边界外**;本批验收=捕获/恢复闭合+存档 Sheep 节接入。
- **实测**:cargo check(lib)通过;cargo check --lib --tests 失败(~20 errors,restorer 新文件在途,含 save/ivy 接线)——README "all-targets 编译"仅对入库前快照成立,在途不算。

### 组 eco 深度逆向产出(re-analyst-eco, 2026-10-15;RE-ANALYSIS.md×3 + 14 份 decomp evidence/country-core-deep/eco-reanalyst/decomp/;11 章节已升级)

- **ivy 生产者→GPU 全链**:ivy_spawner@0x14094cf90 O 全解(门 AppMode==1;株数上限≈40000;路径 A=光标旁根/路径 B=随机根;perchance-0.4.0 RNG range 断言 end>=start@0x142aa5710);存储布局 IvyPoint=0x28B{id,pos,dir,flags}、Ivy=40B 容器+point_id@+0x58、IvyStorage=HashMap+RNG@+0x30+next_id@+0x40、IvyLeaf=0x68B;GPU SSBO 键 IvyLeafSsbo=0x9e3cfa7f2993f406/IvyFlowerSsbo=0x1d6de5e69ac50d2a/IvySegmentSsbo=0x2002d2fbdff3d80f、GPU 元素 48B、add_ivy_systems=22;**closest_segment_index 字节级纠错(0x14094c8c0..c960)**:仅 h0>max 且 h1>max 跳过,参数序(out,xz,0,segments,count,max_height)——与 reviewer RNE 定案互证。
- **sheep 快照 schema 定案**:SheepControlSnapshot{sheep:Vec<SheepSnapshot>}、SheepSnapshot{position:Vec3,goal:Option<Vec3>,move_state:SheepMoveState}(serde 字段名 rne-read 定案);存档节=Option<SheepControlSnapshot>(None→"null");SheepMoveState 判别@+0,0=Idle{timeout_s}/非0=Walking{goal_change_timeout,<f32>,force_reach_goal};capture Query=(Entity,&mut Transform,&mut Sheep,&mut CritterGoal) 内联 save_game;apply@0x1416f24f0 40B→组件{+0x00..+0x38} 逐字段。
- **garden**:replay_garden closure$1::closure$0@0x141447500 I→O;garden_mask 维持 name-only U。
- **卡点(blocked)**:SheepMoveState::Walking 第 2 f32 字段名(=goal_change_timeout,二轮已闭合);Sheep 组件完整布局(systems/critters 跨域,U);ivy GPU 48B↔104B CPU 叶精确映射;Flower03Ssbo NAME(round3 RENDER-U-CLOSURE 已登记)。
- **ivy_spawner 路径 B 已补齐(re-analyst-eco 续派, 2026-10-15;RE-ANALYSIS2.md)**:func_0x140c9cdc0=Vec2::x0y swizzle;侧向=get_bool(FUN_141625a00,调用点 0x14094d6b0,true→−0.68/false→+0.68);2.56 距校验(_UNK_142ab3214,判向点积+|Δpos|²<0.25/2.56 分路径);垂直 u_spawn=(rand×1.5−0.75)/curve_length(PublicWallState+0x30)+clamp01(get_approx_u_from_pos FUN_141469a20),钳[0.000928482@0x142960690,1.0];spawn range=usize_less_than(墙数)+uniform_f32[0,1)+FastNoise 密度闸门(FUN_141743760 noise≥res+0x78)+尝试{2,3,4};grower 10 相指认 chapters+ALGORITHMS §4.1。未决:res+0x78 阈值(IvySpawnNoise 构造)、get_approx_u_from_pos 投影(共享 utils)。

### 组 eco restorer 二轮(restorer-eco, 2026-10-15;RESTORE.md×3 已落盘 ivy/garden/sheep)

- **ivy → in_review**:FIX-1(WallCoord out.y=v±offset_z,RNE 0x140b19acd/b72..ba7)已修;`pub mod ivy_grower` 已声明(grower 自入库);ivy_spawner 按 re-analyst O 全解重写(门 AppMode==1、<3 光标/≥3 随机、±0.34 偏移、0.5 距校验、垂直钳 [0.000928482,1.0]、逐墙随机 u);ivy_pruner 补 is_point_still_valid+墙变更系统;registration 补 add_ivy_systems/insert_ivy_resources/add_ivy_events;IvyDirectionProposer 补 Resource。**仍待**:grower 10 相状态机/空间哈希格采样/max_ivy_grow_height 数值(待取证);GPU 发送链(send_to_gpu_leaf/segments)登记待接线(render 域)。`cargo check -p country-core` 通过;ivy 23 测试全过。
- **garden → review_passed(限定)**:映射已登记 RESTORE.md;startup_garden_brush 网格名纠偏(garden_flower_03 / lowpoly_flower.json,read_memory)。
- **sheep → in_review**:serde 字段名 rne-read 定案(sheep@0x142c91fa9、position/goal/move_state@0x142c91f49/51/55)已注记;SheepMoveState Walking 内存序注释(中间 f32 字段名待取证);capture/apply 闭合测试过。完整 Sheep owner/being_petted 按中央裁决边界外。存档 Sheep 节接入=save 组消费端。
- **实测**:`cargo check -p country-core` 通过;`cargo test -p country-core --lib`(ivy/garden/sheep 过滤)通过。

### 组 eco restorer 三轮(restorer-eco, 2026-10-15;reviewer-eco 二轮 + re-analyst-eco RE-ANALYSIS2 吸收)

- **ivy → in_review(三轮)**:①新增 `ivy_send_to_gpu_leaf.rs`/`ivy_send_to_gpu_segments.rs`(库内缺失闭合)——元素+SsboType 合一(对齐 GroundedBuildingsSsbo)、48B GPU 元素、upload_ivy_leaves/upload_ivy_segments 纯函数、SsboId 常量 rne-read 定案(0x9e3cfa7f2993f406/0x1d6de5e69ac50d2a/0x2002d2fbdff3d80f);已知建模 gap:SsboId::of 哈希 NAME≠原 SsboId(同 GroundedBuildingsSsbo);GPU 提交需 GraphicsDevice(render 域)。②spawner 路径 B 按 RE-ANALYSIS2 重写:随机墙+噪声闸门+get_bool 定侧(删自造掷硬币)+2.56 距校验+`(uniform×1.5−0.75)/curve_length+u_on_curve`(approx_u_from_pos 精确投影)。③grower 10 相状态机结构重构(GrowerState Local:phase@+0x1c/guard@+0x48/timer@+0x11c,跳表分派相0守卫/1·3·4主/2计时/5二级/6-9 panic)。ivy 28 测试全过。仍待取证:grower 主闭包逐相语义/二级表目标、spawner 空间哈希格采样、噪声阈值来源(res+0x78)、max_ivy_grow_height 数值、GPU 48B↔104B 精确映射。
- **garden = review_passed(限定)**(round-2 无改动)。
- **sheep = review_passed(限定)**(serde wire 全验证)。
- **实测**:`cargo check -p country-core` 通过;`cargo test -p country-core --lib` 1591 过/1 败(geometry/sphere 他组,非 eco)。

### 组 eco restorer 四轮(restorer-eco, 2026-10-15;reviewer-eco 三轮 MUST-FIX 已修)

- **ivy → in_review(四轮)**:
  1. **grower 守卫闩死(MUST-FIX)已修**:guard 位改由 phase0 每次 toggle 自然交替(守卫体不再清除),phase0 toggle 后即运行所选体,phase1/3/4 按位运行后回 phase0——校验相与生长相隔帧交替,主生长体带 time.delta 推进。新增相循环测试 `phase_cycle_keeps_advancing_growth_and_fires_completion_events`(60 帧,断言 pending 完成+事件触发)。ivy 29 测试全过。
  2. **侧向偏移路径 A/B 对账**:路径 A(光标)=0.68×0.5=0.34(round-1 常量 0x142ab31e0/0x142ab3200×0.5);路径 B(随机)=0.68 由 get_bool 选侧(RE-ANALYSIS2 原文 true→−0.68/false→+0.68,0x14094d6b0)。ROOT_OFFSET/RANDOM_ROOT_OFFSET 分列。
  3. send_to_gpu 库内业务已闭(reviewer 裁决:GPU 提交/缓冲注册=库外 render 域;SsboId::of 哈希≠原键=库内共享机制缺口,ssbo_library 层对账);48B↔104B 映射待取证;噪声阈值 res+0x78 待取证。
- **garden/sheep = review_passed(限定) 维持**。
- **实测**:`cargo check -p country-core` 通过;`cargo test -p country-core --lib` 1592 过/1 败(geometry/sphere 他组)。

### 组 eco restorer 五轮(restorer-eco, 2026-10-15;coordinator 卡6 SsboId 对账)

- **ivy → in_review(五轮)**:
  1. **SsboId 机制对账落账**(ssbo_library.rs):0x140c9cd80 反编译定案 NameHash::compute=默认 xxh3_64;原 EXE 0x142b2ebb0 192B secret 与默认 xxh3 secret 字节一致(默认复算 Cobblestone/Nani 均命中)——**原版即默认 secret,NameHash::compute 现状正确,无需 custom-secret 改造**。例外待取证:ivy 三个 SsboId(rne-read qword 0x142c2e248/280/330)经 ~2000 候选名均不满足 xxh3_64(name),NAME 串未定位。
  2. **send_to_gpu 提交**:upload_ivy_leaves/segments 已带 &dyn GraphicsDevice+gpu_write(库内 render 抽象);缓冲注册/实际提交=库外 render 域接线;SsboId 名字未解析使 SsboId::of 现不对应原键。
  3. **garden pin**:draw_raster<GardenRasterStagingSsbo> 消费已接线(staging_rasters 宏注册),对账通过。
  4. terrain 回读属 interaction 组,登记。
- **garden/sheep = review_passed(限定) 维持**。
- **实测**:`cargo check -p country-core` 通过(meta_commander 暂态 error=state 组在途);`cargo test -p country-core --lib` 1592 过/1 败(geometry/sphere 他组);ivy 29/garden 11/sheep 2/startup 32 全过。

### 组 eco 卡5 交接(restorer-eco → 新批会话, 2026-10-15)

- **卡5 decorator_storage.rs(114 函数)+decorator.rs(131)+decorator_subtype.rs(49)** 由 coordinator 续派新批会话承接。证据就绪:RE-DECORATOR-STORAGE.md(reports/eco/walls/)+ 章节 decorator_storage.rs.md(已补实体机制/消费端节)。
- **关键证据指针**:per_wall HashMap@+0x00(0x98B)+decorator_to_storage@+0x60(24B)+terrain/stair 内联+immutable;Decorators=removed_ids+内嵌 map<u32,DecoratorInfo 0x48B>+modified_ids;**纯数据存储无 bevy Entity,实体关联靠 DecoratorId 组件**;函数族(查询/插入/删除/墙事件 0x140a1ea40/0x140a1f590/0x140a1f550/快照 restore_snapshot 16B XOR/迭代);wall 发布链 publish_public_wall_state 0x140a8b530→OnWallChanged/OnWallDeleted→on_wall_split/merged/removed;archive_current_decorators_in_storage 0x14123b5c0。纠错:decorator.rs serialize 是 DecoratorId u32 十进制化(newtype serde)非 DecoratorInfo 序列化。未决(U):DecoratorInfo 72B 完整字段表、d2s 24B 判别/payload 语义、实体生成系统 VA(render nani)、on_wall_split 0x28B 记录语义。

### 组 state(状态与持久)— restorer-state / reviewer-state / reanalyst-state

| 模块 | 起点 | 本批目标 | 验收门槛(coordinator 细化 2026-10-15) | 状态 |
|---|---|---|---|---|
| `systems/save` | 仅节表/脏块纯逻辑,无文件消费者 | 完整异步捕获(load_game/save_game/save_thread/snapshot/snapshot_gc/shadow_file) | **原函数闭合**:11 原文件 137 函数。在位:save_game(33)、save_thread(30)、shadow_file(16)、save_backup_list(10)、save_backup_metronome(4)、snapshot_gc(1)、mod.rs(5)。**缺文件(需新建)**:load_game(14:load_game/LoadGameManager::new/parse_saved_game_data/apply_parsed_game_data/apply_backward_compatibility_overrides/upgrade_window_color/extract_max_wall_id_from_history)、snapshot(21:capture_game_snapshot_save_data/capture_game_snapshot/try_chunk/validate_and_load_closest_snapshot_data/try_load_snapshot_data/init_game_state_from_snapshot)、load_completion_checker(5)、save_file_identity(6)。**消费者**:startup_session::load_game_session、render_loop/session_selection 预览、toolbar menu load/save_error、tiny-glade panic_dump::dump_save、run_game。**证据**:章节 + fresh 反编译(save_game 0x140a3f470、load_game 0x14146d030、capture 0x1417a0430、execute_save_cmd 0x141721aa0)。异步捕获/load 回放若依赖 GameSaveData/MetaCommand 见跨组依赖裁决。 | review_passed(限定已实现范围;完整 save 仍 blocked:域组装/commander_update/load_completion_checker) |
| `resources/session` | 仅 ID 与主题谓词 | 完整切换生命周期(NextSession/world 切换、GameSaveData/MetaCommand 缺口) | **原函数闭合**:1 原文件 70(业务 28)。在位:bounded(SessionId/GladeType/SessionMutator/SessionMeta/ActiveSession + serde)。**缺(需实现)**:NextSession(load 0x140b46920/create 0x140b46960/ignore_snapshot 0x140b46a20/append_on_load<MetaCommand> 0x1403b5150/append_on_load<MetaCommandFence> 0x141328ac0/load_latest)、SavedSession(loading_background_path 0x140b46a50/screenshot_path)、refresh_session_list 0x140b46b00、delete_session 0x140b46d60、load_saved_session_list、SessionFlowPlugin impl$10::build 0x140b47840。**消费者**:startup_session::load_game_session(会话切换)、meta_commander::commander_update(on_load 回放)、session_selection 预览、glade_themes/load_glade_audio/grading/tod_and_weather、water/meadow/critters 生成、color_library、music_system(ActiveSession 已接)。**证据**:章节 + 上列 VA fresh 反编译。**blocked 项**:append_on_load<MetaCommand> 与 on_load 回放依赖 meta_commander.rs(当前未建),属跨组裁决。 | review_passed(限定;extend_on_load tag 并入 save 处理) |
| `systems/audio/music` | 仅分派表及边界纯函数 | 播放器拥有状态、随机队列和更新算法(music_player/music_system) | **原函数闭合**:3 原文件 20 函数,全部在位。music_player(15:update/init/load_tracks/switch_track/current_track/stop_immediately/pause/probabilistic_switch_track/trigger_photomode_minus_fpv_snapshot/trigger_fpv_snapshot/stop_photomode_snapshots/get_timeline_beat_properties/generate_track_playlist)、music_system(3:music_audio_system/music_audio_init_system/update_music_richness_based_on_time)、music_debug_gui(2)。**消费者**:startup/load_glade_audio、music_system→music_player 消费、music_debug_gui、ActiveSession 主题、native FMOD(native.rs 现场集成)。**证据**:章节 + fresh 反编译(MusicPlayer::update/switch_track/init/load_tracks VA)+ round3 music closure。 | review_passed |

### 组 state 深度逆向产出(re-analyst-state, 2026-10-15;RE-ANALYSIS.md×3 + 6 份 decomp evidence)

- **GameSaveData/MetaCommand 可解,阻塞解除**:MetaCommand=0x38B 7 qword{tag@+0,载荷×6},**空哨兵=0x8000000000000014**(0x8000000000000000 为变体 0=会话加载判别);consume=pop_front(head 环绕),peek=[+0x18] len 判空。commander_update 0x14091f180 case 0..0x13 分派表:case 0=会话加载全流程(AppMode 门→fence 0x14091ef90→临时 on_load→0x1412e0b90 应用→带 fence 重入队)。on_load 元素=**0x20B**;append_on_load<Fence> 0x141328ac0={0x8000000000000000,fence};append_on_load<[MetaCommand;3]> 0x1403b5150={3,Box<0xa8B>}(调用方 run_game_thread 0x1405a3440);**fence 终止=0x8000000000000003**;0x140b5a180=extend_script。
- **GameSaveData 消费边界闭合**:16+ SystemParam 源 + AutoSaveState 缓冲{ptr,len}@+0x60/+0x68;execute_save_cmd OpenFile 全量重写经 save_game_to_file<Cursor> 0x141750c90。**逐字段 layout 仍卡**。
- **音乐修正**:switch_track 0x1408b70d0=**洗牌弹出器**(身份填充→FUN_14132b1a0 Fisher-Yates→不重复守卫→弹尾),非顺序推进;FMOD start/stop 在 update;0x1408b7620=节拍回调(load_tracks 注册),sysres "probabilistic_switch_track@0x1408b7620" **PDB 错位**。播放列表字段{+0x100/+0x108/+0x110}。
- **卡点(blocked,待 re-analyst 续)**:GameSaveData 逐字段 layout(save_game 0x140a3f470 分段逐槽);run_game_thread 3 命令载荷(rbx+0xb00);fence 双 tag 关系(append 0x8000000000000000 vs 终止 0x8000000000000003);变体 2 unblock 偏移 +2 vs +0x10。
- **域类型证据包(re-analyst-state 续派, 2026-10-15;RE-DOMAIN-TYPES.md 落盘,GameSaveData 组装解锁)**:CameraRigParametersCache 6 字段(to_json_string 0x1416f8470=存档 Camera 节,旧 7 字段表含 position_cache 已修正);HistoryIdentity=u64 newtype(generate=fastrand);GladeColorCustomization 15 序列化字段全表(visit_str 0x14085f550/to_json_string 0x14085ee90,多为 ColorId+3 字节枚举);GameProfile ron 只写 5 字段(player_id 不序列化但 deserialize 接受,default=Uuid::new_v4)。load_completion_checker 澄清:check_load_completion=(AppMode,LoadingGameComplete,LoadingCompletionState,ActiveSession),is_session_ok_to_load=(SessionId,&UnsavedFileList)——均不含 GameProfile;其真实消费方=load_game 第 3 参/save_game/startup。已派 restorer-state 定义 4 类型并完成 save_game 组装。

### 组 state 评审结论(reviewer-state, 2026-10-15 首轮;REVIEW.md×3 落盘)

- 已实现部分多为真实实现,经 fresh 反编译+RNE 逐字节核对;多处 src 比章节更准(9 节键="snapshots"、metronome clamp=8.0/步进=-1.0、音乐 init 四快照+三事件、门资源=EscMenuState 非 AppMode、stop 模式映射 mode0=ALLOWFADEOUT/mode1=IMMEDIATE)。未发现空壳/假数据/伪造加载成功;session 缺口诚实披露。
- **FIX×2**:① music_player looped→PlayingLoopedOnce 转换原版 B1(0x1408b5615)不清 +0x11c,src 清零 elapsed_time 需改+测试同步;② SaveBackupMetronome::add_active_game_time 缺 +0x148<60.0 阈值门(门在累计块内,非 AutoSaveState)。
- **PARTIAL(低)**:SaveBackupList 计数器原版对每个入选条目 max(n+1)(待反编译 0x14166df80);get_timeline_beat_properties 存储顺序(bar 最后写)。
- **状态**:save/music=fix_pending;session 现状可 review_passed(限定已实现范围),完整模块验收仍 blocked(异步捕获/NextSession/MetaCommand/全局接线缺口)。
- **⚠️ 证据链缺口**:contracts.json 引用 evidence/core-recovery-20261015/{critters,garden,glade-settings,input,ivy,music,save,session,terrain,ui-clutter}/REPORT.md 共 10 份 + README 引用 {gui,walls,roofs-stairs,raster}/REPORT.md——**该目录全树不存在**,完成度声明缺证据;已派 coordinator 调查重锚(已执行,见证据链缺口追踪节)。
- **三轮(2026-10-15)**:extend_on_load 批量条目按机器码吸收(OnLoadEntry 枚举 Command/Batch,tag=3,两遍回放按枚举分派,fence 以 tag||payload==0x8000000000000003 终止);SessionFlowPlugin::build 补插 ActiveSession;SaveBackupList 计数器保持待取证。save/session/music → review_passed(限定)。⚠️ 全 crate 编译被 eco ivy_spawner.rs:242 unclosed delimiter 暂时打断(已提醒 restorer-eco 收口)。

### 组 interaction(交互与世界)— restorer-interaction / reviewer-interaction / reanalyst-interaction

| 模块 | 起点 | 本批目标 | 验收门槛(coordinator 细化 2026-10-15) | 状态 |
|---|---|---|---|---|
| `systems/backend/input` | 无实际后端输入消费者 | 键盘/Steam/录制全链(translate_input/input_record_replay/input_replay_for_trailers/update_cursor_position) | **原函数闭合**:7 原文件 21 函数。在位:calculate_input_delta(2:calculate_input_delta/digital_delta)、translate_input_to_cursor_moved(1)。**缺文件(需新建)**:translate_input(5:translate_input+closure)、input_record_replay(8:control_input_replay/advance_input_replay_frame/input_record_replay/normalize_cursor_pos/unnormalize_cursor_pos)、input_replay_for_trailers(3:record_and_replay_input_for_trailers/do_input_replay_ui)、reset_input_state(1)、update_cursor_position(1)。**消费者**:MainActionSet owner → mode_manager、calculate_input_delta 的 digital_delta、input_record_replay、translate_input(OS/设备生产者→翻译→光标/动作增量→录制回放)。**证据**:章节 + fresh 反编译(translate_input/update_cursor_position/input_record_replay VA)+ round3 input closure。 | review_passed(限定;显式未覆盖:动作↔槽映射/Steam/帧 serde/try_load/SyncPoint) |
| `systems/terrain` | 仅高度资源及独立地形算法 | 工具 UI 和历史生产(terrain_editing/terrain_stroke/历史/工具) | **原函数闭合**:9 原文件 31 函数。在位:edit_terrain_stroke(1)、draw_terrain_stroke(1)、delayed_terrain_changed_msg(6)、mod.rs(3)。**缺文件(需新建)**:deferred_terrain_history(2:history_record_terrain)、terrain_hover_idx(2)、ui_draw_terrain(2)、ui_edit_terrain(3:ui_edit_terrain/MouseActivityTimer)、ui_terrain_focus_mode(11)。**消费者**:render_loop read_terrain_data、system_water draw_water、history replay_water、地形工具生产者(AppMode/ToolUI)、OnTerrainChange 音频、延迟回读与聚合通知(已接部分)。**证据**:章节 + fresh 反编译(各 VA)+ round3 terrain closure。 | review_passed(限定;显式未覆盖:EditType::Terrain 真 tag/focus_mode switch 表/ui_draw 消费面/脏掩码位序) |
| `systems/ui_clutter` | 仅 Delete 序列化投影 | 拾取/生成可见消费(ui_autoclutter/rmb_autoclutter/debug_spawn_clutter/mod) | **原函数闭合**:7 原文件 20 函数。在位:defered_autoclutter_history(1)、mod.rs(1:impl$1::default)。**缺文件(需新建)**:ui_autoclutter(7:ui_convert_autoclutter_to_decoclutter+closure/get_clutter_rot_in_roof_space/maintain_autoclutter_parent_hashmap)、ui_focus_decoclutter(6)、decoclutter_show_pins(3)、rmb_autoclutter(1:handle_rmb_autoclutter)、debug_spawn_clutter(1:debug_spawn_all_clutter)。**消费者**:system_clutter::impl$0::build、rmb_autoclutter、ui_autoclutter、history replay_auto_clutter(已闭合的删除集合)。**证据**:章节 + fresh 反编译(各 VA)+ round3 ui-clutter closure。 | review_passed(限定;DecoratorStorage 实体/convert quat 接线/decoclutter 注册/ui_focus 主体/debug_spawn 消费端待验收) |

### 组 interaction 评审结论(reviewer-interaction, 2026-10-15 首轮;RESTORE 未落盘,审已有现状)

- **input**:**FIX×2 已确认(以 fresh 反编译为准,证据 input/REVIEW.md + input/evidence/decomp-0x1416feeb0-fresh-shared.txt 1354 行)**。① calculate_input_delta/digital_delta@0x1416feeb0:槽 20..25(ui_left/right/down/up/tab_prev/tab_next)独立 held 连发分支(Shape B:dt 累加,>=0.2 → 0.1 复位 disc=2 repeat),当前全槽 Shape A,UI 6 槽长按永不 repeat。② translate_input_to_cursor_moved@0x140b17a80:方向与原合同相反——原函数消费录制帧(stride 0x18,tag@+8)→ 产出一个 Events<InputCursorMoved>,夹取上界 [0, cursor-192](0x1429e95b0 read_memory=Vec2(192,192));当前 consumes InputCursorMoved、produces 自造 InputCursorDelta、夹取 window_size、有 captured 分支。③ mod.rs 注册非原事件 InputCursorDelta。input 11 原函数仅 2 在位且均有问题,不可宣布完成。
- **ui_clutter**:defered_autoclutter_history=history_record_auto_clutter@0x1409a9160 语义 OK 但**改名未登记**;delete_auto_clutter 适配器录音门阻止删除本身(原版删除无条件,门只应影响写历史);其余 7 原函数全缺。维持 queued。
- **terrain**:现有 3 CPU 系统逐条核对行为正确(stroke 缺失是 log+skip 非 panic,旧章节写错已证);EditCommand 字段序待取证(原似 {falloff,height,position,id} id@+0x1c,与 tg_terrain 不一致,ui_edit_terrain 时按原序对齐);缺 rocky_terrain/terrain_rock_mask 等。维持 queued。
- **二轮(2026-10-15,REVIEW2.md×3)**:
  裁决:translate_input_to_cursor_moved 方向=restorer 正确**(enriched PDB va=0x140b17a80,笔误 0x140b16a80 作废;PDB 签名+两调用者一致=consumes InputCursorMoved→produces InputCursorDelta;夹取=WindowSize,0x4330000000000000 为 i32→f64 魔数非 192 减法),一轮"cursor-192"已撤回。
  **FIX×3(restorer 引入回归)**:① input 捕获路径 tag 2 反了——捕获时 tag 0+2 都累加(仅 tag 1 跳过),应 `Relative | RelativeWhileCaptured => delta += value`;② terrain ui_edit 点集死代码(DIRTY_MASK_POINTS 未置位);③ ui_clutter 旋转输入疑错(acos(position.y))+SPIRAL_ANGLE/类型表待取证。
  PARTIAL/GAP:input_record_replay(帧偏移 0x68..0xb8 待对账、serde 待取证、同步点 seek/try_load 缺)、translate_input GAP 壳(键绑定表/Steam 待取证)、ui_terrain_focus_mode/ui_focus 门存根、decoclutter register_required_components 空存根。
  三模块维持 in_progress,不可 review_passed;FIX 已路由 restorer-interaction,证据补挖已派 re-analyst-interaction。

### 组 interaction 深度逆向产出(re-analyst-interaction, 2026-10-15;RE-ANALYSIS.md×3 + 11 份反编译 evidence/country-core-deep/interaction-group/decomp/)

- **input**:calculate_input_delta 全解纠错=40 路 digital 通道(非旧章节三通道),死区 0.1/0.05、digital 特例 0.2/0.1(**与 reviewer FIX 互证**);reset_input_state=11 qword 块下移+双队列按 sync 点修剪+门控 reset_all;translate_input 全链=键盘/鼠标/Steam(IsSteamRunningOnSteamDeck)门+3 事件双缓冲+绑定表 0xc3 哨兵,analog 翻译仅 mode==2 或(10 且 sub!=1);update_cursor_position 内联 5 宿主未定位(I)。
- **terrain**:CameraCursorRay 布局纠错 pos@0/dir@0xc(旧章节错);replay_terrain ease_out=1−(1−t)³,instant 断言点数<0x3fffff(panic@0x142aae990);AppMode 门(机器码):ui_draw/ui_edit=10、terrain_hover_idx=modes{0,5,9} 0x421 掩码、focus_mode=首 u32==1;terrain_hover_idx"高度≤0 且 id 有效会写"(纠正旧章节"水位");ui_edit_terrain 16 位脏掩码遍历+6 资源写,panic@0x142ae3acf 行 370。
- **ui_clutter**:0x48B spawn 事件布局(tag=2@0/subtype=3@+0x29/type@+0x3c/entity@+0x40);rmb_autoclutter 旋转=acos+ClutterMeshTy::from;debug_spawn 42 螺旋;auto_clutter_edit serde=单字段。
- **卡点**:① **AppMode==10 已裁决(2026-10-15,RE-APP-MODE.md):mode 10=Terrain 工具**;0x421=bits{0,5,10}(0=All/5=Garden/10=Terrain);AppMode=16B{disc u32@0,payload u64@4,extra u32@0xc} default=0;is_tool_selected=disc∈{6..12,15..18}(0x3cfe);translate_input 相机平移门(mode==10 && byte1!=1)⇒ 运行在 {2,10};ui-input §3.3 作废。disc(7..12,15..18)→变体名仍 U。② ui_terrain_focus_mode 167KB 内联体进入/退出状态表未全译。③ update_cursor_position 内联 5 宿主未定位。

## 跨组依赖裁决(coordinator, 2026-10-15)

**问题**:`resources/session` 的 NextSession/GameSaveData/MetaCommand 缺口可能阻塞 save 的
完整异步捕获与会话切换。核实结果:

- **GameSaveData** = save_game 0x140a3f470 同步保存路径中从 ~9 资源拷贝组装的栈上大结构
  (+0x28..+0x100 密集字段)→ save_game_to_file<Cursor/File>。**不是**独立原文件,是
  save_game 组装逻辑;type/schema 需 re-analyst 从 save_game 反编译定点挖。
- **MetaCommand/MetaCommander** = `meta_commander.rs`(inventory 有索引条目但 0 原函数,
  章节 52 函数、业务 9)。80 B MetaCommander{commands: VecDeque<MetaCommand> 0x38B, sinks,
  RandomState};commander_update 0x14091f180 每帧处理一命令,case 0..0x13 含会话加载/
  TimeUpdateStrategy/InputRecordReplayState/建目录/profiling/ShutdownGame/exit;MetaCommandFence
  = Arc<Mutex<(bool,bool)>>。**当前 src 未建该文件**。消费方:lib.rs、startup_session::
  load_game_session、load_game、finish_replay、timelapse_replay、toolbar report_bug、
  tiny-glade game_loop/run_game/screenshot。
- **阻断面(实测)**:仅两条路径依赖 meta_commander —— (a) session `NextSession::
  append_on_load<MetaCommand,3>/<MetaCommandFence>` 与 load_game 的 on_load 命令回放(fence
  0x8000000000000003 为回放终止条件);(b) `startup_session::load_game_session`
  (0x14166e7d0,78 KB)完整会话切换。

**解耦建议**(save 不依赖 meta_commander 的可先闭合):
1. **先闭合(不依赖阻塞)**:save 的 shadow_file/save_thread(execute_open_file_cmd/
   full_write_history_file/backup/snapshot 线程)/save_backup_list/save_backup_metronome/
   snapshot_gc/save_file_identity/load_completion_checker 与 session 的
   NextSession(load/create/ignore_snapshot)数据结构、SavedSession 列表/删除、
   SessionMeta 落盘——这些是纯逻辑/文件层,不需要 MetaCommand。
2. **中间**:save_game 的捕获路径(capture_game_snapshot_save_data→Arc→set_current_snapshot→
   AsyncSaveCmd::CaptureSnapshot)与 save_game_to_file 各节 to_json_string 不依赖
   MetaCommand,可先闭合;GameSaveData 组装结构由 re-analyst 定点挖出后作为共享类型。
3. **最后(需 meta_commander)**:session 的 append_on_load<MetaCommand/Fence> + load_game 的
   on_load 回放 + load_game_session 完整切换 —— 三者共用同一条会话加载命令链,应作为一条
   "会话切换链"一起验收,re-analyst 先出 meta_commander.rs 章节级证据,restorer 再实现。
   期间 load_game_session 保持现状(不伪造加载成功,与 README 一致)。

**组内分工建议**(组 state):restorer-state 先做步骤 1+2(save 可解部分),re-analyst-state
定点挖 meta_commander.rs(commander_update/peek/consume/fence 已反编译)与 GameSaveData 组装,
出 RE-ANALYSIS.md 后回 restorer 实现步骤 3;仍 blocked 则回主会话裁决是否把会话切换链
作为独立任务派给组 interaction 的 reanalyst。

## 基线编译(coordinator, 2026-10-15)

`cargo check -p country-core --all-targets`(父级
`/Users/yuluo/Desktop/game/games_test/tiny-glade`):**通过,exit 0,1m31s**。
当前模块树(9 模块均已声明于 lib.rs `pub mod systems::*`)编译干净,无失败点。
此仅作 restorer 对照基线;编译通过不等于任何模块业务完成(不据此标 review_passed)。
注意:inventory `compiled_files` 快照为 00:12 生成,晚于当前 src(20:48),不可作
"当前是否编译"依据;以本次现场 check 为准。

## 证据链缺口追踪(coordinator, 2026-10-15)

> 触发:reviewer-state 报告 contracts.json 引用 `evidence/core-recovery-20261015/*/REPORT.md`
> 10 份 + README 引用 4 份,全树不存在。调查与重锚方案见
> `planning/2026-10-15-evidence-reanchor.md`。状态:OPEN → INVESTIGATED → REANCHOR_PENDING。

- **结论 ①(可否找回)**:目录**从未存在**——`find` 无目录;git(games_test,HEAD 6845e43f7)
  `git ls-files`/`git log --all -- <路径>` 均空;全库仅 README 命中该串(引用已提交,目录从未建)。
  这批 REPORT.md 是规划落盘位,实际未写入;"限定核验"结论内联在 contracts.json
  `verified_scope`/`remaining`(规范内容),详细核验此前在 `skeleton/task-index/work/TG-*/core-{gui,
  walls}-20261014/REPORT.md`,本批在 `recovery/agent-team/reports/<组>/<模块>/`(已落盘多份)。
- **受影响**:contracts.json 10 条 → 19 个合同条目(critters 2/garden 2/glade-settings 1/input 2/
  ivy 1/music 1/save 4/session 2/terrain 2/ui-clutter 2);README 4 条(gui/walls/roofs-stairs/raster)。
- **重锚方案(推荐 A)**:contracts 每条缺失引用替换/追加为真实文件——逐文件章节
  `evidence/country-core-deep/chapters/<file>.rs.md`(19 个全部在位,逐一核对)+
  `round3/MODULE-DEEP-DIG.md`;README 的 gui→skeleton/task-index/work/TG-cfac1f498c427933d2/
  core-gui-20261014/REPORT.md、walls→TG-9f411268a571c6bce1/core-walls-20261014/REPORT.md、
  roofs-stairs→country-core-deep/roofs-rt/ALGORITHMS.md、raster→country-core-audit/
  R5_world_raster_terrain.md。备选 B=重建目录做"重锚收集器"(仅聚合既有证据,不写新结论)。
- **✅ 已执行(主会话, 2026-10-15, 方案 A)**:contracts.json 19 条目 evidence 已替换为
  `evidence/country-core-deep/chapters/<key>.md` + `round3/MODULE-DEEP-DIG.md`;备份
  `contracts.json.bak-20261015`;inventory.json `contracts_sha256` 已重算
  (1dd7a90a… → cb494c75…);README 4 处引用已改真实路径并标注"从未落盘,已重锚"。
  状态:OPEN → INVESTIGATED → REANCHOR_PENDING → **REANCHOR_EXECUTED**。
- **待办(余)**:① 每模块 report 落盘后追加 `agent-team/reports/...` 指针;② coordinator 纠错
  (9 节键=snapshots、metronome clamp 8.0、节拍回调 PDB 错位等)回填 contracts/章节。
- **附带**:reviewer-state 已用 fresh 反编译纠错(9 节键=snapshots、metronome clamp 8.0、节拍
  回调 PDB 错位等),建议回填 contracts/章节;session 现状可 review_passed(限定),完整切换 blocked。

## 下一批分派(验收层,2026-10-15,依据 planning/2026-10-15-next-batch.md)

| 卡 | 内容 | 组/角色 | 状态 |
|---|---|---|---|
| 卡0 | 4 域类型证据包 | re-analyst-state | ✅ 完成(RE-DOMAIN-TYPES.md) |
| 卡1 | save GameSaveData 域组装 + load_completion_checker | restorer-state | ✅ 闭合(reviewer-state 终验 REVIEW4.md;3 低优先修正排队) |
| 卡2 | 会话切换链(load_game_session/commander_update/on_load 回放/全局注册) | state+startup | ✅ 闭合(reviewer-state 终验 REVIEW5.md;FIX-1 AppMode 门 {13,14} 待 restorer 吸收;case 2/3/7/8/0xc/run_game_thread/MetaCommand 应用待宿主) |
| 卡3 | Steam 全链(input) | restorer-interaction | ✅ 闭合(库内部分;systems::steam 库外登记) |
| 卡4 | 录制往返(input) | restorer-interaction | ✅ 闭合(InputFrameSerde/try_load/seek 复验通过,REVIEW6.md) |
| 卡5 | DecoratorStorage 实体(decorator_storage.rs 114 函数,库内缺失) | 证据:re-analyst-eco ✅ RE-DECORATOR-STORAGE.md;实现:restorer-decorator ✅ | ✅ review_passed(限定;两项待补 fresh 定案+18 测试过;残留:子侧状态字节公式+0x45 U 登记) |
| 卡6 | render GPU 接线(ssbo_library SsboId 对账/ivy send_to_gpu 提交/garden pin) | restorer-eco | ✅ 闭合(reviewer-eco 五轮:SsboId=默认 xxh3 定案,三轮建模缺口撤销;send_to_gpu/garden pin 对账过;ivy NAME 串+缓冲注册待登记) |
| 卡7 | terrain 回读(render_loop read_terrain_data/SSBO) | interaction(卡3/4 后) | ✅ 闭合(reviewer 复验 REVIEW7.md;mask 四类归卡B 资源 GAP) |

原则:**库内能做则做,库外接线登记**。SsboLibrary/GraphicsDevice/SsboId 均在库内(country-core/asset_libraries+render/nani);decorator_storage.rs 为库内缺失原文件(114 函数),卡5 证据已派。

## 第三批卡片(coordinator 规划, 2026-10-15;planning/2026-10-15-batch3-modules.md)

盘点:9 优先级+卡1-7 之外仍有 **258 个原文件无正式文件在位**(无 contract)。大块:collision/world.rs(170)、walls/decorator.rs(131)、decorator_storage.rs(114)、wall_compound_edit.rs(79)、decorator_subtype.rs(49)、color_edit.rs(48)、audio_resources.rs(47)、valid_enclosure(45)、camera_rig.rs(44)。

| 卡 | 内容(README order) | 组/角色 | 状态 |
|---|---|---|---|
| 卡A | walls/roofs/stairs/collision 收尾(order2-3,拆5子卡:A1 decorator 族/A2 wall systems/A3 roofs/A4 stairs/A5 collision) | interaction 为主 | ✅ 全部闭合(A1 decorator 接线/A2 wall 核对/A3 roofs+roof_noise/A4 stairs 权威图/A5 collision 核对;split/merge 迁移待 wall_compound_edit、实体生成待 render 域登记) |
| 卡B | world_raster 状态机(order4)+terrain_stroke/rocky/rock_mask | interaction | ✅ 闭合(reviewer-interaction REVIEW8.md;mask 四类 GAP 闭合+rocky_terrain 新建+world_raster 已接核验;EditType::Terrain tag 待卡C 协调;打包源布局/逐分支后续子卡补证) |
| 卡C | history 收尾(9 edit 类型 498 funcs+replay 族,order5) | state | ✅ 闭合(reviewer-state REVIEW6.md;5 edit 文件+EditType 扩 14 变体+replay 族库内部分+FIX-1 吸收;40 变体逐 tag 映射/replay_terrain/stairs/pop_bucket 残留登记) |
| 卡D | camera/time/weather(order6,camera_rig 族衔接卡0) | interaction | ✅ 主体闭合(load_parameters/CameraModeStates/转发器/time_of_day 主闭包全部复验通过;camera_rig 剩余大件后置;weather/daily 库内 GAP 归资源层) |
| 卡E | input/UI/mode(order6,ui 族/signifiers/widget_params/slider3d/brush_preview) | interaction | ✅ 闭合(slice1+2+3 全复验通过:widget_params/slider3d/AppMode/signifiers/brush_preview/mode_manager;环形流无锁语义/实体 spawn 登记待接) |
| 卡F | audio 域(order7,34 文件+audio_resources 47 衔接音乐 native) | eco | ✅ 整体 review_passed(限定;三批全复验,kind 表/事件布局/instance-vs-global 对账过;sheep kind4 名/TreeAmbience/栅格采样等待接线登记) |
| 卡G | startup/系统注册(order8,27 缺失+lib.rs 顺序) | state | ✅ 闭合(reviewer-state REVIEW7.md;add_game_startup_systems/register_core_startup/lib.rs 顺序落地;原 28-系统 tuple 域系统批量/load_mesh/run_game_thread 本体残留) |
| 卡H | render 消费桥(order8,宿主接线) | 宿主确权 | ✅ 闭合(reviewer-interaction 复验:render 消费端已实质接线;宿主 run_game 顶层渲染序+ivy/garden 48B 布局登记待接) |

**共享基础缺口 ⚠️**:SsboId 对账已由 restorer-eco(卡6)定案——0x140c9cd80 反编译=默认 `xxh3_64`;原 EXE 0x142b2ebb0 的 192B "secret" 与默认 xxh3 secret 字节一致(默认复算 Cobblestone3RasterStagingSsbo/NaniNextVisibilityBitmaskSsbo 均命中)→ **原版即默认 secret,`NameHash::compute` 现状正确,无需 custom-secret 改造**。例外:ivy 三个 SsboId(rne-read qword)经 ~2000 候选名均不满足 xxh3_64(name),NAME 串待取证。tabloid/tg-history 接线健康,本批无需回补。
**依赖顺序**:卡A→C→D+E→F→G;B∥A;H 依赖全部+宿主确权。**优先**:A1(decorator 衔接卡5)、D(camera_rig 衔接卡0)。

## 第四批残余整合(2026-10-16,coordinator 规划 planning/2026-10-16-batch4-residuals.md)

| 卡 | 内容 | 组/角色 | 状态 |
|---|---|---|---|
| R1 | collision/world.rs(170 rapier 胶水) | restorer-interaction | ✅ review_passed(核心查询族+intersects_any 复验过;spherecast 桩/dirty 瓦片/QueryPipeline 更新/collider 族/intersect_meshes 21/visualize 6 待续) |
| R2 | cached_decorator_transforms(11,B 骨架 9.7MB) | restorer-decorator | ✅ review_passed(限定;10 f32 紧凑变换纠偏+三批有限性校验+双缓冲,4 测试过) |
| R3 | wall_compound_edit 完整族(79) | restorer-state | ✅ review_passed(限定;CompoundEditBuilder+4 结构变体+StateChange 载荷不伪造;~40 操作变体/serde/tag 对齐归 E1) |
| R4+R5 | valid_enclosure(45)+snapping/eraser/shapes | restorer-eco→交接→**restorer-walls 专项** | ✅ review_passed(valid_enclosure+snapping 7 系统+eraser/shapes,22 测试过;InProgressCurve 重采样/字段名/常量待取证登记) |
| R6 | roof_ridge/roof_visual_shape/tiles/dormer/buffers | **restorer-roofs-stairs 专项** | ✅ review_passed(roofs 视觉全 6 业务+roof_ridge 委托+RoofBoundsDims 纠偏;5 例 FIX 复验过) |
| R7 | stairs visual/collider/removed_supports/structural/walls | **restorer-roofs-stairs 专项** | ✅ review_passed(structural_graph 7/7+visual+removed_supports+replay_stairs+共享 infra Stairs 变体;5 例 FIX 复验过) |
| R8 | startup 缺文件批量 27(注册胶水) | restorer-state | ✅ review_passed(REVIEW-R8.md:27 文件四类分类登记诚实,register_core_startup 预留并入缝;域系统并入随各域在途类型闭合) |
| X4 | save_game 本体 ECS 组装 0x140a3f470 | state | ✅ 闭合(REVIEW-X4X6.md:save_game_system 节拍器/触发备份/宿主事件响应;20+ SystemParam 域资源收集由宿主登记) |
| X5 | capture_game_snapshot 域 Query 0x1417a1d00(184KB) | state | ✅ 闭合(REVIEW-X5X7.md:快照收集骨架+节序 RNE 一致;域序列化器/写回字段映射待域恢复) |
| X6 | commander_update case 2/3/7/8/0xc | state | ✅ 闭合(REVIEW-X4X6.md:余 case 机器码语义登记,Handled+注释,sink 载荷/资源面由宿主) |
| X7 | run_game_thread 3 命令(rbx+0xb00,ReVa p-code) | state | ✅ 闭合(REVIEW-X5X7.md:cmd2 变体 8 完整+新会话命令数组;cmd0/cmd1 ReVa 载荷残留) |
| E1 | EditType 40 变体逐 tag(0x142ad88d0/0x142aee31c) | re-analyst-state | ✅ 闭合(REVIEW11.md:EditType 15 变体权威表+src 错位全修;Terrain 保留号 tag15 待定位;WallHistoryOperationTag 登记待并入;tag12 名/serde 对齐残留) |
| E2-E6 | DecoratorInfo+0x45/sheep kind4 名/ivy-garden 48B/NAME 串+NameHash/噪声阈值 | re-analyst-eco | ✅ 完成(E3 kind4=Baah 应用 review 中;E5 复算定案=ivy 键非 xxh3_64(name),ssbo 键来源待另证;E2/E4/E6 处理/登记) |
| E7 | terrain EditCommand 字段序 | re-analyst-interaction | ✅ 完成(RE-EDITCMD.md:0x28B{falloff,height,position,id@0x1c} 定案,与 tg-terrain 序不一致;restorer-interaction R1 后吸收 ABI 对齐) |
| X1-X3 | nani 实体生成/宿主渲染序/28 系统调度 | 宿主确权 | queued(主会话确权) |

验收达成清单:全局调度+存档往返+渲染消费桥+证据完备+宿主确权全闭合方可宣布完整 core 行为验收。

## 状态流转约定

- restorer 完成模块实现并自检 → 改状态 `in_review`,写 `RESTORE.md`。
- reviewer 复核后 → `review_passed`(写 REVIEW.md)或 `fix_pending`(附 FIX 清单)。
- FIX 清单交回 restorer → 修复后再交 reviewer。
- 证据不足/未挖透 → re-analyst 定点补挖,输出 RE-ANALYSIS.md 后再回到 restorer。
- 验收层面(行为等价、消费者闭合、原生证据齐备)达成前不得宣布模块 100%;不得把编译通过
  或局部测试通过当整模块完成。
