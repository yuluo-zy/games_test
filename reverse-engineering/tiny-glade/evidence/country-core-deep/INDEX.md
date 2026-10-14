# country-core systems/resources 深度逆向 — 中央索引（2026-10-14）

目标：深挖 `reconstruction/country-core` 的 `src/systems/**` 与 `src/resources/**`
的真实设计（459 文件 / 1,161 个已登记业务过程）。产出 = 每领域 DIGEST 文档 +
纠错清单；实现代码由后续批次按文档落地。

## 编排

- 工具与技巧：[PLAYBOOK.md](PLAYBOOK.md)（含已证实的逆向事实，agent 必读）
- 权威函数清单：[sysres-procedures.json](sysres-procedures.json)（PDB 编译单元元数据，
  1,161 业务过程，已过滤 derive/依赖胶水）
- 批量 runner：`scripts/run-decomp-batch.py <domain> <dbid>`（带全局串行锁
  `.mcp-batch.lock` —— 机器内存约束，Ghidra 会话串行）

## 领域与 owner

| 领域 | owner (dbid) | 过程数 | 状态 | 交付 |
|---|---|---|---|---|
| history | tg_a1 | 64 | ✅ 完成 | [history/DIGEST.md](history/DIGEST.md) |
| wall | tg_a2 | 268 | ✅ 完成（280 覆盖） | [wall/DIGEST.md](wall/DIGEST.md) |
| session-save | tg_a3 | 66 | ✅ 完成 | [session-save/DIGEST.md](session-save/DIGEST.md) |
| terrain-water | tg_a4 | 92 | ✅ 完成（124 落盘） | [terrain-water/DIGEST.md](terrain-water/DIGEST.md) |
| ui-input | tg_a5 | 157 | ✅ 完成 | [ui-input/DIGEST.md](ui-input/DIGEST.md) |
| camera-time | tg_a6 | 47 | ✅ 完成 | [camera-time/DIGEST.md](camera-time/DIGEST.md) |
| collision-eco | tg_a7 | 154 | ✅ 完成（200 VA 覆盖） | [collision-eco/DIGEST.md](collision-eco/DIGEST.md) |
| audio-misc | tg_a8 | 154 | ✅ 完成 | [audio-misc/DIGEST.md](audio-misc/DIGEST.md) |

## 已收齐领域要点（audio-misc，tg_a8，✅ 154 目标 136 ok + 18 手解）

- **GradingLut::load_cube**（0x140b01050）完整 .cube 解析器全量恢复：TITLE/
  DOMAIN_MIN(必须 0,0,0)/DOMAIN_MAX(必须 1,1,1)/LUT_3D_SIZE（手写 u64 解析）/
  数据行→parse_rgb→RGB9E5→3D 纹理；错误串全录；目录 "assets/luts"；
  GradingLuts = 双 Vec（stride 0x28/0x38，cap-first）。现有 grading_lut.rs 全是
  unimplemented!()，可直接落码。
- **ColorMenuBrightnessValue::to_f32**（0x140b06770）= 三变体无载荷枚举
  判别→{0.25,1.0,4.0} 表（0x142ae785c）；现有 `{Default,Custom(f32)}` 错误；
  default=变体 1。
- **color_id_roof default = 1**（0x14099ca50；wp16 合同/color_id.rs 的 ColorId(0) 错）。
- FMOD 失败全局报告注册表（"module_p_<名>path!(…) :: Warn: <err>" 每函数独立槽，
  锁失败 panic "Mutex was poisonned" 原文双 n）；verbosity 全局 0x14362c3a8 门控。
- Music：0x1408b72e0 = stop_immediately（非 current_track）；get_timeline_beat_properties
  = 向 6 个进程全局（0x143634c68..c90）发布节拍（非 getter）；MusicPlayer 双跳表
  0x142a9fa24/0x142a9fa34。
- Steam：动作表全量（"GameControls"+28 动作/Menu 2/CottageWindow/GothicWindow 层）；
  user_id() 每次现算；is_floating_gamepad_text_input_open = 进程全局 0x143634d94。
- AudioInstance Active 布局 {disc@0, event@8, name cap-first String@0x10, state@0x28}；
  start_if_not_playing 位掩码 0xb；Bank::load 失败 panic；recv = crossbeam 1s 超时。
- PDB 名 8 例错位（update/set_vca_volume/set_parameter/add_audio_events/load_cube/
  stop_immediately/羊叫 to_list/disabled）。
- 小纠错：WindowSize f32×2；ShaderRefactoringState::disabled(&self)->bool；
  FeedbackReportBgTexture::is_available=[+0x36]<7；shutdown 为 bevy 系统。
- 事件串：path_audio "Path Draw"/"Erase Draw"；elevation_supports 3 事件；
  signifier 5 事件（含原版拼写 "HoldTimeSiginfier"）；跨域全局 0x143634d90
  （image_formation 档2 + gpu_lut 非 demo 门禁共用）。
- **未决**：MusicPlayer::update 跳表 case 语义；update_steam_inputs 逐动作映射
  （14+ 同构块）；**0x14362c420 语义**（history 域 finish_potential_mutation 也读它，
  疑似跨域全局）；FeedbackReportResources 0x18..0x36 字段；0x140949c40
  （PDB aspect_ratio，实为析构例程）宿主类型。

## 已收齐领域要点（session-save，tg_a3，✅ 66 目标 57 ok + 9 手解）

- **AsyncSaveCmd 变体 3..11 全解**：3=OpenFile、4=CloseFile、5=ChangeSection、
  6=ReplaceEdits、7=BackupSave、8=ManualBackup、9=CaptureSnapshot、10=Flush、
  11=Shutdown（save_thread `cmd==0xb` 退出）。0/1/2 未出现（U）。
- **存档文件协议**：history.json 手工拼 JSON（edit 间 ",\n"）；ShadowFile =
  `<path>.tmp` + set_length + sync；mark_dirty(start,end) 8 槽脏块区间。
- **快照 schema = JSON 明文 + flate2-1.0.24 deflate**（Terrain 节）；节序
  Version→Walls→Roofs→Decorators→WallColors→GateColors→Terrain→Trees→Raster。
  wp13 "rkyv/ron" 假设作废。
- 备份节拍器：{counter u32@+0x140, 累计 f32@+0x144 clamp[0,1], 阈值 f32@+0x148}，
  档位 {300,600,3600,7200}s，仅阈值<60 时累计。
- meta.json {state,glade,mutators,image}；季节字节 {0=halloween2024,1=其他万圣节,
  2=xmas2024}；is_halloween_any 实为"存在字节≤1"（合同方向相反，已纠错）。
- LogoAnimation 5 阶段状态机（progress[5]/durations[5]；阶段0=Scribble、
  阶段2=Branches；is_waiting=(+0x0c)>0、is_finished=(+0x10)>=1.0）。
- load_game：loading.tmp → NextSessionKind（1=parse+upgrade 备份、2=on_load 回放，
  I）→ PostGameLoadCommand 回放（fence 0x8000000000000003）→ 颜色 ID 升级映射
  {1→2,2→5,4→8,5→3,8→4} → max wall id 恢复。
- 会话目录：{history.json,screenshot.jpg,meta.json,loading_background.jpg}+
  assets/glade；delete_session = deleted_saves 标记 + rename 两段删除。
- prefab：PrefabLibrary 桶 0x28B（Prefab 值仅 16B——现有 `Prefab{parts:Vec}`
  布局存疑）；hot_reload = despawn→audio_trigger→create_prefab_rt 全链。
- PDB 名错位 10 条（finish_wait/is_auto/json_key/screenshot_path/
  snapshots_folder_path 等）。
- **未决**：NextSessionKind 变体 1/2 语义（I）、backup_folder_path 叶名明文
  （描述符 @0x142aef328）、execute_save_cmd 变体 0/1/2、update_loading_screen
  UUID 键业务含义、try_approx_parse_save_file_identity 规则。

## 已收齐领域要点（collision-eco，tg_a7，✅ 200 VA：166 落盘 + 手解）

- **碰撞世界 = 17 个 0x390B 子世界槽** + DirtyTiles 脏瓦片 pub/sub 生命周期
  （flush 每槽 drain→publish→QueryPipeline::update）。
- **SubworldColliderHandle = 24B 带负载枚举**（Empty=0x11；Terrain 判别 0xb=槽 5）。
  重建的 {subworld,index} 与 SubworldIndex 序均错。
- RaycastWorldResult2 = 扁平 0x40B；raycast_simple_exclude_wall_id **忽略 exclude
  参数**（合同/代码需纠）。
- v1 符号清单系统性错位在本域再次证实（10+ 处已按 v2+机器码纠正，DIGEST §7/§9）。
- StairsState GraphMap@+0x28、stubs@+0x29、脏字节 +0x2f（重建 restore 极性反）。
- ClutterMeshTy 实为 **56 路径变体**（重建 11 粗分类错）。
- RaycastGeoSource = 24B{disc,flag}，subworld() = disc→槽表（0x142aef03c/
  0x142aa4415）；Terrain 槽=5 三证锚定（mask 0x20/地形命中 0xb/映射表[11]=5）。
- legacy_spherecast 固定 10 槽顺序 {0,1,3,7,6,4,11,2,12,13}（供 critters）。
- v1 错位纠正补充：0x1408ccc90=legacy_spherecast、0x1408cd5c0=flush_colliders、
  0x1408ffef0=remove_node、0x1408ffd70=add_edge、0x14094c800=closest_segment_index、
  0x1408d9590=dispose_collider_mesh_instance、0x140a879b0=Debug impl。
- StairNodeAnchor Wall 判别=2；AlignKind::position_offset 真计算（重建 stub 错）；
  Sheep::being_petted=+0x58>0；xz_flatten_aabb 输出 {min.x,min.y,max.y}。
- 依赖确认 puffin-0.14.1/bevy_ecs-0.16.0/slotmap-1.0.6。
- **未决**：SubworldIndex 17 槽完整名序（仅 Terrain=5 锚定）；update_rapier_colliders
  (0x1408d0490)/add_node(0x1408ffd30) Ghidra varnode 反编译失败；RaycastMeshData
  写入面（new_*）与读取面（rapier_hit）偏移 ±8 差待裁决；ivy grower 主闭包不在
  符号清单（系统跳表 0x142ab3be0 分派）。

## 已收齐领域要点（terrain-water，tg_a4，✅ 124 反编译落盘）

- v1 索引本域 11 处名/址错位全部纠正（PDB 骨架 RVA + v2 S_PUB32 双确认，如
  0x140a15530=WorldRaster::new 非 unique、0x140868fb0=add_point 非
  resample_and_smooth）。
- **WorldRaster 家族全链 O**：new 布局（composite/scratch 各 0x40000B=512×512
  u8、DirtyTiles 位图 0x4000B=4096 u32 初始 0x07954dbb、56B 头含 parking_lot
  Mutex、staging 0x200B+4 flags）；subscribe = 0x28B Arc 记录压 +0x78 订阅 Vec
  （非 primary panic "Can't subscribe to a secondary dirty tile publisher…"）；
  copy_to_staging 协议（+0x298 选择/+0x299 脏/+0x29a published）；write 置
  +0x299 并 memset staging 0xff——**prior 悬案闭合**；6 槽名序 O。
- ActiveRasterStroke 0x470B 记录全布局（三 Vec 列/弧长@+0x40/笔刷@+0x60/模式
  @+0x64/blend@+0x68）；resample_and_smooth 真址 0x140868e40 = 10 遍 0.25/0.5/0.25；
  add_point 弧长>1.0 触发 solidify。
- terrain：u16 网格@+0x08、min/max@+0x50/54（-2.5/24.0）、intersect_ray=32 步行进+
  3 次二分、copy_from_gpu=8×8 瓦片 16B 差分、视觉实例双线性（load_index_unchecked
  0x1408b54c0 证实）。
- BrushSizes path@+0 0.8[0.32,1.2]/water@+4 2.0[1,5]/garden@+8 3.0[2,4]
  （重建布局需重排）。
- **ICF 折叠警示**：0x14003bb80/0x14067dd80 = 裸 ret/身份函数（v1 曾标多个名字）。
- 合同纠错 15 条（wp7 water +0x30 是位图 Vec、wp8 DirtyTilePublisher 行号、
  0x30B 应为 0x470B 等）。
- **未决**：+0x29b 另一置位点；0x07954dbb 语义；WorldRaster::new 原型来源；
  subscribe 记录消费方；solidify_fluid_into 固化数学；ui_terrain_focus_mode 闭包体。

## 已收齐领域要点（history，tg_a1，✅ 61/64 + public-symbols 399 条）

- **sysres v1 本域 11 处系统性名/址错位**（S_PUB32 裁决）：closest_history_snapshot
  0x140b36460 实为 MultiEdit::deserialize；remove_snapshot_record@0x140a84ba0→
  debug_display；update_compatibility@0x140a82eb0→Debug::fmt；CompoundEditBuilder::build
  真址 0x140a82d50；TerrainEdit::duration@0x140858d30→revert（真 reapply=0x1408592d0）；
  is_default_lantern_brightness 真址 0x140b3f910（与 is_default_stair_width 同址==1.0f）；
  ev_height_changed 真址 0x140a83f80；replay_path→generate_path_replay_cmds 跳板。
- **add_decorator_diff 0x140a854a0 全流程 O**：4 参签名 (&mut History, &DecoratorDiff,
  &StairsDiffTable, &RawTable)；楼梯 checksum（全局开关 0x14362c011）+ 边对循环 push
  tag 0xE/1 HistoryEdit；Arc 替换 pending 表。reconstruction "空 diff 直接 return" 错误。
- **history_process**：9 参 ECS 面（Mut<Events<HistoryCmd>>/Mut<ReplayBuffer>/Local
  双游标第二事件流）；undo 且 cmds>1 时命令 Vec 反转；跳表 0x142ae5420（record
  byte[+9] 分派）/0x142ae545c；ReplayBuffer 静态 0x142ae5430。
- **replay_pop_bucket 0x140b38250 全解**：ReplayBucket 0x40B={decorators@0,stairs@8,
  cmds Vec@0x10}；装饰恢复+楼梯应用链（1408ff210→140900ba0→14157cec0→140b50cc0→
  140b50d60→1408fefe0）。
- **全局族语义**：0x14362c420=panic-in-progress 门控（token Drop 守卫）；0x14362c3a8=
  日志级别；0x14362c011=楼梯校验开关。
- History::add 第三参数 = #[track_caller] 文件路径 &'static str；insert_starting_edit
  为真名（重建改名 new_from_params 错——S_PUB32 无该名）；HistoryChainIndex serde =
  base-100 itoa。
- 快照负载含太阳环境字段（time_of_day/sun_xz_rotation/latitude_degrees）；快照主场在
  save::snapshot 域（validate_and_load_closest_snapshot_data 0x1417a6e50）。
- **未决**：remove_snapshot_record/update_compatibility/TerrainEdit::duration 真址
  （仅候选区间）；快照负载 T 完整类型；两跳表逐 case 语义；History serde impl 位置。

## 已收齐领域要点（wall，tg_a2，✅ 280 覆盖：224 反编译 + 56 手解）

- **WallState = 0x70B（112B）权威布局**：enclosure 记录@+0x38（tag<2=premade）、
  height f32@+0x54、freehand_ty u8@+0x60、arch_type u8@+0x64、removed_supports
  u8@+0x67、双 Vec 曲线；InnerWalls 条目 {WallId, WallState} 步长 0x78；+0xc0 脏代
  计数器、+0x30 工厂子结构。
- **MaterialStats 全布局 + CAS 累加器**：7 特征 prev@+0x28../cur@+0x90..、8 计数
  u32@+0xac..+0xc8、seed@+0x88；arches_length_delta = 7 对 |cur-prev| argmax 带
  -0.01 bias；set_pillar_placements 自旋锁+等待队列。
- WallState serde 13 字段名全表 + 旧版 11 字段表。
- **wall_space/u_space 合同纠错**：{tag!=2, f32}（Option<f32>），判别是形状 tag 非
  is_enclosed；wall_space=+0x28c、u_space=全局常量。
- PDB 名错位 5 处（is_valid/max_y_gable/get_enclosure_params/as_wall_mut/pillar_count）。
- DecoratorType 判别族（值域 0x38..0x3f，7 谓词位运算）。
- publish_public_wall_state（14 系统参数、0x2a8 hashbrown SIMD）与 spatial_hash_update
  权威流 O 级对账；`simplified_curve_2d.is_valid()` 新字段名。
- 原工作室泄漏：/home/h3/code/pouncelight/country-slice-private（Pounce Light）；
  puffin 0.14.1/bevy_ecs 0.16.0/parry3d 0.17.0。
- **未决**：u_space 全局常量磁盘值 0x1（疑重定位）；as_wall_mut 返回 u32 矛盾；
  DecoratorStorage 五集合映射（I）；eraser 倒序串语义。

## 已收齐领域要点（ui-input，tg_a5，✅ 157 目标 137 ok + 20 手解）

- **mode_manager 早退** = 每帧 hash("esc_menu_canvas")，hover 且交点载荷 tag!=2 阻塞。
- **check_if_can_switch（0x140afa8a0）**：非 force 路径置 pending 0xd（Replay）并
  push {事件id, 请求mode}——与 history Replay 判别联动证实；15 个 closure_env_N =
  工具栏类别切换+模式索引推进处理器（映射表已给）。
- Toolbar 布局：Toolbar={categories Vec, current u8@0x18}；ToolbarCategoryModes=
  0x20B；ModeEntry=0x10B {cat u32@0, mode u8@4, arg u64@8}；Toolbar::new 真址
  0x140af9000（重建写 0xaf8c50）；switch_mode 入参 16B 记录；contains 区间 0x38..=0x3f。
- EventInstance={event_id{id,caller ZST},event@8}（bevy_ecs 0.16.0 官方源码定案；
  二进制内嵌路径为 crates.io 官方 registry）。
- send_mode_just_changed_event 载荷={prev,current}；AppMode::is_tool_selected@
  0x140a8a5d0 伪声明（实为 cleanup_timelapse）。
- ui_edit_focus：4 张旋转跳表 = 24 变体 GeoSource 匹配；EditSubject serde 变体名全套。
- UiSettings 7 字段真布局；ToolbarAlignment=Bottom/Left/Right/Top（重建
  Horizontal/Vertical/Bottom/Side 错）。
- PhotoModeCameraMode u8{0..3}、EscMenuState 判别{3,4,7}+flag@8、WelcomeScreenState
  Closed=2、mouse_capture 0x200000002/3 事件流。
- 纠错：next_mode PDB 名错位（实为 serde unknown_variant thunk）、Modifiers::
  disable_snapping=byte@+1、update_mod_key_state@0x140af9000 实为 Toolbar::new。
- **未决**：photo_mode 0..3 ↔ CameraMode 槽 6..12 映射（需与 camera-time 域合并，
  中央已接手）；check_if_can_switch 尾段分发；EscMenuState 变体名。

## 已收齐领域要点（camera-time，tg_a6，✅ 50 反编译 + 90 个 f32 常量解码）

- **CameraMode 是 13 槽（0..12）非 6 变体**：fov 表 0x142aabb98 13 项、
  previous 表 [6,0,1,2,3,4,0,0]、paused update case 6..11、模式 6→0 映射两处；
  6..12 = 拍照/工具模式复用 6 个 body（Normal@+0x100/TiltShift@+0x1b8/Free@+0x270/
  Walk@+0x2f0/TopDown@+0x390/Butterfly@+0x448）+ 各自 fov 槽。
- **MainCamera** @+0x0..0x100 = CameraMatrices 块（calc_matrices+memcpy 0x100）；
  +0x4d0 模式字节、+0x4d8 aspect=w/h、+0x344 鼠标深度、+0xec fov 插值系数。
- **dolly 0.6.0 接线闭合**：TypeId Position=0x6b8463577afc1a66 /
  YawPitch=0x718e4fd258330fe5；match_free_camera_to_normal 重建 Free rig =
  Position+YawPitch+Smooth(1,1)；vtable+0x38/+0x40 = driver 查找。
- **合同纠错**：remap y 基值 {0,3,11,17,22,24}（非 {48,17,…}）；time_update 时钟
  +0x10 为 f64；小时倍率 @+0x23（非 +0x118）；is_active = state!=2；daily 阈值
  432511s≈5.006 天；next/previous 合同地址是截断垃圾（真实 previous @0x140914ac0）；
  CameraRotationPivot 实为 7 变体枚举。
- **会话相机存档 schema**（0x1403b4ca0，PDB 名 xz_projective_rotation 实为 ron
  反序列化器）：{yaw,pitch,position,camera_rig_arm_length,pitch_offset,
  yaw_offset,position_cache} = 0x24B CameraRigParametersCache 元素。
- time_of_day 三资源布局+状态机（hour=fmodf(progress·24+8,24)、day_of_year
  1..365 越界 panic、suncalc 复用）与 weather（glade_type@+0x58==4 正弦 ±0.1·dt）。
- look_at 的 register_required_components = derive 注册胶水；arrow_gizmo =
  主轴拉伸 0.02 增量/−0.1 衰减。
- **未决**（DIGEST §8）：模式 6..12 变体名（→ui-input photo_mode next_demo
  0x140b596b0）；WalkCamera::rotation 0x140994310 16B 拷贝语义；TodLightingSnapshot
  两 u64 字段；suncalc 固定经纬度常量（0x142ad5910..5984）与日期位打包；MainCamera::reset
  0x140911280 全貌；time_of_day param_3/10/11 资源类型名。

## 已由主 agent 直接证实的关键事实（history 核心，2026-10-14）

证据在 [history/decomp/](history/decomp/)，重写已落地
`reconstruction/country-core/src/resources/history/mod.rs`（布局/方法语义均以
机器码为准）：

1. **游戏 Vec 布局 = cap-first**：`{cap@+0x00, ptr@+0x08, len@+0x10}`（与 std
   不同；edit_chain/snapshots/pending.edits 三处一致）。
2. `History` 布局：edit_chain Vec<MultiEdit(40B)>@0x00 / snapshots
   Vec<{Arc, chain_index} 16B>@0x18 / pending 40B@0x30（首字段 i64::MIN 哨兵）/
   count@0x58 / nonce@0x60 / cursor@0x68 / history_start@0x6c / u32@0x70。
3. `MultiEdit` = {edits: Vec<HistoryEdit 24B>, Arc<装饰表 64B>@0x18,
   Arc<楼梯表 96B>@0x20}；空判定四条件（edits.len + 表字段）。
4. 合同纠错：`set_current_snapshot` 是**断言无同 cursor 记录**（panic
   “self.current_snapshot().is_none()”）而非替换；`is_recording`=0x140a86970、
   `finish_potential_mutation`=0x140a86980 签名 `(was_active: bool)`；
   `insert_starting_edit` PDB 真名 `new_from_params`；`add` 的 panic =
   “Tried writing {:?} to History when there was no input recording”。
5. PDB 名与行为存在错位（0x140a84760 PDB=edit_chain_mut 实为
   set_current_snapshot）——**机器码为准**。

## 跨域闭合进展（中央 agent 合并两域证据）

- **photo_mode ↔ CameraMode（部分闭合）**：next_demo 0x140b596b0 跳表
  `{PhotoModeCameraMode 0→1, 1→2, 2→0}`（返回 body 索引；3→panic "unreachable"）。
  结合 camera-time 的 13 槽 fov 表与 previous 表 [6,0,1,2,3,4,0,0]：photo 子模式
  在 body 1/2/0 间轮换，具体占用 6..12 中哪几槽留待实现阶段随枚举恢复定案。
- **全局变量族（三域合证）**：0x14362c420=panic-in-progress 门控（history token
  Drop 守卫）；0x14362c3a8=全局日志级别（history/audio 两域四阈值一致）；
  0x14362c011=楼梯 checksum 开关。
- **颜色 ID 家族（两域合证）**：session-save 升级映射 {1→2,2→5,4→8,5→3,8→4} 与
  audio-misc 的 color_id_roof default=1、ColorMenuBrightnessValue {0.25,1.0,4.0}
  一致；wp16 合同 color_id.rs 的 ColorId(0) 需改。

## 冲突登记（agent 提交 → 主 agent 裁决）

| # | 冲突 | 涉及 | 裁决 |
|---|---|---|---|
| 1 | 0x140af5b30 HistoryCmd::interrupt()=0x02000200 与中断判别
    （byte[1]==4 或 byte[0]==2）字节不符 | history | 待定：需查 interrupt 命令的实际
    消费点（第二事件流 record+0x0B 开关？） |

## 待办

- [ ] 各领域 DIGEST 收齐后：跨领域调用图（把各 DIGEST 的“跨领域线索”合并）
- [ ] 冲突裁决 + 更新 wp 合同
- [ ] 按文档落地实现（下一批次）
